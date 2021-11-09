extern crate blake3;
extern crate js_sys;
extern crate rand;
extern crate schnorrkel;
extern crate serde;
extern crate serde_json;
extern crate vulkan;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate wgpu;

use js_sys::Date;
use rand::rngs::OsRng;
use schnorrkel::{signing_context, Keypair, PublicKey, Signature};
use serde::{Deserialize, Serialize};
use web_sys::window;
use vulkan::runner::ParamsBuilder;
use wasm_bindgen::prelude::*;
// JS Bindings
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//Structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateIdentity {
    pub name: String,
    pub keypair: Keypair,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicIdentity {
    pub name: String,
    pub public: PublicKey,
}
impl Into<PublicIdentity> for PrivateIdentity {
    fn into(self) -> PublicIdentity {
        PublicIdentity {
            public: self.keypair.public,
            name: self.name,
        }
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    // pub hash: String,
    previous_hash: String,
    timestamp: f64,
    msg: Message,
    nonce: u64,
}

impl Block {
    pub async fn new(msg: Message, previous_hash: String) -> Block {
        let mut block = Block {
            // hash: String::new(),
            previous_hash,
            timestamp: Date::now(),
            msg,
            nonce: 0,
        };
        block.mine_gpu().await;
        block
    }

    pub fn mine_cpu(&mut self) {
        loop {
            let hash = self.calculate_hash();
            // log(&hash);
            if hash.starts_with("0000") {
                break;
            }
            self.nonce += 1;
        }
    }

    async fn hash_vec(possiblities: Vec<String>) -> Option<usize> {
        let count= possiblities.len();
        let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) =
            possiblities.into_iter().map(|x| vulkan::runner::prepare_for_gpu(x)).unzip();

        let texts: Vec<u32> = texts.into_iter().flatten().collect();

        let hash = vec![0u32; count * 8];
        // assert_eq!(hash.len() * core::mem::size_of::<u32>() * 8, 8 * 32 * count);

        let mut device = vulkan::runner::Device::new(0).await;
        let text_gpu = device.to_device(texts.as_slice());
        let hash_gpu = device.to_device(hash.as_slice());
        let size_gpu = device.to_device(sizes.as_slice());

        // let shader = wgpu::include_spirv!("/home/coder/wasm-blockchain/wasm/vulkan/target/wasm32-unknown-unknown/spirv-builder/spirv-unknown-unknown/release/kernel.spv");
        let shader = wgpu::include_spirv!("/home/coder/wasm-blockchain/wasm/vulkan/target/spirv-builder/spirv-unknown-unknown/release/kernel.spv");

        let args = ParamsBuilder::new()
            .param(Some(&text_gpu))
            .param(Some(&hash_gpu))
            .param(Some(&size_gpu))
            .build(Some(0));

        let compute = device.compile("main_cs", &shader, &args.0).unwrap();

        device.call(compute, (count as u32, 1, 1), &args.1);

        let hash_res = device.get(&hash_gpu).await.unwrap();
        let hash_res = &hash_res[0..hash.len()];

        let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
        let chunks = result
            .as_bytes()
            .chunks(64)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        for (i, chunk) in chunks.iter().enumerate() {
            if chunk.starts_with("0000") {
                return Some(i);
            }
        }
        None
    }

    pub async fn mine_gpu(&mut self) {
        let mut clone = self.clone();
        let mut outer_i = 0;
        loop {
            let mut possiblities: Vec<String> = Vec::<String>::new();
            for _ in 0..8192 {
                possiblities.push(serde_json::to_string(&clone).unwrap());
                clone.nonce += 1;
            }
            match Block::hash_vec(possiblities).await {
                Some(i) => {
                    self.nonce = (outer_i + i) as u64;
                    return;
                }
                None => {
                    log(clone.nonce.to_string().as_str());
                }
            }
            outer_i = outer_i + 1;
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut s = blake3::Hasher::new();
        s.update(serde_json::to_string(self).unwrap().as_bytes());
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
    pub async fn create_block(this: Self, msg: Message) -> Block {
        let last = this.chain.last();
        if last.is_none() {
            Block::new(msg, String::new()).await
        } else {
            let previous_hash = last.unwrap().calculate_hash();
            Block::new(msg, previous_hash).await
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
            if !block.msg.verify_signature() {
                return false;
            }
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
    id: PrivateIdentity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    data: String,
    sender: PublicIdentity,
    signature: Signature,
}

impl Message {
    pub fn verify_signature(&self) -> bool {
        let context = signing_context(b"Verify message identity");
        match self
            .sender
            .public
            .verify(context.bytes(self.data.as_bytes()), &self.signature)
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Chainholder {
    pub fn new(chains: Vec<Blockchain>, id: PrivateIdentity) -> Chainholder {
        let mut holder = Chainholder {
            chains,
            active_chain: 0,
            id,
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
    pub fn sign_message(&self, data: String) -> Message {
        let context = signing_context(b"Verify message identity");
        let signature = self.id.keypair.sign(context.bytes(data.as_bytes()));
        Message {
            data,
            sender: self.id.clone().into(),
            signature,
        }
    }
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.

fn generate_identity(name: &str) -> PrivateIdentity {
    // Setup pair of keys, message, and signing context
    let keypair = Keypair::generate_with(OsRng);
    // let message = String::from("Hello, world!");
    // let context = signing_context(b"Verify message identity");

    // Signature generation
    // let signature = keypair.sign(context.bytes(message.as_bytes()));

    // // Signature verification
    // let public_key = keypair.public;
    // public_key
    //     .verify(context.bytes(message.as_bytes()), &signature)
    //     .expect("This program crashed due to signature mismatch");

    // // Console success output
    // println!("Signature verified");
    PrivateIdentity {
        name: name.to_string(),
        keypair,
    }
}

#[wasm_bindgen]
pub fn setup(name: &str) -> Chainholder {
    let storage = window()
        .expect("should have a window")
        .local_storage()
        .expect("should have local storage")
        .unwrap();
    let id = match storage
        .get_item("identity")
        .expect("error retrieving identity")
    {
        Some(id_str) => serde_json::from_str::<PrivateIdentity>(&id_str).unwrap(),
        None => {
            let identity = generate_identity(name);
            storage
                .set_item(
                    "identity",
                    serde_json::to_string(&identity)
                        .expect("unable to set identity")
                        .as_str(),
                )
                .unwrap();
            identity
        }
    };

    Chainholder::new(vec![Blockchain::new()], id)
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
pub async fn mine_block(mut holder: Chainholder, data: String) -> Block {
    let msg = holder.sign_message(data.to_string());
    let chain = holder.get_active();
    let then = Date::now();
    let block = Blockchain::create_block(chain.clone(), msg).await;
    let stringified_block = serde_json::to_string(&block).unwrap();
    log(&format!(
        "Mined block {}\nin {} ms",
        stringified_block,
        Date::now() - then
    ));
    block
}

#[wasm_bindgen]
pub fn submit_block_to_holder(holder: &mut Chainholder, block: &Block) -> String{
    let chain = holder.get_active();
    chain.add_block(block.clone());
    serde_json::to_string(&block).unwrap()
}
