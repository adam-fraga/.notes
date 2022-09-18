public abstract class Animal {
  // Attributes
  public String name;
  public String diete;

  // Constructor (same name as class don't have to specify void)
  public Animal(String newName, String newDiete) {
    name = newName;
    diete = newDiete;
    System.out.println("Animal have been created");
  }

  // Static method (belong to class)
  public static void main(String[] args) {
    System.out.println("Animal have been created");
  }

  // Method (belong to object)
  public void printInfos() {
    System.out.println("Animal name: " + this.name);
    System.out.println("Animal dite: " + this.diete);
  }
}
