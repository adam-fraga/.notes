// Pour compiler le TS on utilise tsc "file.ts""
// Avec dossier src et dist npx tsc src/app.ts --outDir dist
// Le fichier tsconfig.json automatise la ligne ci dessus (ajouté --watch)

//SIMPLE
const a: String = 'Hello world'
const n: number = 23
const b: boolean = true
const d: null = null

//ARRAY
const arrStr: string[] = ['Tableau', 'de', 'string']
const arrStr2: Array<String> = ['Tableau', 'de', 'string']

const arrNb: number[] = [10, 20, 30, 40]
const arrNb2: Array<number> = [10, 20, 30, 40]

const arrMix: any[] = ['Tableau', 'mixte', 23, true]

//TUPLE
let tp: [String, number] = ['Hanna', 1]

//OBJECT
const user: {
  firstname: string
  lastname: string
  colors: Array<String>
} = {
  firstname: 'John',
  lastname: 'Doe',
  colors: ['blue', 'red', 'yellow'],
}

const optionnalUser: { firstname?: string; lastname?: string } = {}

//Objet avec une infinité de clés
const infiniteKey: { [key: string]: string } = {
  firstname: 'Adam',
  lastname: 'Frg',
  color: 'blue',
}

const date: Date = new Date()
const callBack: (e: MouseEvent) => void = (e: MouseEvent): number => {
  return 3
}

const printID: Function = (id: number): number => {
  console.log(id.toString())
  return id
}

// Force le typage d'une variable (Cast)
const count = document.querySelector('#count') as HTMLButtonElement
const count2 = <HTMLButtonElement>document.querySelector('#count')

// Union de type
const printMix: Function = (id: number): number | string => {
  console.log(id.toString())
  return id
}

//ENUM
enum Level {
  ADMIN,
  SUPPORT,
  USER,
}

const Adam: { name: String; age: number; level: Level } = {
  name: 'Adam',
  age: 20,
  level: Level.USER,
}

//FUNCTIONS
let fn_no_arg_return_nothing: () => void
let fn_arg_and_return_value: (a: number, b: number) => number
let fn_rest_of_arguments: (one: number, ...restOfArg: String[]) => void
let fn_type_callBack: (a: number, fn: (a: number) => void) => void

fn_no_arg_return_nothing = () => {
  console.log('Hello world')
}
// "?" optionnal params, "=" default params
fn_arg_and_return_value = (a: number = 10, b?: number) => {
  return a + b
}

fn_rest_of_arguments = (_nb, names) => {
  console.log(names)
}

let my_func: Function = fn_rest_of_arguments //Possible in JS
my_func(10, 'Adam', 'Nana', 'Imn')
