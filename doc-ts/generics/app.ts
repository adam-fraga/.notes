//Type Objet utilisateur
type User = {firstname: string, lastname: string}
//Type clés de User (Username = Firstname ou lastname)
type Username = keyof User
//Type date au format str
type DateString = string
//Type union string ou number
type Id = string | number
//Type generique fonction
type identity<ArgType> = (arg: ArgType) => ArgType

//Les générique permettent de spécifier dans cet exemple que le type de donnée retournée
//par la fonction doit être le même que celui qui est passé en parametre.
function identity<ArgType>(arg: ArgType): ArgType{
  return arg
}
//Prends un type number et retourne un type number
const aa = identity(3)

//Autre éxemple avec un tableau
function first<Type>(arg: Type[]): Type {
 return arg[0] 
}

const bb = first(["aze", "baze", "taze"])

//Tableau générique type union
const arr: Array<string | number> = ["aze", 12, "baze"]

//Contrainte générique permet ici de spécifier que le type doit étendre d'un Objet
//et posséder une propriété length
function consoleSize<Type extends {length: number}>(arg: Type): Type {
 console.log(arg.length);
  return arg
}

// consoleSize(3) FAUX 3 n'a pas de propriété length
consoleSize(["aze", "saze"]) //OK array dispose d'une propriété length

