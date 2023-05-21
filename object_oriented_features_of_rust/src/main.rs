fn main() {
    // Defining a Trait for Common Behavior
    {
        // At runtime, a trait object will be a pair of pointers in memory -
        // one to the instance of a specific type that implements our trait,
        // and another to a table of methods defined on the trait to call at runtime
        mod gui {
            pub trait Draw {
                fn draw(&self); // self is th type which implement this trait
            }

            pub struct Screen {
                // store a vector of trait objects that we can call the draw method on
                // why not a generic type parameter in vec? all element need to be the same type.
                // why not a enum in vec? we can't add new types to the enum because.
                // we don't know the lib user will add what enum variant.
                pub components: Vec<Box<dyn Draw>>,
            }

            impl Screen {
                pub fn run(&self) {
                    for component in self.components.iter() {
                        component.draw();
                    }
                }
            }

            pub struct Button {
                pub width: u32,
                pub height: u32,
                pub label: String,
            }

            impl Draw for Button {
                fn draw(&self) {
                    // code to actually draw a button
                    println!("Button: width: {}, height: {}, label: {}", self.width, self.height, self.label)
                }
            }
        };
        use gui::{Button, Draw, Screen};
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                // code to actually draw a select box
                println!("SelectBox: width: {}, height: {}, options: {:?}", self.width, self.height, self.options)
            }
        }
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
            ],
        };
        screen.run();
    }
    
    // Implementing an Object-Oriented Design Pattern
    {
        // post has three states: Draft, Review, and Published
    }


}
