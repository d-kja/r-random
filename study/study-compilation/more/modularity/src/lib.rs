use std::fmt::Debug;

pub trait Logger {
    fn print(self: &Self) -> ()
    where
        Self: Debug,
    {
        println!("{self:#?}");
    }

    fn display(&self) {
        println!("no fn display implemented")
    }
}

#[derive(Debug)]
pub struct Bash<'a> {
    pub stdin: &'a str,
    pub stdout: &'a str,
}

impl<'a> Logger for Bash<'a> {}

pub fn print_logger(value: impl Logger) {
    value.display();
}

pub fn print_logger_dyn(value: &dyn Logger) -> () {
    value.display();
}

pub mod initial_program {
    use super::Bash;

    mod hello_world {
        pub mod hello {
            pub fn handle() {
                println!("hello");
            }

            pub mod world {
                pub fn handle() {
                    println!("world");
                }
            }
        }
    }

    pub fn print() {
        crate::initial_program::hello_world::hello::handle();
        hello_world::hello::world::handle();

        // Crate is global for the current package
        crate::print_logger(Bash {
            stdin: "crate",
            stdout: "crate",
        });

        // super is to access functionality outside the current module (MOD)
        super::print_logger_dyn(&Bash {
            stdin: "super",
            stdout: "super",
        });
    }
}
