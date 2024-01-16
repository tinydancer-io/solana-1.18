// use solana_geyser_plugin_interface::geyser_plugin_interface::{ReplicaBlockInfoV2, SlotStatus};
use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_sdk::{hash::Hash, message::legacy::Message, pubkey::Pubkey, signature::Signature},
    std::collections::HashMap,
};
pub mod utils;
pub type AccountHashAccumulator = HashMap<u64, AccountHashMap>;
pub type TransactionSigAccumulator = HashMap<u64, u64>;
pub type SlotHashProofAccumulator = HashMap<u64, (Hash, BankHashProof)>;
pub type VoteAccumulator = HashMap<u64, VoteHashMap>;
pub type VoteHashMap = HashMap<Signature, VoteInfo>;
pub type AccountHashMap = HashMap<Pubkey, (u64, Hash, AccountInfo)>;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Proof {
    pub path: Vec<usize>, // Position in the chunk (between 0 and 15) for each level.
    pub siblings: Vec<Vec<Hash>>, // Sibling hashes at each level.
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct BankHashComponents {
    parent_bankhash: Hash,
    accounts_delta_hash: Hash,
    num_sigs: u64,
    current_blockhash: Hash,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Data {
    pub pubkey: Pubkey,
    pub hash: Hash,
    pub account: AccountInfo,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct AccountDeltaProof(pub Pubkey, pub (Data, Proof));

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct BankHashProof {
    pub proofs: Vec<AccountDeltaProof>,
    pub num_sigs: u64,
    pub account_delta_root: Hash,
    pub parent_bankhash: Hash,
    pub blockhash: Hash,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Update {
    pub slot: u64,
    pub root: Hash,
    pub proof: BankHashProof,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct AccountInfo {
    /// The Pubkey for the account
    pub pubkey: Pubkey,

    /// The lamports for the account
    pub lamports: u64,

    /// The Pubkey of the owner program account
    pub owner: Pubkey,

    /// This account's data contains a loaded program (and is now read-only)
    pub executable: bool,

    /// The epoch at which this account will next owe rent
    pub rent_epoch: u64,

    /// The data held in this account.
    pub data: Vec<u8>,

    /// A global monotonically increasing atomic number, which can be used
    /// to tell the order of the account update. For example, when an
    /// account is updated in the same slot multiple times, the update
    /// with higher write_version should supersede the one with lower
    /// write_version.
    pub write_version: u64,

    /// Slot number for this update
    pub slot: u64,
}

impl Default for AccountInfo {
    fn default() -> Self {
        AccountInfo {
            pubkey: Pubkey::default(),
            lamports: 0,
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
            data: Vec::new(),
            write_version: 0,
            slot: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransactionInfo {
    pub slot: u64,
    pub num_sigs: u64,
}

#[derive(Debug, Clone)]
pub struct VoteInfo {
    pub slot: u64,
    pub signature: Signature,
    pub vote_for_slot: u64,
    pub vote_for_hash: Hash,
    pub message: Message,
}
