use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::bail;

/// Perform rustfmt on a single file.
pub fn rustfmt(path: &Path) -> anyhow::Result<()> {
    let output = Command::new("rustfmt")
        .args(["--edition", "2024", "--config", "newline_style=Unix"])
        .arg(path.canonicalize()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error during rustfmt: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use temp_dir::TempDir;

    use super::rustfmt;

    #[test]
    fn rustfmt_output_uses_lf_on_every_host() {
        let temp = TempDir::new().expect("create temporary directory");
        let source = temp.path().join("generated.rs");
        fs::write(&source, "pub fn generated() {\r\n}\r\n").expect("write source");

        rustfmt(&source).expect("format generated source");

        let formatted = fs::read(source).expect("read formatted source");
        assert!(!formatted.windows(2).any(|bytes| bytes == b"\r\n"));
        assert!(formatted.contains(&b'\n'));
    }
}
