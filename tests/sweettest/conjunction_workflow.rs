//! Conjunction Workflow Integration Tests
//!
//! Tests the complete workflow from object registration through
//! conjunction detection to CDM exchange and maneuver coordination.

use holochain::sweettest::*;
use holochain::prelude::*;
use mycelix_space_shared::*;
use std::path::PathBuf;

fn dna_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../workdir/dna/mycelix_space.dna")
}

/// Test: Complete conjunction workflow between two operators
///
/// Scenario:
/// 1. SpaceX registers Starlink-1234
/// 2. OneWeb registers OneWeb-0042
/// 3. SSA provider detects conjunction
/// 4. CDM is created and shared
/// 5. SpaceX announces maneuver
/// 6. Conjunction status updated
#[tokio::test(flavor = "multi_thread")]
async fn test_full_conjunction_workflow() {
    let (conductors, _) = setup_conductors(4).await;

    let spacex = &conductors[0];      // SpaceX operator
    let oneweb = &conductors[1];      // OneWeb operator
    let ssa_provider = &conductors[2]; // SSA data provider (like LeoLabs)
    let traffic_mgr = &conductors[3];  // Traffic management authority

    // === Step 1: Register objects ===

    // SpaceX registers Starlink
    let starlink = serde_json::json!({
        "norad_id": 44713,
        "name": "STARLINK-1234",
        "object_type": "Payload",
        "country_code": "US",
        "owner_operator": "SpaceX",
        "rcs_m2": 22.0,
        "hard_body_radius_m": 3.0,
        "maneuverable": true
    });
    let _: ActionHash = spacex.call("orbital_objects", "register_object", starlink).await;

    // OneWeb registers their satellite
    let oneweb_sat = serde_json::json!({
        "norad_id": 48100,
        "name": "ONEWEB-0042",
        "object_type": "Payload",
        "country_code": "UK",
        "owner_operator": "OneWeb",
        "rcs_m2": 8.0,
        "hard_body_radius_m": 2.0,
        "maneuverable": true
    });
    let _: ActionHash = oneweb.call("orbital_objects", "register_object", oneweb_sat).await;

    // === Step 2: SSA provider detects conjunction ===

    let conjunction_event = serde_json::json!({
        "event_id": "CONJ-2024-001234",
        "primary_norad_id": 44713,
        "secondary_norad_id": 48100,
        "tca": "2024-01-20T14:32:15Z",
        "miss_distance_km": 0.15,
        "max_pc": 0.00025,  // 2.5e-4, HIGH risk
        "risk_level": "High"
    });

    let event_hash: ActionHash = ssa_provider
        .call("conjunctions", "create_conjunction_event", conjunction_event)
        .await;

    // Wait for signal propagation
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Both operators should receive the alert (via signals - tested separately)

    // === Step 3: SSA provider submits CDM ===

    let cdm = serde_json::json!({
        "conjunction_id": "CONJ-2024-001234",
        "message_id": "CDM-2024-001234-001",
        "creation_time": "2024-01-18T10:00:00Z",
        "originator": "LEOLABS",
        "tca": "2024-01-20T14:32:15.123Z",
        "miss_distance_km": 0.15,
        "relative_velocity_kms": 14.5,
        "collision_probability": 0.00025,
        "object1": {
            "norad_id": 44713,
            "name": "STARLINK-1234",
            "object_designator": "2019-074A",
            "maneuverable": "YES",
            "covariance": {
                "cr_r": 100.0,
                "ct_r": 0.0, "ct_t": 100.0,
                "cn_r": 0.0, "cn_t": 0.0, "cn_n": 100.0
            }
        },
        "object2": {
            "norad_id": 48100,
            "name": "ONEWEB-0042",
            "object_designator": "2021-024A",
            "maneuverable": "YES",
            "covariance": {
                "cr_r": 150.0,
                "ct_r": 0.0, "ct_t": 150.0,
                "cn_r": 0.0, "cn_t": 0.0, "cn_n": 150.0
            }
        }
    });

    let cdm_input = serde_json::json!({
        "cdm": cdm,
        "version": 1,
        "supersedes": null
    });

    let _: ActionHash = ssa_provider
        .call("conjunctions", "submit_cdm", cdm_input)
        .await;

    // === Step 4: SpaceX announces avoidance maneuver ===

    let maneuver = serde_json::json!({
        "event_id": "CONJ-2024-001234",
        "norad_id": 44713,
        "burn_time": "2024-01-20T10:00:00Z",  // 4.5 hours before TCA
        "delta_v_ms": 0.5,  // 0.5 m/s
        "direction": [0.0, 1.0, 0.0]  // Along-track
    });

    let maneuver_hash: ActionHash = spacex
        .call("conjunctions", "announce_maneuver", maneuver)
        .await;

    // Wait for DHT sync
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // === Step 5: Traffic manager and OneWeb can see the maneuver ===

    // OneWeb queries maneuvers for the conjunction
    let maneuvers: Vec<AvoidanceManeuverRecord> = oneweb
        .call("conjunctions", "get_maneuvers_for_event", "CONJ-2024-001234")
        .await;

    assert_eq!(maneuvers.len(), 1, "OneWeb should see SpaceX's maneuver");
    assert_eq!(maneuvers[0].norad_id, 44713);
    assert_eq!(maneuvers[0].status, "Announced");

    // === Step 6: SpaceX marks maneuver as executed ===

    let exec_input = serde_json::json!({
        "maneuver_hash": maneuver_hash
    });

    let _: ActionHash = spacex
        .call("conjunctions", "mark_maneuver_executed", exec_input)
        .await;

    // Wait for DHT sync
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Traffic manager verifies maneuver was executed
    let updated_maneuvers: Vec<AvoidanceManeuverRecord> = traffic_mgr
        .call("conjunctions", "get_maneuvers_for_event", "CONJ-2024-001234")
        .await;

    assert_eq!(updated_maneuvers[0].status, "Executed");
}

