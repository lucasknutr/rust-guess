use std::io;

fn main() {
    let mut name = String::new();

    println!("What's your name?");

    io::stdin()
        .read_line(&mut name)
        .expect("Oh no, you don't have a name. ERROR!");

    println!("Guess a number between 1 and 10");

    println!("Please input your guess.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You guessed: {input}. Thank you, {name}");
}
