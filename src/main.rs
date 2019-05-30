#[macro_use]
extern crate derive_new;

mod block;

fn main() {
    let mut chain = block::BlockChain::new();
    chain.add_block("Hello, World!".to_owned());
    chain.add_block("Hello, Connor!".to_owned());
    chain.add_block("Hello, Alex!".to_owned());
    chain.add_block("Hello, Friends!".to_owned());
    chain.add_block("Hello, Everybody!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());
    chain.add_block("Hello, Cool People!".to_owned());

    println!("Difficulty: {}", chain[0].difficulty);
    println!("Difficulty: {}", chain[1].difficulty);
    println!("Difficulty: {}", chain[2].difficulty);
    println!("Difficulty: {}", chain[3].difficulty);
    println!("Difficulty: {}", chain[4].difficulty);
    println!("Difficulty: {}", chain[5].difficulty);
    println!("Difficulty: {}", chain[6].difficulty);
    println!("Difficulty: {}", chain[7].difficulty);
    println!("Difficulty: {}", chain[8].difficulty);
    println!("Difficulty: {}", chain[9].difficulty);
    println!("Difficulty: {}", chain[10].difficulty);
    println!("Difficulty: {}", chain[11].difficulty);
    println!("Difficulty: {}", chain[12].difficulty);
    println!("Difficulty: {}", chain[13].difficulty);
    println!("Difficulty: {}", chain[14].difficulty);
    println!("Difficulty: {}", chain[15].difficulty);
    println!("Difficulty: {}", chain[16].difficulty);
    println!("Difficulty: {}", chain[17].difficulty);
    println!("Difficulty: {}", chain[18].difficulty);
    println!("Difficulty: {}", chain[19].difficulty);
    println!("Difficulty: {}", chain[20].difficulty);
    println!("Difficulty: {}", chain[21].difficulty);
    println!("Difficulty: {}", chain[22].difficulty);
    println!("Difficulty: {}", chain[23].difficulty);
    println!("Difficulty: {}", chain[24].difficulty);
    println!("Difficulty: {}", chain[25].difficulty);

    println!("Cumulative difficulty: {}", chain.difficulty());
    println!("Validation: {}", chain.validate());
}
