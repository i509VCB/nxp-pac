use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "MCXA256",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "P1_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_29",
        iomuxc: None,
        feature: Some("dangerous-reset-as-gpio"),
    },
    Pin {
        name: "P1_30",
        iomuxc: None,
        feature: Some("sosc-as-gpio"),
    },
    Pin {
        name: "P1_31",
        iomuxc: None,
        feature: Some("sosc-as-gpio"),
    },
    Pin {
        name: "P4_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_24",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_25",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P2_26",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_31",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_29",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_28",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_27",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_26",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_25",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_24",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_0",
        iomuxc: None,
        feature: Some("swd-as-gpio"),
    },
    Pin {
        name: "P0_1",
        iomuxc: None,
        feature: Some("swd-as-gpio"),
    },
    Pin {
        name: "P0_2",
        iomuxc: None,
        feature: Some("swd-swo-as-gpio"),
    },
    Pin {
        name: "P0_3",
        iomuxc: None,
        feature: Some("jtag-extras-as-gpio"),
    },
    Pin {
        name: "P0_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_6",
        iomuxc: None,
        feature: Some("jtag-extras-as-gpio"),
    },
    Pin {
        name: "P0_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_24",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_25",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_26",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_27",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_7",
        iomuxc: None,
        feature: None,
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x400AF000,
        driver_name: "mcxa/ADC",
        signals: &[
            Signal {
                name: "A0",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A1",
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A10",
                pins: &[SignalPin {
                    pin: "P0_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A11",
                pins: &[SignalPin {
                    pin: "P0_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A12",
                pins: &[SignalPin {
                    pin: "P0_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A13",
                pins: &[SignalPin {
                    pin: "P0_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A14",
                pins: &[SignalPin {
                    pin: "P0_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A15",
                pins: &[SignalPin {
                    pin: "P0_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A16",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A17",
                pins: &[SignalPin {
                    pin: "P1_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A18",
                pins: &[SignalPin {
                    pin: "P1_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A19",
                pins: &[SignalPin {
                    pin: "P1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A2",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A20",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A21",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A22",
                pins: &[SignalPin {
                    pin: "P1_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A23",
                pins: &[SignalPin {
                    pin: "P1_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A3",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A4",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A5",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A6",
                pins: &[SignalPin {
                    pin: "P2_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A7",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A8",
                pins: &[SignalPin {
                    pin: "P0_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A9",
                pins: &[SignalPin {
                    pin: "P0_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC0FifoRequest",
            mux: "DMA3",
            request: 51,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("AdcConfig"),
            bit: "adc0",
        }),
    },
    Peripheral {
        name: "ADC1",
        address: 0x400B0000,
        driver_name: "mcxa/ADC",
        signals: &[
            Signal {
                name: "A0",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A1",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A10",
                pins: &[SignalPin {
                    pin: "P1_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A11",
                pins: &[SignalPin {
                    pin: "P1_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A12",
                pins: &[SignalPin {
                    pin: "P1_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A13",
                pins: &[SignalPin {
                    pin: "P1_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A14",
                pins: &[SignalPin {
                    pin: "P1_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A15",
                pins: &[SignalPin {
                    pin: "P1_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A16",
                pins: &[SignalPin {
                    pin: "P1_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A17",
                pins: &[SignalPin {
                    pin: "P1_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A2",
                pins: &[SignalPin {
                    pin: "P2_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A20",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A21",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A22",
                pins: &[SignalPin {
                    pin: "P3_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A3",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A4",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A5",
                pins: &[SignalPin {
                    pin: "P2_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A6",
                pins: &[SignalPin {
                    pin: "P2_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A7",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A8",
                pins: &[SignalPin {
                    pin: "P1_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A9",
                pins: &[SignalPin {
                    pin: "P1_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC1FifoRequest",
            mux: "DMA3",
            request: 52,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("AdcConfig"),
            bit: "adc1",
        }),
    },
    Peripheral {
        name: "ADC2",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC2FifoRequest",
            mux: "DMA3",
            request: 123,
        }],
        gate: None,
    },
    Peripheral {
        name: "ADC3",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC3FifoRequest",
            mux: "DMA3",
            request: 124,
        }],
        gate: None,
    },
    Peripheral {
        name: "AOI0",
        address: 0x40089000,
        driver_name: "mcxa/AOI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "AOI1",
        address: 0x40097000,
        driver_name: "mcxa/AOI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CAN0",
        address: 0x400CC000,
        driver_name: "mcxa/CAN::Can",
        signals: &[
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_19",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_18",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CAN0",
            mux: "DMA3",
            request: 2,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("CanConfig"),
            bit: "flexcan0",
        }),
    },
    Peripheral {
        name: "CAN1",
        address: 0x400D0000,
        driver_name: "mcxa/CAN::Can",
        signals: &[
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_16",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_17",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_1",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CAN1",
            mux: "DMA3",
            request: 87,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("CanConfig"),
            bit: "flexcan1",
        }),
    },
    Peripheral {
        name: "CDOG0",
        address: 0x40100000,
        driver_name: "mcxa/CDOG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CDOG1",
        address: 0x40107000,
        driver_name: "mcxa/CDOG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CLKOUT",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[
                SignalPin {
                    pin: "P4_2",
                    alt: 1u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P3_8",
                    alt: 12u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P3_6",
                    alt: 1u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P0_6",
                    alt: 12u8,
                    iomuxc_daisy: None,
                },
            ],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CMC",
        address: 0x4008B000,
        driver_name: "mcxa/CMC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CMP0",
        address: 0x400B1000,
        driver_name: "mcxa/CMP",
        signals: &[
            Signal {
                name: "IN0",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN1",
                pins: &[SignalPin {
                    pin: "P1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN2",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN3",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INN4",
                pins: &[SignalPin {
                    pin: "P2_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CMP1",
        address: 0x400B2000,
        driver_name: "mcxa/CMP",
        signals: &[
            Signal {
                name: "IN0",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN1",
                pins: &[SignalPin {
                    pin: "P0_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN2",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN3",
                pins: &[SignalPin {
                    pin: "P1_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INN4",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[SignalPin {
                    pin: "P2_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_6",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CMP2",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CRC0",
        address: 0x4008A000,
        driver_name: "mcxa/CRC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: None,
            bit: "crc0",
        }),
    },
    Peripheral {
        name: "CTIMER0",
        address: 0x40004000,
        driver_name: "mcx/CTIMER",
        signals: &[
            Signal {
                name: "INP0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P2_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP11",
                pins: &[SignalPin {
                    pin: "P3_23",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP12",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP13",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP14",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP16",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP17",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP3",
                pins: &[
                    SignalPin {
                        pin: "P3_29",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP5",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP6",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP7",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP9",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P2_12",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT1",
                pins: &[
                    SignalPin {
                        pin: "P2_13",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT2",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_12",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT3",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_17",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_13",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER0M0",
                mux: "DMA3",
                request: 31,
            },
            DmaMux {
                signal: "CTIMER0M1",
                mux: "DMA3",
                request: 32,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
            bit: "ctimer0",
        }),
    },
    Peripheral {
        name: "CTIMER1",
        address: 0x40005000,
        driver_name: "mcx/CTIMER",
        signals: &[
            Signal {
                name: "INP0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P2_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP11",
                pins: &[SignalPin {
                    pin: "P3_23",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP12",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP13",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP14",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP16",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP17",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP3",
                pins: &[
                    SignalPin {
                        pin: "P3_29",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP5",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP6",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP7",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP9",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT1",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT2",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT3",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER1M0",
                mux: "DMA3",
                request: 33,
            },
            DmaMux {
                signal: "CTIMER1M1",
                mux: "DMA3",
                request: 34,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
            bit: "ctimer1",
        }),
    },
    Peripheral {
        name: "CTIMER2",
        address: 0x40006000,
        driver_name: "mcx/CTIMER",
        signals: &[
            Signal {
                name: "INP0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P2_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP11",
                pins: &[SignalPin {
                    pin: "P3_23",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP12",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP13",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP14",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP16",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP17",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP3",
                pins: &[
                    SignalPin {
                        pin: "P3_29",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP5",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP6",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP7",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP9",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT1",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT2",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT3",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER2M0",
                mux: "DMA3",
                request: 35,
            },
            DmaMux {
                signal: "CTIMER2M1",
                mux: "DMA3",
                request: 36,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
            bit: "ctimer2",
        }),
    },
    Peripheral {
        name: "CTIMER3",
        address: 0x40007000,
        driver_name: "mcx/CTIMER",
        signals: &[
            Signal {
                name: "INP0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P2_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP11",
                pins: &[SignalPin {
                    pin: "P3_23",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP12",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP13",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP14",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP16",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP17",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP3",
                pins: &[
                    SignalPin {
                        pin: "P3_29",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP5",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP6",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP7",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP9",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_18",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT1",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_19",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT2",
                pins: &[
                    SignalPin {
                        pin: "P2_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_18",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT3",
                pins: &[
                    SignalPin {
                        pin: "P2_11",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_19",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_29",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER3M0",
                mux: "DMA3",
                request: 37,
            },
            DmaMux {
                signal: "CTIMER3M1",
                mux: "DMA3",
                request: 38,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
            bit: "ctimer3",
        }),
    },
    Peripheral {
        name: "CTIMER4",
        address: 0x40008000,
        driver_name: "mcx/CTIMER",
        signals: &[
            Signal {
                name: "INP0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P2_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP11",
                pins: &[SignalPin {
                    pin: "P3_23",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP12",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP13",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP14",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP16",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP17",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP3",
                pins: &[
                    SignalPin {
                        pin: "P3_29",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP4",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP5",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP6",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP7",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_24",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP9",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_25",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT1",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT2",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_14",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT3",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER4M0",
                mux: "DMA3",
                request: 39,
            },
            DmaMux {
                signal: "CTIMER4M1",
                mux: "DMA3",
                request: 40,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
            bit: "ctimer4",
        }),
    },
    Peripheral {
        name: "DAC0",
        address: 0x400B4000,
        driver_name: "mcx/DAC",
        signals: &[Signal {
            name: "OUT",
            pins: &[SignalPin {
                pin: "P2_2",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "DAC0FifoRequest",
            mux: "DMA3",
            request: 56,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("DacConfig"),
            bit: "dac0",
        }),
    },
    Peripheral {
        name: "DBGMAILBOX",
        address: 0x40101000,
        driver_name: "mcxa/DBGMAILBOX",
        signals: &[
            Signal {
                name: "TCLK/SWCLK",
                pins: &[SignalPin {
                    pin: "P0_1",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TDI",
                pins: &[SignalPin {
                    pin: "P0_3",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TDO/SWO",
                pins: &[SignalPin {
                    pin: "P0_2",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TMS/SWDIO",
                pins: &[SignalPin {
                    pin: "P0_0",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "DMA0",
        address: 0x40080000,
        driver_name: "mcxa/DMA::DMA8",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: None,
            bit: "dma0",
        }),
    },
    Peripheral {
        name: "EIM0",
        address: 0x4008C000,
        driver_name: "mcxa/EIM0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ERM0",
        address: 0x4008D000,
        driver_name: "mcxa/ERM0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EXTAL48M",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P1_31",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXIO0",
        address: 0x40099000,
        driver_name: "mcxa/FLEXIO",
        signals: &[
            Signal {
                name: "D0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D10",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D11",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D12",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D13",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D14",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D15",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D16",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_8",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D17",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_9",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D18",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_10",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D19",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D2",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D20",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D21",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D22",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_14",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D23",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D24",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D25",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D26",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_18",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D27",
                pins: &[
                    SignalPin {
                        pin: "P1_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D28",
                pins: &[
                    SignalPin {
                        pin: "P2_20",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D29",
                pins: &[
                    SignalPin {
                        pin: "P2_21",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_29",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D3",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D30",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_22",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D31",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_23",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_31",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D4",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D5",
                pins: &[
                    SignalPin {
                        pin: "P0_5",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_13",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D6",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D7",
                pins: &[
                    SignalPin {
                        pin: "P0_7",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D8",
                pins: &[
                    SignalPin {
                        pin: "P4_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D9",
                pins: &[
                    SignalPin {
                        pin: "P4_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXIO0SR0",
                mux: "DMA3",
                request: 71,
            },
            DmaMux {
                signal: "FLEXIO0SR1",
                mux: "DMA3",
                request: 72,
            },
            DmaMux {
                signal: "FLEXIO0SR2",
                mux: "DMA3",
                request: 73,
            },
            DmaMux {
                signal: "FLEXIO0SR3",
                mux: "DMA3",
                request: 74,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "FMC0",
        address: 0x40094000,
        driver_name: "mcxa/FMC0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FMU0",
        address: 0x40095000,
        driver_name: "mcxa/FMU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FREQME0",
        address: 0x40009000,
        driver_name: "mcxa/FREQME0",
        signals: &[
            Signal {
                name: "CLK_IN0",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_18",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CLK_IN1",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_19",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CLK_OUT0",
                pins: &[SignalPin {
                    pin: "P3_1",
                    alt: 12u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CLK_OUT1",
                pins: &[SignalPin {
                    pin: "P3_6",
                    alt: 12u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEX_PWM0",
        address: 0x400A9000,
        driver_name: "mcxa/FLEXPWM",
        signals: &[
            Signal {
                name: "A0",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A1",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A2",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A3",
                pins: &[
                    SignalPin {
                        pin: "P4_0",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B0",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B1",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B2",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B3",
                pins: &[
                    SignalPin {
                        pin: "P4_1",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN0",
                pins: &[
                    SignalPin {
                        pin: "P3_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN1",
                pins: &[
                    SignalPin {
                        pin: "P3_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN10",
                pins: &[
                    SignalPin {
                        pin: "P3_31",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN11",
                pins: &[
                    SignalPin {
                        pin: "P3_28",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN2",
                pins: &[
                    SignalPin {
                        pin: "P3_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN3",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN4",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_9",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN5",
                pins: &[
                    SignalPin {
                        pin: "P4_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_26",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN6",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN7",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN8",
                pins: &[
                    SignalPin {
                        pin: "P2_13",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_20",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_IN9",
                pins: &[
                    SignalPin {
                        pin: "P2_17",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_21",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT0",
                pins: &[
                    SignalPin {
                        pin: "P3_20",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT1",
                pins: &[
                    SignalPin {
                        pin: "P3_21",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT2",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT3",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_8",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT4",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT5",
                pins: &[
                    SignalPin {
                        pin: "P2_10",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_19",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_23",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT6",
                pins: &[
                    SignalPin {
                        pin: "P2_24",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TRIG_OUT7",
                pins: &[
                    SignalPin {
                        pin: "P2_25",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X0",
                pins: &[
                    SignalPin {
                        pin: "P3_18",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X1",
                pins: &[
                    SignalPin {
                        pin: "P3_19",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X2",
                pins: &[
                    SignalPin {
                        pin: "P3_20",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X3",
                pins: &[
                    SignalPin {
                        pin: "P3_21",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FlexPWM0Mcapt0",
                mux: "DMA3",
                request: 41,
            },
            DmaMux {
                signal: "FlexPWM0Mcapt1",
                mux: "DMA3",
                request: 42,
            },
            DmaMux {
                signal: "FlexPWM0Mcapt2",
                mux: "DMA3",
                request: 43,
            },
            DmaMux {
                signal: "FlexPWM0Mcapt3",
                mux: "DMA3",
                request: 44,
            },
            DmaMux {
                signal: "FlexPWM0Mval0",
                mux: "DMA3",
                request: 45,
            },
            DmaMux {
                signal: "FlexPWM0Mval1",
                mux: "DMA3",
                request: 46,
            },
            DmaMux {
                signal: "FlexPWM0Mval2",
                mux: "DMA3",
                request: 47,
            },
            DmaMux {
                signal: "FlexPWM0Mval3",
                mux: "DMA3",
                request: 48,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "FLEX_PWM1",
        address: 0x400AA000,
        driver_name: "mcxa/FLEXPWM",
        signals: &[
            Signal {
                name: "A0",
                pins: &[
                    SignalPin {
                        pin: "P3_30",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A1",
                pins: &[SignalPin {
                    pin: "P3_14",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A2",
                pins: &[SignalPin {
                    pin: "P3_12",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A3",
                pins: &[
                    SignalPin {
                        pin: "P3_27",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B0",
                pins: &[
                    SignalPin {
                        pin: "P3_31",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B1",
                pins: &[SignalPin {
                    pin: "P3_15",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B2",
                pins: &[SignalPin {
                    pin: "P3_13",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B3",
                pins: &[
                    SignalPin {
                        pin: "P3_28",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X0",
                pins: &[
                    SignalPin {
                        pin: "P3_18",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X1",
                pins: &[
                    SignalPin {
                        pin: "P3_19",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X2",
                pins: &[
                    SignalPin {
                        pin: "P3_22",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X3",
                pins: &[
                    SignalPin {
                        pin: "P3_23",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_3",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FlexPWM1Mcapt0",
                mux: "DMA3",
                request: 79,
            },
            DmaMux {
                signal: "FlexPWM1Mcapt1",
                mux: "DMA3",
                request: 80,
            },
            DmaMux {
                signal: "FlexPWM1Mcapt2",
                mux: "DMA3",
                request: 81,
            },
            DmaMux {
                signal: "FlexPWM1Mcapt3",
                mux: "DMA3",
                request: 82,
            },
            DmaMux {
                signal: "FlexPWM1Mval0",
                mux: "DMA3",
                request: 83,
            },
            DmaMux {
                signal: "FlexPWM1Mval1",
                mux: "DMA3",
                request: 84,
            },
            DmaMux {
                signal: "FlexPWM1Mval2",
                mux: "DMA3",
                request: 85,
            },
            DmaMux {
                signal: "FlexPWM1Mval3",
                mux: "DMA3",
                request: 86,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO0",
        address: 0x40102000,
        driver_name: "mcx/GPIO",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "P0_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "P0_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "P0_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "P0_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "P0_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "P0_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "P0_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "P0_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "P0_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "P0_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "P0_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "P0_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "P0_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "P0_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "P0_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "P0_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "P0_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "P0_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "P0_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "P0_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "P0_24",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "P0_25",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "P0_26",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "P0_27",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO0PinEvent0",
            mux: "DMA3",
            request: 60,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc2",
            reset: Some("mrcc_glb_rst2"),
            config: None,
            bit: "gpio0",
        }),
    },
    Peripheral {
        name: "GPIO1",
        address: 0x40103000,
        driver_name: "mcx/GPIO",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "P1_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "P1_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "P1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "P1_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "P1_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "P1_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "P1_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "P1_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "P1_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "P1_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "P1_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "P1_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "P1_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "P1_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "P1_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "P1_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "P1_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "P1_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "P1_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "P1_31",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO1PinEvent0",
            mux: "DMA3",
            request: 61,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc2",
            reset: Some("mrcc_glb_rst2"),
            config: None,
            bit: "gpio1",
        }),
    },
    Peripheral {
        name: "GPIO2",
        address: 0x40104000,
        driver_name: "mcx/GPIO",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "P2_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "P2_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "P2_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "P2_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "P2_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "P2_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "P2_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "P2_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "P2_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "P2_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "P2_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "P2_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "P2_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "P2_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "P2_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "P2_24",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "P2_25",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "P2_26",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO2PinEvent0",
            mux: "DMA3",
            request: 62,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc2",
            reset: Some("mrcc_glb_rst2"),
            config: None,
            bit: "gpio2",
        }),
    },
    Peripheral {
        name: "GPIO3",
        address: 0x40105000,
        driver_name: "mcx/GPIO",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "P3_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "P3_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "P3_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "P3_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "P3_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "P3_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "P3_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "P3_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "P3_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "P3_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "P3_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "P3_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "P3_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "P3_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "P3_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "P3_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "P3_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "P3_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "P3_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "P3_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "P3_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "P3_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "P3_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "P3_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "P3_24",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "P3_25",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "P3_26",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "P3_27",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "P3_28",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "P3_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO3PinEvent0",
            mux: "DMA3",
            request: 63,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc2",
            reset: Some("mrcc_glb_rst2"),
            config: None,
            bit: "gpio3",
        }),
    },
    Peripheral {
        name: "GPIO4",
        address: 0x40106000,
        driver_name: "mcx/GPIO",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "P4_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "P4_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "P4_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "P4_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "P4_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "P4_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "P4_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "P4_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO4PinEvent0",
            mux: "DMA3",
            request: 64,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc2",
            reset: Some("mrcc_glb_rst2"),
            config: None,
            bit: "gpio4",
        }),
    },
    Peripheral {
        name: "I3C0",
        address: 0x40002000,
        driver_name: "mcxa/I3C",
        signals: &[
            Signal {
                name: "PUR",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_2",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCL",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_31",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDA",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_30",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "I3C0Rx",
                mux: "DMA3",
                request: 7,
            },
            DmaMux {
                signal: "I3C0Tx",
                mux: "DMA3",
                request: 8,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("I3cConfig"),
            bit: "i3c0",
        }),
    },
    Peripheral {
        name: "INPUTMUX0",
        address: 0x40001000,
        driver_name: "mcxa/INPUTMUX",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: None,
            bit: "inputmux0",
        }),
    },
    Peripheral {
        name: "ISPMODE",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "N",
            pins: &[
                SignalPin {
                    pin: "P3_29",
                    alt: 1u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P0_6",
                    alt: 1u8,
                    iomuxc_daisy: None,
                },
            ],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C0",
        address: 0x4009A000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "HREQ",
                pins: &[SignalPin {
                    pin: "P0_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCL",
                pins: &[
                    SignalPin {
                        pin: "P1_31",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCLS",
                pins: &[SignalPin {
                    pin: "P0_18",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDA",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDAS",
                pins: &[SignalPin {
                    pin: "P0_19",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C0Rx",
                mux: "DMA3",
                request: 11,
            },
            DmaMux {
                signal: "LPI2C0Tx",
                mux: "DMA3",
                request: 12,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("Lpi2cConfig"),
            bit: "lpi2c0",
        }),
    },
    Peripheral {
        name: "LPI2C1",
        address: 0x4009B000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "SCL",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCLS",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDA",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDAS",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C1Rx",
                mux: "DMA3",
                request: 13,
            },
            DmaMux {
                signal: "LPI2C1Tx",
                mux: "DMA3",
                request: 14,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("Lpi2cConfig"),
            bit: "lpi2c1",
        }),
    },
    Peripheral {
        name: "LPI2C2",
        address: 0x400D4000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "HREQ",
                pins: &[SignalPin {
                    pin: "P4_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCL",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCLS",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDA",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDAS",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C2Rx",
                mux: "DMA3",
                request: 3,
            },
            DmaMux {
                signal: "LPI2C2Tx",
                mux: "DMA3",
                request: 4,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("Lpi2cConfig"),
            bit: "lpi2c2",
        }),
    },
    Peripheral {
        name: "LPI2C3",
        address: 0x400D5000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "HREQ",
                pins: &[SignalPin {
                    pin: "P3_29",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCL",
                pins: &[
                    SignalPin {
                        pin: "P3_27",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCLS",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDA",
                pins: &[
                    SignalPin {
                        pin: "P3_28",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDAS",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C3Rx",
                mux: "DMA3",
                request: 5,
            },
            DmaMux {
                signal: "LPI2C3Tx",
                mux: "DMA3",
                request: 6,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("Lpi2cConfig"),
            bit: "lpi2c3",
        }),
    },
    Peripheral {
        name: "LPSPI0",
        address: 0x4009C000,
        driver_name: "mcxa/LPSPI",
        signals: &[
            Signal {
                name: "PCS0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PCS1",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PCS2",
                pins: &[
                    SignalPin {
                        pin: "P0_16",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PCS3",
                pins: &[
                    SignalPin {
                        pin: "P0_17",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDI",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDO",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPSPI0Rx",
                mux: "DMA3",
                request: 15,
            },
            DmaMux {
                signal: "LPSPI0Tx",
                mux: "DMA3",
                request: 16,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpspiConfig"),
            bit: "lpspi0",
        }),
    },
    Peripheral {
        name: "LPSPI1",
        address: 0x4009D000,
        driver_name: "mcxa/LPSPI",
        signals: &[
            Signal {
                name: "PCS0",
                pins: &[
                    SignalPin {
                        pin: "P2_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PCS1",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PCS2",
                pins: &[
                    SignalPin {
                        pin: "P2_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PCS3",
                pins: &[
                    SignalPin {
                        pin: "P2_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[
                    SignalPin {
                        pin: "P2_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDI",
                pins: &[
                    SignalPin {
                        pin: "P2_15",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SDO",
                pins: &[
                    SignalPin {
                        pin: "P2_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPSPI1Rx",
                mux: "DMA3",
                request: 17,
            },
            DmaMux {
                signal: "LPSPI1Tx",
                mux: "DMA3",
                request: 18,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpspiConfig"),
            bit: "lpspi1",
        }),
    },
    Peripheral {
        name: "LPTMR0",
        address: 0x400AB000,
        driver_name: "mcxa/LPTMR0",
        signals: &[
            Signal {
                name: "ALT2",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "ALT3",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "LPTMR0CounterMatchEvent",
            mux: "DMA3",
            request: 49,
        }],
        gate: None,
    },
    Peripheral {
        name: "LPUART0",
        address: 0x4009F000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "P2_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART0Rx",
                mux: "DMA3",
                request: 21,
            },
            DmaMux {
                signal: "LPUART0Tx",
                mux: "DMA3",
                request: 22,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
            bit: "lpuart0",
        }),
    },
    Peripheral {
        name: "LPUART1",
        address: 0x400A0000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_17",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_16",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_12",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_13",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART1Rx",
                mux: "DMA3",
                request: 23,
            },
            DmaMux {
                signal: "LPUART1Tx",
                mux: "DMA3",
                request: 24,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
            bit: "lpuart1",
        }),
    },
    Peripheral {
        name: "LPUART2",
        address: 0x400A1000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART2Rx",
                mux: "DMA3",
                request: 25,
            },
            DmaMux {
                signal: "LPUART2Tx",
                mux: "DMA3",
                request: 26,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
            bit: "lpuart2",
        }),
    },
    Peripheral {
        name: "LPUART3",
        address: 0x400A2000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART3Rx",
                mux: "DMA3",
                request: 27,
            },
            DmaMux {
                signal: "LPUART3Tx",
                mux: "DMA3",
                request: 28,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
            bit: "lpuart3",
        }),
    },
    Peripheral {
        name: "LPUART4",
        address: 0x400A3000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_31",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART4Rx",
                mux: "DMA3",
                request: 29,
            },
            DmaMux {
                signal: "LPUART4Tx",
                mux: "DMA3",
                request: 30,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
            bit: "lpuart4",
        }),
    },
    Peripheral {
        name: "LPUART5",
        address: 0x400DA000,
        driver_name: "mcxa/LPUART",
        signals: &[
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_19",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_27",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_18",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_26",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_16",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_24",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_17",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_25",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART5Rx",
                mux: "DMA3",
                request: 102,
            },
            DmaMux {
                signal: "LPUART5Tx",
                mux: "DMA3",
                request: 103,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("LpuartConfig"),
            bit: "lpuart5",
        }),
    },
    Peripheral {
        name: "MAU0",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "DMA",
            mux: "DMA3",
            request: 115,
        }],
        gate: None,
    },
    Peripheral {
        name: "OPAMP0",
        address: 0x400B7000,
        driver_name: "mcxa/OPAMP0",
        signals: &[
            Signal {
                name: "INN",
                pins: &[SignalPin {
                    pin: "P2_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OPAMP1",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OPAMP2",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OPAMP3",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OSTIMER0",
        address: 0x400AD000,
        driver_name: "mcxa/OSTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("OsTimerConfig"),
            bit: "ostimer0",
        }),
    },
    Peripheral {
        name: "PKC0",
        address: 0x400EA000,
        driver_name: "mcxa/PKC0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PORT0",
        address: 0x400BC000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
            bit: "port0",
        }),
    },
    Peripheral {
        name: "PORT1",
        address: 0x400BD000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
            bit: "port1",
        }),
    },
    Peripheral {
        name: "PORT2",
        address: 0x400BE000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
            bit: "port2",
        }),
    },
    Peripheral {
        name: "PORT3",
        address: 0x400BF000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
            bit: "port3",
        }),
    },
    Peripheral {
        name: "PORT4",
        address: 0x400C0000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
            bit: "port4",
        }),
    },
    Peripheral {
        name: "EQDC0",
        address: 0x400A7000,
        driver_name: "mcxa/EQDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "BUFFER",
            mux: "DMA3",
            request: 65,
        }],
        gate: None,
    },
    Peripheral {
        name: "EQDC1",
        address: 0x400A8000,
        driver_name: "mcxa/EQDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "BUFFER",
            mux: "DMA3",
            request: 66,
        }],
        gate: None,
    },
    Peripheral {
        name: "RESET",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "B",
            pins: &[
                SignalPin {
                    pin: "P1_29",
                    alt: 1u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P1_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                },
            ],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "RTC0",
        address: 0x400EE000,
        driver_name: "mcxa/RTC2xx::RTC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCG0",
        address: 0x4008F000,
        driver_name: "mcxa/SCG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SGI0",
        address: 0x400EB000,
        driver_name: "mcxa/SGI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "SGI0ReqIdat",
                mux: "DMA3",
                request: 119,
            },
            DmaMux {
                signal: "SGI0ReqOdat",
                mux: "DMA3",
                request: 120,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: None,
            config: Some("Clk1MConfig"),
            bit: "sgi0",
        }),
    },
    Peripheral {
        name: "SLCD0",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P1_0",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P1_1",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P10",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_12",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P11",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_13",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P12",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_14",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P13",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_15",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P14",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_16",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P15",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_17",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P16",
                pins: &[
                    SignalPin {
                        pin: "P0_12",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_12",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P17",
                pins: &[
                    SignalPin {
                        pin: "P0_13",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_13",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P18",
                pins: &[
                    SignalPin {
                        pin: "P0_14",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P19",
                pins: &[
                    SignalPin {
                        pin: "P0_15",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P1_2",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P20",
                pins: &[
                    SignalPin {
                        pin: "P0_16",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P21",
                pins: &[
                    SignalPin {
                        pin: "P0_17",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P22",
                pins: &[
                    SignalPin {
                        pin: "P0_18",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P23",
                pins: &[
                    SignalPin {
                        pin: "P0_19",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P24",
                pins: &[
                    SignalPin {
                        pin: "P0_20",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P25",
                pins: &[
                    SignalPin {
                        pin: "P0_21",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P26",
                pins: &[
                    SignalPin {
                        pin: "P0_22",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P27",
                pins: &[
                    SignalPin {
                        pin: "P0_23",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P28",
                pins: &[
                    SignalPin {
                        pin: "P0_24",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_24",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P29",
                pins: &[
                    SignalPin {
                        pin: "P0_25",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_25",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P1_3",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P30",
                pins: &[
                    SignalPin {
                        pin: "P0_26",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_26",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P31",
                pins: &[
                    SignalPin {
                        pin: "P0_27",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_27",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P32",
                pins: &[
                    SignalPin {
                        pin: "P3_0",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_0",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P33",
                pins: &[
                    SignalPin {
                        pin: "P3_1",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P34",
                pins: &[
                    SignalPin {
                        pin: "P3_6",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P35",
                pins: &[
                    SignalPin {
                        pin: "P3_7",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P36",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P37",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P38",
                pins: &[
                    SignalPin {
                        pin: "P3_10",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P39",
                pins: &[
                    SignalPin {
                        pin: "P3_11",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P1_6",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P40",
                pins: &[
                    SignalPin {
                        pin: "P3_12",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P41",
                pins: &[
                    SignalPin {
                        pin: "P3_13",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P42",
                pins: &[
                    SignalPin {
                        pin: "P3_14",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P43",
                pins: &[
                    SignalPin {
                        pin: "P3_15",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P44",
                pins: &[
                    SignalPin {
                        pin: "P3_16",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P45",
                pins: &[
                    SignalPin {
                        pin: "P3_17",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P46",
                pins: &[
                    SignalPin {
                        pin: "P3_18",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P47",
                pins: &[
                    SignalPin {
                        pin: "P3_19",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P1_7",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_8",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P7",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_9",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P8",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_10",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P9",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_11",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "VLL1",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "VLL2",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPC0",
        address: 0x40090000,
        driver_name: "mcxa/SPC",
        signals: &[Signal {
            name: "LPREQ",
            pins: &[SignalPin {
                pin: "P1_29",
                alt: 2u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SYSCON",
        address: 0x40091000,
        driver_name: "mcxa/SYSCON2xx::SYSCON",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SMART_DMA0",
        address: 0x4000E000,
        driver_name: "mcxa/SMARTDMA0",
        signals: &[
            Signal {
                name: "PIO0",
                pins: &[
                    SignalPin {
                        pin: "P3_0",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_4",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO1",
                pins: &[
                    SignalPin {
                        pin: "P3_1",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_5",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO10",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO11",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO12",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO13",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO14",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_10",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_14",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO15",
                pins: &[
                    SignalPin {
                        pin: "P1_19",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO16",
                pins: &[
                    SignalPin {
                        pin: "P2_12",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO17",
                pins: &[
                    SignalPin {
                        pin: "P2_13",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO18",
                pins: &[
                    SignalPin {
                        pin: "P2_15",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO19",
                pins: &[
                    SignalPin {
                        pin: "P2_16",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO2",
                pins: &[
                    SignalPin {
                        pin: "P3_2",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO20",
                pins: &[
                    SignalPin {
                        pin: "P4_0",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_17",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO21",
                pins: &[
                    SignalPin {
                        pin: "P4_1",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_19",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO22",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_20",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO23",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_21",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO24",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_24",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO25",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_0",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_25",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO26",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_2",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_26",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO27",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_3",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_27",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO28",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_28",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO29",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_29",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO3",
                pins: &[
                    SignalPin {
                        pin: "P3_3",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_7",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO30",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_30",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO31",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_31",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO4",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO5",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_5",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO6",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO7",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO8",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO9",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TAMPER0",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P3_31",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TAMPER1",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P3_29",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TAMPER2",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P3_26",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TAMPER3",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P3_25",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TAMPER4",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P3_19",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TAMPER5",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P3_18",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TDET0",
        address: 0x400E9000,
        driver_name: "mcxa/TDET0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TRNG0",
        address: 0x400EC000,
        driver_name: "mcxa/TRNG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
            bit: "trng0",
        }),
    },
    Peripheral {
        name: "UDF0",
        address: 0x400ED000,
        driver_name: "mcxa/UDF0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USB0",
        address: 0x400A4000,
        driver_name: "mcxa/USB",
        signals: &[Signal {
            name: "VBUS_DET",
            pins: &[SignalPin {
                pin: "P2_12",
                alt: 1u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "UTICK0",
        address: 0x4000B000,
        driver_name: "mcxa/UTICK0",
        signals: &[
            Signal {
                name: "CAP0",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CAP1",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CAP2",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CAP3",
                pins: &[
                    SignalPin {
                        pin: "P0_5",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "VBAT0",
        address: 0x40093000,
        driver_name: "mcxa/VBAT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "VREFI",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P2_7",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "WAKETIMER0",
        address: 0x400AE000,
        driver_name: "mcxa/WAKETIMER0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "WUU0",
        address: 0x40092000,
        driver_name: "mcxa/WUU0",
        signals: &[
            Signal {
                name: "IN0",
                pins: &[SignalPin {
                    pin: "P0_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN1",
                pins: &[SignalPin {
                    pin: "P0_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN10",
                pins: &[SignalPin {
                    pin: "P1_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN11",
                pins: &[SignalPin {
                    pin: "P1_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN12",
                pins: &[SignalPin {
                    pin: "P1_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN13",
                pins: &[SignalPin {
                    pin: "P1_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN14",
                pins: &[SignalPin {
                    pin: "P1_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN15",
                pins: &[SignalPin {
                    pin: "P1_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN16",
                pins: &[SignalPin {
                    pin: "P4_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN17",
                pins: &[SignalPin {
                    pin: "P4_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN18",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN19",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN2",
                pins: &[SignalPin {
                    pin: "P0_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN20",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN21",
                pins: &[SignalPin {
                    pin: "P2_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN22",
                pins: &[SignalPin {
                    pin: "P3_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN23",
                pins: &[SignalPin {
                    pin: "P3_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN24",
                pins: &[SignalPin {
                    pin: "P3_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN25",
                pins: &[SignalPin {
                    pin: "P3_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN26",
                pins: &[SignalPin {
                    pin: "P3_28",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN27",
                pins: &[SignalPin {
                    pin: "P3_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN3",
                pins: &[SignalPin {
                    pin: "P0_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN30",
                pins: &[SignalPin {
                    pin: "P3_27",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN4",
                pins: &[SignalPin {
                    pin: "P0_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN5",
                pins: &[SignalPin {
                    pin: "P0_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN6",
                pins: &[SignalPin {
                    pin: "P1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN7",
                pins: &[SignalPin {
                    pin: "P1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN8",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN9",
                pins: &[SignalPin {
                    pin: "P1_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "WUU0WakeUpEvent",
            mux: "DMA3",
            request: 1,
        }],
        gate: None,
    },
    Peripheral {
        name: "WWDT0",
        address: 0x4000C000,
        driver_name: "mcx/WWDT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: None,
            config: Some("Clk1MConfig"),
            bit: "wwdt0",
        }),
    },
    Peripheral {
        name: "XTAL48M",
        address: 0,
        driver_name: "",
        signals: &[Signal {
            name: "IN",
            pins: &[SignalPin {
                pin: "P1_30",
                alt: 0u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EDMA_0_TCD",
        address: 0x40081000,
        driver_name: "mcxa/EDMA_TCD::TCD8",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "MRCC0",
        address: 0x40091000,
        driver_name: "mcxa/MRCC2xx::MRCC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
];
pub const INTERRUPTS: &[(&str, u32)] = &[
    ("RESERVED16", 0u32),
    ("CMC", 1u32),
    ("DMA0_CH0", 2u32),
    ("DMA0_CH1", 3u32),
    ("DMA0_CH2", 4u32),
    ("DMA0_CH3", 5u32),
    ("DMA0_CH4", 6u32),
    ("DMA0_CH5", 7u32),
    ("DMA0_CH6", 8u32),
    ("DMA0_CH7", 9u32),
    ("ERM0_SINGLE_BIT", 10u32),
    ("ERM0_MULTI_BIT", 11u32),
    ("FMU0", 12u32),
    ("GLIKEY0", 13u32),
    ("MBC0", 14u32),
    ("SCG0", 15u32),
    ("SPC0", 16u32),
    ("TDET", 17u32),
    ("WUU0", 18u32),
    ("CAN0", 19u32),
    ("CAN1", 20u32),
    ("FLEXIO", 23u32),
    ("I3C0", 24u32),
    ("LPI2C0", 26u32),
    ("LPI2C1", 27u32),
    ("LPSPI0", 28u32),
    ("LPSPI1", 29u32),
    ("LPUART0", 31u32),
    ("LPUART1", 32u32),
    ("LPUART2", 33u32),
    ("LPUART3", 34u32),
    ("LPUART4", 35u32),
    ("USB0", 36u32),
    ("CDOG0", 38u32),
    ("CTIMER0", 39u32),
    ("CTIMER1", 40u32),
    ("CTIMER2", 41u32),
    ("CTIMER3", 42u32),
    ("CTIMER4", 43u32),
    ("FLEXPWM0_RELOAD_ERROR", 44u32),
    ("FLEXPWM0_FAULT", 45u32),
    ("FLEXPWM0_SUBMODULE0", 46u32),
    ("FLEXPWM0_SUBMODULE1", 47u32),
    ("FLEXPWM0_SUBMODULE2", 48u32),
    ("FLEXPWM0_SUBMODULE3", 49u32),
    ("EQDC0_COMPARE", 50u32),
    ("EQDC0_HOME", 51u32),
    ("EQDC0_WATCHDOG", 52u32),
    ("EQDC0_INDEX", 53u32),
    ("FREQME0", 54u32),
    ("LPTMR0", 55u32),
    ("OS_EVENT", 57u32),
    ("WAKETIMER0", 58u32),
    ("UTICK0", 59u32),
    ("WWDT0", 60u32),
    ("ADC0", 62u32),
    ("ADC1", 63u32),
    ("CMP0", 64u32),
    ("CMP1", 65u32),
    ("RESERVED82", 66u32),
    ("DAC0", 67u32),
    ("GPIO0", 71u32),
    ("GPIO1", 72u32),
    ("GPIO2", 73u32),
    ("GPIO3", 74u32),
    ("GPIO4", 75u32),
    ("LPI2C2", 77u32),
    ("LPI2C3", 78u32),
    ("FLEXPWM1_RELOAD_ERROR", 79u32),
    ("FLEXPWM1_FAULT", 80u32),
    ("FLEXPWM1_SUBMODULE0", 81u32),
    ("FLEXPWM1_SUBMODULE1", 82u32),
    ("FLEXPWM1_SUBMODULE2", 83u32),
    ("FLEXPWM1_SUBMODULE3", 84u32),
    ("EQDC1_COMPARE", 85u32),
    ("EQDC1_HOME", 86u32),
    ("EQDC1_WATCHDOG", 87u32),
    ("EQDC1_INDEX", 88u32),
    ("LPUART5", 95u32),
    ("MAU", 107u32),
    ("SMARTDMA", 108u32),
    ("CDOG1", 109u32),
    ("PKC", 110u32),
    ("SGI", 111u32),
    ("TRNG0", 113u32),
    ("SECURE_ERR", 114u32),
    ("RESERVED132", 116u32),
    ("RESERVED133", 117u32),
    ("RTC", 119u32),
    ("RTC_1HZ", 120u32),
    ("RESERVED137", 121u32),
];
