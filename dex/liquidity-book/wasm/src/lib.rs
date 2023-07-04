// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback:                       1
// Total number of exported functions:  21

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    liquidity_book
    (
        addLiquidity
        removeLiquidity
        swapTokens
        getPoolState
        getTicks
        getFirstTokenId
        getSecondTokenId
        registerLpToken
        getLpTokenId
        getLpTokenSupply
        addAdmin
        removeAdmin
        updateOwnerOrAdmin
        getPermissions
        addToPauseWhitelist
        removeFromPauseWhitelist
        pause
        resume
        getState
        callBack
    )
}