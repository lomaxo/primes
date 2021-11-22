use std::env::args;
use std::time::Instant;

fn usage() {
    println!("Usage primes <number>");
    println!("Usage primes <number> display"); 
}

fn main() {
    let mut arguments = args().skip(1);
    //let max_prime: usize = arguments[0].parse().unwrap();
    let max_prime: usize = match arguments.next() {
        Some(i) => { i.parse().unwrap() },
        None => {
            usage();
            return;
        }

    };
    let display: bool = match arguments.next() {
        Some(s) => {
            if s == "display" {
                true
            } else {
                usage();
                return;
            }
        },
        None => { false }
    };
    let start_time = Instant::now();
    let mut sieve = vec![true; max_prime];
    for n in 2..max_prime {
        if sieve[n] {
            if display {
                print!("{}, ", n);
            }
            let mut i = n*n;
            while i < max_prime {
                sieve[i] = false;
                i = i + n;
            }
        }
    }
    // Count how many primes found
    let number_of_primes = sieve.into_iter().filter(|&p| p).count()-2;
    let finish_time = Instant::now();
    println!("\nNumber of primes found below {}: {} ", max_prime, number_of_primes);
    println!("Time taken: {:?}", finish_time.duration_since(start_time));
}
