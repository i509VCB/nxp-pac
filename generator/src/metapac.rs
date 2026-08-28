use std::{collections::HashSet, fs, path::Path, str::FromStr};

use anyhow::Context;
use chiptool::commands::{GenerateShared, ModulePath, gen_block::GenBlock, gen_common::GenCommon};
use indexmap::IndexSet;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::metadata::{BlockPath, Metadata};

/// Take all yamls and export them to the pac after being transformed to Rust code using chiptool
pub fn generate_meta_peripherals(current: &Path) -> anyhow::Result<()> {
    let pac_peri_dir = current.join("nxp-pac/src/meta_peripherals");
    let yaml_peri_dir = current.join("data/metadata/peripherals");

    if !pac_peri_dir.exists() {
        fs::create_dir_all(&pac_peri_dir)?;
    }

    for file in read_dir_all::read_dir_all(&pac_peri_dir)? {
        let file = file?;
        // Erasing is for niceness and is not required. If it doesn't work that's fine
        let _ = fs::remove_file(file.path());
    }

    let drivers = read_dir_all::read_dir_all(&yaml_peri_dir)?
        .par_bridge()
        .map(|entry| {
            let entry = entry?;
            let entry_path = entry.path().to_path_buf();

            let relative_entry_path = entry_path.strip_prefix(&yaml_peri_dir)?;
            if relative_entry_path.starts_with("raw")
                || relative_entry_path
                    .components()
                    .any(|component| component.as_os_str() == "post-transforms")
                || entry_path.is_dir()
                || entry_path.extension().map(|e| e.to_string_lossy()) != Some("yaml".into())
            {
                return Ok(None);
            }

            tracing::info!("{}", relative_entry_path.display());
            let driver_name = {
                let mut path = relative_entry_path.to_path_buf();
                path.set_extension(""); // Remove extension
                path.to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR_STR, "/") // Use consistent separators
            };

            let mut output_path = pac_peri_dir.join(relative_entry_path);
            output_path.set_extension("rs");
            fs::create_dir_all(
                output_path
                    .parent()
                    .context("Getting parent of output path")?,
            )?;

            chiptool::commands::gen_block::gen_block(GenBlock {
                input: entry
                    .path()
                    .canonicalize()
                    .context("Canonicalizing entry path")?,
                output: output_path.clone(),
                gen_shared: GenerateShared {
                    common_module: Some(ModulePath::from_str("crate::pac::common")?),
                    defmt_feature: "defmt".to_string(),
                    no_defmt: false,
                    yes_defmt: false,
                    skip_no_std: true,
                },
            })
            .with_context(|| format!("Error generating block {}", relative_entry_path.display()))?;

            crate::util::rustfmt(&output_path).with_context(|| {
                format!("Error formatting block {}", relative_entry_path.display())
            })?;

            Ok(Some(driver_name))
        })
        .filter_map(|x| x.transpose())
        .collect::<Result<Vec<String>, anyhow::Error>>()?;

    generate_meta_peripherals_mod(current, drivers)?;

    Ok(())
}

