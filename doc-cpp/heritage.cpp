// g++ est un compilateur comme gcc mais pour cpp il peut  prendre plusieurs
// paramètres: g++ -std=c++17 -Wall -Wextra -Werror base.cpp -o prog

#include <iostream> //Iostream est une lib contenant des fonctions comme cout etc..
#include <string> // Lib permettant le gestion des strings
#include <vector> // Lib permettant la gestion des vecteur (tableaux dynamique)

using namespace std;

// Classe chef
class Chef{
     public:

          string name;
          int age;

          Chef(string name, int age){
               this->name = name;
               this->age = age;
          }

          void makeChicken(){
               cout << "The chef makes chicken" << endl;
          }

          void makeSalad(){
               cout << "The chef makes salad" << endl;
          }

          void makeSpecialDish(){
               cout << "The chef makes a special dish" << endl;
          }
};

// Class Chef italien hérite de chef
class ItalianChef : public Chef{
     public:

          string countryOfOrigin;
          
          // Constructeur du chef italien ici on appel le constructeur du Chef avec : Chef(name, age) pour ne pas le réecrire
          ItalianChef(string name, int age, string countryOfOrigin) : Chef(name, age){
               this->countryOfOrigin = countryOfOrigin;
          }

          void makePasta(){
               cout << "The chef makes pasta" << endl;
          }

          // override methode (spéciélité chef italien)
          void makeSpecialDish(){
               cout << "The chef makes chicken parm" << endl;
          }
};


int main()
{
Chef myChef("Gordon Ramsay", 50);
myChef.makeChicken();

ItalianChef myItalianChef("Massimo Bottura", 55, "Italy");
myItalianChef.makeChicken();
cout << myItalianChef.age << endl;
}
