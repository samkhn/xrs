extern crate vector;

use vector::Vector;

fn main() {
    let mut v: Vector<i32> = Vector::new();
    v.push(10);
    v.push(15);
    v.push(-13);
    assert!(v.pop() == Some(-13), "-13 failed");
    assert!(v.pop() == Some(15), "15 failed");
    assert!(v.pop() == Some(10), "10 failed");
    println!("Reached end of vector demo")
}
