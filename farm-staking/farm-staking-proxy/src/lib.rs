#![no_std]

use result_types::{ClaimDualYieldResult, StakeProxyResult, UnstakeResult};

elrond_wasm::imports!();

pub mod dual_yield_token;
pub mod external_contracts_interactions;
pub mod lp_farm_token;
pub mod result_types;

#[elrond_wasm::contract]
pub trait FarmStakingProxy:
    dual_yield_token::DualYieldTokenModule
    + external_contracts_interactions::ExternalContractsInteractionsModule
    + lp_farm_token::LpFarmTokenModule
    + token_merge_helper::TokenMergeHelperModule
    + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + utils::UtilsModule
    + token_send::TokenSendModule
{
    #[init]
    fn init(
        &self,
        lp_farm_address: ManagedAddress,
        staking_farm_address: ManagedAddress,
        pair_address: ManagedAddress,
        staking_token_id: TokenIdentifier,
        lp_farm_token_id: TokenIdentifier,
        staking_farm_token_id: TokenIdentifier,
        lp_token_id: TokenIdentifier,
    ) {
        self.require_sc_address(&lp_farm_address);
        self.require_sc_address(&staking_farm_address);
        self.require_sc_address(&pair_address);

        self.require_valid_token_id(&staking_token_id);
        self.require_valid_token_id(&lp_farm_token_id);
        self.require_valid_token_id(&staking_farm_token_id);
        self.require_valid_token_id(&lp_token_id);

        self.lp_farm_address().set(&lp_farm_address);
        self.staking_farm_address().set(&staking_farm_address);
        self.pair_address().set(&pair_address);

        self.staking_token_id().set_if_empty(&staking_token_id);
        self.lp_farm_token_id().set_if_empty(&lp_farm_token_id);
        self.staking_farm_token_id()
            .set_if_empty(&staking_farm_token_id);
        self.lp_token_id().set_if_empty(&lp_token_id);
    }

    #[payable("*")]
    #[endpoint(stakeFarmTokens)]
    fn stake_farm_tokens(&self) -> StakeProxyResult<Self::Api> {
        let payments = self.get_non_empty_payments();
        let lp_farm_token_payment = payments.get(0);
        let additional_payments = payments.slice(1, payments.len()).unwrap_or_default();

        let lp_farm_token_id = self.lp_farm_token_id().get();
        require!(
            lp_farm_token_payment.token_identifier == lp_farm_token_id,
            "Invalid first payment"
        );
        self.dual_yield_token()
            .require_all_same_token(&additional_payments);

        let staking_farm_token_id = self.staking_farm_token_id().get();
        let mut additional_staking_farm_tokens = ManagedVec::new();
        let mut additional_lp_farm_tokens = ManagedVec::new();
        for p in &additional_payments {
            let attributes = self.get_dual_yield_token_attributes(p.token_nonce);

            additional_staking_farm_tokens.push(EsdtTokenPayment::new(
                staking_farm_token_id.clone(),
                attributes.staking_farm_token_nonce,
                self.get_staking_farm_token_amount_equivalent(&p.amount),
            ));

            additional_lp_farm_tokens.push(EsdtTokenPayment::new(
                lp_farm_token_id.clone(),
                attributes.lp_farm_token_nonce,
                self.get_lp_farm_token_amount_equivalent(&attributes, &p.amount),
            ));

            self.burn_dual_yield_tokens(p.token_nonce, &p.amount);
        }

        let lp_tokens_in_farm = self.get_lp_tokens_in_farm_position(
            lp_farm_token_payment.token_nonce,
            &lp_farm_token_payment.amount,
        );
        let staking_token_amount = self.get_lp_tokens_safe_price(lp_tokens_in_farm);
        let staking_farm_enter_result =
            self.staking_farm_enter(staking_token_amount, additional_staking_farm_tokens);
        let received_staking_farm_token = staking_farm_enter_result.received_staking_farm_token;

        let caller = self.blockchain().get_caller();
        let merged_lp_farm_tokens = self.merge_lp_farm_tokens(
            caller.clone(),
            lp_farm_token_payment,
            additional_lp_farm_tokens,
        );

        let new_dual_yield_tokens = self.create_dual_yield_tokens(
            merged_lp_farm_tokens.token_nonce,
            merged_lp_farm_tokens.amount,
            received_staking_farm_token.token_nonce,
            received_staking_farm_token.amount,
        );
        let output_payments = StakeProxyResult {
            dual_yield_tokens: new_dual_yield_tokens,
            boosted_rewards: staking_farm_enter_result.boosted_rewards,
        };

        output_payments.send_and_return(self, &caller)
    }

    #[payable("*")]
    #[endpoint(claimDualYield)]
    fn claim_dual_yield(&self) -> ClaimDualYieldResult<Self::Api> {
        let (payment_token, payment_nonce, payment_amount) =
            self.call_value().single_esdt().into_tuple();
        self.dual_yield_token().require_same_token(&payment_token);

        let attributes = self.get_dual_yield_token_attributes(payment_nonce);

        let staking_farm_token_id = self.staking_farm_token_id().get();
        let staking_farm_token_nonce = attributes.staking_farm_token_nonce;
        let staking_farm_token_amount =
            self.get_staking_farm_token_amount_equivalent(&payment_amount);

        let lp_farm_token_id = self.lp_farm_token_id().get();
        let lp_farm_token_nonce = attributes.lp_farm_token_nonce;
        let lp_farm_token_amount =
            self.get_lp_farm_token_amount_equivalent(&attributes, &payment_amount);

        let lp_tokens_in_position =
            self.get_lp_tokens_in_farm_position(lp_farm_token_nonce, &lp_farm_token_amount);
        let new_staking_farm_value = self.get_lp_tokens_safe_price(lp_tokens_in_position);

        self.burn_dual_yield_tokens(payment_nonce, &payment_amount);

        let lp_farm_claim_rewards_result =
            self.lp_farm_claim_rewards(lp_farm_token_id, lp_farm_token_nonce, lp_farm_token_amount);
        let staking_farm_claim_rewards_result = self.staking_farm_claim_rewards(
            staking_farm_token_id,
            staking_farm_token_nonce,
            staking_farm_token_amount,
            new_staking_farm_value,
        );

        let new_lp_farm_tokens = lp_farm_claim_rewards_result.new_lp_farm_tokens;
        let new_staking_farm_tokens = staking_farm_claim_rewards_result.new_staking_farm_tokens;
        let new_dual_yield_tokens = self.create_dual_yield_tokens(
            new_lp_farm_tokens.token_nonce,
            new_lp_farm_tokens.amount,
            new_staking_farm_tokens.token_nonce,
            new_staking_farm_tokens.amount,
        );

        let caller = self.blockchain().get_caller();
        let claim_result = ClaimDualYieldResult {
            lp_farm_rewards: lp_farm_claim_rewards_result.lp_farm_rewards,
            staking_farm_rewards: staking_farm_claim_rewards_result.staking_farm_rewards,
            new_dual_yield_tokens,
        };

        claim_result.send_and_return(self, &caller)
    }

    #[payable("*")]
    #[endpoint(unstakeFarmTokens)]
    fn unstake_farm_tokens(
        &self,
        pair_first_token_min_amount: BigUint,
        pair_second_token_min_amount: BigUint,
        exit_amount: BigUint,
    ) -> UnstakeResult<Self::Api> {
        let (payment_token, payment_nonce, payment_amount) =
            self.call_value().single_esdt().into_tuple();
        self.dual_yield_token().require_same_token(&payment_token);

        require!(payment_amount >= exit_amount, "Invalid exit amount");

        let attributes = self.get_dual_yield_token_attributes(payment_nonce);
        let full_lp_farm_token_amount =
            self.get_lp_farm_token_amount_equivalent(&attributes, &payment_amount);
        let lp_farm_exit_amount =
            self.get_lp_farm_token_amount_equivalent(&attributes, &exit_amount);
        let lp_farm_exit_result = self.lp_farm_exit(
            attributes.lp_farm_token_nonce,
            full_lp_farm_token_amount,
            lp_farm_exit_amount,
        );

        let remove_liq_result = self.pair_remove_liquidity(
            lp_farm_exit_result.lp_tokens,
            pair_first_token_min_amount,
            pair_second_token_min_amount,
        );

        let full_staking_farm_token_amount =
            self.get_staking_farm_token_amount_equivalent(&payment_amount);
        let staking_farm_exit_amount = self.get_staking_farm_token_amount_equivalent(&exit_amount);
        let staking_farm_exit_result = self.staking_farm_unstake(
            remove_liq_result.staking_token_payment,
            attributes.staking_farm_token_nonce,
            full_staking_farm_token_amount,
            staking_farm_exit_amount,
        );

        let caller = self.blockchain().get_caller();
        let unstake_result = UnstakeResult {
            other_token_payment: remove_liq_result.other_token_payment,
            lp_farm_rewards: lp_farm_exit_result.lp_farm_rewards,
            staking_rewards: staking_farm_exit_result.staking_rewards,
            unbond_staking_farm_token: staking_farm_exit_result.unbond_staking_farm_token,
        };

        self.burn_dual_yield_tokens(payment_nonce, &payment_amount);

        unstake_result.send_and_return(self, &caller)
    }
}
