use crate::ffield_unit::FFieldUnit;
use sha2::Sha256;
use std::collections::HashMap;

struct MerkleTree {
    root: Sha256,
    height: usize,
    num_of_leaves: usize,
    data: Vec<FFieldUnit>,
    facts: HashMap<String>,
}

impl MerkleTree {
    fn new(data: Vec<FFieldUnit>) -> Self {
        let len = data.len();
        let mut new_data = data.clone();
        let num_of_leaves = 2_usize.pow(len.log2().ceil());
        let height = num_of_leaves.log2() as usize;
        (0..(num_of_leaves - len))
            .iter()
            .for_each(|_| new_data.push(FFieldUnit::zero()));

        Self {
            height,
            num_of_leaves,
            data: new_data,
            root: Sha256::new(),
        }
    }

    pub fn build_tree(&self) {
        Self::recursive_build_tree(1)
    }

    pub fn recursive_build_tree(&self, node_id: usize) {
        let len = data.len();
        let mut hasher = Sha256::new();

        if node_id >= len {
            let id_in_data = node_id - len;
            let leaf_data = self.data[id_in_data].to_string();
            hasher.update(leaf_data.as_bytes());

            let h = hasher.finalize()[..];
            self.facts.insert(h, leaf_data);
        } else {
            let left = self.recursive_build_tree(node_id * 2);
            right = 
        }

    }
}
