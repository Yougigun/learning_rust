// ./bath.rs
pub mod bath;

// ./sink/mod.rs
pub mod sink;

pub fn flush() {
    println!("flushing");
}

fn private_flush() {
    println!("private flushing");
}
