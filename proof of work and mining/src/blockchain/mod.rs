use sha2::{Digest, Sha256};
use std::time::SystemTime;
use transaction::*;
pub mod transaction;

use std::cmp::PartialEq;
use std::ops::AddAssign;
use std::ops::Index;
use std::str;
use std::time::Instant;
pub trait Serialization<T> {
    fn serialization(&self) -> Vec<u8>;
    fn deserialization(bytes: Vec<u8>) -> T;
}

pub enum BlockSearch {
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByNonce(i32),
    SearchByTimeStamp(u128),
    SearchByTransaction(Vec<u8>),
}

pub enum BlockSearchResult<'a> {
    Success(&'a Block),
    FailOfEmptyBlocks,
    FailOfIndex(usize),
    FailOfPreviousHash(Vec<u8>),
    FailOfBlockHash(Vec<u8>),
    FailOfNonce(i32),
    FailOfTimeStamp(u128),
    FailOfTransaction(Vec<u8>),
}
/*
remember to use the pub keyword to make things here to be visible by outside
*/
//let compile do some job such as printing for us
#[derive(Debug)]
pub struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

impl AddAssign<i32> for Block {
    fn add_assign(&mut self, rhs: i32) {
        self.nonce += rhs;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        let self_hash = self.hash();
        let other_hash = other.hash();
        self_hash == other_hash
    }
}

impl Block {
    //static method, it dosen't need to access to the struct
    //Self is alias name of the given struct, notice it should Capitalize
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        //notice the parameter of previous_hash will lost ownership
        Block {
            nonce: nonce,
            previous_hash: previous_hash,
            time_stamp: duration_since_epoch.as_nanos(),
            /*
            If the type for vector is complex one such as another kind of
            vector, then we need to have :: between Vec and its type <Vec<u8>>
            */
            transactions: Vec::<Vec<u8>>::new(),
        } //don't add semicolon here, since we want to return the object
    }

    //struct method, it will receive the reference of the struct instance
    pub fn print(&self) {
        /*
        If we won't change any fields in the strint, we use inmutable reference,
        otherwise we need mutable reference &mut self
        */
        //format hex value
        println!("timestamp: {:x}", self.time_stamp);
        //format integer value
        println!("nonce:      {},", self.nonce);
        //let compiler decide how to print vector
        //actually it is called trait, go to it later
        println!("previous_hash:    {:?}", self.previous_hash);
        for (idx, tx) in self.transactions.iter().enumerate() {
            let transaction = Transaction::deserialization(tx.to_vec());
            println!("the {}'th transaction is: {}", idx, transaction);
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamp.to_be_bytes());
        for tx in self.transactions.iter() {
            bin.extend(tx.clone());
        }
        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }

    pub fn add_transaction(&mut self, transaction: &impl Serialization<Transaction>) {
        //check transaction is already add or not
        let bin = transaction.serialization();
        for (_, tx) in self.transactions.iter().enumerate() {
            if *tx == bin {
                return;
            }
        }

        self.transactions.push(bin);
    }
}

//ask the compiler to print the struct
#[derive(Debug)]
pub struct BlockChain {
    /*
    any transaction before going to the chain will wait on the transaction pool
    until they are minted, we will goto the detail at later
    */
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
    //the address for the miner
    blockchain_address: String,
}

impl Index<usize> for BlockChain {
    type Output = Block;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.chain[idx]
    }
}

impl BlockChain {
    const DIFFICULTY: usize = 3;
    const MINING_SENDER: &str = "THE BLOCKCHAIN";
    const MINING_REWOARD: u64 = 1;
    pub fn new(address: String) -> Self {
        /*
        we need to call methods that will change its state, and we need it
        to be mutable
        */
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            /*
            Type in Vec is complex one, then need ::
            */
            chain: Vec::<Block>::new(),
            blockchain_address: address,
        };

        let b = Block::new(0, vec![0 as u8; 32]);
        //Mine a new block after the genisis block
        bc.chain.push(b);
        bc.mining();

