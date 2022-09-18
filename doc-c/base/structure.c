#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Déclaration d'une structure sans alias, Les structure se définissent dans les
fichiers d'entêtes (.h) */

struct Livre
{
    char titre[100];
    char auteur[100];
    int annee;
};

/* Définit la structure comme un type et permet de l'appeler avec le type "Personne"
 cest une forme d'alias */
typedef struct
{
    char nom[100];
    char prenom[100];
    int age;
} Personne;

// Utilisation de pointeur pour modifier une structure
void modifier_infos_personne(Personne *p)
{
    // La syntaxe avec flèches évite d'écrire (*p).age
    p->age = 25;
    strcpy(p->nom, "Detalife");
    strcpy(p->prenom, "Josef");
}

//Fonction affichant une instance de la structure Livre
void afficher_livre(struct Livre livre)
{
    printf("========================\n");
    printf("Titre du livre: %s \n", livre.titre);
    printf("Auteur du livre: %s \n", livre.titre);
    printf("Année de Publication: %d \n", livre.annee);
    printf("========================\n \n");
}

//Grâce à Typdef Plus besoin de la directive Struct en parametre de la fonction
void afficher_infos_personne(Personne personne)
{
    printf("========================\n");
    printf("Nom: %s \n", personne.nom);
    printf("Prénom: %s \n", personne.prenom);
    printf("age: %d \n", personne.age);
    printf("========================\n \n");
}

int main()
{
    /* Initialisation d'une variable de structure sans faire d'alias en lui
     assignant une valeur direct */
    struct Livre livre1 = {"JK Rowling", "Harry Potter", 1997};

    //On affiche l'instance de la structure livre
    afficher_livre(livre1);

    /* Initialisation d'une variable de structure avec alias (typedef)
    On peux choisir de ne pas assigné de valeur directement*/
    Personne personne1;

    //Le "." permet d"acceder au propriété des structures
    personne1.age = 30;

    /* La chaine de caractère étant déja initialisé impossible de la modifier
    on utilise donc la fonction strcpy pour y passé la valeur souhaité */
    strcpy(personne1.prenom, "Adam");
    strcpy(personne1.nom, "Fraga");

    //Affiche les infos de la personne
    afficher_infos_personne(personne1);

    printf("On modifie les infos de la personne \n");

    //Utilisation de pointeurs pour modifier les infos dans la structure
    modifier_infos_personne(&personne1);

    //On affiche les infos modifier
    afficher_infos_personne(personne1);

    //Il est possible de crée des tableau de structure
    Personne groupeDePersonne[20];

    groupeDePersonne[5].age = 12;
    strcpy(groupeDePersonne[5].prenom, "Carla");
    strcpy(groupeDePersonne[5].nom, "Brownie");

    printf("Prenom: %s\n Nom: %s \n Age: %d \n", groupeDePersonne[5].prenom, groupeDePersonne[5].nom, groupeDePersonne[5].age);

    return 0;
}
