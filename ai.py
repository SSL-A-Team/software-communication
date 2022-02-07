import socket, time, threading, random, logging

HOST = '172.25.100.221'
SRVPORT = 8081
CLIPORT = 8080
TIMEOUT = 30
robots = []
lock = threading.Lock()
DATA_SIZE = 508

def setKeepalive(addr):
    for r in robots:
        if r['addr'] == (addr[0], CLIPORT):
            r['keepalive'] = time.time()
            logging.debug(f"Set keepalive for {(addr[0], CLIPORT)}")

def handleRobot(d, addr, s):
    data = bytearray(d)
    cmd = data[0]
    ret = bytearray(DATA_SIZE)
    addr = (addr[0], CLIPORT)
    logging.debug(f"Got traffic from {addr} {data[0]} {data[1]}")

    # Handle client Hello
    if cmd == 219:
        r = {'addr': addr, 'jersey': 0, 'keepalive': time.time()}
        if r not in robots:
            robots.append(r)
            logging.debug(f"Added {r} to robot list")
        else:
            logging.debug(f"Got hello for {r} I already knew about")
        ret[0] = 221

    # Handle client jersey number
    elif cmd == 221:
        isIn = False
        for r in robots:
            if r['addr'] == addr:
                r['jersey'] = data[1]
                logging.debug(f"Set jersey to {data[1]} for {r['addr']}")
                isIn = True
        if not isIn:
            r = {'addr': addr, 'jersey': data[1], 'keepalive': time.time()}
            robots.append(r)
            logging.debug(f"Added new robot {r}")
        fail = False
        for r in robots:
            if r['jersey'] == data[1] and r['addr'] != addr:
                fail = True
                ret[0] = 255
                e = bytearray(256)
                e[0] = 255
                with lock:
                    s.sendto(e, r['addr'])  # Notify conflicting bot
                logging.debug(f"Found conflicting jersey number {data[1]} between {r} and {addr}")
        if not fail:
            ret[0] = 250
        
    # Handle client ACK
    elif cmd == 250:
        isIn = False
        for r in robots:
            if r['addr'] == addr:
                isIn = True
                setKeepalive(addr)
        if not isIn:
            robots.append({"addr": addr, 'journal': 0, 'timeout': time.time()})
            ret[0] = 221
        else:
            logging.debug(f"Got ack from {addr}")
            return  # Do nothing and don't reply

    # Handle client shutdown
    elif cmd == 251:
        for r in robots:
            if r['addr'] == addr and r['jersey'] == data[1]:
                robots.remove(r)
        ret[0] = 250
        setKeepalive(addr)
        logging.debug(f"{addr} is shutting down")

    # Handle client keepalive
    elif cmd == 100:
        ret[0] = 250
        setKeepalive(addr)
        logging.debug(f"Got keepalive from {addr}")

    # Handle unsupported command
    else:
        ret[0] = 254
        setKeepalive(addr)
        logging.debug(f"Got unsupported command {cmd} from {addr}")
    
    with lock:
        s.sendto(ret, addr)
        logging.debug(f"Sent {ret[0]} to {addr}")

def giveOrders(s):
    # Wait for init
    while robots == []:
        logging.debug("Waiting for robots....")
        time.sleep(1)

    # Send random data
    for j in range(10):
        ret = bytearray(DATA_SIZE)
        ret[0] = 125
        for i in range(1,DATA_SIZE-1):
            ret[i] = random.getrandbits(8)
        s.sendto(data, robots[0]['addr'])   

def checkKeepalives():
    while True:
        for r in robots:
            if time.time() - r['keepalive'] > 30:
                robots.remove(r)
                logging.debug(f"Pruned expired bot {r}")
        time.sleep(TIMEOUT / 6)

logging.basicConfig(format='%(asctime)s %(message)s', level=logging.DEBUG)

with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
    s.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    s.bind(("", SRVPORT))
    data = bytearray(DATA_SIZE)
    data[0] = 220
    order_thread = threading.Thread(target=giveOrders, args=(s,))
    order_thread.start()
    keepalive_thread = threading.Thread(target=checkKeepalives)
    keepalive_thread.start()
    with lock:
        s.sendto(data, ('255.255.255.255', CLIPORT))
    logging.debug(f"Sent hello to broadcast")
    while True:
        data, addr = s.recvfrom(DATA_SIZE+8)
        handle_thread = threading.Thread(target=handleRobot, args=(data, addr, s))
        handle_thread.start()