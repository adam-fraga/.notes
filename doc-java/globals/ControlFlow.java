public class ControlFlow {

  public static void main(String[] args) {

    String[] names= {"Adam", "Nana", "Imn", "Yoda"};

    System.out.println("Control Flow: ");

    //One line condition
    if (2 > 3) System.out.println("Wtf ???");
    else System.out.println("Indeed");

    //For each
    for (String name : names) {
     System.out.println(name); 
    }
  }

}
