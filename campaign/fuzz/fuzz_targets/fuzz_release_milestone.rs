#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use campaign::types::{AssetInfo, StellarAsset, MilestoneData};
use campaign::CampaignContract;
use soroban_sdk::{Env, Address, String, Vec};
use soroban_sdk::testutils::arbitrary::*;

#[derive(Arbitrary, Debug)]
struct ReleaseMilestoneInput {
    milestone_index: u32,
}

fuzz_target!(|input: ReleaseMilestoneInput| {
    let env = Env::default();
    
    // Register the contract
    let contract_id = env.register_contract(None, CampaignContract);
    let client = campaign::CampaignContractClient::new(&env, &contract_id);
    
    // Create test addresses
    let creator = Address::from_array(&[0x1; 32]);
    let recipient = Address::from_array(&[0x4; 32]);
    
    // Create a valid asset
    let issuer = Address::from_array(&[0x3; 32]);
    let mut accepted_assets = Vec::new(&env);
    accepted_assets.push_back(StellarAsset {
        asset_code: String::from_str(&env, "USDC"),
        issuer: Some(issuer.clone()),
    });
    
    // Create valid milestones (max 5 as per types::MAX_MILESTONES)
    let mut milestones = Vec::new(&env);
    milestones.push_back(MilestoneData {
        title: String::from_str(&env, "First Milestone"),
        description: String::from_str(&env, "First milestone description"),
        target_amount: 500_000,
        released_amount: 0,
        status: campaign::types::MilestoneStatus::Unlocked, // Start with first unlocked
        deadline: env.ledger().timestamp() + 86400 * 30,
        released_at: None,
        released_to: None,
    });
    milestones.push_back(MilestoneData {
        title: String::from_str(&env, "Second Milestone"),
        description: String::from_str(&env, "Second milestone description"),
        target_amount: 1_000_000,
        released_amount: 0,
        status: campaign::types::MilestoneStatus::Locked,
        deadline: env.ledger().timestamp() + 86400 * 60,
        released_at: None,
        released_to: None,
    });
    
    // Initialize the contract
    let _ = client.initialize(
        &creator,
        &1_000_000, // goal amount
        &(env.ledger().timestamp() + 86400 * 90), // end time (90 days from now)
        &accepted_assets,
        &milestones,
        &100, // min donation amount
    );
    
    // Make a donation to unlock milestones if needed
    let donor = Address::from_array(&[0x2; 32]);
    let asset_info = AssetInfo::Stellar(issuer);
    // Fund the contract with enough tokens
    let _ = client.donate(&donor, &600_000, &asset_info); // This should unlock the first milestone
    
    // Attempt to release milestone with fuzzed index - this should not panic outside of expected errors
    let result = std::panic::catch_unwind(|| {
        client.release_milestone(&input.milestone_index, &recipient);
    });
    
    // If it panicked, check that it's one of our expected error codes
    if let Err(panic) = result {
        let panic_msg = format!("{:?}", panic);
        // The contract should only panic with our defined errors, never with unexpected panics
        assert!(
            panic_msg.contains("MilestoneNotFound") ||
            panic_msg.contains("InvalidMilestoneTransition") ||
            panic_msg.contains("MilestoneAlreadyReleased") ||
            panic_msg.contains("PreviousMilestoneNotReleased") ||
            panic_msg.contains("NotInitialized") ||
            panic_msg.contains("InsufficientContractBalance") ||
            panic_msg.contains("ContractFrozen") ||
            panic_msg.contains("Overflow"),
            "Unexpected panic in release_milestone function: {}", panic_msg
        );
    }
});