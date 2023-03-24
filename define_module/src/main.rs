mod garden {
    pub mod vegetable {
        pub fn plant() {
            // --snip--
            println!("planting");
        }
    }
    pub fn plant() {
        // --snip--
        println!("planting");
    }
}
// src/kitchen.rs
mod kitchen;

// src/toilet/mod.rs
mod toilet;

fn main() {
    garden::plant();
    garden::vegetable::plant();
    kitchen::cook();
    kitchen::kitchen::fire();
    toilet::flush();
    toilet::bath::bath_clean();
    toilet::sink::clean_sink();

    use garden::vegetable::plant as plant_vegetable;
    plant_vegetable();

    use crate::toilet::sink::clean_sink;
    clean_sink();
}
