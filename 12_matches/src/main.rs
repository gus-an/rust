fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // --------------------- //
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    }
    // _ placeholder : takes care of rest of the cases
    // 

    // --------------------- //
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    }
    // 'if let' : handel values that match one pattern, ignoring the rest
    // syntax sugar for 'match'

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1, // count non-quarter coins
    }
    // same as
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1;
    }
    // 'if let else' statement
    // match one, and rest
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
