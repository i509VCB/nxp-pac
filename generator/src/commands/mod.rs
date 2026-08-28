use anyhow::{Result, anyhow};
use itertools::Itertools;

use crate::ChipDescription;

pub mod extract;
pub mod extract_source;
pub mod generate;

fn select_chip(chip_name: &str) -> Result<&'static ChipDescription> {
    crate::CHIPS
        .iter()
        .find(|chip_description| chip_description.chip == chip_name)
        .ok_or_else(|| {
            let list = crate::CHIPS
                .iter()
                .map(|chip| format!("- {}", chip.chip))
                .join("\n");

            anyhow!("Supported chips:\n{}", list)
                .context(format!("Selected chip '{}' not supported", chip_name))
        })
}
