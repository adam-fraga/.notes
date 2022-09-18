//Les générique permettent de spécifier dans cet exemple que le type de donnée retournée
//par la fonction doit être le même que celui qui est passé en parametre.
function identity(arg) {
    return arg;
}
//Prends un type number et retourne un type number
var aa = identity(3);
//Autre éxemple avec un tableau
function first(arg) {
    return arg[0];
}
var bb = first(["aze", "baze", "taze"]);
//Tableau générique type union
var arr = ["aze", 12, "baze"];
//Contrainte générique permet ici de spécifier que le type doit étendre d'un Objet
//et posséder une propriété length
function consoleSize(arg) {
    console.log(arg.length);
    return arg;
}
// consoleSize(3) FAUX 3 n'a pas de propriété length
consoleSize(["aze", "saze"]); //OK array dispose d'une propriété length
