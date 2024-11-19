// En cpp les structure sont quasiment comme les classes à une différence près 
// Les structure on par default leur attribut en public et les class l'ont en privé
// Les structure sont utilisé en Cpp comme des data storage et ne contiennent par convention pas de méthode
// on les appel PODS voir documentation
// On préférera donc utiliser des class pour des objet contenant des attr et des methodes
#include<iostream>
#include<string>

struct Mastructure
{
    // Par default public
    std::string name;
    std::string firstname;
    std::string status;

    private:
    int x;
    int y;
 
};

int main()
{
  Mastructure mastruct;

  mastruct.name = "FRAGA";
  mastruct.firstname = "Adam";
  mastruct.status = "student";

  std::cout << "First name: "<< mastruct.firstname << std::endl;
}
