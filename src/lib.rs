//! # negentropy — The Physics of Information Extraction
//!
//! > *A Phoenix doesn't just survive; it uses intense thermal energy to burn
//! > away its outdated form, collapsing its entire state into ash so it can
//! > reconstitute itself into something pure, ordered, and renewed.*
//!
//! An open-source thermodynamic engine for scoring any system where
//! information reduces entropy. Extracted and generalized from the
//! [orkid FMD physics engine](https://github.com/jjcav84/orkid/tree/main/fmd-physics).
//!
//! ## The Phoenix Cycle
//!
//! Every information extraction — whether a ZK proof, an arbitrage route, or a
//! vote — passes through the same thermodynamic cycle:
//!
//! ```text
//!  ┌──────────┐    ┌──────────┐    ┌──────────────┐    ┌──────────┐
//!  │  ENTROPY │───▶│   BURN   │───▶│  EXTRACTION  │───▶│ REBIRTH  │
//!  │ (chaos)  │    │ (energy) │    │ (negentropy) │    │ (order)  │
//!  └──────────┘    └──────────┘    └──────────────┘    └──────────┘
//!  high uncertainty  Landauer cost   KL divergence      verifiable
//!  private data       k_B·T·ln(2)     bits extracted     artifact
//! ```
//!
//! ### 1. Entropy — The Old State
//!
//! Private data, market inefficiency, unverifiable claims — all are
//! high-entropy states. Without constraints, anyone could claim anything.
//! Shannon entropy quantifies this uncertainty:
//!
//! `H = -Σ p_i · log₂(p_i)`
//!
//! ### 2. Burn — The Thermodynamic Cost
//!
//! Extracting order from chaos is not free. Landauer's principle dictates
//! that erasing one bit of information costs a minimum of:
//!
//! `E ≥ k_B · T · ln(2)`
//!
//! At the silicon level, your CPU performs a microscopic Phoenix cycle
//! millions of times per second — burning old chaotic states into heat to
//! produce deterministic outputs. The proof generation is the flame.
//!
//! ### 3. Extraction — Negentropy Pulled From the Ashes
//!
//! From Brillouin's negentropy principle (1953):
//!
//! > **Negentropy = H_max − H_actual = D_KL(p_informed || p_uninformed)**
//!
//! The information extracted is exactly the entropy reduction. For a ZK proof
//! with N constraints proving threshold T:
//!
//! `N_bits = constraint_count × log₂(threshold)`
//!
//! ### 4. Rebirth — The Verifiable Artifact
//!
//! The output is a pristine, unforgeable artifact — a ZK proof, a settled
//! arbitrage, an anonymous vote. The verifier receives the order without
//! paying the burn cost. The Phoenix is reborn.
//!
//! ## Modules
//!
//! | Module | Phoenix Phase | Origin | Generalized to |
//! |--------|--------------|--------|----------------|
//! | [`entropy`] | Entropy | Shannon (1948) | Any probability distribution |
//! | [`burn`] | Burn | Landauer (1961) | Any irreversible computation |
//! | [`negentropy`] | Extraction | Brillouin (1953), KL divergence | Any observation/constraint |
//! | [`route_energy`] | Extraction | FMD `route_energy.rs` | Any multi-step process |
//! | [`committor`] | Rebirth | FMD `tps.rs` (Transition Path Sampling) | Any rare event prediction |
//! | [`diffusion`] | Entropy → Extraction | FMD formal negentropy model | Any network with information flow |
//! | [`microstructure`] | Burn → Extraction | FMD complex microstructure blog | Any system with phase/timing |
//!
//! ## Quick start
//!
//! ```rust
//! use negentropy::{Negentropy, RouteEnergy, Committor};
//!
//! // Phase 3: Extraction — score a ZK proof (17 constraints, threshold 18)
//! let neg = Negentropy::from_constraints(17, 18);
//! println!("Negentropy extracted: {:.1} bits", neg.bits());
//! // → 70.9 bits
//!
//! // Phase 3: Extraction — score a route through any system
//! let energy = RouteEnergy::new(95.0, 75.0, 1.0, 0.94, 0.001);
//! println!("Route energy: {:.2}", energy.score());
//! // → 779.51
//!
//! // Phase 4: Rebirth — predict a rare event (7/10 trajectories hit target)
//! let comm = Committor::from_ensemble(7, 10);
//! println!("Committor: {:.1}%", comm.probability * 100.0);
//! // → 70.0%
//! ```
//!
//! ## Applications
//!
//! This library powers the energy scoring in:
//!
//! - [zk-age](https://github.com/jjcav84/zk-age) — Privacy-preserving age verification
//! - [zk-attest](https://github.com/jjcav84/zk-attest) — Zero-knowledge attestations on Hedera
//! - [zk-ballot](https://github.com/jjcav84/zk-ballot) — Anonymous on-chain voting with Halo2
//! - [orkid](https://github.com/jjcav84/orkid) — FMD physics engine for MEV detection (origin)
//!
//! ## References
//!
//! - Boltzmann (1877) — Entropy as logarithm of microstates: `S = k_B ln Ω`
//! - Shannon (1948) — Information as entropy reduction: `I = H_before − H_after`
//! - Landauer (1961) — Erasing information costs energy: `E ≥ k_B T ln 2`
//! - Brillouin (1953) — Negentropy principle: information is negative entropy
//! - Kullback & Leibler (1951) — D_KL divergence: `D_KL(p || q) = Σ p log(p/q)`
//! - Bolhuis et al. (2002) — Transition Path Sampling for rare events
//! - orkid blog — ["Blockchain Thermodynamics"](https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-blockchain-thermodynamics-negentropy-mev.md)
//! - orkid blog — ["Negentropy = Information"](https://github.com/jjcav84/orkid/blob/main/blog/2025-11-06-negentropy-information-generalized-framework.md)
//! - orkid blog — ["Formal Negentropy Model"](https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-formal-negentropy-model-mev-dynamics.md)
//! - orkid blog — ["Complex Microstructure and Route Scoring"](https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-complex-microstructure-route-scoring.md)

pub mod constants;
pub mod entropy;
pub mod burn;
pub mod negentropy;
pub mod route_energy;
pub mod committor;
pub mod diffusion;
pub mod microstructure;

pub use constants::{K_B, LN_2, ELEMENTARY_CHARGE};
pub use entropy::Entropy;
pub use burn::Burn;
pub use negentropy::Negentropy;
pub use route_energy::{RouteEnergy, RouteEnergyResult, RoutePotential};
pub use committor::{Committor, CommittorResult};
pub use diffusion::{InformationField, GraphLaplacian, ClosureEquation};
pub use microstructure::{ComplexMicrostructure, PhaseConjugation};