/// Test: High-risk conjunction triggers emergency alert
#[tokio::test(flavor = "multi_thread")]
async fn test_emergency_conjunction_alert() {
    let (conductors, _) = setup_conductors(2).await;

    let ssa = &conductors[0];
    let operator = &conductors[1];

    // Create EMERGENCY level conjunction (Pc > 1e-3)
    let emergency_conjunction = serde_json::json!({
        "event_id": "CONJ-2024-EMERGENCY",
        "primary_norad_id": 25544,  // ISS
        "secondary_norad_id": 99999, // Debris
        "tca": "2024-01-15T06:00:00Z",
        "miss_distance_km": 0.05,  // 50 meters!
        "max_pc": 0.005,  // 5e-3, EMERGENCY
        "risk_level": "Emergency"
    });

    let event_hash: ActionHash = ssa
        .call("conjunctions", "create_conjunction_event", emergency_conjunction)
        .await;

    assert!(!event_hash.as_ref().is_empty());

    // Wait for signal propagation
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Operator queries high-risk conjunctions
    let high_risk: Vec<ConjunctionEventRecord> = operator
        .call("conjunctions", "get_high_risk_conjunctions", ())
        .await;

    // Should include our emergency
    let emergency = high_risk.iter()
        .find(|c| c.event_id == "CONJ-2024-EMERGENCY");

    assert!(emergency.is_some(), "Emergency conjunction should be in high-risk list");
    assert_eq!(emergency.unwrap().risk_level, "Emergency");
}

/// Test: Debris bounty creation and funding
#[tokio::test(flavor = "multi_thread")]
async fn test_debris_bounty_workflow() {
    let (conductors, _) = setup_conductors(3).await;

    let operator1 = &conductors[0];  // Threatened operator 1
    let operator2 = &conductors[1];  // Threatened operator 2
    let remover = &conductors[2];     // Debris removal company

    // Operator1 creates bounty on threatening debris
    let bounty = serde_json::json!({
        "debris_norad_id": 12345,
        "debris_name": "COSMOS 1408 DEB",
        "threat_description": "Frequent conjunctions with our constellation",
        "initial_funding_usd": 50000,
        "requirements": {
            "removal_deadline": "2025-12-31T00:00:00Z",
            "verification_method": "TLE cessation"
        }
    });

    let bounty_hash: ActionHash = operator1
        .call("debris_bounties", "create_bounty", bounty)
        .await;

    // Wait for DHT sync
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Operator2 adds funding to the bounty
    let funding = serde_json::json!({
        "bounty_hash": bounty_hash,
        "amount_usd": 25000,
        "message": "This debris threatens our satellites too"
    });

    let _: ActionHash = operator2
        .call("debris_bounties", "add_funding", funding)
        .await;

    // Remover queries available bounties
    let bounties: Vec<BountyRecord> = remover
        .call("debris_bounties", "get_active_bounties", ())
        .await;

    let our_bounty = bounties.iter()
        .find(|b| b.debris_norad_id == 12345);

    assert!(our_bounty.is_some());
    assert_eq!(our_bounty.unwrap().total_funding_usd, 75000); // 50k + 25k
}

// Helper types for test deserialization
#[derive(Debug, Clone, serde::Deserialize)]
struct AvoidanceManeuverRecord {
    norad_id: u32,
    status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ConjunctionEventRecord {
    event_id: String,
    risk_level: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BountyRecord {
    debris_norad_id: u32,
    total_funding_usd: u64,
}

async fn setup_conductors(n: usize) -> (Vec<SweetConductor>, Vec<AgentPubKey>) {
    let dna = SweetDnaFile::from_file(&dna_path())
        .await
        .expect("DNA file should exist");

    let mut conductors = Vec::new();
    let mut agents = Vec::new();

    for _ in 0..n {
        let conductor = SweetConductor::from_standard_config().await;
        let agent = conductor.setup_app("mycelix-space", &[dna.clone()]).await;
        agents.push(agent.agent_pubkey().clone());
        conductors.push(conductor);
    }

    SweetConductor::exchange_peers(&conductors).await;

    (conductors, agents)
}

trait ConductorTestExt {
    async fn call<I: serde::Serialize, O: serde::de::DeserializeOwned>(
        &self,
        zome: &str,
        fn_name: &str,
        input: I,
    ) -> O;
}

impl ConductorTestExt for SweetConductor {
    async fn call<I: serde::Serialize, O: serde::de::DeserializeOwned>(
        &self,
        zome: &str,
        fn_name: &str,
        input: I,
    ) -> O {
        let cell_id = self.list_cell_ids()[0].clone();
        self.call_zome(cell_id, zome, fn_name, input)
            .await
            .expect("Zome call should succeed")
    }
}
