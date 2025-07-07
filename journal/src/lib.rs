use std::collections::HashMap;
use near_sdk::{env, near_bindgen, AccountId,
    borsh::{self, BorshDeserialize, BorshSerialize},
    PanicOnDefault};
use serde::{Serialize,Deserialize};
mod types;
use types as journal_types;
use journal_types::{JournalEntry, Tag};
use schemars::JsonSchema;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault,Serialize,Deserialize,JsonSchema)]
pub struct JournalingVault {
    entries: HashMap<AccountId, JournalEntry>,
}
#[near_bindgen]
impl JournalingVault {
    #[init]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_journal_entry(&mut self, content: String, tags: Vec<Tag>, is_private: bool) {
        let user: AccountId = env::predecessor_account_id();
        let entry: JournalEntry = JournalEntry {
            user: String::from(user.clone()),
            content,
            tags,
            is_private,
        };
        self.entries.insert(user, entry);
    }

    pub fn get_user_entries(&self, user:AccountId ) -> Option<&JournalEntry> {
        self.entries.get(&user)
    }

    pub fn get_public_entries(&self) -> Vec<&JournalEntry> {
        self.entries
            .values()
            .filter(|entry| !entry.is_private)
            .collect()
    }
}
