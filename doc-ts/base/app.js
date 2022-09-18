// Pour compiler le TS on utilise tsc "file.ts""
// Avec dossier src et dist npx tsc src/app.ts --outDir dist
// Le fichier tsconfig.json automatise la ligne ci dessus (ajouté --watch)
//SIMPLE
var a = 'Hello world';
var n = 23;
var b = true;
var d = null;
//ARRAY
var arrStr = ['Tableau', 'de', 'string'];
var arrStr2 = ['Tableau', 'de', 'string'];
var arrNb = [10, 20, 30, 40];
var arrNb2 = [10, 20, 30, 40];
var arrMix = ['Tableau', 'mixte', 23, true];
//TUPLE
var tp = ['Hanna', 1];
//OBJECT
var user = {
    firstname: 'John',
    lastname: 'Doe',
    colors: ['blue', 'red', 'yellow']
};
var optionnalUser = {};
//Objet avec une infinité de clés
var infiniteKey = {
    firstname: 'Adam',
    lastname: 'Frg',
    color: 'blue'
};
var date = new Date();
var callBack = function (e) {
    return 3;
};
var printID = function (id) {
    console.log(id.toString());
    return id;
};
// Force le typage d'une variable (Cast)
var count = document.querySelector('#count');
var count2 = document.querySelector('#count');
// Union de type
var printMix = function (id) {
    console.log(id.toString());
    return id;
};
//ENUM
var Level;
(function (Level) {
    Level[Level["ADMIN"] = 0] = "ADMIN";
    Level[Level["SUPPORT"] = 1] = "SUPPORT";
    Level[Level["USER"] = 2] = "USER";
})(Level || (Level = {}));
var Adam = {
    name: 'Adam',
    age: 20,
    level: Level.USER
};
//FUNCTIONS
var fn_no_arg_return_nothing;
var fn_arg_and_return_value;
var fn_rest_of_arguments;
fn_no_arg_return_nothing = function () {
    console.log('Hello world');
};
// "?" optionnal params, "=" default params
fn_arg_and_return_value = function (a, b) {
    if (a === void 0) { a = 10; }
    return a + b;
};
fn_rest_of_arguments = function (_nb, names) {
    console.log(names);
};
var my_func = fn_rest_of_arguments; //Possible in JS
my_func(10, 'Adam', 'Nana', 'Imn');
