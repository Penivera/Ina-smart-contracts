// filepath: /journaling-vault-near/journaling-vault-near/src/lib.rs
use near_sdk::near_bindgen;
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::UnorderedMap;

mod models;
mod types;
mod contract;

#[near_bindgen]
#[derive(Default)]
pub struct JournalingVault {
    entries: UnorderedMap<String, models::JournalEntry>,
}

#[near_bindgen]
impl JournalingVault {
    pub fn add_journal_entry(&mut self, user: String, content: String, tags: Vec<types::Tag>, is_private: bool) {
        let entry = models::JournalEntry {
            user: user.clone(),
            content,
            tags: tags,
            is_private,
        };
        self.entries.insert(&user, &entry);
    }

    pub fn get_user_entries(&self, user: String) -> Option<models::JournalEntry> {
        self.entries.get(&user)
    }

    pub fn get_public_entries(&self) -> Vec<models::JournalEntry> {
        self.entries.values().filter(|entry| !entry.is_private).collect()
    }
}