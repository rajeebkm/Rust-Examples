
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    print!("The secret number is : {}\n", secret_number);
    


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", &guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed a number that is smaller than secret number!"),
            Ordering::Greater => println!("You guessed a number that is greater than secret number!"),
            Ordering::Equal => {
                println!("Hurray! You guessed exactly as secret number. You win!");
                break;
            }
        }
    }
}
