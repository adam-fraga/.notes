#include <iostream>
#include <ostream>
#include <string>

class User
{
public:
  std::string firstname;
  std::string lastname;
  static int user_count;
};

int User::user_count = 0;

/* int add_user_if_not_exists(std::vector<User> &users, User user) */
/* { */

/* } */

// cout est un objet de type ostream donc on overload ostream attention à la syntaxe
std::ostream &operator<<(std::ostream &output, const User user)
{
  output << "First name: " << user.firstname << "\nLast name: " << user.lastname << "\n";

  return output;
}

// cin a besoin d'un passage par référence de l'utilisateur pour fonctionner
std::istream &operator>>(std::istream &input, User &user)
{
  // istream reconnait les espace et saura concaterner les 2 valeur dans l'objet
  input >> user.firstname >> user.lastname;

  return input;
}

int main(int argc, char *argv[])
{
  /* OUTPUT */
  User user;
  /*   user.firstname = "Adam"; */
  /*   user.lastname = "Fraga"; */

  // Dans le cas présent nous avons 2 objets distinct "cout" et "user"
  // on définit donc la méthode à l'exterieur de notre objet puisqu'i lse trouve à droite de
  // l'opérateur ">>" on doit le passé en paramètre.
  /* std::cout << user << std::endl; */

  /* INPUT */
  std::cin >> user;
  std::cout << user << std::endl;
  return 0;
}
