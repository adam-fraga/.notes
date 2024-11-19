#include <algorithm>
#include <iostream>
#include <string>

void work(int x)
{
  x++;
}

int main (int argc, char *argv[])
{
  int a = 10;
  // b est une référence à a
  int &b = a;
  int c = 100;
  std::cout << a << std::endl << b << std::endl;
  // & permet d'afficher l'adresse mémoire d'une variable
  std::cout << &a << std::endl << &b << std::endl;
  // En changeant la valeur de a la valeur de b étant la référence  de a b affiche 15
  a = 15;
  std::cout << a << std::endl << b << std::endl;
  // En changeant la valeur de b on modifie la valeur définit dans a et donc 15 deviens 20
  b = 20;
  std::cout << a << std::endl << b << std::endl;

  // les référence permettent d'alterer des data strucutre à l'interieur de fonction
  // Celle ci faisant référence à l'emplacement mémoire on peux y alterer leur valeur 
  // COntrairement au comportement par defaut ou les fonctions récupere une copie des 
  // Data structure passé en paramètres
  std::cout << a << std::endl << b << std::endl;
  // Ici à sera incrémenté car la variable est passé par référence
  work(&a);
  std::cout << a << std::endl << b << std::endl;

  return 0;
}
