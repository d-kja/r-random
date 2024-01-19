use bin::exercise_8;

mod bin {
    pub mod exercise_8;
}

fn main() {
    exercise_8::example();
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
