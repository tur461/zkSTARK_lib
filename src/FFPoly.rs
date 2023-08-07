use crate::ffield_unit::FFieldUnit;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct FFPoly {
    pub var: String,
    pub coeffs: Vec<FFieldUnit>,
}

impl Clone for FFPoly {
    fn clone(&self) -> FFPoly {
        Self {
            var: self.var.clone(),
            coeffs: self.coeffs.clone(),
        }
    }
}

impl FFPoly {
    pub fn new(cfs: Vec<FFieldUnit>, vr: &str) -> Self {
        Self {
            var: String::from(vr),
            coeffs: Self::rm_trailing_with(&cfs, &FFieldUnit::zero()),
        }
    }

    pub fn new_no_trim(cfs: Vec<FFieldUnit>, vr: &str) -> Self {
        Self {
            var: String::from(vr),
            coeffs: cfs,
        }
    }

    pub fn zero(v: &str) -> Self {
        Self::new(vec![FFieldUnit::zero()], v)
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len().checked_sub(1).unwrap_or(0)
    }

    pub fn get_coeff_at_degree(&self, n: usize) -> FFieldUnit {
        if n <= self.degree() {
            self.coeffs[n].clone()
        } else {
            FFieldUnit::zero()
        }
    }

    pub fn scalar_mul(&self, v: &FFieldUnit) -> Self {
        let nw_coeffs = self.coeffs.iter().map(|c| c.clone() * v.clone()).collect();
        Self::new(nw_coeffs, &self.var)
    }

    fn trim_trailing_zeroes(&self) -> Self {
        Self {
            var: self.var.clone(),
            coeffs: Self::rm_trailing_with(&self.coeffs, &FFieldUnit::zero()),
        }
    }

    fn rm_trailing_with(v: &[FFieldUnit], w: &FFieldUnit) -> Vec<FFieldUnit> {
        let mut res = v.to_vec();
        while let Some(&l) = res.last() {
            if l == w.clone() {
                res.pop();
            } else {
                break;
            }
        }
        res
    }

    pub fn eval(&self, vr_val: &FFieldUnit) -> FFieldUnit {
        let val = vr_val.clone();

        self.coeffs
            .iter()
            .rev()
            .fold(FFieldUnit::zero(), |acc, coef| {
                acc * val.clone() + coef.clone()
            })
    }

    pub fn compose(&self, other: &Self) -> Self {
        let mut res = Self::new(vec![FFieldUnit::zero()], &self.var);
        for coef in self.coeffs.iter().rev() {
            res = (res.clone() * other.clone()) + Self::new(vec![coef.clone()], &self.var);
        }
        res.trim_trailing_zeroes()
    }

    pub fn monomial(deg: usize, coef: FFieldUnit, var: &str) -> Self {
        let mut coeffs: Vec<FFieldUnit> =
            (0..deg).into_iter().map(|_| FFieldUnit::zero()).collect();
        coeffs.push(coef);
        Self::new(coeffs, var)
    }

    pub fn div_euclid_recursive(f: &Self, g: &Self) -> (Self, Self) {
        let var = f.var.as_str();
        let (d1, d2) = (f.degree(), g.degree());
        let c1 = &f.coeffs;
        let c2 = &g.coeffs;

        if d1 < d2 {
            (Self::zero(var), f.clone())
        } else if d1 == 0 {
            if c1.get(0).unwrap_or(&FFieldUnit::zero()) == &FFieldUnit::zero() {
                panic!("division by 0!");
            }
            (Self::new_no_trim(vec![c1[0] / c2[0]], var), Self::zero(var))
        } else {
            let [c_a, c_b] = [c1[d1], c2[d2]];
            let mut q_1 = Self::new_no_trim(vec![FFieldUnit::zero(); d1 - d2 + 1], var);
            q_1.coeffs[d1 - d2] = c_a / c_b;
            let h_2 = f.clone() - q_1.clone() * g.clone();
            let (q_2, r) =
                Self::div_euclid_recursive(&h_2.trim_trailing_zeroes(), &g.trim_trailing_zeroes());
            (q_1 + q_2, r)
        }
    }

