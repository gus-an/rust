use std::collections::HashMap;
// idiomatic way of calling items(struct, enum) is to specify full path
use std::fmt;
use std::io;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn function1() -> fmt::Result {

// }

// fn function2() -> io::Result<()> {

// }
// how to bring two items with same name 'Result' into scop with 'use' statement

use std::fmt::Result;
use std::io::Result as IoResult;
// 'as' :  another solution to bringing two types of same name 

// ----------------------- //

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
// pub use : brought an item into scope. And make that item available for other to
// bring into their scope

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// ------------------------ //

use std::{cmp::Ordering, io};
// use std::io;
// use std::cmp::Ordering;

use std::io::{self, Write};
// use std::io;
// use std::io::Write;
// using nested paths reduce number of lines

use std::collections::*;
// * glob operator : bring all public items
// often used when testing to bring everthing under test into the 'tests' module