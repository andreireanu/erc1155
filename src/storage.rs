multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// STORAGE

use crate::formats::Balances;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getBalances)]
    #[storage_mapper("balances")]
    fn balances(&self, id: usize) -> SingleValueMapper<Balances<Self::Api>>;

    #[view(getBalanceById)]
    fn get_balance_by_id(&self, id: usize) -> Balances<Self::Api> {
        self.balances(id).get()
    }

    #[view(tokenCount)]
    #[storage_mapper("token_count")]
    fn token_count(&self) -> SingleValueMapper<usize>;
}