/// Generate a file in meta_peripherals listing all peripherals.
///
/// Lists all peripherals regardless of whether it is in use,
/// used for declaring `#cfg(driver)` attributes.
fn generate_meta_peripherals_mod(current: &Path, mut drivers: Vec<String>) -> anyhow::Result<()> {
    use std::fmt::Write;

    // Ensure a consistent ordering, regardless of directory iteration order.
    drivers.sort();

    let mut contents = String::new();

    writeln!(
        &mut contents,
        "{}",
        quote! {
            /// List of all nxp-pac peripherals, whether they are used or not.
            pub const META_PERIPHERALS: &[&str] = &[#(#drivers),*];
        }
    )?;

    let path = current.join("nxp-pac/src/meta_peripherals/_peripherals.rs");
    fs::write(&path, contents.as_bytes()).context("writing contents to file")?;
    crate::util::rustfmt(&path)?;

    Ok(())
}

/// Generates all core-specific code for a metapac PAC, including any peripherals that have been mapped.
pub fn generate_core(current: &Path, core: &str, mut metadata: Metadata) -> anyhow::Result<()> {
    let core_dir = current.join("nxp-pac/src/chips").join(core.to_lowercase());
    if core_dir.exists() {
        fs::remove_dir_all(&core_dir).context("removing old chip dir")?;
    }
    fs::create_dir_all(&core_dir).context("creating chip dir")?;

    // Remove all peripherals that are defined, but don't have a metaperipheral mapped.
    let yaml_peri_dir = current.join("data/metadata/peripherals");
    metadata.peripherals.retain(|p| {
        let peripheral_block = match p.parse_block_path() {
            Ok(Some(peripheral_block)) => peripheral_block,
            Ok(None) => {
                tracing::warn!("Peripheral {} has no block associated. Skipped.", p.name);
                return false;
            }
            Err(e) => {
                tracing::warn!(
                    "Cannot parse peripheral block `{:?}`: {e}. Skipped.",
                    p.peripheral_block
                );
                return false;
            }
        };

        if fs::exists(
            yaml_peri_dir
                .join(&peripheral_block.path)
                .with_extension("yaml"),
        )
        .unwrap_or(false)
        {
            true
        } else {
            tracing::warn!(
                "Peripheral {} does not point to an existing driver: `{}`. Skipped.",
                p.name,
                peripheral_block.path
            );

            false
        }
    });

    export_device_x(&core_dir, &metadata).context("exporting device.x")?;
    export_vectors_rs(&core_dir, &metadata).context("exporting _vectors.rs")?;
    export_common_rs(&core_dir).context("exporting common.rs")?;
    // mod.rs contains vectors.rs and common.rs, so write it last to avoid issues with rustfmt
    export_mod_rs(&core_dir, &metadata).context("exporting mod.rs")?;

    Ok(())
}

fn export_device_x(chip_dir: &Path, metadata: &Metadata) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut contents = String::new();

    for (name, _) in metadata.interrupts.iter() {
        writeln!(&mut contents, "PROVIDE({name} = DefaultHandler);")?;
    }

    fs::write(chip_dir.join("device.x"), contents.as_bytes())
        .context("writing contents to file")?;

    Ok(())
}

fn export_mod_rs(chip_dir: &Path, metadata: &Metadata) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut contents = String::new();

    writeln!(&mut contents, "{}", create_interrupts(metadata))?;

    // Create instances for all peripherals
    for peripheral in metadata.peripherals.iter() {
        let Some(peripheral_block) = peripheral
            .parse_block_path()
            .context("parsing peripheral block")?
        else {
            tracing::warn!(
                "Peripheral {} has no block associated. Skipped.",
                peripheral.name
            );
            continue;
        };

        let rust_mod_name = Ident::new(&peripheral_block.rust_mod_name, Span::call_site());
        let rust_type_name = Ident::new(&peripheral_block.rust_type_name, Span::call_site());

        let peripheral_name = Ident::new(&peripheral.name, Span::call_site());
        let Some(peripheral_address) = peripheral.peripheral_address.as_ref() else {
            tracing::warn!(
                "Peripheral {} has no address associated. Skipped.",
                peripheral.name
            );
            continue;
        };
        let periperal_address: TokenStream = peripheral_address
            .parse()
            .expect("peripheral_address parsed as tokenstream");

        writeln!(
            &mut contents,
            "{}",
            quote! {
                pub const #peripheral_name: #rust_mod_name::#rust_type_name = unsafe { #rust_mod_name::#rust_type_name::from_ptr(#periperal_address as _) };
            }
        )?;
    }

    // Create mods with a path pointing into meta_peripherals
    let blocks = metadata
        .peripherals
        .iter()
        .map(|p| p.parse_block_path())
        .collect::<anyhow::Result<IndexSet<Option<BlockPath>>>>()?;
    let mut exported_modules = HashSet::new();
    for peripheral_block in blocks.into_iter().flatten() {
        let driver_name_mod = Ident::new(&peripheral_block.rust_mod_name, Span::call_site());

        if !exported_modules.insert(peripheral_block.rust_mod_name) {
            continue;
        }

        let full_driver_path = format!("../../meta_peripherals/{}.rs", peripheral_block.path);

        writeln!(
            &mut contents,
            "{}",
            quote! {
                #[path = #full_driver_path]
                pub mod #driver_name_mod;
            }
        )?;
    }

    writeln!(&mut contents, "pub mod common;")?;

    let path = chip_dir.join("mod.rs");
    fs::write(&path, contents.as_bytes()).context("writing contents to file")?;
    crate::util::rustfmt(&path)?;

    Ok(())
}

