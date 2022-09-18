#include <iostream>
#include <string>

// Une alternative à l'overloading est de définir une fonction template permettant son utilisation avec
// Tout les types existant incluants les objets...
// Il peut toutefois être utile d'overload des fonctions template comme pour les tableaux qui par éxemple sont
// Des data structure spécifique en CPP et ne fonctionnerons car on ne peux pas assigné un tableau à un tableau

template <typename T>
void swap(T &a, T &b)
{
  T temp = a;
  a = b;
  b = temp;

  std::cout << "a: " << a << "\tb: " << b << std::endl;
}

int main (int argc, char *argv[])
{
  int a = 3;
  int b = 7;
  
  swap(a, b);

  std::string c = "Adam";
  std::string d = "Fraga";
  
  swap(a, b);

  return 0;
}
