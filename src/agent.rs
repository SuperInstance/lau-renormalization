//! Application: agent behavior at different scales, universality in agent populations.
//!
//! The renormalization group framework can be applied to multi-agent systems:
//! - Agent behavior at different time/organization scales follows RG-like flows
//! - Populations of agents can exhibit universality classes of collective behavior
//! - Scaling relations describe how aggregate behavior emerges from individual interactions

use serde::{Deserialize, Serialize};

use crate::beta::{FlowConfig, flow};
use crate::critical_exponents::CriticalExponents;
use crate::universality::UniversalityClass;
use crate::scaling::finite_size_scaling;

/// An agent population characterized by a coupling parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPopulation {
    /// Name/identifier.
    pub name: String,
    /// Coupling strength between agents (e.g., imitation tendency).
    pub coupling: f64,
    /// Population size.
    pub size: usize,
    /// Interaction topology type.
    pub topology: InteractionTopology,
}

/// How agents interact.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InteractionTopology {
    /// All-to-all (mean-field).
    FullyConnected,
    /// Regular lattice in d dimensions.
    Lattice { dimension: usize },
    /// Random network with average degree k.
    Random { avg_degree: usize },
    /// Scale-free network with exponent γ.
    ScaleFree { gamma: f64 },
}

/// Agent-level RG: how agent behavior changes when coarse-grained to group level.
///
/// The "coupling" represents how strongly agents influence each other.
/// Under RG (coarse-graining groups of agents), the effective coupling flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRGFlow {
    /// Initial agent coupling.
    pub initial_coupling: f64,
    /// Coupling at each coarse-graining step.
    pub flow: Vec<f64>,
    /// Scale factor at each step.
    pub scales: Vec<f64>,
}

/// Compute the RG flow for an agent population using an Ising-like model.
///
/// Agents are modeled as binary agents (cooperate/defect) with coupling g.
/// The beta function β(g) describes how the effective coupling changes
/// when we coarse-grain to larger organizational scales.
pub fn agent_rg_flow(
    initial_coupling: f64,
    beta_fn: &crate::beta::BetaFn,
    n_steps: usize,
    scale_factor: f64,
) -> AgentRGFlow {
    let config = FlowConfig {
        steps: n_steps,
        d_ln_mu: scale_factor.ln(),
        method: crate::beta::IntegrationMethod::Euler,
    };

    let traj = flow(beta_fn, initial_coupling, &config);

    AgentRGFlow {
        initial_coupling,
        flow: traj.trajectory,
        scales: traj.ln_mu_values,
    }
}

/// Agent fixed point: a behavioral regime that's stable under coarse-graining.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFixedPoint {
    /// Coupling at the fixed point.
    pub coupling: f64,
    /// Behavioral regime.
    pub regime: BehavioralRegime,
    /// Description.
    pub description: String,
}

/// Behavioral regimes that can appear as RG fixed points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BehavioralRegime {
    /// Disordered: agents act independently, no coordination.
    Disordered,
    /// Ordered: agents strongly coordinated, herd behavior.
    Ordered,
    /// Critical: scale-free correlations, maximum sensitivity.
    Critical,
}

/// Classify agent behavior at a given coupling strength.
pub fn classify_agent_behavior(coupling: f64, critical_coupling: f64) -> BehavioralRegime {
    let tol = 0.05 * critical_coupling;
    if (coupling - critical_coupling).abs() < tol {
        BehavioralRegime::Critical
    } else if coupling < critical_coupling {
        BehavioralRegime::Disordered
    } else {
        BehavioralRegime::Ordered
    }
}

/// Compute the effective agent coupling at a given organizational scale.
///
/// This models how decision-making changes when you go from individual agents
/// to teams, departments, organizations, etc.
pub fn effective_coupling_at_scale(
    base_coupling: f64,
    scale: f64,
    beta_fn: &crate::beta::BetaFn,
) -> f64 {
    let config = FlowConfig {
        steps: 100,
        d_ln_mu: scale.ln() / 100.0,
        method: crate::beta::IntegrationMethod::RK4,
    };
    let traj = flow(beta_fn, base_coupling, &config);
    *traj.trajectory.last().unwrap()
}

/// Universality in agent populations: different agent architectures can exhibit
/// the same collective behavior at large scales.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentUniversalityClass {
    /// Name for this class of agent behavior.
    pub name: String,
    /// The underlying physical universality class.
    pub physical_class: UniversalityClass,
    /// Example agent architectures that fall into this class.
    pub example_architectures: Vec<String>,
    /// Key behavioral signatures.
    pub signatures: Vec<String>,
}

impl AgentUniversalityClass {
    /// Cooperative agents: Ising-like Z₂ symmetry (cooperate/defect).
    pub fn cooperative_ising() -> Self {
        AgentUniversalityClass {
            name: "Cooperative Ising".to_string(),
            physical_class: UniversalityClass::ising_3d(),
            example_architectures: vec![
                "Binary choice agents".to_string(),
                "Threshold models".to_string(),
            ],
            signatures: vec![
                "Phase transition from disorder to cooperation".to_string(),
                "Power-law correlations at critical coupling".to_string(),
            ],
        }
    }

