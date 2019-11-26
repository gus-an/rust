fn main() {
    let mut user1 = User {
        username: String::from("ajw"),
        active: true,
        sign_in_count: 1,
    };

    user1.sign_in_count = 2;

    let user2 = User {
        username: String::from("gus"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        username: String::from("jw"),
        ..user1 // '..' specifies remaining fields have the same value as given instance
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // cannot assign origin to black, even if they have similar fields

    // --------------------- //
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // --------------------- //
    // println!("rect1 is {}", rect1);
    // {} : requires 'std::fmt::Display', which isn't provided by struct
    println!("rect1 is {:?}", rect1);
    // {:?} : uses output format called 'std:fmt:Debug'
    println!("rect1 is {:#?}", rect1);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User{
    username: String,
    // using 'String' type is deliberate, rather than '&str'
    // because we want struct to own all of its data, and valid as long as struct is valid
    // we can use '&str', but need to specify 'lifetimes'
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String) -> User {
    User {
        username, // when variables and fields have same name, use shorthand
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 'derive' annotation
// #[derive(Debug)] : explicitly add 'debug' functionality available for struct

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}