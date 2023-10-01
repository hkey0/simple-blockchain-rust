extern crate chrono;
pub mod block;

use block::Block;
use chrono::Utc;

pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Blockchain {
    pub fn create_genesis(&mut self){
        let now = Utc::now();
        let mut my_b = Block{
            timestamp: now.timestamp() as u64,
            data: String::from("Genesis data"),
            previous_hash: String::new(),
            diff: 1,
            hash: String::new()
        };
        my_b.calculate();
        self.chain.push(my_b);
    }


    pub fn add_block(&mut self, mydata: &String){
        if self.check_block() == false {
            panic!("CHAIN BROKEN!");
        } else {

            let last_previous_hash = self.chain.last().map(|b| b.hash.clone()).unwrap_or_default();
            let now = Utc::now();
            let mut node = Block{
                timestamp: now.timestamp() as u64,
                data: mydata.clone(),
                previous_hash: last_previous_hash.clone(),
                diff: 1,
                hash: String::new()
            };
            node.calculate();
            self.chain.push(node);
        };

    }

    pub fn check_block(&self) -> bool{
        for i in 0..=self.chain.len()-1{
            if i != 0 {
                let previous = &self.chain[i-1].hash;
                let current  = &self.chain[i].previous_hash;
                if previous != current {
                    return false;
                }

            };
        };
        return true;
    } 

    pub fn list_blocks(&self){
        for i in 0..=self.chain.len()-1{
            let data = format!(
                "Block: {}\nHash: {}\nPrevious Hash: {}\nTimestamp: {}\nData: {}\nTries: {}\n\n\n",
                i,
                self.chain[i].hash,
                self.chain[i].previous_hash,
                self.chain[i].timestamp,
                self.chain[i].data,
                self.chain[i].diff
            );
            println!("{}", data);
        }
    }
}