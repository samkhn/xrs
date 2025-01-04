// generating random values

use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand::prelude::*;
use rand_distr::{Normal, NormalError};

fn rand_demo() {
    // we can use the rand create to generate random numbers
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random float: {}", rng.gen::<f32>()); // [0, 1)

    // within range
    println!("Integer within range: {}", rng.gen_range(0..10));
    println!("Float within range: {}", rng.gen_range(0.0..10.0));
}

fn uniform_distribution() {
    // uniform distribution
    // example of rolling a die:
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Rolling the die gave us: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn normal_distribution() -> Result<(), NormalError> {
    // we can also create our own distributions with rand_distributions
    // example with Normal distribution below
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?; // mean 2.0, stdev 3.0
    let v = normal.sample(&mut rng);
    println!("{} is from a Normal(2.0, 3.0) distribution", v);
    Ok(())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn custom_type() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Rand tuple contains: {:?}", rand_tuple);
    println!("Rand point is: {:?}", rand_point)
}

fn alphanumeric() {
    let rand_string: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Random 30 char string is: {}", rand_string);
}

fn password_generator() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN).map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    })
    .collect();
    println!("Generated random password: {:?}", password);
}

fn main() {
    rand_demo();
    uniform_distribution();
    let _ = normal_distribution();
    custom_type();
    alphanumeric();
    password_generator();
}
