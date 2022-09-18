  /* 
      XIII - VIEW ET PURE

        !!!IMPORTANT!!! Selon qu'on utilise view, pure ou aucun mot clés Solidity fera appel à 2 API différentes.

                  VIEW/PURE -> ETH CALL -> Pas de cout en GAZ
                  SANS -> ETH SENDTRANSACTION -> Cout en GAZ
  */
//Indique la version du compilateur
pragma soliditv 0.8.7:
// SPDX-License-Identifier: UNLICENSED
import"./api_logements/owner.sol";
//Contrat
contract TestContract is Owner{

  uint nombre;

  //Remarque le setter est un boutton orange sur Remix il caracterise une écriture sur la blockchain
  function setNombre(uint _nombre) public{
    nombre = _nombre;
  }

  //Remarque le getter est un boutton bleu sur Remix il caracterise une lecture seul sur la blockchain
  function getNombre() public view returns (uint){
    return nombre;
  }

  //Le mot clés pure materialise une action sans �criture sur la blockchain
  //0n manipule une data passé en parametre mais a aucun moment l'ont procéde à une affectation
  //Le boutton sur Remix nous informe d'ailleur d'un comportement de lecture par un button bleu
  function X2(uint _nombre) public pure returns(uint){
    return nombre * 2;
  }
}

