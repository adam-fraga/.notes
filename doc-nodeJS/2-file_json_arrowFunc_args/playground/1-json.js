const fs = require('fs')
const book = {
  author: 'JK BLUNT ROWLING',
  title: 'AKHI POTTER',
}
const bookJSON = JSON.stringify(book) //convert object to JSON
const parseData = JSON.parse(bookJSON) //convert JSON to Object

fs.writeFileSync('1-json.json', bookJSON)
const dataBuffer = fs.readFileSync('1-json.json')

const dataJSON = dataBuffer.toString()
const data = JSON.parse(dataJSON)

console.log(data)
