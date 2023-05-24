fn main() {
    // Unsafe code in Rust is code where we're allowed to ignore or bypass some
    // of the restrictions Rust places on us, and tell the compiler "Don't worry, I got this.

    // "unsafe" doesn't necessarily mean incorrect, it's just code that hasn't
    // been inspected by the eagle eye of the Rust compiler.

    // Unsafe Superpowers
    {
        // Dereference a raw pointer
        {
            let mut num = 5;

            let r1 = &num as *const i32;
            let r2 = &mut num as *mut i32;

            // We can create raw pointers in safe code, but we can't dereference
            // raw pointers and read the data being pointed to. For that, we need
            // to wrap the unsafe code in an unsafe block and run the code within
            // the block.
            unsafe {
                println!("r1 is: {}, {:?}", *r1, r1);
                println!("r2 is: {}, {:?}", *r2, r2);
            }
            // Create a pointer to a specific address.
            // (Hopefully this is memory we own!)
            // Note the `as` keyword to cast the value
            // into a raw pointer.
            let address = 0x012345usize;
            let r = address as *const i32;
            unsafe {
                // println!("r is: {}", *r); // this will cause a segmentation fault
            }
        }

        // Call an unsafe function or method
        {
            unsafe fn dangerous() {}

            unsafe {
                dangerous();
            }
        }

        {
            // Calling a Safe Abstraction over Unsafe Code
            let mut v = vec![1, 2, 3, 4, 5, 6];
            let r = &mut v[..];
            // let pv = v.as_ptr();

            let (a, b) = r.split_at_mut(3); // split_at_mut is a safe abstraction over unsafe code
            assert_eq!(a, &mut [1, 2, 3]);
            assert_eq!(b, &mut [4, 5, 6]);

            // alternative to split_at_mut
            unsafe {
                let a = &mut r[0..3] as *mut [i32];
                let b = &mut r[3..] as *mut [i32];
                assert_eq!(*a, [1, 2, 3]);
                assert_eq!(*b, [4, 5, 6]);
                assert_eq!(&*b, &[4, 5, 6]); // turn raw pointer into a reference to slice
            }
        }

        {
            // Using `external` Functions to Call External Code
            // Programming languages can call into code written in other languages
            // via a Foreign Function Interface (FFI). If you wanted to use OpenSSL
            // from Rust, for example, rather than rewriting OpenSSL in Rust you
            // could just call into the existing C code.
            extern "C" {
                fn abs(input: i32) -> i32;
            }
            unsafe {
                println!("Absolute value of -3 according to C: {}", abs(-3));
            }
        }

        // Access or modify a `mutable` static variable
        {
            static mut COUNTER: u32 = 0; // global static `mutable` variable

            fn add_to_count(inc: u32) {
                // modify static variable
                unsafe {
                    COUNTER += inc;
                }
            }

            add_to_count(3);

            //access static variable
            unsafe {
                println!("COUNTER: {}", COUNTER);
            }
        }

        // Implement an unsafe trait
        {
            // This is used for traits that have methods which cannot be checked
            // by Rust's usual safety guarantees, and it's a promise from the
            // programmer to the compiler that they've upheld these guarantees manually.

            unsafe trait Foo {
                // methods go here
            }

            unsafe impl Foo for i32 {
                // method implementations go here
            }
        }

        {
            // Accessing Fields of a Union
            union MyUnion {
                f1: i32,
                f2: u64,
            }
            let mut u = MyUnion { f1: 1 };

            unsafe {
                println!("{}", u.f1); // This is safe because we know we wrote `f1`.
            }
            unsafe {
                u.f1 = -2;
                println!("{}", u.f1);
                println!("{}", u.f2);
            }
        }

        {
            // Soundness
            fn index(idx: usize, arr: &[u8]) -> Option<u8> {
                if idx < arr.len() {
                    unsafe { Some(*arr.get_unchecked(idx)) }
                } else {
                    None
                }
            }
            // This uses unsafe code, but it checks the bounds of the array before
            // calling into get_unchecked, so we can prove this function is sound
            // (it can't cause undefined behavior).
        }
    }
}
