// bring input/output library into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // ! means macro
    // macro not equals function
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // let create new variable
        // by default variable is immutable
        // if we want to make a mutable variable
        // we must write mut before variable name
        let mut guess = String::new();
        // new is a associated function of String type
        // that mean function is implemented on a type

        io::stdin()
            // & indicates that argument is a reference
            // &mut makes reference mutable
            .read_line(&mut guess)
            // if read_line func return Err variant of Result(enum)
            // then expect will cause the program crash
            // and display message
            .expect("Failed to read line");

        // rust allows to shadow the previous value with a new one
        // trim will eliminate whitespaces at the beginning and end
        // parse method on strings converts a string to another type
        // : after guess tells Rust annotate the variable's type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match expression is made up of arms
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
