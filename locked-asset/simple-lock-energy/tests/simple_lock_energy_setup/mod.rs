use elrond_wasm::{
    elrond_codec::multi_types::OptionalValue,
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole, MultiValueEncoded},
};
use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper},
    tx_mock::TxResult,
    DebugApi,
};
use elrond_wasm_modules::pause::PauseModule;
use simple_lock::locked_token::LockedTokenModule;
use simple_lock_energy::{
    energy::EnergyModule, lock_options::LockOptionsModule,
    unlock_with_penalty::UnlockWithPenaltyModule, SimpleLockEnergy,
};

mod fees_collector_mock;
use fees_collector_mock::*;

pub const EPOCHS_IN_YEAR: u64 = 360;
pub const EPOCHS_IN_WEEK: u64 = 7;
pub const USER_BALANCE: u64 = 1_000_000_000_000_000_000;

pub static BASE_ASSET_TOKEN_ID: &[u8] = b"MEX-123456";
pub static LOCKED_TOKEN_ID: &[u8] = b"LOCKED-123456";
pub static LEGACY_LOCKED_TOKEN_ID: &[u8] = b"LEGACY-123456";

pub const FIRST_THRESHOLD_PERCENTAGE: u16 = 4_000;
pub const SECOND_THRESHOLD_PERCENTAGE: u16 = 6_000;
pub const THIRD_THRESHOLD_PERCENTAGE: u16 = 8_000;
pub const FEES_BURN_PERCENTAGE: u16 = 5_000; // 50%
pub static LOCK_OPTIONS: &[u64] = &[EPOCHS_IN_YEAR, 2 * EPOCHS_IN_YEAR, 4 * EPOCHS_IN_YEAR]; // 1, 2 or 4 years

pub struct SimpleLockEnergySetup<ScBuilder>
where
    ScBuilder: 'static + Copy + Fn() -> simple_lock_energy::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner: Address,
    pub first_user: Address,
    pub second_user: Address,
    pub sc_wrapper: ContractObjWrapper<simple_lock_energy::ContractObj<DebugApi>, ScBuilder>,
    pub fees_collector_mock: Address,
}

impl<ScBuilder> SimpleLockEnergySetup<ScBuilder>
where
    ScBuilder: 'static + Copy + Fn() -> simple_lock_energy::ContractObj<DebugApi>,
{
    pub fn new(sc_builder: ScBuilder) -> Self {
        let _ = DebugApi::dummy();
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner = b_mock.create_user_account(&rust_zero);
        let first_user = b_mock.create_user_account(&rust_zero);
        let second_user = b_mock.create_user_account(&rust_zero);
        let sc_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner), sc_builder, "simple lock energy");
        let fees_collector_mock = b_mock.create_sc_account(
            &rust_zero,
            Some(&owner),
            FeesCollectorMock::new,
            "fees collector mock",
        );

        b_mock
            .execute_tx(&owner, &sc_wrapper, &rust_zero, |sc| {
                let mut lock_options = MultiValueEncoded::new();
                for option in LOCK_OPTIONS {
                    lock_options.push(*option);
                }

                // fees_collector_mock address used twice, as we don't test migration here
                // migration is tested in old_to_new_locked_token_migration_test.rs
                sc.init(
                    managed_token_id!(BASE_ASSET_TOKEN_ID),
                    managed_token_id!(LEGACY_LOCKED_TOKEN_ID),
                    FIRST_THRESHOLD_PERCENTAGE,
                    SECOND_THRESHOLD_PERCENTAGE,
                    THIRD_THRESHOLD_PERCENTAGE,
                    FEES_BURN_PERCENTAGE,
                    managed_address!(fees_collector_mock.address_ref()),
                    managed_address!(fees_collector_mock.address_ref()),
                    lock_options,
                );

                assert_eq!(sc.max_lock_option().get(), *LOCK_OPTIONS.last().unwrap());

                sc.locked_token()
                    .set_token_id(managed_token_id!(LOCKED_TOKEN_ID));
                sc.set_paused(false);
            })
            .assert_ok();

        b_mock.set_esdt_local_roles(
            sc_wrapper.address_ref(),
            BASE_ASSET_TOKEN_ID,
            &[EsdtLocalRole::Mint, EsdtLocalRole::Burn],
        );
        b_mock.set_esdt_local_roles(
            sc_wrapper.address_ref(),
            LOCKED_TOKEN_ID,
            &[
                EsdtLocalRole::NftCreate,
                EsdtLocalRole::NftAddQuantity,
                EsdtLocalRole::NftBurn,
                EsdtLocalRole::Transfer,
            ],
        );
        b_mock.set_esdt_local_roles(
            sc_wrapper.address_ref(),
            LEGACY_LOCKED_TOKEN_ID,
            &[EsdtLocalRole::NftBurn],
        );

        b_mock.set_esdt_balance(
            &first_user,
            BASE_ASSET_TOKEN_ID,
            &rust_biguint!(USER_BALANCE),
        );
        b_mock.set_esdt_balance(
            &second_user,
            BASE_ASSET_TOKEN_ID,
            &rust_biguint!(USER_BALANCE),
        );

        Self {
            b_mock,
            owner,
            first_user,
            second_user,
            sc_wrapper,
            fees_collector_mock: fees_collector_mock.address_ref().clone(),
        }
    }
}

