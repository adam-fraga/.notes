/*
XVII - LIBRAIRIES & CONCATENATION

Les @libs, permettent l'ajout de fonctionnalité permettant un gain de temps et une couche de
sécurité supplémentaire lors des interaction avec la blockchain.
//Pour importer une librairie on passe par @nomdelauteur/.../.../file.sol
*/

import'@openzeppelin/contracts/utils/Strings.sol';

//Contrat
contract Events {

/*Les Smart contract mis à disposition par openzepp sont secure et utiles
Il permettent de un gain de temps et mettent à disposition des outils sécurisés
pour le dev de NFT ou tout autre smart contract */

//Exemple de contrat pour la concatenation sur la blockchain
  function concatener(string memory _str, uint _nb, uint _nb2) external pure returns(string memory) {
    string memory result = string(abi.encodePacked(_str, Strings.toString(_nb), Strings.toString(_nb2))) ;
  return result;
  }

}
