use bin::exercise_7::{self, Colors};

mod bin {
    pub mod exercise_7;
}

fn main() {
    exercise_7::example(Colors::Green);
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
