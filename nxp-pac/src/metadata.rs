#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/metadata.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/metadata.rs")]
#[cfg_attr(feature = "mimxrt1064", path = "./chips/mimxrt1064/metadata.rs")]
#[cfg_attr(
    feature = "mimxrt685s_cm33",
    path = "./chips/mimxrt685s_cm33/metadata.rs"
)]
#[cfg_attr(
    feature = "lpc55s69_cm33_core0",
    path = "./chips/lpc55s69_cm33_core0/metadata.rs"
)]
#[cfg_attr(
    feature = "lpc55s69_cm33_core1",
    path = "./chips/lpc55s69_cm33_core1/metadata.rs"
)]
mod _generated;

pub struct Metadata {
    pub name: &'static str,
    pub pins: &'static [Pin],
    pub peripherals: &'static [Peripheral],
    pub interrupts: &'static [&'static str],
}

pub struct Pin {
    pub name: &'static str,
    pub iomuxc: Option<PinIomuxc>,
}

pub struct PinIomuxc {
    pub mux: Option<u32>,
    pub pad: u32,
}

pub struct Peripheral {
    pub name: &'static str,
    pub signals: &'static [Signal],
    pub flexcomm: Option<&'static str>,
    pub dma_muxing: &'static [DmaMux],
}

pub struct Signal {
    pub name: &'static str,
    pub pins: &'static [SignalPin],
    pub iomuxc_daisy: Option<u32>,
}

pub struct SignalPin {
    pub pin: &'static str,
    pub alt: u8,
    pub iomuxc_daisy: Option<u8>,
}

pub struct DmaMux {
    pub signal: &'static str,
    pub mux: &'static str,
    pub request: u8,
}

pub use _generated::*;
