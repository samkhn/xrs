use std::fmt::Debug;

#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(self: &Self) -> f64;
}

impl HasArea for Rectangle {
    fn area(self: &Self) -> f64 {
        self.length * self.height
    }
}

impl HasArea for Triangle {
    fn area(self: &Self) -> f64 {
        0.5 * self.height * self.length
    }
}

fn area<S: HasArea>(shape: &S) -> f64 {
    shape.area()
}

fn main() {
    let r = Rectangle {
        length: 5.0,
        height: 4.0,
    };
    let t = Triangle {
        length: 5.0,
        height: 4.0,
    };
    assert_eq!(area(&r), area(&t) * 2.0);
}