impl<ScBuilder> SimpleLockEnergySetup<ScBuilder>
where
    ScBuilder: 'static + Copy + Fn() -> simple_lock_energy::ContractObj<DebugApi>,
{
    pub fn lock(
        &mut self,
        caller: &Address,
        token_id: &[u8],
        amount: u64,
        lock_epochs: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            caller,
            &self.sc_wrapper,
            token_id,
            0,
            &rust_biguint!(amount),
            |sc| {
                sc.lock_tokens_endpoint(lock_epochs, OptionalValue::Some(managed_address!(caller)));
            },
        )
    }

    pub fn extend_locking_period(
        &mut self,
        caller: &Address,
        token_id: &[u8],
        token_nonce: u64,
        amount: u64,
        lock_epochs: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            caller,
            &self.sc_wrapper,
            token_id,
            token_nonce,
            &rust_biguint!(amount),
            |sc| {
                sc.lock_tokens_endpoint(lock_epochs, OptionalValue::None);
            },
        )
    }

    pub fn unlock(&mut self, caller: &Address, token_nonce: u64, amount: u64) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            caller,
            &self.sc_wrapper,
            LOCKED_TOKEN_ID,
            token_nonce,
            &rust_biguint!(amount),
            |sc| {
                sc.unlock_tokens_endpoint();
            },
        )
    }

    pub fn unlock_early(&mut self, caller: &Address, token_nonce: u64, amount: u64) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            caller,
            &self.sc_wrapper,
            LOCKED_TOKEN_ID,
            token_nonce,
            &rust_biguint!(amount),
            |sc| {
                sc.unlock_early();
            },
        )
    }

    pub fn reduce_lock_period(
        &mut self,
        caller: &Address,
        token_nonce: u64,
        amount: u64,
        epochs_to_reduce: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            caller,
            &self.sc_wrapper,
            LOCKED_TOKEN_ID,
            token_nonce,
            &rust_biguint!(amount),
            |sc| {
                sc.reduce_lock_period(epochs_to_reduce);
            },
        )
    }

    pub fn send_fees_to_collector(
        &mut self,
        caller: &Address,
        token_nonce: u64,
        amount: u64,
    ) -> TxResult {
        self.b_mock.execute_esdt_transfer(
            caller,
            &self.sc_wrapper,
            LOCKED_TOKEN_ID,
            token_nonce,
            &rust_biguint!(amount),
            |sc| {
                sc.send_fees_to_collector();
            },
        )
    }

    pub fn get_penalty_amount(
        &mut self,
        token_amount: u64,
        epochs_to_reduce: u64,
        current_unlock_epoch: u64,
    ) -> num_bigint::BigUint {
        let mut result = rust_biguint!(0);
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                let managed_result =
                    sc.calculate_penalty_amount(&managed_biguint!(token_amount), epochs_to_reduce, current_unlock_epoch);
                result = to_rust_biguint(managed_result);
            })
            .assert_ok();

        result
    }

    pub fn get_user_energy(&mut self, user: &Address) -> num_bigint::BigUint {
        let mut result = rust_biguint!(0);
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                let managed_result = sc.get_energy_amount_for_user(managed_address!(user));
                result = to_rust_biguint(managed_result);
            })
            .assert_ok();

        result
    }
}

pub fn to_rust_biguint(
    managed_biguint: elrond_wasm::types::BigUint<DebugApi>,
) -> num_bigint::BigUint {
    num_bigint::BigUint::from_bytes_be(managed_biguint.to_bytes_be().as_slice())
}

pub fn to_start_of_month(unlock_epoch: u64) -> u64 {
    unlock_epoch - unlock_epoch % 30
}
