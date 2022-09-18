#include <stdio.h>
#include <stdlib.h>

/*Encore une fois Typedef permet un meilleur usage des enumération,
 on a ainsi un type Boolean */
typedef enum Boolean
{
    FALSE,   //La premiere enum vaut 1
    TRUE,    //La seconde vaut 2
    AUTRENUM //etc...
} Boolean;

/* Les unions s'utilise généralement dans des structures */
union nombre
{
    int entier;
    float decimal;
};

int main()
{
    //Les unions permettent d'optimiser les espaces mémoire dans certains cas
    union nombre nb1;
    nb1.entier = 3;
    nb1.decimal = 3.14;

    Boolean Mybool = TRUE;

    if (Mybool == TRUE)
    {
        printf("Les enumération permettent de faciliter le développement");
    }

    return 0;
}
