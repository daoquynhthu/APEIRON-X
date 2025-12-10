//! HPM-DL Parser Module
//! 
//! This module implements the parser for HPM-DL (Hyper-Physical Manifold Description Language).
//! Uses nom or pest for parsing.

use anyhow::{bail, Context, Result};
use pest::iterators::Pair;
use pest::Parser;

pub mod lexer;
pub mod grammar;
pub mod ast;

/// Parse HPM-DL source code into AST
pub fn parse(source: &str) -> Result<ast::Program> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        bail!("syntax error: empty input");
    }

    // 语法检查：使用 pest 覆盖完整语法
    let pairs = grammar::HpmDlParser::parse(grammar::Rule::file, source)
        .map_err(|e| anyhow::anyhow!("syntax error: {}", e))
        .context("failed to parse HPM-DL source")?;

    let mut program = ast::Program {
        axioms: vec![],
        operators: vec![],
        initial_state: None,
        constraints: vec![],
    };

    for pair in pairs {
        if pair.as_rule() == grammar::Rule::stmt {
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    grammar::Rule::axiom => program.axioms.push(parse_axiom(inner)?),
                    grammar::Rule::operator => program.operators.push(parse_operator(inner)?),
                    grammar::Rule::state_decl => program.initial_state = Some(parse_state(inner)?),
                    grammar::Rule::constraint => program.constraints.push(parse_constraint(inner)?),
                    _ => {}
                }
            }
        }
    }

    Ok(program)
}

fn parse_axiom(pair: Pair<grammar::Rule>) -> Result<ast::Axiom> {
    let mut name = String::new();
    let mut expr = ast::Expression::Raw(String::new());
    let mut metadata = ast::AxiomMetadata {
        description: None,
        tags: vec![],
        safety_level: ast::SafetyLevel::Safe,
    };

    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::ident => name = p.as_str().to_string(),
            grammar::Rule::expr => expr = parse_expr(p)?,
            grammar::Rule::tags => metadata.tags = parse_tags(p),
            grammar::Rule::safety_level => {
                metadata.safety_level = match p.as_str() {
                    "requires_human_force" => ast::SafetyLevel::RequiresHumanForce,
                    "dangerous" => ast::SafetyLevel::Dangerous,
                    _ => ast::SafetyLevel::Safe,
                }
            }
            _ => {}
        }
    }

    Ok(ast::Axiom {
        name,
        expression: expr,
        metadata,
    })
}

fn parse_operator(pair: Pair<grammar::Rule>) -> Result<ast::Operator> {
    let mut name = String::new();
    let mut kind = ast::OperatorType::Hamiltonian;
    let raw = pair.as_str().to_string();
    let mut matrix: Option<ast::MatrixSpec> = None;
    let mut tensor_net: Option<ast::TensorNetworkSpec> = None;
    let mut gpu_kernel: Option<String> = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::ident => name = p.as_str().to_string(),
            grammar::Rule::op_kind => {
                kind = match p.as_str() {
                    "hamiltonian" => ast::OperatorType::Hamiltonian,
                    "dissipative" => ast::OperatorType::Dissipative,
                    "topological_mutation" => ast::OperatorType::TopologicalMutation,
                    "entropy_flow" => ast::OperatorType::EntropyFlow,
                    _ => ast::OperatorType::Hamiltonian,
                }
            }
            grammar::Rule::matrix_spec => {
                matrix = Some(parse_matrix_spec(p));
            }
            grammar::Rule::tensor_net_spec => {
                tensor_net = Some(parse_tensor_net_spec(p));
            }
            grammar::Rule::gpu_kernel_spec => {
                gpu_kernel = Some(parse_string_in_paren(p));
            }
            _ => {}
        }
    }

    Ok(ast::Operator {
        name,
        operator_type: kind,
        spec: ast::OperatorSpec {
            matrix_representation: matrix,
            tensor_network: tensor_net,
            gpu_kernel,
            raw: Some(raw),
        },
    })
}

fn parse_state(pair: Pair<grammar::Rule>) -> Result<ast::InitialState> {
    let mut _name = String::new();
    let mut repr = ast::StateRepresentation::Dense;
    let mut dims: Vec<usize> = vec![];
    let mut h0 = None;
    let mut betti = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::ident => _name = p.as_str().to_string(),
            grammar::Rule::state_repr => {
                repr = match p.as_str() {
                    "mps" => ast::StateRepresentation::MPS,
                    "peps" => ast::StateRepresentation::PEPS,
                    "sparse" => ast::StateRepresentation::Sparse,
                    _ => ast::StateRepresentation::Dense,
                }
            }
            grammar::Rule::dims => {
                dims = parse_int_list(p);
            }
            grammar::Rule::hamiltonian_ref => {
                h0 = Some(ast::OperatorSpec {
                    matrix_representation: None,
                    tensor_network: None,
                    gpu_kernel: Some(p.as_str().to_string()),
                    raw: Some(p.as_str().to_string()),
                });
            }
            grammar::Rule::topo_constraints => {
                betti = parse_betti(p);
            }
            _ => {}
        }
    }

    Ok(ast::InitialState {
        psi_0: ast::StateSpec {
            representation: repr,
            dimensions: dims,
        },
        h_0: h0,
        topology_constraints: betti
            .map(|b| vec![ast::TopologyConstraint {
                betti_numbers: Some(b),
                homology_generators: None,
            }])
            .unwrap_or_default(),
    })
}

