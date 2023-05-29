use std::ops::Deref;

fn main() {
    // 19.3 - Advanced Types
    {
        // Using the Newtype Pattern for Type Safety and Abstraction
        {
            // The newtype pattern involves creating a new type in a tuple struct
            struct Millimeters(u32);
            struct Meters(u32);

            let mm = Millimeters(5600);
            let m = Meters(4);

            println!("mm = {}", mm.0);
            println!("m = {}", m.0);
        }

        {
            // Creating Type Synonyms(type alias) with Type Aliases
            type Kilometers = i32;
            let x: i32 = 5;
            let y: Kilometers = 5;
            println!("x + y = {}", x + y);
            // The main use case for type aliases is to reduce the length of long type names.

            let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
            fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
                // --snip--
            }
            fn returns_long_type() -> Box<dyn Fn() -> i32 + Send + 'static> {
                // --snip--
                Box::new(|| 2)
            }
            // Using type aliases, we can write this as:
            type Thunk = Box<dyn Fn() + Send + 'static>;
            let f: Thunk = Box::new(|| println!("hi"));
            fn takes_long_type_using_alias_type(f: Thunk) {
                // --snip--
            }
            fn returns_long_type_using_alias_type() -> Thunk {
                // --snip--
                Box::new(|| ())
            }

            {
                //  Another example of this is in std::io
                type Result<T> = std::result::Result<T, std::io::Error>;
            }
        }

        {
            // The Never Type that Never Returns
            fn bar() -> ! {
                // --snip--
                todo!()
            }
            // Here this tells the compiler that the bar function will never return

            // the return value of continue is ! and ! can be coerced into any other type
            let guess = "3";
            loop {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                break;
            }

            // the panic! macro also return !, so we can use it in a match arm
            // that matches all cases
            enum MyOption<T> {
                Some(T),
                None,
            }
            impl<T> MyOption<T> {
                fn unwrap(self) -> T {
                    match self {
                        MyOption::Some(t) => t,
                        MyOption::None => panic!("called `MyOption::unwrap()` on a `None` value"),
                    }
                }
            }
        }

        {
            // Dynamically Sized Types and the Sized Trait
            // When we create a variable on the stack, Rust needs to know how
            // much space to allocate for that variable at compile time.

            fn my_generic_fn<T: ?Sized>(t: &T) {}

            let trait_obj: Box<dyn std::fmt::Display> = Box::new("Hello");
            my_generic_fn(&*trait_obj);
            my_generic_fn(&trait_obj);
            my_generic_fn(trait_obj.deref());
            println!("{}", trait_obj);

            fn my_generic_fn2<T: ?Sized>(t: &T) {}
            let s1: &str = "Hello";
            my_generic_fn2(s1);

            fn my_generic_fn3<T: ?Sized>(t: &T) {}
            let arr: &[i32] = &[1, 2, 3];
            my_generic_fn3(arr);
        }
    }
}
