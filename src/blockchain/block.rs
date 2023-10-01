use sha2::{Sha256, Digest};

pub struct Block {
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub diff: u32,
    pub hash: String
}

impl Block {
    fn sha256_hash(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn calculate(&mut self) -> &String{
        let mut hash;
        loop {
            let hash_it = format!(
                "{}{}{}{}",
                self.timestamp, self.data, self.previous_hash, self.diff
            );
            hash    = Block::sha256_hash(&hash_it);
            if &hash[0..3] == "000"{
                break
            }
            self.diff += 1;
        }
        
        self.hash = hash;
        &self.hash
    }
}
