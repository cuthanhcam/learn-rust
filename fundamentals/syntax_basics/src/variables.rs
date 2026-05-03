pub fn run() {
    println!("\n-- Variables --");

    let x = 5; // immutable
    let mut y = 10; // mutable

    println!("x: {}, y: {}", x, y);

    y = 20;
    println!("updated y: {}", y);

    const MAX_POINTS: u32 = 100_000; // constant
    println!("constant MAX_POINTS: {}", MAX_POINTS);

    println!();
}
