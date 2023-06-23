mod gov_test_setup;

use gov_test_setup::*;
use governance_v2::{
    configurable::ConfigurablePropertiesModule, proposal::GovernanceProposalStatus,
    proposal_storage::ProposalStorageModule, views::ViewsModule,
};
use multiversx_sc::{codec::Empty, types::ManagedVec};
use multiversx_sc_scenario::{managed_biguint, managed_buffer, rust_biguint};

#[test]
fn init_gov_test() {
    let _ = GovSetup::new(governance_v2::contract_obj);
}

#[test]
fn gov_propose_test() {
    let mut gov_setup = GovSetup::new(governance_v2::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let second_user_addr = gov_setup.second_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    // Give proposer the minimum fee
    gov_setup.b_mock.set_nft_balance(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(MIN_FEE_FOR_PROPOSE),
        &Empty,
    );

    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        MIN_FEE_FOR_PROPOSE,
        &sc_addr,
        b"changeTODO",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    // vote too early
    gov_setup
        .up_vote(&second_user_addr, proposal_id)
        .assert_user_error("Proposal is not active");

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // user 2 vote
    gov_setup
        .up_vote(&second_user_addr, proposal_id)
        .assert_ok();

    // user 2 try vote again
    gov_setup
        .up_vote(&second_user_addr, proposal_id)
        .assert_user_error("Already voted for this proposal");

    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert!(
                sc.get_proposal_status(1) == GovernanceProposalStatus::Succeeded,
                "Action should have been Succeeded"
            );
        })
        .assert_ok();

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            let proposal = sc.proposals().get(1);
            let action = proposal.actions.get(0).unwrap();
            let mut args_managed = ManagedVec::new();
            args_managed.push(managed_buffer!(&1_000u64.to_be_bytes()));

            assert!(
                action.function_name == b"changeTODO",
                "Wrong Action - Endpoint Name"
            );
            assert!(action.arguments == args_managed, "Wrong Action - Arguments");
        })
        .assert_ok();
}

#[test]
fn gov_no_veto_vote_test() {
    let mut gov_setup = GovSetup::new(governance_v2::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let second_user_addr = gov_setup.second_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    // Give proposer the minimum fee
    gov_setup.b_mock.set_nft_balance(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(1_000),
        &Empty,
    );

    // UpVotes = 1_000
    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        MIN_FEE_FOR_PROPOSE,
        &sc_addr,
        b"changeTODO",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert_eq!(sc.quorum().get(), managed_biguint!(QUORUM_PERCENTAGE));
        })
        .assert_ok();

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // Second User UpVote = 1_000
    gov_setup
        .up_vote(&second_user_addr, proposal_id)
        .assert_ok();

    // Third User DownWithVetoVote = 1_100
    gov_setup
        .down_veto_vote(&third_user_addr, proposal_id)
        .assert_ok();

    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert!(
                sc.get_proposal_status(1) == GovernanceProposalStatus::DefeatedWithVeto,
                "Action should have been Defeated"
            );
        })
        .assert_ok();
}

#[test]
fn gov_abstain_vote_test() {
    let mut gov_setup = GovSetup::new(governance_v2::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    // Give proposer the minimum fee
    gov_setup.b_mock.set_nft_balance(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(MIN_FEE_FOR_PROPOSE),
        &Empty,
    );

    // First user Propose + UpVotes = 1_000
    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        MIN_FEE_FOR_PROPOSE,
        &sc_addr,
        b"changeTODO",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // Third user AbstainVotes = 1_100
    gov_setup
        .abstain_vote(&third_user_addr, proposal_id)
        .assert_ok();

    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert!(
                sc.get_proposal_status(1) == GovernanceProposalStatus::Defeated,
                "Action should have been Defeated"
            );
        })
        .assert_ok();
}

#[test]
fn gov_withdraw_defeated_proposal_test() {
    let mut gov_setup = GovSetup::new(governance_v2::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    gov_setup.b_mock.set_nft_balance(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(MIN_FEE_FOR_PROPOSE),
        &Empty,
    );

    // First user Propose + Up Vote
    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        MIN_FEE_FOR_PROPOSE,
        &sc_addr,
        b"changeTODO",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    // Check proposer balance
    gov_setup.b_mock.check_nft_balance::<Empty>(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(0),
        None,
    );

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // Third user Abstain Vote
    gov_setup
        .down_vote(&third_user_addr, proposal_id)
        .assert_ok();

    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert!(
                sc.get_proposal_status(1) == GovernanceProposalStatus::Defeated,
                "Action should have been Defeated"
            );
        })
        .assert_ok();

    // Other user (not proposer) try to withdraw the fee -> Fail
    gov_setup
        .withdraw_after_defeated(&third_user_addr, proposal_id)
        .assert_error(4, "Only original proposer may cancel a pending proposal");

    // Proposer withdraw
    gov_setup
        .withdraw_after_defeated(&first_user_addr, proposal_id)
        .assert_ok();

    // Check proposer balance (fee)
    gov_setup.b_mock.check_nft_balance::<Empty>(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(MIN_FEE_FOR_PROPOSE),
        None,
    );
}

#[test]
fn gov_withdraw_no_with_veto_defeated_proposal_test() {
    let mut gov_setup = GovSetup::new(governance_v2::contract_obj);

    let first_user_addr = gov_setup.first_user.clone();
    let third_user_addr = gov_setup.third_user.clone();
    let sc_addr = gov_setup.gov_wrapper.address_ref().clone();

    gov_setup.b_mock.set_nft_balance(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(MIN_FEE_FOR_PROPOSE),
        &Empty,
    );

    // First user Propose + Up Vote
    let (result, proposal_id) = gov_setup.propose(
        &first_user_addr,
        MIN_FEE_FOR_PROPOSE,
        &sc_addr,
        b"changeTODO",
        vec![1_000u64.to_be_bytes().to_vec()],
    );
    result.assert_ok();
    assert_eq!(proposal_id, 1);

    // Check proposer balance
    gov_setup.b_mock.check_nft_balance::<Empty>(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(0),
        None,
    );

    gov_setup.increment_block_nonce(VOTING_PERIOD_BLOCKS);

    // Third user Abstain Vote
    gov_setup
        .down_veto_vote(&third_user_addr, proposal_id)
        .assert_ok();

    gov_setup.increment_block_nonce(LOCKING_PERIOD_BLOCKS);

    gov_setup
        .b_mock
        .execute_query(&gov_setup.gov_wrapper, |sc| {
            assert!(
                sc.get_proposal_status(1) == GovernanceProposalStatus::DefeatedWithVeto,
                "Action should have been Defeated"
            );
        })
        .assert_ok();

    // Other user (not proposer) try to withdraw the fee -> Fail
    gov_setup
        .withdraw_after_defeated(&third_user_addr, proposal_id)
        .assert_error(4, "Only original proposer may cancel a pending proposal");

    // Proposer withdraw
    gov_setup
        .withdraw_after_defeated(&first_user_addr, proposal_id)
        .assert_ok();

    // Check proposer balance (fee)
    gov_setup.b_mock.check_nft_balance::<Empty>(
        &first_user_addr,
        WXMEX_TOKEN_ID,
        1,
        &rust_biguint!(MIN_FEE_FOR_PROPOSE / 2),
        None,
    );
}
