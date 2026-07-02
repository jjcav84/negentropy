<p align="center">
  <a href="https://www.orkidlabs.com"><img src="assets/logo.png" alt="Orkid Labs" width="220" /></a>
</p>

# negentropy — The Physics of Information Extraction

> *A Phoenix doesn't just survive; it uses intense thermal energy to burn
> away its outdated form, collapsing its entire state into ash so it can
> reconstitute itself into something pure, ordered, and renewed.*
>
> **By [Orkid Labs](https://www.orkidlabs.com)** — privacy-first crypto engineering

An open-source thermodynamic engine for scoring any system where
information reduces entropy. Extracted and generalized from the
[orkid FMD physics engine](https://github.com/jjcav84/orkid/tree/main/fmd-physics).

> **Note:** The orkid repository is private. Access can be provided to
> Thrive Protocol reviewers and other appropriate cases on request —
> contact [Orkid Labs](https://www.orkidlabs.com). The theoretical
> foundation is published as a peer-reviewed preprint:
> ["Negative EV per Unit Time as Blockchain Inefficiency"](https://www.researchgate.net/publication/399474539_Negative_EV_per_Unit_Time_as_Blockchain_Inefficiency)
> — [Jacob Cavazos, ResearchGate](https://www.researchgate.net/profile/Jacob-Cavazos).

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
├── negentropy/         ← this repo — the generalized thermodynamic engine
├── orkid/              ← origin: FMD physics engine for MEV detection
├── zenkinetic/         ← thermodynamic privacy gate for Horizen Base L3
├── horizen-age/        ← privacy-preserving age verification on Horizen
├── horizen-attest/     ← ZK attestations on Horizen
├── horizen-ballot/     ← anonymous on-chain voting on Horizen
├── zk-age/             ← privacy-preserving age verification (original)
├── zk-attest/          ← zero-knowledge attestations on Hedera (original)
└── zk-ballot/          ← anonymous on-chain voting with Halo2 (original)
```

### How they connect

Every sibling project applies the same Phoenix cycle (entropy → burn →
extraction → rebirth) to a different domain. `negentropy` is the shared
library that codifies the physics; each sibling maps its domain-specific
quantities onto the same formula:

```text
energy = confidence × √(depth_ratio × timing_factor) × latency_decay × (1 − cost_penalty)
```

| Project | Confidence | Depth | Timing | Latency | Cost | Adapter module |
|---------|-----------|-------|--------|---------|------|----------------|
| **orkid** | pool TVL / net bps | liquidity depth | hop recency | stage latency | gas | `fmd-physics/src/route_energy.rs` (origin) |
| **zenkinetic** | tx kind weight | anonymity set | proof age | proof gen+verify | ZEN stake discount | `src/gate.rs` |
| **horizen-age** | issuer trust | constraint count | proof age | proof gen+verify | ZEN stake (Pro) | `src/session.rs` |
| **horizen-attest** | attestation kind | constraint count | attestation age | proof gen+verify | ZEN stake (Basic) | `src/session.rs` |
| **horizen-ballot** | registry trust | tree depth | vote age | Halo2 proof time | ZEN stake (Pro) | `src/session.rs` |
| **zk-age** | issuer trust | credential strength | attestation age | proof gen+verify | zkVerify fee | `backend/src/attestation_energy.rs` |
| **zk-attest** | attestation weight | credential depth | attestation recency | HCS+proof latency | HBAR cost | `backend/src/attestation_energy.rs` |
| **zk-ballot** | merkle tree depth | anonymity set | vote recency | Halo2 proof time | gas | `src/ballot_energy.rs` |

### Current state: shared library

All three zk-* siblings now depend on `negentropy` directly — the vendored
FMD physics modules have been replaced with thin domain adapters that
delegate the core formula to this crate:

```toml
# In a sibling's Cargo.toml
[dependencies]
negentropy = { git = "https://github.com/jjcav84/negentropy.git" }
```

```rust
// Each sibling's energy module is now a thin adapter:
use negentropy::{Committor, Negentropy, RouteEnergy};

// Domain-specific mapping → core physics delegation
let energy = RouteEnergy::new(confidence, depth_ratio, timing, latency, cost).energy;
let committor = Committor::score(depth_ratio, timing, cost);
let negentropy_bits = Negentropy::from_constraints(constraints, threshold).bits();
```

Each adapter keeps only its domain mapping (issuer trust → confidence,
attestation type → base depth, tree depth → anonymity set) — roughly 60%
less code per repo, with the physics maintained in one place. The sibling
repos remain the authoritative application code; `negentropy` is the
canonical physics.

The `horizen-*` repos are Horizen-native adaptations of the `zk-*` repos —
they add ZEN token staking and ZenKinetic privacy gate integration on top
of the same negentropy scoring, deploying on Horizen Base L3.

### Origin

`negentropy` was extracted and generalized from the
[orkid FMD physics engine](https://github.com/jjcav84/orkid/tree/main/fmd-physics)
(private repo — access available for reviewers on request). The
theoretical foundation is published as a peer-reviewed preprint:
["Negative EV per Unit Time as Blockchain Inefficiency"](https://www.researchgate.net/publication/399474539_Negative_EV_per_Unit_Time_as_Blockchain_Inefficiency)
by [Jacob Cavazos](https://www.researchgate.net/profile/Jacob-Cavazos).
The route scoring methodology is documented in the [orkid blog series](https://www.orkidlabs.com/blog/).

---

## References

- Boltzmann (1877) — Entropy as logarithm of microstates: `S = k_B ln Ω`
- Shannon (1948) — Information as entropy reduction: `I = H_before − H_after`
- Landauer (1961) — Erasing information costs energy: `E ≥ k_B T ln 2`
- Brillouin (1953) — Negentropy principle: information is negative entropy
- Kullback & Leibler (1951) — D_KL divergence: `D_KL(p || q) = Σ p log(p/q)`
- Bolhuis et al. (2002) — Transition Path Sampling for rare events
- Cavazos, J. — ["Negative EV per Unit Time as Blockchain Inefficiency"](https://www.researchgate.net/publication/399474539_Negative_EV_per_Unit_Time_as_Blockchain_Inefficiency) (preprint)
- orkid blog — ["Blockchain Thermodynamics: How Negentropy Explains MEV, Consensus, and Arbitrage"](https://www.orkidlabs.com/blog/blockchain-thermodynamics-negentropy-mev-physics/)
- orkid blog — ["Negentropy = Information: A Generalized Mathematical Framework"](https://www.orkidlabs.com/blog/negentropy-information-generalized-framework/)
- orkid blog — ["A Formal Mathematical Model of Blockchain Negentropy and MEV Dynamics"](https://www.orkidlabs.com/blog/formal-negentropy-model-mev-dynamics-graph-diffusion/)
- orkid blog — ["Complex Microstructure and Route Scoring in DeFi: Beyond Simple EV"](https://www.orkidlabs.com/blog/complex-microstructure-route-scoring-defi/)

---

## Thrive Ecosystem Alignment

negentropy is the open-source physics core extracted from [`orkid/fmd-physics`](https://github.com/jjcav84/orkid/tree/main/fmd-physics) — the Financial Molecular Dynamics route scoring engine that powers orkid's private DeFi MEV protection system (195+ live fills on Base/Ethereum). The route scoring model is documented in the orkid blog series on blockchain thermodynamics and route energy scoring. We open-sourced it as a standalone crate to give Thrive's grant ecosystem a universal, physics-based quality metric for ZK proofs.

This is not a standalone grant application. It is the shared dependency that every Thrive-ecosystem project builds on:

| Thrive Program | Project | How negentropy is used |
|---------------|---------|----------------------|
| zkVerify Web2 (#44) | zk-age | Scores age proof quality (negentropy bits extracted) |
| zkVerify Web3 (#45) | zk-attest | Scores attestation proof quality |
| zkVerify Web3 (#45) | zk-ballot | Scores vote proof privacy (anonymity set negentropy) |
| Horizen Genesis (#38) | zenkinetic | Powers the thermodynamic privacy gate (fee determination) |
| Horizen Genesis (#38) | horizen-ballot | Scores vote proof privacy on Horizen L3 |
| Horizen Boost (#39) | horizen-age | Scores age proof quality on Horizen L3 |
| Horizen Boost (#39) | horizen-attest | Scores attestation proof quality on Horizen L3 |

**Origin:** The FMD route scoring engine in `orkid/fmd-physics` (private repo — access available for reviewers on request) scores arbitrage routes by their thermodynamic energy — `energy = confidence × √(depth_ratio × timing_factor) × latency_decay × (1 − cost_penalty)`. The same physics generalizes to any system where information reduces entropy. A ZK proof eliminates uncertainty the same way an arbitrage route eliminates market inefficiency. The math is identical; the application domain changes. The theoretical foundation is published as a peer-reviewed preprint: ["Negative EV per Unit Time as Blockchain Inefficiency"](https://www.researchgate.net/publication/399474539_Negative_EV_per_Unit_Time_as_Blockchain_Inefficiency) by [Jacob Cavazos](https://www.researchgate.net/profile/Jacob-Cavazos). Route scoring methodology is documented in the [orkid blog series](https://www.orkidlabs.com/blog/): [Blockchain Thermodynamics](https://www.orkidlabs.com/blog/blockchain-thermodynamics-negentropy-mev-physics/), [Negentropy = Information](https://www.orkidlabs.com/blog/negentropy-information-generalized-framework/), [Formal Negentropy Model](https://www.orkidlabs.com/blog/formal-negentropy-model-mev-dynamics-graph-diffusion/), and [Complex Microstructure and Route Scoring](https://www.orkidlabs.com/blog/complex-microstructure-route-scoring-defi/).

**Proposal to Thrive:** Adopt negentropy as a universal proof quality metric across Thrive grant programs. Every ZK project in the Thrive ecosystem gets scored on the same thermodynamic scale — `N = constraints × log₂(anonymity_set)` bits of negentropy extracted. This gives Guardians a deterministic, physics-based metric for evaluating ZK proof quality across all programs, rather than subjective rubrics. The engine is battle-tested in production MEV route scoring; applying it to ZK proof scoring is a domain transfer, not a research project.

## About

Built by [Orkid Labs](https://www.orkidlabs.com) — a privacy-first crypto
engineering lab building thermodynamic infrastructure for decentralized
systems. See our other work at [orkidlabs.com](https://www.orkidlabs.com).

## License

MIT — see [LICENSE](LICENSE).
