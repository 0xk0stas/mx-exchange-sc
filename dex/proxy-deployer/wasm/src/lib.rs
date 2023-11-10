// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    proxy_deployer
    (
        init => init
        upgrade => upgrade
        deployFarm => deploy_farm
        callFarmEndpoint => call_farm_endpoint
        getAllDeployedFarms => get_all_deployed_farms
        getDeployerFarmAddresses => deployer_farm_addresses
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
