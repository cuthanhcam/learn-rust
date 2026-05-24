//! Defining enums and using them to model related values.
//!
//! This module demonstrates:
//! - Basic enums
//! - Enums with associated data
//! - Struct-like enum variants
//! - Methods on enums
//! - Pattern matching with `match`
//! - The `Option<T>` enum
//! - Exhaustive matching
//! - Enum utility methods

pub fn run() {
    println!("\n-- ENUMS IN RUST --");

    ip_address_demo();
    message_demo();
    coin_demo();
    option_demo();
}

// Basic enums

#[derive(Debug, Clone, Copy)]
enum IpAddrKind {
    V4,
    V6,
}

// Enums with associated data

// Each variant can store different data.

#[derive(Debug)]
enum IpAddr {
    // Tuple-like variants
    V4(u8, u8, u8, u8),

    // IPv6 is represented differently
    V6(String),
}

impl IpAddr {
    fn display(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => {
                println!("IPv4 Address => {a}.{b}.{c}.{d}");
            }

            IpAddr::V6(address) => {
                println!("IPv6 Address => {address}");
            }
        }
    }

    fn version(&self) -> IpAddrKind {
        match self {
            IpAddr::V4(..) => IpAddrKind::V4,
            IpAddr::V6(..) => IpAddrKind::V6,
        }
    }
}

// Enums can hold different types of data

#[derive(Debug)]
enum Message {
    // No associated data
    Quit,

    // Struct-like variant
    Move { x: i32, y: i32 },

    // Tuple-like variant
    Write(String),

    // Multiple values
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Message::Quit -> Program is quitting");
            }

            Message::Move { x, y } => {
                println!("Message::Move -> Moving to ({x}, {y})");
            }

            Message::Write(text) => {
                println!("Message::Write -> \"{text}\"");
            }

            Message::ChangeColor(r, g, b) => {
                println!("Message::ChangeColor -> RGB({r}, {g}, {b})");
            }
        }
    }
}

// Enums + Match expressions

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,

    // Variant with associated data
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    // `match` must be exhaustive.
    // Every possible variant must be handled.
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

// Option<T>
//
// Rust does NOT have null.
// Instead, it uses Option<T>.
//
// enum Option<T> {
//     None,
//     Some(T),
// }
//

fn plus_one(value: Option<i32>) -> Option<i32> {
    value.map(|number| number + 1)
}

// Demostrations

fn ip_address_demo() {
    println!("--- IP ADDRESS DEMO ---");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Kinds:");
    println!("  {four:?}");
    println!("  {six:?}");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    home.display();
    loopback.display();

    println!("home version => {:?}", home.version());
    println!("loopback version => {:?}", loopback.version());

    println!();
}

fn message_demo() {
    println!("--- MESSAGE ENUM DEMO ---");

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello rust")),
        Message::ChangeColor(255, 128, 0),
    ];

    for message in messages {
        message.call();
    }

    println!();
}

fn coin_demo() {
    println!("--- COIN ENUM DEMO ---");

    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::Arizona),
        Coin::Quarter(UsState::California),
    ];

    for coin in &coins {
        println!("{coin:?} => {} cents", value_in_cents(coin));
    }

    println!();
}

fn option_demo() {
    println!("--- OPTION<T> DEMO ---");

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!("some_number = {some_number:?}");
    println!("absent_number = {absent_number:?}");

    println!("plus_one(some_number) = {:?}", plus_one(some_number));

    println!("plus_one(absent_number) = {:?}", plus_one(absent_number));

    // Using `if let`
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("Configured max = {max}");
    }

    // unwrap_or example
    //
    // `std::env::var()` returns Result<String, VarError>
    // `.ok()` converts it into Option<String>
    let username = std::env::var("USER").ok();

    println!("Username = {}", username.unwrap_or(String::from("guest")));

    // match with Option
    let age = Some(18);

    match age {
        Some(value) if value >= 18 => {
            println!("Adult");
        }

        Some(value) => {
            println!("Minor: {value}");
        }

        None => {
            println!("No age provided");
        }
    }

    println!();
}
