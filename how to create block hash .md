One of greateness of blockchain is that, no one can change the data that are already saved on the chain, and we can totaly trust the integrity of the data on the block. We will go to see how blockchain ensure that even with little
monitoring, the algorithm can make sure we can be notified if the data is changed even by a single bit.

This is thanks to the chain structure and the previous_hash field in the block, we will see how to compute the value of previous_hash field in the block now. First we need th sha256 hash uility to compute the hash result, we 
can get it by using command:
```rs
cargo add sha2
```
Then we can use it to compute hash for given bytes array like following in main.rs :
```rs
use sha2::{Digest, Sha256};
fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world\n");
    let result = hasher.finalize();
    println!("hash result is : {:x}", result);
}
```
Running the aboved code we have following result:
```rs
hash result is: ecf701f727d9e2d77c4aa49ac6fbbcc997278aca010bddeeb961c10cf54d435a
```
You can change the string in the update call and you will see even a very small change can cause a completely defferent result. Let's see how we can use the sha256 to compute the previous_hash field, first we need a way to 
convert the Block object into a byte array, and we add a method for the Block struct like following in blockchain/mod.rs:

```rs
impl Block {
    ....
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
```
In the method aboved, we convert nonce and time_stamp from integer to byte array format, and we combine all fields in Block into one byte array, then we can use this byte array to compute the sha256 hash, before that we add 
another method for BlockChain as following:

```rs
impl BlockChain {
 pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, vec![0 as u8; 32]);
        bc //no semicolon
    }
    ....
    pub fn last_block(&self) -> &Block {
        if self.chain.len() > 1 {
            return &self.chain[self.chain.len() - 1];
        }

        &self.chain[0]
    }
}
```
The last_block method just return the last Block object in the chain array, and in the construtor of BlockChain, It will create the first block with its previous_hash field set to all 0,
then we go to main.rs to compute the previous_hash as following:
```rs
fn main() {
    let mut block_chain = BlockChain::new();
    println!("block chain: {:?}", block_chain);
    block_chain.print();

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);
    block_chain.print();
    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(2, previous_hash);
    block_chain.print();
}

```
In the code aboved, each time we call create_block to generate a new block, we use last_block to get the previous block and calling the hash method of the Block struct to generate the hash result of the block, and using the
result to init the previous_hash field of the newly created block, running the aboved code we get the following result:
```rs
========================= Chain 0 =========================
timestamp: 17eb79b8842d0cf8
nonce: 0
previous_hash: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
transactions: []
========================= Chain 1 =========================
timestamp: 17eb79b884318968
nonce: 1
previous_hash: [199, 126, 239, 160, 184, 232, 19, 119, 137, 180, 23, 133, 160, 249, 74, 67, 125, 86, 247, 199, 5, 50, 39, 40, 222, 78, 40, 117, 210, 127, 202, 216]
transactions: []
========================= Chain 2 =========================
timestamp: 17eb79b884333b00
nonce: 2
previous_hash: [215, 14, 107, 84, 68, 124, 75, 39, 36, 114, 237, 2, 52, 144, 238, 135, 106, 121, 106, 146, 145, 104, 88, 87, 42, 195, 34, 153, 127, 185, 242, 76]
transactions: []
************************* 
```

Since we have linking blocks together as chain, now we need to think about how to do the search for given block, if you have used blockchain explorer like :https://www.blockchain.com/explorer, you can find given block by using
its hash, its order, therefore we wish we can provide block searching capability as a like. If we want to find given blocks by using block hash, previous hash, nonce, block index in the chain then, we may need to provide 
several functions like search_block_by_hash, search_block_by_previous_hash, search_block_by_index, but thinks to the powerful enum facility provided by Rust, we can use it to do all the search at one time.

For enum, they are most used for category value , and in c++, golang, java, enum are integer, but enum in Rust are more complicated, because enum value can contain intricate data like struct, we can use this capacity to 
simplify our block search function, in blockchain/mod.rs, we add the following enum definition for block search parameters and for search result:
```rs
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
```
enum of BlockSearch used for indicating searching block by which field, if we want to search block by index, then we will input SearchByIndex as indicator, and searching function can extract the index value from it. If we want
to search block by its hash, then we can use SearchByBlockHash as tag, and the code can extract the hash value from the tag and use it to search given block. The same is for BlockSearchResult, The searching result will send
back by using BlockSearchResult enum.

If the searching is suceess, then the tag value of Success will be returned and the searched block can be extracted from it as a block reference. If search fail, the enum will give fail reason, for example if the return is
FailOfBlockHash, this means we can't find any block with the given hash, and the hash value can extract from the tag. Let's see how we can implement the searching function:

