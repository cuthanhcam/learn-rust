pub fn run() {
    const MAX_POINTS: u32 = 100_000; // constant
    println!("\n-- Variables --");

    let x = 5; // immutable
    let mut y = 10; // mutable

    println!("x: {x}, y: {y}");

    y = 20;
    println!("updated y: {y}");

    println!("constant MAX_POINTS: {MAX_POINTS}");

    println!();
}
