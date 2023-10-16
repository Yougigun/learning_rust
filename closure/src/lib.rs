#![allow(warnings)]
use std::{collections::HashMap, mem::size_of_val};

pub fn closure_task_1() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        closure_task_1()
    }
}