fn create_interrupts(metadata: &Metadata) -> TokenStream {
    let interrupt_tokens = metadata.interrupts.iter().map(|(name, val)| {
        let val_lit = Literal::u32_unsuffixed(*val);
        let name_ident = Ident::new(name, Span::call_site());
        let doc = format!("{val} - {name}");

        quote! {
            #[doc = #doc]
            #name_ident = #val_lit
        }
    });

    let nvic_prio_bits = Literal::u32_unsuffixed(metadata.nvic_prio_bits);

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Interrupt {
            #(#interrupt_tokens),*
        }

        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        #[cfg(feature = "rt")]
        mod _vectors;

        #[doc = r" Number available in the NVIC for configuring priority"]
        #[cfg(feature = "rt")]
        pub const NVIC_PRIO_BITS: u8 = #nvic_prio_bits;
        #[cfg(feature = "rt")]
        pub use Interrupt as interrupt;
        #[cfg(feature = "rt")]
        pub use cortex_m_rt::interrupt;
    }
}

fn export_vectors_rs(chip_dir: &Path, metadata: &Metadata) -> anyhow::Result<()> {
    use std::fmt::Write;

    let extern_functions = metadata.interrupts.iter().map(|(name, _)| {
        let name_identifier = Ident::new(name, Span::call_site());
        quote! { fn #name_identifier(); }
    });

    let extern_functions_tokens = quote! {
        unsafe extern "C" {
            #(#extern_functions)*
        }
    };

    let num_interrupts = metadata
        .interrupts
        .iter()
        .map(|(_, num)| *num)
        .max()
        .unwrap_or_default()
        + 1;

    let vectors = (0..num_interrupts).map(|num| {
        match metadata
            .interrupts
            .iter()
            .find(|(_, irq_num)| **irq_num == num)
        {
            Some((name, _)) => {
                let name_ident = Ident::new(name, Span::call_site());
                quote! { Vector { _handler: #name_ident } }
            }
            None => quote! { Vector { _reserved: 0 } },
        }
    });

    let num_interrupts_lit = Literal::u32_unsuffixed(num_interrupts);

    let interrupt_vector_table = quote! {
        pub union Vector {
            _handler: unsafe extern "C" fn(),
            _reserved: u32,
        }
        #[unsafe(link_section = ".vector_table.interrupts")]
        #[unsafe(no_mangle)]
        pub static __INTERRUPTS: [Vector; #num_interrupts_lit] = [
            #(#vectors),*
        ];
    };

    let mut contents = String::new();

    writeln!(&mut contents, "{}", extern_functions_tokens)?;
    writeln!(&mut contents, "{}", interrupt_vector_table)?;

    let path = chip_dir.join("_vectors.rs");
    fs::write(&path, contents.as_bytes()).context("writing contents to file")?;
    crate::util::rustfmt(&path)?;

    Ok(())
}

fn export_common_rs(chip_dir: &Path) -> anyhow::Result<()> {
    chiptool::commands::gen_common::gen_common(GenCommon {
        output: chip_dir.join("common.rs"),
    })
    .context("Error generating common")
}
