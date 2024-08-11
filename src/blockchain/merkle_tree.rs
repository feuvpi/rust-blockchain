use sha2::{Digest, Sha256};

pub mod merkle_tree {

    #[derive(Debug)]
    struct MerkleTree {
        root: Option<String>,
        nodes: Vec<String>,
    }

    impl MerkleTree {
        pub fn new(data: Vec<String>) -> MerkleTree {
            let mut nodes = data
                .iter()
                .map(|d| MerkleTree::hash_node(d))
                .collect::<Vec<_>>();

            while nodes.len() > 1 {
                let mut new_nodes = Vec::new();

                for i in (0..nodes.len()).step_by(2) {
                    let left = &nodes[i];
                    let right = if i + 1 < nodes.len() {
                        &nodes[i + 1]
                    } else {
                        left
                    };

                    let hash = MerkleTree::hash_node(&format!("{}{}", left, right));
                    new_nodes.push(hash);
                }

                nodes = new_nodes;
            }

            MerkleTree {
                root: nodes.first().cloned(),
                nodes,
            }
        }

        pub fn hash_node(data: &str) -> String {
            format!("{:x}", Sha256::digest(data.as_bytes()))
        }

        pub fn verify(&self, data: Vec<String>) -> bool {
            let root = MerkleTree::new(data).root;
            root == self.root
        }
    }
}
