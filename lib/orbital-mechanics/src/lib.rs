//! Orbital Mechanics Library for Mycelix-Space
//!
//! This library provides:
//! - TLE parsing and validation
//! - SGP4/SDP4 orbital propagation
//! - Covariance matrix propagation (uncertainty tracking)
//! - Conjunction analysis (collision probability)
//! - Coordinate transformations (ECI, ECEF, geodetic)
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Orbital State Model                          │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                 │
//! │  TLE Input ──────► SGP4 Elements ──────► State Vector           │
//! │       │                   │                    │                │
//! │       │                   │                    ▼                │
//! │       │                   │           ┌──────────────┐          │
//! │       │                   │           │ Position (3) │          │
//! │       │                   │           │ Velocity (3) │          │
//! │       │                   │           │ Covariance   │          │
//! │       │                   │           │   (6x6)      │          │
//! │       │                   │           └──────────────┘          │
//! │       │                   │                    │                │
//! │       │                   ▼                    ▼                │
//! │       │           ┌─────────────┐    ┌─────────────────┐        │
//! │       │           │ Propagator  │───►│ Future State    │        │
//! │       │           │ (SGP4/SDP4) │    │ + Uncertainty   │        │
//! │       │           └─────────────┘    └─────────────────┘        │
//! │       │                                       │                 │
//! │       ▼                                       ▼                 │
//! │  ┌──────────┐                        ┌───────────────┐          │
//! │  │ Validate │                        │ Conjunction   │          │
//! │  │ Checksum │                        │ Analysis      │          │
//! │  └──────────┘                        └───────────────┘          │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

pub mod cdm;
pub mod conjunction;
pub mod coordinates;
pub mod covariance;
pub mod propagator;
pub mod state;
pub mod tle;

pub use cdm::{
    CdmBuilder, CdmCovariance, CdmObjectMetadata, CdmRefFrame, CdmStateVector,
    ConjunctionDataMessage, Maneuverable,
};
pub use conjunction::{CollisionProbability, ConjunctionAssessment};
pub use covariance::CovarianceMatrix;
pub use propagator::{PropagationError, Propagator};
pub use state::{OrbitalState, StateVector};
pub use tle::{TleParseError, TwoLineElement};
