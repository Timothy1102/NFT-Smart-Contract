use crate::*;

const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenApproval {
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);
    fn nft_is_approve(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_token_approval_receiver)]
pub trait NonFungibleTokenApprovalReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApproval for Contract {
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        assert_at_least_one_yocto();
        let mut token = self.tokens_by_id.get(&token_id).expect("Not found token");
        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Predecessor must be the token owner"
        );

        let approval_id = token.next_approval_id;
        let is_new_approval = token
            .approved_account_ids
            .insert(account_id.clone(), approval_id)
            .is_none();

        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        token.next_approval_id += 1;
        self.tokens_by_id.insert(&token_id, &token);

        refund_deposit(storage_used);

        if let Some(msg) = msg {
            ext_non_fungible_token_approval_receiver::nft_on_approve(
                token_id,
                token.owner_id,
                approval_id,
                msg,
                &account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            );
        }
    }

    fn nft_is_approve(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        let token = self.tokens_by_id.get(&token_id).expect("Token not found");
        let approval = token.approved_account_ids.get(&approved_account_id);
        if let Some(approval) = approval {
            if approval == &approval_id.unwrap() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
