pub fn run() {
    println!("\n-- Functions --");

    let sum = add(5, 10);
    println!("5 + 10 = {sum}");

    let squared = square(4);
    println!("4^2 = {squared}");

    println!();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(x: i32) -> i32 {
    x * x
}
