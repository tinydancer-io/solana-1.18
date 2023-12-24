use std::{collections::HashMap, thread::{JoinHandle, Builder}, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use crossbeam_channel::{Receiver, RecvTimeoutError};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};
use solana_ledger::blockstore_processor::TransactionStatusMessage;
use solana_sdk::{pubkey::Pubkey, transaction::SanitizedTransaction, message::SanitizedMessage, slot_history::Slot, signature::Signature};
use solana_vote::vote_parser::parse_sanitized_vote_transaction;
use solana_vote_program::vote_state::VoteState;
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

    // filters by signature
    pub fn filter_transaction_of_interest(
        transaction_status_receiver: &Receiver<TransactionStatusMessage>,
        t_o_i_signature: &Signature,
    ) -> Result<SanitizedTransaction, RecvTimeoutError>{
        match transaction_status_receiver.recv_timeout(Duration::from_secs(1)) {
           Ok(TransactionStatusMessage::Batch(batch)) => {
                // filter out vote transactions as we dont need them.
                let filter_txs: Vec<_> = batch.transactions.par_iter().filter_map(|t|{
                    if !t.is_simple_vote_transaction(){
                        Some(t)
                    } else {
                       None
                    }
                }).collect();

                let transaction_of_interest = filter_txs.into_par_iter().find_any(|t| {
                    t.signature() == t_o_i_signature
                }).unwrap(); 
                
                Ok(transaction_of_interest.clone())
            },
            //TODO: can handle this case in a better way.
           Ok(TransactionStatusMessage::Freeze(_)) => { Err(RecvTimeoutError::Timeout)},
           Err(e) => Err(e),
        }
        
    }

    pub fn filter_vote_transactions(
        receiver: &Receiver<TransactionStatusMessage>,
    ) -> Result<Vec<SanitizedTransaction>, RecvTimeoutError> {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => match msg {
                TransactionStatusMessage::Batch(batch) => {
                    let filtered_txs: Vec<_> = batch.transactions.into_par_iter().filter_map(|t| {
                        if t.is_simple_vote_transaction() {
                            Some(t)
                        } else {
                            None
                        }
                    }).collect();
                    Ok(filtered_txs)
                },
                _ => Ok(Vec::new()), // Return an empty vector for non-Batch messages
            },
            Err(err) => Err(err), // Handle the receive error
        }
    }
    // pub fn get_votes_for_slot(
    //     slot: u64,
    // ) -> Vec<VoteState>{

    // }


}

