use std::io; // standard::i/o library
             // 'io' is not in std::prelude, which is brought by default.

fn main() {
    println!("Guss the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // new(): a static method of type 'String'
    // Variables are 'immuatable' by default.

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // without 'use std::io', this would be std::io::stdin
    // which returns an instance of std::io::Stdin.

    println!("You guessed: {}", guess);
}
