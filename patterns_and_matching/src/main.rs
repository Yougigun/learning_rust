use std::vec;

fn main() {
    // 18.1 - All the Places Patterns Can Be Used
    {
        // match arm
        let x = Option::Some(5);
        let x = match x {
            Option::Some(x) => Some(x + 1),
            Option::None => None,
        };
        println!("{:?}", x);
    }
    {
        // Conditional `if let` expressions
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    {
        // `while let` Conditional Loops
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        println!("stack: {:?}, stack_address:{:p}", stack, &stack);

        while let Some(top) = stack.pop() {
            println!("number1: {}, address1: {:p}", top, &top);
        }
        println!("stack: {:?}, stack_address:{:p}", stack, &stack);
    }

    {
        // `for loops`
        let v = vec!["a", "b", "c"];

        for (index, value) in v.iter().enumerate() {
            println!("index: {}, value: {}", index, value);
        }
        println!("v: {:?}", v);
    }

    {
        // `let` statements
        let (x, y, z) = (1, 2, 3);
        println!("x: {}, y: {}, z: {}", x, y, z);
    }

    {
        // Function and Closure Parameters
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({},{})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
    }
    {
        // The `matches!` marco
        let foo = 'f';
        assert!(matches!(foo, 'a'..='j' | 'A'..='J'));

        let bar = Some(4);
        assert!(matches!(bar, Some(x) if x % 2 == 0));
    }

    // 18.2 - Refutability: Whether a Pattern Might Fail to Match

    {}

    // 18.3 Pattern Syntax
    {
        // matching Literals
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        // Matching Named Variables
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            y => println!("Matched, y = {:?}", y),
        }

        println!("at the end: x = {:?}, y = {y}", x)
    }

    {
        // Multiple patterns

        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3..=5 => println!("three, four, or five"),
            _ => println!("anything"),
        }
    }

    {
        // Destructuring to Break Apart Values
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        // Can rename the values when we destructure:
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // Or we can use the same names:
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
        {
            // Destructuring Enums
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }

            let msg = Message::ChangeColor(0, 160, 255);

            match msg {
                Message::Quit => {
                    println!("The Quit variant has no data to destructure.")
                }
                Message::Move { x, y } => {
                    println!("Move in the x direction {} and in the y direction {}", x, y);
                }
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("Change the color to red {}, green {}, and blue {}", r, g, b)
                }
            }
        }

        {
            // Destructuring Nested Structs and Enums
            enum Color {
                Rgb(i32, i32, i32),
                Hsv(i32, i32, i32),
            }

            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(Color),
            }

            let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

            match msg {
                Message::ChangeColor(Color::Rgb(r, g, b)) => {
                    println!("Change the color to red {}, green {}, and blue {}", r, g, b)
                }
                Message::ChangeColor(Color::Hsv(h, s, v)) => {
                    println!(
                        "Change the color to hue {}, saturation {}, and value {}",
                        h, s, v
                    )
                }
                _ => (),
            }
        }
    }

    {
        // Ignoring Values in a Pattern
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);
    }

    {
        // Ignoring an Unused Variable by Starting Its Name with _
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }

        // Ignoring Remaining Parts of a Value with ..

        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }

        // Extra Conditionals with Match Guards
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {}", x, y);

        let y = false;
        let x = Some(4);
        match x {
            Some(x) if y => println!("{x}"),  // this will not be executed
            Some(x) if !y => println!("{x}"), // this will not be executed
            Some(x) => println!("{x}"),       // for catch all
            None => panic!("Silly compiler"), // for catch all
        }
        // @ Bindings
        // Sometimes we want to test a value as part of a pattern, and also
        // assign that value to a variable. We can do this with the at operator
        enum Message {
            Hello { id: i32, _text: &'static str },
        }

        let msg = Message::Hello {
            id: 5,
            _text: "hello",
        };
        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
                _text: _, // cannot text filed in the scope
            } => println!("Found an id in range: {}", id_variable),
            // Message::Hello { id: 10..=12 } => {
            //     println!("Found an id({}) in another range",id)
            // } // cannot compile
            Message::Hello { id, _text: _ } => println!("Found some other id: {}", id),
        }
    }
}
