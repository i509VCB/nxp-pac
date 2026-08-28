use std::env;

use anyhow::{Result, bail};
use clap::Parser;

/// Extract reviewed source artifacts without enabling PAC generation.
#[derive(Parser)]
pub struct ExtractSource {
    /// Source-only chip to extract.
    #[clap(required = true)]
    pub chip: String,
}

pub fn extract_source(args: ExtractSource) -> Result<()> {
    if args.chip != "MCXA156" {
        bail!("source extraction is not configured for {}", args.chip);
    }

    let current = env::current_dir()?;
    crate::source_validation::extract_mcxa156(&current, &current.join("data/source-peripherals"))?;
    crate::source_validation::validate_mcxa156(&current)?;
    Ok(())
}
