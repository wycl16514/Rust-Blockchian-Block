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
