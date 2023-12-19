fn main() {
    println!("Find the sum of all the numbers with odd squares under 1000");
    let upper = 1000;

    // imperative
    let mut acc = 0;
    for n in 0.. {
        let n_s = n * n;
        if n_s >= upper {
            break;
        } else if n_s % 2 == 1 {
            acc += n_s;
        }
    }

    // functional
    let sum_of_odd_squares: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_s| n_s < upper)
        .filter(|&n_s| n_s % 2 == 1)
        .sum();

    assert_eq!(acc, sum_of_odd_squares);
    println!("its {:?}", acc);
}
