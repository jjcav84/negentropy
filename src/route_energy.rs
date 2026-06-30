//! Phase 3: Extraction — Route Energy
//!
//! Adapted from the orkid FMD physics engine (`fmd-physics/src/route_energy.rs`).
//!
//! The route energy formula scores any multi-step process by net output,
//! depth, timing, and cost. Originally designed for MEV arbitrage route
//! scoring, it generalizes to any system with:
//! - A confidence/depth metric (liquidity, trust, registry size)
//! - A timing factor (recency, hops, staleness)
//! - A latency decay (execution speed)
//! - A cost penalty (gas, verification fee, compute)
//!
//! ## The Formula
//!
//! FMD route energy (orkid):
//! `energy = net_bps × √(depth_ratio × timing_factor) × latency_decay × (1 − gas_penalty)`
//!
//! Generalized route energy:
//! `energy = confidence × √(depth_ratio × timing_factor) × latency_decay × (1 − cost_penalty)`
//!
//! ## Example
//!
//! ```rust
//! use negentropy::RouteEnergy;
//!
//! let energy = RouteEnergy::new(95.0, 75.0, 1.0, 0.94, 0.001);
//! assert!(energy.score() > 700.0);
//! ```

use serde::{Deserialize, Serialize};

/// Route energy evaluation result.
///
/// Contains the final energy score and all component factors.
///
/// 48 bytes (6 × f64), implements `Copy`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RouteEnergyResult {
    /// Total energy score (higher = better)
    pub energy: f64,
    /// Confidence input (e.g., issuer trust × 100, pool TVL)
    pub confidence: f64,
    /// Depth ratio (confidence relative to difficulty/threshold)
    pub depth_ratio: f64,
    /// Timing factor (recency/hops decay, 0..1)
    pub timing_factor: f64,
    /// Latency decay (execution speed, 0..1)
    pub latency_decay: f64,
    /// Cost penalty (0..1, higher = more expensive)
    pub cost_penalty: f64,
}

/// Configuration for route energy evaluation.
///
/// Generalized from `RoutePotential` in orkid's `route_energy.rs`.
///
/// 56 bytes, implements `Copy`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RoutePotential {
    /// Confidence score (0..100) — analogous to pool TVL or issuer trust × 100
    pub confidence: f64,
    /// Depth ratio — confidence relative to difficulty (e.g., confidence / log₁₀(threshold))
    pub depth_ratio: f64,
    /// Timing factor (0..1) — recency decay or hop penalty
    pub timing_factor: f64,
    /// Total latency in milliseconds (proof gen + verify, or route execution)
    pub latency_ms: u64,
    /// Cost in USD (gas, verification fee, compute cost)
    pub cost_usd: f64,
    /// Cost normalization factor (maps USD to 0..1 penalty)
    pub cost_normalization: f64,
}

impl Default for RoutePotential {
    fn default() -> Self {
        Self {
            confidence: 95.0,
            depth_ratio: 75.0,
            timing_factor: 1.0,
            latency_ms: 800,
            cost_usd: 0.001,
            cost_normalization: 0.01,
        }
    }
}

impl RoutePotential {
    /// Evaluate route energy using the FMD physics formula.
    ///
    /// `energy = confidence × √(depth_ratio × timing_factor) × latency_decay × (1 − cost_penalty)`
    pub fn energy(&self) -> RouteEnergyResult {
        let latency_decay = 1.0 / (1.0 + self.latency_ms as f64 * 0.0001);
        let cost_penalty = (self.cost_usd * self.cost_normalization).min(0.5);

        let energy = self.confidence
            * (self.depth_ratio * self.timing_factor).sqrt()
            * latency_decay
            * (1.0 - cost_penalty).max(0.0);

        RouteEnergyResult {
            energy,
            confidence: self.confidence,
            depth_ratio: self.depth_ratio,
            timing_factor: self.timing_factor,
            latency_decay,
            cost_penalty,
        }
    }
}

/// Convenience constructor for route energy from raw factors.
///
/// Direct adaptation of the FMD formula — useful when you've already
/// computed the component factors and just need the score.
#[derive(Debug, Clone)]
pub struct RouteEnergy;

impl RouteEnergy {
    /// Compute route energy from pre-computed factors.
    ///
    /// `energy = confidence × √(depth_ratio × timing_factor) × latency_decay × (1 − cost_penalty)`
    #[allow(clippy::new_ret_no_self, reason = "RouteEnergy is a namespace unit struct; new returns the result")]
    pub fn new(
        confidence: f64,
        depth_ratio: f64,
        timing_factor: f64,
        latency_decay: f64,
        cost_penalty: f64,
    ) -> RouteEnergyResult {
        let energy = confidence
            * (depth_ratio * timing_factor).sqrt()
            * latency_decay
            * (1.0 - cost_penalty).max(0.0);

        RouteEnergyResult {
            energy,
            confidence,
            depth_ratio,
            timing_factor,
            latency_decay,
            cost_penalty,
        }
    }

    /// Compute route energy with timing decay from age.
    ///
    /// `timing_factor = exp(−age / half_life)`
    pub fn with_decay(
        confidence: f64,
        depth_ratio: f64,
        age_secs: f64,
        half_life_secs: f64,
        latency_ms: u64,
        cost_usd: f64,
        cost_normalization: f64,
    ) -> RouteEnergyResult {
        let timing_factor = (-age_secs / half_life_secs).exp();
        let latency_decay = 1.0 / (1.0 + latency_ms as f64 * 0.0001);
        let cost_penalty = (cost_usd * cost_normalization).min(0.5);

        Self::new(
            confidence,
            depth_ratio,
            timing_factor,
            latency_decay,
            cost_penalty,
        )
    }
}

impl RouteEnergyResult {
    /// The final energy score (higher = better).
    pub fn score(&self) -> f64 {
        self.energy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_energy() {
        let result = RouteEnergy::new(95.0, 75.0, 1.0, 0.94, 0.001);
        assert!(result.energy > 700.0);
        assert!(result.energy < 900.0);
    }

    #[test]
    fn test_zero_confidence_zero_energy() {
        let result = RouteEnergy::new(0.0, 75.0, 1.0, 0.94, 0.001);
        assert!(result.energy < 0.001);
    }

    #[test]
    fn test_stale_decays() {
        let fresh = RouteEnergy::with_decay(95.0, 75.0, 0.0, 3600.0, 800, 0.001, 0.01);
        let stale = RouteEnergy::with_decay(95.0, 75.0, 7200.0, 3600.0, 800, 0.001, 0.01);
        assert!(stale.energy < fresh.energy);
    }

    #[test]
    fn test_high_cost_reduces_energy() {
        let cheap = RouteEnergy::new(95.0, 75.0, 1.0, 0.94, 0.001);
        let expensive = RouteEnergy::new(95.0, 75.0, 1.0, 0.94, 0.3);
        assert!(expensive.energy < cheap.energy);
    }

    #[test]
    fn test_potential_config() {
        let pot = RoutePotential::default();
        let result = pot.energy();
        assert!(result.energy > 0.0);
        assert!(result.latency_decay > 0.0 && result.latency_decay < 1.0);
    }
}
