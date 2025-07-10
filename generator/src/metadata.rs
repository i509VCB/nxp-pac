use std::{fmt::Write, fs, path::Path};

use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::iomuxc::{self, IomuxcRegisters};

pub fn generate_core(svd: &Path, chips_dir: &Path, core: &str) -> anyhow::Result<()> {
    // IMXRT10xx and 11xx require extra metadata for IOMUXC
    let iomuxc = core
        .starts_with("MIMXRT1")
        .then(|| iomuxc::get_registers(svd))
        .transpose()?;

    let mut metadata = String::new();

    if let Some(iomuxc) = iomuxc {
        writeln!(metadata, "{}", generate_iomuxc(&iomuxc)?)?;
    }

    let metadata_rs = chips_dir.join(core.to_lowercase()).join("metadata.rs");
    fs::write(metadata_rs, metadata)?;

    Ok(())
}

fn generate_iomuxc(registers: &[IomuxcRegisters]) -> anyhow::Result<TokenStream> {
    let registers = registers.iter().map(|registers| {
        let name = &registers.name;
        let mux_ctl = Literal::u32_unsuffixed(registers.mux_address);
        let pad_ctl = Literal::u32_unsuffixed(registers.pad_address);

        quote! {
            IomuxcRegisters { name: #name, mux_ctl: #mux_ctl, pad_ctl: #pad_ctl }
        }
    });

    Ok(quote! {
        pub mod iomuxc {
            pub struct IomuxcRegisters {
                pub name: &'static str,
                pub mux_ctl: u32,
                pub pad_ctl: u32,
            }

            pub const IOMUXC_REGISTERS: &[IomuxcRegisters] = &[
                #(
                    #registers
                ),*
            ];
        }
    })
}
