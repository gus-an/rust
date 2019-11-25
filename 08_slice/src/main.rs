fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 5

    s.clear(); 
    // takes mutable reference to 's'
    // empties the String, making it ""
    // now, word has lost its sync with 's'

    // ------------------- //
    let p = String::from("hello world");

    let hello = &p[0..5]; // [inclusive..exclusive]
    let world = &p[6..11];

    let slice1 = &p[..2];
    let slice2 = &p[3..];
    let entire_string = &p[..];

    // ------------------- //
    
    let mut q = String::from("hello world");
    
    let word = first_word_slice(&q);
    
    q.clear();
    
    //println!("the first word is: {}", word); 
    // compile error : clear makes mutable reference to q.
    // cannot reuse the immutable reference again.
    
    // ------------------- //
    
    let my_string = String::from("hello world");
    let word = first_word_best(&my_string[..]);
    
    let my_string_literal = "hello world";
    let word = first_word_best(&my_string_literal[..]);
    let word = first_word_best(my_string_literal); // already a slice

    // ------------------- //

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // this slice has type '&[i32]'
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert to array of bytes

    /*
    println!("{:#?}", bytes); // print bytes
    println!("{:?}", bytes);  // pretty-print bytes
    */

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { // b' ' : byte literal syntax
            return i;
        }
    }
    // iter() : return each element in a collection
    // enumerate() : wraps result of 'iter' and return each element as 'tuple'

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_best(s: &str) -> &str {
    let a = "abc";
    return &a[0..1];
}
// best practice is to use '&str -> &str'
// because now we can use the function with both 'String', and 'string literal' 
