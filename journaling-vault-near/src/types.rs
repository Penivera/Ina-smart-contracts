use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Mood {
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Cycle {
    CycleDay1,
    CycleDay2,
    CycleDay3,
    CycleDay4,
    CycleDay5,
    CycleDay6,
    CycleDay7,
    CycleDay8,
    CycleDay9,
    CycleDay10,
    CycleDay11,
    CycleDay12,
    CycleDay13,
    CycleDay14,
    CycleDay15,
    CycleDay16,
    CycleDay17,
    CycleDay18,
    CycleDay19,
    CycleDay20,
    CycleDay21,
    CycleDay22,
    CycleDay23,
    CycleDay24,
    CycleDay25,
    CycleDay26,
    CycleDay27,
    CycleDay28,
    CycleDay29,
    CycleDay30,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Tag {
    Mood(Mood),
    Cycle(Cycle),
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JournalEntry {
    pub user: AccountId,
    pub content: String,
    pub tags: Vec<Tag>, // Now supports multiple tags of either type
    pub is_private: bool,
}
