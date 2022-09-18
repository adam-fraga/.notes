const increment = (e: Event) => {
  e.preventDefault()
  i++
  const span: HTMLSpanElement | null | undefined
  const span = count?.querySelector('span')
  if (span) {
   span.innerText = i.toString() 
  }
}

function printId(id: string | number){
  if (typeof id === "number") {
   console.log((id *3).toString())
    
  }
  else{
    console.log(id.toUpperCase());
    
  }
}

function example (a: string | boolean, b: string | number | boolean){
  if (a === b) {
    a    
  }
}

count?.addEventListener('click', increment)
