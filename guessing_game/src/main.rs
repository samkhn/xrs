extern crate rand;

fn main() {
    println!("Welcome to the guessing game");
    println!("Give a non-negative int (upper limit):");
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("able to read line from stdin");
    // We need to trim because read_line() returns string with newline before and after.
    let seed: u32 = input.trim().parse::<u32>().unwrap();
    input.clear();
    let mut engine = rand::generate_mt19937(seed);
    let mut i = 0;
    while i < seed - 1 {
        _ = engine.gen_rand_uint32();
        i += 1;
    }
    let target = engine.gen_rand_uint32() % seed;
    let mut guess: u32;
    let mut guess_count: u32 = 0;
    let guess_count_limit: u32 = 10;
    println!("Ok. Guess what number it is (between 0 and {seed})");
    while guess_count < guess_count_limit {
        std::io::stdin()
            .read_line(&mut input)
            .expect("able to read line from stdin");
        guess = input.trim().parse::<u32>().unwrap();
	input.clear();
        if guess == target {
            println!("You won! It was {target}");
            println!("Took {guess_count} attempts");
            return;
        }
        guess_count += 1;
	println!("Nope!. Try again. This was attempt {guess_count}");
    }
    println!("Sorry, ran out of attempts ({guess_count_limit}). It was {target}");
}
