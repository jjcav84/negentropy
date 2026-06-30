//! negentropy CLI — score any system's thermodynamic energy
//!
//! Usage:
//!   negentropy score --constraints 17 --threshold 18
//!   negentropy route --confidence 95 --depth 75 --latency 800 --cost 0.001
//!   negentropy committor --hits 7 --total 10
//!   negentropy entropy --probs 0.5,0.5
//!   negentropy burn --bits 70.9 --temperature 300

use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "negentropy",
    about = "The physics of information extraction — score any system's thermodynamic energy",
    long_about = "Open-source thermodynamic engine adapted from the orkid FMD physics engine.\n\nThe Phoenix cycle: entropy → burn → extraction → rebirth.\n\nVisit https://github.com/jjcav84/negentropy for the full theory."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Score negentropy extracted by a constraint-based system (e.g., ZK proof)
    Score {
        #[arg(long, help = "Number of constraints in the system")]
        constraints: u64,
        #[arg(long, help = "Threshold or difficulty parameter")]
        threshold: u64,
    },
    /// Score route energy for a multi-step process
    Route {
        #[arg(long, help = "Confidence score (0-100)")]
        confidence: f64,
        #[arg(long, help = "Depth ratio")]
        depth: f64,
        #[arg(long, help = "Timing factor (0-1)", default_value = "1.0")]
        timing: f64,
        #[arg(long, help = "Total latency in milliseconds")]
        latency: u64,
        #[arg(long, help = "Cost in USD")]
        cost: f64,
    },
    /// Compute committor probability from ensemble sampling
    Committor {
        #[arg(long, help = "Number of trajectories that hit target")]
        hits: usize,
        #[arg(long, help = "Total trajectories sampled")]
        total: usize,
    },
    /// Compute Shannon entropy of a probability distribution
    Entropy {
        #[arg(long, help = "Comma-separated probabilities (must sum to 1)")]
        probs: String,
    },
    /// Compute Landauer burn cost for erasing N bits
    Burn {
        #[arg(long, help = "Number of bits erased")]
        bits: f64,
        #[arg(long, help = "Temperature in Kelvin", default_value = "300")]
        temperature: f64,
    },
    /// Show the Phoenix cycle and formulas
    Theory,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Score { constraints, threshold } => {
            let neg = negentropy::Negentropy::from_constraints(constraints, threshold);
            let result = json!({
                "phase": "extraction",
                "negentropy_bits": neg.bits(),
                "negentropy_nats": neg.nats(),
                "source": neg.source,
                "formula": "N = constraint_count × log₂(threshold)",
            });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Route { confidence, depth, timing, latency, cost } => {
            let pot = negentropy::RoutePotential {
                confidence,
                depth_ratio: depth,
                timing_factor: timing,
                latency_ms: latency,
                cost_usd: cost,
                ..Default::default()
            };
            let result = pot.energy();
            let output = json!({
                "phase": "extraction",
                "energy": result.energy,
                "confidence": result.confidence,
                "depth_ratio": result.depth_ratio,
                "timing_factor": result.timing_factor,
                "latency_decay": result.latency_decay,
                "cost_penalty": result.cost_penalty,
                "formula": "energy = confidence × √(depth × timing) × latency_decay × (1 − cost)",
                "origin": "orkid fmd-physics/src/route_energy.rs",
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        Commands::Committor { hits, total } => {
            let comm = negentropy::Committor::from_ensemble(hits, total);
            let result = json!({
                "phase": "rebirth",
                "probability": comm.probability,
                "hits": comm.hits,
                "total": comm.total,
                "confidence": comm.confidence,
                "formula": "q(x) = hits / total",
                "origin": "orkid fmd-physics/src/tps.rs",
            });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Entropy { probs } => {
            let p: Vec<f64> = probs
                .split(',')
                .map(|s| s.trim().parse::<f64>().expect("invalid probability"))
                .collect();
            let h = negentropy::Entropy::from_probabilities(&p);
            let result = json!({
                "phase": "entropy",
                "entropy_bits": h.bits(),
                "entropy_nats": h.nats(),
                "n_states": h.n_states,
                "formula": "H = -Σ p_i × log₂(p_i)",
            });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Burn { bits, temperature } => {
            let burn = negentropy::Burn::landauer(bits, temperature);
            let real = negentropy::Burn::cpu_cost(50.0, 0.8); // 50W for 0.8s
            let result = json!({
                "phase": "burn",
                "bits_erased": burn.bits_erased,
                "temperature_k": burn.temperature_k,
                "landauer_limit_j": burn.energy_joules,
                "landauer_limit_ev": burn.ev(),
                "real_cpu_estimate_j": real,
                "efficiency_ratio": burn.efficiency_ratio(real),
                "formula": "E ≥ k_B × T × ln(2) × N_bits",
                "note": "Real CPUs operate ~10⁹× above the Landauer limit",
            });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Theory => {
            println!("╔══════════════════════════════════════════════════════════╗");
            println!("║           THE PHOENIX CYCLE OF INFORMATION              ║");
            println!("╠══════════════════════════════════════════════════════════╣");
            println!("║                                                          ║");
            println!("║  ┌────────┐   ┌────────┐   ┌────────────┐   ┌────────┐ ║");
            println!("║  │ENTROPY │──▶│  BURN  │──▶│ EXTRACTION │──▶│REBIRTH │ ║");
            println!("║  │ (chaos)│   │(energy)│   │(negentropy)│   │(order) │ ║");
            println!("║  └────────┘   └────────┘   └────────────┘   └────────┘ ║");
            println!("║                                                          ║");
            println!("║  1. ENTROPY     H = -Σ pᵢ log₂(pᵢ)    Shannon 1948    ║");
            println!("║  2. BURN        E ≥ k_B T ln(2) N      Landauer 1961   ║");
            println!("║  3. EXTRACTION  N = D_KL(p||q)          Brillouin 1953 ║");
            println!("║  4. REBIRTH     q(x) = hits/total       TPS (Bolhuis)  ║");
            println!("║                                                          ║");
            println!("║  Route Energy:                                           ║");
            println!("║  E = conf × √(depth × timing) × latency × (1 - cost)   ║");
            println!("║                                                          ║");
            println!("║  Origin: orkid FMD physics engine                       ║");
            println!("║  https://github.com/jjcav84/orkid                        ║");
            println!("║                                                          ║");
            println!("║  Applications:                                           ║");
            println!("║  • zk-age    — privacy-preserving age verification       ║");
            println!("║  • zk-attest — ZK attestations on Hedera                ║");
            println!("║  • zk-ballot — anonymous on-chain voting                 ║");
            println!("║  • orkid     — MEV detection (origin)                   ║");
            println!("║                                                          ║");
            println!("╚══════════════════════════════════════════════════════════╝");
        }
    }
}
