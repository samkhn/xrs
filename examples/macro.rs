use std::ops::{Add, Mul, Sub};

macro_rules! say_hey {
    // macro takes no arguments
    () => {
        // expands contents of this block
        println!("hey")
    };
}

macro_rules! create_function {
    // macro takes argument with designator `ident` and creates function named `func_name`
    // ident is used for variables and functions
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

create_function!(foo);

macro_rules! print_result {
    // another example of
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

// test!() tests $left $right logical relationship
// macros work similar to a match
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    // macros can repeat with +
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

macro_rules! assert_equal_len {
    // tt is for token tree designator
    // used for operators and tokens
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

// DSL, how something like lazy_static gets implemented
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    };
    (eval $e:expr, $(eval $es:expr), +) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    say_hey!();
    foo();
    print_result!({
        let x = 1u32;
        x * x + 2 - 1
    });

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("min of 1 is {}", find_min!(1));
    println!("min of 5 and 2*3 is {}", find_min!(5, 2 * 3));
    println!("min of 5 6 3 is {}", find_min!(5, 6, 3));

    calculate! {
        eval 1 + 2,
        eval 3 + 4
    }
}

mod test {
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = std::iter::repeat($x).take(size).collect();
                    let y: Vec<_> = std::iter::repeat($y).take(size).collect();
                    let z: Vec<_> = std::iter::repeat($z).take(size).collect();
                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
