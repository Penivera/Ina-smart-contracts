// filepath: /journaling-vault-near/journaling-vault-near/src/contract.rs

use near_sdk::near_bindgen;
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(Serialize, Deserialize)]
pub struct JournalEntry {
    pub user: AccountId,
    pub content: String,
    pub tags: Vec<String>,
    pub is_private: bool,
}

#[near_bindgen]
impl Contract {
    pub fn add_journal_entry(&mut self, user: AccountId, content: String, tags: Vec<String>, is_private: bool) {
        let entry = JournalEntry {
            user: user.clone(),
            content,
            tags,
            is_private,
        };
        self.journal_entries.push(entry);
    }

    pub fn get_user_entries(&self, user: AccountId) -> Vec<JournalEntry> {
        self.journal_entries.iter()
            .filter(|entry| entry.user == user)
            .collect()
    }

    pub fn get_public_entries(&self) -> Vec<JournalEntry> {
        self.journal_entries.iter()
            .filter(|entry| !entry.is_private)
            .collect()
    }
}