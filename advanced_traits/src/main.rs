use std::fmt::Display;

fn main() {
    // 19.2 - Advanced Traits
    {
        // Specifying Placeholder Types in Trait Definitions with Associated Types
        // a trait with an associated type can only be implemented for a given type once, but a trait with a generic
        // type could be implemented for a given type multiple times for different generic types.
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>;
        }
        // An iterator that returns an unlimited supply of 7s:
        struct ForeverSevenIterator {}

        impl Iterator for ForeverSevenIterator {
            // Must fill in the concrete type here.
            type Item = i32;

            fn next(&mut self) -> Option<i32> {
                return Some(7);
            }
        }
        #[derive(Clone)]
        struct ForeverIterator<T: Clone> {
            pub val: T,
        }

        impl<T: Clone> Iterator for ForeverIterator<T> {
            type Item = T;

            fn next(&mut self) -> Option<T> {
                return Some(self.val.clone());
            }
        }

        {
            // Default Generic Type Parameters and Operator Overloading
            #[derive(Debug, PartialEq)]
            struct Point<T = i32> {
                x: T,
                y: T,
            }

            // Don't need to specify `Point<i32>` here.
            fn foo(p: Point) {
                println!("{}, {}", p.x, p.y)
            }

            // operator overloading
            use std::ops::Add;

            impl Add for Point {
                type Output = Point;

                fn add(self, other: Point) -> Point {
                    Point {
                        x: self.x + other.x,
                        y: self.y + other.y,
                    }
                }
            }
            assert_eq!(
                Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
                Point { x: 3, y: 3 }
            );
        }

        {
            // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
            trait Pilot {
                fn fly(&self);
            }

            trait Wizard {
                fn fly(&self);
            }

            struct Human;

            impl Pilot for Human {
                fn fly(&self) {
                    println!("This is your captain speaking.");
                }
            }

            impl Wizard for Human {
                fn fly(&self) {
                    println!("Up!");
                }
            }

            impl Human {
                fn fly(&self) {
                    println!("*waving arms furiously*");
                }
            }
            let person = Human;

            // If there's more than one `fly`, and you
            // don't specify the one you want, this
            // will call the one from the struct.
            // This prints "*waving arms furiously*".
            person.fly();

            // We can also call this as:
            Human::fly(&person);

            // We can explicitly call the `fly` method
            // from either trait:
            Pilot::fly(&person);
            Wizard::fly(&person);

            // but if fly in Pilot/Wizard is associated function,which means
            // we don't specify self as parameter to make compiler know which fly we want to call
            // Alternatively, we can use "turbofish" syntax:
            Human::fly(&person);
            <Human as Pilot>::fly(&person);
            <Human as Wizard>::fly(&person);
        }

        {
            // Using Supertraits to Require One Trait's Functionality Within Another Trait
            trait OutlinePrint: std::fmt::Display {
                fn outline_print(&self) {
                    let output = self.to_string(); // we can use to_string() because of Display
                    let len = output.len();
                    println!("{}", "*".repeat(len + 4));
                    println!("*{}*", " ".repeat(len + 2));
                    println!("* {} *", output);
                    println!("*{}*", " ".repeat(len + 2));
                    println!("{}", "*".repeat(len + 4));
                }
            }
            struct Point {
                x: i32,
                y: i32,
            }
            // need to impl Display to use OutlinePrint and to_string()
            impl Display for Point {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "({}, {})", self.x, self.y)
                }
            }
            // No need to implement the outline_print method as we get
            // the default definition, which automatically calls into
            // `fmt` above.
            impl OutlinePrint for Point {}
            let p = Point { x: 1, y: 2 };
            p.outline_print();
        }
        let v = Vec::<u8>::new(); // cause generic type is on Vec
        let v: Vec<u8> = Vec::new();

        {
            // Using the Newtype Patten to Implement External Traits on External Types
            // The orphan rule prevents you from implementing a external trait on a external type
            // as long as either the trait or the type are outside our crate.

            // It's possible to get around this using the newtype pattern
            // (borrowed from Haskell). The basic idea is to create a tuple
            // "wrapper" around the existing type.

            struct Wrapper(Vec<String>);
            impl Display for Wrapper {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "[{}]", self.0.join(", "))
                }
            }

            // impl defer on wrapper
            impl std::ops::Deref for Wrapper {
                type Target = Vec<String>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            // impl defermut on wrapper
            impl std::ops::DerefMut for Wrapper {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            let mut w = Wrapper(vec![String::from("hello"), String::from("world")]);
            println!("w = {}", w);
            (&mut w as &mut Vec<String>).push(String::from("foo"));
            println!("w = {}", w);
            w.push(String::from("bar")); // &mut w as &mut Vec<String> is automatically called
            println!("w = {}", w);

            Vec::<String>::push(&mut w, "baz".to_string());
        }
    }
}
