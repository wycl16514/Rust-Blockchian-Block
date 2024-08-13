use std::time::SystemTime;
#[derive(Debug)]
pub struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

/*
CamelCase for name of struct, snake_case for name of fields
*/

impl Block {
    //methods for the struct, class methods,
    //Two kinds of methods, one kind static method which not reading or
    //writing into fields of the block
    //Self is alias name for object, if we change the name of the struct
    //then we don't need to change the name inside here
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        //the method will take control of the input of previous_hash
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Block {
            nonce: nonce,
            previous_hash: previous_hash,
            time_stamp: time_now.as_nanos(),
            transactions: Vec::<Vec<u8>>::new(),
        } //don't add semicolon because we want to return this object
    }

    //struct method which is need to read or write to fields of the struct
    //self which reference to the struct instance
    pub fn print(&self) {
        //format value as hex
        println!("timestamp: {:x}", self.time_stamp);
        //format integer
        println!("nonce: {}", self.nonce);
        //print vecotr, ask the compiler to do it
        println!("previous_hash: {:?}", self.previous_hash);
        println!("transactions: {:?}", self.transactions)
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };

        bc.create_block(0, "Hash for very first block".to_string().into_bytes());
        bc //no semicolon
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }

    pub fn print(&self) {
        /*
        using ieterator to loop over the vector, it is complicate and powerful
        */
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} Chain {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "*".repeat(25));
    }
}
