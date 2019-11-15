fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);
    // error : borrow of moved value
    let s3 = s2.clone();
    println!("deep copy of s2 = {}", s3);

    //--------------------------------------------------------//
    
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); //s's value moves into function, no longer valid here

    let x = 5;

    makes_copy(x); // i32 is Copy, so okay to use x afterward
    
    let s1 = gives_ownership(); // moves return value into s1

    let s2 = String::from("hello"); // s2 is 'moved'

    let s3 = takes_and_gives_back(s2);
} // s1, s3 are dropped. Nothing happens s2, which was moved

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called. Memory is free.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}