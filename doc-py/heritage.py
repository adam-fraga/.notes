# Class mère
class Vehicule:
    def __init__(self, nom, quantitee_carburant):
        self.nom = nom
        self.carburant = quantitee_carburant

    def se_deplacer(self):
        print("Le véhicule {} se déplace".format(self.nom))


# Class fille
class Voiture(Vehicule):
    # On appel le constructeur parent pour ébiter de redéfinir les attributs et methode
    def __init__(self, nom_voiture, essence, puissance):
        Vehicule.__init__(self, nom_voiture, essence)
        self.puissance = puissance


class Avion(Vehicule):
    def __init__(self, nom_avion, fuel, marchandise):
        Vehicule.__init__(self, nom_avion, fuel)
        self.marchandise = marchandise

    # Surchager une methode pour la réécrire
    def se_deplacer(self):
        print("L'avion {} s'envole".format(self.nom))


vehicule = Vehicule("F22 Raptor", 3400)
voiture = Voiture("Toyota Supra", 32, 420)
avion = Avion("F22", 2000, 100)

voiture.se_deplacer()
avion.se_deplacer()

"""----------------FONCTIONS UTILES------------------"""

# Verifier si un objet est de la classe renseigné
print(isinstance(avion, Vehicule))

# Verifier qu'une classe hérite d'une autre
print(issubclass(Avion, Vehicule))
