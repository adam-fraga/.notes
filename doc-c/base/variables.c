/*Pour compiler on utilise gcc "fic.c" -o "fichierExecutable" puis on l'éxécute
 avec ./fichierExecutable*/

//Directive préprocésseuri incluant la bibliothèque standard obligatoire
#include <stdio.h>

// int correspond au type de donnée retourné par la fonction
int main(void)
{

    //Les types signé peuvent être négatif (-128 à 128) les type non signé sont
    //exclusivement superieur (0 à 255)

    signed char car = 65;
    char carac = 'A';
    unsigned char car2 = 172;
    int entier = 1665;
    float flottant = 6.55957;
    const float PI = 3.14;

    /*Une variable static dans le cadre d'une fonction n'est pas détruite à la
  fin de celle ci */
    static int nb = 5;

    //Les différents flag
    // %d -> int
    // %f -> float et double
    // %c -> char
    // %s -> string
    //L'ordre des variable est important le joker ".4" permet de spécifier le
    //nombre de décimal
    printf("Mon entier: %d \n Mon flottant: %.4f \n Mon caractère: %c \n  Mon autre caractère %c \n", entier, flottant, carac, car);

    //Lire les données au clavier
    int ageUtilisateur = 0;
    printf("Quel âge avez vous?");
    // & -> adresse de la variable on utilise toujours l'adresse pour scanf
    scanf("%d", &ageUtilisateur);
    printf("Vous avez %d ans! \n", ageUtilisateur);

    return 0;
}