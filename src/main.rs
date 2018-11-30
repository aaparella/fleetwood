#![feature(duration_as_u128)]

mod block;

fn main() {
    let mut chain = block::BlockChain::new();
    chain.add_block("Hello, World!".to_owned());
    chain.add_block("Hello, Connor!".to_owned());
    chain.add_block("Hello, Alex!".to_owned());
    chain.add_block("Hello, Friends!".to_owned());
    chain.add_block("Hello, Everybody!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    println!("{:#?}", chain);

    let genesis = &chain[0];
    println!("This is the genesis block: {:#?}", genesis);

    println!("Is it valid? {}", chain.validate());
}
