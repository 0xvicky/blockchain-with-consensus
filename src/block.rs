use std::io::Error;

use super::*;
use chrono::Utc; //Used to get the current timestamp
use log::info;

pub struct Block {
    timestamp: u128,
    prev_hash: String,
    hash: String,
    nonce: i32,
    height: u32,
    transactions: String,
}

impl Block {
    fn new(prev_hash: String, transactions: String, height: u32) -> Result<Block, Error> {
        let timestamp = (Utc::now().timestamp()) as u128;
        let mut block = Block {
            timestamp,
            prev_hash,
            hash: String::new(),
            nonce: 0,
            height,
            transactions,
        };
        //Mining the block using PoW;
        block.mine_block();
        Ok(block)
    }

    fn mine_block(&mut self) {
        info!("Mining the block!!!");
        while !self.validate() {
            self.nonce += 1;
        }
    }
    
    fn hash_txns ()->Vec<u8>{
        let mut txns = Vec::new();
        
    }

    fn validate(&self) -> bool {
        let data = self.produce_hash_data();
    }

    fn produce_hash_data(&self) -> u8 {
        let content = (
         self.prev_hash.clone(),
         self.hash
        )
    }
}
