# Sweettest Integration Tests

Multi-agent integration tests for Mycelix Space using Holochain's Sweettest framework.

## What These Tests Verify

### `multi_agent.rs`
- Two operators can register orbital objects and see each other's entries via DHT
- TLE submission and cross-agent retrieval
- Multiple ground stations submitting observations

### `conjunction_workflow.rs`
- Complete conjunction workflow from detection to maneuver execution
- Emergency conjunction alert propagation
- Debris bounty creation and crowdfunding

## Prerequisites

1. **Holochain CLI** (`hc`) installed
2. **DNA packaged** at `workdir/dna/mycelix_space.dna`

```bash
# Build WASM and package DNA
./scripts/build-happ.sh
```

## Running the Tests

```bash
# Run all sweettest tests
cargo test -p mycelix-space-sweettest

# Run specific test
cargo test -p mycelix-space-sweettest --test multi_agent

# Run with verbose output
cargo test -p mycelix-space-sweettest -- --nocapture
```

## Test Scenarios

### Scenario 1: Multi-Operator Object Registration
```
Alice (SpaceX)           Bob (OneWeb)           DHT
     |                        |                  |
     |-- register ISS ------->|                  |
     |                        |                  |
     |                        |-- register sat ->|
     |                        |                  |
     |<---- query sat --------|------------------|
     |                        |                  |
     |-------- query ISS -----|----------------->|
```

### Scenario 2: Conjunction Workflow
```
SpaceX    OneWeb    SSA Provider    Traffic Mgr
   |         |           |              |
   |-- register obj ---->|              |
   |         |-- register obj --------->|
   |         |           |              |
   |         |<-- detect conjunction ---|
   |         |           |              |
   |         |<---- submit CDM ---------|
   |         |           |              |
   |-- announce maneuver --------------->|
   |         |           |              |
   |-- mark executed ------------------->|
```

## Debugging

If tests fail:

1. **Check DNA exists**: `ls workdir/dna/mycelix_space.dna`
2. **Rebuild WASM**: `cargo build --release --target wasm32-unknown-unknown -p <zome>`
3. **Check logs**: Tests output to stderr, use `--nocapture` to see

## Notes

- Tests spin up temporary Holochain conductors (no persistent state)
- DHT sync may take ~500ms between operations
- Each test is isolated - no state shared between tests
