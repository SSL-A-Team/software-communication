import socket, time, threading, logging

HOST = '172.25.100.221'
CLIPORT = 8080
SRVPORT = 8081
CLIENTS = []
TIMEOUT = 30
JERSEY = 1
KEEPALIVE = 0
SERVER = 0
lock = threading.Lock()
DATA_SIZE = 508

def handle(d, addr, s):
    data = bytearray(d)
    cmd = data[0]
    logging.debug(f"Received {cmd} from {addr}")
    ret = bytearray(DATA_SIZE)
    if cmd == 219:
        logging.debug(f"Traffic ignored from {addr}")
        return  # Ignore broadcast traffic not from the server

    SERVER = (addr[0], SRVPORT)
        
    # Handle server ACK
    if cmd == 250:
        logging.debug(f"Received server ack")

    # Handle server hello
    elif cmd == 220:
        ret[0] = 250
        logging.debug(f"Received hello from {SERVER}")
        with lock:
            KEEPALIVE = time.time()
            s.sendto(ret, SERVER)

    elif cmd == 221:
        ret[0] = 221
        ret[1] = JERSEY
        logging.debug(f"Provided jersey number {JERSEY} to {SERVER}")
        with lock:
            KEEPALIVE = time.time()
            s.sendto(ret, SERVER)

    elif cmd == 125:
        ret[0] = 250
        data.pop(0)
        raw = bytes(data)
        logging.debug(f"Received {raw}")
        with lock:
            KEEPALIVE = time.time()
            s.sendto(ret, SERVER)

    elif cmd == 255:
        ret[0] == 250
        logging.debug(f"Received server stop")
        with lock:
            KEEPALIVE = time.time()
            s.sendto(ret, SERVER)

def keepalive(s):
    while True:
        if SERVER == 0:
            time.sleep(TIMEOUT / 6)
            continue
        with lock:
            if time.time() - KEEPALIVE > 30:
                data = bytearray(DATA_SIZE)
                data[0] = 100
                s.sendto(data, SERVER)
                logging.debug(f"Sent keepalive")
        time.sleep(TIMEOUT / 6)

logging.basicConfig(format='%(asctime)s %(message)s', level=logging.DEBUG)

with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
    s.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    s.bind(("", CLIPORT))
    data = bytearray(DATA_SIZE)
    data[0] = 219
    with lock:
        s.sendto(data, ('255.255.255.255', SRVPORT))
    logging.debug("Sent hello to broadcast")
    keepalive_thread = threading.Thread(target=keepalive, args=(s,))
    keepalive_thread.start()
    while True:
        data, addr = s.recvfrom(DATA_SIZE + 8)
        handle_thread = threading.Thread(target=handle, args=(data, addr, s))
        handle_thread.start()
        