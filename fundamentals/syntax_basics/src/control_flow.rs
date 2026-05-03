pub fn run() {
    println!("\n-- Control Flow --");

    let number = 7;

    // if / else
    if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is not greater than 5", number);
    }

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break;
        }
    }

    println!("loop ended at {}", counter);

    // for
    for i in 0..5 {
        println!("for: {}", i);
    }

    // while
    let mut n = 3;
    while n > 0 {
        println!("while: {}", n);
        n -= 1;
    }

    // match
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3..=10 => println!("between three and ten"),
        _ => println!("other"),
    }

    println!();
}
