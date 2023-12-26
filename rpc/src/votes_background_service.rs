use crossbeam_channel::{Receiver, RecvTimeoutError};
use jsonrpc_core::futures_util::future::Join;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use solana_ledger::blockstore_processor::TransactionStatusMessage;
use solana_program::hash::Hash;
use solana_sdk::{
    message::SanitizedMessage, pubkey::Pubkey, signature::Signature, slot_history::Slot,
    transaction::SanitizedTransaction,
};
use solana_vote::{
    vote_parser::{parse_sanitized_vote_transaction, ParsedVote},
    vote_transaction::VoteTransaction,
};
use solana_vote_program::vote_state::VoteState;
use std::str::FromStr;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{Builder, JoinHandle},
    time::Duration,
};
use dashmap::DashMap;

pub const LIGHT_CLIENT_PROGRAM: &str = "3UVYmECPPMZSCqWKfENfuoTv51fTDTWicX9xmBD2euKe";
#[derive(Debug, Clone)]
pub struct VoteAggregatorServiceConfig {
    //This would be our "Copy-on-chain" program address
    // program_of_interest: Pubkey,
    // validator_set: HashMap<Pubkey, u64>,
}


pub struct VoteAggregatorService {
    thread_hdl: JoinHandle<()>,
    // logger: JoinHandle<()>,
    votedb: Arc<DashMap<(Slot, Hash), Vec<Signature>>>,
}

// Need a thread pool builder using Rayon
//
impl VoteAggregatorService {
    pub fn new(
        config: VoteAggregatorServiceConfig,
        transaction_status_receiver: Arc<Receiver<TransactionStatusMessage>>,
        exit: Arc<AtomicBool>,
    ) -> Self {
        let mut votedb: Arc<DashMap<(Slot, Hash), Vec<Signature>>> = Arc::new(DashMap::default());
        let votedb_t = Arc::clone(&votedb);
        let thread_hdl = Builder::new()
            .name("votesAggService".to_string())
            .spawn(move || loop {
                if exit.load(Ordering::Relaxed) {
                    break;
                }
                
                // listens to receiver channel
                //  X - if it receives status message then filter transaction of interest from batches
                // filter vote txns then parse the data and read the slot and bankhash that they voted on
                // store in hashmap
                /*
                pub enum VoteTransaction {
                    Vote(Vote),
                    VoteStateUpdate(VoteStateUpdate),
                }

                pub struct Vote {
                    /// A stack of votes starting with the oldest vote
                    pub slots: Vec<Slot>,
                    /// signature of the bank's state at the last slot
                    pub hash: Hash,
                    /// processing timestamp of last slot
                    pub timestamp: Option<UnixTimestamp>,
                }


                pub struct VoteStateUpdate {
                    /// The proposed tower
                    pub lockouts: VecDeque<Lockout>,
                    /// The proposed root
                    pub root: Option<Slot>,
                    /// signature of the bank's state at the last slot
                    pub hash: Hash,
                    /// processing timestamp of last slot
                    pub timestamp: Option<UnixTimestamp>,
                }
                 */
                let vote_txns = VoteAggregatorService::filter_vote_transactions(
                    transaction_status_receiver.clone(),
                );
                match vote_txns {
                    Ok(votes) => {
                        let parsed_votes: Vec<ParsedVote> = votes
                            .iter()
                            .map(|tx| parse_sanitized_vote_transaction(tx))
                            .flatten()
                            .collect();
                        let _ = parsed_votes.into_iter().map(|v| {
                            let key = (v.1.slots().last().unwrap().to_owned(), v.1.hash());
                            let binding = votedb_t.get(&key);
                            let maybe_prev_entry: Option<&Vec<Signature>> =
                                binding.as_deref().clone();
                            if let Some(prev_entry) = maybe_prev_entry {
                                let mut new_entry = prev_entry.clone();
                                new_entry.push(v.3);
                                votedb_t.insert((v.1.slots().last().unwrap().to_owned(), v.1.hash()), new_entry.clone());
                                info!("vote_aggregator_service, {:?}, {:?}, {:?}",v.1.slots().last().unwrap().to_owned(), v.1.hash(), new_entry);
                            } else {
                                votedb_t.insert((v.1.slots().last().unwrap().to_owned(), v.1.hash()), vec![v.3]);
                                info!("vote_aggregator_service {:?}, {:?}, {:?} ",v.1.slots().last().unwrap().to_owned(), v.1.hash(), vec![v.3]);
                            }
                        });
                    }
                    _ => {}
                }
                // let display1:Vec<&Vec<Signature>>  = votedb.iter().map(|v| v.value()).collect();
              
            })
            .unwrap();
        Self {
            thread_hdl,
            votedb,
        }
    }

    pub fn join(self) -> std::thread::Result<()> {
        self.thread_hdl.join()
    }

    // filters by signature
    pub fn filter_transaction_of_interest(
        transaction_status_receiver: Arc<Receiver<TransactionStatusMessage>>,
        // t_o_i_pubkey: &Pubkey,
    ) -> Result<Option<SanitizedTransaction>, RecvTimeoutError> {
        match transaction_status_receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(TransactionStatusMessage::Batch(batch)) => {
                // filter out vote transactions as we dont need them.
                let txns = batch.transactions.clone();
                let filter_txs: Vec<_> = txns
                    .into_par_iter()
                    .filter_map(|t| {
                        if !t.is_simple_vote_transaction() {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect();

                let transaction_of_interest = filter_txs.into_par_iter().find_any(|t| {
                    t.message()
                        .account_keys()
                        .iter()
                        .find(|key| key == &&Pubkey::from_str(LIGHT_CLIENT_PROGRAM).unwrap())
                        .is_some()
                });

                Ok(transaction_of_interest.clone())
            }
            //TODO: can handle this case in a better way.
            Ok(TransactionStatusMessage::Freeze(_)) => Err(RecvTimeoutError::Timeout),
            Err(e) => Err(e),
        }
    }

    pub fn filter_vote_transactions(
        receiver: Arc<Receiver<TransactionStatusMessage>>,
    ) -> Result<Vec<SanitizedTransaction>, RecvTimeoutError> {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => match msg {
                TransactionStatusMessage::Batch(batch) => {
                    let filtered_txs: Vec<_> = batch
                        .transactions
                        .into_par_iter()
                        .filter_map(|t| {
                            if t.is_simple_vote_transaction() {
                                Some(t)
                            } else {
                                None
                            }
                        })
                        .collect();
                    Ok(filtered_txs)
                }
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
