extern crate crypto_hash;
extern crate serde_json;
extern crate chrono;use crypto_hash::{hex_digest, Algorithm};
use std::io;
use chrono::prelude::*;#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: i64,
    pub transaction_details: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Block {
    
    pub byte4_prefix: u128,
    pub byte64_encoded_string: String,

}

pub const PREFIX: &str = "cafe";

impl Block {
    pub fn genesis() -> Self {
        println!("Please Enter 64 Byte Encoded String");
        let mut guess= String ::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let transaction = Transaction {
            transaction_id: String::from("1"),
            transaction_details: String::from("This is dummy transaction as genesis block has no transactions"),
            transaction_timestamp: Utc::now().timestamp(),
        };
        Block {
            
            byte4_prefix: 0,
            byte64_encoded_string: guess,
        }
    }
    
    pub fn serialize_block(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    
    pub fn generate_hash(block: &Block) -> String {
        hex_digest(Algorithm::SHA256, block.serialize_block().as_bytes())
    }
    
    pub fn is_block_valid(hash: &str, prefix: &str) -> bool {
    if hash.ends_with(prefix){
        println!("{:#?}", hash);
    }
        hash.ends_with(prefix)
    
    }
    
    pub fn new(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            
            byte4_prefix: 0xAAAAAAAA,
            byte64_encoded_string: Self::generate_hash(previous_block),
        }
    }
    
    pub fn mine_new_block(block_candidate: &mut Block, prefix: &str) {
        while !
        Self::is_block_valid(&Self::generate_hash(block_candidate), prefix) {
           // println!("{}", block_candidate.byte4_prefix);
           //println!("{:#?}", block_candidate);
            block_candidate.byte4_prefix -= 1
        }
    }
}