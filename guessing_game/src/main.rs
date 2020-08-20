use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    //loop keyword makes infinite loop
    loop{
    
        println!("Please input your guess.");

        // mut makes variable mutable - immutable by default
        let mut guess = String::new();

        //references also immutable by default
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // shadowing - used for type convesions no need for two variables
        //trim removes whitespace begining and end
        // : annotate variables type
        //match allows you to handle error except- works like a panic and crashes program.
        let guess: u32 = match guess.trim().parse() {
            //parse returns Result which has enums OK and Err
            Ok(num) => num,
            //_ is a catch all errors value
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
