#include <iostream>
#include <string>

// les namespace s'implémente dans les fichier d'entêtes .h et dans les fichier de définition de nos fonctions .cpp
// on préferera appelée les namespace avec la syntaxe adam::

namespace adam {
    // fonctions du namespace adam:
    void mafonction()
  {
    std::cout << "josef" << std::endl;
  }

}

using namespace adam;

int main (int argc, char *argv[])
{
  // OU
  adam::mafonction();
  return 0;
}
