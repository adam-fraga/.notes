"""
    Une matrice est une liste de liste (Tableau a plusieurs dimension)
    On préferera généralement utiliser la librairie numpy pour les calculs de matrice
"""

# Déclaration d'une matrice

matrice = [
    [1, 2, 3],
    [1, 2, 3],
    [1, 2, 3],
]

# Remplir une matrice par liste de comprehension
matrice = []
size = 5
listSize = [1, 2, 3, 4, 5]

# Avec range pour un entier
for x in range(size):
    matrice.append([i for i in range(size)])

# Pour chaque élément d'une liste
for x in listSize:
    matrice.append([i for i in listSize])

# Acceder au nombre de ligne ou au nombre de colonne d'une matrice
row = len(matrice)
col = len(matrice[0])

# Parcourir une matrice (Parcour chaque ligne x, parcourt chaque colonne (y) de de chaque ligne (x) (donc chaque élément)
for x in matrice:
    for y in x:
        print(y)

"""
    VOCABULAIRE
    Matrice carré : matrice dont le nombre de line est = au nombre de colonne
    Matrice ligne : matrice possédant une seule ligne
    Matrice colonne : matrice possedant une seule colonne = vecteur colonne
    Matrice null :C'est une matrice dont tout les élément sont nuls

    2 Matrices sont égale si:
        Elles ont le même nombre de ligne
        Elles ont le même nombre de colonne
        Pour des corrdonées i, j l'élément est le même dans les 2 matrices.

    Matrice identité:
        Comporte le même nombre de ligne que de colonne
        Pour chaque élément matrice[i][j] vaut 1 si i = j vaut 0 si i différent de j
"""
