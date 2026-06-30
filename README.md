# negentropy — The Physics of Information Extraction

> *A Phoenix doesn't just survive; it uses intense thermal energy to burn
> away its outdated form, collapsing its entire state into ash so it can
> reconstitute itself into something pure, ordered, and renewed.*

An open-source thermodynamic engine for scoring any system where
information reduces entropy. Extracted and generalized from the
[orkid FMD physics engine](https://github.com/jjcav84/orkid/tree/main/fmd-physics).

[![License: MIT](https://img.shields.io/badge/License-MIT-ff6b35.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-a78bfa.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-47%20passing-a78bfa.svg)](#tests)

---

## The Phoenix Cycle

Every information extraction — whether a ZK proof, an arbitrage route, or a
vote — passes through the same thermodynamic cycle:

```text
 ┌──────────┐    ┌──────────┐    ┌──────────────┐    ┌──────────┐
 │  ENTROPY │───▶│   BURN   │───▶│  EXTRACTION  │───▶│ REBIRTH  │
 │ (chaos)  │    │ (energy) │    │ (negentropy) │    │ (order)  │
 └──────────┘    └──────────┘    └──────────────┘    └──────────┘
 high uncertainty  Landauer cost   KL divergence      verifiable
 private data       k_B·T·ln(2)     bits extracted     artifact
```

### 1. Entropy — The Old State

Private data, market inefficiency, unverifiable claims — all are
high-entropy states. Without constraints, anyone could claim anything.
Shannon entropy quantifies this uncertainty:

```
H = -Σ p_i · log₂(p_i)
```

### 2. Burn — The Thermodynamic Cost

Extracting order from chaos is not free. Landauer's principle dictates
that erasing one bit of information costs a minimum of:

```
E ≥ k_B · T · ln(2)
```

At the silicon level, your CPU performs a microscopic Phoenix cycle
millions of times per second — burning old chaotic states into heat to
produce deterministic outputs. The proof generation is the flame.

### 3. Extraction — Negentropy Pulled From the Ashes

From Brillouin's negentropy principle (1953):

> **Negentropy = H_max − H_actual = D_KL(p_informed || p_uninformed)**

The information extracted is exactly the entropy reduction. For a ZK proof
with N constraints proving threshold T:

```
N_bits = constraint_count × log₂(threshold)
```

### 4. Rebirth — The Verifiable Artifact

The output is a pristine, unforgeable artifact — a ZK proof, a settled
arbitrage, an anonymous vote. The verifier receives the order without
paying the burn cost. The Phoenix is reborn.

---

## Modules

| Module | Phoenix Phase | Origin | Generalized to |
|--------|---------------|--------|----------------|
| `entropy` | Entropy | Shannon (1948) | Any probability distribution |
| `burn` | Burn | Landauer (1961) | Any irreversible computation |
| `negentropy` | Extraction | Brillouin (1953), KL divergence | Any observation/constraint |
| `route_energy` | Extraction | FMD `route_energy.rs` | Any multi-step process |
| `committor` | Rebirth | FMD `tps.rs` (Transition Path Sampling) | Any rare event prediction |
| `diffusion` | Entropy → Extraction | FMD formal negentropy model | Any network with information flow |
| `microstructure` | Burn → Extraction | FMD complex microstructure blog | Any system with phase/timing |

---

## Quick start

### As a library

```rust
use negentropy::{Negentropy, RouteEnergy, Committor};

// Phase 3: Extraction — score a ZK proof (17 constraints, threshold 18)
let neg = Negentropy::from_constraints(17, 18);
println!("Negentropy extracted: {:.1} bits", neg.bits());
// → 70.9 bits

// Phase 3: Extraction — score a route through any system
let energy = RouteEnergy::new(95.0, 75.0, 1.0, 0.94, 0.001);
println!("Route energy: {:.2}", energy.score());
// → 779.51

// Phase 4: Rebirth — predict a rare event (7/10 trajectories hit target)
let comm = Committor::from_ensemble(7, 10);
println!("Committor: {:.1}%", comm.probability * 100.0);
// → 70.0%
```

### As a CLI

```bash
cargo run --bin negentropy -- score --constraints 17 --threshold 18
cargo run --bin negentropy -- route --confidence 95 --depth 75 --latency 800 --cost 0.001
cargo run --bin negentropy -- committor --hits 7 --total 10
cargo run --bin negentropy -- entropy --probs 0.5,0.5
cargo run --bin negentropy -- burn --bits 70.9 --temperature 300
cargo run --bin negentropy -- theory
```

Example output (`score`):

```json
{
  "phase": "extraction",
  "negentropy_bits": 70.89868804427568,
  "negentropy_nats": 49.09868804427568,
  "source": "constraints: 17 × log₂(18) = 70.9 bits",
  "formula": "N = constraint_count × log₂(threshold)"
}
```

### Web demo

A single-file, dependency-free interactive demo lives in [`web/index.html`](web/index.html).
Open it in any browser to compute a Phoenix energy score live.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
negentropy = { git = "https://github.com/jjcav84/negentropy.git" }
```

Or build locally:

```bash
git clone https://github.com/jjcav84/negentropy.git
cd negentropy
cargo build --release
```

Requires Rust 1.70+ (edition 2021).

---

## Tests

```bash
cargo test
```

47 tests covering all four Phoenix phases: entropy validation, Landauer
scaling, KL divergence, route energy decay, committor ensembles, graph
diffusion, and complex microstructure phase conjugation.

---

## The negentropy ecosystem

`negentropy` is the generalized extraction of the physics engine that powers a
family of ZK and DeFi projects. It lives in a sibling workspace alongside them:

```text
web3-defi/
├── negentropy/     ← this repo — the generalized thermodynamic engine
├── orkid/          ← origin: FMD physics engine for MEV detection
├── zk-age/         ← privacy-preserving age verification
├── zk-attest/      ← zero-knowledge attestations on Hedera
└── zk-ballot/      ← anonymous on-chain voting with Halo2
```

### How they connect

Every sibling project applies the same Phoenix cycle (entropy → burn →
extraction → rebirth) to a different domain. `negentropy` is the shared
library that codifies the physics; each sibling maps its domain-specific
quantities onto the same formula:

```text
energy = confidence × √(depth_ratio × timing_factor) × latency_decay × (1 − cost_penalty)
```

| Project | Confidence | Depth | Timing | Latency | Cost | Vendored module |
|---------|-----------|-------|--------|---------|------|-----------------|
| **orkid** | pool TVL / net bps | liquidity depth | hop recency | stage latency | gas | `fmd-physics/src/route_energy.rs` (origin) |
| **zk-age** | issuer trust | credential strength | attestation age | proof gen+verify | zkVerify fee | `backend/src/attestation_energy.rs` |
| **zk-attest** | attestation weight | credential depth | attestation recency | HCS+proof latency | HBAR cost | `backend/src/attestation_energy.rs` |
| **zk-ballot** | merkle tree depth | anonymity set | vote recency | Halo2 proof time | gas | `src/ballot_energy.rs` |

### Current state: vendored → shared library

Today each zk-* sibling **vendors** its own copy of the FMD energy formula
(`attestation_energy.rs` / `ballot_energy.rs`), each citing orkid as the
origin. `negentropy` was extracted to be the single source of truth — the
goal is for each sibling to drop its vendored module and depend on
`negentropy` directly:

```toml
# In a sibling's Cargo.toml (roadmap)
[dependencies]
negentropy = { git = "https://github.com/jjcav84/negentropy.git" }
```

```rust
// Before (vendored, ~250 lines per repo):
use crate::attestation_energy::ProofPotential;

// After (shared library):
use negentropy::RoutePotential;
```

Until that migration is complete, the vendored modules and `negentropy`
implement the same formula and produce the same scores. The sibling repos
remain the authoritative application code; `negentropy` is the canonical
physics.

### Origin

`negentropy` was extracted and generalized from the
[orkid FMD physics engine](https://github.com/jjcav84/orkid/tree/main/fmd-physics).
The orkid blog posts are the primary theoretical references for the
formal negentropy model and complex microstructure scoring.

---

## References

- Boltzmann (1877) — Entropy as logarithm of microstates: `S = k_B ln Ω`
- Shannon (1948) — Information as entropy reduction: `I = H_before − H_after`
- Landauer (1961) — Erasing information costs energy: `E ≥ k_B T ln 2`
- Brillouin (1953) — Negentropy principle: information is negative entropy
- Kullback & Leibler (1951) — D_KL divergence: `D_KL(p || q) = Σ p log(p/q)`
- Bolhuis et al. (2002) — Transition Path Sampling for rare events
- orkid blog — ["Blockchain Thermodynamics"](https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-blockchain-thermodynamics-negentropy-mev.md)
- orkid blog — ["Negentropy = Information"](https://github.com/jjcav84/orkid/blob/main/blog/2025-11-06-negentropy-information-generalized-framework.md)
- orkid blog — ["Formal Negentropy Model"](https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-formal-negentropy-model-mev-dynamics.md)
- orkid blog — ["Complex Microstructure and Route Scoring"](https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-complex-microstructure-route-scoring.md)

---

## License

MIT — see [LICENSE](LICENSE).
