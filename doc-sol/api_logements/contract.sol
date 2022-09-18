//Indique la version du compilateur
pragma solidity 0.8.7;
// SPDX-License-Identifier: UNLICENSED

//Importer un contrat
import './owner.sol';

//Contrat hérité d'Owner
//L'héritage permet le partage des methodes et attributs entre les contrats sans mot clés
contract contract is Owner {

  //Une enum permet la définition de type, elles sont renvoyer sous forme 1, 2, 3...
  //Les différents type de biens
  enum typeBien{terrain, appartement, maison}

  //Le bien son identifiant, son nom et son prix
  struct Bien {
    uint id;
    string name;
    uint prix;
 }

  //Type de bien (Enum)
  typeBien _typeBien;
  //Compteur
  uint compteur;

  //Proprietaire des bien définit par leurs addresses respectives
  //Chacun pouvant avoir 0 ou plusieurs bien
  mapping (address => Bien[]) Possession;
 
  //Ajoute un bien (fonction accessible uniquement par le propri�taire du bien (Modifier isOwner)
  //Prends en parametre l'adresse du proprietaire, le nom du bien (ne pas oublier memory pour les string, le prix,
  //Et l'enum du type de bien (maison appart ou terrain)
  function addBien(address _proprietaire, string memory _name, uint _price, typeBien _typeBien) public isOwner{
    //Vérifie les valeur en entrer (important de cast en uint typebien pour operer les conditions dessus
    require(_price > 1000, "Le prix doit couter plus de 1000 Wei") ;
    require(uint(_typeBien) >= 0, "Le Type de bien doit etre compris entre 0 et 2");
    require(uint(_typeBien) <= 2, "Le Type de bien doit etre compris entre 0 et 2");

    //Ajoute au mapping la structure Bien
    Possession[_proprietaire].push(Bien(compteur,name, price, _typeBien));
    compteurt++;

  }

  //Récupere tout les biens d'un propriétaire Attention le tableau retourné est passé par référence
  //Il doit correspondre a celui nommé pendant le mapping
  function getBiens (address _proprietaire) public view is0wner returns(Bien[] memory) {
  //L'adresse passé en index à Possession référence un tableau de bien
  //Le mapping permet de référencer des valeur sur une adresse (Token)
  return Possession[_proprietaire];
  }

  //Permet à n'importe qui de voir ses biens
  function getUsersBiens () public view returns (Bien[] memory) {
  //Retourne la liste des biens pour le proprietaire qui execute le contrat
  return Possession[msg.sender];
  }
}
