"""
Dictionnaires (Equivalent aux tableau associatif paire clés valeurs
Pour accéder au valeur d'un dictionnaire on utilise cette syntaxe:
mon_dictionnaire[clés_de_mon_dictionnaire]
"""

# Methode 1
nouvelle_campagne = {
    "responsable_de_campagne": "Jeanne d'Arc",
    "nom_de_campagne": "Campagne nous aimons les chiens",
    "date_de_début": "01/01/2020",
    "influenceurs_importants": ["@AmourDeChien", "@MeilleuresFriandises"],
}

# Methode 2
taux_de_conversion = {}
taux_de_conversion["facebook"] = 3.4
taux_de_conversion["instagram"] = 1.2
taux_de_conversion = dict()
