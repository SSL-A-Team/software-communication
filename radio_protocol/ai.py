import socket, time, threading, random, logging


class Ai():

    def __init__(self):
        self._host = '172.25.100.221'
        self._srvport = 8081
        self._cliport = 8080
        self._timeout = 30
        self._robots = []
        self._lock = threading.Lock()
        self._data_size = 508
        self._socket = None

        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
            self._socket = s
            self._socket.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
            self._socket.bind(("", self._srvport))
            data = bytearray(self._data_size)
            data[0] = 220
            order_thread = threading.Thread(target=self.giveOrders)
            order_thread.start()
            keepalive_thread = threading.Thread(target=self.checkKeepalives)
            keepalive_thread.start()
            with self._lock:
                self._socket.sendto(data, ('255.255.255.255', self._cliport))
            logging.debug(f"Sent hello to broadcast")
            while True:
                data, addr = s.recvfrom(self._data_size+8)
                handle_thread = threading.Thread(target=self.handleRobot, args=(data, addr))
                handle_thread.start()

    def setKeepalive(self, addr):
        for r in self._robots:
            if r['addr'] == (addr[0], self._cliport):
                with self._lock:
                    r['keepalive'] = time.time()
                logging.debug(f"Set keepalive for {(addr[0], self._cliport)}")

    def handleRobot(self, d, addr):
        data = bytearray(d)
        cmd = data[0]
        ret = bytearray(self._data_size)
        addr = (addr[0], self._cliport)
        logging.debug(f"Got traffic from {addr} {data[0]} {data[1]}")

        # Handle client Hello
        if cmd == 219:
            r = {'addr': addr, 'jersey': 0, 'keepalive': time.time()}
            if r not in self._robots:
                with self._lock:
                    self._robots.append(r)
                logging.debug(f"Added {r} to robot list")
            else:
                logging.debug(f"Got hello for {r} I already knew about")
            ret[0] = 222

        # Handle client jersey number
        elif cmd == 221:
            isIn = False
            for r in self._robots:
                if r['addr'] == addr:
                    with self._lock:
                        r['jersey'] = data[1]
                    logging.debug(f"Set jersey to {data[1]} for {r['addr']}")
                    isIn = True
            if not isIn:
                r = {'addr': addr, 'jersey': data[1], 'keepalive': time.time()}
                with self._lock:
                    self._robots.append(r)
                logging.debug(f"Added new robot {r}")
            fail = False
            for r in self._robots:
                if r['jersey'] == data[1] and r['addr'] != addr:
                    fail = True
                    ret[0] = 255
                    e = bytearray(256)
                    e[0] = 255
                    with self._lock:
                        self._socket.sendto(e, r['addr'])  # Notify conflicting bot
                    logging.debug(f"Found conflicting jersey number {data[1]} between {r} and {addr}")
            if not fail:
                ret[0] = 250
            
        # Handle client ACK
        elif cmd == 250:
            isIn = False
            for r in self._robots:
                if r['addr'] == addr:
                    isIn = True
                    self.setKeepalive(addr)
            if not isIn:
                with self._lock:
                    self._robots.append({"addr": addr, 'journal': 0, 'timeout': time.time()})
                ret[0] = 222
            else:
                logging.debug(f"Got ack from {addr}")
                return  # Do nothing and don't reply

        # Handle client shutdown
        elif cmd == 251:
            for r in self._robots:
                if r['addr'] == addr and r['jersey'] == data[1]:
                    with self._lock:
                        self._robots.remove(r)
            ret[0] = 250
            self.setKeepalive(addr)
            logging.debug(f"{addr} is shutting down")

        # Handle client keepalive
        elif cmd == 100:
            ret[0] = 250
            self.setKeepalive(addr)
            logging.debug(f"Got keepalive from {addr}")

        # Handle unsupported command
        else:
            ret[0] = 254
            self.setKeepalive(addr)
            logging.debug(f"Got unsupported command {cmd} from {addr}")
        
        with self._lock:
            self._socket.sendto(ret, addr)
            logging.debug(f"Sent {ret[0]} to {addr}")

    def giveOrders(self):
        # Wait for init
        while self._robots == []:
            logging.debug("Waiting for robots....")
            time.sleep(1)

        # Send random data
        for j in range(10):
            ret = bytearray(self._data_size)
            ret[0] = 125
            for i in range(1,self._data_size-1):
                ret[i] = random.getrandbits(8)
            with self._lock:
                self._socket.sendto(ret, self._robots[0]['addr'])   

    def checkKeepalives(self):
        while True:
            for r in self._robots:
                if time.time() - r['keepalive'] > 30:
                    with self._lock:
                        self._robots.remove(r)
                    logging.debug(f"Pruned expired bot {r}")
            time.sleep(self._timeout / 6)

logging.basicConfig(format='%(asctime)s %(message)s', level=logging.DEBUG)

a = Ai()