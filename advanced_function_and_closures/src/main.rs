fn main() {
    // 19.4 Advanced Functions and Closures
    {
        // function pointer

        fn add_one(x: i32) -> i32 {
            x + 1
        }
        // fn is a type that represents the type of a function, function pointer
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }

    {
        // passing function in place of closures
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

        // This is equivalent to the above:
        let list_of_strings2: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();

        //Each enum variant we define becomes an initializer function
        enum Status {
            Value(u32),
            Stop,
        }

        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }

    {
        // Returning Closures
        // Fn, FnMut, or FnOnce are traits that closure implements
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
