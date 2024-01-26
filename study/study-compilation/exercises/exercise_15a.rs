// MY SOLUTION

use core::fmt;

// #[derive(Debug)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

impl fmt::Debug for TicketType {
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketType::Backstage(value) => {
                write!(formatter, "Backstage (holder's name: {})", value)
            }
            TicketType::Vip(value) => write!(formatter, "Vip (holder's name: {})", value),
            TicketType::Standard => write!(formatter, "Standard"),
        }
    }
}

struct Ticket {
    price: f32,
    _type: TicketType,
}

impl Ticket {
    fn new(_type: TicketType, price: f32) -> Self {
        Self { _type, price }
    }

    fn print(&self) {
        println!(
            "Ticket\n - type: {:?},\n - price: {:?}",
            self._type, self.price
        );
    }
}

fn main() {
    let tickets = vec![
        Ticket::new(TicketType::Standard, 0.0),
        Ticket::new(TicketType::Backstage("John".to_owned()), 0.0),
        Ticket::new(TicketType::Vip("Jose".to_owned()), 0.0),
    ];

    for ticket in tickets {
        ticket.print();
    }
}
