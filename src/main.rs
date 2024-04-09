use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};

// Define the block structure
#[derive(Debug)]
struct Block {
    prev_hash: String,
    data: String,
    timestamp: u64,
    nonce: u32,
    hash: String,
}

// Hash function
fn hash(block: &Block) -> String {
    let input = format!("{}{}{}{}", block.prev_hash, block.data, block.timestamp, block.nonce);
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

// Proof of Work algorithm
fn proof_of_work(mut block: Block, difficulty: u32) -> Block {
    let mut nonce = 0;
    loop {
        block.nonce = nonce;
        let hash = hash(&block);
        // Check if hash meets difficulty criteria
        if hash.starts_with(&"0".repeat(difficulty as usize)) {
            block.hash = hash;
            return block;
        }
        nonce += 1;
    }
}

// Mining function
fn mine_block(prev_hash: String, data: String, difficulty: u32) -> Block {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut block = Block {
        prev_hash,
        data,
        timestamp,
        nonce: 0,
        hash: String::new(),
    };
    block = proof_of_work(block, difficulty);
    block
}

// Verification
fn verify_block(block: &Block, difficulty: u32) -> bool {
    let hash = hash(block);
    hash.starts_with(&"0".repeat(difficulty as usize))
}

fn main() {
    // Test mining and verification
    let prev_hash = String::from("00000000000000000000000000000000");
    let data = String::from("Hello, world!");
    let difficulty = 3;

    let block = mine_block(prev_hash, data.clone(), difficulty);
    println!("Mined block: {:?}", block);
    println!("Is block valid? {}", verify_block(&block, difficulty));
}

