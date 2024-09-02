//referencing parent or sibling modules by absolute path
use crate::blockchain::*;
use std::fmt;
use std::str;
#[derive(Debug)]
pub struct Transaction {
    pub sender_address: Vec<u8>,
    pub recipient_address: Vec<u8>,
    pub value: u64,
}

impl Transaction {
    pub fn new(sender: Vec<u8>, recipient: Vec<u8>, value: u64) -> Transaction {
        Transaction {
            sender_address: sender,
            recipient_address: recipient,
            value,
        }
    }
}

impl Serialization<Transaction> for Transaction {
    fn serialization(&self) -> Vec<u8> {
        /*
        1,8 bytes for length of sender address vector
        2,bytes for sender address vector
        3,8 bytes for length of recipient address vector
        4, bytes for recipient address vector
        5,8 bytes for length of value
        6, bytes of value
        */
        let mut bin = Vec::<u8>::new();
        let len_sender = self.sender_address.len();
        bin.extend(len_sender.to_be_bytes().to_vec());
        bin.extend(&self.sender_address);
        let len_recipient = self.recipient_address.len();
        bin.extend(len_recipient.to_be_bytes().to_vec());
        bin.extend(&self.recipient_address);
        let len_value = self.value.to_be_bytes().len();
        bin.extend(len_value.to_be_bytes().to_vec());
        bin.extend(self.value.to_be_bytes().to_vec());
        bin
    }

    fn deserialization(bytes: Vec<u8>) -> Transaction {
        //1, first 8 bytes are length of sender address
        let mut pos = 0;
        /*
        from_be_bytes expect to receive byte array [u8; len]
        but bytes[pos..pos+8] is slice, the try_into trait convert slice to
        array
        */
        let len_sender = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        let mut sender_address = Vec::<u8>::new();
        pos += 8;
        //2. bytes to sender address
        sender_address.extend_from_slice(&bytes[pos..pos + len_sender]);
        pos += len_sender;
        //3, 8 bytes are length of recipient address
        let len_recipient = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        //4. bytes to recipient address
        let mut recipient_address = Vec::<u8>::new();
        recipient_address.extend_from_slice(&bytes[pos..pos + len_recipient]);
        pos += len_recipient;
        //5, 8 bytes for length of value
        let len_value = usize::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        //6,read the content of value
        let value: u64 = u64::from_be_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        Transaction {
            sender_address,
            recipient_address,
            value,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nsender address: {:?}\nreceipent address: {:?}\nvalue:{}\n {}",
            "-".repeat(40),
            str::from_utf8(&self.sender_address),
            str::from_utf8(&self.recipient_address),
            self.value,
            "-".repeat(40)
        )
    }
}
