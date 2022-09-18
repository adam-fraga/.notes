# Les Propriété permettent l'encapsulation des données
class Humain:
    """Cette classe réprésente un Humain"""

    def __init__(self, nom, age):
        self.nom = nom
        self._age = age

    # Encapsule l'attribut et interdit dans le cas présent la récuperation
    def _getage(self):
        try:
            return self._age
        except AttributeError:  # Apprendre les AttributeError index etc...
            print("L'age n'éxiste pas")

    # Permet de controller les saisies
    def _setage(self, nouvel_age):
        if nouvel_age < 0:
            self._age = 0
        else:
            self._age = nouvel_age

    def _delage(self):
        del self._age

    # property(<getter>,<setter>,<deleter>,<helper>)
    # Les paramètre doivent respecter cette ordre!!!
    age = property(_getage, _setage, _delage, "Je suis l'âge d'un Humain")


h1 = Humain("Adam", 30)
h2 = Humain("Osef", 34)
h3 = Humain("Yoda", 22)

# Récupère l'age depuis le getter
print(h1.age)

# Modifie l'age avec le setter
print(h2.age)
h1.age = -5
print(h2.age)

# Supprime l'attribut age
print(h2.age)
del h2.age
print(h2.age)

# Helper
help(Humain)
help(Humain.age)
