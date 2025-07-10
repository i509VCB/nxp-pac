//! IOMUXC metadata generation for IMXRT1xxx parts.

use std::{fs, path::Path};

use svd_parser::svd::{MaybeArray, PeripheralInfo};

#[derive(Debug)]
pub struct IomuxcRegisters {
    pub name: String,
    pub pad_address: u32,
    pub mux_address: u32,
}

pub fn get_registers(svd: &Path) -> anyhow::Result<Vec<IomuxcRegisters>> {
    let config = svd_parser::Config::default();
    let xml = fs::read_to_string(svd)?;

    let device = svd_parser::parse_with_config(&xml, &config)?;

    let iomuxc_peripherals = device
        .peripherals
        .iter()
        .filter(|info| info.name.contains("IOMUXC"))
        .collect::<Vec<_>>();

    let mut registers = Vec::new();

    for peripheral in iomuxc_peripherals {
        let base_address = peripheral.base_address;
        registers.extend(get_peripheral_registers(base_address as u32, peripheral)?);
    }

    Ok(registers)
}

fn get_peripheral_registers(
    base_address: u32,
    peripheral: &MaybeArray<PeripheralInfo>,
) -> anyhow::Result<Vec<IomuxcRegisters>> {
    let mut registers = Vec::new();

    for mux_register in peripheral.registers() {
        let mux_register_name = &mux_register.name;

        if !mux_register_name.contains("SW_MUX_CTL_PAD") {
            continue;
        }

        // Find the matching PAD_CTL register
        let pad_register_name = &format!(
            "SW_PAD_CTL_PAD{}",
            mux_register_name.strip_prefix("SW_MUX_CTL_PAD").unwrap()
        );
        let Some(pad_register) = peripheral.get_register(&pad_register_name) else {
            eprintln!("{pad_register_name} has no matching PAD_CTL register");
            continue;
        };

        let mux_address = base_address + mux_register.address_offset;
        let pad_address = base_address + pad_register.address_offset;

        registers.push(IomuxcRegisters {
            name: mux_register_name
                .strip_prefix("SW_MUX_CTL_PAD_")
                .unwrap()
                .to_string(),
            pad_address,
            mux_address,
        });
    }

    Ok(registers)
}
