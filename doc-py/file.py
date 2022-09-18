"""
Modes d'ouverture: r (lecture seul)
                   w (écriture avec remplacement)
                   a (écriture avec ajout en fin de fichier)
                   x (lecture et écriture)
                   r+ (lecture/écriture dans un même fichier)
"""

#LECTURE DE FICHIER
#Ouvre le fichier
fic = open("data.txt","r")
#Lis le fichier
content = fic.read()
print(content)
#Lire une ligne (un second rappel de readline lira la suivante etc...
line = fic.readline()
line2 = fic.readline()
#Lire toutes les lignes restantes sous forme de liste
lines = fic.readlines()

#ECRIRE DANS UN FICHIER (2 syntaxe avec with ou comme ci dessus)
with open("data.txt","w") as fic:
    nombre = 123
    fic.write(str(nombre))
    fic.write("Salut a tous\n")
    fic.write("Josef Smith\n")

fic.close()

if fic.closed:
    print("Fichier fermé")
else:
    print("Fichier fermer")
