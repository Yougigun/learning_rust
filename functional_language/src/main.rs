#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type
)]
use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} get {:?}",
        user_pref1, giveaway1
    );

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // because the sort happens in place, so we need the mutable reference
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // This doesn't compile!
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    let max = data.iter().max();
    let min = data.iter().min();

    println!("Max: {:?}", max);
    println!("Min: {:?}", min);

    // Methods that Produce Other Iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // Using Closure that Capture Their Environment
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes
            .into_iter()
            .filter(|s| s.size == shoe_size)
            .collect::<Vec<_>>()
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}
/// A collection of shirts in a particular size.
/// # Examples
/// ```
/// let store = crate::Inventory {
///    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
/// };
/// let user_pref1 = Some(ShirtColor::Red);
/// let giveaway1 = store.giveaway(user_pref1);
/// assert_eq!(giveaway1, ShirtColor::Red);
/// ```
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    /// Giveaway a shirt based on user's preference or the most stocked one.
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // ps: due to ShirtColor has implemented Copy, so Option is also copyable without being taken ownership
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    /// Find the most stocked shirt color in the inventory.
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red -= 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

mod capture_reference_or_moving_ownership {
    fn immutable_example() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // Here `list` is captured as an immutable reference.
        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    fn mutable_example() {
        // need to declare it as mutable for mut closure
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // Here `list` is captured as a mutable reference,
        // since we `push` a new item onto the list.
        // ! also need to declare closure as mutable
        let mut borrows_mutably = || list.push(7);

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }
}
