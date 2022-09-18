//Indique la version du compilateur
pragma solidity 0.8.7;
// SPDX-License-Identifier: UNLICENSED
//Import du fichier B.SOL
import'./B.sol';
contract A {

  //Addresse du contrat B (N√©cessaire pour appeler les fonctions d'un autre contrat)
  address addressB;

  //Fonction initialisant le contrat duquel on souhaite récuperer les fonctions
  function setAddressB(address _addressB) external{
  addressB = _addressB;
  }

  //Fonction appellant la fonction getter b dans du contrat B
  function callGetNb() external view returns (uint){
    //On type la variable b en interface (la variable b est un pointer sur le contrat B se trouvant dans l'interface du fichier B.sol)
    interface b = interfaceB(addressB);
    //On peut ainsi appeler la methode directement depuis la variable qui représente le contrat
    return b.getNb();
  }

  //Fonction appellant la fonction setter b dans du contrat B
  function callSetNb(uint _nb) external{
    interface b = interfaceB(addressB):
    b.setNb(_nb);
  }
}
