#![no_std]

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
                self.token_count().update(|id| {
                    self.address(*id).insert(caller.clone());
                    self.balance(caller.clone())
                        .insert(*id, initial_supply.clone());
                    self.token_name(*id).set(token_identifier.unwrap_esdt());
                    *id += 1;
                })
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
    // Issue semi fungible token
    #[payable("EGLD")]
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
            ManagedAsyncCallResult::Ok(token_identifier) => self.token_count().update(|id| {
                self.address(*id).insert(caller.clone());
                self.balance(caller.clone())
                    .insert(*id, BigUint::from(1u64));
                self.token_name(*id).set(token_identifier);
                *id += 1;
            }),
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
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, token_identifier: &TokenIdentifier) {
        let sc_address = self.blockchain().get_sc_address();
        let roles = [
            EsdtLocalRole::NftAddQuantity,
        ];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&sc_address, token_identifier, roles[..].iter().cloned())
            .async_call()
            .call_and_exit()
    }

    ////////////////
    // Create Nft
    #[endpoint(createNft)]
    fn create_nft_with_attributes(
        &self,
        id: usize,
        name: ManagedBuffer,
        uri: ManagedBuffer,
        attribute: ManagedBuffer,
    ) {
        let nft_mapper = self.token_name(id-1);

        let attributes = NftAttributes {
            attribute1: attribute,
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();

        let uris = ManagedVec::from_single_item(uri);

        self.send().esdt_nft_create(
            &nft_mapper.get(),    // Token name
            &BigUint::from(1u64), // Amount to mint
            &name,                // Nft display name
            &BigUint::from(0u64), // Royalties
            attributes_hash,      // Nft Hash
            &attributes,          // Non formalized attributes
            &uris,                // uris
        );
    }

    #[endpoint(createNft2)]
    fn create_nft_with_attributes2(&self, id: usize ) {
        let nft_mapper = self.token_name(id).get() ;
        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let _ = self.send().esdt_nft_create_compact(
            &nft_mapper,
            &BigUint::from(1u64),
            &attributes,
        );
    }

        
    ////////////////
    // WARNING: DANGER ZONE!

    // DEV ONLY
    // Clear token count if needed
    #[endpoint(initTokenCount)]
    fn init_token_count(&self) {
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
