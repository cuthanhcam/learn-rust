//! Defining and Instantiating Structs
//!
//! - Named structs
//! - Tuple structs
//! - Unit-like structs
//! - Field init shorthand
//! - Struct update syntax

pub fn run() {
    println!("\n-- Defining and Instantiating Structs --");
    named_struct();
    tuple_struct();
    unit_struct();
    field_init_shorthand();
    struct_update_syntax();
}

// Silence warnings for unused fields
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn named_struct() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Proper way to print individual fields
    println!("user1 email: {}", user1.email);
    println!("user1 username: {}", user1.username);
    println!("user1 sign-in count: {}", user1.sign_in_count);

    user1.email = String::from("anotheremail@example.com");
    println!("user1 after email change: {user1:?}");
}

fn field_init_shorthand() {
    let email = String::from("shorthand@example.com");
    let username = String::from("shorthanduser");
    let user = build_user(email, username);
    println!("user (field init shorthand): {user:?}");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn struct_update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };
    println!("user2 (struct update syntax): {user2:?}");
    // Note: user1.username is moved, so user1 cannot be used after this.
}

#[allow(dead_code)]
#[derive(Debug)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
#[derive(Debug)]
struct Point(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {black:?}, origin: {origin:?}");
}

#[allow(dead_code)]
#[derive(Debug)]
struct AlwaysEqual;

fn unit_struct() {
    let subject = AlwaysEqual;
    println!("unit struct: {subject:?}");
}
