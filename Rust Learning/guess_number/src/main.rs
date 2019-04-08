use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::fs;

fn main() {
    println!("Guess the number!");

    const MIN: u32 = 1;
    const MAX: u32 = 101;

    let data = fs::read_to_string("C:\\Users\\Turbo\\IdeaProjects\\rust_learning\\Rust Learning\\c-labb1\\src\\names.txt").expect("Can't open file");

    println!("{}", data);

    let secret_number: u32 = rand::thread_rng().gen_range(MIN, MAX);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess between {} and {}.", MIN, MAX);

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[ERROR] Please input a number");
                continue;
            }
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
