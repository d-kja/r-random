pub enum Colors {
    Red,
    Blue,
    Green,
}

pub fn example(color: Colors) {
    match color {
        Colors::Red => println!("Red"),
        Colors::Blue => println!("Blue"),
        Colors::Green => println!("Green"),
    };
}
