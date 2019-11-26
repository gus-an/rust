fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area() // method syntax
    );

    let sq = Rectangle::sqaure(3);
    // how to call associate function, constructor
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // &self : don't want ownership, don't write to it
    // &mut self : wnat to change the instance
    // self : rare. when method transforms 'self' into something else,
    //        prevent caller from using the original instance after transformation

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    // associated functions
    // functions without 'self' as parameter. 
    // 'String::from' is an associated function
    // are often used for 'constructors' that return a new instance of the struct
}
// impl : define function within context of 'struct'

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// multiple 'impl' blocks