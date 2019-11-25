use std::io;

fn main() {
    // Q1.
    println!("Q1. fahrenheit to celsius");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: f32 = input.trim().parse()
        .expect("Failed to convert into integer");

    println!("celsius of {} fahrenheit is {}", input, f2c(input));

    // Q2.
    println!("Q2. print nth fibonacci number");
    
    let mut nth = String::new();
    io::stdin().read_line(&mut nth)
        .expect("Failed to read line : nth");
    let nth: u32 = nth.trim().parse()
        .expect("Failed to convert nth");
    
    println!("{}th fibonacci number is {}", nth, fib(nth));
}

// Q1. lessons learned
// - when you want to read from io, use 'let mut' type
// - cannot multiply float to integer
fn f2c(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

// Q2. lessons learned
// - recursive function is allowed in Rust
fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        fib(n - 1) + fib (n - 2)
    }
}