use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut vec: Vec<Person> = Vec::new();
    vec = read_file(
        "C:\\Users\\Turbo\\IdeaProjects\\rust_learning\\Rust Learning\\c-labb1\\src\\names.txt"
            .parse()
            .unwrap(),
    );
    let mut vec2: Vec<u8> = Vec::new();
    vec2 = read_file2(
        "C:\\Users\\Turbo\\IdeaProjects\\rust_learning\\Rust Learning\\c-labb1\\src\\names.txt"
            .parse()
            .unwrap(),
    );

    for item in vec {
        println!("{:?}", item);
    }
    println!("{}", vec.len());
}

#[derive(Debug)]
struct Address {
    street: String,
    zip: u32,
    city: str,
}

#[derive(Debug)]
struct Person {
    name: String,
    id: String,
    location: Address,
}

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
