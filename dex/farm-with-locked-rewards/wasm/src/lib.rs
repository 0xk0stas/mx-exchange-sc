// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           65
// Async Callback:                       1
// Total number of exported functions:  67

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    farm_with_locked_rewards
    (
        enterFarm
        claimRewards
        exitFarm
        calculateRewardsForGivenPosition
        mergeFarmTokens
        startProduceRewards
        endProduceRewards
        setPerBlockRewardAmount
        getRewardPerShare
        getRewardReserve
        getFarmingTokenId
        getRewardTokenId
        getPerBlockRewardAmount
        getLastRewardBlockNonce
        getDivisionSafetyConstant
        setLockingScAddress
        setLockEpochs
        getLockingScAddress
        getLockEpochs
        registerFarmToken
        getFarmTokenId
        getFarmTokenSupply
        updateTotalFarmPosition
        getUserTotalFarmPosition
        addToPauseWhitelist
        removeFromPauseWhitelist
        pause
        resume
        getState
        addAdmin
        removeAdmin
        updateOwnerOrAdmin
        getPermissions
        addSCAddressToWhitelist
        removeSCAddressFromWhitelist
        isSCAddressWhitelisted
        set_penalty_percent
        set_minimum_farming_epochs
        set_burn_gas_limit
        getPenaltyPercent
        getMinimumFarmingEpoch
        getBurnGasLimit
        getPairContractManagedAddress
        claimBoostedRewards
        setBoostedYieldsRewardsPercentage
        collectUndistributedBoostedRewards
        getBoostedYieldsRewardsPercentage
        getAccumulatedRewardsForWeek
        getFarmSupplyForWeek
        getRemainingBoostedRewardsToDistribute
        getUndistributedBoostedRewards
        setBoostedYieldsFactors
        getBoostedYieldsFactors
        getCurrentWeek
        getFirstWeekStartEpoch
        getLastActiveWeekForUser
        getUserEnergyForWeek
        getLastGlobalUpdateWeek
        getTotalRewardsForWeek
        getTotalEnergyForWeek
        getTotalLockedTokensForWeek
        updateEnergyForUser
        getCurrentClaimProgress
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        callBack
    )
}
