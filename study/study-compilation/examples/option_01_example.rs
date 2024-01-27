#[derive(Debug, Clone)]
struct GroceryItem {
    name: String,
    _qtd: u32,
}

fn main() {
    let groceries = vec![
        GroceryItem {
            name: "egg".to_owned(),
            _qtd: 12,
        },
        GroceryItem {
            name: "pizza".to_owned(),
            _qtd: 1,
        },
        GroceryItem {
            name: "coke".to_owned(),
            _qtd: 2,
        },
    ];

    let banana = get_by_name("banana", &groceries);

    match banana {
        Some(value) => println!("Item found! {value:?}"),
        None => println!("You didn't buy the banana........"),
    }

    let pizza = get_by_name("pizza", &groceries);

    match pizza {
        Some(value) => println!("Item found! {value:?}"),
        None => println!("You didn't buy the banana........"),
    }
}

// MY FIRST LIFETIME WITHOUT THE HELP OF THE COMPILER YAY
fn get_by_name<'a>(name: &str, arr: &'a Vec<GroceryItem>) -> Option<&'a GroceryItem> {
    for item in arr {
        if item.name == name {
            // So the reason behind the Option type here is the fact that we aren't certain if the {name}
            // will be in the Vector, obviously it's static as is but in real cases that wouldn't be a thing
            // and we wouldn't be able to know whether or not that name is in the volatile array
            return Some(item); // -> Option::Some(item)
        }
    }

    None
}
