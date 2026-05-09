//! Control flow: if expressions and looping constructs with examples.
//!
pub fn run() {
    println!("\n-- Control Flow --");

    if_expression();
    else_if_chain();
    if_in_let_statement();
    loop_example();
    loop_return_value();
    while_countdown();
    for_loop_examples();
    loop_labels();
}

/// Demonstrate `if` expressions and boolean conditions.
fn if_expression() {
    println!("\n[1] if Expressions");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("Rust requires the condition to be a bool.");
}

/// Show a chain of `else if` branches and first-match behavior.
fn else_if_chain() {
    println!("\n[2] else if Chains");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("Rust stops at the first branch that evaluates to true.");
}

/// Use `if` as an expression inside a `let` binding.
fn if_in_let_statement() {
    println!("\n[3] if in a let Statement");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    println!("Both branches must return the same type.");
}

/// Demonstrate `loop` with `break` to exit.
fn loop_example() {
    println!("\n[4] loop and break");

    let mut counter = 0;

    loop {
        counter += 1;
        println!("loop counter = {counter}");

        if counter == 3 {
            break;
        }
    }

    println!("break exits the current loop immediately.");
}

/// Show returning a value from a `loop` via `break`.
fn loop_return_value() {
    println!("\n[5] Returning Values from loops");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("A loop can yield a value with break <value>.");
}

/// Demonstrate `while` for condition-based repetition.
fn while_countdown() {
    println!("\n[6] while Loops");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

/// Examples using `for` over collections and ranges.
fn for_loop_examples() {
    println!("\n[7] for Loops");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("for loops are the idiomatic choice for collections and counted iteration.");
}

/// Show how loop labels disambiguate control in nested loops.
fn loop_labels() {
    println!("\n[8] Loop Labels");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 2;

        loop {
            println!("remaining = {remaining}");

            if remaining == 1 {
                break;
            }

            if count == 1 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}