fn parse_constraint(pair: Pair<grammar::Rule>) -> Result<ast::Constraint> {
    let mut name = String::new();
    let mut ctype = ast::ConstraintType::EntropyBound;
    let mut expr = ast::Expression::Raw(String::new());

    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::ident => name = p.as_str().to_string(),
            grammar::Rule::constraint_kind => {
                ctype = match p.as_str() {
                    "entropy_bound" => ast::ConstraintType::EntropyBound,
                    "stability_threshold" => ast::ConstraintType::StabilityThreshold,
                    "topology_surgery_threshold" => ast::ConstraintType::TopologySurgeryThreshold,
                    "human_force_gate" => ast::ConstraintType::HumanForceGate,
                    _ => ast::ConstraintType::EntropyBound,
                }
            }
            grammar::Rule::expr => expr = parse_expr(p)?,
            _ => {}
        }
    }

    Ok(ast::Constraint {
        name,
        expression: expr,
        constraint_type: ctype,
    })
}

fn parse_tags(pair: Pair<grammar::Rule>) -> Vec<String> {
    let mut tags = vec![];
    for p in pair.into_inner() {
        if p.as_rule() == grammar::Rule::ident {
            tags.push(p.as_str().to_string());
        }
    }
    tags
}

fn parse_int_list(pair: Pair<grammar::Rule>) -> Vec<usize> {
    let mut nums = vec![];
    for p in pair.into_inner() {
        if p.as_rule() == grammar::Rule::int {
            if let Ok(v) = p.as_str().parse::<usize>() {
                nums.push(v);
            }
        }
    }
    nums
}

fn parse_betti(pair: Pair<grammar::Rule>) -> Option<Vec<usize>> {
    for p in pair.into_inner() {
        if p.as_rule() == grammar::Rule::betti {
            return Some(parse_int_list(p));
        }
    }
    None
}

fn parse_matrix_spec(pair: Pair<grammar::Rule>) -> ast::MatrixSpec {
    let mut rows = 0usize;
    let mut cols = 0usize;
    let mut sparse = false;
    let mut data: Option<Vec<f64>> = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::int if rows == 0 => {
                rows = p.as_str().parse().unwrap_or(0);
            }
            grammar::Rule::int if cols == 0 => {
                cols = p.as_str().parse().unwrap_or(0);
            }
            grammar::Rule::storage => {
                sparse = p.as_str() == "sparse";
            }
            grammar::Rule::matrix_lit => {
                data = Some(parse_matrix_lit(p));
            }
            _ => {}
        }
    }

    ast::MatrixSpec {
        rows,
        cols,
        sparse,
        data,
    }
}

