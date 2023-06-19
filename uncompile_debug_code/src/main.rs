fn main() {
    #[cfg(debug_assertions)]
    println!("1. Debug mode - main");

    #[cfg(not(debug_assertions))]
    println!("1. Release mode - main");

    // using if else for debug mode
    if cfg!(debug_assertions) {
        println!("2. Debug mode - if else");
    } else {
        println!("2. Release mode - if else");
    }

    // this function compile differently in debug and release mode
    debug_function();

    // this only run/compile in debug mode
    #[cfg(debug_assertions)]
    debug_module::debug_function();

    // this only run/compile in release mode
    #[cfg(not(debug_assertions))]
    release_module::release_function();

    // debug marco
    let y = 5;
    debug_assert!(y != 0, "Division by zero"); // only run in debug mode

    #[cfg(debug_assertions)]
    {
        println!("5. Debug mode - main");
    }

    #[cfg(not(debug_assertions))]
    {
        println!("5. Release mode - main");
    }
}
// only define in debug mode
#[cfg(debug_assertions)]
fn debug_function() {
    println!("3. Debug mode - function");
}

// only run in release mode
#[cfg(not(debug_assertions))]
fn debug_function() {
    println!("3. Release mode - function");
}

// only define in debug mode
#[cfg(debug_assertions)]
mod debug_module {
    pub fn debug_function() {
        println!("4. Debug mode - module");
    }
}

// only define in release mode
#[cfg(not(debug_assertions))]
mod release_module {
    pub fn release_function() {
        println!("4. Release mode - module");
    }
}
