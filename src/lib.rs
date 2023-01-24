pub mod front_of_house;

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        super::front_of_house::serving::serve_order();
    }
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
