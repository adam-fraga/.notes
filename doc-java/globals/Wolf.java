//Inheritance  Interface

//It's possible to overwrite method from the main class to specialize them

//It's possible to overload a method in the same class to manage multiple type

//It's Possible to exentds from java built in classes like exception to create
//new exception, StringBuilde...)

public class Wolf extends Animal implements InnerWolf {
  Wolf(String name, String diete) {
    // Refere to the parent constructor
    super(name, diete);
  }

  public void eatMutton() {
    System.out.println(name + " eat mutton");
  }

  public void smellOfBlood() {
    System.out.println("Smell blood odor");
  }
}

interface InnerWolf {
  public void smellOfBlood();
}
