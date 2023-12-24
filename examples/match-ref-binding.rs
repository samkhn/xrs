fn age() -> i32 {
    34
}

fn main() {
    // ref matching
    let a: &i32 = &4; // also let a = &4;
    match a {
        &val => println!("This is just a (reference to 4): {:?}", val),
    }
    match *a {
        val => println!("This is deref a (which references 4): {:?}", val),
    }
    let ref b: i32 = 3; // also let ref b = 3;
    match b {
        ref val => println!("Got a ref to b: {:?}", val),
    }
    // you can declare a reference (above), or ensure that the match is a reference (below)
    let c = 5;
    let mut d = 6;
    match c {
        ref val => println!("Grabbed a reference to a c: {:?}", val),
    }
    match d {
        ref mut val => {
            // to access the value, we have to deref
            *val += 15;
            println!("d, which was 6, was added to 15 to get {:?}", val);
        }
    }

    // binding with @ sigil
    match age() {
        0 => println!("Haven't reached 1"),
        n @ 1..=13 => println!("I am a child of age {:?}", n),
        n @ 14..=21 => println!("I am a teenager of age {:?}", n),
        n @ 22..=100 => println!("I am an adult of age {:?}", n),
        n @ i32::MIN..=0 | n @ 101..=i32::MAX => println!("wow cool age lol: {:?}", n),
    }
}
