//! Fail-closed checks that bind the MCXA156 MetaPAC to its locked SVD.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};

use anyhow::{Context, anyhow, ensure};
use chiptool::ir::{Array, Block, BlockItemInner, IR};
use regex::Regex;
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
    svd: LockedGitInput,
    dma_header: LockedGitInput,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LockedGitInput {
    repository_path: String,
    repository: String,
    revision: String,
    path: String,
    sha256: String,
    license_expression: String,
    copyright_notices: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InterruptArtifact {
    nvic_prio_bits: u32,
    interrupts: BTreeMap<String, u32>,
}
// MCXA156 does not publish request-source enumerations in its SVD. These are
// the exact entries carried from its manifest-pinned PERI_DMA.h; the
// reconciliation evidence records that source and hash explicitly.
const CARRIED_DMA_REQUESTS: &[(&str, &str, &str, u8)] = &[
    ("kDma0RequestWUU0", "WUU0", "WUU0WakeUpEvent", 1),
    ("kDma0RequestMuxFlexCan0DmaRequest", "CAN0", "CAN0", 2),
    ("kDma0RequestLPI2C2Rx", "LPI2C2", "LPI2C2Rx", 3),
    ("kDma0RequestLPI2C2Tx", "LPI2C2", "LPI2C2Tx", 4),
    ("kDma0RequestLPI2C3Rx", "LPI2C3", "LPI2C3Rx", 5),
    ("kDma0RequestLPI2C3Tx", "LPI2C3", "LPI2C3Tx", 6),
    ("kDma0RequestMuxI3c0Rx", "I3C0", "I3C0Rx", 7),
    ("kDma0RequestMuxI3c0Tx", "I3C0", "I3C0Tx", 8),
    ("kDma0RequestLPI2C0Rx", "LPI2C0", "LPI2C0Rx", 11),
    ("kDma0RequestLPI2C0Tx", "LPI2C0", "LPI2C0Tx", 12),
    ("kDma0RequestLPI2C1Rx", "LPI2C1", "LPI2C1Rx", 13),
    ("kDma0RequestLPI2C1Tx", "LPI2C1", "LPI2C1Tx", 14),
    ("kDma0RequestLPSPI0Rx", "LPSPI0", "LPSPI0Rx", 15),
    ("kDma0RequestLPSPI0Tx", "LPSPI0", "LPSPI0Tx", 16),
    ("kDma0RequestLPSPI1Rx", "LPSPI1", "LPSPI1Rx", 17),
    ("kDma0RequestLPSPI1Tx", "LPSPI1", "LPSPI1Tx", 18),
    ("kDma0RequestLPUART0Rx", "LPUART0", "LPUART0Rx", 21),
    ("kDma0RequestLPUART0Tx", "LPUART0", "LPUART0Tx", 22),
    ("kDma0RequestLPUART1Rx", "LPUART1", "LPUART1Rx", 23),
    ("kDma0RequestLPUART1Tx", "LPUART1", "LPUART1Tx", 24),
    ("kDma0RequestLPUART2Rx", "LPUART2", "LPUART2Rx", 25),
    ("kDma0RequestLPUART2Tx", "LPUART2", "LPUART2Tx", 26),
    ("kDma0RequestLPUART3Rx", "LPUART3", "LPUART3Rx", 27),
    ("kDma0RequestLPUART3Tx", "LPUART3", "LPUART3Tx", 28),
    ("kDma0RequestLPUART4Rx", "LPUART4", "LPUART4Rx", 29),
    ("kDma0RequestLPUART4Tx", "LPUART4", "LPUART4Tx", 30),
    ("kDma0RequestMuxCtimer0M0", "CTIMER0", "CTIMER0M0", 31),
    ("kDma0RequestMuxCtimer0M1", "CTIMER0", "CTIMER0M1", 32),
    ("kDma0RequestMuxCtimer1M0", "CTIMER1", "CTIMER1M0", 33),
    ("kDma0RequestMuxCtimer1M1", "CTIMER1", "CTIMER1M1", 34),
    ("kDma0RequestMuxCtimer2M0", "CTIMER2", "CTIMER2M0", 35),
    ("kDma0RequestMuxCtimer2M1", "CTIMER2", "CTIMER2M1", 36),
    ("kDma0RequestMuxCtimer3M0", "CTIMER3", "CTIMER3M0", 37),
    ("kDma0RequestMuxCtimer3M1", "CTIMER3", "CTIMER3M1", 38),
    ("kDma0RequestMuxCtimer4M0", "CTIMER4", "CTIMER4M0", 39),
    ("kDma0RequestMuxCtimer4M1", "CTIMER4", "CTIMER4M1", 40),
    (
        "kDma0RequestMuxFlexPWM0ReqCapt0",
        "FLEX_PWM0",
        "FlexPWM0Mcapt0",
        41,
    ),
    (
        "kDma0RequestMuxFlexPWM0ReqCapt1",
        "FLEX_PWM0",
        "FlexPWM0Mcapt1",
        42,
    ),
    (
        "kDma0RequestMuxFlexPWM0ReqCapt2",
        "FLEX_PWM0",
        "FlexPWM0Mcapt2",
        43,
    ),
    (
        "kDma0RequestMuxFlexPWM0ReqVal0",
        "FLEX_PWM0",
        "FlexPWM0Mval0",
        45,
    ),
    (
        "kDma0RequestMuxFlexPWM0ReqVal1",
        "FLEX_PWM0",
        "FlexPWM0Mval1",
        46,
    ),
    (
        "kDma0RequestMuxFlexPWM0ReqVal2",
        "FLEX_PWM0",
        "FlexPWM0Mval2",
        47,
    ),
    (
        "kDma0RequestMuxLptmr0",
        "LPTMR0",
        "LPTMR0CounterMatchEvent",
        49,
    ),
    (
        "kDma0RequestMuxAdc0FifoRequest",
        "ADC0",
        "ADC0FifoRequest",
        51,
    ),
    (
        "kDma0RequestMuxAdc1FifoRequest",
        "ADC1",
        "ADC1FifoRequest",
        52,
    ),
    (
        "kDma0RequestMuxHsCmp0DmaRequest",
        "CMP0",
        "CMP0DmaRequest",
        53,
    ),
    (
        "kDma0RequestMuxHsCmp1DmaRequest",
        "CMP1",
        "CMP1DmaRequest",
        54,
    ),
    (
        "kDma0RequestMuxDac0FifoRequest",
        "DAC0",
        "DAC0FifoRequest",
        56,
    ),
    (
        "kDma0RequestMuxGpio0PinEventRequest0",
        "GPIO0",
        "GPIO0PinEvent0",
        60,
    ),
    (
        "kDma0RequestMuxGpio1PinEventRequest0",
        "GPIO1",
        "GPIO1PinEvent0",
        61,
    ),
    (
        "kDma0RequestMuxGpio2PinEventRequest0",
        "GPIO2",
        "GPIO2PinEvent0",
        62,
    ),
    (
        "kDma0RequestMuxGpio3PinEventRequest0",
        "GPIO3",
        "GPIO3PinEvent0",
        63,
    ),
    (
        "kDma0RequestMuxGpio4PinEventRequest0",
        "GPIO4",
        "GPIO4PinEvent0",
        64,
    ),
    ("kDma0RequestMuxQdc0", "EQDC0", "BUFFER", 65),
    ("kDma0RequestMuxQdc1", "EQDC1", "BUFFER", 66),
    (
        "kDma0RequestMuxFlexIO0ShiftRegister0Request",
        "FLEXIO0",
        "FLEXIO0SR0",
        71,
    ),
    (
        "kDma0RequestMuxFlexIO0ShiftRegister1Request",
        "FLEXIO0",
        "FLEXIO0SR1",
        72,
    ),
    (
        "kDma0RequestMuxFlexIO0ShiftRegister2Request",
        "FLEXIO0",
        "FLEXIO0SR2",
        73,
    ),
    (
        "kDma0RequestMuxFlexIO0ShiftRegister3Request",
        "FLEXIO0",
        "FLEXIO0SR3",
        74,
    ),
    (
        "kDma0RequestMuxFlexPWM1ReqCapt0",
        "FLEX_PWM1",
        "FlexPWM1Mcapt0",
        79,
    ),
    (
        "kDma0RequestMuxFlexPWM1ReqCapt1",
        "FLEX_PWM1",
        "FlexPWM1Mcapt1",
        80,
    ),
    (
        "kDma0RequestMuxFlexPWM1ReqCapt2",
        "FLEX_PWM1",
        "FlexPWM1Mcapt2",
        81,
    ),
    (
        "kDma0RequestMuxFlexPWM1ReqVal0",
        "FLEX_PWM1",
        "FlexPWM1Mval0",
        83,
    ),
    (
        "kDma0RequestMuxFlexPWM1ReqVal1",
        "FLEX_PWM1",
        "FlexPWM1Mval1",
        84,
    ),
    (
        "kDma0RequestMuxFlexPWM1ReqVal2",
        "FLEX_PWM1",
        "FlexPWM1Mval2",
        85,
    ),
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
    let lock = read_mcxa156_source_lock(current)?;
    validate_retained_license(current, &lock)?;
    let svd_path = materialize_locked_input(current, &lock.svd, &source, "MCXA156.xml")?;
    let dma_header_path =
        materialize_locked_input(current, &lock.dma_header, &source, "PERI_DMA.h")?;
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
    let dma_header = fs::read_to_string(&dma_header_path)
        .with_context(|| format!("reading locked DMA header {}", dma_header_path.display()))?;

    let coverage = validate_complete_inventory(&device, &metadata, &peripheral_dir)?;
    validate_dma_requests(&metadata, &dma_header)?;
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
    let lock = read_mcxa156_source_lock(current)?;
    let svd_path = materialize_locked_input(current, &lock.svd, &source, "MCXA156.xml")?;
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

fn read_mcxa156_source_lock(current: &Path) -> anyhow::Result<SourceLock> {
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
    Ok(lock)
}

fn materialize_locked_input(
    current: &Path,
    input: &LockedGitInput,
    temp: &TempDir,
    output_name: &str,
) -> anyhow::Result<std::path::PathBuf> {
    ensure!(
        input.revision.len() == 40 && input.revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "MCXA156 source lock revision must be a full Git object ID"
    );
    ensure!(
        input.sha256.len() == 64 && input.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "MCXA156 source lock SHA-256 must be a full digest"
    );
    ensure!(
        input.license_expression == "BSD-3-Clause",
        "MCXA156 source input is not licensed BSD-3-Clause"
    );
    ensure!(
        !input.copyright_notices.is_empty()
            && input
                .copyright_notices
                .iter()
                .all(|notice| !notice.trim().is_empty()),
        "MCXA156 source input omits its copyright notice"
    );

    let repository = current.join(&input.repository_path);
    let remote = Command::new("git")
        .arg("-C")
        .arg(&repository)
        .args(["remote", "get-url", "origin"])
        .output()
        .context("reading locked MCXA156 input repository origin")?;
    ensure!(
        remote.status.success(),
        "locked MCXA156 input repository has no origin remote: {}",
        String::from_utf8_lossy(&remote.stderr).trim()
    );
    let actual_repository = String::from_utf8(remote.stdout)
        .context("locked MCXA156 input repository URL is not UTF-8")?;
    ensure!(
        normalize_repository(&actual_repository) == normalize_repository(&input.repository),
        "locked MCXA156 input repository differs: expected {}, got {}",
        input.repository,
        actual_repository.trim()
    );
    let object = format!("{}:{}", input.revision, input.path);
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
        input.repository,
        input.revision
    );

    let actual_sha256 = format!("{:x}", Sha256::digest(&output.stdout));
    ensure!(
        actual_sha256 == input.sha256,
        "locked MCXA156 input SHA-256 mismatch: expected {}, got {}",
        input.sha256,
        actual_sha256
    );

    let output_path = temp.child(output_name);
    fs::write(&output_path, output.stdout).context("writing temporary locked MCXA156 input")?;
    Ok(output_path)
}

fn normalize_repository(value: &str) -> &str {
    value.trim().trim_end_matches(".git")
}

fn validate_retained_license(current: &Path, lock: &SourceLock) -> anyhow::Result<()> {
    let notice_path = current.join("data/source-peripherals/NOTICE-MCXA156");
    let license_path = current.join("data/source-peripherals/LICENSE-BSD-3-Clause");
    let notice = fs::read_to_string(&notice_path)
        .with_context(|| format!("reading retained notice {}", notice_path.display()))?;
    let license = fs::read_to_string(&license_path)
        .with_context(|| format!("reading retained license {}", license_path.display()))?;

    for input in [&lock.svd, &lock.dma_header] {
        ensure!(
            input.license_expression == "BSD-3-Clause",
            "MCXA156 retained source is not BSD-3-Clause"
        );
        for copyright in &input.copyright_notices {
            ensure!(
                notice.contains(copyright),
                "MCXA156 retained notice omits {copyright:?}"
            );
        }
    }
    ensure!(
        notice.contains("SPDX-License-Identifier: BSD-3-Clause")
            && license.contains("Redistribution and use in source and binary forms")
            && license.contains("THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS"),
        "MCXA156 BSD-3-Clause notice or terms are incomplete"
    );
    Ok(())
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

    validate_interrupts(metadata, peripheral_dir)?;

    Ok(SourceValidationCoverage {
        peripheral_instances: metadata_names.len(),
        register_instances,
    })
}

fn validate_interrupts(metadata: &Metadata, peripheral_dir: &Path) -> anyhow::Result<()> {
    let artifact_path = peripheral_dir.join("MCXA156/_interrupts.json");
    let artifact_contents = fs::read_to_string(&artifact_path)
        .with_context(|| format!("reading interrupt artifact {}", artifact_path.display()))?;
    let artifact: InterruptArtifact = serde_json::from_str(&artifact_contents)
        .with_context(|| format!("parsing interrupt artifact {}", artifact_path.display()))?;
    let metadata_interrupts = metadata
        .interrupts
        .iter()
        .map(|(name, number)| (name.clone(), *number))
        .collect::<BTreeMap<_, _>>();
    ensure!(
        metadata.nvic_prio_bits == artifact.nvic_prio_bits,
        "MCXA156 NVIC priority bits differ from the locked-SVD artifact: metadata {}, source {}",
        metadata.nvic_prio_bits,
        artifact.nvic_prio_bits
    );
    ensure!(
        metadata_interrupts == artifact.interrupts,
        "MCXA156 interrupt inventory differs from the locked-SVD artifact"
    );
    Ok(())
}

fn validate_dma_requests(metadata: &Metadata, header: &str) -> anyhow::Result<()> {
    let entry = Regex::new(r"(?m)^\s*(kDma0Request[A-Za-z0-9_]+)\s*=\s*([0-9]+)U,")?;
    let mut header_requests = BTreeMap::new();
    for captures in entry.captures_iter(header) {
        let name = captures.get(1).expect("DMA name capture").as_str();
        if name == "kDma0RequestDisabled" {
            continue;
        }
        let request = captures
            .get(2)
            .expect("DMA request capture")
            .as_str()
            .parse::<u8>()
            .with_context(|| format!("parsing DMA request {name}"))?;
        ensure!(
            header_requests.insert(name, request).is_none(),
            "locked PERI_DMA.h contains duplicate request {name}"
        );
    }

    let expected_header_requests = CARRIED_DMA_REQUESTS
        .iter()
        .map(|(source_name, _, _, request)| (*source_name, *request))
        .collect::<BTreeMap<_, _>>();
    ensure!(
        header_requests == expected_header_requests,
        "locked PERI_DMA.h request inventory differs; expected: {expected_header_requests:?}; actual: {header_requests:?}"
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
        .map(|(_, peripheral, signal, request)| (*peripheral, *signal, "DMA3", *request))
        .collect::<BTreeSet<_>>();
    ensure!(
        dma_requests == expected_dma_requests,
        "MCXA156 DMA request inventory differs from locked PERI_DMA.h mapping; expected: {expected_dma_requests:?}; actual: {dma_requests:?}"
    );
    Ok(())
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
    validate_expanded_register_offsets(svd_peripheral, &ir, block)?;
    Ok(())
}

fn validate_expanded_register_offsets(
    peripheral: &Peripheral,
    ir: &IR,
    block: &Block,
) -> anyhow::Result<()> {
    let source_registers = peripheral
        .all_registers()
        .map(|register| {
            (
                register.address_offset,
                register.properties.size.unwrap_or(32),
            )
        })
        .fold(BTreeMap::new(), |mut counts, register| {
            *counts.entry(register).or_insert(0usize) += 1;
            counts
        });

    let mut generated_registers = BTreeMap::new();
    collect_registers(ir, block, 0, &mut generated_registers)?;
    let generated_offsets = generated_registers
        .keys()
        .map(|(offset, _)| *offset)
        .collect::<BTreeSet<_>>();
    let source_offsets = source_registers
        .keys()
        .map(|(offset, _)| *offset)
        .collect::<BTreeSet<_>>();
    let missing = source_registers
        .iter()
        .filter_map(|(register, source_count)| {
            let generated_count = generated_registers.get(register).copied().unwrap_or(0);
            (generated_count < *source_count).then_some((*register, *source_count, generated_count))
        })
        .collect::<Vec<_>>();
    let extra_offsets = generated_offsets
        .difference(&source_offsets)
        .copied()
        .collect::<Vec<_>>();
    ensure!(
        missing.is_empty() && extra_offsets.is_empty(),
        "MCXA156 {} generated registers do not cover every locked-SVD address/width pair; missing pairs: {missing:?}; extra offsets: {extra_offsets:?}; source count: {}; generated count: {}",
        peripheral.name,
        source_registers.values().sum::<usize>(),
        generated_registers.values().sum::<usize>()
    );
    Ok(())
}

fn collect_registers(
    ir: &IR,
    block: &Block,
    base_offset: u32,
    registers: &mut BTreeMap<(u32, u32), usize>,
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
                BlockItemInner::Register(register) => {
                    *registers
                        .entry((item_offset, register.bit_size))
                        .or_insert(0) += 1;
                }
                BlockItemInner::Block(nested) => {
                    let nested_block = ir.blocks.get(&nested.block).ok_or_else(|| {
                        anyhow!(
                            "generated block references missing nested block {}",
                            nested.block
                        )
                    })?;
                    collect_registers(ir, nested_block, item_offset, registers)?;
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
        let lock = read_mcxa156_source_lock(root).expect("read source lock");
        let svd = materialize_locked_input(root, &lock.svd, &temp, "MCXA156.xml")
            .expect("materialize locked SVD");
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

    fn mcxa156_dma_header(root: &Path) -> String {
        let temp =
            TempDir::with_prefix("nxp-pac-mcxa156-dma-test-").expect("create DMA source temp");
        let lock = read_mcxa156_source_lock(root).expect("read source lock");
        let header = materialize_locked_input(root, &lock.dma_header, &temp, "PERI_DMA.h")
            .expect("materialize locked DMA header");
        fs::read_to_string(header).expect("read locked DMA header")
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
        let mut metadata = mcxa156_metadata(&root);
        metadata
            .peripherals
            .iter_mut()
            .find(|peripheral| peripheral.name == "LPUART0")
            .expect("LPUART0 metadata")
            .dma_muxing[0]
            .request = 20;

        let error = validate_dma_requests(&metadata, &mcxa156_dma_header(&root))
            .expect_err("DMA request drift must be rejected");
        assert!(format!("{error:#}").contains("locked PERI_DMA.h mapping"));
    }

    #[test]
    fn rejects_coordinated_metadata_and_dma_mapping_drift() {
        let root = repository_root();
        let mut metadata = mcxa156_metadata(&root);
        metadata
            .peripherals
            .iter_mut()
            .find(|peripheral| peripheral.name == "LPUART0")
            .expect("LPUART0 metadata")
            .dma_muxing[0]
            .request = 20;
        let header = mcxa156_dma_header(&root).replace(
            "kDma0RequestLPUART0Rx           = 21U",
            "kDma0RequestLPUART0Rx           = 20U",
        );

        let error = validate_dma_requests(&metadata, &header)
            .expect_err("coordinated mapping drift must be rejected by the locked header");
        assert!(format!("{error:#}").contains("PERI_DMA.h request inventory differs"));
    }

    #[test]
    fn rejects_missing_retained_copyright_notice() {
        let root = repository_root();
        let mut lock = read_mcxa156_source_lock(&root).expect("read source lock");
        lock.svd
            .copyright_notices
            .push("Copyright deliberately absent".into());

        let error = validate_retained_license(&root, &lock)
            .expect_err("missing retained copyright must be rejected");
        assert!(format!("{error:#}").contains("retained notice omits"));
    }

    #[test]
    fn rejects_source_repository_drift() {
        let root = repository_root();
        let temp =
            TempDir::with_prefix("nxp-pac-mcxa156-repository-test-").expect("create source temp");
        let mut lock = read_mcxa156_source_lock(&root).expect("read source lock");
        lock.svd.repository = "https://example.invalid/unrelated".into();

        let error = materialize_locked_input(&root, &lock.svd, &temp, "MCXA156.xml")
            .expect_err("repository drift must be rejected");
        assert!(format!("{error:#}").contains("input repository differs"));
    }

    #[test]
    fn rejects_interrupt_and_priority_drift() {
        let root = repository_root();
        let mut interrupt_metadata = mcxa156_metadata(&root);
        *interrupt_metadata
            .interrupts
            .get_mut("LPUART0")
            .expect("LPUART0 interrupt") = 32;
        let interrupt_error =
            validate_interrupts(&interrupt_metadata, &root.join("data/source-peripherals"))
                .expect_err("interrupt drift must be rejected");
        assert!(format!("{interrupt_error:#}").contains("interrupt inventory differs"));

        let mut priority_metadata = mcxa156_metadata(&root);
        priority_metadata.nvic_prio_bits = 4;
        let priority_error =
            validate_interrupts(&priority_metadata, &root.join("data/source-peripherals"))
                .expect_err("priority drift must be rejected");
        assert!(format!("{priority_error:#}").contains("NVIC priority bits differ"));
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
        assert!(format!("{error:#}").contains("do not cover every locked-SVD"));
    }

    #[test]
    fn rejects_missing_full_width_register_view() {
        let root = repository_root();
        let device = mcxa156_device(&root);
        let source = device
            .peripherals
            .iter()
            .find(|peripheral| peripheral.name == "CRC0")
            .expect("CRC0 source peripheral");
        let yaml = fs::File::open(root.join("data/source-peripherals/MCXA156/CRC.yaml"))
            .expect("open MCXA156 CRC metadata");
        let mut ir: IR = serde_yaml::from_reader(yaml).expect("parse MCXA156 CRC metadata");
        ir.blocks
            .get_mut("Crc")
            .expect("CRC block")
            .items
            .retain(|item| item.name != "data32");
        let block = ir.blocks.get("Crc").expect("CRC block");

        let error = validate_expanded_register_offsets(source, &ir, block)
            .expect_err("a narrow alias must not hide a missing full-width source register");
        assert!(format!("{error:#}").contains("missing pairs"));
    }
}
