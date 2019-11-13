fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number < 10 {
        println!("condition was else if");
    } else {
        println!("condition was false");
    }
    // condition must be 'bool'

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    // because 'if' is an expression, can use it on right side of 'let' statement
    // blocks of code evaluate to the last expression
    // 'if' and 'else' arms must have same type. "six" instead of '6' will cause error.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // you can add value you want to return after the 'break' expression.
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    // (1..4) : Range type provided by the std
    // rev() : reverse the range
    println!("LIFTOFF!!!");
}
