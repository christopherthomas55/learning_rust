mod front_of_house;
mod back_of_house;

// Use can be public too, but I'm still a bit unclear why
pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restautant() {
        super::hosting::add_to_waitlist();
    }
}

// structs and enums are often used without the full path
use std::collections::HashMap;

fn hash_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// As syntax...just like python!
use std::fmt::Result;
use std::io::Result as IoResult;


pub fn eat_at_restautant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //Doesn't compile cause private
    //meal.seasonal_fruit = String::from("blueberries");
    
    //Enum's on other hand are always public cause they're useless otherwise
    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;


}

fn deliver_order() {}
