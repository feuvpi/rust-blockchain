mod merkle_tree;
use merkle_tree::MerkleTree;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Transaction {
    // Define your transaction structure here
}

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let (hash, nonce) =
            Block::hash_with_proof_of_work(index, timestamp, &transactions, &previous_hash);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn hash_with_proof_of_work(
        index: u32,
        timestamp: u64,
        transactions: &Vec<Transaction>,
        previous_hash: &String,
    ) -> (String, u64) {
        let mut nonce = 0;
        loop {
            let data = format!(
                "{}{}{:?}{}{}",
                index, timestamp, transactions, previous_hash, nonce
            );
            let hash = format!("{:x}", Sha256::digest(data.as_bytes()));

            if hash.starts_with("0000") {
                return (hash, nonce);
            }

            nonce += 1;
        }
    }
}
