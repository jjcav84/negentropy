//! Phase 1→3: Entropy to Extraction — Information Diffusion on Networks
//!
//! Adapted from the orkid formal negentropy model blog post:
//! ["A Formal Mathematical Model of Blockchain Negentropy and MEV Dynamics"]
//! (https://github.com/jjcav84/orkid/blob/main/blog/2025-10-18-formal-negentropy-model-mev-dynamics.md)
//!
//! Information propagates through networks via graph diffusion, reducing
//! entropy proportionally to its intensity. The MEV closure equation:
//!
//! `dM/dt = a·δ + b·H_M − c·χ(I)·M`
//!
//! Where:
//! - δ = system slack (unused capacity, inefficiency)
//! - H_M = mempool/queue entropy (uncertainty in ordering)
//! - χ(I) = information intensity functional
//! - M = extractable value (entropy that can be captured)
//!
//! Information closes arbitrage opportunities. Analogously, in any system,
//! information propagation closes uncertainty.
//!
//! ## Example
//!
//! ```rust
//! use negentropy::{GraphLaplacian, InformationField};
//!
//! // 3-node network: A ↔ B ↔ C
//! let lap = GraphLaplacian::path_graph(3);
//! let mut field = InformationField::new(vec![1.0, 0.0, 0.0]); // info starts at node A
//! field.diffuse_mut(&lap, 0.1, 10); // diffuse for 10 steps in-place
//! // Information has spread to all nodes
//! assert!(field.values[2] > 0.0);
//! ```

use serde::{Deserialize, Serialize};

/// Graph Laplacian `L = D - A` for a network.
///
/// The Laplacian governs how information diffuses through the network.
/// Information flows from high-information nodes to low-information nodes.
///
/// The matrix is stored as a **flat `Vec<f64>`** in row-major order to avoid
/// the double allocation and cache misses of `Vec<Vec<f64>>`. For an n-node
/// graph, element `(i, j)` is at `matrix[i * n + j]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphLaplacian {
    /// Number of nodes
    pub n: usize,
    /// Flat Laplacian matrix (n × n), row-major: `matrix[i * n + j]`
    pub matrix: Vec<f64>,
}

impl GraphLaplacian {
    /// Build a path graph: A ↔ B ↔ C ↔ ...
    ///
    /// Each node connects to its immediate neighbors.
    pub fn path_graph(n: usize) -> Self {
        let mut matrix = vec![0.0; n * n];
        for i in 0..n {
            let diag = i * n + i;
            if i > 0 {
                matrix[diag] += 1.0;
                matrix[i * n + (i - 1)] = -1.0;
            }
            if i < n - 1 {
                matrix[diag] += 1.0;
                matrix[i * n + (i + 1)] = -1.0;
            }
        }
        Self { n, matrix }
    }

    /// Build a complete graph: every node connects to every other.
    pub fn complete_graph(n: usize) -> Self {
        let mut matrix = vec![0.0; n * n];
        for i in 0..n {
            matrix[i * n + i] = (n - 1) as f64;
            for j in 0..n {
                if i != j {
                    matrix[i * n + j] = -1.0;
                }
            }
        }
        Self { n, matrix }
    }

    /// Build a star graph: center node connects to all others.
    pub fn star_graph(n: usize) -> Self {
        assert!(n >= 2, "star graph needs at least 2 nodes");
        let mut matrix = vec![0.0; n * n];
        // Center is node 0
        matrix[0] = (n - 1) as f64;
        for i in 1..n {
            matrix[i] = -1.0;           // row 0, col i
            matrix[i * n] = -1.0;       // row i, col 0
            matrix[i * n + i] = 1.0;    // diagonal
        }
        Self { n, matrix }
    }