    pub fn qdiv(&self, other: &Self) -> (Self, Self) {
        Self::div_euclid_recursive(&self.trim_trailing_zeroes(), &other.trim_trailing_zeroes())
        // let poly1 = self.trim_trailing_zeroes();
        // let poly2 = other.trim_trailing_zeroes();
        //
        // let len1 = poly1.coeffs.len();
        // let len2 = poly2.coeffs.len();
        // // println!("len1: {} len2: {}", len1, len2);
        //
        // if len1 == 0 {
        //     return (Self::new(vec![], &self.var), Self::new(vec![], &self.var));
        // }
        // let mut rem = poly1.clone();
        // let mut deg_diff = len1 - len2;
        // // println!("deg diff: {}", deg_diff);
        // let mut quotient: Vec<FFieldUnit> = (0..(deg_diff + 1))
        //     .into_iter()
        //     .map(|_| FFieldUnit::zero())
        //     .collect();
        // let g_msc_inv = poly2.coeffs.last().unwrap().inverse();
        // // println!("q: {:?}", quotient);
        //
        // while deg_diff >= 0 {
        //     let tmp = g_msc_inv * rem.coeffs.last().unwrap();
        //     quotient[deg_diff] = quotient[deg_diff] + &tmp;
        //     let mut last_non_zero = deg_diff - 1;
        //     for (i, coef) in poly2.coeffs.iter().enumerate() {
        //         rem.coeffs[i + deg_diff] = rem.coeffs[i + deg_diff] - (tmp * coef);
        //         if !rem.coeffs[i + deg_diff].is_zero() {
        //             last_non_zero = i + deg_diff;
        //         }
        //     }
        //     rem = rem.trim_trailing_zeroes();
        //     deg_diff = last_non_zero - len2;
        // }
        // println!("q: {:?}", quotient);
        // (Self::new(quotient, &self.var).trim_trailing_zeroes(), rem)
    }
}

impl Add<FFPoly> for FFPoly {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (longer, shorter) = if self.coeffs.len() >= other.coeffs.len() {
            (self.clone(), other)
        } else {
            (other, self.clone())
        };

        let mut result = longer.coeffs.clone();
        for (i, coef) in shorter.coeffs.iter().enumerate() {
            result[i] = result[i].clone() + coef.clone();
        }
        Self::new(result, &self.var)
    }
}

impl Sub<FFPoly> for FFPoly {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (longer, shorter) = if self.coeffs.len() >= other.coeffs.len() {
            (self.clone(), other)
        } else {
            (other, self.clone())
        };
        // we can make utility fn out of this
        let result: Vec<FFieldUnit> = longer
            .coeffs
            .iter()
            .enumerate()
            .map(|(i, x)| x.clone() - shorter.coeffs.get(i).unwrap_or(&FFieldUnit::zero()).clone())
            .collect();

        Self::new(result, &self.var)
    }
}

impl Mul<FFPoly> for FFPoly {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = vec![FFieldUnit::zero(); self.coeffs.len() + other.coeffs.len() - 1];

        for (i, coef1) in self.coeffs.iter().enumerate() {
            for (j, coef2) in other.coeffs.iter().enumerate() {
                result[i + j] = result[i + j].clone() + (coef1.clone() * coef2.clone());
            }
        }
        Self::new(result, &self.var).trim_trailing_zeroes()
    }
}

impl Mul<FFieldUnit> for FFPoly {
    type Output = Self;

    fn mul(self, scalar: FFieldUnit) -> Self {
        self.scalar_mul(&scalar)
    }
}

impl Mul<FFPoly> for FFieldUnit {
    type Output = FFPoly;

    fn mul(self, other: FFPoly) -> FFPoly {
        other.scalar_mul(&self)
    }
}

impl Div<FFPoly> for FFPoly {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let res = self.qdiv(&other);
        assert_eq!(res.1.coeffs.len(), 0, "polynomials not divisible");
        // return only q
        res.0
    }
}

impl Div<FFieldUnit> for FFPoly {
    type Output = Self;

    fn div(self, scalar: FFieldUnit) -> Self {
        let scalar_inv = FFieldUnit::one() / scalar;
        // return only q
        self.scalar_mul(&scalar_inv)
    }
}

