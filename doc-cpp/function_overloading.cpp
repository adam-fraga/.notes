#include <iostream>
#include <ostream>
#include <string>

// L'overloading consiste en la redefinition d'une fonction en gardant son nom et en permettant
// Ainsi l'utilisation de cette même fonction avec un typage différents lorsqu'on appelera celle ci
// Elle sera capable de reconnaitre en fonction du type quel swap doit être appelé

void swap(int &a, int &b)
{
  int temp = a;
  a = b;
  b = temp;

  std::cout << "a: " << a << "\tb: " << b << std::endl;
}

void swap(std::string &a, std::string &b)
{
  std::string temp = a;
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
