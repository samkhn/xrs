#![allow(dead_code)]

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

// Unit structs, useful for generics
struct Unit;

// Tuple
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(self: &Self) -> f64 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;
        ((x1 - x2) * (y1 - y2)).abs().into()
    }
}

// enum example is in linked-list.rs

fn main() {
    println!("This is the {} programming language", LANGUAGE);

    let rectangle = Rectangle {
        top_left: Point { x: -1.0, y: 1.0 },
        bottom_right: Point { x: 1.0, y: -1.0 },
    };

    println!("area of rectangle is {} (should be 4)", rectangle.area());
}
