#![allow(warnings)]
use std::{collections::HashMap, mem::size_of_val};

fn add_one(x: i32) -> i32 {
    x + 1
}
// Suppose this is your closure
static ADD_ONE: fn(i32) -> i32 = add_one;

// The Rust compiler essentially turns it into something like this
struct ClosureEnvironment;
impl ClosureEnvironment {
    fn call(&self, x: i32) -> i32 {
        add_one(x)
    }
}

pub fn closure_size() {
    // length is 0
    let c1 = || println!("closure 1");
    //  regardless of the argument, length is 0
    let c2 = |arg: i32| println!("closure 2: {}", arg);
    let name = String::from("tyr");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("Hello", "world");
    // capture one ref, length is 8
    let c3 = || println!("closure 3:{ }", name);
    // capture one moved values, name1(length: 24) + table(length: 48) = 72, closure's length is 72
    let c4 = move || println!("closure 4: name: { }, table: {:?}", name1, table);
    let name2 = name.clone();
    // irrespective of the local variable, capture one moved String, length is 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("Gary");
        println!("closure 5: name:{ } x:{ } name3:{ }", name2, x, name3);
    };
    println!(
        "c1: { }, c2:{ }, c3:{},c4:{},c5:{}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
    )
}

// pub trait FnOnce<Args> {
//     type Output;
//     
//     When you call a closure, the compiler will generate code that calls the call_once method on the closure.
//     This method takes ownership of the closure leading to closure being only able to be called once.
//     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
// }
pub fn demo_fn_once() {
    let name = String::from("gary");
    
    
    // move name to closure and because of name's move, closure is FnOnce
    let c = move |greeting: String| (greeting, name);
    let res = c(String::from("Hello"));
    println!("res: {:?}", res);

    // can't call c again, because name has been moved leading to c being FnOnce
    // c(String::from("Hi"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        closure_size()
    }
    #[test]
    fn test_fn_once() {
        demo_fn_once()
    }
}
