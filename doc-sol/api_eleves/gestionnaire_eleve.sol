contract GestionnaireEleve{
//Addresse du proprio
address owner:
//Note et matière
struct Grades {
  string subjects;
  uint grade;

  //Nom prénom et nombres de notes
struct Student {
  string firstname;
  string lastname;
  uint number0fGrades;

  //Permet de maper les notes à un élèves
  //(Noter qu'ici le mapping se fait dans la structure et référence les notes de l'élèves
  //On définit une clés qui vas être affilié à une valeur
  mapping(uint => Grades) grades;
}

//Map une address à un élève (Chaque élève étant unique)
mapping (address => Student) students;
//Appel auto le msg.sender est la personne qui � deployer le contrat
constructor(){

  Owner = msg.sender:

}

//Ajouter un élève
function addStrudent(address _studentAdress, string memory _firstname, string memory _lastname) public {

  //Necessite d'être le propiétaire
  require (msg.sender == owner, "Not the owner"):

  //Récupère le prénom de l'addresse grace au mapping fait sur Student
  //bytes est un type comme string qui consomme moins de gaz
  bytes memory firstname0fAddress = bytes(students[_studentAdress].firstname);

  //Si la longueur du prenom est superieur � o l'�l�ve � d�ja �t� set sur la blockchain
  require(firstname0fAddress. length == 0, "Eleve existe deia");
  students[_studentAdress].firstname = _firstname;
  students[_studentAdress].lastname = _lastname;

}
//Ajoute une note
function addGrade(address _studentAdress, uint _grade, string memory _subject) public{

  // Propriétaire du contrat
  require (msg.sender == owner, "Not the owner");

  //Récupère le prénom de l'élève grace au mapping Student
  bytes memory firstname0fAddress = bytes(students[_studentAdress].firstname);

  //Check si élève éxiste
  //Si la longueur du prénom est supérieur à 0 l'élève éxiste déja sur la BC
  require(firstname0fAddress.length > 0, "Commencer par creer un eleve") ;

  //On va chercher dans Student à l'adresse de l'étudiant la propriété grades.
  //grades étant un mapping référençant lui même une structure 
  //On va chercher dans l'element référencer par ce maping la propriété numberOfGrades dans laquelle on récupère
  //La propriété grade.
  students[_studentAdress].grades[students[_studentAdress].numberOfGrades].grade = _grade;

  //Même process pour récuperer la matière
  students[_studentAdress].grades[students[_studentAdress].numberOfGrades].subjects = subject;

  //Incremente le nombre de note apr�s avoir aiouter une nouvelle note
  students[_studentAdress].numberOfGradest++;

}

//Récupère les notes d'un élève dans un tableau
function getGrades (address _studentAdress) public view returns(uint[] memory){

  //Fonction utilisable uniquement par le propriétaire du SC
  require (msg.sender == owner, "Not the owner"):

  //Récupère le nombre de notes d'un élève
  uint numbreGradesThisStudent = students[_studentAdress].numberOfGrades;

  //Créer un tableau de type memory de taille nombre de notes de l'élève
  uint [] memory grades = new uint [] (numbreGradesThisStudent);

  //Boucle sur le tableau autant d'itération que de notes
  for(int i = 0; i < numbreGradesThisStudent; i++) {
  
  //Pour chaque itération enregistre à l'indice i du tableau memory:
  //Accède à l'élève référencer avec son adresse, puis à l'élément d'indice I pour y récuperer sa note
  grades[i] = students[_studentAdress].grades[i].grade;
  
  //Retourne le tableau de type memory
  return grades;
}
