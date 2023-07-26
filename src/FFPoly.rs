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

    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
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
        while let Some(&l) = v.last() {
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

    pub fn qdiv(&self, other: &Self) -> (Self, Self) {
        let poly1 = self.trim_trailing_zeroes();
        let poly2 = other.trim_trailing_zeroes();

        if poly1.coeffs.is_empty() {
            return (Self::new(vec![], &self.var), Self::new(vec![], &self.var));
        }
        let mut rem = poly1.clone();
        let mut deg_diff = poly1.degree() - poly2.degree();
        let mut quotient: Vec<FFieldUnit> = [FFieldUnit::zero()]
            .iter()
            .map(|x| x.clone() * (deg_diff as i128 + 1_i128))
            .collect();
        let g_msc_inv = poly2.coeffs.last().unwrap().inverse();

        while deg_diff >= 0 {
            let tmp = g_msc_inv * rem.coeffs.last().unwrap();
            quotient[deg_diff] = quotient[deg_diff] + &tmp;
            let mut last_non_zero = deg_diff - 1;
            for (i, coef) in poly2.coeffs.iter().enumerate().rev() {
                rem.coeffs[i + deg_diff] = rem.coeffs[i + deg_diff] - (tmp * coef);
                if !rem.coeffs[i + deg_diff].is_zero() {
                    last_non_zero = i + deg_diff;
                }
            }
            rem = rem.trim_trailing_zeroes();
            deg_diff = last_non_zero - poly2.coeffs.len();
        }
        (Self::new(quotient, &self.var).trim_trailing_zeroes(), rem)
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

        let mut result = longer.coeffs.clone();
        for (i, coef) in shorter.coeffs.iter().enumerate() {
            result[i] = result[i].clone() - coef.clone();
        }
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
        // return only q
        self.qdiv(&other).0
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
        let mut denominator = FFieldUnit::one();
        for (i, x) in x_vals.iter().enumerate() {
            if i != j {
                denominator = denominator * (x_vals[j].clone() - x.clone());
            }
        }
        let poly = numerator.clone() / denominator;
        lang_polys.push(poly);
    }
    lang_polys
}

/// y_vals: &[FFieldUnit]
/// lang_polys: &[FFPoly]
///
pub fn interpolate_lang_poly(y_vals: &[FFieldUnit], lang_polys: &[FFPoly], var: &str) -> FFPoly {
    let mut poly = FFPoly::new(vec![FFieldUnit::zero()], var);
    for (j, y_val) in y_vals.iter().enumerate() {
        poly = poly + lang_polys[j].clone() * y_val.clone();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_instance() {
        let coeffs: Vec<FFieldUnit> = (1..=3).into_iter().map(|x| FFieldUnit::new(x)).collect();
        let fpoly = FFPoly::new(coeffs.clone(), "x");

        assert_eq!(fpoly.var, "x");
        assert_eq!(fpoly.coeffs.len(), 3);
        assert_eq!(fpoly.coeffs, coeffs);
        assert_eq!(
            fpoly.coeffs.into_iter().map(|x| x.0).collect::<Vec<i128>>(),
            [1, 2, 3]
        );
    }

    #[test]
    fn evals_a_poly_on_given_val() {
        let coeffs: Vec<FFieldUnit> = (1..=3).into_iter().map(|x| FFieldUnit::new(x)).collect();
        let fpoly = FFPoly::new(coeffs.clone(), "x");

        let res = fpoly.eval(&FFieldUnit::new(2));

        // poly: 3.x^2 + 2.x + 1
        // for x = 2,
        // poly = 17
        assert_eq!(res, FFieldUnit::new(17));
    }
}
