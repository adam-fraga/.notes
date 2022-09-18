public class Strings {
  public static void main(String[] args) {

    // Class imutable
    String str = "Hello world"; // = String str = new String("Hello world");

    // Work but here it's a new string the previous str was destroyed
    str = "Hello" + "World".concat("!"); // concat() more efficient than + operator

    // String Builder (muable mono thread async)
    StringBuilder str2 = new StringBuilder("Adam");
    System.out.println(str2.capacity()); //Memory space allocated for the string
    System.out.println(str2.length()); //Memory space allocated for the string

    // String Buffer (muable multi threads sync)
    StringBuffer str3 = new StringBuffer("Fraga");
    System.out.println(str3.capacity());
    System.out.println(str3.length());
  }
}
