#include <stdio.h>

/*      [VARIABLES]
        variable = valeur de la variable
        &variable = adresse de la variable

        [POINTEURS]
        pointeur = adresse de la variable pointée
        *pointeur = valeur de la variable pointée
        &pointeur = adresse du pointeur
*/

void inverser_nombres(int *nombre1, int *nombre2)
{
    int tmp = 0;

    // Le * indique la valeur de la variable pointée
    printf("valeur du pointeur *nombre1 vaut: %d \n", *nombre1);
    printf("valeur du pointeur *nombre2 vaut: %d \n \n", *nombre2);

    //le & indique l'adresse du pointeur
    printf("adresse du pointeur *nombre1 vaut: %p \n", &nombre1);
    printf("adresse du pointeur *nombre2 vaut: %p \n \n", &nombre2);

    //Le pointeur contient en valeur l'adresse de la variable pointé
    printf("adresse de la variable pointée par pointeur *nombre1 vaut: %p \n", nombre1);
    printf("adresse de la variable pointée par pointeur *nombre2 vaut: %p \n \n", nombre2);

    tmp = *nombre1;
    *nombre1 = *nombre2;
    *nombre2 = tmp;
}


/*Les adresse déclaré sur la stack se suivent en déscendant et sont donc décroissantes.
 Il est possible de se déplacer d'adresses en adresses grace aux pointeurs, 
 le compilateur est assez intelligent pour comprendre qu'un int vaut 4 octets
 ou qu'un char en vaut un ainsi en ajoutant +1 à une adresse on se retrouve une adresse plus loin.
*/

/*Un pointeur sur char est de valeur constante on peut le parser avec une boucler et ainsi le copiermais pas       le modifier. On peut ainsi faire pointer des adresse les unes sur les autreset ainsi retrouver en mémoi      re les chaines de caractère constante stocker à un endroit. Un pointeur sur char pointe sur le premier c      aractère d'une chaine soit str[0] on peux donc itérer sur celui cipour afficher toute la chaine ou plus       simplement avec printf et le flag %s*/



void  arithmetiques_pointeur(void)
 {
       int   a;
       int   b;
       int   *ptr;
 
      a = 6;
      b = 42;
 
      ptr = &b;
 
      printf("Adresse de a: %p \n", (ptr+1));
 
      printf("Adresse de b: %p \n", ptr);
 }

int main(void)
 {
    int nb1 = 15;
    int nb2 = 28;

    // Le flag "%p" permet d'afficher une adresse
    printf("Adresse de nb1 vaut %p \n", &nb1);
    printf("Adresse de nb2 vaut %p \n \n", &nb2);

    printf(" nb1 vaut %d \n", nb1);
    printf(" nb2 vaut %d \n \n", nb2);

    printf("On inverse les nombres \n\n");

    inverser_nombres(&nb1, &nb2);

    printf(" nb1 vaut %d \n", nb1);
    printf(" nb2 vaut %d \n \n ", nb2);



    return 0;
 }  
