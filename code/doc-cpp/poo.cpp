// g++ est un compilateur comme gcc mais pour cpp il peut  prendre plusieurs
// paramètres: g++ -std=c++17 -Wall -Wextra -Werror base.cpp -o prog

#include <iostream> //Iostream est une lib contenant des fonctions comme cout etc..
#include <string> // Lib permettant le gestion des strings
#include <vector> // Lib permettant la gestion des vecteur (tableaux dynamique)

using namespace std;


// SEMI COLOUMN FIN DE CLASSE On définit habituellement les classes dans un autre fichier
class Book
{    
          // Attribut public private ou protected
     private:
          string label;

     public:
          // Un attribut ou une methode static est propre à la class
          // On ne peux affecter de valeur à un attribut static qu' a l'exterieur de la classe
          static int book_count;
          string title;
          string author;
          
          // Les methode static s'appel avec la syntaxe :: en dehors de la classe et non pas le .
          static int get_book_count()
          {
            return book_count;
          }
        
          // CONSTRUCTEUR
          Book(string title, string author){
               // incrémente le compteur static de livre
               book_count ++;
               this->title = title;
               this->author = author;
          }

          // GETTERS
          string getPrenom()
          {
               return this->label;
          }

          // SETTER
          void setPrenom(string prenom)
          {
               this->label = prenom;
          }
          
          // METHOD
          void readBook(){
               cout << "Reading " + this->title + " by " + this->author << endl; // L'opérateur this comme en php
          }
          // DESTRUCTOR
          ~Book(){
            // En appellant manuellement le destructeur on décrémente le nombre de livre
            book_count --;
          }
};

int main()
{
// Comme il est impossible d'affecter une valeur à un attribut static dans une classe on le fait à l'exterieur
  Book::book_count = 0;
// Instancier un objet (Paramegtre du constructeur)
  Book book1("NIOURK", "Adam");

// Change les attribut de l'objet
  book1.title = "Harry Potter";
  book1.author = "JK Rowling";

// Appel la methode
  book1.readBook();
  cout << book1.title << endl;

  Book book2("Lord of the ring", "JRR Tolken");

  book2.readBook();
  cout << book2.title << endl;

// Affiche le nombre de livre créer grace à l'attribut static incrémenter dans le constructeur
  cout << Book::get_book_count() << endl;
// Décrémente le nombre de livre
  book1.~Book();
  cout << Book::get_book_count() << endl;
}
