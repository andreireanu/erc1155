multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// STORAGE


#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getAddress)]
    #[storage_mapper("address")]
    fn address(&self, id: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getBalance)]
    #[storage_mapper("balance")]
    fn balance(&self, address: ManagedAddress) -> MapMapper<usize, BigUint>;

    #[view(getTokenName)]
    #[storage_mapper("token_name")]
    fn token_name(&self, id: usize) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTokenCount)]
    #[storage_mapper("token_count")]
    fn token_count(&self) -> SingleValueMapper<usize>;

    #[view(getCreatorToken)]
    #[storage_mapper("creatorToken")]
    fn creator_token(&self, user: &ManagedAddress) -> SingleValueMapper<TokenIdentifier>;
}
