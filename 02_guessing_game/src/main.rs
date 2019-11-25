use std::io; // standard::i/o library
             // 'io' is not in std::prelude, which is brought by default.
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    // gen_range() : method defined by 'Rng' trait.
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // new(): a static method of type 'String'
        // Variables are immuatable by default.
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // without 'use std::io', this would be std::io::stdin
        // which returns an instance of std::io::Stdin.
        // read_line() takes 'string' as arg.
        // & : reference. Access one data multiple times without copy into memory multiple times.
        // &mut guess (not &guess) : references are immutable by default.
        // .expect() : new line for multiple method calls
        // io::Result : type returned by 'read_line()'
        //              type is enumeration. Variants are 'Ok', 'Err' each contains info.
        // io::Result's 'expect()' : If has 'Err', 'expect' will crash the program, display message.
        //                           If unused, cause warning when compile.
        //                           If has 'Ok', return the value.
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // _ : catchall value. ignores all errors that 'parse()' might encounter.
        };
        // number types : i32(default), i64, u32, u64..
        // 'shadow' the previous value of 'guess' with a new one.
        // This feature is used when you convert value's type, let you reuse the variable.
        // trim() : remove whitespace in String value. INPUT VALUE is '5\n', including ENTER.
        // : u32 : annotate variable's type.
        // parse() : returns Result type.
    
        println!("You guessed: {}", guess);
        // {} : placeholder.
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        // std:cmp:Ordering : is enum with variants 'Less', 'Greater', 'Equal'.
        // match : decided what to do based on variant of 'Ordering' returned from 'cmp'.
        // arm : 'pattern' => 'code'. Runs code if pattern matches beggining of 'match' expression.
        // 'cmp' with 'guess' infers that 'secret_number' should be u32 as well.
    }
}
