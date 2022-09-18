// g++ est un compilateur comme gcc mais pour cpp il peut  prendre plusieurs
// paramètres: g++ -std=c++17 -Wall -Wextra -Werror base.cpp -o prog

#include <iostream> //Iostream est une lib contenant des fonctions comme cout etc..
#include <string> // Lib permettant le gestion des strings
#include <vector> // Lib permettant la gestion des vecteur (tableaux dynamique)

using namespace std;

double division(int a, int b) {
   if( b == 0 ) {
      throw "Division by zero error!";
   }
   return (a/b);
}

int main(){

  // EXCEPTION CATCHING
     try {
        division(10, 0);
     } catch (const char* msg) {
      cerr << msg << endl;
     }

}


