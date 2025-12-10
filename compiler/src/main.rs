use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};
use serde_json;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod parser;
mod typechecker;
mod semantic;
mod ir;
mod codegen;
mod packager;
mod safety;

fn ast_to_ir(program: &parser::ast::Program) -> ir::IrProgram {
    let operators = program
        .operators
        .iter()
        .map(|op| ir::IrOperator {
            name: op.name.clone(),
            operator_type: match op.operator_type {
                parser::ast::OperatorType::Hamiltonian => ir::IrOperatorType::Hamiltonian,
                parser::ast::OperatorType::Dissipative => ir::IrOperatorType::Dissipative,
                parser::ast::OperatorType::TopologicalMutation => {
                    ir::IrOperatorType::TopologicalMutation
                }
                parser::ast::OperatorType::EntropyFlow => ir::IrOperatorType::EntropyFlow,
            },
            spec: ir::IrOperatorSpec {
                target: ir::IrTarget::CPU,
                implementation: op
                    .spec
                    .matrix_representation
                    .as_ref()
                    .map(|m| ir::IrOperatorImplementation::DenseMatrix {
                        data: m.data.clone().unwrap_or_default(),
                    })
                    .or_else(|| {
                        op.spec.tensor_network.as_ref().map(|tn| {
                            ir::IrOperatorImplementation::TensorNetwork {
                                network: ir::TensorNetworkSpec {
                                    network_type: format!("{:?}", tn.network_type),
                                    tensors: vec![],
                                    bonds: tn
                                        .bond_dimensions
                                        .iter()
                                        .enumerate()
                                        .map(|(i, d)| ir::BondSpec {
                                            tensor1: "t".into(),
                                            index1: i,
                                            tensor2: "t".into(),
                                            index2: i + 1,
                                            dimension: *d,
                                        })
                                        .collect(),
                                },
                            }
                        })
                    })
                    .or_else(|| {
                        op.spec
                            .gpu_kernel
                            .as_ref()
                            .map(|k| ir::IrOperatorImplementation::CUDAKernel {
                                kernel_name: k.clone(),
                            })
                    })
                    .unwrap_or(ir::IrOperatorImplementation::DenseMatrix { data: vec![] }),
            },
        })
        .collect();

    ir::IrProgram {
        modules: vec![ir::IrModule {
            name: "main".into(),
            functions: vec![],
            operators,
        }],
        entry_point: "main".into(),
    }
}

#[derive(ClapParser, Debug)]
#[command(name = "apeiron-compiler", version = "0.1.0", about = "HPM-DL to IR compiler (CLI-first)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile HPM-DL to IR / operator catalog / job descriptor
    Compile {
        /// HPM-DL source file
        #[arg(short, long)]
        input: PathBuf,
        /// Output IR path (JSON-L)
        #[arg(long)]
        out_ir: PathBuf,
        /// Output operator catalog path (JSON)
        #[arg(long)]
        out_ops: Option<PathBuf>,
        /// Output job descriptor path (JSON)
        #[arg(long)]
        out_job: Option<PathBuf>,
    },
    /// Run safety checks and report
    Check {
        /// HPM-DL source file
        #[arg(short, long)]
        input: PathBuf,
        /// Output safety report (JSON)
        #[arg(long)]
        report: Option<PathBuf>,
    },
}

fn init_tracing() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

fn main() -> Result<()> {
    init_tracing()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            out_ir,
            out_ops,
            out_job,
        } => run_compile(input, out_ir, out_ops, out_job),
        Commands::Check { input, report } => run_check(input, report),
    }
}

fn run_compile(
    input: PathBuf,
    out_ir: PathBuf,
    out_ops: Option<PathBuf>,
    out_job: Option<PathBuf>,
) -> Result<()> {
    info!("compile: {:?}", input);
    let source = std::fs::read_to_string(&input)
        .with_context(|| format!("failed to read input file {:?}", input))?;

    // Pipeline: parse -> typecheck -> semantic -> IR -> codegen -> package
    let program = parser::parse(&source).context("syntax error")?;

    let mut tc = typechecker::TypeChecker::new();
    let tc_program = tc.typecheck(&program).context("typecheck failed")?;

    let mut sem = semantic::SemanticAnalyzer::new();
    let _expanded = sem
        .expand_axioms(&tc_program.program)
        .context("semantic analysis failed")?;

    // Build IR from AST (non-placeholder)
    let ir_program = ast_to_ir(&tc_program.program);

    let codegen = codegen::CodeGenerator::new();
    let catalog = codegen
        .generate_operator_catalog(&ir_program)
        .context("codegen catalog failed")?;
    let _numeric_specs = codegen
        .generate_numeric_specs(&ir_program)
        .context("codegen numeric specs failed")?;
    let job = codegen
        .generate_job_descriptor(&ir_program)
        .context("codegen job descriptor failed")?;

    let packager = packager::ArtifactPackager::new();
    packager
        .package_ir(&ir_program, &out_ir)
        .context("writing IR failed")?;
    if let Some(out_ops) = out_ops {
        packager
            .package_operators(&catalog, &out_ops)
            .context("writing operator catalog failed")?;
    }
    if let Some(out_job) = out_job {
        packager
            .package_job(&job, &out_job)
            .context("writing job descriptor failed")?;
    }

    info!("compile finished");
    Ok(())
}

fn run_check(input: PathBuf, report: Option<PathBuf>) -> Result<()> {
    info!("check: {:?}", input);
    let source = std::fs::read_to_string(&input)
        .with_context(|| format!("failed to read input file {:?}", input))?;

    let program = parser::parse(&source).context("syntax error")?;
    let safety = safety::SafetyChecker::new();
    let human_force = safety
        .check_human_force_gates(&program)
        .context("human_force check failed")?;
    let entropy = safety
        .check_entropy_blowup(&program)
        .context("entropy check failed")?;
    let anomaly = safety
        .check_anomaly_thresholds(&program)
        .context("anomaly check failed")?;
    let topology = safety
        .check_topology_surgery_throttle(&program)
        .context("topology check failed")?;

    let report_obj = serde_json::json!({
        "human_force_gates": human_force.human_force_gates,
        "warnings": human_force.warnings,
        "errors": human_force.errors,
        "entropy": {
            "estimated_entropy": entropy.estimated_entropy,
            "threshold": entropy.threshold,
            "safe": entropy.safe,
        },
        "anomaly": {
            "anomalies": anomaly.anomalies,
            "threshold": anomaly.threshold,
        },
        "topology": {
            "surgery_count": topology.surgery_count,
            "throttle_limit": topology.throttle_limit,
            "safe": topology.safe,
        }
    });

    if let Some(path) = report {
        std::fs::write(&path, serde_json::to_string_pretty(&report_obj)?)
            .with_context(|| format!("failed to write report {:?}", path))?;
        info!("report written to {:?}", path);
    } else {
        println!("{}", serde_json::to_string_pretty(&report_obj)?);
    }

    Ok(())
}