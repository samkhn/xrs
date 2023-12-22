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

// macros can repeat with +
macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y), +))
    )
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
    println!("min of 5 6 3 is {}", find_min!(5, 6, 3))
}
