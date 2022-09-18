fn main() {
    //conditionnal struct accept only boolean value as evaluation
    let nombre = 3;

    if nombre != 0 {
        println!("Le nombre valait autre chose que zéro");
    }

    // conditionnal are expression and can be store in instruction
    let condition = true;
    let nombre = if condition { 5 } else { 6 };

    println!("La valeur du nombre est : {}", nombre);

    // This code will throw an error if and else branch must contain the same data type
    let condition = true;

    let nombre = if condition { 5 } else { "six" };

    println!("La valeur du nombre est : {}", nombre);

    //loop can have label and you can play with that label to break or continue the concern loop
    // Inside an other to change behaviors of differents loops
    let mut compteur = 0;
    'increment: loop {
        println!("compteur = {}", compteur);
        let mut restant = 10;

        loop {
            println!("restant = {}", restant);
            if restant == 9 {
                break;
            }
            if compteur == 2 {
                break 'increment;
            }
            restant -= 1;
        }

        compteur += 1;
    }
    println!("Fin du compteur = {}", compteur);

    //We can store the result of a loop into a variable to test if somthing failed or not
    //Here the result is 20
    let resultat = loop {
        compteur += 1;

        if compteur == 10 {
            break compteur * 2;
        }
    };

    println!("Le résultat est {}", resultat);

    // Conventionnal while
    let mut nombre = 3;

    while nombre != 0 {
        println!("{} !", nombre);

        nombre -= 1;
    }

    // for to iterate on array and collecitons
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("La valeur est : {}", element);
    }

    // for with range and rev for reverse interval
    for nombre in (1..4).rev() {
        println!("{} !", nombre);
    }

}
