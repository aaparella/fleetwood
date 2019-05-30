extern crate crypto;
extern crate derive_new;

use self::crypto::digest::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, new)]
pub struct Block {
    index: usize,
    previous_hash: String,
    timestamp: u128,
    data: String,
    nonce: i64,
    pub difficulty: usize,
}

impl Block {
    fn new_genesis(time: u128, data: String, difficulty: usize) -> Block {
        Self::new(0, String::from(""), time, data, 0, difficulty)
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
    pub fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }

    pub fn add_block(&mut self, data: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is before the unix epoch?")
            .as_millis();
        let new_block = match self.blocks.len() {
            0 => Block::new_genesis(now, data, 2),
            n => {
                let prev = self.blocks.get(n - 1).unwrap();
                let difficulty = self.current_difficulty();
                self.mine(n, data, prev.hash(), now, difficulty)
            }
        };
        self.blocks.push(new_block);
    }

    fn mine(&self, ind: usize, data: String, prev: String, now: u128, difficulty: usize) -> Block {
        let mut nonce = 0;
        let mut new = Block::new(ind, prev, now, data, nonce, difficulty);
        let prefix = "0".repeat(difficulty);
        while !new.hash().starts_with(&prefix) {
            nonce = nonce + 1;
            new = new.with_nonce(nonce);
        }
        new
    }

    fn current_difficulty(&self) -> usize {
        let last_block = &self.blocks[self.blocks.len() - 1];
        // We only want to update the difficulty every ten blocks, as opposed
        // to every single block.
        if last_block.index % 10 == 0 && last_block.index != 0 {
            let last_adjustment_block = &self.blocks[last_block.index - 10];
            let time_passed = last_block.timestamp - last_adjustment_block.timestamp;
            let time_expected = 50;
            if time_passed < (time_expected / 2) {
                last_block.difficulty + 1
            } else if time_passed > time_expected * 2 {
                last_block.difficulty - 1
            } else {
                last_block.difficulty
            }
        } else {
            last_block.difficulty
        }
    }

    pub fn difficulty(&self) -> u64 {
        self.blocks
            .iter()
            .fold(0, |a, b| a + ((b.difficulty * b.difficulty) as u64))
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

                let prefix = "0".repeat(curr.difficulty);

                if prev.hash() != curr.previous_hash
                    || curr.index != ind
                    || !curr.hash().starts_with(&prefix)
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
