pub mod commands;
pub mod metadata;
pub mod metapac;
pub mod pac;
pub mod util;

pub struct ChipDescription {
    /// The name of the chip to generate.
    chip: &'static str,

    /// Metadata file for this chip.
    metadata: Option<&'static str>,

    /// Cores to generate. If the chip has a single core, then this is the same as the
    /// [`name`](Feature::name) of the chip.
    cores: &'static [&'static str],

    /// When true, the source of the chip won't be the SVD anymore, but the manually curated peripherals
    metapac: bool,
}

/// Parts (and cores) to generate.
#[rustfmt::skip]
pub const CHIPS: &[ChipDescription] = &[
    ChipDescription { chip: "MIMXRT1011", metadata: Some("MIMXRT1011"), cores: &["MIMXRT1011"], metapac: false },
    ChipDescription { chip: "MIMXRT1062", metadata: Some("MIMXRT106x"), cores: &["MIMXRT1062"], metapac: false },
    ChipDescription { chip: "MIMXRT1064", metadata: Some("MIMXRT106x"), cores: &["MIMXRT1064"], metapac: false },
    ChipDescription { chip: "MIMXRT685S", metadata: None, cores: &["MIMXRT685S_cm33"], metapac: false },
    ChipDescription { chip: "LPC55S16", metadata: Some("LPC55S16"), cores: &["LPC55S16"], metapac: false },
    ChipDescription { chip: "LPC55S69", metadata: Some("LPC55S6x"), cores: &["LPC55S69_cm33_core0", "LPC55S69_cm33_core1"], metapac: false },
    ChipDescription { chip: "MCXN947", metadata: None, cores: &["MCXN947_cm33_core0", "MCXN947_cm33_core1"], metapac: false },
    ChipDescription { chip: "MCXA156", metadata: Some("MCXA1xx"), cores: &["MCXA156"], metapac: true },
    ChipDescription { chip: "MCXA256", metadata: Some("MCXA2xx"), cores: &["MCXA256"], metapac: true },
    ChipDescription { chip: "MCXA577", metadata: Some("MCXA5xx"), cores: &["MCXA577"], metapac: true },
];
