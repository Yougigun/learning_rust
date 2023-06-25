#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type,
    clippy::single_component_path_imports,
    clippy::match_single_binding,
    clippy::needless_borrow,
    clippy::missing_safety_doc,
    unused_unsafe
)]
fn main() {
    // Writing Automated Tests
    // https://doc.rust-lang.org/book/ch11-00-testing.html#writing-automated-tests
    // In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that
    // “Program testing can be a very effective way to show the presence of bugs,
    //  but it is hopelessly inadequate for showing their absence.

    // How to Write Tests
    // Tests are Rust functions that verify that the non-test code is functioning
    // in the expected manner
    // 1. Set up any needed data or state
    // 2. Run the code you want to test
    // 3. Assert the results are what you expect

    // The Anatomy of a Test Function
    // To change a function into a test function, add #[test] on the line before fn

    // Checking Results with the assert! Marco
    //https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-results-with-the-assert-macro
    // If the value is true, nothing happens and the test passes. If the value
    // is false, the assert! macro calls panic! to cause the test to fail.

    // Testing Equality with the assert_eq! and assert_ne! Marcos

    // Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=,
    // respectively. When the assertions fail, these macros print their arguments
    // using debug formatting, which means the values being compared must implement
    // the PartialEq and Debug traits. All primitive types and most of the standard
    // library types implement these traits. For structs and enums that you define
    // yourself, you’ll need to implement PartialEq to assert equality of those
    // types. You’ll also need to implement Debug to print the values when the
    // assertion fails. Because both traits are derivable traits, as mentioned
    // in Listing 5-12 in Chapter 5, this is usually as straightforward as adding
    // the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.

    // Adding Custom Failure Messages
    // https://doc.rust-lang.org/book/ch11-01-writing-tests.html#adding-custom-failure-messages

    // 11.2 Controlling How Tests Are Run
    // Just as `cargo run` compiles your code and then runs the resulting binary,
    // `cargo test` compiles your code in test mode and runs the resulting test binary

    // Running Tests in Parallel or Consecutively
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively
    // When you run multiple tests, by default they run in parallel using threads,
    // meaning they finish running faster and you get feedback quicker.
    // Because the tests are running at the same time, you must make sure your
    // tests don’t depend on each other or on any shared state, including a
    // shared environment, such as the current working directory or environment variables.

    // We set the number of test threads to 1, telling the program not to use any parallelism.
    // $ cargo test -- --test-threads=1

    // Showing Function Output
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output
    // When running a test, by default all output is captured.
    // Output is only shown for tests that fail.
    // You can run `cargo test -- --show-output`
    // to see output from passing tests, as well.

    // Running a Subset of Tests by Name
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-a-subset-of-tests-by-name
    // # Run all tests
    // $ cargo test

    // Running Single Test
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-single-tests
    // # Run the "function name" test only
    // $ cargo test "function_name"

    // # Run any test with "partial name" in the name
    // $ cargo test "partial_function_name"

    // Filtering to Run Multiple Tests
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#filtering-to-run-multiple-tests
    // We can specify part of a test name, and any test whose name matches that value will be run.

    // Ignoring Some Tests Unless Specifically Requested
    // https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
    {
        // #[test]
        // fn it_works() {
        //     assert_eq!(2 + 2, 4);
        // }

        // #[test]
        // #[ignore]
        // fn expensive_test() {
        //     // code that takes an hour to run
        // }
    }
    // cargo test // only run the test without `ignore` attribute
    // cargo test -- --ignored // only run the ignored tests
    // cargo test -- --include-ignored // run all

    // 11.3 Test Organization
    // https://doc.rust-lang.org/book/ch11-03-test-organization.html#test-organization
    // - Unite test are small and more focused, testing one module
    // in isolation at a time, and can test private interfaces
    // - Integration tests are entirely external to your library and use your
    // code in the same way any other external code would, using only the public
    // interface and potentially exercising multiple modules per test.

    // Unit tests
    // The Tests module and #[cfg(test)]
    // https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-module-and-cfgtest

    // Testing Private Functions

    // Integrations Tests
    // https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests
    // The tests Directory
    // https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-directory
    // adder
    // ├── Cargo.lock
    // ├── Cargo.toml
    // ├── src
    // │   └── lib.rs
    // └── tests
    //     └── integration_test.rs
    // cargo test 
    // cargo test --test integration_test // run specific integration test

    // Submodules in Integration Tests
    // https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests
    

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // cargo test -p writing_automated_tests // only run non ignored test but show ignored reason and ignored numbers
    // cargo test -p writing_automated_tests -- --ignored // only run ignored test
    // cargo test -p writing_automated_tests -- --include-ignored // run ignored test
    #[test]
    #[ignore = "no reason"]
    fn it_works_ignore() {
        let result = 2 + 3;
        assert_eq!(result, 5);
    }

    #[test]
    // Each test is run in a new thread, and when the main thread sees that a
    // test thread has died, the test is marked as failed.
    fn it_does_not_work() {
        panic!("Make this test fail")
    }

    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger))
    }

    fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn it_adds_two_1() {
        assert_eq!(add_two(4), 6)
    }
    #[test]
    fn it_adds_two_2() {
        assert_eq!(add_two(4), 7)
    }

    fn greeting(name: &str) -> String {
        // format!("Hello {}!", name)
        String::from("Hello")
    }

    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Creating for Panics with should_panic
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if !(1..=100).contains(&value) {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100_with_expected() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests
    #[test]
    fn it_works_using_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }
}
