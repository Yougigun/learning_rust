pub fn cook() {
    println!("cooking");
    clean_sink();
}
fn private_cook() {
    println!("cooking");
    clean_sink();
}

// sub module
pub mod kitchen {
    use crate::toilet::{sink::{clean_sink}, bath::bath_clean};

    pub fn fire() {
        println!("fire");
        clean_sink();
        bath_clean();
        // relative path
        super::super::toilet::sink::clean_sink();
    }
}

use crate::toilet::sink::clean_sink;

pub fn fire() {
    kitchen::fire();
}
