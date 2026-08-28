//! Fail-closed checks that bind the MCXA156 MetaPAC to its locked SVD.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};

use anyhow::{Context, anyhow, ensure};
use chiptool::ir::{Array, Block, BlockItemInner, IR};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use svd_parser::svd::{Device, Peripheral};
use temp_dir::TempDir;

use crate::metadata::Metadata;

const CORTEX_M_PERIPHERALS: [&str; 4] = ["SCnSCB", "SysTick", "NVIC", "SCB"];
const HAL_ENABLED_PERIPHERALS: [&str; 8] = [
    "GPIO0", "GPIO3", "LPUART0", "MRCC0", "OSTIMER0", "PORT0", "PORT3", "SCG0",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceLock {
    schema: u8,
    chip: String,
    repository: String,
    revision: String,
    path: String,
    sha256: String,
}
// MCXA156 does not publish request-source enumerations in its SVD. These are
// the exact entries carried from its manifest-pinned PERI_DMA.h; the
// reconciliation evidence records that source and hash explicitly.
const CARRIED_DMA_REQUESTS: &[(&str, &str, u8)] = &[
    ("WUU0", "WUU0WakeUpEvent", 1),
    ("CAN0", "CAN0", 2),
    ("LPI2C2", "LPI2C2Rx", 3),
    ("LPI2C2", "LPI2C2Tx", 4),
    ("LPI2C3", "LPI2C3Rx", 5),
    ("LPI2C3", "LPI2C3Tx", 6),
    ("I3C0", "I3C0Rx", 7),
    ("I3C0", "I3C0Tx", 8),
    ("LPI2C0", "LPI2C0Rx", 11),
    ("LPI2C0", "LPI2C0Tx", 12),
    ("LPI2C1", "LPI2C1Rx", 13),
    ("LPI2C1", "LPI2C1Tx", 14),
    ("LPSPI0", "LPSPI0Rx", 15),
    ("LPSPI0", "LPSPI0Tx", 16),
    ("LPSPI1", "LPSPI1Rx", 17),
    ("LPSPI1", "LPSPI1Tx", 18),
    ("LPUART0", "LPUART0Rx", 21),
    ("LPUART0", "LPUART0Tx", 22),
    ("LPUART1", "LPUART1Rx", 23),
    ("LPUART1", "LPUART1Tx", 24),
    ("LPUART2", "LPUART2Rx", 25),
    ("LPUART2", "LPUART2Tx", 26),
    ("LPUART3", "LPUART3Rx", 27),
    ("LPUART3", "LPUART3Tx", 28),
    ("LPUART4", "LPUART4Rx", 29),
    ("LPUART4", "LPUART4Tx", 30),
    ("CTIMER0", "CTIMER0M0", 31),
    ("CTIMER0", "CTIMER0M1", 32),
    ("CTIMER1", "CTIMER1M0", 33),
    ("CTIMER1", "CTIMER1M1", 34),
    ("CTIMER2", "CTIMER2M0", 35),
    ("CTIMER2", "CTIMER2M1", 36),
    ("CTIMER3", "CTIMER3M0", 37),
    ("CTIMER3", "CTIMER3M1", 38),
    ("CTIMER4", "CTIMER4M0", 39),
    ("CTIMER4", "CTIMER4M1", 40),
    ("FLEX_PWM0", "FlexPWM0Mcapt0", 41),
    ("FLEX_PWM0", "FlexPWM0Mcapt1", 42),
    ("FLEX_PWM0", "FlexPWM0Mcapt2", 43),
    ("FLEX_PWM0", "FlexPWM0Mval0", 45),
    ("FLEX_PWM0", "FlexPWM0Mval1", 46),
    ("FLEX_PWM0", "FlexPWM0Mval2", 47),
    ("LPTMR0", "LPTMR0CounterMatchEvent", 49),
    ("ADC0", "ADC0FifoRequest", 51),
    ("ADC1", "ADC1FifoRequest", 52),
    ("CMP0", "CMP0DmaRequest", 53),
    ("CMP1", "CMP1DmaRequest", 54),
    ("DAC0", "DAC0FifoRequest", 56),
    ("GPIO0", "GPIO0PinEvent0", 60),
    ("GPIO1", "GPIO1PinEvent0", 61),
    ("GPIO2", "GPIO2PinEvent0", 62),
    ("GPIO3", "GPIO3PinEvent0", 63),
    ("GPIO4", "GPIO4PinEvent0", 64),
    ("EQDC0", "BUFFER", 65),
    ("EQDC1", "BUFFER", 66),
    ("FLEXIO0", "FLEXIO0SR0", 71),
    ("FLEXIO0", "FLEXIO0SR1", 72),
    ("FLEXIO0", "FLEXIO0SR2", 73),
    ("FLEXIO0", "FLEXIO0SR3", 74),
    ("FLEX_PWM1", "FlexPWM1Mcapt0", 79),
    ("FLEX_PWM1", "FlexPWM1Mcapt1", 80),
    ("FLEX_PWM1", "FlexPWM1Mcapt2", 81),
    ("FLEX_PWM1", "FlexPWM1Mval0", 83),
    ("FLEX_PWM1", "FlexPWM1Mval1", 84),
    ("FLEX_PWM1", "FlexPWM1Mval2", 85),
];

/// Number of source facts checked by an MCXA156 source validation pass.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceValidationCoverage {
    pub peripheral_instances: usize,
    pub register_instances: usize,
}

