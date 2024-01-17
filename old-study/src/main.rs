use bin::exercise_5a;

mod bin {
    pub mod exercise_5a;
}

fn main() {
    exercise_5a::example();
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
