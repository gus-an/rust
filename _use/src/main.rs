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