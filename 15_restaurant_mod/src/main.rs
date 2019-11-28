use restaurant_rev;
// using 'crate::' refers to lib.rs..
// use instead the 'name' of 'Cargo.toml' to refer to modules in library.

fn main() {
    restaurant_rev::eat_at_restaurant();
}

// in order to not use 'src/lib.rs', create a 'src/utils.rs' file.
// and perhaps a 'utils/' folder for many modules.
// call by 'use utils::foo::bar' in 'main.rs'