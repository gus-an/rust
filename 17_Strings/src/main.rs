fn main() {
    // CREATE
    let mut s = String::new();

    let data = "initial data";

    let s = data.to_string();
    // any data that implements 'Display' trait may use 'to_string' method to create a String
    // same as 'String::from'
    let s = "initial data".to_string();

    // UPDATE
    let mut s1 = String::from("foo");
    s1.push_str("bar");
    // 'push_str' takes a string slice, and not take the ownership of the parameter
    println!("{}", s);

    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    // we can still use 's2', meaning we have ownership
    s1.push('l');
    // 'push' method takes single character as parameter

    // -------------------- //

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2: {}, s3: {}", s2, s3);
    // s1 has been moved, can no long be used
    // use reference to concatenate (+)
    // + : uses 'add' method 'fn add(self, s: &str) -> String {}'
    // - we can only add '&str' to 'String', no two 'String' together
    // - compiler coerce the '&String' argument into a '&str'
    // - 'deref coercion' : '&s2' -> '&s2[..]'
    // this implementation is a lot effieicnt than copying

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    // s1 becomes invalid

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{},{},{},{}", s1,s2,s3,s);
    // all s's valid

    // SLICING
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // returns "Зд"
    // but [0..1] will cause crash, because it is inside '3'

    // ITERATE OVER STRINGS
    for c in hello.chars() { 
        println!("{}", c);
    }
    // 'chars()' method perform operations on individual Unicode scalar values
    for b in hello.bytes() {
        println!("{}", b);
    }
    // 'bytes()' method returns each raw type, ex) 224
    // valid Unocode scalar values may be made up of more than 1 byte
}
