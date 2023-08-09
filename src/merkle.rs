use crate::{ffield_unit::FFieldUnit, utils::hash256_str};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Node {
    leaf_data: Option<String>,
    children: Option<(String, String)>,
}

impl Node {
    fn new(ld: Option<String>, ch: Option<(String, String)>) -> Self {
        Self {
            children: ch,
            leaf_data: ld,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: String,
    height: usize,
    num_of_leaves: usize,
    data: Vec<FFieldUnit>,
    facts: HashMap<String, Node>,
}

impl MerkleTree {
    pub fn new(data: &[FFieldUnit]) -> Self {
        let len = data.len();
        let num_of_leaves = 2_usize.pow((len as f32).log2().ceil() as u32);
        let height = (num_of_leaves as f32).log2() as usize;

        // pad with zeroes
        let mut new_data = Vec::<FFieldUnit>::from(data);
        new_data.extend(std::iter::repeat(FFieldUnit::zero()).take(num_of_leaves - len));

        Self {
            height,
            num_of_leaves,
            data: new_data,
            root: String::new(),
            facts: HashMap::new(),
        }
    }

    pub fn root(&self) -> String {
        self.root.clone()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn num_of_leaves(&self) -> usize {
        self.num_of_leaves
    }

    pub fn facts(&self) -> HashMap<String, Node> {
        self.facts.clone()
    }

    pub fn build_tree(&mut self) {
        self.root = self.recursive_build_tree(1)
    }

    pub fn recursive_build_tree(&mut self, node_id: usize) -> String {
        let len = self.data.len();

        if node_id >= len {
            // a leaf
            let id_in_data = node_id - len;
            let leaf_data = self.data[id_in_data].to_string();
            let h = hash256_str(&leaf_data.as_bytes());
            self.facts
                .insert(h.clone(), Node::new(Some(leaf_data), None));
            return h;
        }
        // inner node
        let left = self.recursive_build_tree(node_id * 2);
        let right = self.recursive_build_tree(node_id * 2 + 1);

        let h = hash256_str(&(left.clone() + &right).as_bytes());
        self.facts
            .insert(h.clone(), Node::new(None, Some((left, right))));
        return h;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_ffunits_in_range;

    #[test]
    fn test_creates_instance() {
        let expected_hashes = vec![
            "97f570aecc5b1bd2597915fdf15120f4980f217bde6f723bafb42beea63822a0",
            "cb2fb90b4eabe8cca9b2016ec74afe88524771d04177936bc593823b468a9101",
            "6910135541ac31846c09079af34f5284c65f0fd755443cc4ad54aa83b2a59a13",
            "6e5a2a7de3d21d29e30e87c4cc4c78eeddcac830dac92bbc8e33e6cb8d0a960f",
            "350e965a4139fbade0c154ac20877e026e7e6cc435f029f1aee544f0aa306fc8",
            "b3c81b753d787a3d5363671a00d9066ca73a534beaccd97cbec37209631d85f1",
            "3131c12fdfbae13db9e47b5bdf0d46a7f9b92e5e65cc6dae2bde1fcc6452d980",
            "547b0f18fd5a8b030711435c82f5b2fbe7058a2ec5232362785dc8bd9e370569",
            "7e9aaf25d82f08ae1b9f73a023fb1964615fccd70e87f92c96a3b07ff70a791d",
            "3ae3c7b1e8a3d8619bf179da070f5ed9270ac7191ff61a04285981e24225b3db",
        ];
        for i in 0..10 {
            let units = get_ffunits_in_range(0 + i, 8192 + i);
            let mut merkle = MerkleTree::new(&units);
            merkle.build_tree();
            assert_eq!(merkle.facts().len(), 16383);
            assert_eq!(merkle.height(), 13);
            assert_eq!(merkle.num_of_leaves(), 8192);
            assert_eq!(expected_hashes[i as usize], merkle.root);
        }
    }
}
