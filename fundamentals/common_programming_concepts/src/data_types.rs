//! Data types: scalar and compound examples and notes.
//!
//! - scalar types (integers, floats, bool, char)
//! - type annotations and parsing
//!
pub fn run() {
    println!("\n-- Data Types --");

    scalar_types();
    type_annotations();
    compound_types();
}

/// Demonstrate scalar primitives and basic numeric operations.
fn scalar_types() {
    println!("\n[1] Scalar Types");

    let integer: i32 = 42;
    let floating_point = 2.0;
    let precise: f32 = 3.0;
    let truth = true;
    let letter = 'z';
    let cat = '😻';

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("integer = {integer}");
    println!("floating_point = {floating_point}");
    println!("precise = {precise}");
    println!("truth = {truth}");
    println!("letter = {letter}");
    println!("cat = {cat}");
    println!("sum = {sum}");
    println!("difference = {difference}");
    println!("product = {product}");
    println!("quotient = {quotient}");
    println!("remainder = {remainder}");

    println!("Integers default to i32 when Rust can infer the type safely.");
}

/// Show type annotations and parsing where the compiler needs help.
fn type_annotations() {
    println!("\n[2] Type Annotations");

    let guess: u32 = "42".parse().expect("Not a number!");
    let explicit_width: u8 = 255;

    println!("guess = {guess}");
    println!("explicit_width = {explicit_width}");
    println!("The compiler uses type annotations when multiple types are possible.");
    println!("Rust uses static typing, so every variable must have one known type.");
}

/// Overview of compound types: tuples and arrays.
fn compound_types() {
    println!("\n[3] Compound Types");

    tuple_example();
    array_example();
}

/// Show tuple creation, destructuring and indexed access.
fn tuple_example() {
    println!("\n[3.1] Tuples");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("tup = {tup:?}");
    println!("destructured values: x = {x}, y = {y}, z = {z}");
    println!(
        "direct access: tup.0 = {}, tup.1 = {}, tup.2 = {}",
        tup.0, tup.1, tup.2
    );
    println!("Tuples can hold values of different types but their length is fixed.");
}

/// Show arrays, fixed length initialization and indexing.
fn array_example() {
    println!("\n[3.2] Arrays");

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let repeated: [i32; 5] = [3; 5];

    println!("first = {}", a[0]);
    println!("second = {}", a[1]);
    println!("months[0] = {}", months[0]);
    println!("repeated = {repeated:?}");
    println!("Arrays have the same type for every element and a fixed length.");
    println!("If you use an invalid index, Rust checks bounds at runtime and panics.");
}
