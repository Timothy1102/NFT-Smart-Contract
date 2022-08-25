use std::collections::HashMap;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub owner_id: AccountId,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub next_approval_id: u64,
    pub royalty: HashMap<AccountId, u32>,
    pub users: UnorderedSet<AccountId>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub owner_id: AccountId,
    pub token_id: TokenId,
    pub metadata: TokenMetadata,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub royalty: HashMap<AccountId, u32>,
    pub users: Vec<AccountId>,
}
