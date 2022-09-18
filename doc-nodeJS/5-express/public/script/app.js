"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const path_1 = __importDefault(require("path"));
const helper_js_1 = require("./helper.js");
const mock_pokemon_js_1 = require("./mock-pokemon.js");
const port = 3000;
// Path module allow dynamic path with ".."
const publicDirectory = path_1.default.join(__dirname, '..');
const app = (0, express_1.default)(); //Instanciate express
// Serve automaticaly the corresponding html file in public matching with file.html in url
app.use(express_1.default.static(publicDirectory));
//Define Middleware (Launch before request)
const logger = (req, res, next) => {
    console.log(`URL: ${req.url}`);
    next(); //End of the middleware treatment
};
//Call middleware
app.use(logger);
//Define route (Root will never be use in this case because of line 13)
/* Arguments: URL - Object Request - Object Response */
app.get('/', (req, res) => 
//Send response to the client (use res.render if you use a template engine like handlebar)
res.send('<h1>Welcome on Pokemon API use "/api/pokemon/ID" to show the pokemon related to the id</h1>'));
/* Route with dinamyc ID */
app.get('/api/pokemon/:id', (req, res) => {
    const id = parseInt(req.params.id); //Express renvoi par defaut les param en string
    const pokemon = mock_pokemon_js_1.pokemons.find((pokemon) => pokemon.id === id);
    res.json((0, helper_js_1.success)('POKEMON', pokemon));
});
//Start up the server on a given port
/* Arguments: PORT - CALLBACK function run while the server is running */
app.listen(port, () => console.log(`Server listen on port: http://localhost:${port}`));