```rs
impl BlockChain {
    ....
pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(index) => {
                    if index == idx {
                        return BlockSearchResult::Success(block);
                    }

                    if idx >= self.chain.len() {
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

                    if index >= self.chain.len() {
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

}
```
In the searching function aboved, we loop over the chain of blocks, and check what should we use for searching by matching the enum of input, if the value of the enum is SearchByIndex then it means we should return the block
with given index in the chain, then we can get the index which is attached to the tag value. There is one thing need to be cleared, the tag matching will cause ownership move, for example if we want to search by block hash,
then the hash value will be attached to the tag value of SearchByBlockHash, and the match of BlockSearch::SearchByBlockHash(ref hash) => {...} will be executed.

And the owershi of the hash vector will transfer from the input parameter to the local variable hash, this will cause problem that is when the loop exectue the second time, the hash vector that is passing through from the
parameter will invalidated, and if we don't return from the first loop, the code will fail at the second loop, that is why we using the "ref" keyword which means we convert the local variable in the match arm  BlockSearch::SearchByBlockHash(ref hash) => {...}  that is the local variable "hash" is a reference, then it will not cause the ownership transfer.

Then we go to main.rs and use code to call the function aboved as following:
```js
pub mod blockchain;
use blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult};
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
    let b = Block::new(0, vec![0 as u8; 32]);
    println!("block hash : {:?}", b.hash());

    let mut block_chain = BlockChain::new();
    println!("block chain: {:?}", block_chain);
    block_chain.print();

    let previous_hash = block_chain.last_block().hash();
    let hash_to_find = previous_hash.clone();

    block_chain.create_block(1, previous_hash);
    block_chain.print();
    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(2, previous_hash);
    block_chain.print();

    let result = block_chain.search_block(BlockSearch::SearchByIndex(1));
    get_block_search_result(result);

    let result = block_chain.search_block(BlockSearch::SearchByIndex(5));
    get_block_search_result(result);

    let result = block_chain.search_block(BlockSearch::SearchByBlockHash(hash_to_find));
    get_block_search_result(result);
}

```

The get_block_search_result is used to analyze the return from search_block block, it need to check all possible cases that are defined in BlockSearchResult, Since there are 8 cases in it, then we we use match to hande each
case, we have to consider 8 cases. If the tag value is Success which means given block is found, then we can get the reference of the block from the attachment of the tag value. Otherwise the search fail and the given tag
value indicates the fail reason, and give out the search condition that cause the search to fail. For example if the tag value is FailOfIndex, then we can get the index value used for the searching from the attachemnt of the
tag value.

Running aboved code we can get the following result:
```rs
*************************
========================= Chain 0  =========================
timestamp: 17ebea17bfbfc470
nonce:      0,
previous_hash:    [35, 17, 204, 91, 108, 147, 239, 78, 46, 113, 209, 244, 106, 143, 122, 217, 44, 226, 253, 118, 68, 255, 132, 195, 203, 232, 62, 80, 111, 88, 88, 23]
transactions:     []
========================= Chain 1  =========================
timestamp: 17ebea17bfc08ba8
nonce:      1,
previous_hash:    [249, 220, 134, 38, 186, 179, 167, 147, 130, 1, 246, 52, 153, 238, 129, 90, 12, 175, 73, 246, 186, 235, 85, 67, 17, 249, 87, 236, 196, 146, 135, 173]
transactions:     []
========================= Chain 2  =========================
timestamp: 17ebea17bfc26838
nonce:      2,
previous_hash:    [125, 7, 68, 249, 127, 52, 56, 253, 36, 154, 66, 72, 236, 67, 210, 104, 11, 15, 251, 12, 161, 200, 107, 248, 52, 103, 35, 162, 120, 212, 207, 26]
transactions:     []
*************************
find given block: Block { nonce: 1, previous_hash: [249, 220, 134, 38, 186, 179, 167, 147, 130, 1, 246, 52, 153, 238, 129, 90, 12, 175, 73, 246, 186, 235, 85, 67, 17, 249, 87, 236, 196, 146, 135, 173], time_stamp: 1723728670121561000, transactions: [] }
fail to find block with index: 5
find given block: Block { nonce: 0, previous_hash: [35, 17, 204, 91, 108, 147, 239, 78, 46, 113, 209, 244, 106, 143, 122, 217, 44, 226, 253, 118, 68, 255, 132, 195, 203, 232, 62, 80, 111, 88, 88, 23], time_stamp: 1723728670121510000, transactions: [] }
```

As you can see, when we use index 5 to seach block, we get the fail reason with the error index 5.
