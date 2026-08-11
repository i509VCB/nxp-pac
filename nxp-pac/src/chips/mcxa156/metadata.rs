use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "MCXA156",
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
        name: "P2_23",
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
        name: "P0_6",
        iomuxc: None,
        feature: Some("jtag-extras-as-gpio"),
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
        name: "INPUTMUX0",
        address: 0x40001000,
        driver_name: "mcxa/INPUTMUX",
        signals: &[
            Signal {
                name: "OUT2",
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
                name: "IN3",
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
                name: "OUT3",
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
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN4",
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
                name: "IN5",
                pins: &[
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
                        pin: "P3_10",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN6",
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
                name: "IN7",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
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
                name: "OUT4",
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
                name: "OUT5",
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
                name: "IN8",
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
                name: "IN9",
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
                name: "IN10",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT6",
                pins: &[SignalPin {
                    pin: "P3_30",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN11",
                pins: &[SignalPin {
                    pin: "P3_28",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT7",
                pins: &[SignalPin {
                    pin: "P3_27",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT1",
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
                name: "OUT0",
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
                name: "IN2",
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
                name: "IN1",
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
                name: "IN0",
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
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc0",
            reset: Some("mrcc_glb_rst0"),
            config: None,
        }),
    },
    Peripheral {
        name: "I3C0",
        address: 0x40002000,
        driver_name: "mcxa/I3C",
        signals: &[
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "I3C0Rx",
                mux: "DMA0",
                request: 7,
            },
            DmaMux {
                signal: "I3C0Tx",
                mux: "DMA0",
                request: 8,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("I3cConfig"),
        }),
    },
    Peripheral {
        name: "CTIMER0",
        address: 0x40004000,
        driver_name: "mcxa/CTIMER",
        signals: &[
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
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
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                    pin: "P1_15",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P2_1",
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
                name: "INP12",
                pins: &[
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
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P0_23",
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
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
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
                        pin: "P0_18",
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
                        pin: "P0_19",
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
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER0M0",
                mux: "DMA0",
                request: 31,
            },
            DmaMux {
                signal: "CTIMER0M1",
                mux: "DMA0",
                request: 32,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
        }),
    },
    Peripheral {
        name: "CTIMER1",
        address: 0x40005000,
        driver_name: "mcxa/CTIMER",
        signals: &[
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
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
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                    pin: "P1_15",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P2_1",
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
                name: "INP12",
                pins: &[
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
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P0_23",
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
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
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
                mux: "DMA1",
                request: 33,
            },
            DmaMux {
                signal: "CTIMER1M1",
                mux: "DMA1",
                request: 34,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
        }),
    },
    Peripheral {
        name: "CTIMER2",
        address: 0x40006000,
        driver_name: "mcxa/CTIMER",
        signals: &[
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
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
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                    pin: "P1_15",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P2_1",
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
                name: "INP12",
                pins: &[
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
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P0_23",
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
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
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
                        pin: "P2_1",
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
                mux: "DMA2",
                request: 35,
            },
            DmaMux {
                signal: "CTIMER2M1",
                mux: "DMA2",
                request: 36,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
        }),
    },
    Peripheral {
        name: "CTIMER3",
        address: 0x40007000,
        driver_name: "mcxa/CTIMER",
        signals: &[
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
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
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                    pin: "P1_15",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P2_1",
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
                name: "INP12",
                pins: &[
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
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P0_23",
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
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
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
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 5u8,
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
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
        }),
    },
    Peripheral {
        name: "CTIMER4",
        address: 0x40008000,
        driver_name: "mcxa/CTIMER",
        signals: &[
            Signal {
                name: "INP8",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
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
                        pin: "P3_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP10",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                    pin: "P1_15",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P2_1",
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
                name: "INP12",
                pins: &[
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
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP15",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP18",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "INP19",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
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
                        pin: "P0_23",
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
                name: "INP2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
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
                mux: "DMA4",
                request: 39,
            },
            DmaMux {
                signal: "CTIMER4M1",
                mux: "DMA4",
                request: 40,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("CTimerConfig"),
        }),
    },
    Peripheral {
        name: "FREQME0",
        address: 0x40009000,
        driver_name: "mcxa/FREQME",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "UTICK0",
        address: 0x4000B000,
        driver_name: "mcxa/UTICK",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "WWDT0",
        address: 0x4000C000,
        driver_name: "mcxa/WWDT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: None,
            config: Some("Clk1MConfig"),
        }),
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
        }),
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
        }),
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
        name: "EIM0",
        address: 0x4008C000,
        driver_name: "mcxa/EIM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ERM0",
        address: 0x4008D000,
        driver_name: "mcxa/ERM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "MBC0",
        address: 0x4008E000,
        driver_name: "mcxa/MBC",
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
        name: "SPC0",
        address: 0x40090000,
        driver_name: "mcxa/SPC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "MRCC0",
        address: 0x40091000,
        driver_name: "mcxa/MRCC1xx::MRCC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SYSCON",
        address: 0x40091000,
        driver_name: "mcxa/SYSCON1xx::SYSCON",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GLIKEY0",
        address: 0x40091D00,
        driver_name: "mcxa/GLIKEY",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "WUU0",
        address: 0x40092000,
        driver_name: "mcxa/WUU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "WUU0WakeUpEvent",
            mux: "DMA0",
            request: 1,
        }],
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
        name: "FMC0",
        address: 0x40094000,
        driver_name: "mcxa/FMC",
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
        name: "FLEXIO0",
        address: 0x40099000,
        driver_name: "mcxa/FLEXIO",
        signals: &[
            Signal {
                name: "D16",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
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
                name: "D30",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
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
                name: "D8",
                pins: &[
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
                        pin: "P2_1",
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
            Signal {
                name: "D24",
                pins: &[
                    SignalPin {
                        pin: "P2_16",
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
                        pin: "P2_17",
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
                name: "D27",
                pins: &[
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
                name: "D26",
                pins: &[SignalPin {
                    pin: "P3_18",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
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
                name: "D6",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
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
                name: "D4",
                pins: &[SignalPin {
                    pin: "P0_20",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D5",
                pins: &[SignalPin {
                    pin: "P0_21",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D7",
                pins: &[SignalPin {
                    pin: "P0_23",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXIO0SR0",
                mux: "DMA0",
                request: 71,
            },
            DmaMux {
                signal: "FLEXIO0SR1",
                mux: "DMA0",
                request: 72,
            },
            DmaMux {
                signal: "FLEXIO0SR2",
                mux: "DMA0",
                request: 73,
            },
            DmaMux {
                signal: "FLEXIO0SR3",
                mux: "DMA0",
                request: 74,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LPI2C0",
        address: 0x4009A000,
        driver_name: "mcxa/LPI2C",
        signals: &[
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
                name: "HREQ",
                pins: &[SignalPin {
                    pin: "P0_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
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
                mux: "DMA0",
                request: 11,
            },
            DmaMux {
                signal: "LPI2C0Tx",
                mux: "DMA0",
                request: 12,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("Lpi2cConfig"),
        }),
    },
    Peripheral {
        name: "LPI2C1",
        address: 0x4009B000,
        driver_name: "mcxa/LPI2C",
        signals: &[
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
                mux: "DMA1",
                request: 13,
            },
            DmaMux {
                signal: "LPI2C1Tx",
                mux: "DMA1",
                request: 14,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("Lpi2cConfig"),
        }),
    },
    Peripheral {
        name: "LPI2C2",
        address: 0x400D4000,
        driver_name: "mcxa/LPI2C",
        signals: &[
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
                name: "HREQ",
                pins: &[SignalPin {
                    pin: "P4_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPI2C2Rx",
                mux: "DMA2",
                request: 3,
            },
            DmaMux {
                signal: "LPI2C2Tx",
                mux: "DMA2",
                request: 4,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("Lpi2cConfig"),
        }),
    },
    Peripheral {
        name: "LPI2C3",
        address: 0x400D5000,
        driver_name: "mcxa/LPI2C",
        signals: &[
            Signal {
                name: "SDAS",
                pins: &[SignalPin {
                    pin: "P3_31",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
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
                name: "HREQ",
                pins: &[SignalPin {
                    pin: "P3_29",
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPSPI0Rx",
                mux: "DMA0",
                request: 15,
            },
            DmaMux {
                signal: "LPSPI0Tx",
                mux: "DMA0",
                request: 16,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpspiConfig"),
        }),
    },
    Peripheral {
        name: "LPSPI1",
        address: 0x4009D000,
        driver_name: "mcxa/LPSPI",
        signals: &[
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPSPI1Rx",
                mux: "DMA1",
                request: 17,
            },
            DmaMux {
                signal: "LPSPI1Tx",
                mux: "DMA1",
                request: 18,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpspiConfig"),
        }),
    },
    Peripheral {
        name: "LPUART0",
        address: 0x4009F000,
        driver_name: "mcxa/LPUART",
        signals: &[
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
                        pin: "P2_1",
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART0Rx",
                mux: "DMA0",
                request: 21,
            },
            DmaMux {
                signal: "LPUART0Tx",
                mux: "DMA0",
                request: 22,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
        }),
    },
    Peripheral {
        name: "LPUART1",
        address: 0x400A0000,
        driver_name: "mcxa/LPUART",
        signals: &[
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
                        pin: "P3_11",
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
                mux: "DMA1",
                request: 23,
            },
            DmaMux {
                signal: "LPUART1Tx",
                mux: "DMA1",
                request: 24,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
        }),
    },
    Peripheral {
        name: "LPUART2",
        address: 0x400A1000,
        driver_name: "mcxa/LPUART",
        signals: &[
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART2Rx",
                mux: "DMA2",
                request: 25,
            },
            DmaMux {
                signal: "LPUART2Tx",
                mux: "DMA2",
                request: 26,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
        }),
    },
    Peripheral {
        name: "LPUART3",
        address: 0x400A2000,
        driver_name: "mcxa/LPUART",
        signals: &[
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
        }),
    },
    Peripheral {
        name: "LPUART4",
        address: 0x400A3000,
        driver_name: "mcxa/LPUART",
        signals: &[
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
                        pin: "P2_1",
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LPUART4Rx",
                mux: "DMA4",
                request: 29,
            },
            DmaMux {
                signal: "LPUART4Tx",
                mux: "DMA4",
                request: 30,
            },
        ],
        gate: Some(Gate {
            enable: "mrcc_glb_acc0",
            reset: Some("mrcc_glb_rst0"),
            config: Some("LpuartConfig"),
        }),
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
        name: "QDC0",
        address: 0x400A7000,
        driver_name: "mcxa/QDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "QDC0",
            mux: "DMA0",
            request: 65,
        }],
        gate: None,
    },
    Peripheral {
        name: "QDC1",
        address: 0x400A8000,
        driver_name: "mcxa/QDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "QDC1",
            mux: "DMA1",
            request: 66,
        }],
        gate: None,
    },
    Peripheral {
        name: "FLEX_PWM0",
        address: 0x400A9000,
        driver_name: "mcxa/FLEXPWM",
        signals: &[
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
                name: "A0",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
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
                name: "B0",
                pins: &[
                    SignalPin {
                        pin: "P4_7",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
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
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXPWM0capt0",
                mux: "DMA0",
                request: 41,
            },
            DmaMux {
                signal: "FLEXPWM0capt1",
                mux: "DMA0",
                request: 42,
            },
            DmaMux {
                signal: "FLEXPWM0capt2",
                mux: "DMA0",
                request: 43,
            },
            DmaMux {
                signal: "FLEXPWM0val0",
                mux: "DMA0",
                request: 45,
            },
            DmaMux {
                signal: "FLEXPWM0val1",
                mux: "DMA0",
                request: 46,
            },
            DmaMux {
                signal: "FLEXPWM0val2",
                mux: "DMA0",
                request: 47,
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
                name: "B0",
                pins: &[
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
                name: "A0",
                pins: &[
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
                name: "B1",
                pins: &[SignalPin {
                    pin: "P3_15",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
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
                name: "B2",
                pins: &[SignalPin {
                    pin: "P3_13",
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXPWM1capt0",
                mux: "DMA1",
                request: 79,
            },
            DmaMux {
                signal: "FLEXPWM1capt1",
                mux: "DMA1",
                request: 80,
            },
            DmaMux {
                signal: "FLEXPWM1capt2",
                mux: "DMA1",
                request: 81,
            },
            DmaMux {
                signal: "FLEXPWM1val0",
                mux: "DMA1",
                request: 83,
            },
            DmaMux {
                signal: "FLEXPWM1val1",
                mux: "DMA1",
                request: 84,
            },
            DmaMux {
                signal: "FLEXPWM1val2",
                mux: "DMA1",
                request: 85,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LPTMR0",
        address: 0x400AB000,
        driver_name: "mcxa/LPTMR",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "LPTMR0CounterMatchEvent",
            mux: "DMA0",
            request: 49,
        }],
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
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("OsTimerConfig"),
        }),
    },
    Peripheral {
        name: "WAKETIMER0",
        address: 0x400AE000,
        driver_name: "mcxa/WAKETIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
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
                    pin: "P2_1",
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
                name: "A7",
                pins: &[SignalPin {
                    pin: "P2_7",
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
                name: "A2",
                pins: &[SignalPin {
                    pin: "P2_15",
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
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC0FifoRequest",
            mux: "DMA0",
            request: 51,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("AdcConfig"),
        }),
    },
    Peripheral {
        name: "ADC1",
        address: 0x400B0000,
        driver_name: "mcxa/ADC",
        signals: &[
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
                name: "A4",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A0",
                pins: &[SignalPin {
                    pin: "P2_4",
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
                name: "A3",
                pins: &[SignalPin {
                    pin: "P2_6",
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
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "ADC1FifoRequest",
            mux: "DMA1",
            request: 52,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: Some("AdcConfig"),
        }),
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
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP0",
            mux: "DMA0",
            request: 53,
        }],
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
                name: "OUT",
                pins: &[
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
                name: "IN2",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP1",
            mux: "DMA1",
            request: 54,
        }],
        gate: None,
    },
    Peripheral {
        name: "DAC0",
        address: 0x400B4000,
        driver_name: "mcxa/DAC",
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
            mux: "DMA0",
            request: 56,
        }],
        gate: None,
    },
    Peripheral {
        name: "OPAMP0",
        address: 0x400B7000,
        driver_name: "mcxa/OPAMP",
        signals: &[
            Signal {
                name: "INP0",
                pins: &[SignalPin {
                    pin: "P2_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
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
        name: "PORT0",
        address: 0x400BC000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "PORT1",
        address: 0x400BD000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "PORT2",
        address: 0x400BE000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "PORT3",
        address: 0x400BF000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "PORT4",
        address: 0x400C0000,
        driver_name: "mcxa/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: Some(Gate {
            enable: "mrcc_glb_cc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "CAN0",
        address: 0x400CC000,
        driver_name: "mcxa/CAN",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_13",
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
                    SignalPin {
                        pin: "P1_6",
                        alt: 11u8,
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
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_12",
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
                    SignalPin {
                        pin: "P1_7",
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
            mux: "DMA0",
            request: 2,
        }],
        gate: None,
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
        name: "DBGMAILBOX",
        address: 0x40101000,
        driver_name: "mcxa/DBGMAILBOX",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO0",
        address: 0x40102000,
        driver_name: "mcxa/GPIO",
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
                name: "6",
                pins: &[SignalPin {
                    pin: "P0_6",
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
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO0PinEvent0",
            mux: "DMA0",
            request: 60,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "GPIO1",
        address: 0x40103000,
        driver_name: "mcxa/GPIO",
        signals: &[
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
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO1PinEvent0",
            mux: "DMA1",
            request: 61,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "GPIO2",
        address: 0x40104000,
        driver_name: "mcxa/GPIO",
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
                name: "23",
                pins: &[SignalPin {
                    pin: "P2_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "GPIO2PinEvent0",
            mux: "DMA2",
            request: 62,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "GPIO3",
        address: 0x40105000,
        driver_name: "mcxa/GPIO",
        signals: &[
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "P3_31",
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
                name: "29",
                pins: &[SignalPin {
                    pin: "P3_29",
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
                name: "27",
                pins: &[SignalPin {
                    pin: "P3_27",
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
                name: "21",
                pins: &[SignalPin {
                    pin: "P3_21",
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
                name: "19",
                pins: &[SignalPin {
                    pin: "P3_19",
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
                name: "17",
                pins: &[SignalPin {
                    pin: "P3_17",
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
                name: "15",
                pins: &[SignalPin {
                    pin: "P3_15",
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
                name: "13",
                pins: &[SignalPin {
                    pin: "P3_13",
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
                name: "11",
                pins: &[SignalPin {
                    pin: "P3_11",
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
                name: "9",
                pins: &[SignalPin {
                    pin: "P3_9",
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
                name: "7",
                pins: &[SignalPin {
                    pin: "P3_7",
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
                name: "2",
                pins: &[SignalPin {
                    pin: "P3_2",
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
                name: "0",
                pins: &[SignalPin {
                    pin: "P3_0",
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
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "GPIO4",
        address: 0x40106000,
        driver_name: "mcxa/GPIO",
        signals: &[
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
            mux: "DMA4",
            request: 64,
        }],
        gate: Some(Gate {
            enable: "mrcc_glb_acc1",
            reset: Some("mrcc_glb_rst1"),
            config: None,
        }),
    },
    Peripheral {
        name: "SCnSCB",
        address: 0xE000E000,
        driver_name: "mcxa/SCnSCB",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SysTick",
        address: 0xE000E010,
        driver_name: "mcxa/SysTick",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "NVIC",
        address: 0xE000E100,
        driver_name: "mcxa/NVIC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCB",
        address: 0xE000ED00,
        driver_name: "mcxa/SCB",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SAU",
        address: 0xE000EDD0,
        driver_name: "mcxa/SAU",
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
    ("VBAT0", 17u32),
    ("WUU0", 18u32),
    ("CAN0", 19u32),
    ("RESERVED36", 20u32),
    ("RESERVED37", 21u32),
    ("RESERVED38", 22u32),
    ("FLEXIO", 23u32),
    ("I3C0", 24u32),
    ("RESERVED41", 25u32),
    ("LPI2C0", 26u32),
    ("LPI2C1", 27u32),
    ("LPSPI0", 28u32),
    ("LPSPI1", 29u32),
    ("RESERVED46", 30u32),
    ("LPUART0", 31u32),
    ("LPUART1", 32u32),
    ("LPUART2", 33u32),
    ("LPUART3", 34u32),
    ("LPUART4", 35u32),
    ("USB0", 36u32),
    ("RESERVED53", 37u32),
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
    ("RESERVED65", 49u32),
    ("QDC0_COMPARE", 50u32),
    ("QDC0_HOME", 51u32),
    ("QDC0_WATCHDOG", 52u32),
    ("QDC0_INDEX", 53u32),
    ("FREQME0", 54u32),
    ("LPTMR0", 55u32),
    ("RESERVED72", 56u32),
    ("OS_EVENT", 57u32),
    ("WAKETIMER0", 58u32),
    ("UTICK0", 59u32),
    ("WWDT0", 60u32),
    ("RESERVED77", 61u32),
    ("ADC0", 62u32),
    ("ADC1", 63u32),
    ("CMP0", 64u32),
    ("CMP1", 65u32),
    ("RESERVED82", 66u32),
    ("DAC0", 67u32),
    ("RESERVED84", 68u32),
    ("RESERVED85", 69u32),
    ("RESERVED86", 70u32),
    ("GPIO0", 71u32),
    ("GPIO1", 72u32),
    ("GPIO2", 73u32),
    ("GPIO3", 74u32),
    ("GPIO4", 75u32),
    ("RESERVED92", 76u32),
    ("LPI2C2", 77u32),
    ("LPI2C3", 78u32),
    ("FLEXPWM1_RELOAD_ERROR", 79u32),
    ("FLEXPWM1_FAULT", 80u32),
    ("FLEXPWM1_SUBMODULE0", 81u32),
    ("FLEXPWM1_SUBMODULE1", 82u32),
    ("FLEXPWM1_SUBMODULE2", 83u32),
    ("RESERVED100", 84u32),
    ("QDC1_COMPARE", 85u32),
    ("QDC1_HOME", 86u32),
    ("QDC1_WATCHDOG", 87u32),
    ("QDC1_INDEX", 88u32),
];
