//! Multi-Agent Integration Tests for Mycelix Space
//!
//! These tests verify that the zomes work correctly in multi-agent scenarios
//! where different operators interact through the DHT.

use holochain::sweettest::*;
use holochain::prelude::*;
use mycelix_space_shared::*;
use std::path::PathBuf;

/// Get the path to the compiled DNA
fn dna_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../workdir/dna/mycelix_space.dna")
}

/// Test: Two operators can register orbital objects and see each other's entries
#[tokio::test(flavor = "multi_thread")]
async fn test_two_operators_register_objects() {
    // Set up the test environment with 2 agents
    let (conductors, _agents) = setup_conductors(2).await;

    let alice = &conductors[0];
    let bob = &conductors[1];

    // Alice registers the ISS
    let iss_input = serde_json::json!({
        "norad_id": 25544,
        "name": "ISS (ZARYA)",
        "object_type": "Payload",
        "country_code": "ISS",
        "launch_date": "1998-11-20T00:00:00Z",
        "decay_date": null,
        "rcs_m2": 400.0,
        "hard_body_radius_m": 50.0
    });

    let iss_hash: ActionHash = alice
        .call("orbital_objects_coordinator", "register_object", iss_input)
        .await;

    assert!(!iss_hash.as_ref().is_empty(), "ISS should be registered");

    // Bob registers Starlink-1234
    let starlink_input = serde_json::json!({
        "norad_id": 44713,
        "name": "STARLINK-1234",
        "object_type": "Payload",
        "country_code": "US",
        "launch_date": "2020-01-07T00:00:00Z",
        "decay_date": null,
        "rcs_m2": 22.0,
        "hard_body_radius_m": 3.0
    });

    let starlink_hash: ActionHash = bob
        .call("orbital_objects_coordinator", "register_object", starlink_input)
        .await;

    assert!(!starlink_hash.as_ref().is_empty(), "Starlink should be registered");

    // Wait for DHT sync
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Alice should be able to query Bob's object
    let bob_object: Option<OrbitalObjectEntry> = alice
        .call("orbital_objects_coordinator", "get_object_by_norad_id", 44713u32)
        .await;

    assert!(bob_object.is_some(), "Alice should see Bob's Starlink");
    assert_eq!(bob_object.unwrap().name, "STARLINK-1234");

    // Bob should be able to query Alice's object
    let alice_object: Option<OrbitalObjectEntry> = bob
        .call("orbital_objects_coordinator", "get_object_by_norad_id", 25544u32)
        .await;

    assert!(alice_object.is_some(), "Bob should see Alice's ISS");
    assert_eq!(alice_object.unwrap().name, "ISS (ZARYA)");
}

/// Test: TLE submission and retrieval across agents
#[tokio::test(flavor = "multi_thread")]
async fn test_tle_submission_cross_agent() {
    let (conductors, _agents) = setup_conductors(2).await;

    let alice = &conductors[0];
    let bob = &conductors[1];

    // Alice submits a TLE for ISS
    let tle_input = serde_json::json!({
        "norad_id": 25544,
        "line1": "1 25544U 98067A   24001.50000000  .00016717  00000-0  10270-3 0  9997",
        "line2": "2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391424577",
        "source": "SpaceTrack"
    });

    let tle_hash: ActionHash = alice
        .call("orbital_objects_coordinator", "submit_tle", tle_input)
        .await;

    assert!(!tle_hash.as_ref().is_empty(), "TLE should be submitted");

    // Wait for DHT sync
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Bob retrieves the latest TLE for ISS
    let latest_tle: Option<TleData> = bob
        .call("orbital_objects_coordinator", "get_latest_tle", 25544u32)
        .await;

    assert!(latest_tle.is_some(), "Bob should see Alice's TLE");
    let tle = latest_tle.unwrap();
    assert_eq!(tle.norad_id, 25544);
    assert!(tle.line1.contains("25544U"));
}

/// Test: Observation submission from multiple ground stations
#[tokio::test(flavor = "multi_thread")]
async fn test_multi_station_observations() {
    let (conductors, _agents) = setup_conductors(3).await;

    let station_us = &conductors[0];   // US ground station
    let station_eu = &conductors[1];   // European ground station
    let analyst = &conductors[2];       // Analyst querying data

    // US station observes ISS
    let us_obs = serde_json::json!({
        "norad_id": 25544,
        "station_id": "USAF-1",
        "observation_type": "Radar",
        "timestamp": "2024-01-15T12:00:00Z",
        "azimuth_deg": 45.5,
        "elevation_deg": 30.2,
        "range_km": 500.0,
        "range_rate_kms": -2.1
    });

    let _: ActionHash = station_us
        .call("observations_coordinator", "submit_observation", us_obs)
        .await;

    // EU station observes same pass
    let eu_obs = serde_json::json!({
        "norad_id": 25544,
        "station_id": "ESA-ESTRACK-1",
        "observation_type": "Radar",
        "timestamp": "2024-01-15T12:05:00Z",
        "azimuth_deg": 120.3,
        "elevation_deg": 45.8,
        "range_km": 450.0,
        "range_rate_kms": -1.8
    });

    let _: ActionHash = station_eu
        .call("observations_coordinator", "submit_observation", eu_obs)
        .await;

    // Wait for DHT sync
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Analyst queries all observations for ISS
    let observations: Vec<ObservationRecord> = analyst
        .call("observations_coordinator", "get_observations_for_object", 25544u32)
        .await;

    assert_eq!(observations.len(), 2, "Should see observations from both stations");
}

/// Helper: Set up N conductors with the mycelix-space DNA
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

    // Connect all conductors to each other
    SweetConductor::exchange_peers(&conductors).await;

    (conductors, agents)
}

/// Helper trait extension for cleaner test code
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
