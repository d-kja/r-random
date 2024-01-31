use std::collections::HashMap;

#[derive(Debug)]
struct StoreItem {
    stock: u8,
   
}

fn main() {
    let mut items = HashMap::new();

    items.insert("chairs", StoreItem {
        stock: 5,
    });

    items.insert("beds", StoreItem {
        stock: 3,
    });

    items.insert("tables", StoreItem {
        stock: 2,
    });

    items.insert("couches", StoreItem {
        stock: 0,
    });

    for (key, value) in items.iter() {
        match value.stock {
           0 => println!("{key:?} - out of stock"),  
           _ => println!("{key:?} - {value:?}")
        };
    }

    let mut total = 0;

    for value in items.values() {
        total += value.stock;
    }

    println!("total: {}", total);
}
