// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    distribution
    (
        init => init
        upgrade => upgrade
        setCommunityDistribution => set_community_distribution
        setPerUserDistributedLockedAssets => set_per_user_distributed_locked_assets
        claimLockedAssets => claim_locked_assets
        clearUnclaimableAssets => clear_unclaimable_assets
        undoLastCommunityDistribution => undo_last_community_distrib
        undoUserDistributedAssetsBetweenEpochs => undo_user_assets_between_epochs
        setUnlockPeriod => set_unlock_period
        calculateLockedAssets => calculate_locked_assets_view
        deleteUserDistributedLockedAssets => delete_user_distributed_locked_assets
        getUsersDistributedLockedAssetsLength => get_users_distributed_locked_assets_length
        getUnlockPeriod => unlock_period
        getCommunityDistributionList => community_distribution_list
        getAssetTokenId => asset_token_id
        startGlobalOperation => global_op_start
        endGlobalOperation => global_op_stop
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
