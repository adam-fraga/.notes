/* 
   XX - Créer Smart contrat a partir d'un autre

    Cette technique minimise le risque, si il existe une faille de sécurité dans un factory
    contract il y a moins de chance que les contrat sous jacent soit affecté.
    A partir du contrat factory on va créer des contrats sous jacents et les placer dans un array
    de contrat, cette approche apporte du dynamisme dans la cr�ation de plusieurs contrats.
*/

//Factory contract permet la création sécurisé et dinamyque de plusieurs contrat sous jacents
contract FactoryNumber {

  //Définit un tableau de contrat Number (Number étant un contrat externe)
  Number[] numbersContracts;

  //permet de créer un nouveau contrat number de l'ajouter dans un tableau de contrats et de returner son adresse
  function createNumberContract() external returns (address) {
    //Initialiser un contrat externe à la fonction
    //La valeur 100 est celle demandé par le constructeur du contrat externe
    Number n = new Number (100);

    //Ajoute le contrat que l'on vient de créer dans le tableau de contrats Number
    // (Number étant un contrat externe)
    numbersContracts.push(n);
    return address (n);
  }

  //R�cupere le nombre stocker dans un contrat number représenté par son adresse
  //Tout les contrat ont une adresse précise!
  function getNumberByContract (Number _Contract) external view returns(uint){
    return _Contract.getNumber();
  }

    //Permet 'assigner un nombre à un contrat spécifique
  function setNumberByContract (Number _Contract, uint _number) external {
    _Contract.setNumber(_number);
  }

contract Number{

  uint number;

  //Initialise number au deploiement du contrat
  constructor(uint number){
  number = _number;
  }

  //Getter
  function getNumber() external view returns(uint){
    return number;
  }
  //Setter
  function setNumber(uint _number) external{
    number = _number;
  }
