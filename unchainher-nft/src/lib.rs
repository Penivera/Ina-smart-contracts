// Find all our documentation at https://docs.near.org
#[allow(unused_imports)]
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near, AccountId, PanicOnDefault,
};

use near_contract_standards::non_fungible_token::{
    metadata::{NFTContractMetadata, TokenMetadata},
    NonFungibleToken,
};

use near_sdk::collections::{LazyOption, UnorderedSet};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    minters: UnorderedSet<AccountId>,
}

trait HasNew {
    fn new(media_url: String) -> Self;
}

impl HasNew for NFTContractMetadata {
    fn new(media_url: String) -> Self {
        let extract_cid = |url: &str| url.split("/ipfs/").nth(1).unwrap_or("").to_string();
        Self {
            spec: "nft-2.0.0".to_string(),
            name: "unchainher-nft".to_string(),
            symbol: "INA".to_string(),
            icon: Some(media_url.clone()),
            base_uri: Some(format!("ipfs://{}", extract_cid(&media_url))),
            reference: None,
            reference_hash: None,
        }
    }
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn new(media_url: String) -> Self {
        let owner_id: AccountId = env::predecessor_account_id();
        log!("Initializing contract with owner: {}", owner_id);
        let tokens: NonFungibleToken = NonFungibleToken::new(
            b"nft".to_vec(),
            owner_id.clone(),
            Some(b"nft_approvals".to_vec()),
            Some(b"nft_enumeration".to_vec()),
            Some(b"nft_metadata".to_vec()),
        );

        let metadata: NFTContractMetadata = NFTContractMetadata::new(media_url.clone());

        let contract: Contract = Self {
            tokens,
            metadata: LazyOption::new(b"metadata".to_vec(), Some(&metadata)),
            minters: UnorderedSet::new(b"minters".to_vec()),
        };

        contract
    }
    pub fn mint_nft(&mut self) {
        let caller_id: AccountId = env::signer_account_id();
        log!("Minting NFT for caller: {}", caller_id);
        assert!(
            !self.minters.contains(&caller_id),
            "You have already minted your NFT"
        );
        let token_metadata: TokenMetadata = TokenMetadata {
            title: Some("UnchainHer NFT".to_string()),
            description: Some("Empowering Women with Ownership".to_string()),
            media: Some(
                self.metadata
                    .get()
                    .unwrap()
                    .icon
                    .clone()
                    .unwrap_or_default(),
            ),
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
            caller_id.clone().to_string(),
            caller_id.clone(),
            Some(token_metadata),
        );

        self.minters.insert(&caller_id);
    }
    pub fn get_metadata(&self) -> NFTContractMetadata {
        self.metadata
            .get()
            .unwrap_or_else(|| NFTContractMetadata::new("".to_string()))
    }
    pub fn get_minters(&self) -> Vec<AccountId> {
        self.minters.to_vec()
    }
    pub fn get_token_metadata(&self) -> Option<TokenMetadata> {
        let token_id: AccountId = env::signer_account_id();
        self.tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|map| map.get(&token_id.to_string()))
    }
}
