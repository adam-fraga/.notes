/*Ces directives permettent d'éviter les inclusions multiples
 On définit une constante de tel sorte à ce qu'une fois celle ci définit  on sait que le programme à été inclut le #ifndef est une simpledirective if permettant d'englober le fichier à inclure */
#ifndef PLAYER_H
#define PLAYER_H

/*Le fichier .h contient les prototypes des fonctions définit dans le .c qui
lui est associé (du même nom)*/
void bonjour(void);
int reset_niveau(void);

#endif
