#include <stdio.h>
/* Pour compiler plusieurs fichier .c on utilise gcc *.c -o "executable"
Les fichiers .h sont des fichier d'entêtes qui peuvent être intégré avec la directive preprocesseur ci dessous, on y indique uniquement les prototype des fonctions définit dans nos autres fichiers .c */
#include "player.h"

int main (void)
{
 int level = 35;
 printf("Vous êtes niveau %d \n",level);

 level =  reset_niveau();

 printf("Vous êtes maintenant niveau %d \n",level);
  return 0;
}
