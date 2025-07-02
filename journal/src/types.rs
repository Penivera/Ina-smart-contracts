use near_sdk::{
    serde::{Deserialize, Serialize},
    borsh::{self, BorshDeserialize, BorshSerialize},
};

use schemars::JsonSchema;

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone
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
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Clone
)]
pub struct JournalEntry {
    pub user: String,
    pub content: String,
    pub tags: Vec<Tag>,
    pub is_private: bool,
}
