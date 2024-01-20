use std::fmt;

enum Access {
    FULL,
    PARTIAL,
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Access::FULL => write!(f, "{}", "FULL".to_lowercase()),
            Access::PARTIAL => write!(f, "{}", "PARTIAL".to_lowercase()),
        }
    }
}

struct User {
    name: String,
    age: u8,
    access_type: Access,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {},\n Age: {},\n Access Type: {}",
            self.name, self.age, self.access_type
        )
    }
}

pub fn example() {
    let admins = gen_users();
    println!("# Henrique:\n {},\n\n # Andre:\n {}", admins.0, admins.1);
}

fn gen_users() -> (User, User) {
    (
        User {
            name: "Andre".to_string(),
            age: 38,
            access_type: Access::PARTIAL,
        },
        User {
            name: "Henrique".to_string(),
            age: 32,
            access_type: Access::FULL,
        },
    )
}
