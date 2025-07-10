use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use temp_dir::TempDir;

use crate::rustfmt;

pub fn generate_core(
    svd: &Path,
    chip_dir: &Path,
    transforms_dir: &Path,
    core: &str,
) -> anyhow::Result<()> {
    if !fs::exists(&svd)? {
        bail!(
            "SVD file for {} does not exist. help: did you clone submodules?",
            core
        );
    }

    let transform = transforms_dir
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

    let lib_temp = temp.path().join("lib.rs");
    rustfmt(&lib_temp)?;

    let device_x = temp.path().join("device.x");
    let output_dir = chip_dir.join(core.to_lowercase());
    fs::create_dir_all(&output_dir)?;
    fs::copy(&device_x, output_dir.join("device.x"))?;

    // Remove #![no_std] attribute, as this is not lib.rs
    let mut pac = fs::read_to_string(&lib_temp)?;
    pac = pac.replace("#![no_std]\n", "");
    fs::write(&lib_temp, pac)?;

    Command::new("form")
        .arg("-i")
        .arg(lib_temp.canonicalize()?)
        .arg("-o")
        .arg(output_dir.canonicalize()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()?;

    fs::rename(output_dir.join("lib.rs"), output_dir.join("mod.rs"))?;

    Ok(())
}
