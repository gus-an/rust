#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        // child modules are still 'private'

        fn seat_at_table() {}
    }
    // mark 'hosting' module with 'pub' keyword, so that parent module
    // may have access to child module

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}
// mod : define a module
// can define other modules inside a module
// 'eat_at_restaurant' is sibling, so this private module can be referred to

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
    // 'super' : specify relative path
    // chef fixes an incorrect order and personally bring it out to customer

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // customer can pick type of bread
    // chef decides which fruit acoompanies

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        // without this function, we couldn't set 'seasonal_fruit' field in 'eat_at_restaurant'
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

use crate::front_of_house::hosting;
// use keyword shortens paths

pub fn eat_at_restaurant() {
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    //shorter path
    hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // we can read public 'toast' field
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
}
// pub : part of library crate's public API
// absolute is more preferred because it more likely to move code
// definitions and item calls independently of each other