    /// Multi-strategy agents: Potts-like with q strategies.
    pub fn multi_strategy_potts(q: usize) -> Self {
        AgentUniversalityClass {
            name: format!("{}-Strategy Potts", q),
            physical_class: UniversalityClass {
                name: format!("{}-state Potts", q),
                dimension: 3,
                symmetry: crate::universality::SymmetryGroup::Potts(q),
                interaction_range: crate::universality::InteractionRange::Short,
                exponents: CriticalExponents::mean_field(), // placeholder
                description: format!("{}-state Potts model universality", q),
            },
            example_architectures: vec![format!("{}-strategy game agents", q)],
            signatures: vec![format!("First-order transition for q > {}", if q > 3 { "some" } else { "all" })],
        }
    }
}

/// Scale-dependent agent performance: how metrics change with organizational level.
pub fn agent_performance_scaling(
    base_performance: f64,
    organization_level: f64,
    scaling_exponent: f64,
    nu: f64,
) -> f64 {
    finite_size_scaling(scaling_exponent, nu, organization_level, base_performance)
}

/// Detect phase transitions in agent population data.
///
/// Given time-series data of an order parameter (e.g., average cooperation rate),
/// detect if the system undergoes a phase transition.
pub fn detect_phase_transition(order_params: &[f64], couplings: &[f64]) -> Option<f64> {
    if order_params.len() < 3 || couplings.len() != order_params.len() {
        return None;
    }

    // Look for the coupling where the order parameter changes most rapidly
    let mut max_derivative = 0.0_f64;
    let mut critical_coupling = 0.0_f64;

    for i in 1..order_params.len() - 1 {
        let d2m_dc2 = (order_params[i + 1] - 2.0 * order_params[i] + order_params[i - 1])
            / (couplings[1] - couplings[0]).powi(2);
        if d2m_dc2.abs() > max_derivative {
            max_derivative = d2m_dc2.abs();
            critical_coupling = couplings[i];
        }
    }

    if max_derivative > 0.1 {
        Some(critical_coupling)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_rg_flow_creation() {
        let beta: crate::beta::BetaFn = Box::new(|g: f64| -g + g * g);
        let flow = agent_rg_flow(0.5, &beta, 100, 1.1);
        assert_eq!(flow.flow.len(), 101);
        assert!((flow.initial_coupling - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_classify_agent_behavior_disordered() {
        let regime = classify_agent_behavior(0.1, 1.0);
        assert_eq!(regime, BehavioralRegime::Disordered);
    }

    #[test]
    fn test_classify_agent_behavior_ordered() {
        let regime = classify_agent_behavior(2.0, 1.0);
        assert_eq!(regime, BehavioralRegime::Ordered);
    }

    #[test]
    fn test_classify_agent_behavior_critical() {
        let regime = classify_agent_behavior(1.0, 1.0);
        assert_eq!(regime, BehavioralRegime::Critical);
    }

    #[test]
    fn test_effective_coupling_at_scale() {
        let beta: crate::beta::BetaFn = Box::new(|g: f64| -g);
        let g_eff = effective_coupling_at_scale(1.0, 2.0, &beta);
        assert!(g_eff < 1.0); // decaying coupling
    }

    #[test]
    fn test_agent_universality_cooperative() {
        let cls = AgentUniversalityClass::cooperative_ising();
        assert_eq!(cls.name, "Cooperative Ising");
        assert!(!cls.example_architectures.is_empty());
    }

    #[test]
    fn test_agent_performance_scaling() {
        let perf = agent_performance_scaling(1.0, 10.0, 0.5, 0.63);
        assert!(perf.is_finite());
        assert!(perf > 0.0);
    }

    #[test]
    fn test_detect_phase_transition() {
        // Create a sharp transition
        let couplings: Vec<f64> = (0..100).map(|i| i as f64 * 0.02).collect();
        let order_params: Vec<f64> = couplings
            .iter()
            .map(|&c| if c > 1.0 { 1.0 } else { 0.0 })
            .collect();
        let kc = detect_phase_transition(&order_params, &couplings);
        assert!(kc.is_some());
        assert!((kc.unwrap() - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_detect_no_transition() {
        let couplings: Vec<f64> = (0..50).map(|i| i as f64 * 0.02).collect();
        let order_params: Vec<f64> = couplings.iter().map(|_| 0.5).collect();
        let kc = detect_phase_transition(&order_params, &couplings);
        assert!(kc.is_none());
    }

    #[test]
    fn test_agent_population_creation() {
        let pop = AgentPopulation {
            name: "test".to_string(),
            coupling: 0.5,
            size: 100,
            topology: InteractionTopology::FullyConnected,
        };
        assert_eq!(pop.size, 100);
    }
}
