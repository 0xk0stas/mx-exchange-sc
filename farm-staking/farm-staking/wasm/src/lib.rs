////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    farm_staking
    (
        callBack
        acceptSynchronization
        addAddressToWhitelist
        calculateRewardsForGivenPosition
        claimRewards
        claimRewardsWithNewValue
        compoundRewards
        end_produce_rewards
        getAccumulatedRewards
        getAnnualPercentageRewards
        getBurnGasLimit
        getCurrentCheckpointBlockNonce
        getDivisionSafetyConstant
        getFarmTokenId
        getFarmTokenSupply
        getFarmingTokenId
        getGlobalFarmTokenSupply
        getLastRewardBlockNonce
        getLockedAssetFactoryManagedAddress
        getMinUnbondEpochs
        getMinimumFarmingEpoch
        getPairContractManagedAddress
        getPenaltyPercent
        getPerBlockRewardAmount
        getRewardCapacity
        getRewardPerShare
        getRewardTokenId
        getSiblingSuppliesReceived
        getSiblingSupply
        getSiblingWhitelist
        getState
        isSiblingWhitelisted
        isWhitelisted
        mergeFarmTokens
        pause
        registerFarmToken
        removeAddressFromWhitelist
        resume
        setMaxApr
        setMinUnbondEpochs
        setPerBlockRewardAmount
        setSiblingWhitelist
        set_burn_gas_limit
        set_minimum_farming_epochs
        set_penalty_percent
        stakeFarm
        stakeFarmThroughProxy
        startProduceRewards
        synchronize
        topUpRewards
        unbondFarm
        unstakeFarm
        unstakeFarmThroughProxy
    )
}
