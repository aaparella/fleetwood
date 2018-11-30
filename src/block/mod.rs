extern crate crypto;

use self::crypto::digest::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Block {
    index: usize,
    previous_hash: String,
    timestamp: u128,
    data: String,
    nonce: i64,
}

impl Block {
    fn new(index: usize, prev: String, now: u128, data: String, nonce: i64) -> Block {
        Block {
            index: index,
            previous_hash: prev,
            timestamp: now,
            data: data,
            nonce: nonce,
        }
    }

    fn new_genesis(time: u128, data: String) -> Block {
        Self::new(0, String::from(""), time, data, 0)
    }

    fn with_nonce(mut self, nonce: i64) -> Block {
        self.nonce = nonce;
        self
    }

    fn hash(&self) -> String {
        let mut hasher = crypto::sha2::Sha256::new();
        hasher.input_str(&format!("{}{}", self.index, self.timestamp));
        hasher.input_str(&format!("{}", self.nonce));
        hasher.input_str(&self.previous_hash);
        hasher.input_str(&self.data);
        hasher.result_str()
    }
}

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    // We'll need to extract this out to allow for the difficulty to change over
    // time dynamically, but for now because this will literally never see the
    // light of day, who cares?
    const PREFIX: &'static str = "00";

    pub fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }

    pub fn add_block(&mut self, data: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is before the unix epoch?")
            .as_millis();
        let new_block = match self.blocks.len() {
            0 => Block::new_genesis(now, data),
            n => {
                let prev = self.blocks.get(n - 1).unwrap();
                self.mine(n, data, prev.hash(), now)
            }
        };
        self.blocks.push(new_block);
    }

    fn mine(&self, ind: usize, data: String, prev: String, now: u128) -> Block {
        let mut nonce = 0;
        let mut new = Block::new(ind, prev, now, data, nonce);
        while !new.hash().starts_with(Self::PREFIX) {
            nonce = nonce + 1;
            new = new.with_nonce(nonce);
        }
        new
    }

    // Checks that the current chain is actual valid, i.e all block hashes and
    // indexes are correct and in the proper order. In theory should only need
    // to be called once, when validating a chain coming from the outside
    // world.
    pub fn validate(&self) -> bool {
        if self.blocks.len() == 0 || self.blocks.len() == 1 {
            // Not correct but close enough
            true
        } else {
            if self.blocks[0].index != 0 {
                return false;
            }
            for ind in 1..self.blocks.len() {
                let prev = &self.blocks[ind - 1];
                let curr = &self.blocks[ind];

                if prev.hash() != curr.previous_hash
                    || curr.index != ind
                    || !curr.hash().starts_with(Self::PREFIX)
                {
                    return false;
                }
            }
            true
        }
    }
}

impl ::std::ops::Index<usize> for BlockChain {
    type Output = Block;

    fn index(&self, ind: usize) -> &Block {
        &self.blocks[ind]
    }
}
