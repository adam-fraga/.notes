# Importer des modules
from requests import get

# Variables
livre = "Harry potter"
nombre = 23
nombre_flottant = 3.4
booleen = True

# Listes (équivalent aux tableaux)
plateformes_sociales = ["Facebook", "Instagram", "Snapchat", "Twitter"]
# Methode pour ajouter, supprimet et trier
plateformes_sociales.append("TikTok")
plateformes_sociales.remove("Snapchat")
plateformes_sociales.sort()

# Les mot clés
# del Supprimer un élément d'un dictionnaire
del plateformes_sociales["facebook"]

# in pour vérifier si est présent dans le dicionnaire
"poids" in infos_labradoodle

# Conditions
avec_soleil = True
en_semaine = False

if avec_soleil and not en_semaine:
    print("on va à la plage !")
elif avec_soleil and en_semaine:
    print("on va au travail !")
else:
    print("on reste à la maison !")

# Les boucles

# For
races_de_chien = ["golden retriever", "chihuahua", "terrier", "carlin"]
for chien in races_de_chien:
    print(chien)

# For avec la fonction range
for x in range(5):
    print(x)

# While
capacite_maximale = 10
capacite_actuelle = 3
while capacite_actuelle < capacite_maximale:
    capacite_actuelle += 1


# Fonctions
def add(a, b):
    return a + b
