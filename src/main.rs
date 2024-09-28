
// Opgave 1: Magisk terningkast
use std::io;
use rand::Rng; // Til at generere tilfældige tal

fn main() {
    let mut input = String::new();
    println!("Hvor mange terninger vil du kaste?");

    io::stdin().read_line(&mut input)
        .expect("Fejl ved læsning af input");

    let number_of_dice: i32 = input.trim().parse().expect("Indtast venligst et tal!");

    let mut rng = rand::thread_rng();
    let mut sum = 0;

    for _ in 0..number_of_dice {
        let roll = rng.gen_range(1..=6);
        sum += roll;
        println!("Du kastede: {}", roll);
    }

    if sum == 12 {
        println!("Magisk kast! Summen er 12!");
    } else {
        match sum {
            7 => println!("Heldigt kast, summen er 7!"),
            _ => println!("Summen af kastene er: {}", sum),
        }
    }
}

// Opgave 2: Automatisk kaffemaskine
use std::io;

fn main() {
    let espresso = vec!["Vand", "Espresso bønner"];
    let latte = vec!["Vand", "Espresso bønner", "Mælk"];
    let cappuccino = vec!["Vand", "Espresso bønner", "Mælk", "Skum"];

    println!("Vælg din kaffe: 1) Espresso, 2) Latte, 3) Cappuccino");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Fejl ved læsning af input");

    let choice: i32 = input.trim().parse().expect("Indtast venligst et tal!");

    match choice {
        1 => print_ingredients("Espresso", &espresso),
        2 => print_ingredients("Latte", &latte),
        3 => print_ingredients("Cappuccino", &cappuccino),
        _ => println!("Ugyldigt valg!"),
    }
}

fn print_ingredients(coffee_name: &str, ingredients: &Vec<&str>) {
    println!("{} ingredienser:", coffee_name);
    for ingredient in ingredients {
        println!("- {}", ingredient);
    }
}

// Opgave 3: Skattejagt med Ownership og Borrowing
fn main() {
    let mut room1 = String::from("Guld");
    let mut room2 = String::from("Sølv");

    println!("Du er i rum 1, der indeholder: {}", &room1);
    take_treasure(&mut room1, &mut room2);

    println!("Efter at have taget skatten, har rum 1: {}", room1);
    println!("Rum 2 indeholder nu: {}", room2);
}

fn take_treasure(from: &mut String, to: &mut String) {
    *to = from.clone(); // Flyt skatten fra ét rum til et andet
    *from = String::from("Tomt");
}

// Opgave 4: Divisionsmester med fejlhåndtering
use std::io;

fn main() {
    let mut input = String::new();

    println!("Indtast to tal til division:");

    io::stdin().read_line(&mut input).expect("Kunne ikke læse input");
    let numbers: Vec<&str> = input.trim().split_whitespace().collect();

    let a: i32 = numbers[0].parse().expect("Indtast venligst et gyldigt tal");
    let b: i32 = numbers[1].parse().expect("Indtast venligst et gyldigt tal");

    match divide(a, b) {
        Ok(result) => println!("Resultatet af divisionen er: {}", result),
        Err(e) => println!("Fejl: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Kan ikke dividere med nul")
    } else {
        Ok(a / b)
    }
}
