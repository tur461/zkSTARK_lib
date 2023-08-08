// use num::integer::Roots;
// use polynomial::Polynomial as Poly;
// use std::mem::replace;
use crate::FFPoly::FFPoly;
use std::ops::{Add, Div, Mul, Sub};
// use std::time::Instant;

// pub fn to_u512(n: &str) -> i128 {
//     i128::from_dec_str(n).unwrap()
// }
//
// pub fn u32_to_u512(n: u32) -> i128 {
//     to_u512(format!("{}", n).as_str())
// }
//
// pub fn usize_u512(n: usize) -> i128 {
//     to_u512(format!("{}", n).as_str())
// }

#[derive(Debug, Copy, Clone)]
pub struct FFieldUnit(pub i128);

impl FFieldUnit {
    pub fn new(n: i128) -> Self {
        //println!("new FF: {:?}", n);
        Self(n.rem_euclid(Self::modulo_prime()))
    }

    pub fn one() -> Self {
        Self(1)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn inner(&self) -> i128 {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.clone() == Self::zero()
    }

    pub fn generator_value() -> i128 {
        5i128
    }

    pub fn ffgenerator() -> Self {
        Self::new(Self::generator_value())
    }

    pub fn generator() -> Self {
        Self::pow(Self::new(Self::generator_value()), 3 * 2_u32.pow(20))
    }

    pub fn modulo_prime() -> i128 {
        3 * 2_i128.pow(30) + 1
    }

    pub fn pow(n: Self, e: u32) -> Self {
        let mut x = n;
        let mut p = e;
        let mut tot = Self::new(1i128);
        while p != 0 {
            if p % 2 == 1 {
                tot = tot * x;
            }
            p /= 2;
            x = x * x;
        }
        tot
    }

    pub fn eq(r: Self, l: Self) -> bool {
        r.0 == l.0
    }

    pub fn neg(&self) -> Self {
        Self::zero() - self.clone()
    }

    pub fn inverse(&self) -> Self {
        let mut t = 0;
        let mut new_t = 1;
        let mut m = Self::modulo_prime();
        let mut v = self.0;
        while v != 0 {
            let q = m / v;
            (t, new_t) = (new_t, (t - (q * new_t)));
            (m, v) = (v, m - q * v)
        }
        assert_eq!(m, 1);
        Self::new(t)
    }
}

impl From<i128> for FFieldUnit {
    fn from(n: i128) -> Self {
        Self::new(n)
    }
}

impl Add<i128> for FFieldUnit {
    type Output = Self;
    fn add(self, other: i128) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) + (other.rem_euclid(m)))
    }
}

impl Add for FFieldUnit {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) + (other.0.rem_euclid(m)))
    }
}

impl Add<&FFieldUnit> for FFieldUnit {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) + (other.0.rem_euclid(m)))
    }
}

impl Sub for FFieldUnit {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) - (other.0.rem_euclid(m)))
    }
}

impl Sub<i128> for FFieldUnit {
    type Output = Self;
    fn sub(self, other: i128) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) - (other.rem_euclid(m)))
    }
}

impl Mul<i128> for FFieldUnit {
    type Output = Self;
    fn mul(self, other: i128) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) * (other.rem_euclid(m)))
    }
}

impl Mul for FFieldUnit {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) * (other.0.rem_euclid(m)))
    }
}

impl Mul<&FFieldUnit> for FFieldUnit {
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        let m = Self::modulo_prime();
        Self::new((self.0.rem_euclid(m)) * (other.0.rem_euclid(m)))
    }
}

impl Div for FFieldUnit {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

impl Div<i128> for FFieldUnit {
    type Output = Self;

    fn div(self, other: i128) -> Self {
        self * Self::new(other).inverse()
    }
}

impl PartialEq<u32> for FFieldUnit {
    fn eq(&self, other: &u32) -> bool {
        self.0 == other.to_owned().into()
    }
}

impl PartialEq<FFieldUnit> for FFieldUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ToString for FFieldUnit {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_an_instance() {
        let ffu = FFieldUnit::new(1);
        assert_eq!(ffu.0, 1);
    }

    #[test]
    fn generator_works() {
        let g_v = FFieldUnit::generator_value();
        assert_eq!(g_v, 5_i128);
        let g = FFieldUnit::generator();
        assert_eq!(g.0, 1855261384_i128);
    }

    #[test]
    fn pow_works() {
        let pow = FFieldUnit::pow(FFieldUnit::new(13), 17);
        assert_eq!(
            pow.0,
            13_i128.pow(17).rem_euclid(FFieldUnit::modulo_prime())
        );
    }

    #[test]
    fn partial_equality_works() {
        let f1 = FFieldUnit::new(1);
        let f2 = FFieldUnit::new(1);
        let r = f1 == f2;

        assert!(r);

        let f1 = FFieldUnit::new(0);
        let f2 = FFieldUnit::new(0);
        let r = f1 == f2;

        assert!(r);
    }

    #[test]
    fn addition_of_similar_types_works() {
        let f1 = FFieldUnit::new(1);
        let f2 = FFieldUnit::new(1);

        assert_eq!(f1 + f2, FFieldUnit::new(2));
    }

    #[test]
    fn addition_of_disimilar_types_works() {
        let f1 = FFieldUnit::new(1);
        let f2 = 1;

        assert_eq!(f1 + f2, FFieldUnit::new(2));
    }

    #[test]
    fn diff_of_similar_types_works() {
        let f1 = FFieldUnit::new(2);
        let f2 = FFieldUnit::new(1);

        assert_eq!(f1 - f2, FFieldUnit::new(1));
    }

    #[test]
    fn diff_of_disimilar_types_works() {
        let f1 = FFieldUnit::new(2);
        let f2 = 1;

        assert_eq!(f1 - f2, FFieldUnit::new(1));
    }

    #[test]
    fn product_of_similar_types_works() {
        let f1 = FFieldUnit::new(2);
        let f2 = FFieldUnit::new(2);

        assert_eq!(f1 * f2, FFieldUnit::new(4));
    }

    #[test]
    fn product_of_disimilar_types_works() {
        let f1 = FFieldUnit::new(2);
        let f2 = 2;

        assert_eq!(f1 * f2, FFieldUnit::new(4));
    }

    #[test]
    fn div_of_similar_types_works() {
        let f1 = FFieldUnit::new(4);
        let f2 = FFieldUnit::new(2);
        let m = FFieldUnit::modulo_prime();

        assert_eq!(f1 / f2, FFieldUnit::new(2));
    }

    #[test]
    fn div_of_disimilar_types_works() {
        let f1 = FFieldUnit::new(4);
        let f2 = 2;

        assert_eq!(f1 / f2, FFieldUnit::new(2));
    }
}
