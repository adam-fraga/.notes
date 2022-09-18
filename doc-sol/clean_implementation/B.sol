/*
   XIX - INTERFACE

  L'interface permet 'implémenter proprement une méthode permettant l'inter opérabilité
  entre des contrats. On commence par définir l'interface et son nom, celle ci va contenir la
  signature des fonctions du contrat se trouvant sur la même page.
*/

//Indique la version du compilateur
pragma solidity 0.8.7;
//SPDX-License-Identifier: UNLICENSED

//Comme les .h les interface contiennent les signatures de fonctions
//Créer une interface
interface interfaceB{
  function getNb() external view returns(uint);
  function setNb(uint nb) external;
}

contract B {

  uint nb;

  function getNb() external view returns(uint){
    return nb;
  }

  function setNb(uint _nb) external{
  nb = _nb;
  }

/* 
       Il suffira ensuite d'importer dans le contrat le fichier contenant l'interface des
    fonctionnalités que l'on souhaite utilisés dans notre autre contrat.
    Une syntaxe spécifique à retenir contenant interfaceNom Del interface permettra de pointer
    sur 'interface et de stocker le contrat dans une variable.
    Les attributs et méthodes du contrat implémenter par 'interface pourront donc être
    appelés dans notre contrat courant
*/
