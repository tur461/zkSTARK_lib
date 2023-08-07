use num::integer::Roots;
use polynomial::Polynomial as Poly;
use std::mem::replace;
use std::time::Instant;

mod ffield_unit;
use ffield_unit::FFieldUnit;

mod FFPoly;
use FFPoly::{interpolate_poly, FFPoly as P};

mod utils;

mod channel;

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

fn main() {
    //let mut ip = String::new();
    //std::io::stdin().read_line(&mut ip).expect("a num");
    //let ip = ip.trim().parse::<usize>().unwrap();
    // let p = P::new(coeffs);
    // println!("Xpns: {}", p.display());
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use super::*;
    use channel::serialize;
    use sha2::{Digest, Sha256};
    use utils::u8_to_hexstr;

    #[test]
    fn test_part_one() {
        let start = Instant::now();
        let mut j: usize = 2;

        //----------------------------------------
        // fast way of generating fib squares sums
        //----------------------------------------
        let mut fibsq = Vec::<FFieldUnit>::new();
        let mut f0 = FFieldUnit::new(1);
        let mut f1 = FFieldUnit::new(3141592);
        fibsq.push(f0);
        fibsq.push(f1);

        while j < 1023 {
            f0 = fibsq[j - 2];
            f1 = fibsq[j - 1];
            fibsq.push((f0 * f0) + (f1 * f1));
            j += 1;
        }
        //----------------------------------------

        let generator = FFieldUnit::generator();
        // group: G
        let group: Vec<FFieldUnit> = (0..1024)
            .into_iter()
            .map(|n| FFieldUnit::pow(generator, n))
            .collect();

        let group_len = group.len();

        println!("G len {}", group_len);
        println!("result len {}", fibsq.len());
        // all
        let xvals = group[..group_len - 1].to_vec();
        let yvals = fibsq;
        //println!("x vals: {:?} len: {}", xvals[1013..].to_vec(), xvals.len());
        //println!("y vals: {:?} len: {}", yvals[1013..].to_vec(), yvals.len());
        // first 10
        // let xvals = xvals[..10].to_vec();
        // let yvals = yvals[..10].to_vec();
        // last 10
        //let xvals = xvals[1013..].to_vec();
        //let yvals = yvals[1013..].to_vec();
        let f = interpolate_poly(&xvals, &yvals, "x");
        // println!(
        //     "Interpolated Polynomial: \nFirst 10:\n{:?} \n..\nLast 10:\n{:?}",
        //     f.coeffs[..10].to_vec(),
        //     f.coeffs[1013..].to_vec()
        // );
        assert_eq!(FFieldUnit::new(1302089273), f.eval(&FFieldUnit::new(2)));
        println!("[interpolation done] calculated in {:?}", start.elapsed());

        // now we need to evaluate on larger domain
        // The trace, viewed as evaluation of a polynomial f (interpolated poly) on G (group),
        // can now be extended by evaluating f over a larger domain, thereby creating a
        // Reed-Solomon error correction code
        //
        // Cosets
        // we must decide on a larger domain on which f will be evaluated.
        // we'll work with a domain which is 8 times larger than the G (group): len = 8 * group_len
        //
        // A natural choice for such a domain is to take some group H of size 8 * group_len
        // = 2^3 * 2^10 = 2^13 = 8192
        //
        let w = FFieldUnit::ffgenerator();
        let h_generator = FFieldUnit::pow(w, 2u32.pow(30).mul(3) / 8192);
        assert_eq!(h_generator, FFieldUnit::new(1734477367));

        // generate the group same as we did before
        // here len will be 8 x that
        let h_group_coset: Vec<FFieldUnit> = (0..8192)
            .into_iter()
            .map(|i| FFieldUnit::pow(h_generator, i))
            .collect();

        assert_eq!(h_group_coset.len(), 8 * group_len);
        let eval_domain: Vec<FFieldUnit> =
            h_group_coset.clone().into_iter().map(|h| w * h).collect();
        assert_eq!(eval_domain.len(), 8 * group_len);
        let mut hasher = Sha256::new();

        hasher.update(h_group_coset[1].to_string().as_bytes());
        let hash = hasher.finalize();
        let w_inv = w.inverse();
        assert_eq!(
            "957ebc19754464f1dc110b6f7683961c2abf380955da4124888902763806beaa",
            u8_to_hexstr(&hash),
            "h_group_coset is wrong, as h_group_coset[1] must be h_generator"
        );

        (0..8191usize).into_iter().for_each(|i| {
            assert_eq!(
                FFieldUnit::pow(w_inv * eval_domain[1], i as u32) * w,
                eval_domain[i]
            )
        });

        // now time to evaluate on the Coset
        let f_evals: Vec<FFieldUnit> = eval_domain.iter().map(|ed| f.eval(ed)).collect();
        let serialized = serialize(&f_evals[..]);
    }
}
