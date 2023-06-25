#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type,
    clippy::module_inception
)]
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
    use crate::toilet::{bath::bath_clean, sink::clean_sink};

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

mod module_parent {
    pub mod module2 {
        pub fn function() {
            println!("called `module1::module2::function()`");
        }
    }
    // privare function
    fn private_function() {
        println!("called `module1::private_function()`");
    }
    pub mod module3 {
        pub fn function() {
            super::module2::function();
            super::private_function();
            println!("called `module1::module3::function()`");
        }
    }
}
