use std::{collections::BTreeMap, fmt::Write, fs, path::Path};

use anyhow::{Context, anyhow, bail, ensure};
use chiptool::commands::{ExtractShared, extract_all::ExtractAll};
use indexmap::IndexMap;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use serde::Deserialize;

use crate::util::rustfmt;

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub chips: Vec<String>,
    pub pins: Vec<Pin>,
    pub nvic_prio_bits: u32,
    pub interrupts: IndexMap<String, u32>,
    pub peripherals: Vec<Peripheral>,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct Pin {
    pub name: String,

    /// Supply for this pin.
    ///
    /// An example of when this is [`None`] is supply for a VREF pin (the pin is itself a supply).
    pub supply: Option<String>,

    /// IOMUXC information for this pin. Only applicable on RT1xxx chips.
    pub iomuxc: Option<PinIomuxc>,

    /// Rust Feature required to "unlock" the pin.
    ///
    /// An example of this is when a pin is used for SWD communication by default,
    /// and it would be dangerous to unlock unless explicitly designed around it.
    pub feature: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinIomuxc {
    /// Some pins only have a mux, thereby not being usable as GPIO.
    pub mux: Option<String>,

    /// Pins that are usable by IOMUXC require a pad register.
    pub pad: String,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
pub struct Peripheral {
    pub name: String,
    #[serde(rename = "block")]
    pub peripheral_block: Option<String>,
    #[serde(rename = "module")]
    pub rust_module_name: Option<String>,
    #[serde(rename = "address")]
    pub peripheral_address: Option<String>,
    pub signals: Vec<Signal>,
    pub flexcomm: Option<String>,
    #[serde(default)]
    pub dma_muxing: Vec<DmaMux>,
    pub only_in: Option<String>,
    /// Generate the PAC instance but do not advertise a HAL driver mapping.
    #[serde(default)]
    pub pac_only: bool,
    pub gate: Option<Gate>,
}

impl Peripheral {
    pub fn parse_block_path(&self) -> anyhow::Result<Option<BlockPath>> {
        let Some(mut peripheral_block) = self.peripheral_block.as_deref() else {
            return Ok(None);
        };

        let mut type_name = None;

        if let Some((stripped_path, specified_block_name)) = peripheral_block.split_once("::") {
            peripheral_block = stripped_path;
            type_name.get_or_insert(specified_block_name);
        }

        let original_mod_name = peripheral_block
            .split('/')
            .next_back()
            .context("bad type path name")?;

        let type_name = *type_name.get_or_insert(original_mod_name);

        let mod_name = match self.rust_module_name.as_deref() {
            Some(name) => name,
            None => original_mod_name,
        };

        Ok(Some(BlockPath {
            path: peripheral_block.into(),
            rust_mod_name: mod_name.to_lowercase(),
            rust_type_name: inflections::Inflect::to_pascal_case(type_name),
        }))
    }
}

fn runtime_driver_name(peripheral: &Peripheral) -> &str {
    if peripheral.pac_only {
        ""
    } else {
        peripheral.peripheral_block.as_deref().unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Signal {
    pub name: String,
    pub pins: Vec<SignalPin>,

    /// IOMUXC daisy register used for this signal.
    ///
    /// Depending on the peripheral type and instance, this may some be [`None`] even for a
    /// peripheral which usually has a daisy register.
    ///
    /// If this is [`Some`], each pin's [`Signal::iomuxc_daisy`] value must be [`Some`].
    pub iomuxc_daisy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignalPin {
    pub pin: String,
    pub alt: u8,

    /// IOMUXC daisy value to write into the daisy register of the parent [`Signal`].
    ///
    /// This is required if [`Signal::iomuxc_daisy`] is [`Some`]
    pub iomuxc_daisy: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DmaMux {
    pub signal: String,
    pub mux: String,
    pub request: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Gate {
    pub enable: String,
    pub reset: Option<String>,
    pub config: Option<String>,
    #[serde(default)]
    pub bit: Option<String>,
}

fn generate_metadata(name: &str, metadata: &Metadata) -> TokenStream {
    let pins = metadata.pins.iter().map(|pin| {
        let name = &pin.name;
        let iomuxc = pin
            .iomuxc
            .as_ref()
            .map(|iomuxc| {
                let pad = u32::from_str_radix(&iomuxc.pad[2..], 16).unwrap();

                let mux = iomuxc
                    .mux
                    .as_ref()
                    .map(|mux| {
                        let mux = u32::from_str_radix(&mux[2..], 16).unwrap();
                        quote! { Some(#mux) }
                    })
                    .unwrap_or_else(|| quote! { None });

                quote! {
                    Some(PinIomuxc {
                        mux: #mux,
                        pad: #pad,
                    })
                }
            })
            .unwrap_or_else(|| quote! { None });
        let feature = pin
            .feature
            .as_ref()
            .map(|feature| quote! { Some(#feature) })
            .unwrap_or_else(|| quote! { None });

        quote! {
            Pin {
                name: #name,
                iomuxc: #iomuxc,
                feature: #feature
            }
        }
    });

    let peripherals = metadata.peripherals.iter().map(|peripheral| {
        let name = &peripheral.name;
        let flexcomm = peripheral
            .flexcomm
            .as_ref()
            .map(|ref fc| quote! { Some(#fc) })
            .unwrap_or_else(|| quote! { None });

        let signals = peripheral.signals.iter().map(|signal| {
            let name = &signal.name;

            let iomuxc_daisy = signal
                .iomuxc_daisy
                .as_ref()
                .map(|iomuxc| {
                    let daisy = u32::from_str_radix(&iomuxc[2..], 16).unwrap();

                    quote! {
                        Some(#daisy)
                    }
                })
                .unwrap_or_else(|| quote! { None });

            let pins = signal.pins.iter().map(|signal| {
                let pin = &signal.pin;
                let alt = signal.alt;
                let iomuxc_daisy = signal
                    .iomuxc_daisy
                    .as_ref()
                    .map(|daisy| quote! { Some(#daisy) })
                    .unwrap_or_else(|| quote! { None });

                quote! {
                    SignalPin {
                        pin: #pin,
                        alt: #alt,
                        iomuxc_daisy: #iomuxc_daisy,
                    }
                }
            });

            quote! {
                Signal {
                    name: #name,
                    pins: &[#(#pins),*],
                    iomuxc_daisy: #iomuxc_daisy,
                }
            }
        });

        let dma_muxing = peripheral.dma_muxing.iter().map(|dma_mux| {
            let signal = &dma_mux.signal;
            let mux = &dma_mux.mux;
            let request = Literal::u8_unsuffixed(dma_mux.request);

            quote! {
                DmaMux {
                    signal: #signal,
                    mux: #mux,
                    request: #request,
                }
            }
        });

        let address = match peripheral.peripheral_address.as_ref() {
            Some(val) => {
                let val: TokenStream = val
                    .parse()
                    .expect("Peripheral address is parsed to tokenstream");
                quote! { #val }
            }
            None => quote! { 0 },
        };

        let driver_name = runtime_driver_name(peripheral);

        let gate = match peripheral.gate.as_ref() {
            Some(Gate {
                enable,
                reset,
                config,
                bit,
            }) => {
                let reset = match reset {
                    Some(reset) => quote! { Some(#reset) },
                    None => quote! { None },
                };
                let config = match config {
                    Some(config) => quote! { Some(#config) },
                    None => quote! { None },
                };
                let bit = match bit.clone() {
                    Some(bit) => bit,
                    None => name.to_lowercase(),
                };

                quote! {
                    Some(Gate {
                        enable: #enable,
                        reset: #reset,
                        config: #config,
                        bit: #bit,
                    })
                }
            }
            None => quote! { None },
        };

        quote! {
            Peripheral {
                name: #name,
                address: #address,
                driver_name: #driver_name,
                signals: &[#(#signals),*],
                flexcomm: #flexcomm,
                dma_muxing: &[#(#dma_muxing),*],
                gate: #gate,
            }
        }
    });

    let interrupts = metadata
        .interrupts
        .iter()
        .map(|(name, val)| quote! { (#name, #val) });

    quote! {
        use crate::metadata::*;

        pub const METADATA: Metadata = Metadata {
            name: #name,
            pins: PINS,
            peripherals: PERIPHERALS,
            interrupts: INTERRUPTS,
        };

        pub const PINS: &[Pin] = &[#(#pins),*];
        pub const PERIPHERALS: &[Peripheral] = &[#(#peripherals),*];
        pub const INTERRUPTS: &[(&str, u32)] = &[#(#interrupts),*];
    }
}

/// Read the metadata, generate the Rust source files for the metadata file used in build.rs and return the metadata.
pub fn generate(chips_dir: &Path, metadata: &Path, core: &str) -> anyhow::Result<Metadata> {
    let metadata = fs::read_to_string(metadata).context("Read metadata")?;
    let metadata = serde_json::from_str::<Metadata>(&metadata).context("Deserialize metadata")?;

    let mut metadata_out = String::new();
    write!(metadata_out, "{}", generate_metadata(core, &metadata))?;

    let metadata_rs = chips_dir.join(core.to_lowercase()).join("metadata.rs");
    if !metadata_rs
        .parent()
        .context("getting metadata.rs parent")?
        .exists()
    {
        fs::create_dir_all(metadata_rs.parent().context("getting metadata.rs parent")?)?;
    }
    fs::write(&metadata_rs, metadata_out)?;
    rustfmt(&metadata_rs).context("Formatting metadata")?;

    Ok(metadata)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BlockPath {
    pub path: String,
    pub rust_mod_name: String,
    pub rust_type_name: String,
}

/// Extract peripheral metadata definitions from a SVD and puts them in a .gitignored raw folder.
pub fn extract_peripherals(
    svd: &Path,
    core: &str,
    transforms_dir: Option<&Path>,
    output_dir: &Path,
) -> Result<(), anyhow::Error> {
    use std::fmt::Write;

    let transform_path =
        transforms_dir.map(|path| path.join(core.to_lowercase()).with_extension("yaml"));

    let transform = if let Some(transform_path) = transform_path {
        if !fs::exists(&transform_path).context("checking transform existance")? {
            bail!(
                "transform {} for core \"{}\" does not exist?",
                transform_path.display(),
                core.to_lowercase()
            );
        }
        vec![transform_path.canonicalize()?]
    } else {
        vec![]
    };

    if !fs::exists(output_dir).context("checking output directory existance")? {
        fs::create_dir(output_dir)
            .with_context(|| format!("creating output directory {}", output_dir.display()))?;
    }

    for entry in fs::read_dir(output_dir).context("reading raw peripherals dir")? {
        let entry = entry?;
        if entry.file_name().to_string_lossy() != ".gitignore" {
            if entry.file_type()?.is_dir() {
                fs::remove_dir_all(entry.path())
                    .with_context(|| format!("removing {}", entry.path().display()))?;
            } else {
                fs::remove_file(entry.path())
                    .with_context(|| format!("removing {}", entry.path().display()))?;
            }
        }
    }

    let post_transforms_dir = &output_dir.join("post-transforms");

    if !fs::exists(post_transforms_dir)
        .context("checking output post-transforms subdirectory existance")?
    {
        fs::create_dir(post_transforms_dir).with_context(|| {
            format!(
                "creating post-transforms subdirectory {}",
                post_transforms_dir.display()
            )
        })?;
    }

    chiptool::commands::extract_all::extract_all(ExtractAll {
        output: post_transforms_dir.canonicalize()?,
        extract_shared: ExtractShared {
            svd: svd.canonicalize()?,
            transform,
            namespaces: chiptool::svd2ir::NamespaceMode::Block,
        },
        mode: chiptool::commands::extract_all::ExtractionMode::Block,
    })
    .with_context(|| format!("Error generating peripheral yamls for {core}"))?;

    let path_regex = regex::Regex::new("^(.+)__.+$")?;
    let mut entries = fs::read_dir(post_transforms_dir)
        .context("reading post-transforms subdir")?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    // chiptool emits one IR fragment per block. Multiple blocks may share a
    // namespace, so combine them before stripping that namespace. Writing each
    // fragment directly would silently leave only the last block.
    let mut modules = BTreeMap::<String, chiptool::ir::IR>::new();
    for entry in entries {
        let filename: String = entry.file_name().to_string_lossy().into_owned();
        let name = path_regex
            .captures(&filename)
            .and_then(|c| c.get(1))
            .ok_or_else(|| anyhow!("Failed strip namespace from filename {:?}", &entry))?;

        let ir: chiptool::ir::IR = serde_yaml::from_reader(fs::File::open(entry.path())?)?;
        merge_ir_fragment(
            modules.entry(name.as_str().to_uppercase()).or_default(),
            ir,
            name.as_str(),
        )?;
    }

    let from: chiptool::transform::common::RegexSet = serde_yaml::from_str("(.*)::(.+)")?;
    for (name, mut ir) in modules {
        chiptool::transform::rename::Rename {
            from: from.clone(),
            to: "$2".to_string(),
            r#type: chiptool::transform::rename::RenameType::All,
        }
        .run(&mut ir)?;

        let data = serde_yaml::to_string(&ir)?;
        fs::write(output_dir.join(format!("{name}.yaml")), data.as_bytes())?;
    }

    let svd_contents = fs::read_to_string(svd).context("Read SVD")?;
    let svd = svd_parser::parse(&svd_contents).context("Parse SVD")?;

    let nvic_priority_bits = svd
        .cpu
        .map(|cpu| cpu.nvic_priority_bits)
        .unwrap_or_default();

    let mut interrupts = Vec::new();

    for peripheral in svd.peripherals.iter() {
        for interrupt in peripheral.interrupt.iter() {
            // Rust uses fully capitalized interrupt names for singletons.
            interrupts.push((interrupt.name.clone().to_uppercase(), interrupt.value));
        }
    }

    interrupts.sort_unstable_by_key(|(_, val)| *val);
    interrupts.dedup();

    let mut interrupts_json = String::new();
    writeln!(
        &mut interrupts_json,
        "{{\n  \"nvic_prio_bits\": {nvic_priority_bits},\n  \"interrupts\": {{"
    )?;
    for (i, (name, num)) in interrupts.iter().enumerate() {
        writeln!(
            &mut interrupts_json,
            "    \"{name}\": {num}{}",
            if i != interrupts.len() - 1 { "," } else { "" }
        )?;
    }
    writeln!(&mut interrupts_json, "  }}\n}}")?;

    fs::write(
        output_dir.join("_interrupts.json"),
        interrupts_json.as_bytes(),
    )
    .context("writing _interrupts.json")?;

    let peripheral_addresses = svd
        .peripherals
        .iter()
        .map(|p| (&p.name, p.base_address))
        .collect::<Vec<_>>();
    let mut addresses_json = String::new();
    writeln!(&mut addresses_json, "{{")?;
    for (i, (name, address)) in peripheral_addresses.iter().enumerate() {
        writeln!(
            &mut addresses_json,
            "  \"{name}\": \"{address:#010X}\"{}",
            if i != peripheral_addresses.len() - 1 {
                ","
            } else {
                ""
            }
        )?;
    }
    writeln!(&mut addresses_json, "}}")?;
    fs::write(
        output_dir.join("_addresses.json"),
        addresses_json.as_bytes(),
    )
    .context("writing _addresses.json")?;

    Ok(())
}

fn merge_ir_fragment(
    target: &mut chiptool::ir::IR,
    fragment: chiptool::ir::IR,
    namespace: &str,
) -> anyhow::Result<()> {
    merge_ir_map(&mut target.devices, fragment.devices, namespace, "device")?;
    merge_ir_map(&mut target.blocks, fragment.blocks, namespace, "block")?;
    merge_ir_map(
        &mut target.fieldsets,
        fragment.fieldsets,
        namespace,
        "fieldset",
    )?;
    merge_ir_map(&mut target.enums, fragment.enums, namespace, "enum")?;
    Ok(())
}

fn merge_ir_map<T: PartialEq>(
    target: &mut BTreeMap<String, T>,
    fragment: BTreeMap<String, T>,
    namespace: &str,
    kind: &str,
) -> anyhow::Result<()> {
    for (name, value) in fragment {
        if let Some(existing) = target.get(&name) {
            ensure!(
                existing == &value,
                "conflicting {kind} {name} while combining extracted namespace {namespace}"
            );
        } else {
            target.insert(name, value);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chiptool::ir::{Block, FieldSet, IR};

    use super::{Peripheral, merge_ir_fragment, runtime_driver_name};

    fn block(description: &str) -> Block {
        Block {
            extends: None,
            description: Some(description.into()),
            items: Vec::new(),
        }
    }

    fn fieldset(description: &str) -> FieldSet {
        FieldSet {
            extends: None,
            description: Some(description.into()),
            bit_size: 32,
            fields: Vec::new(),
        }
    }

    fn peripheral(pac_only: bool) -> Peripheral {
        Peripheral {
            name: "GPIO0".into(),
            peripheral_block: Some("mcxa/GPIO".into()),
            rust_module_name: None,
            peripheral_address: Some("0x4000_0000".into()),
            signals: Vec::new(),
            flexcomm: None,
            dma_muxing: Vec::new(),
            only_in: None,
            pac_only,
            gate: None,
        }
    }

    #[test]
    fn pac_only_uses_existing_empty_driver_name_convention() {
        assert_eq!(runtime_driver_name(&peripheral(true)), "");
        assert_eq!(runtime_driver_name(&peripheral(false)), "mcxa/GPIO");
    }

    #[test]
    fn namespace_fragment_merge_deduplicates_equal_types_and_keeps_distinct_blocks() {
        let mut target = IR::new();
        target.blocks.insert("port::Port".into(), block("PORT"));
        target.fieldsets.insert("port::Pcr".into(), fieldset("PCR"));
        let mut fragment = IR::new();
        fragment.blocks.insert("port::Port1".into(), block("PORT"));
        fragment
            .fieldsets
            .insert("port::Pcr".into(), fieldset("PCR"));

        merge_ir_fragment(&mut target, fragment, "port").expect("equal shared types merge");
        assert!(target.blocks.contains_key("port::Port"));
        assert!(target.blocks.contains_key("port::Port1"));
    }

    #[test]
    fn namespace_fragment_merge_rejects_conflicting_duplicate_types() {
        let mut target = IR::new();
        target.blocks.insert("port::Port".into(), block("first"));
        let mut fragment = IR::new();
        fragment.blocks.insert("port::Port".into(), block("second"));

        let error = merge_ir_fragment(&mut target, fragment, "port")
            .expect_err("conflicting duplicate must fail closed");
        assert!(format!("{error:#}").contains("conflicting block port::Port"));
    }
}
