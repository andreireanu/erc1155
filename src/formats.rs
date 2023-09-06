multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(PartialEq, TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Balances<M: ManagedTypeApi> {
    pub owner: ManagedAddress<M>,
    pub amount: BigUint<M>,
}
