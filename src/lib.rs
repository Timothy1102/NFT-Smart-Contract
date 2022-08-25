use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{collections::LookupMap, AccountId};
use near_sdk::{
    env, ext_contract, log, near_bindgen, Balance, CryptoHash, Gas, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};
use std::collections::HashMap;

pub use crate::approval::*;
pub use crate::enumeration::*;
pub use crate::event::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::royalty::*;
pub use crate::utils::*;
mod approval;
mod enumeration;
mod event;
mod internal;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod utils;

pub type TokenId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {
    pub owner_id: AccountId,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub tokens_by_id: LookupMap<TokenId, Token>,
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    TokenPerOwnerKey,
    ContractMetadataKey,
    TokenByIdKey,
    TokenMetaDataByIdKey,
    TokenPerOwnerInnerKey { account_id_hash: CryptoHash },
    UsersPerTokenKey { token_id_hash: CryptoHash },
}

#[near_bindgen]
impl Contract {
    
}
