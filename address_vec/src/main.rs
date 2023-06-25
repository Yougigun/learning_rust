fn main() {
    // create a vec of i32 with 5 capacity
    println!("create a vec of i32 with 5 capacity");
    let mut v = Vec::<i32>::with_capacity(5);
    // print reference of the vec
    println!("ref: {:p}", &v);
    // push 3 elements into the vec
    println!("push 1, 2, 3");
    v.push(1);
    v.push(2);
    v.push(3);
    // print the ptr in vec
    println!("ptr: {:p}", v.as_ptr());
    // print the len of vec
    println!("len: {}", v.len());
    // print the cap of vec
    println!("cap: {}", v.capacity());
    // push 2 elements into the vec
    println!("push 4, 5");
    v.push(4);
    v.push(5);
    // print the ptr in vec
    println!("ptr: {:p}", v.as_ptr());
    // print the len of vec
    println!("len: {}", v.len());
    // print the cap of vec
    println!("cap: {}", v.capacity());
    // push 1 element into the vec
    println!("push 6");
    v.push(6);
    // print the ptr in vec
    println!("ptr: {:p}", v.as_ptr());
    // print the len of vec
    println!("len: {}", v.len());
    // print the cap of vec
    println!("cap: {}", v.capacity());

    // print the reference of the vec
    println!("ref: {:p}", &v);

    // get the value of ptr
    let ptr = v.as_ptr();
    unsafe {
        // get the value of ptr
        println!("\nptr: {:p}", ptr);
        println!("value: {}", *ptr);
        // get the value of ptr
        println!("ptr: {:p}", ptr.offset(1));
        println!("value: {}", *ptr.offset(1));
        // get the value of ptr
        println!("ptr: {:p}", ptr.offset(2));
        println!("value: {}", *ptr.offset(2));
        // get the value of ptr
        println!("ptr: {:p}", ptr.offset(3));
        println!("value: {}", *ptr.offset(3));
        // get the value of ptr
        println!("ptr: {:p}", ptr.offset(4));
        println!("value: {}", *ptr.offset(4));
        // get the value of ptr
        println!("ptr: {:p}", ptr.offset(5));
        println!("value: {}", *ptr.offset(5));
    }
}
