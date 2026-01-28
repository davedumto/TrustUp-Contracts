use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

use crate::ReputationContract;
use crate::ReputationContractClient;

/// Test: Sets the contract admin
/// Verifies that an address can be assigned as the contract administrator.
/// Receives: Admin Address. Returns: void. Validates that the admin is stored correctly.
#[test]
fn it_sets_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let retrieved_admin = client.get_admin();
    assert_eq!(retrieved_admin, admin);
}

/// Test: Gets the contract admin
/// Verifies that the current contract administrator can be queried.
/// Receives: nothing. Returns: Admin Address. Validates that it returns the correct address.
#[test]
fn it_gets_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let retrieved = client.get_admin();
    assert_eq!(retrieved, admin);
}

/// Test: Assigns updater permissions
/// Verifies that the admin can grant updater permissions to an address.
/// Receives: Admin Address, Updater Address, bool allowed. Returns: void. Validates that the updater is authorized.
#[test]
fn it_sets_updater() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    assert!(client.is_updater(&updater));
}

/// Test: Checks updater permissions
/// Verifies that it can be queried whether an address has updater permissions or not.
/// Receives: Address to check. Returns: bool (true if updater, false if not). Validates both cases.
#[test]
fn it_checks_updater() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    let non_updater = Address::generate(&env);

    client.set_updater(&admin, &updater, &true);

    assert!(client.is_updater(&updater));
    assert!(!client.is_updater(&non_updater));
}

/// Test: Gets the reputation score
/// Verifies that a user's score can be queried. New users have a score of 0.
/// Receives: User Address. Returns: u32 (score 0-100). Validates initial read and after set.
#[test]
fn it_gets_score() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    // New user should have score 0
    assert_eq!(client.get_score(&user), 0);

    // Set score and verify
    client.set_score(&updater, &user, &50);
    assert_eq!(client.get_score(&user), 50);
}

/// Test: Increases the reputation score
/// Verifies that an authorized updater can increase a user's score.
/// Receives: Updater Address, User Address, u32 amount. Returns: void. Validates that the score increases correctly.
#[test]
fn it_increases_score() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &50);
    client.increase_score(&updater, &user, &20);

    assert_eq!(client.get_score(&user), 70);
}

/// Test: Decreases the reputation score
/// Verifies that an authorized updater can decrease a user's score.
/// Receives: Updater Address, User Address, u32 amount. Returns: void. Validates that the score decreases correctly.
#[test]
fn it_decreases_score() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &50);
    client.decrease_score(&updater, &user, &20);

    assert_eq!(client.get_score(&user), 30);
}

/// Test: Sets the score to a specific value
/// Verifies that an authorized updater can set a user's score to any valid value.
/// Receives: Updater Address, User Address, u32 new_score. Returns: void. Validates multiple score changes.
#[test]
fn it_sets_score() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &75);
    assert_eq!(client.get_score(&user), 75);

    client.set_score(&updater, &user, &25);
    assert_eq!(client.get_score(&user), 25);
}

/// Test: Prevents unauthorized updates
/// Verifies that only authorized updaters can modify scores. Users without permissions must fail.
/// Receives: Unauthorized Address attempting to update. Returns: panic with NotUpdater error (#2).
#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn it_prevents_unauthorized_updates() {
    let env = Env::default();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin);

    let user = Address::generate(&env);
    let unauthorized = Address::generate(&env);

    // Try to update score without being an updater (should panic)
    client.mock_all_auths().set_score(&unauthorized, &user, &50);
}

/// Test: Validates score bounds (0-100)
/// Verifies that scores cannot be set outside the valid range (0-100).
/// Receives: Invalid score (>100 or <0). Returns: panic with OutOfBounds error (#3). Protects data integrity.
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn it_enforces_score_bounds() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    // Try to set score above maximum (should panic)
    client.set_score(&updater, &user, &101);
}

/// Test: Gets the contract version
/// Verifies that the contract returns its version identifier correctly.
/// Receives: nothing. Returns: Symbol "v1_0_0". Useful for verifying deployed version in production.
#[test]
fn it_gets_version() {
    let env = Env::default();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let version = client.get_version();
    assert_eq!(version, symbol_short!("v1_0_0"));
}

// ============================================================================
// Admin Succession Tests (Feature #14)
// ============================================================================

/// Test: Supports multiple consecutive admin changes
/// Verifies that admin can be transferred multiple times (Admin1 → Admin2 → Admin3)
/// and that old admins lose their permissions after transfer.
/// Receives: Sequential admin addresses. Returns: void. Validates complete ownership transfer.
#[test]
fn it_supports_admin_succession() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let admin3 = Address::generate(&env);

    // Admin1 initializes the contract
    client.set_admin(&admin1);
    assert_eq!(client.get_admin(), admin1);

    // Admin1 transfers to Admin2
    client.set_admin(&admin2);
    assert_eq!(client.get_admin(), admin2);

    // Admin2 transfers to Admin3
    client.set_admin(&admin3);
    assert_eq!(client.get_admin(), admin3);

    // Verify the final admin is Admin3
    let final_admin = client.get_admin();
    assert_eq!(final_admin, admin3);
}

