use chrono::prelude::*;
use utils::coder;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String, // tx_hash对应了一个区块里的交易 - transaction data merkle root hash
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: Vec<String>, // transaction data
}

impl Block {
    fn set_hash(&mut self) {
        let header = coder::my_serialize(&self.header);
        self.hash = coder::get_hash(&header[..]);
    }

    // data决定了tx_hash从而决定了header从而决定了Block.hash
    pub fn new_block(data: Vec<String>, pre_hash: String) -> Block {
        let mut transactions = Vec::new();
        for d in &data {
            transactions.push(coder::my_serialize(&d));
        }
        let merkle = merkle_hash(&transactions);
        let tx_hash = coder::get_hash(&merkle[0][..]);

        let time = Utc::now().timestamp();
        let mut block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,
                pre_hash: pre_hash,
            },
            hash: "".to_string(),
            data: data,
        };

        block.set_hash();

        block
    }
}

fn merkle_hash(value: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let len = value.len();

    if len == 1 {
        let ret = value.clone();
        return ret;
    }

    let mut sub_tx = Vec::new();

    let mut i = 0;
    while i < len {
        let h1 = coder::get_hash(&value[i]);

        let h2 = if i + 1 < len {
            coder::get_hash(&value[i + 1])
        } else {
            h1.clone()
        };

        sub_tx.push(coder::my_serialize(&(h1 + &h2)));

        i += 2;
    }

    merkle_hash(&sub_tx)
}
