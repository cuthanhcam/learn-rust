pub fn run() {
    println!("\n-- Data Types --");

    let integer: i32 = 13;
    let float: f64 = std::f64::consts::PI;
    let boolean: bool = true;
    let character: char = 'C';

    let tuple: (i32, f64, char) = (10, 1.59, 'N');
    let (a, b, c) = tuple; // destructuring

    let array = [1, 2, 3, 4, 5];

    println!("integer = {integer}");
    println!("float = {float}");
    println!("bool = {boolean}");
    println!("char = {character}");

    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    println!("destructuring = ({a}, {b}, {c})");
    println!("array = {array:?}");
    println!("array[0] = {}", array[0]);

    println!();
}
