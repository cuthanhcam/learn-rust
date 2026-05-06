use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

/// Difficulty levels for the guessing game
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Entry point for the improved guessing game
pub fn run() {
    println!("Improved Guessing Game");

    let difficulty = choose_difficulty();
    let (min, max) = difficulty_range(&difficulty);

    println!("Guess a number between {min} and {max}");

    let secret_number = generate_secret(min, max);

    loop {
        // let guess = match read_guess(min, max) {
        //     Some(num) => num,
        //     None => continue,
        // };

        // Alternative using let-else for cleaner code
        let Some(guess) = read_guess(min, max) else {
            continue;
        };

        println!("You guessed: {guess}");

        if handle_guess(guess, secret_number) {
            break;
        }
    }
}

/// Function to choose difficulty level
fn choose_difficulty() -> Difficulty {
    println!("Choose difficulty level:");
    println!("1. Easy (1-50)");
    println!("2. Medium (1-100)");
    println!("3. Hard (1-200)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "1" => Difficulty::Easy,
        "2" => Difficulty::Medium,
        "3" => Difficulty::Hard,
        _ => {
            println!("Invalid choice, defaulting to Medium.");
            Difficulty::Medium
        }
    }
}

/// Function to get the range of numbers based on difficulty
fn difficulty_range(difficulty: &Difficulty) -> (u32, u32) {
    match difficulty {
        Difficulty::Easy => (1, 50),
        Difficulty::Medium => (1, 100),
        Difficulty::Hard => (1, 200),
    }
}

/// Function to generate a random secret number within the specified range
fn generate_secret(min: u32, max: u32) -> u32 {
    thread_rng().gen_range(min..=max)
}

/// Function to read and validate the user's guess
fn read_guess(min: u32, max: u32) -> Option<u32> {
    println!("Please input your guess:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) if num >= min && num <= max => Some(num),
        Ok(_) => {
            println!("Number must be between {min} and {max}");
            None
        }
        Err(_) => {
            println!("Please enter a valid number!");
            None
        }
    }
}

/// Compare guess with secret number. Returns true if correct
fn handle_guess(guess: u32, secret: u32) -> bool {
    match guess.cmp(&secret) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
