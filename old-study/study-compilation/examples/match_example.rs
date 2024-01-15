fn main() {
    let ref_value = "Bao";

    match ref_value {
        "bob" => println!("what a silly name..."),
        "Bob" => println!("So ya callin' yourself Bob huh... how pretentious"),
        "bOb" => println!("That seems silly, why would you call yourself bOb huh?"),
        "boB" => println!("Weirdo..."),
        _ => println!("it ain't bob..."),
    };
}
