//! Methods and Associated Functions on Structs
//!
//! - Defining methods with impl
//! - &self, &mut self, and self
//! - Associated functions (constructors)
//! - Method syntax

pub fn run() {
    println!("\n-- Methods and Associated Functions --");
    area_method();
    can_hold_method();
    associated_function();
    method_same_name_as_field();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn area_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area (method): {}", rect1.area());
}

fn can_hold_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn associated_function() {
    let square = Rectangle::square(30);
    println!("square (associated function): {square:?}");
}

fn method_same_name_as_field() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
