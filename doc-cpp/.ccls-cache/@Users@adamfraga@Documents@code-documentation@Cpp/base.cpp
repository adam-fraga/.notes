// g++ est un compilateur comme gcc mais pour cpp il peut  prendre plusieurs
// paramètres: g++ -std=c++17 -Wall -Wextra -Werror base.cpp -o prog

#include <iostream> //Iostream est une lib contenant des fonctions comme cout etc..
#include <string>

int main() // Comme en C la fonction main demarre tout programme Cpp
{
  /* cout affichage standard (Memoire tampon)
     std::cerr erreur (affichage direct)
     std::clog journalisation (Memoire tampon)
     std::endl retour a la ligne et flush (vide le tampon)
     std::flush vide le tampon manuellement

  << et >> sont des operateurs de redirection

  std est un namespace contenant des fonction se trouvant dans la lib standart
  iostream
*/

  std::cout << "Choisissez un nombre entier!" << std::endl;

  int number;

  std::cin >> number;

  std::cout << "Vous avez choisie le " <<  number << std::endl;

  return 0;
}
