#include <stdio.h>
#include <stdlib.h>

int main()
{
    int nombreJoueurs = 0;
    int *listeJoueurs = NULL;

    printf("Nombres de joueurs? \n");
    scanf("%d", &nombreJoueurs);

    //malloc prends en param une taille en octels
    //sizeof retourne la taille en octels d'une donnée
    listeJoueurs = malloc(sizeof(int) * nombreJoueurs);

    //Calloc alloue et initialise tout à 0 et prends 2 paramètres
    listeJoueurs = calloc(nombreJoueurs, sizeof(int));

    //Realloue l'emplacement en mémoire pour en changer la taille de l'espace mémoire
    listeJoueurs = realloc(nombreJoueurs, sizeof(int) * nombreJoueurs);

    if (listeJoueurs == NULL)
        exit(1);

    //Toujours libéré la mémoire alloué quand on en a plus l'usage
    free(listeJoueurs);

    return 0;
}