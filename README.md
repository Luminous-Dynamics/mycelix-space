# Mycelix Space

**Decentralized Space Domain Awareness Network**

A Holochain-based peer-to-peer network for tracking orbital objects, predicting conjunctions, and coordinating space traffic without relying on any single nation's Space Force.

## Vision

Transform space situational awareness from a government monopoly into a global commons, enabling:
- Operators to share and verify orbital data
- Communities to protect orbital lanes
- Markets for debris removal (Kessler bounties)
- Automated traffic negotiation between operators

## Architecture

```
mycelix-space/
├── lib/orbital-mechanics/     # Core orbital mechanics (no Holochain deps)
│   ├── tle.rs                 # TLE parsing and validation
│   ├── state.rs               # State vectors with covariance
│   ├── covariance.rs          # 6x6 uncertainty matrices
│   ├── propagator.rs          # SGP4/SDP4 orbital propagation
│   ├── conjunction.rs         # Collision probability analysis
│   └── coordinates.rs         # Frame transformations (TEME, ECI, ECEF, geodetic)
│
├── zomes/shared/              # Shared types across all DNAs
│   └── lib.rs                 # NoradId, SpaceTimestamp, TrustLevel, CDM types
│
└── dna/zomes/                 # Holochain DNA Zomes
    ├── orbital_objects/       # Catalog of tracked objects
    ├── observations/          # Sensor data ingestion
    ├── conjunctions/          # Collision prediction & CDMs
    ├── debris_bounties/       # Kessler cleanup market
    └── traffic_control/       # Automated negotiation
```

## Key Features

### 1. Orbital Object Catalog
Track satellites, debris, and rocket bodies with decentralized consensus.
- TLE submission and validation
- Operator claims and verification
- Object metadata (RCS, mass, HBR)

### 2. Sensor Observations
Ingest data from ground and space-based sensors.
- Angles-only (optical)
- Radar range/range-rate
- Full state vectors with covariance

### 3. Conjunction Analysis
Calculate collision probabilities with proper uncertainty handling.
- Covariance propagation
- 2D Alfano Pc calculation
- Conjunction Data Messages (CDMs)

### 4. Debris Bounties (Kessler Cleanup Market)
Crowdfunded incentives for debris removal.
- Post bounties on threatening debris
- Aggregate funding from multiple parties
- Verified removal and payout

### 5. Automated Traffic Control
AI-mediated negotiation between operators.
- Capability and preference exchange
- Maneuver proposal generation
- Cryptographic agreement signing

## Why Covariance Matters

**This network tracks "probability clouds", not points.**

Every orbital state includes a 6x6 covariance matrix representing uncertainty. This enables:
- Meaningful collision probability (miss distance alone is meaningless)
- Proper conjunction screening (filter by statistical significance)
- Trust-weighted data fusion (lower uncertainty = higher weight)
- Zero-knowledge proofs (prove properties without revealing orbits)

## Building

```bash
# Check the library (no Holochain)
cargo check -p orbital-mechanics

# Run tests
cargo test --workspace

# Build WASM zomes
cargo build --release --target wasm32-unknown-unknown

# Package DNA and hApp (requires Holochain CLI)
./scripts/build-happ.sh

# Or manually:
hc dna pack dna -o workdir/dna/mycelix_space.dna
hc app pack dna -o workdir/happ/mycelix_space.happ
```

### Requirements

- Rust (stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Holochain CLI (`hc`): Install via Nix or from [holochain releases](https://github.com/holochain/holochain/releases)

## Development Status

- [x] Orbital mechanics library (complete)
- [x] TLE parsing with checksum validation
- [x] State vectors with covariance
- [x] SGP4 propagation wrapper
- [x] Conjunction probability calculation
- [x] Coordinate transformations
- [x] CDM generation (CCSDS standard)
- [x] Shared zome types
- [x] Orbital objects zomes (integrity + coordinator)
- [x] Observations zomes
- [x] Conjunctions zomes with real-time alerts
- [x] Debris bounties zomes
- [x] Traffic control zomes
- [x] DNA packaging
- [x] hApp packaging
- [x] Integration tests (113 passing)
- [x] CelesTrak data ingestion pipeline
- [ ] Web UI
- [ ] Sweettest multi-agent tests

## Tools

### celestrak-demo
Fetch orbital data from CelesTrak and optionally ingest into Holochain.

```bash
# Fetch ISS TLE
cargo run -p celestrak-demo -- fetch --source iss

# Fetch active satellites
cargo run -p celestrak-demo -- fetch --source active --limit 100

# Dry-run ingestion (no conductor needed)
cargo run -p celestrak-demo -- ingest --source stations --dry-run
```

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR.

## Related Projects

Part of the [Mycelix](https://github.com/Luminous-Dynamics) ecosystem of decentralized applications.

---

*"The stars belong to no nation, and neither should the knowledge of what moves among them."*
