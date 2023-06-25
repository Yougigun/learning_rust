#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type
)]
pub fn clean_sink() {
    println!("cleaning sink");
}

pub mod soap {
    pub fn soap_clean() {
        println!("soap cleaning");

        // this is private in parent module
        super::super::super::toilet::private_flush();
    }
}
