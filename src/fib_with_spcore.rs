use num::integer::Roots;
use sp_core::U512;
use std::mem::replace;
use std::time::{Duration, Instant};

// Calculate large fibonacci numbers.
fn fib_sq(n: usize) -> U512 {
    let mut f0 = U512::from_dec_str("0").unwrap();
    let mut f1 = U512::from_dec_str("1").unwrap();
    let two = U512::from_dec_str("2").unwrap();
    for _ in 0..n {
        let f2 = f0.pow(two) + &f1.pow(two);
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn is_prime(n: u32) -> bool {
    let ubound = n.sqrt();
    println!("ubound of {} is {} ", n, ubound);

    for d in (2..=ubound).into_iter() {
        if n % d == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut ip = String::new();
    std::io::stdin().read_line(&mut ip).expect("a num");
    let ip = ip.trim().parse::<usize>().unwrap();

    let start = Instant::now();
    let result: Vec<U512> = (1..=ip).into_iter().map(|n| fib_sq(n)).collect();

    println!("Result {:?} in {:?}", result, start.elapsed());
    let _2pow30: U512 =
        U512::from(3) * U512::from(2).pow(U512::from(30)) + <i32 as Into<U512>>::into(1);
    let prime_modulo = 3 * 2_u32.pow(30) + 1;
    println!(
        "is {} prime? Reply: {}",
        prime_modulo,
        is_prime(prime_modulo)
    );
    println!("Log of 2 pow 30 {:?}", _2pow30);
    //let r = fib_sq(10);
    //println!("Found 1023th term: {} in {:?}", r, start.elapsed());
}
