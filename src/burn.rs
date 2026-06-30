//! Phase 2: Burn — The Thermodynamic Cost
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
//! ## Example
//!
//! ```rust
//! use negentropy::Burn;
//!
//! // Energy cost to erase 70.9 bits at room temperature (300K)
//! let burn = Burn::landauer(70.9, 300.0);
//! println!("Minimum energy: {:.2e} J", burn.energy_joules);
//! ```

use serde::{Deserialize, Serialize};

use crate::constants::{ELEMENTARY_CHARGE, K_B, LN_2};

/// The thermodynamic cost of extracting negentropy.
///
/// This is the "burn" phase of the Phoenix cycle — the energy paid to
/// collapse uncertainty into deterministic order.
///
/// 32 bytes, implements `Copy`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Burn {
    /// Number of bits erased (negentropy extracted)
    pub bits_erased: f64,
    /// Temperature in Kelvin
    pub temperature_k: f64,
    /// Minimum energy required (Joules) — Landauer limit
    pub energy_joules: f64,
    /// Energy efficiency vs Landauer limit (real systems are ~10^9x above)
    pub efficiency_factor: f64,
}

impl Burn {
    /// Compute the Landauer limit for erasing N bits at temperature T.
    ///
    /// `E = k_B · T · ln(2) · N_bits`
    ///
    /// This is the **theoretical minimum** energy. Real CPUs operate
    /// billions of times above this limit.
    pub fn landauer(bits: f64, temperature_k: f64) -> Self {
        let energy = K_B * temperature_k * LN_2 * bits;
        Self {
            bits_erased: bits,
            temperature_k,
            energy_joules: energy,
            efficiency_factor: 1.0,
        }
    }

    /// Estimate real CPU energy cost for a computation.
    ///
    /// `E_real = power_watts × time_seconds`
    ///
    /// Real systems operate at ~10^9 times the Landauer limit due to
    /// transistor switching overhead, leakage, and cooling. The actual
    /// energy is dominated by power × time, not per-bit cost.
    pub fn cpu_cost(power_watts: f64, time_seconds: f64) -> f64 {
        power_watts * time_seconds
    }

    /// The Landauer cost in eV (electron-volts), useful for semiconductor physics.
    pub fn ev(&self) -> f64 {
        self.energy_joules / ELEMENTARY_CHARGE
    }

    /// Ratio of real CPU energy to the Landauer minimum.
    ///
    /// Real systems are typically 10^8 to 10^10 times above the limit.
    pub fn efficiency_ratio(&self, real_energy_j: f64) -> f64 {
        if self.energy_joules > 0.0 {
            real_energy_j / self.energy_joules
        } else {
            f64::INFINITY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landauer_single_bit() {
        let burn = Burn::landauer(1.0, 300.0);
        // E = k_B * T * ln(2) ≈ 2.87e-21 J at 300K
        assert!(burn.energy_joules > 2.8e-21 && burn.energy_joules < 3.0e-21);
    }

    #[test]
    fn test_landauer_scales_linearly() {
        let one_bit = Burn::landauer(1.0, 300.0);
        let ten_bits = Burn::landauer(10.0, 300.0);
        assert!((ten_bits.energy_joules / one_bit.energy_joules - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_temperature_scales_linearly() {
        let cold = Burn::landauer(1.0, 100.0);
        let hot = Burn::landauer(1.0, 300.0);
        assert!((hot.energy_joules / cold.energy_joules - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_zk_proof_burn() {
        // A ZK proof extracting 70.9 bits at 300K
        let burn = Burn::landauer(70.9, 300.0);
        assert!(burn.energy_joules > 0.0);
        // E = k_B * T * ln(2) * 70.9 ≈ 2.03e-19 J
        assert!(burn.energy_joules < 3.0e-19, "got {} J", burn.energy_joules);
    }

    #[test]
    fn test_cpu_cost() {
        // 50W for 0.8s = 40J
        let cost = Burn::cpu_cost(50.0, 0.8);
        assert!((cost - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_efficiency_ratio() {
        let burn = Burn::landauer(70.9, 300.0);
        let real = 40.0; // 40J real CPU cost
        let ratio = burn.efficiency_ratio(real);
        assert!(ratio > 1e18); // way above Landauer
    }
}
