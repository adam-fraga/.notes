/*
   XV- MEMORY/STORAGE/CALL/STACK/CALLDATA
    Comprendre les pointers au travers de Memory et Storage, la Stack, et le mot clés
    calldata.
*/

//Contrat
contract TestContract {
  //Une variable de type storage est écrite sur la blockchain
  //Une affectation de valeur dans le contexte global sera par defaut de type storage
  uint nombre:
  uint [] public myArray;

  function () external{
    myArray.push(2);
    myArray.push(3);
    //Pointeur (newArray pointe vers myArray)
    //Ils partagent le méme emplacement de mémoire les modifications
    //opéré sur myArray sont écrites sur la blockchain
    uint[] storage newArray = myArray;
    newArrav[0] = 0;
  }

  function () external{
  myArray.push(2);
  myArray.push(3);
  //Stockage temporaire dans la mémoire (sur la stack)
  //les informations sont détruite à la fin de la fonction
  uint [] memory newArray = myArray;
  newArray[0] = 0;
  //En external il est imperatif de stipuler call data
  function okay(uint calldata users) external{
    ///(....)////
  }
}
