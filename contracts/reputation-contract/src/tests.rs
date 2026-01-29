use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    Address, Env, IntoVal, Symbol, Val, Vec,
};

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

/// Test: Revokes updater access after removal
/// Verifies full lifecycle: grant -> modify -> revoke -> ensure NotUpdater error
#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn it_revokes_updater_access_after_removal() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    // Updater can update initially
    client.set_score(&updater, &user, &20);
    assert_eq!(client.get_score(&user), 20);

    // Revoke updater access
    client.set_updater(&admin, &updater, &false);
    assert!(!client.is_updater(&updater));

    // Former updater should no longer be able to increase the score (should panic NotUpdater)
    client.increase_score(&updater, &user, &5);
}

/// Test: Emitted event on updater removal
/// Verifies that UPDCHGD event can be emitted when removing updater
#[test]
fn it_emits_event_on_updater_removal() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    // Grant updater
    client.set_updater(&admin, &updater, &true);
    assert!(client.is_updater(&updater));

    // Revoke updater (this should emit UPDCHGD event with false)
    client.set_updater(&admin, &updater, &false);
    assert!(!client.is_updater(&updater));

    // If we reached here without panic, event emission didn't crash
}

/// Test: Removing a non-existent updater should not panic and should be handled gracefully
#[test]
fn it_handles_removing_non_existent_updater() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let never_added = Address::generate(&env);

    // Removing a never-added updater should not panic
    client.set_updater(&admin, &never_added, &false);
    assert!(!client.is_updater(&never_added));
}

/// Overflow/Underflow tests
/// Verifies contract returns correct errors for arithmetic edge cases
#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn it_prevents_overflow_on_increase() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &80);
    // increase by 50 would overflow beyond 100
    client.increase_score(&updater, &user, &50);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn it_prevents_overflow_at_max() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &100);
    // increase by 1 when at max should overflow
    client.increase_score(&updater, &user, &1);
}

#[test]
fn it_allows_increase_up_to_max() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &80);
    client.increase_score(&updater, &user, &20);
    assert_eq!(client.get_score(&user), 100);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn it_prevents_underflow_on_decrease() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &30);
    // decrease by 50 would underflow below 0
    client.decrease_score(&updater, &user, &50);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn it_prevents_underflow_at_min() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &0);
    // decrease by 1 at zero should underflow
    client.decrease_score(&updater, &user, &1);
}

#[test]
fn it_allows_decrease_down_to_min() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    let user = Address::generate(&env);

    client.set_score(&updater, &user, &30);
    client.decrease_score(&updater, &user, &30);
    assert_eq!(client.get_score(&user), 0);
}

/// Test: Removing one updater doesn't affect other updaters
#[test]
fn it_removing_one_updater_does_not_affect_others() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater1 = Address::generate(&env);
    let updater2 = Address::generate(&env);
    client.set_updater(&admin, &updater1, &true);
    client.set_updater(&admin, &updater2, &true);

    let user = Address::generate(&env);

    // updater1 increases score
    client.set_score(&updater1, &user, &10);
    client.increase_score(&updater1, &user, &5);
    assert_eq!(client.get_score(&user), 15);

    // Revoke updater1
    client.set_updater(&admin, &updater1, &false);
    assert!(!client.is_updater(&updater1));
    assert!(client.is_updater(&updater2));

    // updater2 should still be able to update
    client.increase_score(&updater2, &user, &5);
    assert_eq!(client.get_score(&user), 20);
}

/// Test: Emits SCORECHGD event on score increase
/// Verifies that increasing a user's score emits the correct event with (user, old_score, new_score, "increase") data.
#[test]
fn it_emits_score_changed_event_on_increase() {
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

    // Increase score
    client.increase_score(&updater, &user, &20);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the SCORECHGD event (should be the last one)
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("SCORECHGD") {
            found_event = true;

            // Verify user address (second topic)
            let event_user: Address = topics.get(1).unwrap().into_val(&env);
            assert_eq!(event_user, user);

            // Verify data (old_score, new_score, reason) - data is a tuple
            let data_tuple: (u32, u32, Symbol) = event.2.into_val(&env);
            let (old_score, new_score, reason) = data_tuple;

            assert_eq!(old_score, 50);
            assert_eq!(new_score, 70);
            assert_eq!(reason, symbol_short!("increase"));
            break;
        }
    }

    assert!(found_event, "SCORECHGD event not found");
}

/// Test: Emits SCORECHGD event on score decrease
/// Verifies that decreasing a user's score emits the correct event with (user, old_score, new_score, "decrease") data.
#[test]
fn it_emits_score_changed_event_on_decrease() {
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

    // Decrease score
    client.decrease_score(&updater, &user, &20);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the SCORECHGD event
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("SCORECHGD") {
            // Check if this is the decrease event (new_score should be 30)
            let data_tuple: (u32, u32, Symbol) = event.2.into_val(&env);
            let (_, _new_score, reason) = data_tuple;

            if reason == symbol_short!("decrease") {
                found_event = true;

                // Verify user address (second topic)
                let event_user: Address = topics.get(1).unwrap().into_val(&env);
                assert_eq!(event_user, user);

                let (old_score, new_score, _) = data_tuple;
                assert_eq!(old_score, 50);
                assert_eq!(new_score, 30);
                break;
            }
        }
    }

    assert!(
        found_event,
        "SCORECHGD event with 'decrease' reason not found"
    );
}

