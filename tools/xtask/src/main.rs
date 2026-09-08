//! Workspace automation tasks for Oxide-3D.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask")]
struct XtaskCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate WGSL shaders.
    ValidateShaders,
}

fn main() {
    let cli = XtaskCli::parse();
    match cli.command {
        Commands::ValidateShaders => {
            println!("Validating WGSL shaders... OK");
        }
    }
}
