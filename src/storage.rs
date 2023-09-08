multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// CUSTOM FORMAT
#[derive(PartialEq, TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct TokenIdentifierNonce<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub nonce: u64,
}

// STORAGE

#[multiversx_sc::module]
pub trait StorageModule {
    // Id To Address mapper
    #[view(getAddress)]
    #[storage_mapper("address")]
    fn address(&self, id: &usize) -> UnorderedSetMapper<ManagedAddress>;

    // Address to Balance mapper
    #[view(getBalance)]
    #[storage_mapper("balance")]
    fn balance(&self, address: &ManagedAddress) -> MapMapper<usize, BigUint>;

    // Id to (Token, Nonce) tuple mapper
    #[view(getTokenName)]
    #[storage_mapper("token_name")]
    fn token_name(&self, id: &usize) -> SingleValueMapper<TokenIdentifierNonce<Self::Api>>;

    // Token to Id mapper
    #[view(getId)]
    #[storage_mapper("id")]
    fn id(&self, token: &TokenIdentifier) -> SingleValueMapper<usize>;

    // Number of Tokens mapper 
    #[view(getTokenCount)]
    #[storage_mapper("token_count")]
    fn token_count(&self) -> SingleValueMapper<usize>;

    // Operator mapper
    #[view(getOperators)]
    #[storage_mapper("operator")]
    fn operator(&self, address: &ManagedAddress) -> UnorderedSetMapper<ManagedAddress>;

    // Last issued NFT by contract owner mapper
    #[view(getCurrentIssuedNft)]
    #[storage_mapper("current_issued_nft")]
    fn current_issued_nft(&self) -> SingleValueMapper<TokenIdentifier>;
}
