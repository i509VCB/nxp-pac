//! nxp-pac generator
//!
//! This is used to generate the PAC. This applies the patches to each chip which has been enabled. For
//! some chips this may also include fetching metadata.
//!
//! ## Running
//!
//! Running the generate is done using `cargo run -p generator`. Additionally to only generate a single part,
//! you can specify the name of the part. For example to generate only `MIMXRT1011`:
//!
//! ```text
//! cargo run -p generator -- MIMXRT1011
//! ```

use clap::Parser;
use generator::commands::{
    extract::{Extract, extract},
    extract_source::{ExtractSource, extract_source},
    generate::{Generate, generate},
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
enum Subcommand {
    Generate(Generate),
    Extract(Extract),
    ExtractSource(ExtractSource),
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let opts: Opts = Opts::parse();

    match opts.subcommand {
        Subcommand::Generate(args) => generate(args),
        Subcommand::Extract(args) => extract(args),
        Subcommand::ExtractSource(args) => extract_source(args),
    }
}
