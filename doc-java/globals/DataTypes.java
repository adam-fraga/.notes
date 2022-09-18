import java.util.ArrayList;

public class DataTypes {
  public static void main(String[] args) {

    // Primitives => Associate Object=> memory space

    int number = 34; // Integer => 4 bytes
    double number2 = 2.2; // Double => 8 bytes
    float number4 = 2.2f; // Float => 4 bytes
    char letter = 'a'; // Chararacter => 2 bytes
    short number3 = 2; // Short => 2 bytes
    boolean bool = true;// Boolean => 1 byte
    byte oct = 'u'; // Byte => 1 byte

    final int MY_CONST = 78;

    int[] arr = { 1, 2, 3, 4 };
    int[] arr2 = new int[3];
    int[][] twoDimensionalArr = {
        { 1, 2, 3, 4 },
        { 1, 2, 3, 4 },
        { 1, 2, 3, 4 },
        { 1, 2, 3, 4 } };
    ArrayList<String> list = new ArrayList<>();
  }
}
