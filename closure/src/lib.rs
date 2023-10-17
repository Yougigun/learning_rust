#![allow(warnings)]
use std::process::Output;
use std::{collections::HashMap, mem::size_of_val};

fn add_one(x: i32) -> i32 {
    x + 1
}
// Suppose this is your closure
static ADD_ONE: fn(i32) -> i32 = add_one;

// The Rust compiler essentially create a anonymous type like this
struct Closure<'a> {
    // capture by value
    env_var: i32,
    // capture by move
    env_var1: String,
    // capture by reference
    env_var2: &'a String,
    // capture by mutable reference
    env_var3: &'a mut String,
}

// Compiler implement Fn, FnMut or FnOnce for Closure
impl<'a> Closure<'a> {
    // compiler implement it by checking it is Fn, FnMut or FnOnce
    fn call(&self, x: i32) -> i32 {
        add_one(x)
    }

    fn call_fumut(&mut self, x: i32) -> i32 {
        add_one(x)
    }

    fn call_fnonce(self, x: i32) -> i32 {
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
pub fn demo_fn_once_closure() {
    let name = String::from("gary");

    // move name to closure and because of name's move, compiler will implement FnOnce for closure
    let c = move |greeting: String| (greeting, name);
    // when you call a closure, the compiler will generate code that calls the call_once method on the closure.
    let res = c(String::from("Hello"));
    println!("res: {:?}", res);

    // can't call c again, because name has been moved leading to c being FnOnce
    // c(String::from("Hi"));
}

trait FakeFnOnce {
    type Output;
    fn call_once(self, args: String) -> Self::Output;
}

fn use_fake_fn_once<T>(f: impl FakeFnOnce<Output = T>, args: String) -> T {
    f.call_once(args)
}

pub fn demo_fn_closure() {
    let name = String::from("gary");

    // use clone to avoid move name out of closure leading to c being Fn
    let c = move |greeting: String| (greeting, name.clone());
    // can call multiple times
    println!("c: {:?}", c(String::from("Hello")));
    println!("c: {:?}", c(String::from("Hi")));

    let res = call_once("Hi".into(), c);
    println!("res: {:?}", res);

    // cannot call again, because c has been moved
    // println!("c: {:?}", c(String::from("Hi")));

    // Fn also can be used as FnOnce as long as the interface is satisfied
    println!("result: {:?}", call_once("Bonjour".into(), not_closure));
    // println!("name: {}", name);
}
fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}
fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}

// it use closure as &mut reference, so it need to be mut and can be called multiple times
// pub trait FnMut<Args>: FnOnce<Args> {
//     extern "rust-call" fn call_mut(
//         &mut self,
//         args: Args
//     ) -> Self::Output;
// }

pub fn demo_fn_mut_closure() {
    let mut name1 = String::from("gary");
    let mut name2 = String::from("rosie");

    // capture &mut name1, length is 8
    let mut c = || {
        name1.push_str(" you");
    };

    // capture name2, length is 24
    let mut c1 = move || {
        name2.push_str(" chen");
    };
    c();
    c1();
    call_mut(&mut c);
    call_mut(&mut c1);
    call_once2(c);
    call_once2(c1);
}

// even as parameter , it need to be mut because FnMut will take closure as mut reference
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// FnOnce noe need to be mut as parameter, because it will take ownership
fn call_once2(c: impl FnOnce()) {
    c();
}

// it use closure as & reference, so it need to be mut and can be called multiple times
// pub trait Fn<Args>: FnMut<Args> {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
// }

fn demo_fn_closure2() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];

    let mut a = 1234;
    // Fn，不移动所有权
    let mut c = |x: u64| v.len() as u64 * x;
    // Fn，移动所有权
    let mut c1 = move |x: u64| v1.len() as u64 * x;
    let mut c2 = |x: u64| {
        a += 1;
        a * x
    };

    println!("direct call: {}", c(2));
    println!("direct call: {}", c1(2));

    println!("call: {}", call(3, &c));
    println!("call: {}", call(3, &c1));

    println!("call_mut: {}", call_mut2(4, &mut c));
    println!("call_mut: {}", call_mut2(4, &mut c1));

    println!("call_once: {}", call_once3(5, c));
    println!("call_once: {}", call_once3(5, &mut c1));
    println!("call_once: {}", call_once3(5, &mut c2));
    // c2 can be called again, because when closure in call_once3 is called as FnMut not FnOnce.
    println!("call_once: {}", call_once3(5, c2));
}
fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut2(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once3(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}

use std::ops::Mul;
fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Copy,
{
    move |y| x * y
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
        demo_fn_once_closure()
    }
    #[test]
    fn test_demo_fn_closure2() {
        demo_fn_closure2()
    }
    #[test]
    fn test_curry() {
        let c = curry(2);
        let res = c(3);
        println!("res: {}", res);
    }
}
