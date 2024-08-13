pub mod blockchain;
use blockchain::BlockChain;

fn main() {
    //as_bytes convert string to bytes array
    // let b = Block::new(0, "this is our first block!".to_string().into_bytes());
    // b.print();

    // println!("the first block is : {:?}", b);
    /*
    need mutable since we will change its state by calling create_block
    */

    let mut block_chain = BlockChain::new();
    println!("block chain: {:?}", block_chain);
    block_chain.print();

    block_chain.create_block(1, "hash 1".to_string().into_bytes());
    block_chain.print();
    block_chain.create_block(2, "hash 2".to_string().into_bytes());
    block_chain.print();
}
