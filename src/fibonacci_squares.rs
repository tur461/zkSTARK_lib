#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use std::time::Instant;

    use crate::{
        channel::{serialize, Channel},
        ffield_unit::FFieldUnit,
        merkle::MerkleTree,
        utils::hash256_str,
        FFPoly::{interpolate_poly, FFPoly},
    };

    fn part_one() -> (
        Vec<FFieldUnit>,
        FFieldUnit,
        Vec<FFieldUnit>,
        FFieldUnit,
        Vec<FFieldUnit>,
        Vec<FFieldUnit>,
        FFPoly,
        Vec<FFieldUnit>,
        MerkleTree,
        Channel,
    ) {
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
        let yvals = fibsq.clone();
        let f = interpolate_poly(&xvals, &yvals, "x");

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

        let w_inv = w.inverse();

        assert_eq!(
            "957ebc19754464f1dc110b6f7683961c2abf380955da4124888902763806beaa",
            hash256_str(h_group_coset[1].to_string().as_bytes()),
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
        assert_eq!(
            "1d357f674c27194715d1440f6a166e30855550cb8cb8efeb72827f6a1bf9b5bb",
            hash256_str(&serialized.as_bytes())
        );

        // commitments
        // using merkle tree

        let mut f_merkle = MerkleTree::new(&f_evals);
        f_merkle.build_tree();

        assert_eq!(
            "59e7ca76ed81c58aa10eacb4614e9e5ac598013d4562b71131bf5ef4e1cf42c6",
            f_merkle.root()
        );
        let mut channel = Channel::new();
        channel.send(&f_merkle.root());

        return (
            fibsq,
            generator,
            group,
            h_generator,
            h_group_coset,
            eval_domain,
            f,
            f_evals,
            f_merkle,
            channel,
        );
    }

    /// constraints over trace 'a': fibsq
    fn part_two() -> (
        FFPoly,
        Vec<FFieldUnit>,
        MerkleTree,
        Channel,
        Vec<FFieldUnit>,
    ) {
        // 3 steps:
        // 1. start by specifying the constraints we care about (the fibonacciSq constraints)
        // 2. translate the constraints into polynomial contraints
        // 3. translate those into rational functions that represent polynomials iff the original
        //    constraints hold
        let var = "x";
        let (a, g, G, h, H, eval_dom, f, f_eval, f_mrk, mut chan) = part_one();
        // step 1. FibSq constraints
        // For 'a' to be correct trace of a FibonacciSq  sequence that proves our claim:
        // a. the first element has to be 1, a[0] = 1.
        // b. the last element has to be 2338775057, a[1022] = 2338775057.
        // c. the fibonacci rule must apply, that is - for every i < 1021, a[i+2] = a[i+1]^2 +
        // a[i]^2.
        //
        // step 2. Polynomial constraints
        // recall that f is a polynomial over the trace domain, that evaluates exactly to 'a' over
        // G \ {g^1023} where G = {g^i: 0 <= i <= 1023} is a 'small' group generated by 'g'
        // we now rewrite the above 3 constraints in a form of polynomial constraints over f:
        // a. a[0] = 1 is translated as f(x) = 1 => f(x) - 1 = 0 => poly = f(x) - 1 which evaluates
        // to 0 for x = g^0 (note that g^0 is 1).
        // b. a[2022] = 2338775057, becomes f(x) - 2338775057, which evaluated to 0 for x = g^1022.
        // c.

        // step 1.a
        let numer_0 = f.clone() - FFPoly::new(vec![FFieldUnit::one()], var);
        let denom_0 = FFPoly::gen_linear_term(&FFieldUnit::one(), var);
        let (q_0, r_0) = numer_0.qdiv(&denom_0);
        assert_eq!(r_0, FFPoly::zero(var));
        assert_eq!(
            q_0.eval(&FFieldUnit::new(2718)),
            FFieldUnit::new(2509888982)
        );
        // step 1.b
        let numer_1 = f.clone() - FFPoly::new(vec![FFieldUnit::new(2338775057)], var);
        let denom_1 = FFPoly::gen_linear_term(&G[1022], var);
        let (q_1, r_1) = numer_1.qdiv(&denom_1);
        assert_eq!(r_1, FFPoly::zero(var));
        assert_eq!(q_1.eval(&FFieldUnit::new(5772)), FFieldUnit::new(232961446));

        // step 1.3
        let inner_poly_0 = FFPoly::new(vec![FFieldUnit::zero(), G[2].clone()], var);
        let final_0 = f.clone().compose(&inner_poly_0);

        let inner_poly_1 = FFPoly::new(vec![FFieldUnit::zero(), G[1].clone()], var);
        let composition = f.clone().compose(&inner_poly_1);

        let final_1 = composition.clone() * composition;
        let final_2 = f.clone() * f;

        let numer_2 = final_0 - final_1 - final_2;
        let mut coef = vec![FFieldUnit::one()];
        for _ in 0..1024 {
            coef.push(FFieldUnit::zero());
        }
        coef.push(FFieldUnit::new(-1));

        let numer_of_denom_2 = FFPoly::new(coef, var);

        let factor_0 = FFPoly::gen_linear_term(&G[1021], var);
        let factor_1 = FFPoly::gen_linear_term(&G[1022], var);
        let factor_2 = FFPoly::gen_linear_term(&G[1023], var);

        let denom_of_denom_2 = factor_0 * factor_1 * factor_2;

        let (denom_2, r_denom_2) = numer_of_denom_2.qdiv(&denom_of_denom_2);
        // assert_eq!(r_denom_2, FFPoly::zero(var));

        let (q_2, r_2) = numer_2.qdiv(&denom_2);
        // assert_eq!(r_2, FFPoly::zero(var));

        // from onwards we use channel
        let cp_0 = q_0.scalar_mul(&chan.receive_rnd_ffunit());
        let cp_1 = q_1.scalar_mul(&chan.receive_rnd_ffunit());
        let cp_2 = q_2.scalar_mul(&chan.receive_rnd_ffunit());

        let cp = cp_0 + cp_1 + cp_2;
        let cp_ev: Vec<FFieldUnit> = eval_dom.iter().map(|d| cp.eval(&d)).collect();
        let mut mkt = MerkleTree::new(&cp_ev);
        mkt.build_tree();
        assert_eq!(
            "e7eef880221bc0089d3b8f7997b334adcc1a9ad9893cad2df6acdc9b3acdb79c",
            mkt.root()
        );
        chan.send(&mkt.root());

        (cp, cp_ev, mkt, chan, eval_dom)
    }

    #[test]
    fn test_part_three() {
        let (cp, cp_ev, cp_mkt, chan, domain) = part_two();
        // FRI commit function
        let fri_polys = vec![cp];
        let fri_domains = vec![domain];
        let fri_layers = vec![cp_ev];
        let merkles = [cp_mkt];
    }
}
