use crate::ffield_unit::FFieldUnit;
use crate::utils::hash256_str;

pub fn serialize(units: &[FFieldUnit]) -> String {
    units
        .iter()
        .map(|unit| unit.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// a channel can be used by a prover or a verifier to preseerve the semantics of an
/// interactive proof system, while under the hood its infact non-interactive, and
/// uses Sha256 to generate randomness when this is required.
/// It allows writting string-form data to it, and reading either random integers of
/// random FFieldUnit from it.

#[derive(Clone, Debug)]
pub struct Channel {
    proof: String,
    state: String,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            proof: String::new(),
            state: String::from("0"),
        }
    }

    pub fn send(&mut self, s: &str) {
        let hash = hash256_str((self.state.clone() + s).as_bytes());
        self.state = String::from(&hash[..(hash.len() / 2) - 1]);
        self.proof.push_str(s);
    }

    pub fn receive_rnd_int(&mut self, min: &FFieldUnit, max: &i128, show_in_proof: bool) -> i128 {
        let min = min.inner();
        // let max = max.inner();
        dbg!(&self.state);
        let num = min + (i128::from_str_radix(&self.state, 16).unwrap() % (max - min + 1));
        let hash = hash256_str(&self.state.as_bytes());
        self.state = String::from(&hash[..(hash.len() / 2) - 1]);
        if show_in_proof {
            self.proof.push_str(&format!("{}", num));
        }
        num
    }

    pub fn receive_rnd_ffunit(&mut self) -> FFieldUnit {
        let num = self.receive_rnd_int(&FFieldUnit::zero(), &FFieldUnit::modulo_prime(), false);
        self.proof.push_str(&format!("{:?}", num));
        return FFieldUnit::new(num);
    }
}
