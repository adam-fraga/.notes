# coding:utf-8
import socket as so

host, port = ('', 5566)

socket = so.socket(so.AF_INET, so.SOCK_STREAM)
socket.bind((host, port))
print("Le serveur est demarré...")

while True:
    socket.listen()
    conn, address = socket.accept()
    print("En écoute...")
    data = conn.recv(1024)
    data = data.decode("utf-8")
    print(data)

conn.close()
socket.close()
