import socket, time, threading, logging

class Robot():

    def __init__(self):
        self._host = '172.25.100.221'
        self._cliport = 8080
        self._srvport = 8081
        self._timeout = 30
        self._jersey = 1
        self._keepalive = 0
        self._server = ()
        self._lock = threading.Lock()
        self._data_size = 508
        self._socket = None

        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s: 
            s.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
            s.bind(("", self._cliport))
            self._socket = s
            data = bytearray(self._data_size)
            data[0] = 219
            with self._lock:
                self._socket.sendto(data, ('255.255.255.255', self._srvport))
            logging.debug("Sent hello to broadcast")
            keepalive_thread = threading.Thread(target=self.keepalive)
            keepalive_thread.start()
            while True:
                data, addr = s.recvfrom(self._data_size + 8)
                handle_thread = threading.Thread(target=self.handle, args=(data, addr))
                handle_thread.start()

    def handle(self, d, addr):
        s = self._socket
        if not self._server:
            self._server = ('0.0.0.0', self._srvport)
        data = bytearray(d)
        cmd = data[0]
        logging.debug(f"Received {cmd} from {addr}")
        ret = bytearray(self._data_size)
        
        # Handle server ACK
        if cmd == 250:
            logging.debug(f"Received server ack")

        # Handle server hello
        elif cmd == 220:
            ret[0] = 250
            with self._lock:
                self._server = (addr[0], self._srvport)
                logging.debug(f"Received hello from {self._server}")
                self._keepalive = time.time()
                s.sendto(ret, self._server)

        elif cmd == 222:
            ret[0] = 221
            ret[1] = self._jersey
            with self._lock:
                self._server = (addr[0], self._srvport)
                print(self._server)
                logging.debug(f"Provided jersey number {self._jersey} to {self._server}")
                self._keepalive = time.time()
                s.sendto(ret, self._server)

        elif cmd == 125:
            ret[0] = 250
            data.pop(0)
            raw = bytes(data)
            logging.debug(f"Received {raw}")
            with self._lock:
                self._keepalive = time.time()
                s.sendto(ret, self._server)

        elif cmd == 255:
            ret[0] == 250
            logging.debug(f"Received server stop")
            with self._lock:
                self._keepalive = time.time()
                s.sendto(ret, self._server)

        else:
            logging.debug(f"Ignoring invalid traffic {cmd} from {addr}")

    def keepalive(self):
        s = self._socket
        while True:
            if self._server == ():
                time.sleep(self._timeout / 6)
                continue
            with self._lock:
                if time.time() - self._keepalive > 30:
                    data = bytearray(self._data_size)
                    data[0] = 100
                    s.sendto(data, self._server)
                    logging.debug(f"Sent keepalive")
            time.sleep(self._timeout / 6)

logging.basicConfig(format='%(asctime)s %(message)s', level=logging.DEBUG)

r = Robot()
        