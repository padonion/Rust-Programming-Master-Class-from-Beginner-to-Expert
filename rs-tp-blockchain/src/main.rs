// BlockChain implementation in Rust
// https://andersbrownworth.com/blockchain/

use chrono::Utc;
use sha256::digest;

#[derive(Debug,Clone)]
struct BlockChain {
    blocks: Vec<Block>,
}

#[derive(Debug,Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl BlockChain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn starting_block(&mut self) {
        let genesis_block = Block::new(1, "Genesis".to_owned(),
         "0000000000000000000000000000000000000000000000000000000000000000".to_owned());
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) -> &mut Self {
        match self.blocks.last() {
            None => {
                println!("No blocks in the chain yet");
            }
            Some(last_block) => {
                if Self::is_block_valid(last_block, &block) {
                    self.blocks.push(block);
                } else {
                    println!("New block is not valid");
                }
            }
        }
        self
    }

    fn is_block_valid(last: &Block, block: &Block) -> bool {
        if block.id != last.id + 1 {
            println!("New block id {} is not one more than the last block id {}",block.id, last.id);
            return false;
        }
        if block.previous_hash != last.hash {
            println!("New block previous hash {} is not the same as the last block hash",block.previous_hash);
            return false;
        }
        if !block.hash.starts_with("0000") {
            println!("New block hash {} does not start with 0000",block.hash);
            return false;
        }
        if digest(format!("{}{}{}{}{}", block.id, block.previous_hash, block.data, block.timestamp, block.nonce)) != block.hash {
            println!("New block hash {} is not valid",block.hash);
            return false;
        }
        true
    }

    fn is_chain_valid(chain: &[Block]) -> bool {
        match chain.len() {
            0 => {
                println!("No blocks in the chain yet");
            },
            1 => {
                println!("Only one block in the chain (verify genesis block)");
                if chain[0].id != 1 {
                    println!("Genesis block id is not 1");
                    return false;
                }
                if chain[0].data != "Genesis" {
                    println!("Genesis block data is not 'Genesis'");
                    return false;
                }
                if chain[0].previous_hash != "0000000000000000000000000000000000000000000000000000000000000000" {
                    println!("Genesis block previous hash is not all zeros");
                    return false;
                }
                if chain[0].hash != digest(format!("{}{}{}{}{}", chain[0].id, chain[0].previous_hash, chain[0].data, chain[0].timestamp, chain[0].nonce)) {
                    println!("Genesis block hash is not valid");
                    return false;
                }
                if chain[0].hash.starts_with("0000") {
                    println!("Genesis block hash does not start with 0000");
                    return false;
                }
            },
            _ => {
                for i in 1..chain.len() {
                    let previous = chain.get(i - 1).unwrap();
                    let current = chain.get(i).unwrap();
                    if !Self::is_block_valid(previous, current) {
                        println!("Block id {} is not valid",chain[i].id);
                        return false;
                    }
                }
            }
        }
        println!("Chain is valid");
        true
    }

    fn chain_selector(local: Vec<Block>, remote: Vec<Block>) -> Option<Vec<Block>> {
        let is_valid_local = Self::is_chain_valid(&local);
        let is_valid_remote = Self::is_chain_valid(&remote);
        match (is_valid_local, is_valid_remote) {
            (true, true) => {
                if local.len() > remote.len() {
                    println!("Local chain is longer, using local chain");
                    Some(local)
                } else {
                    println!("Remote chain is longer, using remote chain");
                    Some(remote)
                }
            },
            (true, false) => {
                println!("Remote chain is not valid, using local chain");
                Some(local)
            },
            (false, true) => {
                println!("Local chain is not valid, using remote chain");
                Some(remote)
            },
            (false, false) => {
                println!("Both chains are not valid, returning None");
                None
            },
        }
    }
}

impl Block {
    fn new(id: u64, data: String, previous_hash: String) -> Self {
        let now = Utc::now();
        let now_timestamp = now.timestamp();
        let (nonce, hash) = Self::mine_block(id, now_timestamp, &previous_hash, &data);

        Self {
            id,
            nonce,
            data,
            hash,
            previous_hash,
            timestamp: now_timestamp,
        }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("Mining block id={} ...", id);
        let mut nonce = 0;
        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            let hash = digest(block_string);
            if hash.starts_with("0000") {
                println!("Block id={} mined: nonce={}, hash={}", id, nonce, hash);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let mut bc = BlockChain::new();
    bc.starting_block();
    println!("{:?}", bc);

    let new_block = Block::new(2, "Azam".to_owned(), bc.blocks[0].hash.to_owned());
    bc.try_add_block(new_block);
    println!("{:?}", bc);

    BlockChain::is_chain_valid(&bc.blocks);

    // lets tamper with the chain
    println!("Tampering with the chain ...");
    bc.blocks[1].data = "Egoist person :)".to_owned();
    BlockChain::is_chain_valid(&bc.blocks);

    // lets select the best chain
    println!("Creating blockchain1 (local) and blockchain2 (remote) ...");
    let mut bc1 = BlockChain::new();
    bc1.starting_block();

    let b = Block::new(2, "Azam".to_owned(), bc1.blocks.last().unwrap().hash.to_owned());
    bc1.try_add_block(b);

    let b = Block::new(3, "Stephan".to_owned(), bc1.blocks.last().unwrap().hash.to_owned());
    bc1.try_add_block(b);

    let b = Block::new(4, "Egoist".to_owned(), bc1.blocks.last().unwrap().hash.to_owned());
    bc1.try_add_block(b);

    BlockChain::is_chain_valid(&bc1.blocks);
    
    let mut bc2 = bc1.clone();
    let b = Block::new(5, "God".to_owned(), bc2.blocks.last().unwrap().hash.to_owned());
    bc2.try_add_block(b);

    BlockChain::is_chain_valid(&bc2.blocks);

    let selected_chain = BlockChain::chain_selector(bc1.blocks.to_owned(), bc2.blocks.to_owned());
    println!("{:?}", selected_chain);
}