/// Validate that the MCXA156 metadata is a complete inventory of the locked
/// SVD's device peripherals and that every checked-in register description is
/// byte-for-byte reproducible from that SVD and the checked-in transforms.
///
/// Cortex-M architectural peripherals are intentionally excluded because the
/// PAC obtains those definitions from `cortex-m`.
pub fn validate_mcxa156(current: &Path) -> anyhow::Result<SourceValidationCoverage> {
    let source = TempDir::with_prefix("nxp-pac-mcxa156-source-")
        .context("creating temporary MCXA156 source directory")?;
    let svd_path = materialize_locked_mcxa156_svd(current, &source)?;
    let metadata_path = current.join("data/metadata/MCXA156.json");
    let peripheral_dir = current.join("data/source-peripherals");

    let svd_contents = fs::read_to_string(&svd_path)
        .with_context(|| format!("reading locked SVD {}", svd_path.display()))?;
    let device = svd_parser::parse_with_config(
        &svd_contents,
        &svd_parser::Config::default()
            .expand(true)
            .expand_properties(true),
    )
    .with_context(|| format!("parsing locked SVD {}", svd_path.display()))?;

    let metadata_contents = fs::read_to_string(&metadata_path)
        .with_context(|| format!("reading metadata {}", metadata_path.display()))?;
    let metadata: Metadata = serde_json::from_str(&metadata_contents)
        .with_context(|| format!("parsing metadata {}", metadata_path.display()))?;

    let coverage = validate_complete_inventory(&device, &metadata, &peripheral_dir)?;
    validate_reproducible_register_artifacts(current, &svd_path)?;

    tracing::info!(
        peripheral_instances = coverage.peripheral_instances,
        register_instances = coverage.register_instances,
        "MCXA156 complete MetaPAC inventory matches locked SVD and transforms"
    );
    Ok(coverage)
}

/// Recreate the reviewed MCXA156 register artifacts from the locked SVD and
/// checked-in transforms without enabling PAC generation for the chip.
pub fn extract_mcxa156(current: &Path, output: &Path) -> anyhow::Result<()> {
    let source = TempDir::with_prefix("nxp-pac-mcxa156-source-")
        .context("creating temporary MCXA156 source directory")?;
    let svd_path = materialize_locked_mcxa156_svd(current, &source)?;
    extract_mcxa156_from_svd(current, &svd_path, output)
}

fn extract_mcxa156_from_svd(current: &Path, svd_path: &Path, output: &Path) -> anyhow::Result<()> {
    crate::metadata::extract_peripherals(
        svd_path,
        "MCXA156",
        Some(&current.join("data/transforms")),
        &output.join("MCXA156"),
    )
    .context("extracting MCXA156 source artifacts")
}

