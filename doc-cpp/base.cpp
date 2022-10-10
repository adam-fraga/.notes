// paramètres: g++ -std=c++17 -Wall -Wextra -Werror base.cpp -o prog

#include <iostream> // Lib contains functions like cout etc..
#include <string> // Lib allow string management
#include <vector> // Lib allow vector usage
#include <climit> // Contains function that show size of int, short, long etc...

using namespace std; // Namespace contains standard library functions

// Function in CPP must be prototyped on top of main function
// It's possible to prototype function behind main but you must declare the signature on the top.
// signature = first top line of a prototype without brackets

int addNumbers(int nb1, int nb2)
  {
    return nb1 + nb2;
  }

int main() // Main is the entrypoint
{

  /* cout standard display (Buffer memory)
     std::cerr err (direct display)
     std::clog journalisation (Buffer memory)
     std::endl backline & flush (Remove Buffer)
     std::flush remove buffer manualy

  << & >> Redirection operators
  */

  cout << "Hello ";
  cout << "World";
  cout << "!" << endl;

  // TYPES DE VARIABLES

  // String provide by string library are not primitives
  string name = "Florent";
  char c = 'A';  // char 8bits
  
  // Number
  
  signed short nb = 1;   // minimum 16bits signed int
  unsigned int nb2 = 30; // minimum 16bits signed int (not smaller than short)
  long nb3 = 40;         // minimum 32bits signed int
  long long nb4 = 50;    // minimum 64bits signed int
  
  float nb5 = 2.5f;     // Single percision floating point
  double nb6 = 2.5;     // Double percision floating point
  long double nb7 = 2.5;// extended percision floating point

  //Sizeof return amount of bits taken in memory
  cout << sizeof(nb4) << endl;

  bool isTall;
  isTall = true; // 1bit -> true/false
  
  int const BIRTHDAY = 1991;

  // Casting

  cout << (int)3.14 << endl;
  cout << (double)3/2 << endl;

  // Pointers

  int num = 10;

  cout << &num << endl; // Display address
  
  int *pNum = &num; // * to declare pointer

  cout << pNum << endl; // Display pointer address
  cout << *pNum << endl; // Display the pointed value (dereferencing)

  // STRINGS

  string greeting = "Hello";

  cout << greeting.length(); // Length of the string
  cout << greeting[0] << endl; // display index 0
  cout << greeting.find("llo") << endl; // Display llo or bgin the sequence
  cout << greeting.substr(2) << endl; // Return characters after the second chars
  cout << greeting.substr(1, 3) << endl; // Return chars between first and third chars

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

  // Reserve an array of 9 element
  int array = [9];
  luckyNumbers[0] = 90;
  cout << luckyNumbers[0] << endl;
  
  // Ndim array
  int numberGrid[2][3] = {
    {1, 2, 3},
    {4, 5, 6}
  };

  //OR

  numberGrid[0][1] = 99;

  // VECTOR

  vector<string> friends; // Dynamic array of string

  friends.push_back("Oscar");
  friends.push_back("Adam");
  friends.push_back("Florent");
  friends.push_back("Angela");

  // friends.begin return the pointer of the first array element
  friends.insert(friends.begin() + 1 "Jim"); // The number is required to specify the emplacement of the specified string
  friends.insert(friends.begin() + 3 "John");
  friends.erase(friends.begin() + 3); // Delete the third element


  // Display value
  cout << friends.at(0) << endl;
  cout << friends.at(3)  << endl;
  cout << friends.size() << endl;

  //FONCTIONS
 
  // Function define on top of main
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

for (size_t i = 0; i < length; i++) {
 std::cout << "message" << endl; 
}
