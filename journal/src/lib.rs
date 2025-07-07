mod types;
use near_sdk::{env,
    log,
    AccountId,
    PanicOnDefault,
    near_bindgen,
    store::LazyOption,
    collections::UnorderedSet,
    borsh::{self, BorshDeserialize, BorshSerialize},
};
use crate::types::{JournalEntry, Tag,HasNew};
use near_contract_standards::non_fungible_token::{
    metadata::{ NFTContractMetadata, TokenMetadata},
    NonFungibleToken,  
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault,)]
pub struct JournalingContract{
    pub tokens: NonFungibleToken,
    pub metadata: LazyOption<NFTContractMetadata>,
    pub minters: UnorderedSet<AccountId>,
    pub journal_entries: HashMap<String, JournalEntry>,
    pub next_entry_id: u64, // Optional: if you want to track entries by ID
}

#[near_bindgen]
impl JournalingContract {
    #[init]
    #[private]
    pub fn initiate(media_url:String) -> Self {
        let owner_id: AccountId = env::signer_account_id();
        Self {
            journal_entries: HashMap::new(),
            tokens: NonFungibleToken::new(
                b"nft".to_vec(),owner_id.clone(),
                Some(b"nft_approvals".to_vec()),
                Some(b"nft_enumeration".to_vec()),
                Some(b"nft_metadata".to_vec()),
            ),
            metadata: LazyOption::new(b"metadata".to_vec(), Some(NFTContractMetadata::new(media_url.clone()))),
            minters: UnorderedSet::new(b"minters".to_vec()),
            next_entry_id: 0,
        }
    }

    pub fn add_journal_entry(&mut self, content: String, tags: Vec<Tag>, is_private: bool) {
        let user: AccountId = env::predecessor_account_id();
        assert!(content.len() > 0, "Entry content cannot be empty");
        let entry: JournalEntry = JournalEntry {
            user: String::from(user.clone()),
            content,
            tags,
            is_private,
            timestamp: env::block_timestamp()/1e9 as u64,
            entry_id: self.next_entry_id,
            minted_nft: false, // Initially set to false, can be updated later
        };
        log!("Adding journal entry for user: {}, Private {:?}", user, entry.is_private);
        self.journal_entries.insert(user.to_string(), entry);
    }
    
    #[private]
    pub fn get_user_entries(&self, user:AccountId ) -> Option<&JournalEntry> {
        self.journal_entries.get(&user.to_string())
    }

    #[private]
    pub fn get_public_entries(&self) -> Vec<&JournalEntry> {
        self.journal_entries.values().filter(|entry: &&JournalEntry| !entry.is_private).collect()
    }

    #[private]
    pub fn get_private_entries(&self, user: AccountId) -> Vec<&JournalEntry> {
        self.journal_entries.values().filter(|entry: &&JournalEntry| entry.is_private && entry.user == user).collect()
    }

    #[private]
    pub fn get_all_entries(&self) -> Vec<&JournalEntry> {
        self.journal_entries.values().collect()
    }

    #[payable]
    pub fn mint_nft_for_entry(&mut self, entry_id: u64,media_url:String) {
        let caller_id: AccountId = env::signer_account_id();
        log!("Minting NFT for entry ID: {}, Caller: {}", entry_id, caller_id);
        
        let entry: &JournalEntry = self.journal_entries.get(&caller_id.to_string()).expect("Entry not found for caller");
        assert!(!entry.minted_nft, "This entry has already been minted as an NFT");
        assert!(entry.user == caller_id, "You can only mint your own entries");
        //assert!(entry.is_private == false, "You cannot mint a private entry as an NFT");
        assert!(media_url.len() > 0, "Media URL cannot be empty");


        let token_metadata = TokenMetadata {
            title: Some(format!("{:?} Journal Entry ", entry.tags)),
            description: Some(entry.content.clone()),
            media: Some(media_url.clone()),
            copies: Some(1),
            issued_at: Some(env::block_timestamp().to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            media_hash: None,
        };

        self.tokens.internal_mint(
            entry_id.to_string(),
            caller_id.clone(),
            Some(token_metadata),
        );
        log!("NFT minted successfully for entry ID: {}", entry_id);

        // Update the journal entry to mark it as minted
        let mut updated_entry = entry.clone();
        updated_entry.minted_nft = true;
        self.journal_entries.insert(caller_id.to_string(), updated_entry);
    }

    #[private]
    pub fn nft_metadata(&self) -> NFTContractMetadata {
        let metadata = self.metadata.clone();
        metadata.unwrap_or_else(|| NFTContractMetadata::new("".to_string()))
    }

    #[private]
    pub fn get_minters(&self) -> Vec<AccountId> {
        self.minters.to_vec()
    }

    #[private]
    pub fn get_token_metadata(&self, journal_id: String) -> Option<TokenMetadata> {
         assert!(!journal_id.is_empty(), "Journal ID cannot be empty");

        let entry = self.journal_entries.get(&journal_id).expect("Journal entry not found");

        assert!(entry.minted_nft, "This journal entry has not been minted as an NFT");
        self.tokens.token_metadata_by_id.as_ref()?.get(&journal_id)

    }

    #[private]
    pub fn get_journal_by_token_id(&self, token_id: String) -> Option<&JournalEntry> {
        self.journal_entries.get(&token_id)
    }

    #[private]
    pub fn get_journal_by_id(&self, entry_id: u64) -> Option<&JournalEntry> {
        log!("Fetching journal entry for Entry ID: {}", entry_id);
        self.journal_entries.values().find(|entry| entry.entry_id == entry_id)
    }

    pub fn get_minted_entries(&self) -> Vec<&JournalEntry> {
        let user: AccountId = env::predecessor_account_id();
        self.journal_entries.values().filter(|entry| entry.minted_nft && entry.user == user).collect()
    }

    #[private]
    pub fn get_all_minted_entries(&self) -> Vec<&JournalEntry> {
        self.journal_entries.values().filter(|entry| entry.minted_nft).collect()
    }

    pub fn get_my_journal_entries(&self) -> Vec<&JournalEntry> {
        let user: AccountId = env::predecessor_account_id();
        self.journal_entries.values().filter(|entry| entry.user == user).collect()
    }
    pub fn get_my_private_journal_entries(&self) -> Vec<&JournalEntry> {
        let user: AccountId = env::predecessor_account_id();
        self.journal_entries.values().filter(|entry| entry.is_private && entry.user == user).collect()
    }

    pub fn get_my_minted_journal_entries(&self) -> Vec<&JournalEntry> {
        let user: AccountId = env::predecessor_account_id();
        self.journal_entries.values().filter(|entry| entry.minted_nft && entry.user == user).collect()
    }
    pub fn get_my_public_journal_entries(&self) -> Vec<&JournalEntry> {
        let user: AccountId = env::predecessor_account_id();
        self.journal_entries.values().filter(|entry| !entry.is_private && entry.user == user).collect()
    }

    pub fn get_my_journal_entries_by_tag(&self, tag: Tag) -> Vec<&JournalEntry> {
        let user: AccountId = env::predecessor_account_id();
        self.journal_entries.values().filter(|entry| entry.user == user && entry.tags.contains(&tag)).collect()
    }


}