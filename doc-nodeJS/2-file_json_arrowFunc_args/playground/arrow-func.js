/*
In context of an object this keyword in arrow function in opposite to normal function
refere to the parent of the context it's useful when using forEach, map or filter and 
other function inside of a function to refere to the property of a parent object
*/

const ev = {
  name: 'My Event',
  body: 'Lorem hipopotsum',
  guests: ['Josef', 'Fayad', 'Nakid', 'Habel'],
  n_func() {
    console.log('Name: ' + this.name + ' from normal function') //OK
  },
  a_func: () => {
    console.log('Name: ' + this.name + ' from arrow function') //OK
  },
  print_guest() {
    this.guests.forEach((guest) => {
      //For each nest this keyword
      console.log(guest + ' is attending for the event ' + this.name) //OK with Arrow func
    })
  },
}

ev.n_func()
ev.a_func()
ev.print_guest()