/// Test: Allows admin to set same admin (no-op case)
/// Verifies that calling set_admin with the current admin address works correctly.
/// Receives: Current Admin Address. Returns: void. Validates idempotent operation.
#[test]
fn it_allows_admin_to_set_same_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Set initial admin
    client.set_admin(&admin);
    assert_eq!(client.get_admin(), admin);

    // Set same admin again (no-op case)
    client.set_admin(&admin);
    assert_eq!(client.get_admin(), admin);

    // Admin should still be able to perform admin operations
    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);
    assert!(client.is_updater(&updater));
}

// ============================================================================
// State Persistence Tests (Feature #14)
// ============================================================================

/// Test: Preserves user scores during admin changes
/// Verifies that user reputation scores remain unchanged when admin is transferred.
/// Receives: Multiple user addresses with scores. Returns: void. Validates data persistence.
#[test]
fn it_preserves_user_scores_during_admin_changes() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);

    // Setup: Admin1 creates updater and sets user scores
    client.set_admin(&admin1);

    let updater = Address::generate(&env);
    client.set_updater(&admin1, &updater, &true);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    client.set_score(&updater, &user1, &75);
    client.set_score(&updater, &user2, &50);
    client.set_score(&updater, &user3, &90);

    // Verify initial scores
    assert_eq!(client.get_score(&user1), 75);
    assert_eq!(client.get_score(&user2), 50);
    assert_eq!(client.get_score(&user3), 90);

    // Transfer admin to Admin2
    client.set_admin(&admin2);
    assert_eq!(client.get_admin(), admin2);

    // Verify all scores are preserved after admin change
    assert_eq!(client.get_score(&user1), 75);
    assert_eq!(client.get_score(&user2), 50);
    assert_eq!(client.get_score(&user3), 90);
}

/// Test: Preserves updater permissions during admin changes
/// Verifies that updater permissions persist when admin is transferred,
/// and that the new admin can manage these updaters.
/// Receives: Admin addresses and updater addresses. Returns: void. Validates permission persistence.
#[test]
fn it_preserves_updater_permissions_during_admin_changes() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);

    // Admin1 registers multiple updaters
    client.set_admin(&admin1);

    let updater1 = Address::generate(&env);
    let updater2 = Address::generate(&env);

    client.set_updater(&admin1, &updater1, &true);
    client.set_updater(&admin1, &updater2, &true);

    // Verify updaters are registered
    assert!(client.is_updater(&updater1));
    assert!(client.is_updater(&updater2));

    // Transfer admin to Admin2
    client.set_admin(&admin2);

    // Verify updaters still have permissions after admin change
    assert!(client.is_updater(&updater1));
    assert!(client.is_updater(&updater2));

    // Verify new admin can revoke updater permissions
    client.set_updater(&admin2, &updater1, &false);
    assert!(!client.is_updater(&updater1));
    assert!(client.is_updater(&updater2));

    // Verify new admin can add new updaters
    let updater3 = Address::generate(&env);
    client.set_updater(&admin2, &updater3, &true);
    assert!(client.is_updater(&updater3));
}

// ============================================================================
// Permission Revocation Tests (Feature #14)
// ============================================================================

/// Test: Revokes old admin permissions completely
/// Verifies that after admin transfer, the old admin cannot perform admin operations.
/// Tests that old admin cannot call set_updater with their address.
/// Receives: Old admin attempting set_updater. Returns: panic with NotAdmin error (#1).
#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn it_revokes_old_admin_permissions_completely() {
    let env = Env::default();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);

    // Admin1 initializes and transfers to Admin2
    env.mock_all_auths();
    client.set_admin(&admin1);
    client.set_admin(&admin2);

    // Verify admin2 is now the admin
    assert_eq!(client.get_admin(), admin2);

    // Old admin (Admin1) tries to set updater (should panic with NotAdmin error)
    let updater = Address::generate(&env);
    client
        .mock_all_auths()
        .set_updater(&admin1, &updater, &true);
}

/// Test: New admin can perform all admin operations after transfer
/// Verifies that the new admin has full admin privileges after transfer.
/// Receives: New admin performing operations. Returns: void. Validates complete ownership transfer.
#[test]
fn it_grants_new_admin_full_permissions() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);

    // Admin1 initializes and transfers to Admin2
    client.set_admin(&admin1);
    client.set_admin(&admin2);

    // Verify Admin2 can set updaters
    let updater = Address::generate(&env);
    client.set_updater(&admin2, &updater, &true);
    assert!(client.is_updater(&updater));

    // Verify Admin2 can revoke updaters
    client.set_updater(&admin2, &updater, &false);
    assert!(!client.is_updater(&updater));

    // Verify Admin2 can transfer admin to another address
    let admin3 = Address::generate(&env);
    client.set_admin(&admin3);
    assert_eq!(client.get_admin(), admin3);
}
