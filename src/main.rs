use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {

//1------------------------------------------------------------------------------
    // let name = String::from("Peter");
    // let fama = "Leon";
    // let data = [1,2,3,4,5];
    // println!("Hello, world!");
    // println!("Yo soy {} el {}", name, fama.to_uppercase());
    // println!("Mis datos {data:?}")

//2------------------------------------------------------------------------------
    // let correct = rand::thread_rng().gen_range(1..=10);
    // println!("Correct: {correct}");
    // println!("Hey, guess a number 1-10:");
    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // let guess: u32 = guess.trim().parse().expect("Error with parse.");

    // // let message = if correct < guess {
    // //     String::from("You guessed too high.")
    // // } else if correct > guess {
    // //     String::from("You guessed too low.")
    // // } else {
    // //     String::from("You guessed CORRECT")
    // // };

    // let message = match guess.cmp(&correct) {
    //     Ordering::Greater => "You guessed too high.",
    //     Ordering::Less => "You guessed too low.",
    //     Ordering::Equal => "You guessed CORRECT",  
    // };

    // println!("{}", message)

//3------------------------------------------------------------------------------
    // let correct = rand::thread_rng().gen_range(1..=10);
    // // println!("Correct: {correct}");
    // println!("Hey, guess a number 1-10:");
    // loop {
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(e) => {
    //             println!("Error with parse, try again. {e}");
    //             continue;
    //         }
    //     };
    
    //     match guess.cmp(&correct) {
    //         Ordering::Greater => println!("You guessed too high."),
    //         Ordering::Less => println!("You guessed too low."),
    //         Ordering::Equal => {
    //             println!("You guessed CORRECT");
    //             break;
    //         }  
    //     };
    // }

//4------------------------------------------------------------------------------
    let mut how_many = String::new();
    println!("How many random numbers you want to generate?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");
    
    let num_guess = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for _ in 0..num_guess {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    // println!("Correct: {correct:?}");

    let mut guesses_made = 0;

    println!("Hey, guess a number 1-10 :");
    while guesses_made < num_guess {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };
    
        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low."),
            Ordering::Equal => {
                println!("You guessed CORRECT");
                guesses_made += 1;
                if guesses_made < num_guess {
                    println!("Let's now try the next number!");
                }
            }  
        };
    }
    println!("Thanks for playing! The correct answers were:");
    for item in correct {
        println!("{item}")
    } 

}
