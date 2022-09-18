setTimeout(() => {
  console.log('Two seconds are up')
}, 2000)

const names: Array<String> = ['Adam', 'Nana', 'Imn', 'Yoda', 'Loucha']
const shortNames: Array<String> = names.filter((name) => {
  return name.length <= 4
})

/*
Async doest allowed return pattern instead to get the value we have to pass a callback
in 2nd argument callback functions are just a way to get the value outside without 
using the return, just pass the value to retrieve in argument to the callback declaration
(which is in parameter of the main calling function). Ex1: line 31 or Ex2: 41 and then you 
can access the data when you call the callback function (Ex1: line 27 or Ex2: 37 ).
*/

const getFood = (food: string, callback: Function): void => {
  setTimeout(() => {
    // Instead of => return data
    if (food === 'banana') {
      const food_infos = {
        fruit: true,
        name: 'Banana',
        sentence: "Monkey's favorite food",
      }
      callback(food_infos)
    } else if (food === 'Tacos' || food === 'Pizza') {
      const food_infos = {
        fruit: false,
        name: 'Tacos or Pizza',
        sentence: "Adam's favorite Food",
      }
      callback(food_infos)
    } else {
      callback('Error this food does not exist', undefined)
    }
  }, 2000)
}

getFood('Pizza', (error: any, response: any) => {
  console.log('Error: ' + error)
  console.log('Response: ' + response)
})

const add = (nb1: number, nb2: number, sum: Function) => {
  setTimeout(() => {
    sum(nb1 + nb2)
  }, 2000)
}

add(4, 5, (sum: number) => {
  console.log(sum)
})
