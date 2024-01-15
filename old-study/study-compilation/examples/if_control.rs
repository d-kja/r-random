pub fn example() {
    let age = 16;

    if age >= 18 {
        println!("You can drink and drive");
    } else if age >= 16 && age <= 17 {
        println!("No you can't but you're close buddy");
    } else {
        println!("No, you can neither drive nor drink, because you're underage");
    }
}
