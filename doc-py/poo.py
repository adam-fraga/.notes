# Définir une class
class Humain:
    # Attribut de class le même pour tout les objet
    humains_crees = 0
    lieu_habitation = "Terre"

    # Constructeur
    def __init__(self, prenom_humain, age_humain=20):
        print("Création d'un humain")
        self.prenom = prenom_humain
        self.age = age_humain

        # On appel la class car l'attribut de class lui est propre
        Humain.humains_crees += 1

    # Methode
    def parler(self, message):
        print("{} a dit : {}".format(self.prenom, message))

    # Methode de class
    def changer_planete(self, cls, nouvelle_planete):  # cls = methode de class
        Humain.lieu_habitation = nouvelle_planete

    # Ajouter cette ligne a l'exterieur de la method
    cls = classmethod(changer_planete)

    # Methode static

    def definition(self):
        print("L'Humain est classé au sommet de la chaine alimentaire...")

    # definition = staticmethod(definition) -> revoir methode static
    # comprehension flou


# Instancier un objet
h1 = Humain("Adam", 30)

# Accès aux attributs globaux des objets
print("Prénom de h1 -> {}".format(h1.prenom))
print("Age de h1 -> {}".format(h1.age))

# Accès aux attributs de class
print("Nombres d'humains crées -> {}".format(Humain.humains_crees))

# Accès à une méthode
h1.parler("Bonjour à tous :)")

# Accès à une méthode de class
print("Planete actuelle -> {}".format(Humain.lieu_habitation))
# Humain.changer_planete("Pluton") Revoir -> Maubaise comprehension
print("Planete après change planete -> {}".format(Humain.lieu_habitation))

# Appel Methode static
Humain.definition
