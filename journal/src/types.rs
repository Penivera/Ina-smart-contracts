use near_sdk::{
    serde::{Deserialize, Serialize},
    borsh::{self, BorshDeserialize, BorshSerialize},
    
};
use near_contract_standards::non_fungible_token::{
    metadata::{NFTContractMetadata}
};
use schemars::JsonSchema;

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone,Debug
)]
pub enum Tag {
    Happy,
    Sad,
    Angry,
    Anxious,
    Hopeful,
    Grateful,
    Lonely,
    Confident,
    Tired,
    Overwhelmed,
}

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone,Debug
)]
pub struct JournalEntry {
    pub user: String,
    pub content: String,
    pub tags: Vec<Tag>,
    pub is_private: bool,
    pub timestamp: u64, // Optional: to track when the entry was created
    pub entry_id: u64, // Optional: if you want to track entries by ID
    pub minted_nft:bool, // Optional: to track if the entry has been minted as an NFT
}

pub trait HasNew {
    fn new(media_url: String) -> Self;
}

impl HasNew for NFTContractMetadata {
    fn new(media_url: String) -> Self {
        let extract_cid = |url: &str| url.split("/ipfs/").nth(1).unwrap_or("").to_string();
        Self {
            spec: "nft-2.0.0".to_string(),
            name: "unchainher-nft".to_string(),
            symbol: "UHER".to_string(),
            icon: Some(media_url.clone()),
            base_uri: Some(format!("ipfs://{}", extract_cid(&media_url))),
            reference: None,
            reference_hash: None,
        }
    }
}




