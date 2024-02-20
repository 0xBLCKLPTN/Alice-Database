import socket
from threading import Thread
from parser import parser

class Server:
    def __init__(self, address: str = 'localhost', port: int = 8888) -> None:
        self.s = socket.socket()
        self.s.bind((address, port))
        self.s.listen(5)

    def _on_new_client(self, client_socket: object, addr: str) -> None:
        while True:
            data = client_socket.recv(1024).decode('utf-8')

            if not data:
                break

            res = parser(data)
            client_socket.send(res.encode())
        client_socket.close()
    
    def run(self):
        while True:
            c, addr = self.s.accept()
            print(f"New connection from: {addr}")
            thread = Thread(target=self._on_new_client, args=(c, addr))
            thread.start()
        c.close()
        thread.join()
 #server.shutdown()

Server().run()       
