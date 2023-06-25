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
fn main() {
    // Generic Types, Traits, and Lifetimes
    // generics are abstract stand-ins for concrete types or other properties
    // - how to define your own types, functions, and methods with generics
    // - make a generic function two functions that differ only in their types
    // - how to use generic types in struct and enum definitions
    // - how to use `trait` to define behavior in a generic way
    // lifetime: a variety of generics that give the compiler information about
    // how references relate to each other

    // Removing Duplication by Extracting a Function
    let mut number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let largest = find_largest(&number_list);
    println!("The largest number is {}", largest);

    // use generic type in function
    let largest = find_largest_generic_type(&number_list);
    println!("The largest number is {}", largest);

    let number_list_float = vec![11.1, 34., 6000., 89., 54., 2., 43., 8.];
    let largest = find_largest_generic_type(&number_list_float);
    println!("The largest number is {}", largest);

    // Generic Data Types
    // how to define functions, structs, enum, and methods using generics

    // In Function Definitions
    // fn largest number
    fn find_largest_generic_type<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_largest_generic_type(&char_list);
    println!("The largest char is {}", result);

    // in Struct Definitions
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point::<u8> { x: 5, y: 10 }; // explicit
    let integer2 = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!(
        "integer = {:?}, integer2 = {:?}, float = {:?}",
        integer, integer2, float
    );
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "both_integer = {:?}, both_float = {:?}, integer_and_float = {:?}",
        both_integer, both_float, integer_and_float
    );

    // In Enum Definitions
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }
    #[derive(Debug)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // In Method Definitions
    struct Point3<T> {
        x: T,
        t: T,
    }
    // use same T is conventional
    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &(self.x)
        }
    }
    // We could also use different name, because it just a generic parameter
    // impl<U> Point3<U> {
    //     fn x(&self) -> &U {
    //         &(self.x)
    //     }
    // }
    let p = Point3 { x: 5, t: 10 };
    println!("p.x = {}, p.t ={}", p.x(), p.t);
    // specify constrains on generic types when defining methods on the type
    impl Point3<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.t.powi(2)).sqrt()
        }
    }
    // Generic type parameter in struct definition aren't always the same as
    // those you use in that same struct's method signatures
    struct Point4<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Point4<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
            Point4 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Performance of Code Using Generics
    // Monomorphization at Compile Time
    let integer = Some(5);
    let float = Some(5.0);
    // The monomorphized version of the code looks similar to the following:
    enum OptionI32 {
        Some(i32),
        None,
    }

    enum OptionF64 {
        Some(f64),
        None,
    }
    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);

    // Traits: Defining Shared Behavior
    // - trait define functionality a particular type has and can share with other types
    // - trait bound to specify that a generic type can be any type that has certain behavior

    // Define a Trait
    // A Summary trait that consists of the behavior provided by a summarize method
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // implementing a Trait on a Type
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    impl NewsArticle {
        fn summarize(&self) -> String {
            format!("author: {}", self.author)
        }
    }
    let text = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best"),
    };
    println!("New article available! {}", text.summarize());
    println!("New article available! {}", Summary::summarize(&text));
    // Universal Function Call Syntax
    println!(
        "New article available! {}",
        <NewsArticle as Summary>::summarize(&text)
    );
    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // Default Implementations
    pub trait Summary2 {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    impl Summary2 for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", Summary::summarize(&tweet));
    println!("1 new tweet: {}", Summary2::summarize(&tweet));
    println!("1 new tweet: {}", <Tweet as Summary2>::summarize(&tweet));

    // Traits as Parameters
    // https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
    // how to use traits to define functions that accept many different types.

    // Define a public function named `notify` that takes a reference to an object that implements the `Summary` trait.
    pub fn notify(item: &impl Summary) {
        // Print a message to the console with the result of calling the `summarize` method on the `item` parameter.
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound Syntax
    // https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax
    // The impl Trait syntax works for straightforward cases but is actually
    // syntax sugar for a longer form known as a trait bound
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // fuller trait bound syntax can express more complexity in other cases
    pub fn notify4(item1: &impl Summary, item2: &impl Summary) {}
    pub fn notify5<T: Summary>(item1: &T, item2: &T) {}

    // Specifying Multiple Trait Bounds with the + Syntax
    // we specify in the notify definition that item must implement both Display and Summary
    use std::fmt::Display;
    pub fn notify3(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }
    pub fn notify6<T: Summary + Display>(item: &T) {}

    // Clearer trait Bounds with `where` Clauses
    use std::fmt::Debug;
    fn some_function<T, U>(t: &T, u: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }

    // Returning Types that Implement Traits
    fn return_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    // However, you can only use `impl Trait` if you're returning a single type.
    // Returning either a NewsArticle or a Tweet isn’t allowed due to
    // restrictions around how the impl Trait syntax is implemented in the compiler.

    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from(
    //                 "Penguins win the Stanley Cup Championship!",
    //             ),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                  hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from(
    //                 "of course, as you probably already know, people",
    //             ),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }

    // Using Trait Bounds to Conditionally Implement Methods
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y)
            }
        }
    }
    // we can also conditionally implement a trait for any type that implements another trait.
    // blanket implementations and are extensively used in the Rust standard library
    // Blanket implementations appear in the documentation for the trait in the
    // “Implementors” section.
    // impl<T: Display> ToString for T {
    //     // --snip--
    // }

    // Validating References with Lifetimes
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
    // Lifetimes are another kind of generic that we’ve already been using.
    //  lifetimes ensure that references are valid as long as we need them to be

    // Preventing Dangling References with Lifetimes
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#preventing-dangling-references-with-lifetimes

    // The Borrow Checker
    // The Rust compiler has a borrow checker that compares scopes to determine
    // whether all borrows are valid.
    // fn main() {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    // fn main() {
    //     let x = 5;            // ----------+-- 'b
    //                           //           |
    //     let r = &x;           // --+-- 'a  |
    //                           //   |       |
    //     println!("r: {}", r); //   |       |
    //                           // --+       |
    // }                         // ----------+

    // Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Lifetime Annotations Syntax
    // Lifetime annotations don’t change how long any of the references live
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // Lifetime Annotations in Function Signatures
    // the function takes two parameters, both of which are string slices that
    // live at least as long as lifetime 'a. The function signature also tells
    // Rust that the string slice returned from the function will live at least
    // as long as lifetime 'a. In practice, it means that the lifetime of the
    // reference returned by the longest function is the same as the smaller of
    // the lifetimes of the values referred to by the function arguments.
    fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Thinking in Terms of Lifetimes
    fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
        // the lifetime of y does not have any relationship with the lifetime
        // of x or the return value.
        x
    }

    // If the reference returned does not refer to one of the parameters,
    // it must refer to a value created within this function.
    // However, this would be a dangling reference because the value will go
    // out of scope at the end of the function. Consider this attempted implementation of the longest function that won’t compile:
    // the best fix would be to return an owned data type rather than a reference so the calling function is then
    // responsible for cleaning up the value.
    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    // Lifetime Annotations in Struct Definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    // The compiler uses three rules to figure out the lifetimes of the references
    // when there aren’t explicit annotations. The first rule applies to input
    // lifetimes, and the second and third rules apply to output lifetimes.
    // - The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
    // - The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    // - The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self
    //   because this is a method, the lifetime of self is assigned to all output lifetime parameters.
    fn first_word(s: &str) -> &str {
        // fn first_word<'a>(s: &'a str) -> &'a str {} is same.
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        s
    }

    // Lifetime Annotations in Method Definitions
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-method-definitions

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            // There are two input lifetimes, so Rust applies the first lifetime
            // elision rule and gives both &self and announcement their own
            // lifetimes. Then, because one of the parameters is &self, the return
            // type gets the lifetime of &self, and all lifetimes have been accounted for.
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime
    // You might see suggestions to use the 'static lifetime in error messages.
    // But before specifying 'static as the lifetime for a reference, think
    // about whether the reference you have actually lives the entire lifetime
    // of your program or not, and whether you want it to.
    let s: &'static str = "I have a static lifetime.";

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

// fn largest i32
fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest number
fn find_largest_number<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