fn materialize_locked_mcxa156_svd(
    current: &Path,
    temp: &TempDir,
) -> anyhow::Result<std::path::PathBuf> {
    let lock_path = current.join("data/source-locks/MCXA156.json");
    let lock_contents = fs::read_to_string(&lock_path)
        .with_context(|| format!("reading MCXA156 source lock {}", lock_path.display()))?;
    let lock: SourceLock = serde_json::from_str(&lock_contents)
        .with_context(|| format!("parsing MCXA156 source lock {}", lock_path.display()))?;

    ensure!(lock.schema == 1, "unsupported MCXA156 source-lock schema");
    ensure!(
        lock.chip == "MCXA156",
        "MCXA156 source lock names a different chip"
    );
    ensure!(
        lock.revision.len() == 40 && lock.revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "MCXA156 source lock revision must be a full Git object ID"
    );
    ensure!(
        lock.sha256.len() == 64 && lock.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "MCXA156 source lock SHA-256 must be a full digest"
    );

    let repository = current.join("data/mcux-soc-svd");
    let object = format!("{}:{}", lock.revision, lock.path);
    let output = Command::new("git")
        .arg("-C")
        .arg(&repository)
        .args(["show", &object])
        .output()
        .context("running git to materialize the locked MCXA156 SVD")?;
    ensure!(
        output.status.success(),
        "locked MCXA156 source object is unavailable: {}; fetch it with `git -C {} fetch --depth=1 {} {}`",
        String::from_utf8_lossy(&output.stderr).trim(),
        repository.display(),
        lock.repository,
        lock.revision
    );

    let actual_sha256 = format!("{:x}", Sha256::digest(&output.stdout));
    ensure!(
        actual_sha256 == lock.sha256,
        "locked MCXA156 SVD SHA-256 mismatch: expected {}, got {}",
        lock.sha256,
        actual_sha256
    );

    let svd_path = temp.child("MCXA156.xml");
    fs::write(&svd_path, output.stdout).context("writing temporary locked MCXA156 SVD")?;
    Ok(svd_path)
}

