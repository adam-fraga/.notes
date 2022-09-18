//nodemon allow node to watch our script already installed globaly npm i nodemon -g)

const fs = require('fs') // File system module
const validator = require('validator') //npm package (installed)
const chalk = require('chalk') //npm package (installed)

//Import element from utils.js (Personal module)
const utils = require('./utils.js')

fs.writeFileSync('file.txt', 'Hello Hanna \n')
fs.appendFileSync('file.txt', 'How are you today my baby?')

console.log('Object Adam: \n', utils.Adam)
console.log('Object Hanna: \n', utils.Hanna)

utils.Adam.eat('Tacos')
utils.Hanna.eat('Bannana')
console.log(chalk.green('Success'))

console.log(validator.isEmail('adam.fraga@example.com'))
