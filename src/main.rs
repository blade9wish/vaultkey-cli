mod config;
mod error;

use clap::{Parser, Subcommand};
use anyhow::Result;

/// vaultkey-cli: manage encrypted secret bundles
#[derive(Parser, Debug)]
#[command(name = "vaultkey", version, about)]
struct Cli {
    /// Path to the config file
    #[arg(short, long, default_value = "vaultkey.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show the current configuration
    ShowConfig,
    /// Validate the configuration file
    ValidateConfig,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ShowConfig => {
            let cfg = config::VaultConfig::load(&cli.config)?;
            println!("{:#?}", cfg);
        }
        Commands::ValidateConfig => {
            config::VaultConfig::load(&cli.config)?;
            println!("Config is valid: {}", cli.config);
        }
    }

    Ok(())
}
