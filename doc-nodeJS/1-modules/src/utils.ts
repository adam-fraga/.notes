type Human = {
  name: String
  firstname: String
  age: number
  friends: Array<String | Human>
  eat: Function
}

let Hanna: Human = {
  name: 'Fraga',
  firstname: 'Hanna',
  age: 0,
  friends: ['Loudem', 'Baby Yoda'],
  eat: function (food: String): void {
    console.log(`${this.firstname} eat ${food}\n`)
  },
}

let Adam: Human = {
  name: 'Fraga',
  firstname: 'Adam',
  age: 31,
  friends: ['Curly', Hanna],
  eat: function (food: String): void {
    console.log(`${this.firstname} eat ${food}\n`)
  },
}

const add = () => {
  console.log('Josef smith')
}

module.exports = { Hanna, Adam, add } // Export object contain other objects
