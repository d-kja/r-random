fn main() {
    let first_name: &str = "Nicolas";
    let last_name: &str = "Ribeiro";

    display_names(first_name, last_name);
}

pub fn display_names(first_name: &str, last_name: &str) {
    display_first_name(first_name);
    print!(" ");
    display_last_name(last_name);
}

fn display_first_name(first_name: &str) {
    print!("{first_name}");
}
fn display_last_name(last_name: &str) {
    print!("{last_name}\n");
}
