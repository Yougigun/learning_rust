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
)]
use std::{
    cell::Cell,
    ops::Deref,
    os::unix::thread,
    sync::{Arc, Mutex},
    thread::spawn,
};

fn main() {
    //  Box

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }
    let n1 = Node {
        value: 1,
        next: None,
    };
    let n2 = Node {
        value: 2,
        next: Some(Box::new(n1)),
    };
    // iterator over n2 until no data and print the value of each node
    let mut current = Some(Box::new(n2));
    while let Some(node) = current {
        println!("{}", node.value);
        current = node.next;
    }

    enum Node2 {
        Cons(i32, Box<Node2>),
        Nil,
    }

    // create three nodes with Node2 enum
    let n3 = Node2::Cons(1, Box::new(Node2::Nil));
    let n4 = Node2::Cons(2, Box::new(n3));
    let n5 = Node2::Cons(3, Box::new(n4));

    // iterate over n5 until Nil and print the value of each node
    let mut current = &n5;
    while let Node2::Cons(value, next) = current {
        println!("{}", value);
        current = next;
    }

    // create a custom smart pinter called MyBox
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // dereference the value inside MyBox
    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    // dereference the value inside MyBox and change it
    impl<T> std::ops::DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }
    // dereference the value inside MyBox and print it
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y);

    // example of how to use my smart pointer with deref_mut
    let mut z = MyBox::new(String::from("Rust"));
    std::ops::DerefMut::deref_mut(&mut z).push_str(" is awesome!");
    assert_eq!("Rust is awesome!", *z);

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    let x = String::from("Rust");
    hello(&x);

    // Rc
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    match &(*a) {
        Cons(value, next) => {
            println!("value = {}", value);
            match next as &List {
                Cons(_, _) => {}
                Nil => println!("Nil"),
            }
        }
        Nil => println!("Nil"),
    };

    println!("count after creating c = {}", Rc::strong_count(&a));
    use core::cell::RefCell;
    let a = Rc::new(RefCell::new(32));
    let b = Rc::clone(&a);
    // *a = 35;
    {
        let mut k = b.borrow_mut();
        *k = 35;
        println!("{}", k);
    }
    println!("{:?}", *a.borrow());

    {
        let mut a = 1;
        let bb = &mut a;
        *bb = 5;
        let cc = &bb;
        // *cc = &mut 5;
        println!("bb = {bb},cc = {cc}");
    }
    // T.method() -> (&T).method(), (&(T.deference())).method()
    #[derive(Debug, Clone)]
    struct People {
        name: String,
    }
    impl People {
        fn test(self) {
            println!("test");
        }
        fn test2(&self) {
            println!("test2");
        }
    }
    People {
        name: String::from("test"),
    }
    .test();
    People {
        name: String::from("test"),
    }
    .test2();
    (&People {
        name: String::from("test"),
    })
        .test2();
    // (&People{name:String::from("test")}).test1(); // cannot work

    let box_int = Box::new(5);
    let n = *box_int;

    let box_people = Box::new(People {
        name: String::from("test"),
    });
    let p = *box_people; // move the ownership of box_people to p, so box_people cannot be used anymore

    println!("n = {}, p = {}", n, p.name);

    let box_people = Box::new(People {
        name: String::from("test"),
    });
    let p = *box_people;

    let box_people = Box::new(People {
        name: String::from("test"),
    });
    let p = &*box_people;
    let p2 = &box_people as &People;
    let p2 = box_people.deref();
    println!("box_people = {:?}", box_people);

    let box_people2 = Box::new(People {
        name: String::from("test"),
    });
    let p = &*box_people2; // or box_people.deref()

    let rc_people = Rc::new(People {
        name: String::from("test"),
    });
    let p = (*rc_people).clone(); // cannot  move the ownership of rc_people to p

    // 15.5 - RefCell<T> and the interior Mutability Pattern

    // using marco to derive common traits on StringContainer
    #[derive(Debug, Default, Clone)]
    struct StringContainer {
        to_lowercase_called: RefCell<usize>,
        to_lowercase_called_2: Cell<usize>,
    }
    impl StringContainer {
        fn new() -> StringContainer {
            StringContainer::default()
        }
        fn to_lowercase(&self, input: &str) -> String {
            *self.to_lowercase_called.borrow_mut() += 1;
            let old_count = self.to_lowercase_called_2.get();
            self.to_lowercase_called_2.set(old_count + 1);
            input.to_lowercase()
        }
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let iter1 = v.iter();
    let iter2 = v.iter();

    let v1 = iter1.collect::<Vec<_>>();
    let v2: Vec<_> = iter2.collect();

    // let v3 = &mut v; // cannot work. because of v3 and v1, one is mutable and another is immutable.
    println!("{:?}", v1); // prints: [1, 2, 3, 4, 5]

    struct Foo {
        x: i32,
    }
    let foo_ = Foo { x: 42 };
    let rc_foo = Rc::new(foo_);

    // Get a reference to the data using `as_ref`
    let foo_ref = Rc::as_ref(&rc_foo);
    println!("{}", foo_ref.x);

    // Or simply dereference the smart pointer
    println!("{}", rc_foo.x);

    let box_vec = Box::new(vec![1, 2, 3, 4, 5]);
    let box_vec2 = *box_vec;

    struct MyBox2<T>(T);
    impl<T> MyBox2<T> {
        fn new(x: T) -> MyBox2<T> {
            MyBox2(x)
        }
    }
    impl<T> Deref for MyBox2<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    let my_box2_vec = MyBox2::new(vec![1, 2, 3, 4, 5]);
    // let mybox2_vec2 = *mybox2_vec; // cannot compile but box allows this by compiler
    let my_box2_vec2: &Vec<i32> = &my_box2_vec; // call deref() on MyBox2<Vec<i32>> then * and &
    let my_box2_vec2: &Vec<i32> = my_box2_vec.as_ref(); // call as_ref() on Vec<i32>
    let my_box2_vec2 = &my_box2_vec as &Vec<i32>; // using coercion
    let my_box2_vec2 = my_box2_vec.deref(); // call deref() on MyBox2<Vec<i32>>

    let rc_vec = Rc::new(vec![1, 2, 3]);
    // let rv = *rc_vec; // cannot compile
    let rc_vec = Rc::new(RefCell::new(vec![1, 2, 3]));
    {
        let mut rv = rc_vec.borrow_mut();
        *rv = vec![4, 5, 6];
        println!("rv = {:?}", rv);
    }
    let rrv = rc_vec.borrow();
    println!("rrv = {:?}", rrv);

    let mut a = 5;
    let b = &mut a;
    println!("b = {:?}", b);
    *b = 10;
    let c = &a;
    println!("a = {:?}", a);

    // example of using Arc + mutex
    let arc_mutex_vec = Arc::new(Mutex::new(vec![1, 2, 3]));
    // show the address of arc_mutex_vec
    println!("arc_mutex_vec = {:p}", arc_mutex_vec);
    {
        let arc_mutex_vec = Arc::clone(&arc_mutex_vec);
        // show the address of arc_mutex_vec
        let mut arc_mutext_vec = arc_mutex_vec.lock();
        match arc_mutext_vec {
            Ok(mut v) => {
                println!("arc_mutext_vec = {:?}", v);
                *v = vec![4, 5, 6];
                println!("arc_mutext_vec = {:?}", v);
            }
            Err(e) => {
                let x = e.into_inner();
                // println!("error = {:?}", e.into_inner())
            }
        }

        // *arc_mutext_vec = vec![4, 5, 6];
        // let i = arc_mutext_vec.iter();
        // println!("arc_mutex_vec = {:?}", arc_mutext_vec);
    }

    //  Reference Cycles Can Leak Memory
    println!("\nScenario 1 - cycle reference");
    {
        enum List {
            Node(i32, Rc<RefCell<List>>),
            Nil,
        }
        let node1: Rc<RefCell<List>>;
        let node2: Rc<RefCell<List>>;
        {
            println!("Create node1 and node2 with node2 pointing to node1");
            node1 = Rc::new(RefCell::new(List::Node(
                2,
                Rc::new(RefCell::new(List::Nil)),
            )));
            node2 = Rc::new(RefCell::new(List::Node(1, Rc::clone(&node1))));
            // print the ref count of node1 and node2
            println!(
                "node1 strong = {}, weak = {}",
                Rc::strong_count(&node1),
                Rc::weak_count(&node1)
            );
            println!(
                "node2 strong = {}, weak = {}",
                Rc::strong_count(&node2),
                Rc::weak_count(&node2)
            );

            // put node2 into node1's next << this cause a reference cycle and drop node2 will not drop node1
            // if we don't put the node2 into node1, then drop node2 will drop node1
            println!("Put node2 into node1");
            match &*node1.borrow() {
                List::Node(v, next) => {
                    *next.borrow_mut() = List::Node(*v, Rc::clone(&node2));
                }
                List::Nil => {}
            }
            println!(
                "node1 strong = {}, weak = {}",
                Rc::strong_count(&node1),
                Rc::weak_count(&node1)
            );
            // print node2's ref count
            println!(
                "node2 strong = {}, weak = {}",
                Rc::strong_count(&node2),
                Rc::weak_count(&node2)
            );

            drop(node2);
            println!("After Drop node2 - node1 does not drop because of the reference cycle");
            println!(
                "node1 strong = {}, weak = {}",
                Rc::strong_count(&node1),
                Rc::weak_count(&node1)
            );
            // println!("step 2: node2 strong = {}, weak = {}", Rc::strong_count(&node2), Rc::weak_count(&node2));
        }
    }
    println!("\nScenario 2 - no cycle reference");
    {
        enum List {
            Node(i32, Rc<RefCell<List>>),
            Nil,
        }
        let node1: Rc<RefCell<List>>;
        let node2: Rc<RefCell<List>>;
        {
            println!("Create node1 and node2 with node2 pointing to node1");
            node1 = Rc::new(RefCell::new(List::Node(
                2,
                Rc::new(RefCell::new(List::Nil)),
            )));
            node2 = Rc::new(RefCell::new(List::Node(1, Rc::clone(&node1))));
            // print the ref count of node1 and node2
            println!(
                "node1 strong = {}, weak = {}",
                Rc::strong_count(&node1),
                Rc::weak_count(&node1)
            );
            println!(
                "node2 strong = {}, weak = {}",
                Rc::strong_count(&node2),
                Rc::weak_count(&node2)
            );

            drop(node2);
            println!("Drop node2 which also drop node1 automatically because they are not in a reference cycle");
            println!(
                "node1 strong = {}, weak = {}",
                Rc::strong_count(&node1),
                Rc::weak_count(&node1)
            );
            // println!("step 2: node2 strong = {}, weak = {}", Rc::strong_count(&node2), Rc::weak_count(&node2));
        }
    }
    {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            next: RefCell<Option<Rc<Node>>>,
            previous: RefCell<Option<Weak<Node>>>,
        }
        let node1 = Rc::new(Node {
            value: 1,
            next: RefCell::new(None),
            previous: RefCell::new(None),
        });
        let node2 = Rc::new(Node {
            value: 2,
            next: RefCell::new(None),
            previous: RefCell::new(None),
        });

        *node1.next.borrow_mut() = Some(Rc::clone(&node2));
        *node2.previous.borrow_mut() = Some(Rc::downgrade(&node1));

        println!(
            "node1 rc count: {}, weak count: {}",
            Rc::strong_count(&node1),
            Rc::weak_count(&node1)
        );
        println!(
            "node2 rc count: {}, weak count: {}",
            Rc::strong_count(&node2),
            Rc::weak_count(&node2)
        );

        match Rc::try_unwrap(node1) {
            Ok(_) => println!("Freed node1"),
            Err(_) => println!("Couldn't free node1"),
        }
        println!(
            "node2 rc count: {}, weak count: {}",
            Rc::strong_count(&node2),
            Rc::weak_count(&node2)
        );

        match Rc::try_unwrap(node2) {
            Ok(_) => println!("Freed node2"),
            Err(_) => println!("Couldn't free node2"),
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
