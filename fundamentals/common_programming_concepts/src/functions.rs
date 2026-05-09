//! Functions: examples of definition, parameters, statements/expressions and returns.
//!
pub fn run() {
    println!("\n-- Functions --");

    another_function();
    print_labeled_measurement(5, 'h');
    statements_and_expressions();
    return_values();
}

/// Simple example function to show definition and invocation.
fn another_function() {
    println!("\n[1] Defining and Calling Functions");
    println!("Another function ran after main called it.");
    println!("Rust lets you define functions before or after their call site.");
}

/// Demonstrate typed parameters in a function signature.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("\n[2] Parameters");
    println!("The measurement is: {value}{unit_label}");
    println!("Function parameters must declare their types in the signature.");
}

/// Show difference between statements and expressions including block expressions.
fn statements_and_expressions() {
    println!("\n[3] Statements and Expressions");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The block expression evaluates to y = {y}");
    println!("Statements do not return values, but expressions do.");
    println!("A trailing semicolon turns an expression into a statement that returns ().");
}

/// Demonstrate functions that return values and implicit returns.
fn return_values() {
    println!("\n[4] Return Values");

    let x = five();
    let y = plus_one(5);
    let z = half_or_zero(8);

    println!("five() returned {x}");
    println!("plus_one(5) returned {y}");
    println!("half_or_zero(8) returned {z}");
    println!("The final expression in a function body is the implicit return value.");
}

/// Return a constant integer.
fn five() -> i32 {
    5
}

/// Increment a number by one and return it.
fn plus_one(x: i32) -> i32 {
    x + 1
}

/// Return half of `x` or zero if `x` is zero (example of early return).
fn half_or_zero(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    x / 2
}
