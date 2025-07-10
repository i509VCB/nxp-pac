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

mod iomuxc;
mod metadata;
mod pac;

use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Feature {
    /// The name of the chip to generate.
    chip: &'static str,

    /// Cores to generate. If the chip has a single core, then this is the same as the
    /// [`name`](Feature::name) of the chip.
    cores: &'static [&'static str],
}

/// Parts (and cores) to generate.
#[rustfmt::skip]
const GENERATE: &[Feature] = &[
    Feature { chip: "MIMXRT1011", cores: &["MIMXRT1011"] },
    Feature { chip: "MIMXRT1062", cores: &["MIMXRT1062"] },

    Feature { chip: "MCXN947", cores: &["MCXN947_cm33_core0", "MCXN947_cm33_core1"]}
];

fn main() -> anyhow::Result<()> {
    verify_chiptool().context("chiptool is not installed")?;

    let current = env::current_dir()?;

    let chips = current.join("src").join("chips");

    let mut args = env::args();

    let generate_chips: Vec<&Feature> = if args.len() > 1 {
        let chip = args.nth(1).context("unreachable")?;

        let feature = GENERATE
            .iter()
            .find(|feature| feature.chip == chip)
            .context("selected chip does not exist in generate list")?;

        vec![feature]
    } else {
        // Might not exist.
        let _ = fs::remove_dir_all(chips);

        GENERATE.iter().collect()
    };

    // Generating code for SVDs can take a moment (RT11xx can generate a million lines of code)
    // so it is best to run multiple cores in parallel.
    let outputs: Vec<anyhow::Result<()>> = generate_chips
        .par_iter()
        .map(|&feature| generate_chip(&current, feature))
        .collect();

    let mut error = false;

    for output in outputs {
        if let Err(e) = output {
            error |= true;
            eprintln!("{e}");
        }
    }

    if error {
        bail!("Failed to generate chips");
    }

    println!("Formatting code");
    Command::new("cargo")
        .arg("fmt")
        .current_dir(current)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?;

    Ok(())
}

fn verify_chiptool() -> anyhow::Result<()> {
    Command::new("chiptool")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;
    Ok(())
}

fn generate_chip(current_dir: &Path, feature: &Feature) -> anyhow::Result<()> {
    let chip_src_dir = current_dir
        .join("data")
        .join("mcux-soc-svd")
        .join(feature.chip);

    for core in feature.cores {
        let svd = chip_src_dir.join(core).with_extension("xml");
        let transforms_dir = current_dir.join("transforms");
        let chips_dir = current_dir.join("src").join("chips");

        println!("Generating {}/{}", feature.chip, core);
        pac::generate_core(&svd, &chips_dir, &transforms_dir, &core).context("Generating PAC")?;
        metadata::generate_core(&svd, &chips_dir, &core).context("Generating metadata")?;
    }

    Ok(())
}

fn rustfmt(path: &Path) -> anyhow::Result<()> {
    let output = Command::new("rustfmt")
        .arg(path.canonicalize()?)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error during rustfmt: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
