"""
Dictionnaires (Equivalent aux tableau associatif paire clés valeurs
"""

# Cration Methode 1
nouvelle_campagne = {
    "responsable_de_campagne": "Jeanne d'Arc",
    "nom_de_campagne": "Campagne nous aimons les chiens",
    "date_de_début": "01/01/2020",
    "influenceurs_importants": ["@AmourDeChien", "@MeilleuresFriandises"],
}

# Creation Methode 2
taux_de_conversion = {}
taux_de_conversion["facebook"] = 3.4
taux_de_conversion["instagram"] = 1.2
taux_de_conversion = dict()

# Interpolation (Variable dynamique) possible pour les clés & valeur
company = "CIMPA"
project = "3D Jump"

my_dict = {
    "company": "{company}",
    "project": "{project}",
    "members": ["Marina, Abde, Marie"],
}

key_interpolation = my_dict["company"].format(company=company)
value_interpolation = my_dict["film"].format(project=project)

# Supprimer une valeur
del my_dict["company"]

# Accéder au valeurs
nouvelle_campagne["responsable_de_campagne"]

# Check if in dict
if "project" in my_dict:
    print("The project key exists in the dictionary")
else:
    print("The project key does not exist in the dictionary")

# Access All keys and valus in Dic
print(my_dict.keys())
print(my_dict.values())

# Iterer sur dic
for key, value in my_dict.items():
    print(key, value)
