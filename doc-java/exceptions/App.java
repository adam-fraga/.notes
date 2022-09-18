import java.util.InputMismatchException;
import java.util.Scanner;

//In Java all exception extends from Exception Class which extends from Throwable
//Java uses polymorphism and inheritance from those class to specialise different type
//Of exception

public class App {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Birth Date");
    try {
      int yearOfBirth = sc.nextInt();
      System.out.println(yearOfBirth);
    } catch (InputMismatchException e) {
      System.out.println("Type error date of birth must be a valid number");
      e.printStackTrace(); // Print strack trace
      System.out.println(e.getMessage()); // Get Error message
    } catch (MyOtherException e) {
      System.out.println("Second exception");
      e.printStackTrace();
      // All Exceptions
    } catch (Exception e) {
      System.out.println("Second exception");
    } finally {
      System.out.println("Executed whatever");
    }

  }
}
