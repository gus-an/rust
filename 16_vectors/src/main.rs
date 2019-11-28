fn main() {
    let v1: Vec<i32> = Vec::new();
    // vectors are implemented using generics

    let v2 = vec![1, 2, 3];
    // more common code using macro, type inferred

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);

    //access data
    let third: &i32 = &v2[2];
    println!("third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // 'get' method gives 'Option<&T>'

    let does_not_exist1 = v2.get(100);
    // no problem. 
    // use logic to handle 'None'
    // let does_not_exist2 = &v2[100];
    // use this if you want your program to catch 'index out of bounds'

    // -------------------- //
    // v2.push(7);
    // error : cannot borrow as mutable !! it is borrowed immutable beforehand
    // adding new element require allocating new memory and copying the old elements
    // to the new space, if there isn't enough room.
    // -> reference to the first element would be pointing to deallocated memory.

    // ------------------- //
    for i in &v2 {
        println!("{}", i);
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    // add 50 to each elements
    // * dereference operator : change value that mutable reference refers to
    println!("{:?}", v4);

    // ------------------- //
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // use 'enum' to create vector with different types
    
    println!("{:?}", row);
    
}
