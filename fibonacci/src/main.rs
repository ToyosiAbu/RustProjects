extern crate cmn;
use std::io;
use cmn::Constants;

fn main() {
    println!("Good Morning, I am the Number Man!");
    let constants = Constants::new();
    let golden_ratio = constants.constant("PHI").unwrap().value;
    let golden_ratio: f64 = golden_ratio.parse()
        .expect("Failed to parse PHI value as f64");
    let sqrt_5 = constants.constant("SQRT5").unwrap().value;
    let sqrt_5: f64 = sqrt_5.parse()
        .expect("Failed to parse SQRT5 value as f64");

    loop {
        println!("Please enter which fibonnaci number you want (Ex: for 3rd, enter 3): ");
        let mut nth_fib = String::new();

        io::stdin().read_line(&mut nth_fib)
            .expect("Failed to read line");

        let nth_fib: i32 = match nth_fib.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You inputted fibonnaci number {}", nth_fib);
        let binet = ((golden_ratio.powi(nth_fib) - (1.0 - golden_ratio).powi(nth_fib)) / sqrt_5).round();
        println!("Using Binet's forumula, this is {}", binet);
        let fib_recurse = indexToNumber(nth_fib);

        println!("Using Recursion, this is {}",fib_recurse);
    }
}

fn indexToNumber(n: i32) -> i32 {
    /// if n positive, let f(n) = f(n-1) + f(n-2), else let f(n) = f(n+2) - f(n+1)
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n > 1 {
        indexToNumber(n-1) + indexToNumber(n-2)
    } else {
        indexToNumber(n+2) - indexToNumber(n+1)
    }
} 