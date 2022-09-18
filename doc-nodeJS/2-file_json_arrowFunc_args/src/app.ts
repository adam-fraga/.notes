const chalk = require('chalk')
const yargs = require('yargs')

yargs.version('1.1.0')

yargs.command({
  command: 'add',
  describe: 'Add a new note',
  builder: {
    title: {
      describe: 'Note title',
      demandOption: true, //Title is now required for add command
      type: 'string', //Value must be a string
    },
    body: {
      describe: 'Note content',
      demandOption: true, //body is now required for add command
      type: 'string', //Value must be a string
    },
  },
  handler: (argv: any) => {
    console.log('Title: ', argv.title, '\nBody: ', argv.body)
  },
})

yargs.command({
  command: 'remove',
  describe: 'Remove a  note',
  handler: () => {
    console.log('Removing a new note')
  },
})

yargs.command({
  command: 'read',
  describe: 'Read a given note',
  handler: () => {
    console.log('Reading a note')
  },
})

yargs.command({
  command: 'list',
  describe: 'list all notes',
  handler: () => {
    console.log(' Listing of all notes')
  },
})

yargs.parse()
