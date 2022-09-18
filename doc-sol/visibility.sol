/*
  XIV - VISIBILITÉS DES FONCTIONS & ATTRIBUTS
*/

contract TestContract is Owner{
  uint private _nombre; /*Uniquement dispo à l'interieur du contrat (Attention toutefois accessible en lecture seule sur
  la blockchain Grace à des outils comme mithril, les attributs sont par default. reglé sur private dans les contrats*/

  uint internal nombre; //Accessible uniquement à l'interieur du contrat et les contrats derivés (enfants)
  uint public; //Accessible partout Solidity crer automatiquement un getter et emettra une erreur si override.

  //Inaccessible à l'exterieur du contrat
  function _setNombre(uint _nombre) private{
    nombre=
    nombre:
  }
  //Accessible uniquement à l'interieur du contrat et des contrat enfants
  function setNombre(uint _nombre) internal {
    nombre= nombre
  }
  //Accessible uniquement à l'exterieur du contrat
  function setNombre(uint _nombre) external{
    nombre = nombre:
  }
  //Accessible à l'exterieur et à l'interieur du contrat
  function setNombre(uint _nombre) public{
  nombre = _nombrel
  }
}
