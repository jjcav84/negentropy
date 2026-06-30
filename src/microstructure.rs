//! Phase 2→3: Burn to Extraction — Complex Microstructure
//!
//! Adapted from the orkid blog post:
//! ["Complex Microstructure and Route Scoring in DeFi: Beyond Simple EV"]
//! (https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-complex-microstructure-route-scoring.md)
//!
//! Traditional scoring uses simple expected value. Complex microstructure
//! uses a **complex-valued** scoring factor that captures both amplitude
//! (opportunity size) and phase (timing alignment).
//!
//! ## The Formula
//!
//! Complex microstructure factor:
//! `Q_C = A_C × e^(iφ_C)`
//!
//! Where:
//! - `A_C` = amplitude ∝ bandwidth × depth × fill_probability
//! - `φ_C` = phase = arctan(latency_skew / expected_latency)
//!
//! Time-normalized score:
//! `S_C = (I_C × Re{Q_C*}) / Δt_C^ms`
//!
//! Phase conjugation `Q_C*` means executing **opposite** to market skew —
//! analogous to wave interference cancellation in signal processing and
//! quantum mechanics. The Phoenix burns against the entropy gradient.
//!
//! ## Example
//!
//! ```rust
//! use negentropy::{ComplexMicrostructure, PhaseConjugation};
//!
//! let ms = ComplexMicrostructure::new(100.0, 0.05, 50.0);
//! let score = ms.time_normalized_score(80.0, 100.0);
//! assert!(score > 0.0);
//! ```

use serde::{Deserialize, Serialize};

/// Complex microstructure factor `Q_C = A_C × e^(iφ_C)`.
///
/// Captures both the amplitude (opportunity size) and phase (timing alignment)
/// of a system's microstructure. The phase conjugate `Q_C*` represents
/// executing opposite to the market skew — the Phoenix burning against entropy.
///
/// 32 bytes (4 × f64), implements `Copy`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ComplexMicrostructure {
    /// Amplitude A_C ∝ bandwidth × depth × fill_probability
    pub amplitude: f64,
    /// Phase φ_C = arctan(latency_skew / expected_latency) in radians
    pub phase: f64,
    /// Real part of Q_C = A × cos(φ)
    pub real: f64,
    /// Imaginary part of Q_C = A × sin(φ)
    pub imag: f64,
}

impl ComplexMicrostructure {
    /// Construct from amplitude and phase.
    ///
    /// `Q_C = A_C × e^(iφ_C) = A_C × (cos(φ) + i·sin(φ))`
    pub fn new(amplitude: f64, phase: f64, _expected_latency_ms: f64) -> Self {
        Self {
            amplitude,
            phase,
            real: amplitude * phase.cos(),
            imag: amplitude * phase.sin(),
        }
    }

    /// Construct from raw components (amplitude, latency_skew, expected_latency).
    ///
    /// `φ = arctan(latency_skew / expected_latency)`
    pub fn from_latency(amplitude: f64, latency_skew_ms: f64, expected_latency_ms: f64) -> Self {
        let phase = if expected_latency_ms > 0.0 {
            (latency_skew_ms / expected_latency_ms).atan()
        } else {
            0.0
        };
        Self::new(amplitude, phase, expected_latency_ms)
    }

    /// Phase conjugate Q_C* = A × e^(-iφ) = A × (cos(φ) - i·sin(φ))
    ///
    /// This represents executing opposite to the market skew — wave
    /// interference cancellation. The Phoenix burns against the gradient.
    pub fn conjugate(&self) -> PhaseConjugation {
        PhaseConjugation {
            amplitude: self.amplitude,
            phase: -self.phase,
            real: self.amplitude * self.phase.cos(),
            imag: -self.amplitude * self.phase.sin(),
        }
    }

    /// Time-normalized score: `S_C = (I_C × Re{Q_C*}) / Δt_C^ms`
    ///
    /// Where `I_C` is the information/intensity and `Δt_C` is the time window.
    /// Uses the phase conjugate (executing opposite to skew).
    pub fn time_normalized_score(&self, intensity: f64, time_window_ms: f64) -> f64 {
        let conj = self.conjugate();
        if time_window_ms > 0.0 {
            (intensity * conj.real) / time_window_ms
        } else {
            0.0
        }
    }

    /// Magnitude |Q_C| = A_C (always equals amplitude by construction)
    pub fn magnitude(&self) -> f64 {
        self.amplitude
    }
}

/// Phase conjugate Q_C* — executing opposite to market skew.
///
/// In wave physics, phase conjugation reverses the phase of a wave, causing
/// it to retrace its path. In the Phoenix cycle, this means executing
/// opposite to the entropy gradient — burning against the chaos to extract
/// maximum negentropy.
///
/// 32 bytes (4 × f64), implements `Copy`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PhaseConjugation {
    /// Amplitude (same as original)
    pub amplitude: f64,
    /// Phase (negated)
    pub phase: f64,
    /// Real part of Q_C*
    pub real: f64,
    /// Imaginary part of Q_C*
    pub imag: f64,
}

impl PhaseConjugation {
    /// The real part of Q_C* — used in the time-normalized score.
    ///
    /// `Re{Q_C*} = A × cos(φ)`
    pub fn real_part(&self) -> f64 {
        self.real
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_microstructure_basic() {
        let ms = ComplexMicrostructure::new(100.0, 0.0, 50.0);
        assert!((ms.real - 100.0).abs() < 0.001);
        assert!(ms.imag.abs() < 0.001);
    }

    #[test]
    fn test_phase_conjugate_negates_phase() {
        let ms = ComplexMicrostructure::new(100.0, 0.5, 50.0);
        let conj = ms.conjugate();
        assert!((conj.phase - (-0.5)).abs() < 0.001);
        assert!((conj.real - ms.real).abs() < 0.001);
        assert!((conj.imag + ms.imag).abs() < 0.001);
    }

    #[test]
    fn test_time_normalized_score() {
        let ms = ComplexMicrostructure::new(100.0, 0.1, 50.0);
        let score = ms.time_normalized_score(80.0, 100.0);
        assert!(score > 0.0);
    }

    #[test]
    fn test_from_latency() {
        let ms = ComplexMicrostructure::from_latency(100.0, 5.0, 50.0);
        // φ = arctan(5/50) = arctan(0.1) ≈ 0.0997
        assert!((ms.phase - 0.0997).abs() < 0.01);
    }

    #[test]
    fn test_zero_phase_maximal_score() {
        // Zero phase (perfectly aligned) should give maximal real component
        let aligned = ComplexMicrostructure::new(100.0, 0.0, 50.0);
        let skewed = ComplexMicrostructure::new(100.0, 1.0, 50.0);
        assert!(aligned.conjugate().real_part() > skewed.conjugate().real_part());
    }
}
