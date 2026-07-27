#[cfg_attr(feature = "mcxa256", path = "./chips/mcxa256/metadata.rs")]
#[cfg_attr(feature = "mcxa577", path = "./chips/mcxa577/metadata.rs")]
#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/metadata.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/metadata.rs")]
#[cfg_attr(feature = "mimxrt1064", path = "./chips/mimxrt1064/metadata.rs")]
#[cfg_attr(
    feature = "mimxrt685s_cm33",
    path = "./chips/mimxrt685s_cm33/metadata.rs"
)]
#[cfg_attr(feature = "lpc55s16", path = "./chips/lpc55s16/metadata.rs")]
#[cfg_attr(
    feature = "lpc55s69_cm33_core0",
    path = "./chips/lpc55s69_cm33_core0/metadata.rs"
)]
#[cfg_attr(
    feature = "lpc55s69_cm33_core1",
    path = "./chips/lpc55s69_cm33_core1/metadata.rs"
)]
mod _generated;

#[path = "./meta_peripherals/_peripherals.rs"]
mod meta_peripherals;

#[derive(Debug)]
pub struct Metadata {
    pub name: &'static str,
    pub pins: &'static [Pin],
    pub peripherals: &'static [Peripheral],
    pub interrupts: &'static [(&'static str, u32)],
}

#[derive(Debug)]
pub struct Pin {
    pub name: &'static str,
    pub iomuxc: Option<PinIomuxc>,
    pub feature: Option<&'static str>,
}

#[derive(Debug)]
pub struct PinIomuxc {
    pub mux: Option<u32>,
    pub pad: u32,
}

#[derive(Debug)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u32,
    pub driver_name: &'static str,
    pub signals: &'static [Signal],
    pub flexcomm: Option<&'static str>,
    pub dma_muxing: &'static [DmaMux],
    pub gate: Option<Gate>,
}

#[derive(Debug)]
pub struct Signal {
    pub name: &'static str,
    pub pins: &'static [SignalPin],
    pub iomuxc_daisy: Option<u32>,
}

#[derive(Debug)]
pub struct SignalPin {
    pub pin: &'static str,
    pub alt: u8,
    pub iomuxc_daisy: Option<u8>,
}

#[derive(Debug)]
pub struct DmaMux {
    pub signal: &'static str,
    pub mux: &'static str,
    pub request: u8,
}

#[derive(Debug)]
pub struct Gate {
    pub enable: &'static str,
    pub reset: Option<&'static str>,
    pub config: Option<&'static str>,
    pub bit: &'static str,
}

pub use _generated::*;
pub use meta_peripherals::META_PERIPHERALS;
