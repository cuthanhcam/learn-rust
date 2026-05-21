//! Example Program Using Structs: Rectangle Area
//!
//! - Area calculation with variables, tuple, and struct
//! - Debug printing
//! - Refactoring for clarity

pub fn run() {
    println!("\n-- Example Program Using Structs --");
    area_with_variables();
    area_with_tuple();
    area_with_struct();
    debug_printing();
}

fn area_with_variables() {
    let width1 = 30;
    let height1 = 50;
    println!("area (variables): {}", area_vars(width1, height1));
}

fn area_vars(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple() {
    let rect1 = (30, 50);
    println!("area (tuple): {}", area_tuple(rect1));
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area (struct): {}", area_struct(&rect1));
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn debug_printing() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 pretty:\n{rect1:#?}");

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}
