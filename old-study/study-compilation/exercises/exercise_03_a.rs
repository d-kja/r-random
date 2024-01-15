fn main() {
    let user_age: i32 = 21;
    let minimum_required_age: i32 = 18;
    let has_user_met_the_requirements: bool = user_age >= minimum_required_age;

    if has_user_met_the_requirements {
        println!("Hello!");
        return;
    }

    println!("Goodbye");
}
