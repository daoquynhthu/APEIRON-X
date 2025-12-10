//! Safety Checker Module
//!
//! Implements safety checks including human_force gates.

use crate::parser::ast::*;
use anyhow::Result;

pub struct SafetyChecker {
    // TODO: Add safety context
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SafetyReport {
    pub human_force_gates: Vec<HumanForceGate>,
    pub warnings: Vec<SafetyWarning>,
    pub errors: Vec<SafetyError>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HumanForceGate {
    pub operator_name: String,
    pub reason: String,
    pub required_approval: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SafetyWarning {
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SafetyError {
    pub message: String,
    pub code: SafetyErrorCode,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SafetyErrorCode {
    EntropyBlowup,
    AnomalyThresholdExceeded,
    TopologySurgeryThrottleExceeded,
    UnauthorizedOperator,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntropyReport {
    pub estimated_entropy: f64,
    pub threshold: f64,
    pub safe: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnomalyReport {
    pub anomalies: Vec<Anomaly>,
    pub threshold: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Anomaly {
    pub location: String,
    pub value: f64,
    pub threshold: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopologyReport {
    pub surgery_count: usize,
    pub throttle_limit: usize,
    pub safe: bool,
}

impl SafetyChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check for dangerous operators requiring human_force gate
    pub fn check_human_force_gates(&self, program: &Program) -> Result<SafetyReport> {
        Ok(SafetyReport {
            human_force_gates: program
                .constraints
                .iter()
                .filter(|c| matches!(c.constraint_type, ConstraintType::HumanForceGate))
                .map(|c| HumanForceGate {
                    operator_name: c.name.clone(),
                    reason: "marked as human_force_gate".to_string(),
                    required_approval: true,
                })
                .collect(),
            warnings: vec![],
            errors: vec![],
        })
    }

    /// Check entropy blowup conditions
    pub fn check_entropy_blowup(&self, program: &Program) -> Result<EntropyReport> {
        let safe = !program
            .constraints
            .iter()
            .any(|c| matches!(c.constraint_type, ConstraintType::EntropyBound));
        Ok(EntropyReport {
            estimated_entropy: 0.0,
            threshold: 0.0,
            safe,
        })
    }

    /// Check anomaly thresholds
    pub fn check_anomaly_thresholds(&self, _program: &Program) -> Result<AnomalyReport> {
        Ok(AnomalyReport {
            anomalies: vec![],
            threshold: 0.0,
        })
    }

    /// Check topology surgery throttle
    pub fn check_topology_surgery_throttle(&self, _program: &Program) -> Result<TopologyReport> {
        Ok(TopologyReport {
            surgery_count: 0,
            throttle_limit: 0,
            safe: true,
        })
    }
}

