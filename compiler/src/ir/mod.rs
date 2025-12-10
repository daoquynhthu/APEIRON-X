//! Intermediate Representation (IR) Module
//! 
//! Defines the IR structure for HPM computations.

use serde::{Deserialize, Serialize};

/// HPM IR Program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrProgram {
    pub modules: Vec<IrModule>,
    pub entry_point: String,
}

/// IR Module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrModule {
    pub name: String,
    pub functions: Vec<IrFunction>,
    pub operators: Vec<IrOperator>,
}

/// IR Function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrFunction {
    pub name: String,
    pub parameters: Vec<IrParameter>,
    pub body: IrBlock,
    pub return_type: IrType,
}

/// IR Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrBlock {
    pub statements: Vec<IrStatement>,
}

/// IR Statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrStatement {
    Assign { var: String, value: IrExpression },
    Call { function: String, args: Vec<IrExpression> },
    Return { value: Option<IrExpression> },
    If {
        condition: IrExpression,
        then_block: IrBlock,
        else_block: Option<IrBlock>,
    },
    Loop { body: IrBlock },
}

/// IR Expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrExpression {
    Variable(String),
    Constant(IrConstant),
    BinaryOp {
        op: IrBinaryOp,
        left: Box<IrExpression>,
        right: Box<IrExpression>,
    },
    UnaryOp {
        op: IrUnaryOp,
        expr: Box<IrExpression>,
    },
    Call {
        function: String,
        args: Vec<IrExpression>,
    },
    TensorOp {
        op: TensorOperation,
        operands: Vec<IrExpression>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrConstant {
    Int(i64),
    Float(f64),
    Complex { re: f64, im: f64 },
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Dot,
    Cross,
    TensorProduct,
    Contract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrUnaryOp {
    Neg,
    Transpose,
    Conjugate,
    Trace,
    SVD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TensorOperation {
    MPSContract,
    PEPSContract,
    SVDTruncate,
    QRDecompose,
    Renormalize,
}

/// IR Operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrOperator {
    pub name: String,
    pub operator_type: IrOperatorType,
    pub spec: IrOperatorSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrOperatorType {
    Hamiltonian,
    Dissipative,
    TopologicalMutation,
    EntropyFlow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrOperatorSpec {
    pub target: IrTarget,
    pub implementation: IrOperatorImplementation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrTarget {
    CPU,
    GPU,
    MultiGPU,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrOperatorImplementation {
    DenseMatrix { data: Vec<f64> },
    SparseMatrix { indices: Vec<(usize, usize)>, values: Vec<f64> },
    TensorNetwork { network: TensorNetworkSpec },
    CUDAKernel { kernel_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorNetworkSpec {
    pub network_type: String,
    pub tensors: Vec<TensorSpec>,
    pub bonds: Vec<BondSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorSpec {
    pub name: String,
    pub shape: Vec<usize>,
    pub data: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondSpec {
    pub tensor1: String,
    pub index1: usize,
    pub tensor2: String,
    pub index2: usize,
    pub dimension: usize,
}

/// IR Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrType {
    Int,
    Float,
    Complex,
    Bool,
    Tensor { shape: Vec<usize> },
    Operator { spec: IrOperatorSpec },
    State,
}

/// IR Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrParameter {
    pub name: String,
    pub param_type: IrType,
}