fn validate_complete_inventory(
    device: &Device,
    metadata: &Metadata,
    peripheral_dir: &Path,
) -> anyhow::Result<SourceValidationCoverage> {
    let mut source_peripherals = BTreeMap::new();
    for peripheral in device
        .peripherals
        .iter()
        .filter(|peripheral| !is_cortex_m_peripheral(&peripheral.name))
    {
        ensure!(
            source_peripherals
                .insert(peripheral.name.clone(), peripheral)
                .is_none(),
            "locked SVD contains duplicate peripheral {}",
            peripheral.name
        );
    }

    let mut metadata_names = BTreeSet::new();
    let mut covered_source_names = BTreeSet::new();
    let mut register_instances = 0;
    for peripheral in &metadata.peripherals {
        ensure!(
            metadata_names.insert(peripheral.name.clone()),
            "MCXA156 metadata contains duplicate peripheral {}",
            peripheral.name
        );

        let source_name = svd_peripheral_name(&peripheral.name);
        ensure!(
            covered_source_names.insert(source_name.to_owned()),
            "MCXA156 metadata maps more than one peripheral to locked-SVD peripheral {source_name}"
        );
        let svd_peripheral = source_peripherals.get(source_name).ok_or_else(|| {
            anyhow!(
                "MCXA156 metadata peripheral {} is not a non-core peripheral in the locked SVD",
                peripheral.name
            )
        })?;
        validate_peripheral(peripheral, svd_peripheral, peripheral_dir)?;
        register_instances += svd_peripheral.all_registers().count();
    }

    let source_names = source_peripherals.keys().cloned().collect::<BTreeSet<_>>();
    let missing = source_names
        .difference(&covered_source_names)
        .cloned()
        .collect::<Vec<_>>();
    let extra = covered_source_names
        .difference(&source_names)
        .cloned()
        .collect::<Vec<_>>();
    ensure!(
        missing.is_empty() && extra.is_empty(),
        "MCXA156 metadata is not a complete locked-SVD inventory; missing: {missing:?}; extra: {extra:?}"
    );

    let hal_enabled = metadata
        .peripherals
        .iter()
        .filter(|peripheral| !peripheral.pac_only)
        .map(|peripheral| peripheral.name.as_str())
        .collect::<BTreeSet<_>>();
    let expected_hal_enabled = HAL_ENABLED_PERIPHERALS.into_iter().collect::<BTreeSet<_>>();
    ensure!(
        hal_enabled == expected_hal_enabled,
        "MCXA156 HAL-enabled peripheral inventory differs; expected: {expected_hal_enabled:?}; actual: {hal_enabled:?}"
    );

    let dma_requests = metadata
        .peripherals
        .iter()
        .flat_map(|peripheral| {
            peripheral.dma_muxing.iter().map(|request| {
                (
                    peripheral.name.as_str(),
                    request.signal.as_str(),
                    request.mux.as_str(),
                    request.request,
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let expected_dma_requests = CARRIED_DMA_REQUESTS
        .iter()
        .map(|(peripheral, signal, request)| (*peripheral, *signal, "DMA3", *request))
        .collect::<BTreeSet<_>>();
    ensure!(
        dma_requests == expected_dma_requests,
        "MCXA156 carried DMA request inventory differs; expected: {expected_dma_requests:?}; actual: {dma_requests:?}"
    );

    Ok(SourceValidationCoverage {
        peripheral_instances: metadata_names.len(),
        register_instances,
    })
}

fn validate_peripheral(
    peripheral: &crate::metadata::Peripheral,
    svd_peripheral: &Peripheral,
    peripheral_dir: &Path,
) -> anyhow::Result<()> {
    let block_path = peripheral.parse_block_path()?.ok_or_else(|| {
        anyhow!(
            "MCXA156 peripheral {} has no source-bound block",
            peripheral.name
        )
    })?;
    ensure!(
        block_path.path.starts_with("MCXA156/"),
        "MCXA156 peripheral {} uses non-derivative block {}",
        peripheral.name,
        block_path.path
    );

    let address = peripheral.peripheral_address.as_deref().ok_or_else(|| {
        anyhow!(
            "MCXA156 peripheral {} has a source-bound block but no address",
            peripheral.name
        )
    })?;
    let metadata_address = parse_address(address).with_context(|| {
        format!(
            "parsing address {address:?} for MCXA156 peripheral {}",
            peripheral.name
        )
    })?;
    ensure!(
        metadata_address == svd_peripheral.base_address,
        "MCXA156 peripheral {} base address disagrees with locked SVD: metadata {metadata_address:#x}, SVD {:#x}",
        peripheral.name,
        svd_peripheral.base_address
    );

    let yaml_path = peripheral_dir.join(&block_path.path).with_extension("yaml");
    let yaml = fs::File::open(&yaml_path)
        .with_context(|| format!("opening generated MetaPAC block {}", yaml_path.display()))?;
    let ir: IR = serde_yaml::from_reader(yaml)
        .with_context(|| format!("parsing generated MetaPAC block {}", yaml_path.display()))?;
    let block = ir.blocks.get(&block_path.rust_type_name).ok_or_else(|| {
        anyhow!(
            "MetaPAC block {} has no type {} for peripheral {}",
            yaml_path.display(),
            block_path.rust_type_name,
            peripheral.name
        )
    })?;
    if svd_peripheral.name.starts_with("PORT") {
        validate_sparse_port_registers(svd_peripheral, block)?;
    }
    if svd_peripheral.name == "CAN0" {
        validate_expanded_register_offsets(svd_peripheral, &ir, block)?;
    }
    Ok(())
}

fn validate_expanded_register_offsets(
    peripheral: &Peripheral,
    ir: &IR,
    block: &Block,
) -> anyhow::Result<()> {
    let mut source_offsets = peripheral
        .all_registers()
        .map(|register| register.address_offset)
        .collect::<Vec<_>>();
    source_offsets.sort_unstable();

    let mut generated_offsets = Vec::new();
    collect_register_offsets(ir, block, 0, &mut generated_offsets)?;
    generated_offsets.sort_unstable();
    ensure!(
        generated_offsets == source_offsets,
        "MCXA156 {} generated register offsets differ from the complete locked SVD inventory; source count: {}; generated count: {}",
        peripheral.name,
        source_offsets.len(),
        generated_offsets.len()
    );
    Ok(())
}

fn collect_register_offsets(
    ir: &IR,
    block: &Block,
    base_offset: u32,
    offsets: &mut Vec<u32>,
) -> anyhow::Result<()> {
    for item in &block.items {
        let array_offsets = match &item.array {
            None => vec![0],
            Some(Array::Regular(array)) => {
                (0..array.len).map(|index| index * array.stride).collect()
            }
            Some(Array::Cursed(array)) => array.offsets.clone(),
        };
        for array_offset in array_offsets {
            let item_offset = base_offset + item.byte_offset + array_offset;
            match &item.inner {
                BlockItemInner::Register(_) => offsets.push(item_offset),
                BlockItemInner::Block(nested) => {
                    let nested_block = ir.blocks.get(&nested.block).ok_or_else(|| {
                        anyhow!(
                            "generated block references missing nested block {}",
                            nested.block
                        )
                    })?;
                    collect_register_offsets(ir, nested_block, item_offset, offsets)?;
                }
            }
        }
    }
    Ok(())
}

fn validate_sparse_port_registers(peripheral: &Peripheral, block: &Block) -> anyhow::Result<()> {
    let source_registers = peripheral
        .all_registers()
        .map(|register| {
            (
                register.name.trim_end_matches('_').to_ascii_lowercase(),
                register.address_offset,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut generated_registers = BTreeMap::new();
    for item in &block.items {
        ensure!(
            matches!(item.inner, BlockItemInner::Register(_)) && item.array.is_none(),
            "MCXA156 {} must preserve its exact sparse scalar register inventory; generated item {} is nested or an array",
            peripheral.name,
            item.name
        );
        ensure!(
            generated_registers
                .insert(item.name.to_ascii_lowercase(), item.byte_offset)
                .is_none(),
            "MCXA156 {} generated duplicate register {}",
            peripheral.name,
            item.name
        );
    }
    ensure!(
        generated_registers == source_registers,
        "MCXA156 {} generated register inventory differs from the locked sparse SVD; source: {source_registers:?}; generated: {generated_registers:?}",
        peripheral.name
    );
    Ok(())
}

fn validate_reproducible_register_artifacts(current: &Path, svd_path: &Path) -> anyhow::Result<()> {
    let temp = TempDir::with_prefix("nxp-pac-mcxa156-")
        .context("creating temporary MCXA156 extraction directory")?;
    let generated_dir = temp.child("MCXA156");
    extract_mcxa156_from_svd(current, svd_path, temp.path())
        .context("regenerating MCXA156 peripheral artifacts")?;

    let committed_dir = current.join("data/source-peripherals/MCXA156");
    let generated_files = artifact_files(&generated_dir)?;
    let committed_files = artifact_files(&committed_dir)?;
    ensure!(
        generated_files == committed_files,
        "MCXA156 generated artifact inventory differs; generated: {generated_files:?}; committed: {committed_files:?}"
    );

    for name in generated_files {
        let generated = fs::read(generated_dir.join(&name))
            .with_context(|| format!("reading regenerated MCXA156 artifact {name}"))?;
        let committed = fs::read(committed_dir.join(&name))
            .with_context(|| format!("reading committed MCXA156 artifact {name}"))?;
        ensure!(
            generated == committed,
            "MCXA156 artifact {name} is stale; regenerate it from the locked SVD"
        );
    }
    Ok(())
}

fn artifact_files(directory: &Path) -> anyhow::Result<BTreeSet<String>> {
    let mut files = BTreeSet::new();
    for entry in fs::read_dir(directory)
        .with_context(|| format!("reading MCXA156 artifact directory {}", directory.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".yaml") || name.ends_with(".json") {
            files.insert(name);
        }
    }
    Ok(files)
}

fn is_cortex_m_peripheral(name: &str) -> bool {
    CORTEX_M_PERIPHERALS
        .iter()
        .any(|core_name| name.eq_ignore_ascii_case(core_name))
}

fn svd_peripheral_name(metadata_name: &str) -> &str {
    match metadata_name {
        "EDMA_0_TCD" => "EDMA_0_TCD0",
        "FLEX_PWM0" => "FLEXPWM0",
        "FLEX_PWM1" => "FLEXPWM1",
        "EQDC0" => "QDC0",
        "EQDC1" => "QDC1",
        "CDOG0" => "CDOG",
        name => name,
    }
}

fn parse_address(value: &str) -> anyhow::Result<u64> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        Ok(u64::from_str_radix(hex, 16)?)
    } else {
        Ok(value.parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("generator crate has repository parent")
            .to_path_buf()
    }

    fn mcxa156_device(root: &Path) -> Device {
        let temp = TempDir::with_prefix("nxp-pac-mcxa156-test-").expect("create source temp");
        let svd = materialize_locked_mcxa156_svd(root, &temp).expect("materialize locked SVD");
        let xml = fs::read_to_string(svd).expect("read MCXA156 SVD");
        svd_parser::parse_with_config(
            &xml,
            &svd_parser::Config::default()
                .expand(true)
                .expand_properties(true),
        )
        .expect("parse MCXA156 SVD")
    }

    fn mcxa156_metadata(root: &Path) -> Metadata {
        let json = fs::read_to_string(root.join("data/metadata/MCXA156.json"))
            .expect("read MCXA156 metadata");
        serde_json::from_str(&json).expect("parse MCXA156 metadata")
    }

    #[test]
    fn mcxa156_inventory_and_artifacts_match_locked_svd() {
        let coverage =
            validate_mcxa156(&repository_root()).expect("MCXA156 PAC must match locked SVD");
        assert_eq!(
            coverage,
            SourceValidationCoverage {
                peripheral_instances: 68,
                register_instances: 3_477,
            }
        );
    }

    #[test]
    fn rejects_missing_svd_peripheral() {
        let root = repository_root();
        let device = mcxa156_device(&root);
        let mut metadata = mcxa156_metadata(&root);
        metadata
            .peripherals
            .retain(|peripheral| peripheral.name != "GPIO4");

        let error =
            validate_complete_inventory(&device, &metadata, &root.join("data/source-peripherals"))
                .expect_err("missing SVD peripheral must be rejected");
        assert!(format!("{error:#}").contains("GPIO4"));
    }

    #[test]
    fn rejects_peripheral_address_drift() {
        let root = repository_root();
        let device = mcxa156_device(&root);
        let mut metadata = mcxa156_metadata(&root);
        metadata
            .peripherals
            .iter_mut()
            .find(|peripheral| peripheral.name == "GPIO3")
            .expect("GPIO3 metadata")
            .peripheral_address = Some("0x40105004".into());

        let error =
            validate_complete_inventory(&device, &metadata, &root.join("data/source-peripherals"))
                .expect_err("address drift must be rejected");
        assert!(format!("{error:#}").contains("GPIO3"));
    }

    #[test]
    fn rejects_carried_dma_request_drift() {
        let root = repository_root();
        let device = mcxa156_device(&root);
        let mut metadata = mcxa156_metadata(&root);
        metadata
            .peripherals
            .iter_mut()
            .find(|peripheral| peripheral.name == "LPUART0")
            .expect("LPUART0 metadata")
            .dma_muxing[0]
            .request = 20;

        let error =
            validate_complete_inventory(&device, &metadata, &root.join("data/source-peripherals"))
                .expect_err("DMA request drift must be rejected");
        assert!(format!("{error:#}").contains("carried DMA request inventory"));
    }

    #[test]
    fn rejects_incomplete_can_register_inventory() {
        let root = repository_root();
        let device = mcxa156_device(&root);
        let source = device
            .peripherals
            .iter()
            .find(|peripheral| peripheral.name == "CAN0")
            .expect("CAN0 source peripheral");
        let yaml = fs::File::open(root.join("data/source-peripherals/MCXA156/CAN.yaml"))
            .expect("open MCXA156 CAN metadata");
        let mut ir: IR = serde_yaml::from_reader(yaml).expect("parse MCXA156 CAN metadata");
        ir.blocks
            .get_mut("Can")
            .expect("CAN block")
            .items
            .pop()
            .expect("CAN register");
        let block = ir.blocks.get("Can").expect("CAN block");

        let error = validate_expanded_register_offsets(source, &ir, block)
            .expect_err("incomplete CAN register inventory must be rejected");
        assert!(format!("{error:#}").contains("complete locked SVD inventory"));
    }
}
