elrond_wasm::imports!();

use common_errors::{ERROR_DIFFERENT_ATTRIBUTES_FOR_MERGE, ERROR_EMPTY_PAYMENTS};
use common_structs::{Nonce, PaymentsVec};
use weekly_rewards_splitting::ClaimProgress;

#[elrond_wasm::module]
pub trait ClaimProgressModule:
    farm_boosted_yields::FarmBoostedYieldsModule
    + permissions_module::PermissionsModule
    + energy_query::EnergyQueryModule
    + week_timekeeping::WeekTimekeepingModule
    + weekly_rewards_splitting::events::WeeklyRewardsSplittingEventsModule
    + weekly_rewards_splitting::WeeklyRewardsSplittingModule
{
    // Clears the claim progress for all the additional payment tokens
    // in order to be able to merge the users claim progress under a single token nonce
    fn clear_payments_claim_progress(
        &self,
        user: &ManagedAddress,
        all_payments: &PaymentsVec<Self::Api>,
    ) -> Nonce {
        require!(!all_payments.is_empty(), ERROR_EMPTY_PAYMENTS);
        if all_payments.len() > 1 {
            let mut additional_payments = all_payments.clone();
            let first_payment = additional_payments.get(0);
            additional_payments.remove(0);
            let first_claim_progress =
                self.get_claim_progress_or_default(user, first_payment.token_nonce);
            let first_payment_claim_week = first_claim_progress.week;
            let first_payment_energy = first_claim_progress.energy;
            for payment in additional_payments.iter() {
                let payment_claim_progress =
                    self.get_claim_progress_and_clear(user, payment.token_nonce);
                require!(
                    first_payment_claim_week == payment_claim_progress.week
                        && first_payment_energy == payment_claim_progress.energy,
                    ERROR_DIFFERENT_ATTRIBUTES_FOR_MERGE
                );
            }
        }
        return all_payments.get(0).token_nonce;
    }

    fn update_user_claim_progress(
        &self,
        user: &ManagedAddress,
        old_nonce: OptionalValue<Nonce>,
        new_nonce: Nonce,
    ) {
        match old_nonce {
            OptionalValue::Some(old_nonce) => {
                let old_claim_progress_mapper = self.farm_claim_progress(user, old_nonce);
                if old_claim_progress_mapper.is_empty() {
                    let new_claim_progress = self.new_claim_progress_for_user(user);
                    self.farm_claim_progress(user, new_nonce)
                        .set(new_claim_progress);
                } else {
                    self.farm_claim_progress(user, new_nonce)
                        .set(old_claim_progress_mapper.get());
                    old_claim_progress_mapper.clear();
                }
            }
            OptionalValue::None => {
                let new_claim_progress = self.new_claim_progress_for_user(user);
                self.farm_claim_progress(user, new_nonce)
                    .set(new_claim_progress);
            }
        }
    }

    fn get_claim_progress_or_default(
        &self,
        user: &ManagedAddress,
        token_nonce: Nonce,
    ) -> ClaimProgress<Self::Api> {
        let current_claim_mapper = self.farm_claim_progress(user, token_nonce);
        let claim_progress;
        if current_claim_mapper.is_empty() {
            claim_progress = self.new_claim_progress_for_user(user);
        } else {
            claim_progress = current_claim_mapper.get();
        }

        claim_progress
    }

    fn get_claim_progress_and_clear(
        &self,
        user: &ManagedAddress,
        token_nonce: Nonce,
    ) -> ClaimProgress<Self::Api> {
        let current_claim_mapper = self.farm_claim_progress(user, token_nonce);
        let claim_progress;
        if current_claim_mapper.is_empty() {
            claim_progress = self.new_claim_progress_for_user(user);
        } else {
            claim_progress = current_claim_mapper.get();
            current_claim_mapper.clear();
        }

        claim_progress
    }

    fn new_claim_progress_for_user(&self, user: &ManagedAddress) -> ClaimProgress<Self::Api> {
        let current_week = self.get_current_week();
        let current_user_energy = self.get_energy_entry(user.clone());
        self.update_user_energy_for_current_week(user, current_week, &current_user_energy);
        ClaimProgress {
            energy: current_user_energy,
            week: current_week,
        }
    }
}
