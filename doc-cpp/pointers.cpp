#include <iostream>
#include <string>

void change(int *x)
{
  *x = 10;
}

int main (int argc, char *argv[])
{
  // La principale différence entre un pointeur et une référence en CPP est que l'on peux modifier
  // à tout moment la référence d'un pointeur comme ci dessous, la ou une référence référencera toujours
  // Le même emplacement mémoire.
  int x = 5;
  int *y = &x;
  x = 6;
  *y = 8;
  int z = 10;
  y = &z;
  x = 5;
  *y = 60;

  std::cout << &x << std::endl;
  std::cout << x << " "<< *y << " " << z << std::endl;
  
  // new permet d'aller de la mémoire dynamiquement sur la hipe (ici pour un int valant 5)
  // new renvoi un pointeur qu'on stock logiquement dans un pointeur sur int qui va ici pointer sur 5
  int *w = new int(5);
  std::cout << "W avant: "<< *w << std::endl;
  //On passe le pointeur en parametre de la fonction change et celle ci permet d'alterer la valeur 5 stocké en mémoire
  change(w);
  std::cout << "W après: "<< *w << std::endl;
  delete(w);
  return 0;
}