        bc //no semicolon for returning it
    }
    //we need to write to fields of the given struct
    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut b = Block::new(nonce, previous_hash);

        for tx in self.transaction_pool.iter() {
            b.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();

        //do proof of work
        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut b);
        let elapsed = now.elapsed();
        println!(
            "computing time: {:?},proof hash for current block is: {:?}",
            elapsed, proof_hash
        );

        //the chain own the block
        self.chain.push(b);
    }

    pub fn print(&self) {
        /*
        using iterator to loop over vector, it is complcate and powerful,
        we will go to it later
        */
        for (i, x) in self.chain.iter().enumerate() {
            println!("{} Chain {}  {}", "=".repeat(25), i, "=".repeat(25));
            x.print();
        }
        println!("{}", "*".repeat(25));
    }

    pub fn last_block(&self) -> &Block {
        if self.chain.len() > 1 {
            return &self.chain[self.chain.len() - 1];
        }

        &self.chain[0]
    }

    pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(index) => {
                    if index == idx {
                        return BlockSearchResult::Success(block);
                    }

                    if index >= self.chain.len() {
                        return BlockSearchResult::FailOfIndex(index);
                    }
                }
                /*
                matching will move the ownership that's why we need reference
                */
                BlockSearch::SearchByPreviousHash(ref hash) => {
                    if block.previous_hash == *hash {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        //to_vec make a clone of the vec being referenced
                        //prevent ownersihp moved
                        return BlockSearchResult::FailOfPreviousHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByBlockHash(ref hash) => {
                    if block.hash() == *hash {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        //to_vec make a clone of the vec being referenced
                        return BlockSearchResult::FailOfBlockHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByNonce(nonce) => {
                    if block.nonce == nonce {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfNonce(nonce);
                    }
                }

                BlockSearch::SearchByTimeStamp(time_stamp) => {
                    if block.time_stamp == time_stamp {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
                        return BlockSearchResult::FailOfTimeStamp(time_stamp);
                    }
                }

                BlockSearch::SearchByTransaction(ref transaction) => {
                    for tx in block.transactions.iter() {
                        if tx == transaction {
                            return BlockSearchResult::Success(block);
                        }
                    }

                    if idx >= self.chain.len() {
                        //to_vec make a clone of the vec being referenced
                        return BlockSearchResult::FailOfTransaction(transaction.to_vec());
                    }
                }
            }
        }

        return BlockSearchResult::FailOfEmptyBlocks;
    }

    pub fn add_transaction(&mut self, tx: &impl Serialization<Transaction>) {
        for tx_in_pool in self.transaction_pool.iter() {
            if *tx_in_pool == tx.serialization() {
                break;
            }
        }

        self.transaction_pool.push(tx.serialization())
    }

    fn do_proof_of_work(block: &mut Block) -> String {
        loop {
            let hash = block.hash();
            let hash_str = hex::encode(&hash);
            if hash_str[0..BlockChain::DIFFICULTY] == "0".repeat(BlockChain::DIFFICULTY) {
                return hash_str;
            }

            *block += 1;
        }
    }

    pub fn mining(&mut self) -> bool {
        /*
        if a block is mined, a transaction will created and the chain will send
        a coin to the miner
        */
        let tx = Transaction::new(
            BlockChain::MINING_SENDER.clone().into(),
            self.blockchain_address.clone().into(),
            BlockChain::MINING_REWOARD,
        );
        self.add_transaction(&tx);

        self.create_block(0, self.last_block().hash());

        true
    }

    pub fn calculate_total_amount(&self, address: String) -> i64 {
        let mut total_amount: i64 = 0;
        for i in 0..self.chain.len() {
            //we overload index operator before
            let block = &self[i];
            for t in block.transactions.iter() {
                let tx = Transaction::deserialization(t.clone());
                let value = tx.value;
                /*
                into is a trait used for converting one type into another type, String implement the trait
                of into with many different type, such as Into<str>, Into<i32>, Into<u64>...
                therefore we need to convert String to the trait with the
                right type,here we want string convert itself to Vec<u8>, then we need to convert String
                to type Into<Vec<u8>>.
                */
                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.recipient_address {
                    total_amount += value as i64;
                }

                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.sender_address {
                    total_amount -= value as i64;
                }
            }
        }

        total_amount
    }
}
