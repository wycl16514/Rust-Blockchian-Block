use std::time::SystemTime;
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
        println!("transactions:     {:?}", self.transactions);
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
}

impl BlockChain {
    pub fn new() -> Self {
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
        };

        bc.create_block(0, "Hash for the very first block".to_string().into_bytes());
        bc //no semicolon for returning it
    }
    //we need to write to fields of the given struct
    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
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
}
