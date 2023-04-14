# Les tuples sont des conteneur immuable (constantes)

mon_tuple = (45, 64)
print(mon_tuple[1])

# Déclaration multiple
(nombre1, nombre2) = (21, 46)


# Retour multiple de fonction
def get_player_position():
    posX = 148
    posY = 86

    return (posX, posY)


coordX, coordY = get_player_position()
