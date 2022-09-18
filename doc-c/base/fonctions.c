#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    //Fonctions
    int posX;

    int mafonction(int posX)
    {
        posX = 150;
        return posX;
    }

    positionX = mafonction;

    /*Attention si la fonction est écrite après la fonction main à bien la prototypé avant le main inutile de rappelé le nom de la variable en parametre dans
    le prototype seul le typage de la valeur de retour et des parametres sont nécéssaire. */

    int mafonction(int);

    /*Le mot clés static dans le cas d'une fonction autorise l'utilisation de
    celle ci uniquement dans le fichier ou elle se trouve. */

    static int mafonction(int posX)
    {
        posX = 150;
        return posX;
    }
    return 0;
}

return 0;
}