use num::integer::Roots;
use polynomial::Polynomial as Poly;
use std::mem::replace;
use std::time::Instant;

mod ffield_unit;
use ffield_unit::FFieldUnit;

mod FFPoly;
use FFPoly::FFPoly as P;

// Calculate large fibonacci numbers.
fn fib_sq(n: u32) -> FFieldUnit {
    let mut f0 = FFieldUnit::new(1);
    let mut f1 = FFieldUnit::new(3141592);
    for _ in 0..n {
        let f2 = (f0 * f0) + (f1 * f1);
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

//fn is_prime(n: U512) -> bool {
//    let ubound = n.sqrt();
//    println!("ubound of {} is {} ", n, ubound);
//
//    for d in (2..=ubound).into_iter() {
//        if n % d == 0 {
//            return false;
//        }
//    }
//
//    true
//}

fn evalute_zk() {
    let ip: usize = 1023;
    let to_find = FFieldUnit::new(2338775057);
    let start = Instant::now();
    let mut j: usize = 2;

    let mut result = Vec::<FFieldUnit>::new();
    let mut f0 = FFieldUnit::new(1);
    let mut f1 = FFieldUnit::new(3141592);
    result.push(f0);
    result.push(f1);

    while j < 1021 {
        f0 = result[j - 2];
        f1 = result[j - 1];
        result.push((f0 * f0) + (f1 * f1));
        j += 1;
    }

    // let result: Vec<U512> = (0..=ip)
    //     .into_iter()
    //     .enumerate()
    //     .map(|(n, i)| {
    //         let x = fib_sq(n as u32);
    //         if FFieldUnit::eq(x, to_find) {
    //             println!("FOUND!! At: {}", i);
    //             j = i
    //         }
    //         x.0
    //     })
    //     .collect();
    println!("Prime modulo: {}", FFieldUnit::modulo_prime());
    // println!("pow: {}", ff_pow(8, 80, prime_modulo));
    println!(
        "Result at index {} is {:?} calculated in {:?}",
        j,
        result[j - 1],
        start.elapsed()
    );
    //println!("{:?}", result);
    let g = FFieldUnit::generator();
    println!("g: {:?} {}", g, g.0);
    let G: Vec<i128> = (0..1024)
        .into_iter()
        .map(|n| FFieldUnit::pow(g, n).0)
        .collect();

    println!("Generated: {:?}", G[1021]);
    println!("Generated: {:?}", G[1022]);
    println!("Generated: {:?}", G[1023]);
    let poly = Poly::new(vec![1, 0, 2]);
    println!("poly: {:?}\npretty: {:?}", poly.eval(2), poly.pretty("x"));
    // println!("{:?}", G);
}

fn main() {
    //let mut ip = String::new();
    //std::io::stdin().read_line(&mut ip).expect("a num");
    //let ip = ip.trim().parse::<usize>().unwrap();
    evalute_zk();
    let coeffs: Vec<i128> = vec![1i128, 2i128, 3i128];
    // let p = P::new(coeffs);
    // println!("Xpns: {}", p.display());
}
