//! Pattern matching with `match`.
//!
//! This module demonstrates:
//! - Basic `match` expressions
//! - Exhaustive matching
//! - Multiple patterns
//! - Range patterns
//! - Match guards
//! - Destructuring enums
//! - Ignoring values with `_`
//! - Binding values
//! - Matching `Option<T>`
//! - Matching enums with associated data

pub fn run() {
    println!("\n-- The match Control Flow Construct --");

    bacic_match();
    multi_pattern_match();
    state_quarter_match();
    exhaustive_match();
    tuple_destructuring_match();
    enum_destructuring_match();
    match_guards_demo();
    if_let_demo();
}

// Basic match
fn bacic_match() {
    println!("-- Basic Match --");

    let dice_roll = 3;

    match dice_roll {
        1 => println!("You rolled a 1"),
        2 => println!("You rolled a 2"),
        3 => println!("You rolled a 3"),
        // `_` = catch-all pattern
        _ => println!("You rolled something else"),
    }

    println!();
}

// Multiple patterns + ranges
fn multi_pattern_match() {
    println!("-- Multiple Patterns + Ranges --");

    let number = 6;

    match number {
        // Multiple patterns with `|`
        1 | 2 => println!("One or Two"),

        // Inclusive range pattern with `..=`
        3..=5 => println!("Three through Five"),

        6..=10 => println!("Six through Ten"),

        _ => println!("Something else"),
    }

    println!();
}

// Matching enums
#[derive(Debug)]
enum UsState {
    Alaska,
    California,
    Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }

        Coin::Nickel => 5,

        Coin::Dime => 10,

        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn state_quarter_match() {
    println!("--- MATCHING ENUMS ---");

    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::California),
        Coin::Quarter(UsState::Texas),
    ];

    for coin in &coins {
        println!("{coin:?} => {} cents", value_in_cents(coin));
    }

    println!();
}

// Exhaustive matching
fn exhaustive_match() {
    println!("--- EXHAUSTIVE MATCH ---");

    let optional = Some(0);

    match optional {
        Some(0) => println!("zero"),

        // Match guard
        Some(value) if value > 0 => {
            println!("positive: {value}");
        }

        Some(value) => {
            println!("some other number: {value}");
        }

        None => println!("nothing"),
    }

    println!();
}

// Tuple destructuring
fn tuple_destructuring_match() {
    println!("--- TUPLE DESTRUCTURING ---");

    let point = (3, 7);

    match point {
        (0, 0) => println!("Origin"),

        (0, y) => println!("On Y axis at y = {y}"),

        (x, 0) => println!("On X axis at x = {x}"),

        (x, y) => println!("Point at ({x}, {y})"),
    }

    println!();
}

// Enum destructuring
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_destructuring_match() {
    println!("--- ENUM DESTRUCTURING ---");

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello rust")),
        Message::ChangeColor(255, 128, 0),
    ];

    for message in messages {
        match message {
            Message::Quit => {
                println!("Quit message");
            }

            Message::Move { x, y } => {
                println!("Move to ({x}, {y})");
            }

            Message::Write(text) => {
                println!("Text message: {text}");
            }

            Message::ChangeColor(r, g, b) => {
                println!("RGB({r}, {g}, {b})");
            }
        }
    }

    println!();
}

// Match guards
fn match_guards_demo() {
    println!("--- MATCH GUARDS ---");

    let number = Some(8);

    match number {
        Some(value) if value % 2 == 0 => {
            println!("{value} is even");
        }

        Some(value) => {
            println!("{value} is odd");
        }

        None => {
            println!("No number");
        }
    }

    println!();
}

// if let
fn if_let_demo() {
    println!("--- IF LET ---");

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("Configured maximum = {max}");
    } else {
        println!("No configured maximum");
    }

    println!();
}
