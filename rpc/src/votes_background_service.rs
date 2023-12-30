use crossbeam_channel::{Receiver, RecvTimeoutError};
use dashmap::DashMap;
use jsonrpc_core::futures_util::future::Join;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator, ParallelBridge};
use solana_ledger::{blockstore_processor::TransactionStatusMessage, blockstore::Blockstore, blockstore_meta::VoteSignatureMeta};
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
        blockstore: Arc<Blockstore>,
        exit: Arc<AtomicBool>,
    ) -> Self {
        info!("vote_aggregator_service | entrypoint");
        let votedb: Arc<DashMap<(Slot, Hash), Vec<Signature>>> = Arc::new(DashMap::default());
        let votedb_t = Arc::clone(&votedb);
        let thread_hdl = Builder::new()
            .name("votesAggService".to_string())
            .spawn(move || loop {
                info!("vote_aggregator_service | spawn");
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
                info!("vote_aggregator_service | start filter");
                let vote_txns = VoteAggregatorService::filter_vote_transactions(
                    transaction_status_receiver.clone(),
                );
                info!("vote_aggregator_service | filtered votes {:?}", vote_txns);
                match vote_txns {
                    Ok(votes) => {
                        let parsed_votes: Vec<ParsedVote> = votes
                            .iter()
                            .map(|tx| parse_sanitized_vote_transaction(tx))
                            .flatten()
                            .collect();
                        let mut votes_by_slot: HashMap<Slot, Vec<Signature>> = HashMap::new();
                        info!("vote_aggregator_service | parsed votes {:?}", parsed_votes);
                        for v in parsed_votes {
                            let slot = *v.1.slots().last().unwrap(); // Get the slot
                            let signature = v.3; // Get the signature

                            // Step 3: Aggregate signatures
                            votes_by_slot.entry(slot).or_insert_with(Vec::new).push(signature);                        
                        }
                        // Step 4: Populating the blockstore.
                        for (slot, signatures) in votes_by_slot {
                            let vote_signature_meta = VoteSignatureMeta { signature: signatures };
                            let _ = blockstore.write_vote_signature(slot, vote_signature_meta);
                        }
                    }
                    _ => {}
                }
                // let display1:Vec<&Vec<Signature>>  = votedb.iter().map(|v| v.value()).collect();
            })
            .unwrap();
        Self { thread_hdl, votedb }
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
                        .into_iter()
                        .zip(batch.execution_results.into_iter())
                        .par_bridge()
                        .filter_map(|(t, b)| {
                            if t.is_simple_vote_transaction() && !b.is_none(){
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
