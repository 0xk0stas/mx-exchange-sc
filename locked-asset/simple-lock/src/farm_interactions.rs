elrond_wasm::imports!();
elrond_wasm::derive_imports!();

type EnterFarmResultType<BigUint> = EsdtTokenPayment<BigUint>;
type ExitFarmResultType<BigUint> =
    MultiValue2<EsdtTokenPayment<BigUint>, EsdtTokenPayment<BigUint>>;
type ClaimRewardsResultType<BigUint> =
    MultiValue2<EsdtTokenPayment<BigUint>, EsdtTokenPayment<BigUint>>;
type CompoundRewardsResultType<BigUint> = EsdtTokenPayment<BigUint>;

pub struct EnterFarmResultWrapper<M: ManagedTypeApi> {
    pub farm_tokens: EsdtTokenPayment<M>,
}

pub struct ExitFarmResultWrapper<M: ManagedTypeApi> {
    pub initial_farming_tokens: EsdtTokenPayment<M>,
    pub reward_tokens: EsdtTokenPayment<M>,
}

pub struct FarmClaimRewardsResultWrapper<M: ManagedTypeApi> {
    pub new_farm_tokens: EsdtTokenPayment<M>,
    pub reward_tokens: EsdtTokenPayment<M>,
}

pub struct FarmCompoundRewardsResultWrapper<M: ManagedTypeApi> {
    pub new_farm_tokens: EsdtTokenPayment<M>,
}

mod farm_proxy {
    elrond_wasm::imports!();
    use super::{
        ClaimRewardsResultType, CompoundRewardsResultType, EnterFarmResultType, ExitFarmResultType,
    };

    #[elrond_wasm::proxy]
    pub trait FarmProxy {
        #[payable("*")]
        #[endpoint(enterFarm)]
        fn enter_farm(&self) -> EnterFarmResultType<Self::Api>;

        #[payable("*")]
        #[endpoint(exitFarm)]
        fn exit_farm(&self) -> ExitFarmResultType<Self::Api>;

        #[payable("*")]
        #[endpoint(claimRewards)]
        fn claim_rewards(&self) -> ClaimRewardsResultType<Self::Api>;

        #[payable("*")]
        #[endpoint(compoundRewards)]
        fn compound_rewards(&self) -> CompoundRewardsResultType<Self::Api>;
    }
}

#[elrond_wasm::module]
pub trait FarmInteractionsModule {
    fn call_farm_enter(
        &self,
        farm_address: ManagedAddress,
        farming_token: TokenIdentifier,
        farming_token_amount: BigUint,
    ) -> EnterFarmResultWrapper<Self::Api> {
        let farm_tokens: EnterFarmResultType<Self::Api> = self
            .farm_proxy(farm_address)
            .enter_farm()
            .add_token_transfer(farming_token, 0, farming_token_amount)
            .execute_on_dest_context_custom_range(|_, after| (after - 1, after));

        EnterFarmResultWrapper { farm_tokens }
    }

    fn call_farm_exit(
        &self,
        farm_address: ManagedAddress,
        farm_token: TokenIdentifier,
        farm_token_nonce: u64,
        farm_token_amount: BigUint,
    ) -> ExitFarmResultWrapper<Self::Api> {
        let exit_farm_result: ExitFarmResultType<Self::Api> = self
            .farm_proxy(farm_address)
            .exit_farm()
            .add_token_transfer(farm_token, farm_token_nonce, farm_token_amount)
            .execute_on_dest_context_custom_range(|_, after| (after - 2, after));

        let (initial_farming_tokens, reward_tokens) = exit_farm_result.into_tuple();
        ExitFarmResultWrapper {
            initial_farming_tokens,
            reward_tokens,
        }
    }

    fn call_farm_claim_rewards(
        &self,
        farm_address: ManagedAddress,
        farm_token: TokenIdentifier,
        farm_token_nonce: u64,
        farm_token_amount: BigUint,
    ) -> FarmClaimRewardsResultWrapper<Self::Api> {
        let farm_claim_rewards_result: ClaimRewardsResultType<Self::Api> = self
            .farm_proxy(farm_address)
            .claim_rewards()
            .add_token_transfer(farm_token, farm_token_nonce, farm_token_amount)
            .execute_on_dest_context_custom_range(|_, after| (after - 2, after));

        let (new_farm_tokens, reward_tokens) = farm_claim_rewards_result.into_tuple();
        FarmClaimRewardsResultWrapper {
            new_farm_tokens,
            reward_tokens,
        }
    }

    fn call_farm_compound_rewards(
        &self,
        farm_address: ManagedAddress,
        farm_token: TokenIdentifier,
        farm_token_nonce: u64,
        farm_token_amount: BigUint,
    ) -> FarmCompoundRewardsResultWrapper<Self::Api> {
        let new_farm_tokens: CompoundRewardsResultType<Self::Api> = self
            .farm_proxy(farm_address)
            .compound_rewards()
            .add_token_transfer(farm_token, farm_token_nonce, farm_token_amount)
            .execute_on_dest_context_custom_range(|_, after| (after - 1, after));

        FarmCompoundRewardsResultWrapper { new_farm_tokens }
    }

    #[proxy]
    fn farm_proxy(&self, sc_address: ManagedAddress) -> farm_proxy::Proxy<Self::Api>;
}