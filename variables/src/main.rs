fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; 
    // compiler error
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of const MAX_POINTS is: {}", MAX_POINTS);

    let mut spaces = "    ";
    // spaces = spaces.len();
    // compiler error : mismatched types.
    // mut variables cannot be assigned with different type.

    let y: f32 = 3.0;
    println!("The value of y is: {}", y);

    let f: bool = false;

    let c = "Z";
    // char : ''
    // String : ""

    // let z = U+10FFF;
    // not allowed 

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    // destructuring
    println!("The value of y is: {}", y);
    let y = tup.0;
    // pattern matching
    println!("The vlaue of y is: {}", y);

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; 
    // [3, 3, 3, 3, 3]
    let first = a[0];

    let index = 10;
    let element1 = a[index];
    // CASE1 : runtime error. build success
    let element2 = a[10];
    // CASE2 : compiler error.

}
