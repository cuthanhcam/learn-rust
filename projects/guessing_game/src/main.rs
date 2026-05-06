mod improved;
mod simple;

fn main() {
    println!("Choose version:");
    println!("1 - Simple");
    println!("2 - Improved");

    let mut choice = String::new();

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => simple::run(),
        "2" => improved::run(),
        _ => println!("Invalid choice"),
    }
}
