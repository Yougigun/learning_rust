use std::convert::Infallible;
use std::error::Error;
use std::fs::File;

fn main() {
    // Error Handling
    println!("Error Handling");
    // - Rust requires you to acknowledge that errors are a possibility
    // - Rust groups error into two major categories: `recoverable` and `unrecoverable` errors
    // - Rust does not have exceptions. Instead, it has the type Result<T, E> for recoverable errors
    // - Rust has a macro called `panic!` that stops execution when the program is in an unrecoverable state

    // 9.1 Unrecoverable Errors with panic!
    // https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic
    // There are two ways to cause a panic in practice:
    // - By calling the `panic!` macro explicitly
    // - By taking an action that causes our code to panic
    // By default, when a panic occurs, the program starts unwinding, which means
    // Rust walks back up the stack and cleans up the data from each function it encounters
    // - This walking back and cleanup is the default behavior when a thread panics
    // - If you want to abort instead of unwinding, you can call `std::process::abort`
    // - If you want to unwind the stack without cleaning up the data, you can call `std::mem::forget`

    // Unwinding the Stack or Aborting in Response to a Panic
    // https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic
    // By default, when a panic occurs, the program starts unwinding, which means
    // Rust walks back up the stack and cleans up the data from each function it encounters.
    // Aborting, which ends the program without cleaning up. Memory that the
    // program was using will then need to be cleaned up by the operating system.
    // If in your project you need to make the resulting binary as small as possible,
    // you can switch from unwinding to aborting upon a panic by adding panic = 'abort'
    // to the appropriate [profile] sections in your Cargo.toml file. For example,
    // if you want to abort on panic in release mode, add this:
    // [profile.release]
    // panic = 'abort'

    // panic!("crash and burn"); // this will cause a panic

    // Using a panic! `Backtrace`
    // https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#using-a-panic-backtrace

    let v = vec![1, 2, 3];
    // v[99]; // this will cause a panic // RUST_BACKTRACE=1 cargo run
    let val = v.get(99); // this will not cause a panic
    match val {
        Some(x) => println!("Value: {}", x),
        None => println!("No value"),
    }

    // 9.2 Recoverable Errors with Result
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result
    // The Result type is defined as having two generic type parameters, T and E
    // - T is the type of the value that will be returned in a success case within the Ok variant
    // - E is the type of the error that will be returned in a failure case within the Err variant
    // - The Result type has many helper methods defined on it to do various things
    // - The Result type, as with the Option type, is defined as an enum

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error), // no need to return File type
    // };

    // Matching on Different Errors
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors
    // match_on_different_error(); // uncomment to run

    // Alternative to Using match with Result<T,E>
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#alternative-to-using-match-with-resultt-e
    // alternative_to_using_match_with_result(); // uncomment to run

    // Shortcuts for Panic on Error: `unwrap` and `expect`
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect
    // We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
    // using_unwrap_on_result(); // uncomment to run
    // using_expect_on_result() // uncomment to run

    // Propagating Errors
    let username = read_username_from_file();
    // A Shortcut for Propagating Errors: the ? Operator
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    let username = read_username_from_file_question_mark();
    println!("Username: {:?}", username);
    // custom error type
    let username = read_username_from_file_question_mark_from();
    println!("Username: {:?}", username);
    // shorten the code further by chaining method calls after the ? operator
    let username = read_username_from_file_question_mark_chain();

    // Where The ? Operator Can Be Used
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used
    let last_char = last_char_of_first_line_short("hello world");
    println!("Last char: {}", last_char.unwrap());

    // To panic! or Not to panic!
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#to-panic-or-not-to-panic

    // Examples, Prototype Code, and Tests
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#examples-prototype-code-and-tests
    // - When you’re writing an example to illustrate some concept, also including
    // robust error-handling code can make the example less clear
    // - Similarly, the `unwrap` and `expect` methods are very handy when prototyping,
    // - If a method call fails in a test, you’d want the whole test to fail,
    // even if that method isn’t the functionality under test

    // Case in Which You Have More Information Than the Compiler
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#case-in-which-you-have-more-information-than-the-compiler
    // It would also be appropriate to call unwrap or expect when you have some
    // other logic that ensures the Result will have an Ok value, but the logic
    // isn’t something the compiler understands. And even better to document the
    // reason you think you’ll never have an Err variant in the expect text.
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("Home: {}", home);

    // Guidelines for Error Handling
    // It’s advisable to have your code panic when it’s possible that your code 
    // could end up in a bad state. In this context, a bad state is when some 
    // assumption, guarantee, contract, or invariant has been broken.
    //   - The bad state is something that is unexpected
    //   - Your code after this point needs to rely on not being in this bad 
    //     state, rather than checking for the problem at every step.
    //   - There’s not a good way to encode this information in the types you use.

    // Creating Custom Types for Validation
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#creating-custom-types-for-validation
    
    
}

// demo function for matching on different errors
fn match_on_different_error() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// demo function for alternative to using match with Result<T,E>
fn alternative_to_using_match_with_result() {
    use std::io::ErrorKind;
    // using unwrap_or_else to unwrap the ok enum or
    // to operate the closure if it is error enum
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// demo function to use unwrap on Result<T,E>
fn using_unwrap_on_result() {
    let greeting_file = File::open("hello.txt").unwrap();
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
    //     code: 2, kind: NotFound, message: "No such file or directory" }',
    //     src/main.rs:4:49
}

// demo function to use expect on Result<T,E>
fn using_expect_on_result() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// demo propagating error - long version
use std::io::{self, Read};
use std::net::IpAddr;
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//demo propagating error - using ? to make code concise
fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hello.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}

#[derive(Debug)]
enum CustomError {
    InvalidInput,
    InvalidOutput,
}

impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError::InvalidInput
    }
}

fn read_username_from_file_question_mark_from() -> Result<String, CustomError> {
    let mut username_file_result = File::open("hello.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_question_mark_chain() -> Result<String, CustomError> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(texrt: &str) -> Option<char> {
    let mut lines = texrt.lines();
    let first_line = lines.next()?;
    let chars_iterator = first_line.chars();
    let last_char = chars_iterator.last()?;
    Some(last_char)
}

fn last_char_of_first_line_short(texrt: &str) -> Option<char> {
    texrt.lines().next()?.chars().last()
}

// The following function is the entry point of the program.
// It returns a Result type with an empty tuple () and a Boxed Error trait object.
fn _main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
