use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod orchestrator;
mod grpc_api;
mod scheduler;
mod sandbox;
mod monitoring;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("APEIRON-X Runtime v0.1.0");
    info!("Orchestrator and Task Scheduler");

    // TODO: Initialize orchestrator
    // TODO: Start gRPC server
    // TODO: Start scheduler
    // TODO: Initialize sandbox manager
    // TODO: Start monitoring

    Ok(())
}

