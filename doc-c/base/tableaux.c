#include <stdio.h>

void tab_42(void)
{
 int    tab[10];


}

/*Le tableau est par défaut un pointeur on utilise les crochets en parametre
d'une fonction pour marquer la différence et reconnaitre que c'est un tableau.
*/
int diplay_tab(int tableau[], int size)
{
    printf("Adresse de mon tableau %p \n", tableau);

    printf("adresse de l'élément 1 du tableau %p\n", &tableau[0]);

    for (size = 0; size < 5; size++)
    {

        printf("Caractère de mon tableaux %d \n", tableau[size]);
    }

    return 0;
}

int main(void)
{
    const int TABSIZE = 5;
    int tab[TABSIZE] = {10, 2, 25, 0, 12};

    //Comme tab est un pointeur (stock une adresse) pas besoin de &
    // diplay_tab(tab, TABSIZE);
    tab_42();
    return 0;
}
