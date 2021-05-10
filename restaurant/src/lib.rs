fn serve_order() {}

mod front_of_house;

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(_toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from("toast"),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::hosting_child_module::foo();
    front_of_house::hosting::add_to_waitlist();
    back_of_house::fix_incorrect_order();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please", meal.toast);
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
