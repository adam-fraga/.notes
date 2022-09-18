fn main() {
    let x = plus_un(5);

    println!("La valeur de x est : {}", x);

    afficher_mesure_avec_unite(5, 'h');
    expression();
}

fn plus_un(x: i32) -> i32 {
    x + 1 //return keyword is facultatif in rust omit semicolumn work as a return
}

// All params must be typed in rust
fn afficher_mesure_avec_unite(valeur: i32, unite: char) {
    println!("La mesure est : {}{}", valeur, unite);
}

//Scope can be store as a variable it's an expression
fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("La valeur de y est : {}", y);
}
