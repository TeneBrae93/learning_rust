use std::io; // Use the io library from the standard libary
use std::cmp::Ordering; // Use the cmp library from the standard library and Ordering 
use rand::Rng; // Use the rand library from the standard library

fn main() {
    println!("Guess the number!"); 

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100 

    loop { 
    
        println!("Please input your guess."); 

        let mut guess = String::new(); // Create a mutable variable to store user input

        io::stdin()
            .read_line(&mut guess) // Reading user input from stdin 
            .expect("Failed to read line"); 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue, 
        };  
        
        match guess.cmp(&secret_number) { 
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => { 
                println!("You win!");
                break;  
            }
        } 

    }
} 