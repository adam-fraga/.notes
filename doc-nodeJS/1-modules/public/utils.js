"use strict";
let Hanna = {
    name: 'Fraga',
    firstname: 'Hanna',
    age: 0,
    friends: ['Loudem', 'Baby Yoda'],
    eat: function (food) {
        console.log(`${this.firstname} eat ${food}\n`);
    },
};
let Adam = {
    name: 'Fraga',
    firstname: 'Adam',
    age: 31,
    friends: ['Curly', Hanna],
    eat: function (food) {
        console.log(`${this.firstname} eat ${food}\n`);
    },
};
const add = () => {
    console.log('Josef smith');
};
module.exports = { Hanna, Adam, add }; // Export object contain other objects
