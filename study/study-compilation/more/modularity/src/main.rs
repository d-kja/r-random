use modularity::{Bash, Logger};

use modularity::initial_program::print;
// not public
// use crate::custom_module::example::example_nested;

fn main() {
    let bash = Bash {
        stdin: "input",
        stdout: "output",
    };

    bash.print();
    bash.display();

    print();
}