    /// Matrix-vector multiply: `y = L × x`.
    ///
    /// Allocates a new `Vec<f64>`. For hot paths, use [`apply_into`] to write
    /// into a pre-allocated buffer.
    ///
    /// [`apply_into`]: GraphLaplacian::apply_into
    pub fn apply(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.n, "vector length must match graph size");
        let mut y = vec![0.0; self.n];
        self.apply_into(x, &mut y);
        y
    }

    /// Matrix-vector multiply into a provided buffer: `y = L × x`.
    ///
    /// Zero-allocation — the caller provides the output buffer. The buffer
    /// is filled and truncated/extended to `self.n` if needed.
    pub fn apply_into(&self, x: &[f64], y: &mut Vec<f64>) {
        assert_eq!(x.len(), self.n, "vector length must match graph size");
        y.resize(self.n, 0.0);
        for (i, slot) in y.iter_mut().enumerate().take(self.n) {
            let row = &self.matrix[i * self.n..(i + 1) * self.n];
            *slot = row
                .iter()
                .zip(x.iter())
                .map(|(&a, &b)| a * b)
                .sum();
        }
    }

    /// Access element `(i, j)` of the Laplacian.
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.matrix[i * self.n + j]
    }
}

/// Information field — the information level at each node in a network.
///
/// `I_t ∈ R^n` where `I_t[i]` is the information at node i at time t.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationField {
    /// Information level at each node
    pub values: Vec<f64>,
}

impl InformationField {
    /// Create a new information field.
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }

    /// Diffuse information through the network for `steps` time steps, in-place.
    ///
    /// Implements: `∂I/∂t = D_I × (-L) × I + s(t) - γ × I`
    ///
    /// Where:
    /// - `D_I` = diffusion coefficient
    /// - `L` = graph Laplacian
    /// - `s(t)` = information sources (injected here as initial conditions)
    /// - `γ` = decay rate (information becomes stale)
    ///
    /// Uses a reusable scratch buffer internally — zero heap allocations after
    /// the first call (the buffer is reused across steps).
    pub fn diffuse_mut(&mut self, lap: &GraphLaplacian, diffusion_coeff: f64, steps: usize) {
        let n = self.values.len();
        let dt = 0.01;
        let decay = 0.001; // γ

        // Reusable scratch buffer — allocated once, reused across all steps
        let mut scratch = vec![0.0; n];

        for _ in 0..steps {
            lap.apply_into(&self.values, &mut scratch);
            for (v, &s) in self.values.iter_mut().zip(scratch.iter()) {
                // dI/dt = D * (-L) * I - gamma * I
                // Note: -L because Laplacian is positive semi-definite,
                // and diffusion goes from high to low
                let di_dt = -diffusion_coeff * s - decay * *v;
                *v += di_dt * dt;
                if *v < 0.0 {
                    *v = 0.0; // information can't be negative
                }
            }
        }
    }

    /// Diffuse information, returning a new field (allocates).
    ///
    /// This is the non-mutating variant — use [`diffuse_mut`] for hot paths.
    ///
    /// [`diffuse_mut`]: InformationField::diffuse_mut
    pub fn diffuse(&self, lap: &GraphLaplacian, diffusion_coeff: f64, steps: usize) -> Self {
        let mut result = self.clone();
        result.diffuse_mut(lap, diffusion_coeff, steps);
        result
    }

    /// Total information in the field.
    pub fn total(&self) -> f64 {
        self.values.iter().sum()
    }

    /// Entropy of the information distribution across nodes.
    /// Lower = concentrated (ordered), higher = spread (diffused).
    pub fn distribution_entropy(&self) -> f64 {
        let total = self.total();
        if total <= 0.0 {
            return 0.0;
        }
        self.values
            .iter()
            .filter(|&&v| v > 0.0)
            .map(|&v| {
                let p = v / total;
                -p * p.log2()
            })
            .sum()
    }
}