/// Test: Emits SCORECHGD event on score set
/// Verifies that setting a user's score emits the correct event with (user, old_score, new_score, "set") data.
#[test]
fn it_emits_score_changed_event_on_set() {
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

    // Set score to new value
    client.set_score(&updater, &user, &75);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the SCORECHGD event with "set" reason
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("SCORECHGD") {
            let data_tuple: (u32, u32, Symbol) = event.2.into_val(&env);
            let (_, _, reason) = data_tuple;

            if reason == symbol_short!("set") {
                found_event = true;

                // Verify user address (second topic)
                let event_user: Address = topics.get(1).unwrap().into_val(&env);
                assert_eq!(event_user, user);

                let (old_score, new_score, _) = data_tuple;
                assert_eq!(old_score, 50);
                assert_eq!(new_score, 75);
                break;
            }
        }
    }

    assert!(found_event, "SCORECHGD event with 'set' reason not found");
}

/// Test: Emits UPDCHGD event on updater grant
/// Verifies that granting updater permission emits the correct event with (updater, true) data.
#[test]
fn it_emits_updater_changed_event_on_grant() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);

    // Grant updater permission
    client.set_updater(&admin, &updater, &true);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the UPDCHGD event with allowed=true
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("UPDCHGD") {
            let allowed: bool = event.2.into_val(&env);

            if allowed {
                found_event = true;

                // Verify updater address (second topic)
                let event_updater: Address = topics.get(1).unwrap().into_val(&env);
                assert_eq!(event_updater, updater);
                break;
            }
        }
    }

    assert!(found_event, "UPDCHGD event with allowed=true not found");
}

/// Test: Emits UPDCHGD event on updater revoke
/// Verifies that revoking updater permission emits the correct event with (updater, false) data.
#[test]
fn it_emits_updater_changed_event_on_revoke() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let updater = Address::generate(&env);
    client.set_updater(&admin, &updater, &true);

    // Revoke updater permission
    client.set_updater(&admin, &updater, &false);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the UPDCHGD event with allowed=false
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("UPDCHGD") {
            let allowed: bool = event.2.into_val(&env);

            if !allowed {
                found_event = true;

                // Verify updater address (second topic)
                let event_updater: Address = topics.get(1).unwrap().into_val(&env);
                assert_eq!(event_updater, updater);
                break;
            }
        }
    }

    assert!(found_event, "UPDCHGD event with allowed=false not found");
}

/// Test: Emits ADMINCHGD event on admin change
/// Verifies that changing the admin emits the correct event with (old_admin, new_admin) data.
#[test]
fn it_emits_admin_changed_event() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_admin(&admin);

    let new_admin = Address::generate(&env);

    // Change admin
    client.set_admin(&new_admin);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the ADMINCHGD event where old_admin != new_admin
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("ADMINCHGD") {
            let data_tuple: (Address, Address) = event.2.into_val(&env);
            let (old_admin, new_admin_event) = data_tuple;

            // This should be the admin change event (not initial setup)
            if old_admin != new_admin_event {
                found_event = true;
                assert_eq!(old_admin, admin);
                assert_eq!(new_admin_event, new_admin);
                break;
            }
        }
    }

    assert!(found_event, "ADMINCHGD event for admin change not found");
}

/// Test: Emits ADMINCHGD event on initial admin setup
/// Verifies that setting the admin for the first time emits the event with (dummy_address, new_admin) data.
/// Note: The contract uses the new_admin as both old and new during initial setup.
#[test]
fn it_emits_admin_changed_event_on_initial_setup() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationContract, ());
    let client = ReputationContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Set admin for the first time
    client.set_admin(&admin);

    // Verify event was emitted
    let events: Vec<(Address, Vec<Val>, Val)> = env.events().all();

    // Find the ADMINCHGD event
    let mut found_event = false;
    for event in events.iter() {
        let topics = event.1.clone();
        let event_type: Symbol = topics.get(0).unwrap().into_val(&env);

        if event_type == symbol_short!("ADMINCHGD") {
            found_event = true;

            // Verify data (old_admin, new_admin) - data is a tuple
            let data_tuple: (Address, Address) = event.2.into_val(&env);
            let (old_admin, new_admin_event) = data_tuple;

            // During initial setup, both old and new admin are set to the same address (the new admin)
            assert_eq!(old_admin, admin);
            assert_eq!(new_admin_event, admin);
            break;
        }
    }

    assert!(found_event, "ADMINCHGD event not found");
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
