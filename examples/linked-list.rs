use crate::List::{Cons, Nil};

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    // not &self because we are consuming the passed in list
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
	    // self is of type &List
	    // tail here is a reference as well
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    // uses format() which is similar to print! but returns heap allocated string instead of
    // printing to console
    fn to_string(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.to_string())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(20);
    list = list.prepend(10);
    println!(
        "list has length {} and contains {}",
        list.len(),
        list.to_string()
    );
}