fn parse_expr(pair: Pair<grammar::Rule>) -> Result<ast::Expression> {
    match pair.as_rule() {
        grammar::Rule::expr => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        grammar::Rule::add_expr => {
            let mut iter = pair.into_inner();
            let first = parse_expr(iter.next().unwrap())?;
            iter.try_fold(first, |left, p| {
                let op = match p.as_rule() {
                    grammar::Rule::add_op => match p.as_str() {
                        "-" => ast::BinaryOp::Sub,
                        _ => ast::BinaryOp::Add,
                    },
                    grammar::Rule::mul_expr => {
                        let right = parse_expr(p)?;
                        return Ok(ast::Expression::BinaryOp {
                            op: ast::BinaryOp::Add,
                            left: Box::new(left),
                            right: Box::new(right),
                        });
                    }
                    _ => ast::BinaryOp::Add,
                };
                // next should be mul_expr
                let right = parse_expr(iter.next().unwrap())?;
                Ok(ast::Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            })
        }
        grammar::Rule::mul_expr => {
            let mut iter = pair.into_inner();
            let first = parse_expr(iter.next().unwrap())?;
            iter.try_fold(first, |left, p| {
                let op = match p.as_rule() {
                    grammar::Rule::mul_op => match p.as_str() {
                        "/" => ast::BinaryOp::Div,
                        "*" => ast::BinaryOp::Mul,
                        "dot" | "·" => ast::BinaryOp::Dot,
                        "⊗" | "otimes" | "tensor" | "x" => ast::BinaryOp::Cross,
                        _ => ast::BinaryOp::Mul,
                    },
                    grammar::Rule::unary_expr => {
                        let right = parse_expr(p)?;
                        return Ok(ast::Expression::BinaryOp {
                            op: ast::BinaryOp::Mul,
                            left: Box::new(left),
                            right: Box::new(right),
                        });
                    }
                    _ => ast::BinaryOp::Mul,
                };
                let right = parse_expr(iter.next().unwrap())?;
                Ok(ast::Expression::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            })
        }
        grammar::Rule::unary_expr => {
            let mut ops: Vec<ast::UnaryOp> = vec![];
            let mut prim: Option<ast::Expression> = None;
            for p in pair.into_inner() {
                match p.as_rule() {
                    grammar::Rule::unary_op => {
                        let op = match p.as_str() {
                            "-" => ast::UnaryOp::Neg,
                            "dagger" | "†" => ast::UnaryOp::Conjugate,
                            "transpose" | "T" => ast::UnaryOp::Transpose,
                            "conj" => ast::UnaryOp::Conjugate,
                            "trace" => ast::UnaryOp::Trace,
                            _ => ast::UnaryOp::Neg,
                        };
                        ops.push(op);
                    }
                    _ => {
                        prim = Some(parse_expr(p)?);
                    }
                }
            }
            let mut expr = prim.unwrap_or(ast::Expression::Raw(String::new()));
            for op in ops.into_iter().rev() {
                expr = ast::Expression::UnaryOp {
                    op,
                    expr: Box::new(expr),
                };
            }
            Ok(expr)
        }
        grammar::Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        grammar::Rule::call => {
            let mut name = String::new();
            let mut args = vec![];
            for p in pair.into_inner() {
                match p.as_rule() {
                    grammar::Rule::ident => name = p.as_str().to_string(),
                    grammar::Rule::arg_list => {
                        for a in p.into_inner() {
                            if a.as_rule() == grammar::Rule::expr {
                                args.push(parse_expr(a)?);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(ast::Expression::FunctionCall { name, args })
        }
        grammar::Rule::grouped => {
            let inner = pair.into_inner().next().unwrap();
            parse_expr(inner)
        }
        grammar::Rule::number => {
            let v: f64 = pair.as_str().parse().unwrap_or(0.0);
            Ok(ast::Expression::Constant(v))
        }
        grammar::Rule::int => {
            let v: f64 = pair.as_str().parse().unwrap_or(0.0);
            Ok(ast::Expression::Constant(v))
        }
        grammar::Rule::float => {
            let v: f64 = pair.as_str().parse().unwrap_or(0.0);
            Ok(ast::Expression::Constant(v))
        }
        grammar::Rule::ident => Ok(ast::Expression::Variable(pair.as_str().to_string())),
        _ => Ok(ast::Expression::Raw(pair.as_str().to_string())),
    }
}

fn parse_matrix_lit(pair: Pair<grammar::Rule>) -> Vec<f64> {
    let mut vals = vec![];
    for p in pair.into_inner() {
        if p.as_rule() == grammar::Rule::row_list {
            for row in p.into_inner() {
                if row.as_rule() == grammar::Rule::row {
                    for numlist in row.into_inner() {
                        if numlist.as_rule() == grammar::Rule::number_list {
                            for num in numlist.into_inner() {
                                if num.as_rule() == grammar::Rule::number {
                                    if let Ok(v) = num.as_str().parse::<f64>() {
                                        vals.push(v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vals
}

fn parse_tensor_net_spec(pair: Pair<grammar::Rule>) -> ast::TensorNetworkSpec {
    let mut network_type = ast::TensorNetworkType::MPS;
    let mut bond_dimensions = vec![];
    let mut truncation = None;

    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::net_kind => {
                network_type = match p.as_str() {
                    "mps" => ast::TensorNetworkType::MPS,
                    "peps" => ast::TensorNetworkType::PEPS,
                    "mera" => ast::TensorNetworkType::MERA,
                    _ => ast::TensorNetworkType::MPS,
                }
            }
            grammar::Rule::bond_dims => {
                bond_dimensions = parse_int_list(p);
            }
            grammar::Rule::trunc_spec => {
                truncation = Some(parse_trunc_spec(p));
            }
            _ => {}
        }
    }

    ast::TensorNetworkSpec {
        network_type,
        bond_dimensions,
        truncation,
    }
}

fn parse_trunc_spec(pair: Pair<grammar::Rule>) -> ast::TruncationSpec {
    let mut method = ast::TruncationMethod::SVD;
    let mut max_bond_dim = 0usize;
    let mut tolerance = 0.0;

    for p in pair.into_inner() {
        match p.as_rule() {
            grammar::Rule::trunc_method => {
                method = match p.as_str() {
                    "qr" => ast::TruncationMethod::QR,
                    "rg" => ast::TruncationMethod::RG,
                    _ => ast::TruncationMethod::SVD,
                }
            }
            grammar::Rule::int => {
                if max_bond_dim == 0 {
                    max_bond_dim = p.as_str().parse().unwrap_or(0);
                }
            }
            grammar::Rule::float => {
                tolerance = p.as_str().parse().unwrap_or(0.0);
            }
            _ => {}
        }
    }

    ast::TruncationSpec {
        method,
        max_bond_dim,
        tolerance,
    }
}

fn parse_string_in_paren(pair: Pair<grammar::Rule>) -> String {
    pair.into_inner()
        .find(|p| p.as_rule() == grammar::Rule::string)
        .map(|p| p.as_str().trim_matches('"').to_string())
        .unwrap_or_default()
}
