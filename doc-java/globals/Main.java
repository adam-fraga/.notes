import java.util.ArrayList; //Import

//Compile file with javac file.java
//Run with java file.java or with java binary.class (without class extension)

public class Main { // sc => class
  // Main is the constructor (executed automaticaly)
  public static void main(String[] args) { // sc => main
    // Object Type
    appendLastName("Adam", "Fraga"); // Work because it's a static method so can be use in the class itself
    Wolf wolf = new Wolf("Law", "meat");
    wolf.printInfos();
    wolf.smellOfBlood();
  }

  public static void appendLastName(String name, String lastName) {
    // This keyword can't be use in static method (static refere to the class)
    System.out.println(name + " " + lastName); // Concat in Java same as JS
  }
}