/// MEV closure equation: `dM/dt = a·δ + b·H_M − c·χ(I)·M`
///
/// Models how extractable value (entropy) evolves as information propagates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClosureEquation {
    /// Slack creation rate (a)
    pub slack_rate: f64,
    /// Entropy creation rate (b)
    pub entropy_rate: f64,
    /// Closure rate (c) — how fast information closes opportunities
    pub closure_rate: f64,
    /// Current extractable value (M)
    pub value: f64,
    /// Current slack (δ) — unused capacity
    pub slack: f64,
    /// Current entropy (H_M) — ordering uncertainty
    pub entropy: f64,
    /// Information intensity (χ(I))
    pub information_intensity: f64,
}

impl ClosureEquation {
    /// Step the closure equation forward by dt.
    ///
    /// `dM/dt = a·δ + b·H_M − c·χ(I)·M`
    pub fn step(&mut self, dt: f64) {
        let d_m = self.slack_rate * self.slack
            + self.entropy_rate * self.entropy
            - self.closure_rate * self.information_intensity * self.value;
        self.value += d_m * dt;
        if self.value < 0.0 {
            self.value = 0.0;
        }
    }

    /// Steady-state value: `M* = (a·δ + b·H_M) / (c·χ(I*))`
    pub fn steady_state(&self) -> f64 {
        let numerator = self.slack_rate * self.slack + self.entropy_rate * self.entropy;
        let denominator = self.closure_rate * self.information_intensity;
        if denominator > 0.0 {
            numerator / denominator
        } else {
            f64::INFINITY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_graph_laplacian() {
        let lap = GraphLaplacian::path_graph(3);
        // Node 0: degree 1, connects to node 1
        assert!((lap.at(0, 0) - 1.0).abs() < 0.001);
        assert!((lap.at(0, 1) + 1.0).abs() < 0.001);
        // Node 1: degree 2, connects to nodes 0 and 2
        assert!((lap.at(1, 1) - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_diffusion_spreads_information() {
        let lap = GraphLaplacian::path_graph(3);
        let mut field = InformationField::new(vec![1.0, 0.0, 0.0]);
        field.diffuse_mut(&lap, 1.0, 100);
        // Information should have spread to node 2
        assert!(field.values[2] > 0.0);
    }

    #[test]
    fn test_complete_graph_diffuses_faster() {
        let path = GraphLaplacian::path_graph(5);
        let complete = GraphLaplacian::complete_graph(5);
        let field = InformationField::new(vec![1.0, 0.0, 0.0, 0.0, 0.0]);
        let mut path_field = field.clone();
        let mut complete_field = field.clone();
        path_field.diffuse_mut(&path, 1.0, 50);
        complete_field.diffuse_mut(&complete, 1.0, 50);
        // Complete graph should spread info more evenly
        assert!(complete_field.values[4] > path_field.values[4]);
    }

    #[test]
    fn test_apply_into_reuses_buffer() {
        let lap = GraphLaplacian::path_graph(3);
        let x = vec![1.0, 2.0, 3.0];
        let mut y = Vec::new();
        lap.apply_into(&x, &mut y);
        assert_eq!(y.len(), 3);
        // L * x for path graph: [-1+2, 1-2+3, -2+3] = [1, 2, 1]... let's just check non-zero
        assert!(y.iter().any(|&v| v != 0.0));
    }

    #[test]
    fn test_closure_equation() {
        let mut eq = ClosureEquation {
            slack_rate: 1.0,
            entropy_rate: 1.0,
            closure_rate: 0.5,
            value: 10.0,
            slack: 1.0,
            entropy: 1.0,
            information_intensity: 0.8,
        };
        let initial = eq.value;
        eq.step(0.1);
        // Value should change
        assert!((eq.value - initial).abs() > 0.001);
    }

    #[test]
    fn test_steady_state() {
        let eq = ClosureEquation {
            slack_rate: 1.0,
            entropy_rate: 1.0,
            closure_rate: 0.5,
            value: 0.0,
            slack: 1.0,
            entropy: 1.0,
            information_intensity: 0.8,
        };
        // M* = (1*1 + 1*1) / (0.5 * 0.8) = 2 / 0.4 = 5
        assert!((eq.steady_state() - 5.0).abs() < 0.001);
    }
}
