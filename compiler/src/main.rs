use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod parser;
mod typechecker;
mod semantic;
mod ir;
mod codegen;
mod packager;
mod safety;

fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("APEIRON-X Compiler v0.1.0");
    info!("HPM-DL to IR Compiler");

    // TODO: Implement CLI interface
    // TODO: Parse HPM-DL source files
    // TODO: Type checking
    // TODO: Semantic analysis
    // TODO: IR generation
    // TODO: Operator spec generation
    // TODO: Artifact packaging

    Ok(())
}