/// x_vals: &[FFieldUnit]
///
pub fn calc_langrange_polys(x_vals: &[FFieldUnit], var: &str) -> Vec<FFPoly> {
    let len = x_vals.len();
    let mut lang_polys = Vec::<FFPoly>::new();
    let monomials: Vec<FFPoly> = x_vals
        .iter()
        .map(|&x| FFPoly::monomial(1, FFieldUnit::one(), var) - FFPoly::monomial(0, x, var))
        .collect();
    let numerator = prod(&monomials, var);
    for j in 0..len {
        //let mut denominator = FFieldUnit::one();
        let mut v: Vec<FFieldUnit> = Vec::new();
        for (i, x) in x_vals.iter().enumerate() {
            if i != j {
                v.push(x_vals[j].clone() - x.clone());
            }
        }
        let denominator = prod_ffunits(&v);

        let (poly, _) = numerator
            .clone()
            .qdiv(&monomials[j].scalar_mul(&denominator));
        lang_polys.push(poly);
    }
    lang_polys
}

/// y_vals: &[FFieldUnit]
/// lang_polys: &[FFPoly]
///
pub fn interpolate_lang_poly(y_vals: &[FFieldUnit], lang_polys: &[FFPoly], var: &str) -> FFPoly {
    let mut poly = FFPoly::new(vec![], var);
    for (j, y_val) in y_vals.iter().enumerate() {
        // println!("i: {}", j);
        poly = poly + lang_polys[j].clone().scalar_mul(y_val);
    }
    poly
}

/// multi threaded version
/// y_vals: &[FFieldUnit]
/// lang_polys: &[FFPoly]
///
pub fn interpolate_lang_poly_threaded(
    y_vals: &[FFieldUnit],
    lang_polys: &[FFPoly],
    var: &str,
    jobs: usize,
) -> FFPoly {
    let mut poly = FFPoly::new(vec![], var);
    for (j, y_val) in y_vals.iter().enumerate() {
        // println!("i: {}", j);
        poly = poly + lang_polys[j].clone().scalar_mul(y_val);
    }
    poly
}

/// x_vals: &[FFieldUnit]
/// y_vals: &[FFieldUnit]
///
pub fn interpolate_poly(x_vals: &[FFieldUnit], y_vals: &[FFieldUnit], var: &str) -> FFPoly {
    assert_eq!(x_vals.len(), y_vals.len());

    let lang_polys = calc_langrange_polys(x_vals, var);
    interpolate_lang_poly(y_vals, &lang_polys, var)
}

pub fn prod(vals: &[FFPoly], var: &str) -> FFPoly {
    let len = vals.len();
    match len {
        0 => FFPoly::new(vec![FFieldUnit::zero()], var),
        1 => vals[0].clone(),
        _ => {
            let half = len / 2;
            prod(&vals[..half], var) * prod(&vals[half..], var)
        }
    }
}

