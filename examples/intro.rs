// Important concepts in rust: structs, vectors, iteration, Result and Option
// Unique about rust: borrowing, ownership, lifetimes, reference, smartptrs

use std::collections::HashMap;

// If you want to instantiate Greeting
// let msg = Greeting{
//     prefix: String::from("Hello"),
//     name: String::from("Samiur"),
// };
struct Greeting {
    prefix: String,
    name: String,
}

// Commented for historical purpose.
// Constructor with &s only.
// When constructing msg you cannot pass a string literal such as prefix: "S".
// "S" is a ref-to-str and prefix expects a string.
// fn new(prefix: &str, name: &str) -> Self {
//    Greeting {
//        prefix: prefix.to_string(),
//        name: name.to_string(),
//    }
// }
impl Greeting {
    // Take any type T that can be represented as a ref to string
    fn new<T: AsRef<str>>(prefix: T, name: T) -> Self {
        Greeting {
            prefix: prefix.as_ref().to_string(),
            name: name.as_ref().to_string(),
        }
    }
}

// To make Greeting print itself by implementing a Trait std::fmt::Display
// Equivalent to operator overloading
impl std::fmt::Display for Greeting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Notice: this statement does not end in semicolon
        //  Result of this expression is returned
        write!(f, "{} {}!", self.prefix, self.name)
    }
}

// This is roughly how std::convert::Into is implemented
// Read as: for type T, we implement Into<U> if U implements From<T>, T will
// implement Into<U> impl<T, U> Into<U> for T where
//    U: From<T>,
// {
//    fn into(self) -> U {
//        U::from(self)
//    }
// }

// Warning
// clone() and to_string() makes copies.
//  Okay at the beginning to learn but prune it from vocab

fn main() {
    const ENGLISH_PREFIX: &str = "Hello";
    let name = "Sam";
    let msg = Greeting::new(ENGLISH_PREFIX, name);
    println!("{} {}!", msg.prefix, msg.name);

    let value: i32 = 58310;
    println!("Base 10: {}", value);
    println!("Base 2: {:b}", value);
    println!("Base 8: {:o}", value);
    println!("Base 16: {:x}", value);
    println!("Base 16: {:X}", value);

    // println!("{}", <String as AsRef<str>>::as_ref(&msg.prefix));  // prints
    println!("{}", Greeting::new("Hallo", "Bob"));
    // "Hello"

    // Rust ownership model
    //  (aside: T: Templated function that takes any type.)
    //  Norm: variables that start with _ are dropped.
    //  Under the hood, when you pass x by value here, _T ownership will
    //  transfer to the function scope Upon return, _T will get freed.
    let v = vec![1, 2, 3];
    // Decl is approximately: pub fn drop<T>(_x: T) {}
    drop(v);

    let mut letters = HashMap::new();
    for ch in "some text".chars() {
        // entry is an entry in the HashMap.
        // More: doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
        // Entry API
        // counter is a mutable reference to value in HashMap
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    letters.remove(&' ');
    for (key, value) in letters {
        println!("{} : {}", key, value);
    }

    // match (similar to C switch)
    let number = 15;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        12..=19 => println!("A teen"),
        _ => println!("Not interesting"),
    }
}
