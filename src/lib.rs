#![no_std]

multiversx_sc::imports!();

mod storage;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Erc1155Contract: crate::storage::StorageModule {
    #[init]
    fn init(&self) {}

    ////////////////
    // Issue fungible token
    #[payable("EGLD")]
    #[endpoint(issueFungibleToken)]
    fn issue_fungible_token(
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
            .with_callback(self.callbacks().issue_fungible_callback(&caller))
            .call_and_exit()
    }

    #[callback]
    fn issue_fungible_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_identifier, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // self.creator_token(caller)
                //     .set(token_identifier.unwrap_esdt());
                // ?? Moving all logic in the token_count update doesn't work for some reason
                let id = self.token_count().get();
                self.address(id).insert(caller.clone());
                self.token_count().update(|id| *id += 1);
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
    // Set minting roles for sc address
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, token_identifier: &TokenIdentifier) {
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, token_identifier, roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    ////////////////
    // WARNING: DANGER ZONE!

    // DEV ONLY
    // Clear token count if needed
    #[endpoint(clear)]
    fn clear(&self) {
        self.token_count().set(1usize);
    }

    // DEV ONLY
    // Manually add to storage with token, id
    #[endpoint(addToStorage)]
    fn add_to_storage(&self, token: TokenIdentifier) {
        let caller = self.blockchain().get_caller();
        self.token_count().update(|id| {
            self.address(*id).insert(caller.clone());
            self.balance(caller.clone())
                .insert(*id, BigUint::from(100u64));
            self.token_name(*id).set(token);
            *id += 1;
        });
    }
}
