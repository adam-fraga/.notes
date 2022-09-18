# Les listes

# Listes (équivalent aux tableaux)
plateformes_sociales = ["Facebook", "Instagram", "Snapchat", "Twitter"]

maliste = []
maliste2 = ["arc"] * 10
maliste3 = range(20)

i = 0

# Parcourir une liste
while i < len(maliste2):
    print(maliste2[i])
    i += 1
# Parcourir avec range
for valeur in maliste3:
    print(valeur)

# Conditions sur liste
if "arc" in maliste2:
    print("Je possède un arc")

# Afficher tout les éléments
print(maliste3[:])

# Affiche les 2 premiers éléments
print(maliste3[:2])

# Affiche les dernier élément après le 3eme
print(maliste[3:])

# Affiche les élément compris entre les index
print(maliste3[2:5])

# Methode pour ajouter,inserer a un indice, supprimet et trier
plateformes_sociales.append("TikTok")
plateformes_sociales.remove("Snapchat")
del plateformes_sociales[2]
plateformes_sociales.sort()
plateformes_sociales.insert(2, "Linkedin")

# Syntaxe de comprehension de liste
# Elle permet de créer une nouvelle liste depuis une liste éxistante et ainsi
# de pouvoir modifier les index de cette liste en les remplaçant par d'autres,
# en leur appliquant des opération mathematique ou en les supprimant
# L'éxemple ci dessous montre comment parcourir une liste et supprimer les
# nombres  pairs grace à la comprehension de liste.

mylist = [1, 4, 7, 8, 20]

newlist = [x for x in mylist if x % 2 == 0]
print(newlist)

# Afficheera donc
[4, 8, 20]

# Liste multidimensionnel ou matrices
# Toujours se rappeler que dans le cadre d'un tableau à 2 dimensions
# Le premier index représente les lignes et le second les colonne
# Pour acceder à l'indice ligne 3 colonne 2 on fera donc
maliste[2][1]
