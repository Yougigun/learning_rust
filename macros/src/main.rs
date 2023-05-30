fn main() {
    // 19.5 Marcos
    {
        // Declarative Macros with macro_rules! for General Meta programming
        macro_rules! four {
            () => {
                1 + 3
            };
        }

        assert_eq!(4, four!());

        #[macro_export]
        macro_rules! vec2 {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

        let v = vec2!(1, 2, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    {
        // Procedural Macros for Generating Code from Attributes
        // - custom derive
        // - attribute-like macros
        // - function-like macros
        {
            // How to Write a Custom derive Macro
            use hello_macro::HelloMacro;
            use hello_macro_derive::HelloMacro;

            #[derive(HelloMacro)]
            struct Pancakes;

            Pancakes::hello_macro();
        }
        {
            // Attribute-like marcos

            use hello_macro_derive::route;
            #[route(GET, "/")]
            fn index() {}
        }

        {
            // function-like marcos
            // The name comes from the fact that we can call them like a
            // function, similar to macro_rules! macros
            use hello_macro_derive::sql;

            let sql = sql!("SELECT * FROM posts WHERE id=1");
            println!("{}",sql)
        }
    }
}
