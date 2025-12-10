//! Abstract Syntax Tree for HPM-DL

use serde::{Deserialize, Serialize};

/// HPM-DL Program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub axioms: Vec<Axiom>,
    pub operators: Vec<Operator>,
    pub initial_state: Option<InitialState>,
    pub constraints: Vec<Constraint>,
}

/// Axiom definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axiom {
    pub name: String,
    pub expression: Expression,
    pub metadata: AxiomMetadata,
}

/// Operator definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub name: String,
    pub operator_type: OperatorType,
    pub spec: OperatorSpec,
}

/// Operator types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperatorType {
    Hamiltonian,
    Dissipative,
    TopologicalMutation,
    EntropyFlow,
}

/// Operator specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorSpec {
    pub matrix_representation: Option<MatrixSpec>,
    pub tensor_network: Option<TensorNetworkSpec>,
    pub gpu_kernel: Option<String>,
    pub raw: Option<String>,
}

/// Expression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Variable(String),
    Constant(f64),
    BinaryOp {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Raw(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Dot,
    Cross,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg,
    Transpose,
    Conjugate,
    Trace,
}

/// Initial state specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialState {
    pub psi_0: StateSpec,
    pub h_0: Option<OperatorSpec>,
    pub topology_constraints: Vec<TopologyConstraint>,
}

/// State specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSpec {
    pub representation: StateRepresentation,
    pub dimensions: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateRepresentation {
    MPS,
    PEPS,
    Dense,
    Sparse,
}

/// Constraint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub expression: Expression,
    pub constraint_type: ConstraintType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    EntropyBound,
    StabilityThreshold,
    TopologySurgeryThreshold,
    HumanForceGate,
}

/// Matrix specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixSpec {
    pub rows: usize,
    pub cols: usize,
    pub sparse: bool,
    pub data: Option<Vec<f64>>,
}

/// Tensor network specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorNetworkSpec {
    pub network_type: TensorNetworkType,
    pub bond_dimensions: Vec<usize>,
    pub truncation: Option<TruncationSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensorNetworkType {
    MPS,
    PEPS,
    MERA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruncationSpec {
    pub method: TruncationMethod,
    pub max_bond_dim: usize,
    pub tolerance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TruncationMethod {
    SVD,
    QR,
    RG,
}

/// Topology constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyConstraint {
    pub betti_numbers: Option<Vec<usize>>,
    pub homology_generators: Option<Vec<HomologyGenerator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomologyGenerator {
    pub dimension: usize,
    pub representation: String,
}

/// Axiom metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomMetadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub safety_level: SafetyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    RequiresHumanForce,
    Dangerous,
}

