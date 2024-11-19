#include<iostream>
#include<string>

// L'overloading d'operateur consiste a réecrire les comportant de base des différents opérateur
// du langage dans le cadre de nos objets et permet ainsi de créer des comportement puissant pour nos objets 
// lors de leurs interactions.

class Position {
  public:
      int x = 10;
      int y = 30;
      // On spécifie ici que pour l'operateur + vas être "overload" (réecrit) pour l'addition de not objets
      //cette methode est appelé et l'élément à additionnée est passé en paramètre
      Position operator + (Position pos) // l'objet passé en paramètre correspond ici à l'objet situé à droite de l'operateur
      {
        Position new_pos;      // Nouvelle objet à retourné (resultat)
        new_pos.x = x + pos.x; // x fait référence à l'attribut x de pos1 car ce contexte est celui de pos1 
                               //& pos.x est l'attribut x de pos2 qui est passé en paramètre de la méthode
        new_pos.y = y + pos.y;

        return new_pos;
      }
      // On overload l'operateur de stricte égalité dans le cadre de notre objet
      bool operator == (Position pos) // parametre pos est celui a droite de l'operateur ==
      {
        if(x == pos.x && y == pos.y) // x et y sont les attribut de l'objet à gauche de ==
                                    // pos.x et pos.y sont donc les attribut de l'objet passé en pparamètre       
        {
          return true;
        }
        return false;
      }
};


int main (int argc, char *argv[])
{

   Position pos1, pos2; 
   // Pardefault le complilateur ne sait pas calculer nos 2 objets
   Position pos3 = pos1 + pos2;

    std::cout << pos3.x << " " << pos3.y << std::endl;
   // Quand on redéfinit un operateur avec la methode operator le parametre passé est celui à droite, 
   // Dans la methode on se trouve dans le contexte de l'objet a gauche de l'operateur
    if(pos1 == pos2)
    {
    std::cout << "Les 2 objets sont bien égaux\n";
    }

  return 0;

}
