pub mod blockchain;
use crate::blockchain::Serialization;
use blockchain::{transaction::Transaction, Block, BlockChain, BlockSearch, BlockSearchResult};
use sha2::{Digest, Sha256};

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!("find given block: {:?}", block);
        }

        BlockSearchResult::FailOfEmptyBlocks => {
            println!("no block in the chain");
        }

        BlockSearchResult::FailOfIndex(idx) => {
            println!("fail to find block with index: {}", idx);
        }

        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!("not block hash given previous hash: {:?}", hash);
        }

        BlockSearchResult::FailOfBlockHash(hash) => {
            println!("not block has hash as :{:?}", hash);
        }

        BlockSearchResult::FailOfNonce(nonce) => {
            println!("not block has nonce with value: {}", nonce);
        }

        BlockSearchResult::FailOfTimeStamp(time_stamp) => {
            println!("not block has given time stamp: {}", time_stamp);
        }

        BlockSearchResult::FailOfTransaction(tx) => {
            println!("not block contains given trasaction: {:?}", tx);
        }
    }
}

fn main() {
    let my_blockchain_addr = "my blockchain address";
    let mut block_chain = BlockChain::new(my_blockchain_addr.into());
    block_chain.print();

    block_chain.add_transaction(&Transaction::new("A".into(), "B".into(), 1));
    block_chain.mining();
    block_chain.print();

    block_chain.add_transaction(&Transaction::new("C".into(), "D".into(), 2));
    block_chain.add_transaction(&Transaction::new("X".into(), "Y".into(), 3));
    block_chain.mining();
    block_chain.print();

    println!(
        "value for miner: {}",
        block_chain.calculate_total_amount(my_blockchain_addr.to_string())
    );
    println!(
        "value for C: {}",
        block_chain.calculate_total_amount("C".to_string())
    );
    println!(
        "value for D: {}",
        block_chain.calculate_total_amount("D".to_string())
    );
}
