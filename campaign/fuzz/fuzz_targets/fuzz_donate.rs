#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use campaign::types::{AssetInfo, StellarAsset, MilestoneData, CampaignStatus};
use campaign::CampaignContract;
use soroban_sdk::{Env, Address, String, Vec, BytesN};
use soroban_sdk::testutils::arbitrary::*;

#[derive(Arbitrary, Debug)]
struct DonateInput {
    amount: i128,
    asset_code: [u8; 12], // Max asset code length
}

fuzz_target!(|input: DonateInput| {
    let env = Env::default();
    
    // Register the contract
    let contract_id = env.register_contract(None, CampaignContract);
    let client = campaign::CampaignContractClient::new(&env, &contract_id);
    
    // Create test addresses
    let creator = Address::from_array(&[0x1; 32]);
    let donor = Address::from_array(&[0x2; 32]);
    
    // Create a valid asset
    let issuer = Address::from_array(&[0x3; 32]);
    let mut asset_code = String::from_str(&env, "USDC");
    // Use the fuzzed asset code (truncate if needed)
    if let Ok(s) = core::str::from_utf8(&input.asset_code) {
        asset_code = String::from_str(&env, s.trim_matches('\0'));
    }
    
    let mut accepted_assets = Vec::new(&env);
    accepted_assets.push_back(StellarAsset {
        asset_code,
        issuer: Some(issuer.clone()),
    });
    
    // Create valid milestones
    let mut milestones = Vec::new(&env);
    milestones.push_back(MilestoneData {
        title: String::from_str(&env, "First Milestone"),
        description: String::from_str(&env, "First milestone description"),
        target_amount: 500_000,
        released_amount: 0,
        status: campaign::types::MilestoneStatus::Locked,
        deadline: env.ledger().timestamp() + 86400 * 30,
        released_at: None,
        released_to: None,
    });
    milestones.push_back(MilestoneData {
        title: String::from_str(&env, "Final Milestone"),
        description: String::from_str(&env, "Final milestone description"),
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
    
    // Create asset info for donation - AssetInfo::Stellar wraps the token address
    let asset_info = AssetInfo::Stellar(issuer);
    
    // Attempt to donate with fuzzed amount - this should not panic outside of expected errors
    let result = std::panic::catch_unwind(|| {
        client.donate(&donor, &input.amount, &asset_info);
    });
    
    // If it panicked, check that it's one of our expected error codes
    if let Err(panic) = result {
        let panic_msg = format!("{:?}", panic);
        // The contract should only panic with our defined errors, never with unexpected panics
        assert!(
            panic_msg.contains("CampaignNotActive") ||
            panic_msg.contains("DonationTooSmall") ||
            panic_msg.contains("Overflow") ||
            panic_msg.contains("NotInitialized") ||
            panic_msg.contains("ContractFrozen"),
            "Unexpected panic in donate function: {}", panic_msg
        );
    }
});