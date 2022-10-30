use serde_derive::{Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction { //交易结构体
    sender: String,
    reciever: String,
    amount:f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader { //区块头
    timestamp:i64,
    nonce:u32,
    pre_hash:String,
    merkle:String,
    difficulty:u32,
}

#[derive(Debug, Serialize)]
pub struct Block { //区块信息
    header:Blockheader,
    count:u32,
    transactions:Vec<Transaction>,
}

pub struct Chain { //整个链
    chain:Vec<Block>,
    curr_trans:Vec<Transaction>,
    difficulty:u32,
    miner_addr:String,
    reward:f32,
}

impl Chain {
    pub fn new(miner_addr:String, difficulty:u32) -> Chain { //初始化一条链
        let mut chain = Chain {
            chain:Vec::new(),
            curr_trans:Vec::new(),
            difficulty,
            miner_addr,
            reward:100.0,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender:String, reciever:String, amount:f32)->bool{
        self.curr_trans.push(Transaction{ // 将新的交易放入当前交易列表中
            sender,
            reciever,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String { //hash区块头
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48;64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool { //更新难度
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool { //更新区块奖励数量
        self.reward = reward;
        true
    }

    //核心 如何产生新的区块
    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader { //得到当前区块头 简化来说 nonce为零
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction { //向矿工转账
            sender: String::from("Root"),
            reciever: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block { //创造一个空白区块
            header,
            count: 0,
            transactions: vec![],
        };

        //向区块中添加信息
        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);//打印区块信息
        self.chain.push(block);//区块入链
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new(); //空白merkle树

        for t in &curr_trans { //hash每笔交易并放入merkle树
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 { // 如果merkle树叶子节点为奇数，即不平衡，则将最后一个hash复制加入
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 { //每两个左右相邻的hash值生成一个
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap() //最后得到根节点
    }

    pub fn proof_of_work(header: &mut Blockheader) { //工作量证明
        loop {
            let hash = Chain::hash(header); //hash区块头
            let slice = &hash[..header.difficulty as usize]; //将难度附加到hash值中
            match slice.parse::<u32>() { //检查数据类型
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String { //得到hash值并转化为String
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String { //hash值转化为string
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}



