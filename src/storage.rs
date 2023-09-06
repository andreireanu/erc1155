multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// STORAGE

use crate::Balances;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("balances")]
    fn balances(&self, id: usize) -> SingleValueMapper<Balances>;

    #[storage_mapper("token_count")]
    fn token_count(&self) -> SingleValueMapper<usize>;
}
