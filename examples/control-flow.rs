use std::str::FromStr;

// Foo, used in if-let demonstration
enum Foo {
    Bar,
    Baz,
    Quz(u32),
}

// example: "3 chairs" becomes (3, "chairs")
// alternative implementation (less clean):
// let (count_str, item) = match (iter.next(), iter.next()) {
//     (Some(count_str), Some(item)) => (count_str, item),
//     _ => panic!("Can't segment count item pair: '{s}'"),
// };
// let count = if let Ok(count) = u64::from_str(count_str) {
//     count
// } else {
//     panic!("Can't parse int from '{count_str}'");
// };
fn get_count_item(s: &str) -> (u64, &str) {
    let mut iter = s.split(' ');
    let (Some(count_str), Some(item)) = (iter.next(), iter.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn main() {
    // if let to match optionals
    // Instead of this:
    // let o = Some(7);
    // match o {
    // 	Some(i) => println!("Got value {:?}", i),
    // 	_ => {},  // required to be exhaustive
    // }
    let does = Some(10);
    let doesnt: Option<i32> = None;
    if let Some(i) = does {
        println!("We have a value {:?}", i);
    }
    if let Some(i) = doesnt {
        println!("`doesnt` does have the value {:?}", i);
    } else {
        println!("`doesnt` doesnt have a value");
    }

    // if-let to match enums
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Quz(100);
    if let Foo::Bar = a {
        println!("a is Foo::Bar");
    }
    if let Foo::Bar = b {
        println!("b is Foo::Bar");
    } else {
        println!("b is NOT Foo::Bar");
    }
    if let Foo::Quz(val @ 100) = c {
        println!("c is Foo::Quz({:?})", val);
    }

    // let-else
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // while-let
    let mut optional = Some(0);
    // alternative implementation (less clean)
    // loop {
    //     match optional {
    //         Some(i) => {
    //             if i > 9 {
    //                 println!("Greater than 9, lets go!");
    //             } else {
    //                 println!("i is {:?}, lets incr");
    //                 optional = Some(i + 1);
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, lets go!");
            break;
        } else {
            println!("i is {:?}, lets incr", i);
            optional = Some(i + 1);
        }
    }
}
