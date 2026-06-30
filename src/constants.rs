//! Shared physical constants used across all Phoenix cycle phases.

/// Boltzmann constant in J/K (exact, CODATA 2019)
pub const K_B: f64 = 1.380649e-23;

/// ln(2) — the natural log of 2
pub const LN_2: f64 = std::f64::consts::LN_2;

/// Electron charge in Coulombs (exact, CODATA 2019) — for eV conversion
pub const ELEMENTARY_CHARGE: f64 = 1.602176634e-19;
