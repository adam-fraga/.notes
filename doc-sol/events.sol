/* 
   XVI-EVENTS

    Comprendre le système d'évènement sur solidity, il est possible de returner des valeur
    dans une fonction solidity mais il n'est pas possible de lire ces valeur depuis l'extérieur.
    Par exemple depuis le front end. Pour que le Front puissent recevoir des informations du
    back nous utilisons les Events.
*/

//Contrat
contract Events {

/*
   !!!IMPORTANT!!!
      Les évenements ne peuvent pas être lu depuis les smart contrat.
      Stocker dans les events coute mons chere en gaz qu'en mode storage.
*/
  uint[] numbers;
  //Créer un évenement
  event numberAdded (address _by, uint number);

  function addNumbers(uint _number) external{
  numbers.push(_number);

  //Déclencher l'évenement
  emit numberAdded (msg. sender, _number);
  }
/*
  En d�pliant la fl�che a droite du bouton d�but on peux voir dans l'onglet lors l'event ainsi
  qu'une succession d'argument dont notre numer, l'adresse de l'�metteur etc... On pourra
  mettre en forme ces data avec web3 JS
*/

