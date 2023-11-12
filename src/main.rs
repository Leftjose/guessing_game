use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;



fn main() {
    //This will print the text "Guess the number!"
    println!("Guess the number!");

    //This line will generate a number between 1-100
    let secret_number = rand::thread_rng().gen_range(1,101);

    //This line will print the line "The secret number is" + whatever random number the line above will generate
    println!("The secret number is: {}", secret_number);

    //We created a *loop* to continue requesting a new guess until the correct number is guessed
    loop {
        println!("Please input your guess.");

    //We are making guess mutable beacuase we want to be able to change it from a string to a u32 integer
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //reassigning *guess* 
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,

    };

    //defining is our guess is correct, too high or too low, we also added color and a *break* so the code only runs till we guess the correct number
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".red()),
        Ordering::Equal => {
            println!("{}", "You win!".green());
            break;
        },
      }

    }

    
}
