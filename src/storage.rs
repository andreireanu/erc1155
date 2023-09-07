multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// CUSTOM FORMAT
#[derive(PartialEq, TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct TokenIdentifierNonce<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: Option<u64>,
}


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
    fn token_name(&self, id: usize) -> SingleValueMapper<TokenIdentifierNonce<Self::Api>>;

    #[view(getTokenCount)]
    #[storage_mapper("token_count")]
    fn token_count(&self) -> SingleValueMapper<usize>;

    #[view(getCurrentIssuedNft)]
    #[storage_mapper("current_issued_nft")]
    fn current_issued_nft(&self) -> SingleValueMapper<TokenIdentifier>;
}
