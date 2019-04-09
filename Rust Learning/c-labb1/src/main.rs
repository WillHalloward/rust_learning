use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io::stdin;

use rand::Rng;

fn main() {
    let mut p_vec: Vec<Person> = Vec::new();
    loop {
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[ERROR] Please input a number");
                continue;
            }
        };

        match choice {
            1 => {
                let mut name = String::new();
                println!("What name do you want to search for?");
                stdin().read_line(&mut name).expect("Failed to read line");
                println!("{}", find_in_names(p_vec, name))
            }
            2 => {
                let mut city = String::new();
                println!("What city do you want to search for?");
                stdin().read_line(&mut city).expect("Failed to read line");
                let mut c_vec: Vec<String>;
                c_vec = find_person_from_city(p_vec, city)
            }
            3 => {
                println!("Exiting!");
                break;
            }
            _ => println!("[Error] Not a choice"),
        };
    }
}

fn find_in_names(p_vec: Vec<Person>, name: String) -> u32 {
    return 0;
}

fn find_person_from_city(p_vec: Vec<Person>, city: String) -> Vec<String> {
    let mut c_vec: Vec<String> = vec![];
    return c_vec;
}

#[derive(Debug)]
struct Address {
    street: String,
    zip: u32,
    city: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    id: String,
    location: Address,
}

/*
fn read_file(filename: String) -> Vec<Person> {
    let mut vec: Vec<Person> = Vec::new();
    let data = fs::read_to_string(filename).expect("Can't open file");
    return vec;
}
fn read_file2(filename: String) -> Vec<u8> {
    let mut vec: Vec<Person> = Vec::new();
    let data = fs::read(filename).expect("Can't open file");
    return data;
}
*/
