// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;
use multiversx_sc_scenario::multiversx_sc;

pub struct PairProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PairProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PairProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PairProxyMethods { wrapped_tx: tx }
    }
}

pub struct PairProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PairProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
        Arg3: ProxyArg<ManagedAddress<Env::Api>>,
        Arg4: ProxyArg<u64>,
        Arg5: ProxyArg<u64>,
        Arg6: ProxyArg<ManagedAddress<Env::Api>>,
        Arg7: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        first_token_id: Arg0,
        second_token_id: Arg1,
        router_address: Arg2,
        router_owner_address: Arg3,
        total_fee_percent: Arg4,
        special_fee_percent: Arg5,
        initial_liquidity_adder: Arg6,
        admins: Arg7,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&first_token_id)
            .argument(&second_token_id)
            .argument(&router_address)
            .argument(&router_owner_address)
            .argument(&total_fee_percent)
            .argument(&special_fee_percent)
            .argument(&initial_liquidity_adder)
            .argument(&admins)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PairProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxProxyUpgrade<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PairProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_lp_token_identifier<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setLpTokenIdentifier")
            .argument(&token_identifier)
            .original_result()
    }

    pub fn is_fee_enabled(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, bool> {
        self.wrapped_tx
            .raw_call("getFeeState")
            .original_result()
    }

    pub fn whitelist_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("whitelist")
            .argument(&address)
            .original_result()
    }

    pub fn remove_whitelist<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeWhitelist")
            .argument(&address)
            .original_result()
    }

    pub fn add_trusted_swap_pair<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        first_token: Arg1,
        second_token: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("addTrustedSwapPair")
            .argument(&pair_address)
            .argument(&first_token)
            .argument(&second_token)
            .original_result()
    }

    pub fn remove_trusted_swap_pair<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        first_token: Arg0,
        second_token: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeTrustedSwapPair")
            .argument(&first_token)
            .argument(&second_token)
            .original_result()
    }

    /// `fees_collector_cut_percentage` of the special fees are sent to the fees_collector_address SC 
    ///  
    /// For example, if special fees is 5%, and fees_collector_cut_percentage is 10%, 
    /// then of the 5%, 10% are reserved, and only the rest are split between other pair contracts. 
    pub fn setup_fees_collector<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        fees_collector_address: Arg0,
        fees_collector_cut_percentage: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setupFeesCollector")
            .argument(&fees_collector_address)
            .argument(&fees_collector_cut_percentage)
            .original_result()
    }

    pub fn set_fee_on<
        Arg0: ProxyArg<bool>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        enabled: Arg0,
        fee_to_address: Arg1,
        fee_token: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setFeeOn")
            .argument(&enabled)
            .argument(&fee_to_address)
            .argument(&fee_token)
            .original_result()
    }

    pub fn get_fee_destinations(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueEncoded<Env::Api, (ManagedAddress<Env::Api>, TokenIdentifier<Env::Api>)>> {
        self.wrapped_tx
            .raw_call("getFeeDestinations")
            .original_result()
    }

    pub fn get_trusted_swap_pairs(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueEncoded<Env::Api, (common_structs::wrapper_types::TokenPair<Env::Api>, ManagedAddress<Env::Api>)>> {
        self.wrapped_tx
            .raw_call("getTrustedSwapPairs")
            .original_result()
    }

    pub fn get_whitelisted_managed_addresses(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getWhitelistedManagedAddresses")
            .original_result()
    }

    pub fn fees_collector_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFeesCollectorAddress")
            .original_result()
    }

    pub fn fees_collector_cut_percentage(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getFeesCollectorCutPercentage")
            .original_result()
    }

    pub fn set_state_active_no_swaps(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setStateActiveNoSwaps")
            .original_result()
    }

    pub fn set_fee_percent<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        total_fee_percent: Arg0,
        special_fee_percent: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setFeePercents")
            .argument(&total_fee_percent)
            .argument(&special_fee_percent)
            .original_result()
    }

    pub fn get_lp_token_identifier(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getLpTokenIdentifier")
            .original_result()
    }

    pub fn total_fee_percent(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getTotalFeePercent")
            .original_result()
    }

    pub fn special_fee_percent(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getSpecialFee")
            .original_result()
    }

    pub fn router_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getRouterManagedAddress")
            .original_result()
    }

    pub fn first_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getFirstTokenId")
            .original_result()
    }

    pub fn second_token_id(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getSecondTokenId")
            .original_result()
    }

    pub fn lp_token_supply(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getTotalSupply")
            .original_result()
    }

    pub fn initial_liquidity_adder(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, Option<ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getInitialLiquidtyAdder")
            .original_result()
    }

    pub fn pair_reserve<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getReserve")
            .argument(&token_id)
            .original_result()
    }

    pub fn safe_price_current_index(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, usize> {
        self.wrapped_tx
            .raw_call("getSafePriceCurrentIndex")
            .original_result()
    }

    pub fn get_lp_tokens_safe_price_by_default_offset<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        liquidity: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getLpTokensSafePriceByDefaultOffset")
            .argument(&pair_address)
            .argument(&liquidity)
            .original_result()
    }

    pub fn get_lp_tokens_safe_price_by_round_offset<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        round_offset: Arg1,
        liquidity: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getLpTokensSafePriceByRoundOffset")
            .argument(&pair_address)
            .argument(&round_offset)
            .argument(&liquidity)
            .original_result()
    }

    pub fn get_lp_tokens_safe_price_by_timestamp_offset<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        timestamp_offset: Arg1,
        liquidity: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getLpTokensSafePriceByTimestampOffset")
            .argument(&pair_address)
            .argument(&timestamp_offset)
            .argument(&liquidity)
            .original_result()
    }

    pub fn get_lp_tokens_safe_price<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        start_round: Arg1,
        end_round: Arg2,
        liquidity: Arg3,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getLpTokensSafePrice")
            .argument(&pair_address)
            .argument(&start_round)
            .argument(&end_round)
            .argument(&liquidity)
            .original_result()
    }

    pub fn get_safe_price_by_default_offset<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<EsdtTokenPayment<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        input_payment: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("getSafePriceByDefaultOffset")
            .argument(&pair_address)
            .argument(&input_payment)
            .original_result()
    }

    pub fn get_safe_price_by_round_offset<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<EsdtTokenPayment<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        round_offset: Arg1,
        input_payment: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("getSafePriceByRoundOffset")
            .argument(&pair_address)
            .argument(&round_offset)
            .argument(&input_payment)
            .original_result()
    }

    pub fn get_safe_price_by_timestamp_offset<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<EsdtTokenPayment<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        timestamp_offset: Arg1,
        input_payment: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("getSafePriceByTimestampOffset")
            .argument(&pair_address)
            .argument(&timestamp_offset)
            .argument(&input_payment)
            .original_result()
    }

    pub fn get_safe_price<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<EsdtTokenPayment<Env::Api>>,
    >(
        self,
        pair_address: Arg0,
        start_round: Arg1,
        end_round: Arg2,
        input_payment: Arg3,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("getSafePrice")
            .argument(&pair_address)
            .argument(&start_round)
            .argument(&end_round)
            .argument(&input_payment)
            .original_result()
    }

    pub fn get_price_observation_view<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        pair_address: Arg0,
        search_round: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, PriceObservation<Env::Api>> {
        self.wrapped_tx
            .raw_call("getPriceObservation")
            .argument(&pair_address)
            .argument(&search_round)
            .original_result()
    }

    pub fn update_and_get_tokens_for_given_position_with_safe_price<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        liquidity: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("updateAndGetTokensForGivenPositionWithSafePrice")
            .argument(&liquidity)
            .original_result()
    }

    pub fn update_and_get_safe_price<
        Arg0: ProxyArg<EsdtTokenPayment<Env::Api>>,
    >(
        self,
        input: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("updateAndGetSafePrice")
            .argument(&input)
            .original_result()
    }

    pub fn set_locking_deadline_epoch<
        Arg0: ProxyArg<u64>,
    >(
        self,
        new_deadline: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setLockingDeadlineEpoch")
            .argument(&new_deadline)
            .original_result()
    }

    pub fn set_locking_sc_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        new_address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setLockingScAddress")
            .argument(&new_address)
            .original_result()
    }

    pub fn set_unlock_epoch<
        Arg0: ProxyArg<u64>,
    >(
        self,
        new_epoch: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("setUnlockEpoch")
            .argument(&new_epoch)
            .original_result()
    }

    pub fn locking_sc_address(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("getLockingScAddress")
            .original_result()
    }

    pub fn unlock_epoch(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getUnlockEpoch")
            .original_result()
    }

    pub fn locking_deadline_epoch(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, u64> {
        self.wrapped_tx
            .raw_call("getLockingDeadlineEpoch")
            .original_result()
    }

    pub fn add_admin_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("addAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn remove_admin_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn update_owner_or_admin_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        previous_owner: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("updateOwnerOrAdmin")
            .argument(&previous_owner)
            .original_result()
    }

    pub fn permissions<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, permissions_module::permissions::Permissions> {
        self.wrapped_tx
            .raw_call("getPermissions")
            .argument(&address)
            .original_result()
    }

    pub fn add_to_pause_whitelist<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        address_list: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("addToPauseWhitelist")
            .argument(&address_list)
            .original_result()
    }

    pub fn remove_from_pause_whitelist<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        address_list: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeFromPauseWhitelist")
            .argument(&address_list)
            .original_result()
    }

    pub fn pause(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("pause")
            .original_result()
    }

    pub fn resume(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("resume")
            .original_result()
    }

    pub fn state(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, pausable::State> {
        self.wrapped_tx
            .raw_call("getState")
            .original_result()
    }

    pub fn add_initial_liquidity(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue3<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("addInitialLiquidity")
            .original_result()
    }

    pub fn add_liquidity<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        first_token_amount_min: Arg0,
        second_token_amount_min: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue3<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("addLiquidity")
            .argument(&first_token_amount_min)
            .argument(&second_token_amount_min)
            .original_result()
    }

    pub fn remove_liquidity<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        first_token_amount_min: Arg0,
        second_token_amount_min: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("removeLiquidity")
            .argument(&first_token_amount_min)
            .argument(&second_token_amount_min)
            .original_result()
    }

    pub fn remove_liquidity_and_burn_token<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_to_buyback_and_burn: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeLiquidityAndBuyBackAndBurnToken")
            .argument(&token_to_buyback_and_burn)
            .original_result()
    }

    pub fn swap_no_fee<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        token_out: Arg0,
        destination_address: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("swapNoFeeAndForward")
            .argument(&token_out)
            .argument(&destination_address)
            .original_result()
    }

    pub fn swap_tokens_fixed_input<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_out: Arg0,
        amount_out_min: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call("swapTokensFixedInput")
            .argument(&token_out)
            .argument(&amount_out_min)
            .original_result()
    }

    pub fn swap_tokens_fixed_output<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_out: Arg0,
        amount_out: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("swapTokensFixedOutput")
            .argument(&token_out)
            .argument(&amount_out)
            .original_result()
    }

    pub fn get_tokens_for_given_position<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        liquidity: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<EsdtTokenPayment<Env::Api>, EsdtTokenPayment<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getTokensForGivenPosition")
            .argument(&liquidity)
            .original_result()
    }

    pub fn get_reserves_and_total_supply(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue3<BigUint<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>>> {
        self.wrapped_tx
            .raw_call("getReservesAndTotalSupply")
            .original_result()
    }

    pub fn get_amount_out_view<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_in: Arg0,
        amount_in: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getAmountOut")
            .argument(&token_in)
            .argument(&amount_in)
            .original_result()
    }

    pub fn get_amount_in_view<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_wanted: Arg0,
        amount_wanted: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getAmountIn")
            .argument(&token_wanted)
            .argument(&amount_wanted)
            .original_result()
    }

    pub fn get_equivalent<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_in: Arg0,
        amount_in: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getEquivalent")
            .argument(&token_in)
            .argument(&amount_in)
            .original_result()
    }
}

