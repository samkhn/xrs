use core::fmt;

// The `derive` attribute automatically creates the implementation required to make this `struct`
// printable with `fmt::Debug`.
#[derive(Debug)]
struct SimpleStructure(i32);

#[derive(Debug)]
struct DeepStructure(SimpleStructure);

#[derive(Debug)]
struct Coordinate {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    // {} surrounds all formatting directives. Invokes the Display trait.
    // {:} : seperates the name or ordinal of the object being printed
    let v = vec![1, 2, 3];
    println!("v contains: {v:?}");

    // {:?} ? triggers std::fmt to use Debug as opposed to the default Display trait or other
    // traits like UpperHex.
    println!("{:?} months in a year.", 12);
    println!(
        "Print some struct with Debug trait like this: {:?}",
        SimpleStructure(7)
    );
    println!(
        "Or even deeper structures: {:?}",
        DeepStructure(SimpleStructure(10))
    );

    println!("Here is a coordinate: {}", Coordinate { x: 10.0, y: 10.0 });
}
