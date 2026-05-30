//! Concise control flow with `if let` and `let...else`.
// ! `if let` and `let...else` are powerful tools for handling pattern matching in a more concise way. They allow you to match on specific patterns while providing a clear and readable syntax for handling the cases where the pattern does not match. In this example, we demonstrate how to use `if let` to check for a favorite color and `let...else` to handle responses and error paths effectively.

pub fn run() {
    println!("\n-- Concise Control Flow with if let and let...else --");

    if_let_demo();
    let_else_demo();
    let_else_error_path();
}

fn if_let_demo() {
    let favorite_color = Some("green");

    if let Some(color) = favorite_color {
        println!("Your favorite color is {color}");
    } else {
        println!("No favorite color was set");
    }
}

fn let_else_demo() {
    let response = Some("accepted");

    let Some(status) = response else {
        println!("No status available");
        return;
    };

    println!("Status = {status}");
}

fn let_else_error_path() {
    let maybe_name: Option<&str> = None;

    let Some(name) = maybe_name else {
        println!("let...else can exit early when data is missing");
        return;
    };

    println!("Name = {name}");
}
