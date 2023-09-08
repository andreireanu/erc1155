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
                self.update_storage(caller, initial_supply.clone(), 0u64, &token_identifier.unwrap_esdt());
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
        let caller = self.blockchain().get_caller();
        self.update_storage(&caller, BigUint::from(1u64), nonce, &token_identifier);
    }

    ////////////////
    // Update storage with new Token or NFT
    #[inline]
    fn update_storage(
        &self,
        caller: &ManagedAddress,
        supply: BigUint,
        nonce: u64,
        token: &TokenIdentifier,
    ) {
        let tin = TokenIdentifierNonce {
            token: token.clone(),
            nonce,
        };
        self.token_count().update(|id| {
            self.address(id).insert(caller.clone());
            self.balance(caller).insert(*id, supply);
            self.token_name(id).set(tin);
            self.id(token).set(id.clone());
            *id += 1;
        })
    }

    #[endpoint(depositToken)]
    #[payable("*")]
    fn deposit_token(&self, supply: BigUint, nonce: u64, token: TokenIdentifier) {
        let payment = self.call_value().single_esdt();
        require!(
            &payment.amount == &supply,
            "Incorrect parameters for function call. Payment amount other than deposited one."
        );
        require!(
            &payment.token_nonce == &nonce,
            "Incorrect parameters for function call. Payment token nonce other than deposited one."
        );
        require!(
            &payment.token_identifier == &token,
            "Incorrect parameters for function call. Payment token other than deposited one."
        );
        let id = self.id(&token);
        let caller = self.blockchain().get_caller();
        match id.is_empty() {
            // Register new Token
            true => {
                self.update_storage(&caller, supply, 0u64, &token);
            }
            // Update existing Token
            false => {
                let mut balance = self.balance(&caller).get(&id.get()).unwrap();
                balance += supply;
                self.balance(&caller).insert(id.get(), balance);
            }
        }
    }


    #[endpoint(withdrawToken)]
    fn withdraw_token(&self, supply: BigUint, nonce: u64, token: TokenIdentifier) {
        // Check if withdraw call valid
        let id = self.id(&token);
        require!(!id.is_empty(), "Token non existent in Smart Contract");
        let id = id.get();
        let caller = self.blockchain().get_caller();
        require!(
            self.address(&id).contains(&caller),
            "Token not registered for caller"
        );
        let mut balance = self.balance(&caller).get(&id).unwrap();
        require!(balance >= supply, "Token balance lower than requested one");

        // Match on token type
        match nonce {
            0 => {
                let _ = self.send().direct_esdt(&caller, &token, 0, &supply);
                balance -= supply;
                if balance == 0 {
                    self.balance(&caller).remove(&id);
                } else {
                    self.balance(&caller).insert(id, balance);
                }
            }
            _ => {}
        }
    }

 
    ////////////////
    // WARNING: DANGER ZONE!
    // THESE CALLS BREAK THE STORAGE LOGIC

    // Clear token count if needed (After deploying contract)
    #[only_owner]
    #[endpoint(initTokenCount)]
    fn init_token_count(&self) {
        self.token_count().set(1usize);
    }
}
