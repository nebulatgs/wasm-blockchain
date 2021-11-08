// sample code below was taken from https://github.com/rustwasm/wasm-bindgen

extern crate blake3;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use wasm_bindgen::prelude::*;
// JS Bindings
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> u32;
}

//Structs
#[derive(Serialize, Deserialize, Hash, Debug, Clone)]
pub struct Block {
    // pub hash: String,
    pub previous_hash: String,
    pub timestamp: u32,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(data: String, previous_hash: String) -> Block {
        let mut block = Block {
            // hash: String::new(),
            previous_hash,
            timestamp: now(),
            data,
            nonce: 0,
        };
        block.mine();
        block
    }

    pub fn mine(&mut self) {
        loop {
            let hash = self.calculate_hash();
            // log(&hash);
            if hash.starts_with("0000") {
                break;
            }
            self.nonce += 1;
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut s = blake3::Hasher::new();
        s.update_rayon(serde_json::to_string(self).unwrap().as_bytes());
        s.finalize().to_hex().to_string()
    }
}
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            let previous_hash = last.unwrap().calculate_hash();
            let block = Block::new(data, previous_hash);
            self.chain.push(block);
            self.chain.last().unwrap()
        }
    }
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    pub fn get_length(&self) -> usize {
        self.chain.len()
    }

    pub fn verify(&self) -> bool {
        let mut previous_hash = String::new();
        for block in self.chain.iter() {
            let hash = block.calculate_hash();
            if !hash.starts_with("0000") {
                return false;
            }
            if previous_hash != block.previous_hash {
                return false;
            }
            previous_hash = hash;
        }
        true
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Chainholder {
    chains: Vec<Blockchain>,
    active_chain: usize,
}

impl Chainholder {
    pub fn new(chains: Vec<Blockchain>) -> Chainholder {
        let mut holder = Chainholder {
            chains,
            active_chain: 0,
        };
        holder.calculate_active_chain();
        holder
    }

    pub fn get_active(&mut self) -> &mut Blockchain {
        &mut self.chains[self.active_chain]
    }

    pub fn add_chain(&mut self, chain: Blockchain) -> &Blockchain {
        self.chains.push(chain);
        self.calculate_active_chain()
    }

    pub fn remove_chain(&mut self, i: usize) -> &Blockchain {
        self.chains.remove(i);
        self.calculate_active_chain()
    }

    pub fn calculate_active_chain(&mut self) -> &Blockchain {
        let mut i = 0;
        let mut max_length = 0;
        for chain in self.chains.iter() {
            if chain.get_length() > max_length {
                max_length = chain.get_length();
                i = self.chains.len() - 1;
            }
        }
        self.active_chain = i;
        self.get_active()
    }
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.

#[wasm_bindgen]
pub fn setup() -> Chainholder {
    Chainholder::new(vec![Blockchain::new()])
}

#[wasm_bindgen]
pub fn verify_chain_in_holder(holder: &mut Chainholder) -> bool {
    let chain = holder.get_active();
    chain.verify()
}

#[wasm_bindgen]
pub fn get_chain_from_holder(holder: &mut Chainholder) -> String {
    let chain = holder.get_active();
    serde_json::to_string(chain).unwrap()
}

#[wasm_bindgen]
pub fn add_block_to_holder(holder: &mut Chainholder, data: &str) {
    let chain = holder.get_active();
    log(data);
    let block: Block = serde_json::from_str(data).unwrap();
    let mut local_chain = chain.clone();
    local_chain.add_block(block.clone());
    if local_chain.verify() {
        *chain = local_chain;
        log(format!(
            "Accepted remote block: {:?}\n New chain is {:?}",
            block, chain
        )
        .as_str());
    } else {
        log(format!(
            "Rejected remote block: {:?}\n Current chain is {:?}",
            block, chain
        )
        .as_str());
    }
    // let then = now();
    // let block = chain.create_block(data.to_string());
    // log(&format!(
    //     "Added block {:?}\n mined in {} ms",
    //     block,
    //     now() - then
    // ));
    // format!("{:?}", block)
}

#[wasm_bindgen]
pub fn add_chain_to_holder(holder: &mut Chainholder, data: &str) {
    let chain: Blockchain = serde_json::from_str(data).unwrap();
    if !&chain.verify() {
        log("Rejected remote chain");
        return;
    }
    holder.add_chain(chain);
    log("Accepted remote chain");
}

#[wasm_bindgen]
pub fn submit_block_to_holder(holder: &mut Chainholder, data: &str) -> String {
    let chain = holder.get_active();
    let then = now();
    let block = chain.create_block(data.to_string());
    log(&format!(
        "Added block {}\n mined in {} ms",
        serde_json::to_string(block).unwrap(),
        now() - then
    ));
    serde_json::to_string(block).unwrap()
}
