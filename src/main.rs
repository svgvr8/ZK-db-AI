use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Hash function
fn hash(data: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

// Create a Merkle tree from a vector of data
fn create_merkle_tree(data: &[&str]) -> HashMap<String, u64> {
    let mut merkle_tree = HashMap::new();
    let mut current_level = Vec::new();

    for item in data {
        let hash_value = hash(item);
        let key = format!("L1-{}", item);
        merkle_tree.insert(key, hash_value);
        current_level.push(hash_value);
    }

    let mut level = 2;
    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        for i in (0..current_level.len()).step_by(2) {
            let left = current_level[i];
            let right = if i + 1 < current_level.len() {
                current_level[i + 1]
            } else {
                left
            };

            let combined = format!("{}{}", left, right);
            let hash_value = hash(&combined);
            let key = format!("L{}-{}-{}", level, left, right);
            merkle_tree.insert(key, hash_value);
            next_level.push(hash_value);
        }

        current_level = next_level;
        level += 1;
    }

    merkle_tree
}

// Generate the proof
fn generate_proof(merkle_tree: &HashMap<String, u64>, data: &str) -> Vec<u64> {
    let mut proof = Vec::new();
    let mut current_hash = hash(data);
    let mut level = 1;

    while let Some(&parent_hash) = merkle_tree.values().find(|&&hash| hash == current_hash) {
        let left_key = format!("L{}-{}", level, current_hash);
        let right_key = format!("L{}-{}-{}", level - 1, current_hash, current_hash);

        if let Some(&sibling_hash) = merkle_tree.get(&left_key) {
            proof.push(sibling_hash);
        } else if let Some(&sibling_hash) = merkle_tree.get(&right_key) {
            proof.push(sibling_hash);
        }

        current_hash = parent_hash;
        level += 1;
    }

    proof
}

// Verify the proof
fn verify_proof(proof: &[u64], root_hash: u64, data: &str) -> bool {
    let mut current_hash = hash(data);

    for sibling_hash in proof {
        let combined = format!("{}{}", current_hash, sibling_hash);
        current_hash = hash(&combined);
    }

    current_hash == root_hash
}

fn main() {
    let data = vec!["A", "B", "C", "D"];
    let merkle_tree = create_merkle_tree(&data);
    let root_hash = merkle_tree.values().next().unwrap().clone();

    // Generate proof for the data item "C"
    let proof = generate_proof(&merkle_tree, "C");

    // Verify the proof for the data item "C"
    let is_valid = verify_proof(&proof, root_hash, "C");
    println!("Proof is valid: {:?}", is_valid);
}
