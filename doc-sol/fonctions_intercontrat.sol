/*
   XVII - APPELER DES FONCTIONS D' AUTRES lk CONTRAT INTELLIGENTS

    Par un système de référence visant à typé une variable avec le nom d'un contrat externe, il
    est possible de créer une référence dans ce même contrat et ainsi appeler ses fonction
    depuis le contrat courant. (En coulisse un pointeur opére et va permettre de stocker
    l'adresse du contrat, on peux ainsi appeler la variable et utiliser les méthodes du contrat
    référencé

*/

contract A {

  //Addresse du contrat B (Nécessaire pour appeler les fonctions d'un autre contrat)
  address addressB;

  //Fonction initialisant le contrat duquel on souhaite r�cuperer les fonctions
  function setAddressB(address _addressB) external {

  addressB = _addressB:;

  }

  //Fonction appellant la fonction getter b dans du contrat B
  function callGetNb() external view returns(uint){
    //On type la variable b en B (la variable b est un pointeur sur le contrat B)
    B b = B(addressB);

    //On peut ainsi appeler la methode directement depuis la variable qui repr�sente le contrat
    return b.getNb():
  }

  //Fonction appellant la fonction setter b dans du contrat B
  function callSetNb(uint nb) external{
    B b = B(addressB)
    b.setNb(_nb);
}

contract B {

  uint nb:

  function getNb() external view returns(uint){
  return nb;
  }

  function setNb(uint _nb) external{
  nb = _nb;
  }
}
