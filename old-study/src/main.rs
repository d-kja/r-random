use bin::exercise_6a;

mod bin {
    pub mod exercise_6a;
}

fn main() {
    exercise_6a::example();
}

// let user_input = loop {
//     let mut input: String = String::new();
//     match io::stdin().read_line(&mut input) {
//         Ok(_) => {
//             break input;
//         }
//         Err(_) => continue,
//     }
// };
// println!("{:}", user_input.trim_end());
