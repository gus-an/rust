fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1 doesn't own s1, so don't drop s1.

    println!("The length of '{}' is {}.", s1, len);

    // ---------------------- //
    let mut s = String::from("hello");  // 1. mut s
    change(&s1, &mut s);                // 2. &mut s
    println!("mutated string: {}", s);

    // ---------------------- //
    let mut s2 = String::from("hello");
    let r1 = &mut s2;
    // let r2 = &mut s2;
    // println!("{}, {}", r1, r2);
    // error. only one mutable reference to particular piece of data in particular 'scope' allwed

    // --------------------- //
    let r1 = &s; // no problem
    let r2 = &s; // no problem (immutable borrow)
    println!("{}, {}", r1, r2);
    // let r3 = &mut s; // problem (cannot mutable borrow from a variable which has already immutably borrowed)
    // println!("{}, {}", r1, r3);
    let r3 = &mut s;
    println!("{}", r3); // okay because last usage of immutable references occurs before the mutable one is introduced.
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // using 'references' let you not to return back the ownership of variables.
// because we never had ownership.

fn change(some_string: &String, mutable_string: &mut String) { // 3. &mut String
    //some_string.push_str(", world"); 
    //error, cannot modify something we're borrowing

    mutable_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello"); // there's a string

//     &s  // returns references to string
// }  // s goes out of scope, and is dropped.

// Fix : return lifetime specifier, String itself