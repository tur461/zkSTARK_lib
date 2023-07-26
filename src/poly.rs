use crate::ffield_unit::FFieldUnit;

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub coeffs: Vec<i128>,
    pub points: Vec<(i128, i128)>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<i128>) -> Self {
        Self {
            coeffs,
            points: Vec::new(),
        }
    }

    fn is_empty_or_zero(&self) -> bool {
        self.coeffs.len() == 0 || self.coeffs.iter().all(|x| x == &0_i128)
    }

    pub fn display(&self) -> String {
        if self.is_empty_or_zero() {
            return "0".to_string();
        }

        let x = "x";
        let one = 1_i128;
        let mut s = Vec::new();
        for (i, n) in self.coeffs.clone().into_iter().enumerate() {
            // output n*x^i / -n*x^i
            if n == 0_i128 {
                continue;
            }

            let term = if i == 0 {
                n.to_string()
            } else if i == 1 {
                if n == one {
                    x.to_string()
                } else if n == -one {
                    format!("-{}", x)
                } else {
                    format!("{}*{}", n, x)
                }
            } else if n == one {
                format!("{}^{}", x, i)
            } else if n == -one {
                format!("-{}^{}", x, i)
            } else {
                format!("{}*{}^{}", n, x, i)
            };

            if !s.is_empty() && n > 0_i128 {
                s.push("+".to_string());
            }
            s.push(term);
        }

        s.concat()
    }

    pub fn interpolate(x: i128) {}
}
