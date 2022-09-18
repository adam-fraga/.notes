//Read only permet de définir un comportement en lecture seul et ainsi de bloqué
//L'alteration de la donnée passé en parametre. Pour modifier le tableau dans le
//cas présent on en créer un nouveau grace au spread operator.
function reverse<T>(arr: readonly T[]): readonly T[] {
  return [...arr].reverse()
}

class A {
  private a = 3
  protected b = 'Hello'
  public c = 'World'

  log() {
    console.log(this.a)
  }
}

class B extends A {
  log() {
    console.log(this.b) //Ok car protected
  }
}

const aInstance = new A()
// console.log(aInstance.a); //FAUX attr private
console.log(aInstance.log()) //OK methode public
console.log(aInstance.c) //OK attr public