pub fn prod_ffunits(vals: &[FFieldUnit]) -> FFieldUnit {
    let len = vals.len();
    match len {
        0 => FFieldUnit::zero(),
        1 => vals[0].clone(),
        _ => {
            let half = len / 2;
            prod_ffunits(&vals[..half]) * prod_ffunits(&vals[half..])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_ffunits_in_range(s: i128, e: i128) -> Vec<FFieldUnit> {
        (s..e).into_iter().map(|x| FFieldUnit::new(x)).collect()
    }

    fn get_instance_in_range(s: i128, e: i128) -> FFPoly {
        let coeffs: Vec<FFieldUnit> = get_ffunits_in_range(s, e);
        FFPoly::new(coeffs.clone(), "x")
    }

    #[test]
    fn test_creates_instance() {
        let fpoly = get_instance_in_range(0, 3);

        assert_eq!(fpoly.var, "x");
        assert_eq!(fpoly.coeffs.len(), 3);
        assert_eq!(
            fpoly.coeffs.into_iter().map(|x| x.0).collect::<Vec<i128>>(),
            [0, 1, 2]
        );
    }

    #[test]
    fn test_evaluates_a_poly_on_given_val() {
        let fpoly = get_instance_in_range(1, 4);

        let res = fpoly.eval(&FFieldUnit::new(2));

        // poly: 3.x^2 + 2.x + 0
        // for x = 2,
        // poly = 17
        assert_eq!(res, FFieldUnit::new(17));
    }

    #[test]
    fn test_generates_monomials() {
        let units = get_ffunits_in_range(0, 2);
        assert_eq!(units.len(), 2);

        let mono = FFPoly::monomial(1, FFieldUnit::one(), "x");

        assert_eq!(mono.coeffs.len(), 2);
        assert_eq!(mono.coeffs, units);

        let mono = FFPoly::monomial(0, FFieldUnit::one(), "x");

        assert_eq!(mono.coeffs.len(), 1);
        assert_eq!(mono.coeffs[0], units[1]);

        let mono = FFPoly::monomial(0, FFieldUnit::zero(), "x");
        // because the single item would have been FFUnit::zero() which
        // is removed by trim_trailing funtion
        assert_eq!(mono.coeffs.len(), 0);
    }

    #[test]
    fn test_calculates_product_of_monomials() {
        let units = get_ffunits_in_range(0, 3);
        assert_eq!(units.len(), 3);
        let monos: Vec<FFPoly> = units
            .iter()
            .map(|unit| {
                FFPoly::monomial(1, FFieldUnit::one(), "x") - FFPoly::monomial(0, unit.clone(), "x")
            })
            .collect();
        assert_eq!(monos.len(), 3);

        let pd = prod(&monos, "x");
        let expected = vec![
            FFieldUnit::new(0),
            FFieldUnit::new(2),
            FFieldUnit::new(-3),
            FFieldUnit::new(1),
        ];
        assert_eq!(pd.coeffs.len(), 4);
        assert_eq!(pd.coeffs, expected);
    }

    #[test]
    fn test_euclids_poly_div() {
        let var = "x";
        let poly1 = FFPoly::new(
            vec![FFieldUnit::new(2), FFieldUnit::new(3), FFieldUnit::new(1)],
            var,
        );
        let poly2 = FFPoly::new(vec![FFieldUnit::new(1), FFieldUnit::new(1)], var);

        let (q, r) = FFPoly::div_euclid_recursive(&poly1, &poly2);

        let xpected_r = FFPoly::new(vec![FFieldUnit::new(0)], var);
        let xpected_q = FFPoly::new(vec![FFieldUnit::new(2), FFieldUnit::new(1)], var);

        assert_eq!(q, xpected_q);
        assert_eq!(r, xpected_r);
    }

    #[test]
    fn test_division_of_polynomials() {
        let v = "x";
        let polys = get_instance_in_range(0, 10);
        assert_eq!(polys.coeffs.len(), 10);
        let f1 = vec![
            FFieldUnit::new(-24),
            FFieldUnit::new(10),
            FFieldUnit::new(6),
        ];
        // assert!(false, "{:?}", f1);
        let f2 = vec![FFieldUnit::new(6), FFieldUnit::new(2)];

        let p1 = FFPoly::new(f1, v);
        let p2 = FFPoly::new(f2, v);
        // let polys = calc_langrange_polys(&ffunits, "x");
        let div = p1 / p2;
        assert_eq!(div.coeffs, vec![FFieldUnit::new(-4), FFieldUnit::new(3)]);

        let f1 = vec![FFieldUnit::new(1), FFieldUnit::new(2), FFieldUnit::new(3)];
        // assert!(false, "{:?}", f1);
        let f2 = vec![FFieldUnit::new(1), FFieldUnit::new(2), FFieldUnit::new(3)];

        let p1 = FFPoly::new(f1, v);
        let p2 = FFPoly::new(f2, v);
        // let polys = calc_langrange_polys(&ffunits, "x");
        let div = p1 / p2;
        assert_eq!(div.coeffs, vec![FFieldUnit::new(1)]);

        // check failure of division; when rem is non-zero polynomial
        // let f1 = vec![FFieldUnit::new(10), FFieldUnit::new(2), FFieldUnit::new(3)];
        // // assert!(false, "{:?}", f1);
        // let f2 = vec![FFieldUnit::new(1), FFieldUnit::new(2), FFieldUnit::new(3)];
        //
        // let p1 = FFPoly::new(f1, v);
        // let p2 = FFPoly::new(f2, v);
        // // let polys = calc_langrange_polys(&ffunits, "x");
        // let div = p1 / p2;
        // assert_eq!(div.coeffs, vec![FFieldUnit::new(1)]);
    }

    #[test]
    fn test_calculation_of_langranges_polynomials() {
        let ffunits = get_ffunits_in_range(2, 5);
        let lng_poly = calc_langrange_polys(&ffunits, "x");
        //assert_eq!(0, 1, "{:?}", lng_poly);
    }

    // #[test]
    // fn test_prod() {
    //     let ffunits = get_instance_in_range(2, 5);
    //     let prd = prod(&ffunits, "x");
    //     assert_eq!(prd, FFieldUnit::new(24));
    // }

    #[test]
    fn test_prod_ffunits() {
        let ffunits = get_ffunits_in_range(2, 5);
        let prd = prod_ffunits(&ffunits);
        assert_eq!(prd, FFieldUnit::new(24));
    }
}
