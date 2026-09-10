//! Oxide-3D Headless Automation & CI Runner.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "oxide-headless")]
#[command(about = "Oxide-3D Headless CAD/CAE/CAM/PLM Automation Runner", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open a CAD model file and optionally export it.
    Open {
        /// Input file path (.oxd, .step, .stl).
        path: PathBuf,
        /// Optional export path for STEP AP242.
        #[arg(long)]
        export_step: Option<PathBuf>,
    },
    /// Run an automated FEA or CFD simulation case.
    Simulate {
        /// Case JSON configuration path.
        case: PathBuf,
        /// Output results directory.
        #[arg(long)]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Open { path, export_step } => {
            println!("Opening model: {:?}", path);
            if let Some(out) = export_step {
                println!("Exporting STEP geometry to: {:?}", out);
            }
        }
        Commands::Simulate { case, output } => {
            println!("Simulating case: {:?} -> Output: {:?}", case, output);
        }
    }
}
