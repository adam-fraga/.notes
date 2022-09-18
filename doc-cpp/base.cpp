// g++ est un compilateur comme gcc mais pour cpp il peut  prendre plusieurs
// paramètres: g++ -std=c++17 -Wall -Wextra -Werror base.cpp -o prog

#include <iostream> //Iostream est une lib contenant des fonctions comme cout etc..
#include <string> // Lib permettant le gestion des strings
#include <vector> // Lib permettant la gestion des vecteur (tableaux dynamique)
#include <climit> // Permet d'utiliser des fonction pour voir le nombre de place que prend un int, short, long etc...
/*
   std est un namespace contenant des fonction se trouvant dans la lib standart on peux l'appeler avant les fonctions
  de la librairie stantard ou l'appelé au dessus du fichier.
*/

using namespace std;

// Pour rendre la fonction fonctionnel dans main on la prorotype au dessus
// Il lest possible de l'appelé après en déclarant la signature de la fonction au dessus
// La signature est literralement la première ligne du prototype avec un semi column a la fin

int addNumbers(int nb1, int nb2)
  {
    return nb1 + nb2;
  }

int main() // Comme en C la fonction main demarre tout programme Cpp
{

  /* cout affichage standard (Memoire tampon)
     std::cerr erreur (affichage direct)
     std::clog journalisation (Memoire tampon)
     std::endl retour a la ligne et flush (vide le tampon)
     std::flush vide le tampon manuellement

  << et >> sont des operateurs de redirection

  */
  cout << "Hello ";
  cout << "World";
  cout << "!" << endl;

  // TYPES DE VARIABLES

  // La librairie <string> permet l'utilisation de string ce ne sont pas des types primitifs
  string name = "Florent";
  char c = 'A';  // caractere 8bit
  
  // NOMBRES
  //Possible de mettre unsigned pour les négatif sachant que la valeur maximal en est impacté
  short nb = 1;         // minimum 16bits signed int
  int nb2 = 30;          // minimum 16bits signed int (not smaller than short)
  long nb3 = 40;         // minimum 32bits signed int
  long long nb4 = 50;    // minimum 64bits signed int
  
  // decimal
  float nb5 = 2.5f;     // Single percision floating point
  double nb6 = 2.5;     // Double percision floating point
  long double nb7 = 2.5;// extended percision floating point

  //Sizeof renvoi le nombre de bit que prend une variable en mémoire
  cout << sizeof(nb4) << endl;

  // bool
  bool isTall;
  isTall = true; // 1bit -> true/false
  
  // constantes
  int const BIRTHDAY = 1991;

  // casting

  cout << (int)3.14 << endl;
  cout << (double)3/2 << endl;

  // pointers
  int num = 10;

  cout << &num << endl; // Afficher l'adresse da la variable
  
  int *pNum = &num; // Pour stocker un pointeur (adresse) on utilise * et on indique le type du pointeur

  cout << pNum << endl; // Afficher l'adresse du pointeur
  cout << *pNum << endl; // Afficher la valeur contenu à l'adresse (déréférence le pointeur)

  // STRINGS

  string greeting = "Hello";

  cout << greeting.length(); // Longueur de la chaine
  cout << greeting[0] << endl; // récup index
  cout << greeting.find("llo") << endl; // Si llo est présent retourne l'index ou demarre la séquence 
  cout << greeting.substr(2) << endl; // Retourne les caractère après le 2eme index (après e)
  cout << greeting.substr(1, 3) << endl; // Retourne les caractere entre le 1er et le 3eme index (ell)

  // USERS INPUT

  string user_name;

  cout << "Entrez votre nom!" << endl;
  cin >> user_name;
  cout << "Salut " <<  user_name << endl;

  int num2, num3;

  cout << "Entrez un nombre!" << endl;
  cin >> num2;
  cout << "Entrez un autre nombre!" << endl;
  cin >> num3;

  cout << "Vous avez choisie " <<  num2 << " et " << num3 << endl;
  cout << "Leur somme vaut " <<  num2 + num3 << endl;

  // ARRAYS

  int luckyNumbers[] = {4, 8, 15, 16, 23, 42};
  // Ou on donne le nombre d'element qu'on souhaite stocker et l'allocation mémoire se fait pour ici 9 element
  int array = [9];

  luckyNumbers[0] = 90;
  cout << luckyNumbers[0] << endl;
  
  // Ndim array
  int numberGrid[2][3] = {
    {1, 2, 3},
    {4, 5, 6}
  };
  //ou

  numberGrid[0][1] = 99;

  // VECTOR
  vector<string> friends; // créer un tableau dynamique de string

  friends.push_back("Oscar");
  friends.push_back("Adam");
  friends.push_back("Florent");
  friends.push_back("Angela");

  // friends.begin renvoi le pointeur de début du tableau 
  friends.insert(friends.begin() + 1 "Jim"); //En indiquant ensuite un nombre on insert la valeur à la position souhaité
  friends.insert(friends.begin() + 3 "John");
  friends.erase(friends.begin() + 3); // Supprime la personne présente à l'index 3 donc John


  // Afficher les valeur dans un vecteur
  cout << friends.at(0) << endl;
  cout << friends.at(3)  << endl;
  cout << friends.size() << endl;

  //FONCTIONS
  
  // La fonction est définit au dessus du main
  int sum = addNumbers(12, 43);

  cout << sum << endl;

  // CONDITIONS

  bool isStudent = false;
  bool isSmart = false;

  if(isStudent && isSmart){
       cout << "You are a student" << endl;
  } else if(isStudent && !isSmart){
       cout << "You are not a smart student" << endl;
  } else {
       cout << "You are not a student and not smart" << endl;
  }

  // >, <, >=, <=, !=, ==
  if(1 > 3){
       cout << "number omparison was true" << endl;
  }

  if('a' > 'b'){
       cout << "character comparison was true" << endl;
  }

  string myString = "cat";
  if(myString.compare("cat") != 0){
       cout << "string comparison was true" << endl;
  }

  // SWITCH STATEMENTS
  char myGrade = 'A';
  switch(myGrade){
       case 'A':
            cout << "You Pass" << endl;
            break;
       case 'F':
            cout << "You fail" << endl;
            break;
       default:
            cout << "Invalid grade" << endl;
  }

  // LOOPS

  int index = 1;
  while(index <= 5){
       cout << index << endl;
       index++;
  }

  do{
       cout << index << endl;
       index++;
  }while(index <= 5);

  return 0;
}
