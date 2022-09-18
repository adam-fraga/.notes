# Gestion d'erreur

ageUtilisateur = input("Quel age as-tu?")

try:
    ageUtilisateur = int(ageUtilisateur)
except Exception as e:
    print("L'âge indiqué est incorrect")
    raise e

except ZeroDivisionError as e:
    print("Vous ne pouvez pas diviser par zero")
    raise e

except ValueError as e:
    print("Vous devez entrer un nombr")
    raise e

else:
    print("Tu as", ageUtilisateur, "ans")
finally:
    print("Fin du programme")  # finally est éxécuter quoi qu'il arrive

# Lever une exception
age = int(age)

if age < 25:
    raise InventoryError("Mon message d'erreur")

# Assertion
try:
    age = input("Quel age as-tu?")
    age = int(age)

    assert age > 25  # Je veux que age soit superieur a 25
except AssertionError:
    print("J'ai attrapper l'exception")

# Is instance et type permettent d'effectuer des ferification sur le type de l'objet
# La différence entre les 2 est que issinstance() prends en compte l'héritage
# isinstance prend en premier parametre la variable et en second le type d'objet a verifier

x = isinstance("Hello", (float, int, str, list, dict, tuple))


class myObj:
    name = "John"


y = myObj()

x = isinstance(y, myObj)
