# Mycelix Space

**Decentralized Space Domain Awareness Network**

A Holochain-based peer-to-peer network for tracking orbital objects, predicting conjunctions, and coordinating space traffic without relying on any single nation's Space Force.

![Space Traffic Control Dashboard](docs/dashboard-preview.png)

## Quick Start

### Run the Demo UI (No Holochain Required)

```bash
cd ui
npm install
npm run dev
# Open http://localhost:5173
```

The UI works standalone with demo data and real-time SGP4 propagation.

### Full Stack (With Holochain)

```bash
# Enter development environment
nix develop

# Build and run sandbox with hApp
./scripts/sandbox-setup.sh

# In another terminal, run UI
cd ui && npm install && npm run dev
```

## Vision

Transform space situational awareness from a government monopoly into a global commons, enabling:
- Operators to share and verify orbital data
- Communities to protect orbital lanes
- Markets for debris removal (Kessler bounties)
- Automated traffic negotiation between operators

## Space Traffic Control Dashboard

A real-time web interface for monitoring orbital objects and conjunction threats.

### Features
- **3D Live Map**: CesiumJS globe with real-time satellite positions
- **SGP4 Propagation**: TLEs propagated to current positions every second
- **Conjunction Monitoring**: Risk-based filtering and CDM generation
- **Object Browser**: Search 20,000+ tracked objects
- **Debris Bounties**: Crowdfunded cleanup incentives

### Tech Stack
- **Frontend**: SvelteKit + TypeScript
- **3D Visualization**: CesiumJS
- **Orbital Math**: satellite.js (SGP4/SDP4)
- **Backend**: Holochain (optional - works offline with demo data)
- **Data**: CelesTrak public TLE feeds

## Architecture

```
mycelix-space/
├── ui/                        # Space Traffic Control Dashboard
│   ├── src/lib/orbital/       # SGP4 propagation, CelesTrak fetcher
│   ├── src/lib/holochain/     # Conductor client wrapper
│   └── src/routes/            # Dashboard pages
│
├── lib/orbital-mechanics/     # Core orbital mechanics (Rust)
│   ├── tle.rs                 # TLE parsing and validation
│   ├── state.rs               # State vectors with covariance
│   ├── propagator.rs          # SGP4/SDP4 orbital propagation
│   └── conjunction.rs         # Collision probability (Pc)
│
├── zomes/                     # Holochain DNA Zomes
│   ├── orbital_objects/       # Catalog of tracked objects
│   ├── observations/          # Sensor data ingestion
│   ├── conjunctions/          # Collision prediction & CDMs
│   ├── debris_bounties/       # Kessler cleanup market
│   └── traffic_control/       # Automated negotiation
│
├── tools/celestrak-demo/      # Data pipeline from CelesTrak
│
├── tests/
│   ├── sweettest/             # Multi-agent integration tests
│   └── integration/           # Rust integration tests
│
└── scripts/
    ├── build-happ.sh          # Package DNA and hApp
    └── sandbox-setup.sh       # Quick-start Holochain sandbox
```

## Key Features

### 1. Real-Time Orbital Tracking
Track satellites using SGP4 propagation from TLE data.
- Live position updates every second
- Orbital path visualization
- Velocity and altitude display

### 2. Conjunction Analysis
Calculate collision probabilities with proper uncertainty.
- Covariance propagation
- 2D Alfano Pc calculation
- CCSDS-standard CDM generation
- Risk-based alerting (Emergency/High/Medium/Low)

### 3. Decentralized Catalog
Holochain-based distributed ledger for orbital data.
- No single point of failure
- Operator-submitted TLEs
- Trust-weighted data fusion
- Cryptographic verification

### 4. Debris Bounties (Kessler Cleanup Market)
Crowdfunded incentives for debris removal.
- Post bounties on threatening debris
- Track funding progress
- Multiple contributor support

## Development

### Prerequisites

- **Node.js 18+**: For UI development
- **Rust**: For zome development
- **Nix** (optional): For reproducible environment with Holochain tools

### Development Shells

```bash
# Full environment with Holochain CLI
nix develop

# Rust-only (faster, no Holochain)
nix develop .#rust

# UI-only (Node.js)
nix develop .#ui
```

### Running Tests

```bash
# Rust unit tests
cargo test --workspace

# Integration tests (113 tests)
cargo test --workspace -- --test-threads=1

# Sweettest (requires Holochain)
# cargo test -p sweettest
```

### Building

```bash
# Build WASM zomes
cargo build --release --target wasm32-unknown-unknown

# Package hApp (requires hc CLI)
./scripts/build-happ.sh

# Build UI
cd ui && npm run build
```

## Data Sources

### CelesTrak Integration
Fetch real orbital data directly in the browser or via the Rust pipeline.

```typescript
// In browser (via UI)
import { fetchTLEs, CATALOG_GROUPS } from '$lib/orbital/celestrak';
const stations = await fetchTLEs(CATALOG_GROUPS.STATIONS);
```

```bash
# Via Rust tool
cargo run -p celestrak-demo -- fetch --source active --limit 100
```

### Supported Catalogs
- Space Stations (ISS, Tiangong, etc.)
- Starlink Constellation (~6,000 satellites)
- OneWeb Constellation
- GPS/GLONASS/Galileo
- Weather Satellites
- Debris fields (Cosmos 2251, Fengyun 1C, Iridium 33)

## Development Status

- [x] Orbital mechanics library (complete)
- [x] TLE parsing with checksum validation
- [x] State vectors with covariance
- [x] SGP4 propagation
- [x] Conjunction probability calculation
- [x] CCSDS CDM generation
- [x] All 5 Holochain zomes
- [x] DNA/hApp packaging
- [x] 113 integration tests passing
- [x] **Space Traffic Control Dashboard**
- [x] **Real-time SGP4 visualization**
- [x] **CelesTrak data integration**
- [x] Sweettest multi-agent tests
- [ ] Production Holochain deployment
- [ ] Voice-activated queries
- [ ] Mobile app

## Contributing

Contributions welcome! Please open an issue or PR.

### Areas of Interest
- Additional data sources (Space-Track, LeoLabs)
- Improved conjunction algorithms
- Mobile/tablet UI optimization
- Voice interface integration
- Additional language support

## License

MIT

## Related Projects

Part of the [Luminous Dynamics](https://github.com/Luminous-Dynamics) ecosystem.

- **Mycelix Network**: P2P infrastructure
- **Terra Atlas**: Energy investment platform
- **Luminous Nix**: NixOS natural language interface

---

*"The stars belong to no nation, and neither should the knowledge of what moves among them."*
