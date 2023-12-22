use std::{collections::HashMap, thread::{JoinHandle, Builder}, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use crossbeam_channel::{Receiver, RecvTimeoutError};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use solana_ledger::blockstore_processor::TransactionStatusMessage;
use solana_sdk::{pubkey::Pubkey, transaction::SanitizedTransaction, message::SanitizedMessage};


#[derive(Debug, Clone)]
pub struct VoteAggregatorServiceConfig{
    //This would be our "Copy-on-chain" program address
    program_of_interest: Pubkey,
    validator_set: HashMap<Pubkey, u64>,
}


pub struct VoteAggregatorService{
    thread_hdl: JoinHandle<()>
}

impl VoteAggregatorService {
    pub fn new(
        config: VoteAggregatorServiceConfig,
        transaction_status_receiver: &Receiver<TransactionStatusMessage>,
        exit: Arc<AtomicBool>,
    ) -> Self{
        let thread_hdl = Builder::new().name("votesAggService".to_string()).spawn(move || 
            loop {
                if exit.load(Ordering::Relaxed){
                   break; 
                }



            }
        ).unwrap();
        Self { thread_hdl }
    }

    pub fn join(self) -> std::thread::Result<()> {
        self.thread_hdl.join()
    }

    pub fn filter_transaction_of_interest(
        transaction_status_receiver: &Receiver<TransactionStatusMessage>,
        t_o_i_pubkey: Pubkey,
    ) -> Result<SanitizedTransaction, RecvTimeoutError>{
        match transaction_status_receiver.recv_timeout(Duration::from_secs(1))? {
            TransactionStatusMessage::Batch(batch) => {

                // filter out vote transactions as we dont need them.
                let filter_txs: Vec<_> = batch.transactions.par_iter().filter_map(|t|{
                    if !t.is_simple_vote_transaction(){
                        Some(t)
                    } else {
                        None
                    }
                }).collect();

                // extract out `TransactionMessage`s from the filtered transactions.
                let extracted_tx_messages: Vec<_> = filter_txs.par_iter().map(|t|{
                    t.message()
                }).collect();

                for m in extracted_tx_messages.iter(){
                    // any operation on m in this block
                    match m {
                        SanitizedMessage::Legacy(m) => {
                            let txs = m.message.account_keys.par_iter().for_each(|k| k == t_o_i_pubkey).collect();
                        },
                        SanitizedMessage::V0(m) => {

                        },
                    }
                }

                let tx = extracted_tx_messages.par_iter().map(|m|{
                    match m{
                        SanitizedMessage::Legacy(m) => {
                            if m.message.account_keys.par_iter()
                        },
                        SanitizedMessage::V0(m) => {

                        },
                    }
                })




                Ok(())
            },
            TransactionStatusMessage::Freeze(slot) => todo!(),
        }
        
    }


}

