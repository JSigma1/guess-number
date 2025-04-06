use std::io;
use rand::Rng;

fn get_input() -> f64 {
    let mut value = String::new();
    println!("Enter number:");
    io::stdin().read_line(&mut value).expect("Failed to read line");
    
    match value.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            get_input() 
        }
    }
}

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Try to guess the number between 0 and 100.");
    
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..100);
    
    let max_attempts = 10;
    let mut attempts = 0;
    
    while attempts < max_attempts {
        println!("\nAttempt {} of {}", attempts + 1, max_attempts);
        let guess = get_input();
        
        if guess == random_number as f64 {
            println!("Congratulations! You guessed the number correctly!");
            break;
        } else if guess > random_number as f64 {
            println!("Your guess is too high.");
        } else {
            println!("Your guess is too low.");
        }
        
        attempts += 1;
        
        if attempts == max_attempts {
            println!("\nGame over! You've used all your attempts.");
            println!("The number was: {}", random_number);
        }
    }
}