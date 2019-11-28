#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod front_of_house;
// 'front_of_house' module my be defined in another file.

pub use crate::front_of_house;
// if functions are called by 'front_of_house::hosting::add_to_waitlist',
// this code isn't necessary.
// 'use' is re-importing what 'mod' has.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}