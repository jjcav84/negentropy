//! Phase 3: Extraction — Negentropy Pulled From the Ashes
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
//! ## Example
//!
//! ```rust
//! use negentropy::Negentropy;
//!
//! // Negentropy extracted by a ZK proof (17 constraints, threshold 18)
//! let neg = Negentropy::from_constraints(17, 18);
//! assert!((neg.bits() - 70.89).abs() < 0.1);
//!
//! // Negentropy as KL divergence between informed and uninformed distributions
//! let neg = Negentropy::kl_divergence(&[0.9, 0.1], &[0.5, 0.5]);
//! assert!(neg.bits() > 0.0);
//! ```

use serde::{Deserialize, Serialize};

/// Negentropy — the information extracted from a system by applying constraints.
///
/// This is the core quantity of the Phoenix cycle: the order that emerges
/// from the ashes after the burn phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negentropy {
    /// Negentropy in bits (Shannon)
    pub value: f64,
    /// Source description (e.g., "ZK proof: 17 constraints, threshold 18")
    pub source: String,
}

impl Negentropy {
    /// Compute negentropy extracted by a constraint-based system (e.g., ZK proof).
    ///
    /// `N = constraint_count × log₂(threshold)`
    ///
    /// Each constraint contributes ~1 bit of negentropy (order from chaos).
    /// The threshold (or tree depth, or anonymity set) determines how much
    /// uncertainty each constraint eliminates.
    pub fn from_constraints(constraint_count: u64, threshold: u64) -> Self {
        let bits = constraint_count as f64 * (threshold.max(1) as f64).log2();
        Self {
            value: bits,
            source: format!(
                "constraints: {} × log₂({}) = {:.1} bits",
                constraint_count, threshold, bits
            ),
        }
    }

    /// Compute negentropy as KL divergence between informed and uninformed distributions.
    ///
    /// `D_KL(p || q) = Σ p_i × log₂(p_i / q_i)`
    ///
    /// This is the fundamental identity: Negentropy = Information = D_KL.
    pub fn kl_divergence(informed: &[f64], uninformed: &[f64]) -> Self {
        assert_eq!(
            informed.len(),
            uninformed.len(),
            "distributions must have same length"
        );

        let kl: f64 = informed
            .iter()
            .zip(uninformed.iter())
            .filter(|(&p, &q)| p > 0.0 && q > 0.0)
            .map(|(&p, &q)| p * (p / q).log2())
            .sum();

        Self {
            value: kl,
            source: format!("D_KL(informed || uninformed) = {:.4} bits", kl),
        }
    }

    /// Compute negentropy from entropy reduction.
    ///
    /// `N = H_before - H_after`
    pub fn from_entropy_reduction(h_before: f64, h_after: f64) -> Self {
        let n = h_before - h_after;
        Self {
            value: n,
            source: format!("H_before={:.2} - H_after={:.2} = {:.2} bits", h_before, h_after, n),
        }
    }

    /// Negentropy in bits (Shannon).
    pub fn bits(&self) -> f64 {
        self.value
    }

    /// Negentropy in nats (natural log).
    pub fn nats(&self) -> f64 {
        self.value * std::f64::consts::LN_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zk_proof_negentropy() {
        // 17 constraints, threshold 18: N = 17 * log2(18) ≈ 70.89
        let neg = Negentropy::from_constraints(17, 18);
        assert!((neg.bits() - 70.89).abs() < 0.1);
    }

    #[test]
    fn test_vote_negentropy() {
        // 20 constraints, tree depth 4: N = 20 * log2(16) = 80
        let neg = Negentropy::from_constraints(20, 16);
        assert!((neg.bits() - 80.0).abs() < 0.1);
    }

    #[test]
    fn test_kl_divergence_identical_distributions() {
        // D_KL(p || p) = 0 — no information gained
        let neg = Negentropy::kl_divergence(&[0.5, 0.5], &[0.5, 0.5]);
        assert!(neg.bits() < 0.001);
    }

    #[test]
    fn test_kl_divergence_informed_vs_uniform() {
        // Informed distribution is concentrated, uninformed is uniform
        let neg = Negentropy::kl_divergence(&[0.99, 0.01], &[0.5, 0.5]);
        assert!(neg.bits() > 0.5); // significant information
    }

    #[test]
    fn test_entropy_reduction() {
        let neg = Negentropy::from_entropy_reduction(4.0, 1.5);
        assert!((neg.bits() - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_higher_threshold_more_negentropy() {
        let low = Negentropy::from_constraints(17, 13);
        let high = Negentropy::from_constraints(17, 25);
        assert!(high.bits() > low.bits());
    }
}
