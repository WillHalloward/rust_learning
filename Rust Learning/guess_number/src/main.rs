use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let min = 1;
    let max = 101;

    let secret_number:u32 = rand::thread_rng().gen_range(min, max);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess between {} and {}.", min, max);

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[ERROR] Please input a number");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}