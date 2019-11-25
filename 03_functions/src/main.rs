fn main() {
    println!("Hello, world!");

    another_function(1);

    let y = {
        let x = 3;
        x + 1
    };
    // block that evaluates to 4
    // is an expression
    // no semicolon after 'x + 1' causes Error. The block doesn't return a value.
    println!("The value of y is: {}", y);

    let x = plus_one(5);
    println!("The vlaue of x is: {}", x);
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}
// it doesn't matter where to define a function

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}