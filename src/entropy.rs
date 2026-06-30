//! Phase 1: Entropy — The Old State
//!
//! Shannon entropy quantifies the uncertainty of a system before information
//! is extracted. This is the chaos the Phoenix burns away.
//!
//! `H = -Σ p_i · log₂(p_i)`
//!
//! Maximum entropy (uniform distribution): `H_max = log₂(N)`
//!
//! ## Example
//!
//! ```rust
//! use negentropy::Entropy;
//!
//! // Entropy of a fair coin (50/50)
//! let h = Entropy::from_probabilities(&[0.5, 0.5]);
//! assert!((h.bits() - 1.0).abs() < 0.001);
//!
//! // Entropy of a loaded die
//! let h = Entropy::from_probabilities(&[0.5, 0.25, 0.125, 0.125]);
//! assert!(h.bits() > 1.0 && h.bits() < 2.0);
//! ```

use serde::{Deserialize, Serialize};

use crate::constants::K_B;

/// Shannon entropy of a probability distribution.
///
/// Measures the uncertainty (chaos) of a system before information extraction.
///
/// This struct is 16 bytes (f64 + usize) and implements `Copy` — pass by value
/// to avoid unnecessary clones.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Entropy {
    /// The entropy value in bits
    pub value: f64,
    /// Number of possible states
    pub n_states: usize,
}

impl Entropy {
    /// Compute Shannon entropy from a probability distribution.
    ///
    /// `H = -Σ p_i · log₂(p_i)`
    ///
    /// Panics if probabilities don't sum to ~1.0 or any is negative.
    ///
    /// Single-pass: validates and computes in one iteration.
    pub fn from_probabilities(probs: &[f64]) -> Self {
        let mut sum = 0.0_f64;
        let mut h = 0.0_f64;

        for &p in probs {
            assert!(p >= 0.0, "probabilities must be non-negative, got {}", p);
            sum += p;
            if p > 0.0 {
                h -= p * p.log2();
            }
        }

        assert!(
            (sum - 1.0).abs() < 0.01,
            "probabilities must sum to 1.0, got {}",
            sum
        );

        Self {
            value: h,
            n_states: probs.len(),
        }
    }

    /// Maximum entropy for N states (uniform distribution).
    ///
    /// `H_max = log₂(N)`
    pub fn max(n_states: usize) -> Self {
        Self {
            value: (n_states as f64).log2(),
            n_states,
        }
    }

    /// Entropy in bits (Shannon).
    pub fn bits(&self) -> f64 {
        self.value
    }

    /// Entropy in nats (natural log, used in thermodynamics).
    ///
    /// `H_nats = H_bits × ln(2)`
    pub fn nats(&self) -> f64 {
        self.value * std::f64::consts::LN_2
    }

    /// Entropy in Boltzmann units (used in statistical mechanics).
    ///
    /// `S = k_B × H_nats`
    pub fn boltzmann(&self) -> f64 {
        self.nats() * K_B
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fair_coin_entropy() {
        let h = Entropy::from_probabilities(&[0.5, 0.5]);
        assert!((h.bits() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_fair_die_entropy() {
        let h = Entropy::from_probabilities(&[1.0 / 6.0; 6]);
        assert!((h.bits() - 6.0f64.log2()).abs() < 0.001);
    }

    #[test]
    fn test_certain_outcome_zero_entropy() {
        let h = Entropy::from_probabilities(&[1.0, 0.0]);
        assert!(h.bits() < 0.001);
    }

    #[test]
    fn test_max_entropy() {
        let h = Entropy::max(16);
        assert!((h.bits() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_nats_conversion() {
        let h = Entropy::from_probabilities(&[0.5, 0.5]);
        assert!((h.nats() - std::f64::consts::LN_2).abs() < 0.001);
    }
}
