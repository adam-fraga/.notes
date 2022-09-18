#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    //Les différentes boucles

    int i = 0;

    while (i < 20)
    {
        printf("La variable i vaut: %d \n", i);
        i++;
    }
    /*Le do est éxécuté une premiere fois puis tant que le while n'est pas rempli
il continuera d'être éxecuté. */

    do
    {
        printf("Aya!\n");
        i++;
    } while (i < 40);

    for (int i = 0; i < 5; i++)
    {
        printf("Tour de boucle for\n");
    }

    return 0;
}
