//! This module demonstrates how Rust handles.
//!
//! - immutable variables (default)
//! - mutable variables (`mut`)
//! - constants
//! - shadowing
//! - scope-based shadowing
//! - type transformation via shadowing
//! - `mut` vs shadowing differences

/// Global constants (compile-time values)
const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const THREE_HOURS_IN_SECONDS: u32 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR * 3;
const MAX_PLAYERS: u8 = 4;

// Entry point
pub fn run() {
    println!("\n-- Variables and Mutability --");

    immutable_variables();
    mutable_variables();
    constants_example();
    shadowing_example();
    shadowing_with_scope();
    shadowing_type_transformation();
    mutability_and_ownership();
    mut_vs_shadowing();
}

/// 1. Immutable Variables
fn immutable_variables() {
    println!("\n[1] Immutable Variables");

    let x = 5;
    println!("Initial value of x: {x}");

    // x = 10; // compile-time error: immutable variable

    println!("x is immutable, so it cannot be changed after assignment.");
}

/// 2. Mutable Variables
fn mutable_variables() {
    println!("\n[2] Mutable Variables");

    let mut counter = 0;

    counter += 1;
    println!("counter after increment: {counter}");

    let mut score = 100;
    score -= 25;

    println!("score after decrement: {score}");
}

/// 3. Constants
fn constants_example() {
    println!("\n[3] Constants");

    println!("3 hours = {THREE_HOURS_IN_SECONDS} seconds");
    println!("Max players = {MAX_PLAYERS}");
}

/// 4. Shadowing
fn shadowing_example() {
    println!("\n[4] Shadowing");

    let x = 5;
    println!("x = {x}");

    let x = x + 1; // shadows previous x
    println!("x after shadowing: {x}");

    let x = "Now I'm a string"; // shadows previous x with a different type
    println!("x after type transformation: {x}");
}

/// 5. Scope Shadowing
fn shadowing_with_scope() {
    println!("\n[5] Scope Shadowing");

    let x = 10;
    println!("Outer scope x: {x}");

    {
        let x = 20; // shadows outer x within this block
        println!("Inner scope x: {x}");
    }

    println!("Outer scope x after inner block: {x}");
}

/// 6. Type Transformation via Shadowing
fn shadowing_type_transformation() {
    println!("\n[6] Shawdowing Type Change");

    let spaces = "   ";
    println!("spaces (str): '{spaces}'");

    let spaces = spaces.len(); // shadows previous 'spaces' with a new type (usize)
    println!("spaces (len): {spaces}");

    // mut version would fail:
    // let mut spaces = "   ";
    // spaces = spaces.len(); // compile-time error: cannot assign usize to &str
}

/// 7. Mutability + Ownership
fn mutability_and_ownership() {
    println!("\n[7] Mutability + Ownership");

    let mut message = String::from("Hello");
    println!("{message}");

    message.push_str(", world!");
    println!("{message}");
}

/// 8. mut vs Shadowing
fn mut_vs_shadowing() {
    println!("\n[8] mut vs Shadowing");

    let mut value = 10;
    println!("Initial mutable value: {value}");

    value += 5; // modifies the same variable
    println!("After mutation: {value}");

    let value = value.to_string(); // shadows previous 'value' with a new variable of type String
    println!("After shadowing with type change: {value}");
}
