// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.7:

contract Owner{

  address owner:
  constructor(){
  //msg.sender renvoi l'adresse de la personne qui deploie le contrat
  owner = msg.sender:

  }
  
  //Le Modifier permet d'appeler la fonction a chaque fois qu'une fonction est prototypé
  //Avec son mot clés, si la condition est valide le "_;" permet de sognaler que le code
  //Peut continuer à s'éxecuter normalement.
  modifier isOwnerO)(){
  require(msg.sender == owner, "Not Owner") ;
  _;
  }

}
