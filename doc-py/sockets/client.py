# coding:utf-8

import socket as so

host, port = ("localhost", 5566)

try:
    socket = so.socket(so.AF_INET, so.SOCK_STREAM)
    socket.connect((host, port))
    print("Client connecté!")

    data = "Bonjour à toi je suis le client! :)"
    data = data.encode("utf-8")
    socket.sendall(data)

except ConnectionRefusedError:
    print("La connexion au serveur a échoué")
finally:
    socket.close()
