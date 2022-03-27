// pub use crate::back_of_house::Breakfast;

// pub fn eat_at_restaurant() {
//     let mut meal = Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please",meal.toast);

//     // meal.fruit = String::from("orange");
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }