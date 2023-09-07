// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  11

#![no_std]

// Configuration that works with rustc < 1.71.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    erc1155
    (
        init => init
        mintFungibleToken => mint_fungible_token
        issueNonFungibleToken => issue_non_fungible_token
        mintNft => create_nft_with_attributes
        initTokenCount => init_token_count
        getAddress => address
        getBalance => balance
        getTokenName => token_name
        getTokenCount => token_count
        getCurrentIssuedNft => current_issued_nft
    )
}

multiversx_sc_wasm_adapter::async_callback! { erc1155 }
