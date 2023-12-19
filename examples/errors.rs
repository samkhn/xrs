fn drink1(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAA!!!")
    }
    println!("Drinking {}", beverage);
}

// panic strategy can be set by command line
// `$ rustc errors.rs -C panic=abort`
fn drink2(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("lemonade aint it. run!")
        } else {
            panic!("AAA!!!")
        }
    } else {
        println!("Drinking {}", beverage);
    }
}

#[cfg(panic = "unwind")]
fn react() {
    println!("run!")
}

#[cfg(not(panic = "unwind"))]
fn react() {
    println!("AAA!")
}

fn drink3(beverage: &str) {
    if beverage == "lemonade" {
        react()
    } else {
        println!("Drinking {}", beverage);
    }
}

fn main() {
    drink1("water");
    drink2("water");
    drink3("water");
}