#[derive(TopEncode, TopDecode)]
pub struct SwapEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub token_id_in: TokenIdentifier<Api>,
    pub token_amount_in: BigUint<Api>,
    pub token_id_out: TokenIdentifier<Api>,
    pub token_amount_out: BigUint<Api>,
    pub fee_amount: BigUint<Api>,
    pub token_in_reserve: BigUint<Api>,
    pub token_out_reserve: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[derive(TopEncode, TopDecode)]
pub struct SwapNoFeeAndForwardEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub token_id_in: TokenIdentifier<Api>,
    pub token_amount_in: BigUint<Api>,
    pub token_id_out: TokenIdentifier<Api>,
    pub token_amount_out: BigUint<Api>,
    pub destination: ManagedAddress<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[derive(TopEncode, TopDecode)]
pub struct AddLiquidityEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub first_token_id: TokenIdentifier<Api>,
    pub first_token_amount: BigUint<Api>,
    pub second_token_id: TokenIdentifier<Api>,
    pub second_token_amount: BigUint<Api>,
    pub lp_token_id: TokenIdentifier<Api>,
    pub lp_token_amount: BigUint<Api>,
    pub lp_supply: BigUint<Api>,
    pub first_token_reserves: BigUint<Api>,
    pub second_token_reserves: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[derive(TopEncode, TopDecode)]
pub struct RemoveLiquidityEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub first_token_id: TokenIdentifier<Api>,
    pub first_token_amount: BigUint<Api>,
    pub second_token_id: TokenIdentifier<Api>,
    pub second_token_amount: BigUint<Api>,
    pub lp_token_id: TokenIdentifier<Api>,
    pub lp_token_amount: BigUint<Api>,
    pub lp_supply: BigUint<Api>,
    pub first_token_reserves: BigUint<Api>,
    pub second_token_reserves: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[derive(TopEncode, TopDecode)]
pub struct PriceObservation<Api>
where
    Api: ManagedTypeApi,
{
    pub first_token_reserve_accumulated: BigUint<Api>,
    pub second_token_reserve_accumulated: BigUint<Api>,
    pub weight_accumulated: u64,
    pub recording_round: u64,
    pub lp_supply_accumulated: BigUint<Api>,
}