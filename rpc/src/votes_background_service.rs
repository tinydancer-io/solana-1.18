use std::{collections::{HashSet, HashMap}, thread::JoinHandle};
use solana_sdk::pubkey::Pubkey;


#[derive(Debug, Clone)]
pub struct VoteAggregatorServiceConfig{
    //This would be our "Copy-on-chain" program address
    program_of_interest: Pubkey,
    validator_set: HashMap<Pubkey, u64>,
}


pub struct VoteAggregatorService{
    thread_hdl: JoinHandle<()>
}

impl VoteAggregatorService{
    pub fn new() -> Self{
        // let thread_hdl = 
        Self { thread_hdl: () }
    } 
}