#[derive(Debug)]
enum Status {
    AUTHORIZED,
    UNAUTHORIZED,
}

#[derive(Clone, Debug)]
enum AccessLevel {
    HIGH,    // 1000
    _MEDIUM, // 800
    LOW,     // 500
}

impl AccessLevel {
    fn get_level(&self) -> u16 {
        match self {
            AccessLevel::HIGH => 1000,
            AccessLevel::_MEDIUM => 800,
            AccessLevel::LOW => 500,
        }
    }
}

#[derive(Clone, Debug)]
struct Employee {
    name: String,
    keycard: Option<AccessLevel>,
}

impl Employee {
    fn get_keycard(&self) -> Result<&AccessLevel, &str> {
        match &self.keycard {
            Some(value) => Ok(value),
            None => Err("doesn't have a keycard"),
        }
    }
}

struct Database {
    employees: Vec<Employee>,
}

impl Database {
    fn connect() -> Self {
        Self {
            employees: Vec::new(),
        }
    }

    fn update_employees(&mut self, employees: Vec<Employee>) {
        self.employees = employees;
    }

    fn _filter_employee(&self, name: &str) {
        let employee = self
            .employees
            .iter()
            .filter(|&value| {
                if value.name.eq(name) {
                    return true;
                }

                false
            })
            .cloned()
            .collect::<Vec<Employee>>();

        println!("{employee:?}");
    }

    fn find_employee(&self, name: &str) -> Result<&Employee, &str> {
        let employee = self.employees.iter().find(|&value| value.name.eq(name));

        match employee {
            Some(value) => Ok(value),
            None => Err("employee not found"),
        }
    }
}

struct Location {
    required_access_level: u16,
}

impl Location {
    fn has_access(self: &Self, keycard: &AccessLevel) -> Result<Status, String> {
        let level = keycard.get_level();

        if level < self.required_access_level {
            return Err(format!(
                "doesn't meet the requirements, level required {} is higher than {}",
                self.required_access_level, level
            ));
        }

        Ok(Status::AUTHORIZED)
    }
}

fn main() -> Result<(), String> {
    let result = authorize();

    if let Err((err, status)) = result {
        println!("Access not granted! {} | Status: {:?}", err, status);
    }

    Ok(())
}

fn authorize() -> Result<Status, (String, Status)> {
    let mut db = Database::connect();

    // Private location, requires high access level
    let location = Location {
        required_access_level: 900,
    };

    // Creating mock data
    db.update_employees(vec![
        Employee {
            name: "john".to_owned(),
            keycard: None,
        },
        Employee {
            name: "jose".to_owned(),
            keycard: Some(AccessLevel::LOW),
        },
    ]);

    let res = db.find_employee("john");

    if let Ok(employee) = res {
        let employee_keycard = match employee.get_keycard() {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        };

        let has_access: Result<Status, String> = location.has_access(&employee_keycard);

        return match has_access {
            Ok(_) => {
                println!("granted");

                Ok(Status::AUTHORIZED)
            }
            Err(err) => Err((err, Status::UNAUTHORIZED)),
        };
    }

    Err(("employee not found".to_owned(), Status::UNAUTHORIZED))
}
