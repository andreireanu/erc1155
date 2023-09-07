#![no_std]

use storage::TokenIdentifierNonce;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub attribute1: ManagedBuffer<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Erc1155Contract: crate::storage::StorageModule {
    #[init]
    fn init(&self) {}

    ////////////////
    // Issue fungible token
    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(mintFungibleToken)]
    fn mint_fungible_token(
        &self,
        token_display_name: &ManagedBuffer,
        token_ticker: &ManagedBuffer,
        initial_supply: &BigUint,
    ) {
        let issue_cost = self.call_value().egld_value().clone_value();
        let caller = self.blockchain().get_caller();

        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(
                self.callbacks()
                    .mint_fungible_callback(&caller, &initial_supply),
            )
            .call_and_exit()
    }

    #[callback]
    fn mint_fungible_callback(
        &self,
        caller: &ManagedAddress,
        initial_supply: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // need to also mint when issuing, otherwise callback doesn't work
                let tin = TokenIdentifierNonce {
                    token: token_identifier.unwrap_esdt(),
                    nonce: None,
                };
                self.update_storage(caller, initial_supply.clone(), tin);
            }
            ManagedAsyncCallResult::Err(_message) => {
                // return issue cost to the caller
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
            }
        }
    }

    ////////////////
    // Issue non fungible token
    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(issueNonFungibleToken)]
    fn issue_non_fungible_token(
        &self,
        token_display_name: &ManagedBuffer,
        token_ticker: &ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value().clone_value();
        let caller = self.blockchain().get_caller();
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().nft_issue_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn nft_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_identifier) => {
                self.current_issued_nft().set(token_identifier.clone());
                self.set_local_roles(&token_identifier.clone());
            }
            ManagedAsyncCallResult::Err(_message) => {
                // return issue cost to the caller
                let (token_identifier, returned_tokens) =
                    self.call_value().egld_or_single_fungible_esdt();
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens);
                }
            }
        }
    }

    ////////////////
    // Set minting roles for sc address
    #[inline]
    fn set_local_roles(&self, token_identifier: &TokenIdentifier) {
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftBurn,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, token_identifier, roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    #[only_owner]
    #[endpoint(mintNft)]
    fn create_nft_with_attributes(&self) {
        let token_identifier = self.current_issued_nft().get();

        require!(token_identifier.is_valid_esdt_identifier(), "No NFT issued");
        
        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let nonce = self.send().esdt_nft_create_compact(
            &token_identifier,
            &BigUint::from(1u64),
            &attributes,
        );
        let tin = TokenIdentifierNonce {
            token: token_identifier,
            nonce: Some(nonce),
        };
        let caller = self.blockchain().get_caller();
        self.update_storage(&caller, BigUint::from(1u64), tin);
    }

    ////////////////
    // Update storage with new Token or NFT 
    #[inline]
    fn update_storage(
        &self,
        caller: &ManagedAddress,
        initial_supply: BigUint,
        token_identifier: TokenIdentifierNonce<Self::Api>,
    ) {
        self.token_count().update(|id| {
            self.address(*id).insert(caller.clone());
            self.balance(caller.clone()).insert(*id, initial_supply);
            self.token_name(*id).set(token_identifier);
            *id += 1;
        })
    }

    ////////////////
    // WARNING: DANGER ZONE!

    // DEV ONLY
    // Clear token count if needed
    #[only_owner]
    #[endpoint(initTokenCount)]
    fn init_token_count(&self) {
        self.token_count().set(1usize);
    }
}
