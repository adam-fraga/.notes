#la ligne ci dessous est appelé shebang et permet d'intereter le code.
#!/bin/zsh


#VARIABLES
#La déclaration des variables ne doit pas contenir d'espace en shell.

#N'interprete rien
#str='Salut'
#Interprete les variables et affiche les espace
#str2="La vie $str"
#Interprete les commandes et stock leur retour dans la variable.
#str4=`ls -la`
#Interprete les calcul
#let calc=5+5
#Vide une variable
#unset data

#CONSTANTE
#readonly="data"


#echo $str4



#COMMANDE READ
# -p ajouter un texte avant la saisit
# -s masque la saisit de l'utilisateur
# -n limiter à N caractères lus
#  -t autoriser la saisit pendant N seconde
#read -p "Entre un texte: " -n -s -t 5 name
#echo -e "\n Saisit:  $name"i

#RÉCUPERATION DES PARAMETRES
#   $# : contient le nombres de paramètres
#   $0 : contient l\'executable (.sh)
#   $1 à $9 : contient les parametres de 1 à 9
#   ${15}: permet de traiter les prametre superieur à 9
#   $$:Contient le numero du processus attribué lors de l\'éxecution
#   $*: Traite les parametre sous forme d\'une même chaine de caractere
#   $@: Traite les parametre indépendanment les un des autres

#L'instruction SET remplace les parametre par les valeur passé $2 = B
#set A B C D E

#STRUCTURE CONDITIONNELLES
  # Si le resultat de la condition est égal à 0 alors la condition est
  #si le resultat est égal à 1 ou autre chose alors c'est faux
  # les caractre || et && fonctionnent pour "ou" et "et".
  
#CONDITIONS SUR LES ENTIERS
  # < et > étant des caractère reservé pour les redirection en shell on utilise
  # donc les opereateur de comparasain suivant pour les entier.
#  [ $nb1 -eq $nb2 ] pour equal, égal.
#  [ $nb1 -ne $nb2] pour not equal, non égal.
#  [ $nb1 -gt $nb2 ] pour greater than, plus grand que.
#  [ $nb1 -ge $nb2 ] pour greater than or equal, plus grand ou égal.
#  [ $nb1 -lt $nb2 ] pour less than, plus petit.
#  [ $nb1 -le $nb2 ] pour less than or equal.
  
#    nb1=5
#    nb2=12


#    if [ $nb1 <  $nb2 || $nb1 < $nb2 ] ; then
#      echo "nb1 < nb2"
#    elif [ $nb1 > $nb2] ; then
#      echo "nb1 > nb2"
#    else
#      echo "nb1 = nb2"
#    fi

#BOUCLES
#break casse la boucle et continue execute une nouvelle itération
  
  i=0
  
  #WHILE ET UNTIL
  #il existe un inverse à while qui est until
  #while(($i < 10))
  #do
  #  echo $i

  #  let i++
  #done
  
  #FOR

  #for in va itéré sur les différent argument récupéré dans &@
  #On voit ici qu'en échapant &@ que les parametre récupéré entre guillemet
  #compte pour un seul parametre
  #for arg  in "$@"   
  #do
  #  echo $arg
  #done

  #for((i=0 ; i < 10 ; i++))
  #do
  #  echo "Boucle for normal"
  #done

#TRAITEMENTS SUR LES CHAINES DE CARACTERES

str="Ma Chaine De Caractères!"
  
  #Le signe diese récupere la longueur de la chaine
  echo ${#str}
  #Passe les caractere en minuscule "," premier caractere ",," tous
  #[D] passe le D en minuscule 
  echo ${str,,[D]}
  #Pareil mais pour majuscule 
  echo ${str^}
  #Exrait une sous chaine à l'indice souhaité de la chaine
  echo ${str:6}
  #Extrait la chaine de l'indice n à l'indice n
  echo ${str:0:9}
  #Remplacer la première occurences ou premier mot dans une chaine
  echo ${str/a/z}
  #Remplacer toutes les occurences ou tout les mot dans uen chaine
  echo ${str//a/z}
  #Remplace le premier caractere de la chaine si il correspond
  echo ${str/#M/v}
  #Remplace le dernier caractere si il correspond
  echo ${str/%!/?}
  #Supprimer des occurences ou des mots sur une chaine
  #(On laisse simplement le caractere de remplacement vide)
  echo ${str//De/}
  #Supprimer du premier carractere  au x caractere double # pour supprimer plus
  echo ${str#M*e}

