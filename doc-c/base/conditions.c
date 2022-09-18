#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    //Structures conditionnelles
    //Les parenthèse permettent de priorisé certains cas
    int nombre = 20;
    if (nombre == 20 || nombre > 0)
    {
        printf("Nombre est égal à %d \n", nombre);
    }
    else if (nombre == 20 && nombre != 0)
    {
        printf("Nombre est égal à %d \n", nombre);
    }
    else
    {
        printf("Nombre n'est ni égal à 25 ni égal à 0\n");
    }

    switch (nombre)
    {
    case (20):
        printf("nombre vaut 20 \n");
        break;
    case (32):
        printf("nombre vaut 20 \n");
        break;
    default:
        printf("Nombre vaut ni 20 ni 32 \n");
        break;
    }
    //Ternaires
    int ternaire = (nombre == 20) ? 1 : 0;
    printf("La condition vaut %d \n", ternaire);

    return 0;
}
