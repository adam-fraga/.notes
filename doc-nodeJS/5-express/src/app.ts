import express from 'express'
import path from 'path'
import { success } from './helper.js'
import { pokemons } from './mock-pokemon.js'

const port = 3000

// Path module allow dynamic path with ".."
const publicDirectory = path.join(__dirname, '..')

const app = express() //Instanciate express

// Serve automaticaly the corresponding html file in public matching with file.html in url
app.use(express.static(publicDirectory))

//Define Middleware (Launch before request)
const logger = (req: any, res: any, next: any): void => {
  console.log(`URL: ${req.url}`)
  next() //End of the middleware treatment
}

//Call middleware
app.use(logger)

//Define route (Root will never be use in this case because of line 13)
/* Arguments: URL - Object Request - Object Response */
app.get('/', (req: any, res: any) =>
  //Send response to the client (use res.render if you use a template engine like handlebar)
  res.send(
    '<h1>Welcome on Pokemon API use "/api/pokemon/ID" to show the pokemon related to the id</h1>'
  )
)

/* Route with dinamyc ID */
app.get('/api/pokemon/:id', (req: any, res: any) => {
  const id = parseInt(req.params.id) //Express renvoi par defaut les param en string
  const pokemon = pokemons.find((pokemon: any) => pokemon.id === id)
  res.json(success('POKEMON', pokemon))
})

//Start up the server on a given port

/* Arguments: PORT - CALLBACK function run while the server is running */
app.listen(port, () =>
  console.log(`Server listen on port: http://localhost:${port}`)
)
