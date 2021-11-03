// sample code below was taken from https://github.com/rustwasm/wasm-bindgen

extern crate wasm_bindgen;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use wasm_bindgen::prelude::*;
// JS Bindings
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> u32;
}

//Structs
#[derive(Hash, Debug)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u32,
    pub data: String,
    pub nonce: u64,
}

pub fn get_timestamp_in_millis() -> u128 {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    since_the_epoch.as_millis()
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Block {
        let mut block = Block {
            hash: String::new(),
            previous_hash,
            timestamp: now(),
            data,
            nonce: 0,
        };
        block.mine();
        block
    }

    pub fn mine(&mut self) {
        let mut nonce = 0;
        loop {
            self.hash = self.calculate_hash();
            if self.hash.ends_with("000000") {
                break;
            }
            nonce += 1;
        }
        self.nonce = nonce;
    }

    pub fn calculate_hash(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish().to_string()
    }
}
#[wasm_bindgen]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { chain: Vec::new() }
    }
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
    pub fn create_block(&mut self, data: String) -> &Block {
        let last = self.chain.last();
        if last.is_none() {
            let block = Block::new(data, String::new());
            self.chain.push(block);
            self.chain.last().unwrap()
        } else {
            let previous_hash = last.unwrap().hash.clone();
            let block = Block::new(data, previous_hash);
            self.chain.push(block);
            self.chain.last().unwrap()
        }
    }
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}
// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.

#[wasm_bindgen]
pub fn setup() -> Blockchain {
    Blockchain::new()
}

#[wasm_bindgen]
pub fn add_block(chain: &mut Blockchain, data: &str) -> String {
    let then = now();
    let block = chain.create_block(data.to_string());
    log(&format!(
        "Added block {:?}\n mined in {} ms",
        block,
        now() - then
    ));
    format!("{:?}", block)
    // alert(&format!("Added block {:?}", block));
}
