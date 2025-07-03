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

use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use temp_dir::TempDir;

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
];

fn main() -> anyhow::Result<()> {
    verify_chiptool().context("chiptool is not installed")?;

    let current = env::current_dir()?;

    let chips = current.join("src").join("chips");
    // Might not exist.
    let _ = fs::remove_dir_all(chips);

    let mut args = env::args();

    let generate_chips: Vec<&Feature> = if args.len() > 1 {
        let chip = args.nth(1).context("unreachable")?;

        let feature = GENERATE
            .iter()
            .find(|feature| feature.chip == chip)
            .context("selected chip does not exist in generate list")?;

        vec![feature]
    } else {
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

    Ok(())
}

fn verify_chiptool() -> anyhow::Result<()> {
    Command::new("chiptool")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    Ok(())
}

fn generate_chip(current_dir: &Path, feature: &Feature) -> anyhow::Result<()> {
    let chip_dir = current_dir
        .join("data")
        .join("mcux-soc-svd")
        .join(feature.chip);

    for core in feature.cores {
        println!("Generating {}/{}", feature.chip, core);

        let svd = chip_dir.join(core).with_extension("xml");

        if !fs::exists(&svd)? {
            bail!(
                "SVD file for {} does not exist. help: did you clone submodules?",
                feature.chip
            );
        }

        let transform = current_dir
            .join("transforms")
            .join(core.to_lowercase())
            .with_extension("yaml");

        if !fs::exists(&transform)? {
            bail!(
                "transform for core \"{}\" does not exist?",
                core.to_lowercase()
            );
        }

        let temp = TempDir::new()
            .context("Creating temp dir")?
            .dont_delete_on_drop();

        let output = Command::new("chiptool")
            .arg("generate")
            .arg("--svd")
            .arg(svd.canonicalize()?)
            .arg("--transform")
            .arg(transform.canonicalize()?)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(temp.path())
            .output()?;

        if !output.status.success() {
            bail!(
                "Error generating {core}:\nSTDERR:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let lib = temp.path().join("lib.rs");
        rustfmt(&lib)?;

        let device_x = temp.path().join("device.x");

        let output_dir = current_dir
            .join("src")
            .join("chips")
            .join(core.to_lowercase());
        let pac_rs = output_dir.join("pac.rs");

        fs::create_dir_all(&output_dir)?;

        fs::copy(&lib, &pac_rs)?;
        fs::copy(&device_x, output_dir.join("device.x"))?;

        // Remove #![no_std] attribute, as this is not lib.rs
        let mut pac = fs::read_to_string(&pac_rs)?;
        pac = pac.replace("#![no_std]\n", "");

        fs::write(&pac_rs, pac)?;
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
