//! Phase 4: Rebirth — Committor Function (Rare Event Prediction)
//!
//! Adapted from the orkid FMD physics engine (`fmd-physics/src/tps.rs`).
//!
//! The committor function q(x) is borrowed directly from Transition Path
//! Sampling (TPS) in statistical mechanics — it quantifies the probability
//! of reaching a target state from the current state.
//!
//! In the Phoenix cycle, this is the rebirth phase: after the burn and
//! extraction, what is the probability that the resulting artifact is valid,
//! uncontested, and achieves its purpose?
//!
//! ## The Formula
//!
//! `q(x) = hits / total_trajectories`
//!
//! For ensemble-based TPS:
//! - `predicted_time = avg_steps × dt`
//! - `confidence = 1 / (1 + std_dev / avg_steps)`
//!
//! ## Example
//!
//! ```rust
//! use negentropy::Committor;
//!
//! // 7 out of 10 trajectories reached the target state
//! let comm = Committor::from_ensemble(7, 10);
//! assert!((comm.probability - 0.7).abs() < 0.001);
//! assert!(comm.confidence > 0.0);
//! ```

use serde::{Deserialize, Serialize};

/// Committor calculation result — the probability of reaching a target state.
///
/// Adapted from `CommittorResult` in orkid's `tps.rs`.
///
/// 40 bytes (f64 + 2×usize + 2×f64), implements `Copy`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CommittorResult {
    /// Committor probability (0.0 to 1.0)
    pub probability: f64,
    /// Number of trajectories that hit the target
    pub hits: usize,
    /// Total trajectories sampled
    pub total: usize,
    /// Predicted time to reach target (in arbitrary units)
    pub predicted_time: f64,
    /// Confidence in the prediction (0.0 to 1.0)
    pub confidence: f64,
}

/// Committor function — probability of reaching a target state.
///
/// This is the rebirth phase of the Phoenix cycle: the probability that
/// the extracted negentropy results in a valid, uncontested artifact.
#[derive(Debug, Clone, Copy)]
pub struct Committor;

impl Committor {
    /// Compute committor from ensemble sampling results.
    ///
    /// `q(x) = hits / total`
    pub fn from_ensemble(hits: usize, total: usize) -> CommittorResult {
        assert!(total > 0, "total trajectories must be > 0");
        let probability = hits as f64 / total as f64;
        CommittorResult {
            probability,
            hits,
            total,
            predicted_time: 0.0,
            confidence: 1.0 / (1.0 + 1.0 / total as f64),
        }
    }

    /// Compute committor with timing prediction from path lengths.
    ///
    /// `predicted_time = avg_path_length × dt`
    /// `confidence = 1 / (1 + std_dev / avg)`
    ///
    /// Uses Welford's algorithm for single-pass mean and variance.
    pub fn with_timing(
        hits: usize,
        total: usize,
        path_lengths: &[usize],
        dt: f64,
    ) -> CommittorResult {
        assert!(total > 0, "total trajectories must be > 0");
        let probability = hits as f64 / total as f64;

        let (predicted_time, confidence) = if !path_lengths.is_empty() {
            // Welford's algorithm — single pass for mean and variance
            let mut count = 0.0_f64;
            let mut mean = 0.0_f64;
            let mut m2 = 0.0_f64;

            for &len in path_lengths {
                count += 1.0;
                let delta = len as f64 - mean;
                mean += delta / count;
                let delta2 = len as f64 - mean;
                m2 += delta * delta2;
            }

            let variance = m2 / count;
            let std_dev = variance.sqrt();
            let time = mean * dt;

            let conf = if mean <= f64::EPSILON {
                1.0
            } else {
                1.0 / (1.0 + std_dev / mean)
            };

            (time, conf)
        } else {
            (-1.0, 0.0)
        };

        CommittorResult {
            probability,
            hits,
            total,
            predicted_time,
            confidence,
        }
    }

    /// Simplified committor for scoring systems (no ensemble needed).
    ///
    /// Uses depth, timing, and cost as features for a probability estimate.
    /// Adapted from the TPS committor in the FMD engine.
    ///
    /// `committor = (depth / (1 + depth)) × timing × (1 − cost × 0.5)`
    pub fn score(depth_ratio: f64, timing_factor: f64, cost_penalty: f64) -> f64 {
        (depth_ratio / (1.0 + depth_ratio))
            * timing_factor
            * (1.0 - cost_penalty * 0.5).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensemble_committor() {
        let comm = Committor::from_ensemble(7, 10);
        assert!((comm.probability - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_all_hits() {
        let comm = Committor::from_ensemble(10, 10);
        assert!((comm.probability - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_no_hits() {
        let comm = Committor::from_ensemble(0, 10);
        assert!(comm.probability < 0.001);
    }

    #[test]
    fn test_timing_prediction() {
        let comm = Committor::with_timing(5, 10, &[3, 5, 4, 6, 2], 0.1);
        assert!(comm.predicted_time > 0.0);
        assert!(comm.confidence > 0.0 && comm.confidence <= 1.0);
    }

    #[test]
    fn test_score_function() {
        let p = Committor::score(75.0, 1.0, 0.001);
        assert!(p > 0.98); // high depth, fresh, low cost → high probability
    }

    #[test]
    fn test_score_low_depth() {
        let high = Committor::score(75.0, 1.0, 0.001);
        let low = Committor::score(1.0, 1.0, 0.001);
        assert!(low < high);
    }
}
