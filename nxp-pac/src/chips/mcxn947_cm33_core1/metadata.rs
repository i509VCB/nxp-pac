use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "MCXN947_cm33_core1",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "P5_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P5_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_3",
        iomuxc: None,
        feature: None,
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
        feature: None,
    },
    Pin {
        name: "P0_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_11",
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
        name: "P0_28",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_29",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P0_31",
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
        name: "P1_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P1_31",
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
        name: "P3_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P3_23",
        iomuxc: None,
        feature: None,
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
        name: "P4_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "P4_23",
        iomuxc: None,
        feature: None,
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "SYSCON0",
        address: 0x40000000,
        driver_name: "mcxn/SYSCON",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PINT0",
        address: 0x40004000,
        driver_name: "mcxn/PINT0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "PINT0INT0",
                mux: "DMAMUX",
                request: 3,
            },
            DmaMux {
                signal: "PINT0INT1",
                mux: "DMAMUX",
                request: 4,
            },
            DmaMux {
                signal: "PINT0INT2",
                mux: "DMAMUX",
                request: 5,
            },
            DmaMux {
                signal: "PINT0INT3",
                mux: "DMAMUX",
                request: 6,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "INPUTMUX0",
        address: 0x40006000,
        driver_name: "mcxn/INPUTMUX0",
        signals: &[
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
                name: "IN2",
                pins: &[
                    SignalPin {
                        pin: "P1_20",
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
                name: "IN3",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_22",
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
                        pin: "P2_6",
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
                        pin: "P2_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
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
                        pin: "P4_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
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
                        pin: "P4_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_3",
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
                        pin: "P4_13",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_20",
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
                        pin: "P4_17",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_21",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN10",
                pins: &[
                    SignalPin {
                        pin: "P5_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P5_5",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN11",
                pins: &[
                    SignalPin {
                        pin: "P5_3",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P5_7",
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
                name: "OUT2",
                pins: &[
                    SignalPin {
                        pin: "P1_21",
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
                name: "OUT3",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_5",
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
                        pin: "P4_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_15",
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
                        pin: "P4_19",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_23",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT6",
                pins: &[
                    SignalPin {
                        pin: "P5_1",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P5_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT7",
                pins: &[
                    SignalPin {
                        pin: "P5_4",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P5_8",
                        alt: 1u8,
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
        name: "CTIMER0",
        address: 0x4000C000,
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
                        pin: "P0_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 4u8,
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
                        pin: "P0_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
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
                    SignalPin {
                        pin: "P0_30",
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
                    SignalPin {
                        pin: "P0_31",
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
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
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
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
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
                        pin: "P4_3",
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
                        pin: "P1_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
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
                        pin: "P1_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
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
                        pin: "P4_0",
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
                        pin: "P4_1",
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
                        pin: "P4_6",
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
                        pin: "P4_7",
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
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 4u8,
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
                        pin: "P0_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_11",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 4u8,
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
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MAT3",
                pins: &[
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
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "CTIMER0M0",
                mux: "DMAMUX",
                request: 7,
            },
            DmaMux {
                signal: "CTIMER0M1",
                mux: "DMAMUX",
                request: 8,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "CTIMER1",
        address: 0x4000D000,
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
                        pin: "P0_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 4u8,
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
                        pin: "P0_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
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
                    SignalPin {
                        pin: "P0_30",
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
                    SignalPin {
                        pin: "P0_31",
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
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
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
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
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
                        pin: "P4_3",
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
                        pin: "P1_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
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
                        pin: "P1_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
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
                        pin: "P4_0",
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
                        pin: "P4_1",
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
                        pin: "P4_6",
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
                        pin: "P4_7",
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
                name: "MAT0",
                pins: &[
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
                mux: "DMAMUX",
                request: 9,
            },
            DmaMux {
                signal: "CTIMER1M1",
                mux: "DMAMUX",
                request: 10,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "CTIMER2",
        address: 0x4000E000,
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
                        pin: "P0_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 4u8,
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
                        pin: "P0_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
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
                    SignalPin {
                        pin: "P0_30",
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
                    SignalPin {
                        pin: "P0_31",
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
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
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
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
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
                        pin: "P4_3",
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
                        pin: "P1_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
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
                        pin: "P1_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
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
                        pin: "P4_0",
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
                        pin: "P4_1",
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
                        pin: "P4_6",
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
                        pin: "P4_7",
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
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_20",
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
                        pin: "P4_21",
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
                        pin: "P4_22",
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
                        pin: "P4_23",
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
                mux: "DMAMUX",
                request: 11,
            },
            DmaMux {
                signal: "CTIMER2M1",
                mux: "DMAMUX",
                request: 12,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "CTIMER3",
        address: 0x4000F000,
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
                        pin: "P0_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 4u8,
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
                        pin: "P0_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
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
                    SignalPin {
                        pin: "P0_30",
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
                    SignalPin {
                        pin: "P0_31",
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
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
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
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
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
                        pin: "P4_3",
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
                        pin: "P1_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
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
                        pin: "P1_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
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
                        pin: "P4_0",
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
                        pin: "P4_1",
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
                        pin: "P4_6",
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
                        pin: "P4_7",
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
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_16",
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
                        pin: "P1_19",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_17",
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
                        pin: "P1_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_18",
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
                        pin: "P1_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_19",
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
                signal: "CTIMER3M0",
                mux: "DMAMUX",
                request: 13,
            },
            DmaMux {
                signal: "CTIMER3M1",
                mux: "DMAMUX",
                request: 14,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "CTIMER4",
        address: 0x40010000,
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
                        pin: "P0_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 4u8,
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
                        pin: "P0_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
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
                    SignalPin {
                        pin: "P0_30",
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
                    SignalPin {
                        pin: "P0_31",
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
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
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
                        pin: "P1_16",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
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
                        pin: "P4_3",
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
                        pin: "P1_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
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
                        pin: "P1_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
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
                        pin: "P4_0",
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
                        pin: "P4_1",
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
                        pin: "P4_6",
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
                        pin: "P4_7",
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
                name: "MAT0",
                pins: &[
                    SignalPin {
                        pin: "P4_12",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_2",
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
                        pin: "P4_13",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_3",
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
                        pin: "P4_14",
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
                        pin: "P4_15",
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
                mux: "DMAMUX",
                request: 15,
            },
            DmaMux {
                signal: "CTIMER4M1",
                mux: "DMAMUX",
                request: 16,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "FREQME0",
        address: 0x40011000,
        driver_name: "mcxn/FREQME",
        signals: &[
            Signal {
                name: "CLK_IN0",
                pins: &[
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
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "UTICK0",
        address: 0x40012000,
        driver_name: "mcxn/UTICK",
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
        name: "MRT0",
        address: 0x40013000,
        driver_name: "mcxn/MRT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "WWDT0",
        address: 0x40016000,
        driver_name: "mcx/WWDT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "WWDT1",
        address: 0x40017000,
        driver_name: "mcx/WWDT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CACHE64_CTRL0",
        address: 0x4001B000,
        driver_name: "mcxn/CACHE64Ctrl::CACHE64_CTRL",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CACHE64_POLSEL0",
        address: 0x4001B000,
        driver_name: "mcxn/CACHE64Polsel::CACHE64_POLSEL",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "I3C0",
        address: 0x40021000,
        driver_name: "mcxn/I3C",
        signals: &[
            Signal {
                name: "PUR",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
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
                        pin: "P0_17",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
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
                        pin: "P0_16",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
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
                mux: "DMAMUX",
                request: 95,
            },
            DmaMux {
                signal: "I3C0Tx",
                mux: "DMAMUX",
                request: 96,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "I3C1",
        address: 0x40022000,
        driver_name: "mcxn/I3C",
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
                        pin: "P1_15",
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
                        pin: "P1_17",
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
                        pin: "P1_16",
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
                signal: "I3C1Rx",
                mux: "DMAMUX",
                request: 97,
            },
            DmaMux {
                signal: "I3C1Tx",
                mux: "DMAMUX",
                request: 98,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GDET0",
        address: 0x40024000,
        driver_name: "mcxn/GDET",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GDET1",
        address: 0x40025000,
        driver_name: "mcxn/GDET",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ITRC0",
        address: 0x40026000,
        driver_name: "mcxn/ITRC0",
        signals: &[
            Signal {
                name: "TAMPER0",
                pins: &[SignalPin {
                    pin: "P5_2",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER1",
                pins: &[SignalPin {
                    pin: "P5_3",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER2",
                pins: &[SignalPin {
                    pin: "P5_4",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER3",
                pins: &[SignalPin {
                    pin: "P5_5",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER4",
                pins: &[SignalPin {
                    pin: "P5_6",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER5",
                pins: &[SignalPin {
                    pin: "P5_7",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER6",
                pins: &[SignalPin {
                    pin: "P5_8",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TAMPER7",
                pins: &[SignalPin {
                    pin: "P5_9",
                    alt: 3u8,
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
        name: "PKC0",
        address: 0x4002B000,
        driver_name: "mcxn/PKC0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF",
        address: 0x4002C000,
        driver_name: "mcxn/PUF",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_ALIAS1",
        address: 0x4002D000,
        driver_name: "mcxn/PUF",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_ALIAS2",
        address: 0x4002E000,
        driver_name: "mcxn/PUF",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_ALIAS3",
        address: 0x4002F000,
        driver_name: "mcxn/PUF",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_CTRL",
        address: 0x4002C000,
        driver_name: "mcxn/PUF_CTRL",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_CTRL_ALIAS1",
        address: 0x4002D000,
        driver_name: "mcxn/PUF_CTRL",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_CTRL_ALIAS2",
        address: 0x4002E000,
        driver_name: "mcxn/PUF_CTRL",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PUF_CTRL_ALIAS3",
        address: 0x4002F000,
        driver_name: "mcxn/PUF_CTRL",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "BSP32_0",
        address: 0x40032000,
        driver_name: "mcxn/BSP32",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SMARTDMA0",
        address: 0x40033000,
        driver_name: "mcxn/SMARTDMA",
        signals: &[
            Signal {
                name: "PIO0",
                pins: &[
                    SignalPin {
                        pin: "P3_0",
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
                name: "PIO2",
                pins: &[
                    SignalPin {
                        pin: "P3_2",
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
                name: "PIO3",
                pins: &[
                    SignalPin {
                        pin: "P3_3",
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
                name: "PIO4",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
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
                        pin: "P3_14",
                        alt: 7u8,
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
                        pin: "P3_15",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO16",
                pins: &[
                    SignalPin {
                        pin: "P1_20",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO17",
                pins: &[
                    SignalPin {
                        pin: "P1_21",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO18",
                pins: &[
                    SignalPin {
                        pin: "P1_22",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO19",
                pins: &[
                    SignalPin {
                        pin: "P1_23",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
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
                        pin: "P2_0",
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
                name: "PIO21",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
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
                name: "PIO22",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO23",
                pins: &[
                    SignalPin {
                        pin: "P2_3",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO24",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_0",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO25",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_1",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO26",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO27",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_3",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO28",
                pins: &[
                    SignalPin {
                        pin: "P2_8",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO29",
                pins: &[
                    SignalPin {
                        pin: "P2_9",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
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
                        pin: "P2_11",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_6",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PIO31",
                pins: &[
                    SignalPin {
                        pin: "P2_10",
                        alt: 7u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_7",
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
        name: "PLU0",
        address: 0x40034000,
        driver_name: "mcxn/PLU0",
        signals: &[
            Signal {
                name: "CLK",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN0",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_0",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN1",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_1",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN2",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN3",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_3",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN4",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN5",
                pins: &[
                    SignalPin {
                        pin: "P1_19",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT0",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_12",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT1",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_13",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT2",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_14",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT3",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_15",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT4",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_16",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT5",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_17",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT6",
                pins: &[
                    SignalPin {
                        pin: "P1_20",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_18",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT7",
                pins: &[
                    SignalPin {
                        pin: "P1_21",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_19",
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
        name: "GPIO5",
        address: 0x40040000,
        driver_name: "mcx/GPIO",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "P5_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "P5_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "P5_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "P5_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "P5_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "P5_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "P5_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "P5_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "P5_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "P5_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "GPIO5PinEvent0",
                mux: "DMAMUX",
                request: 118,
            },
            DmaMux {
                signal: "GPIO5PinEvent1",
                mux: "DMAMUX",
                request: 119,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO5_ALIAS1",
        address: 0x40041000,
        driver_name: "mcx/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PORT5",
        address: 0x40042000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FMU0",
        address: 0x40043000,
        driver_name: "mcxn/FMU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FMU0TEST",
        address: 0x40043000,
        driver_name: "mcxn/FMU0TEST",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCG0",
        address: 0x40044000,
        driver_name: "mcxn/SCG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPC0",
        address: 0x40045000,
        driver_name: "mcxn/SPC",
        signals: &[Signal {
            name: "LPREQ",
            pins: &[
                SignalPin {
                    pin: "P5_2",
                    alt: 2u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P5_4",
                    alt: 2u8,
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
        name: "WUU0",
        address: 0x40046000,
        driver_name: "mcxn/WUU",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "WUU0WakeUpEvent",
            mux: "DMAMUX",
            request: 17,
        }],
        gate: None,
    },
    Peripheral {
        name: "CMC0",
        address: 0x40048000,
        driver_name: "mcxn/CMC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OSTIMER0",
        address: 0x40049000,
        driver_name: "mcxn/OSTIMER",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPTMR0",
        address: 0x4004A000,
        driver_name: "mcxn/LPTMR",
        signals: &[Signal {
            name: "ALT2",
            pins: &[
                SignalPin {
                    pin: "P5_0",
                    alt: 2u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P5_5",
                    alt: 2u8,
                    iomuxc_daisy: None,
                },
            ],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "LPTMR0CounterMatchEvent",
            mux: "DMAMUX",
            request: 57,
        }],
        gate: None,
    },
    Peripheral {
        name: "LPTMR1",
        address: 0x4004B000,
        driver_name: "mcxn/LPTMR",
        signals: &[Signal {
            name: "ALT2",
            pins: &[
                SignalPin {
                    pin: "P5_1",
                    alt: 2u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P5_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                },
            ],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "LPTMR1CounterMatchEvent",
            mux: "DMAMUX",
            request: 58,
        }],
        gate: None,
    },
    Peripheral {
        name: "RTC0",
        address: 0x4004C000,
        driver_name: "mcx/RTC5xx::Rtc",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "RTC_SUBSYSTEM0",
        address: 0x4004C000,
        driver_name: "mcxn/RTC_SUBSYSTEM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TSI0",
        address: 0x40050000,
        driver_name: "mcxn/TSI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "TSI0EndOfScan",
                mux: "DMAMUX",
                request: 120,
            },
            DmaMux {
                signal: "TSI0OutOfRange",
                mux: "DMAMUX",
                request: 121,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "CMP0",
        address: 0x40051000,
        driver_name: "mcxn/CMP",
        signals: &[Signal {
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
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP0DmaRequest",
            mux: "DMAMUX",
            request: 28,
        }],
        gate: None,
    },
    Peripheral {
        name: "CMP1",
        address: 0x40052000,
        driver_name: "mcxn/CMP",
        signals: &[Signal {
            name: "OUT",
            pins: &[
                SignalPin {
                    pin: "P0_4",
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
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP1DmaRequest",
            mux: "DMAMUX",
            request: 29,
        }],
        gate: None,
    },
    Peripheral {
        name: "CMP2",
        address: 0x40053000,
        driver_name: "mcxn/CMP",
        signals: &[Signal {
            name: "OUT",
            pins: &[
                SignalPin {
                    pin: "P0_6",
                    alt: 8u8,
                    iomuxc_daisy: None,
                },
                SignalPin {
                    pin: "P0_11",
                    alt: 8u8,
                    iomuxc_daisy: None,
                },
            ],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "CMP2DmaRequest",
            mux: "DMAMUX",
            request: 30,
        }],
        gate: None,
    },
    Peripheral {
        name: "ELS",
        address: 0x40054000,
        driver_name: "mcxn/ELS",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ELS_ALIAS1",
        address: 0x40055000,
        driver_name: "mcxn/ELS",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ELS_ALIAS2",
        address: 0x40056000,
        driver_name: "mcxn/ELS",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ELS_ALIAS3",
        address: 0x40057000,
        driver_name: "mcxn/ELS",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TDET0",
        address: 0x40058000,
        driver_name: "mcxn/TDET",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "VBAT0",
        address: 0x40059000,
        driver_name: "mcxn/VBAT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EIM0",
        address: 0x4005B000,
        driver_name: "mcxn/EIM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "ERM0",
        address: 0x4005C000,
        driver_name: "mcxn/ERM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "INTM0",
        address: 0x4005D000,
        driver_name: "mcxn/INTM0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "DMA0",
        address: 0x40080000,
        driver_name: "mcxn/DMA::DMA16",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "DMA1",
        address: 0x400A0000,
        driver_name: "mcxn/DMA::DMA16",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EDMA_0_TCD",
        address: 0x40081000,
        driver_name: "mcxn/EDMA_TCD::TCD16",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCT0",
        address: 0x40091000,
        driver_name: "mcxn/SCT",
        signals: &[
            Signal {
                name: "IN0",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
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
                name: "IN1",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
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
                name: "IN2",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN3",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN4",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN5",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_11",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IN6",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
                        alt: 5u8,
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
                name: "IN7",
                pins: &[
                    SignalPin {
                        pin: "P1_19",
                        alt: 5u8,
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
                name: "OUT0",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT1",
                pins: &[
                    SignalPin {
                        pin: "P2_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT2",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT3",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_5",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT4",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_22",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_6",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT5",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_23",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P2_7",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT6",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
                        alt: 5u8,
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
                name: "OUT7",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
                        alt: 5u8,
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
                name: "OUT8",
                pins: &[
                    SignalPin {
                        pin: "P1_20",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_30",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT9",
                pins: &[
                    SignalPin {
                        pin: "P1_21",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_31",
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
                signal: "SCT0DmaRequest0",
                mux: "DMAMUX",
                request: 19,
            },
            DmaMux {
                signal: "SCT0DmaRequest1",
                mux: "DMAMUX",
                request: 20,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LPSPI0",
        address: 0x40092000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI1",
        address: 0x40093000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI2",
        address: 0x40094000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI3",
        address: 0x40095000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI4",
        address: 0x400B4000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI5",
        address: 0x400B5000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI6",
        address: 0x400B6000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI7",
        address: 0x400B7000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI8",
        address: 0x400B8000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPSPI9",
        address: 0x400B9000,
        driver_name: "mcxn/LPSPI",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40092000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40093000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART2",
        address: 0x40094000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART3",
        address: 0x40095000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART4",
        address: 0x400B4000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART5",
        address: 0x400B5000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART6",
        address: 0x400B6000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART7",
        address: 0x400B7000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART8",
        address: 0x400B8000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPUART9",
        address: 0x400B9000,
        driver_name: "mcxn/LPUART",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM0",
        address: 0x40092000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_12",
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
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P0_5",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_13",
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
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P0_7",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_15",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P0_8",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P0_9",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P0_10",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_30",
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
                signal: "LP_FLEXCOMM0Rx",
                mux: "DMAMUX",
                request: 69,
            },
            DmaMux {
                signal: "LP_FLEXCOMM0Tx",
                mux: "DMAMUX",
                request: 70,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM1",
        address: 0x40093000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P0_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_24",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P0_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_21",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_25",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P0_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_26",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P0_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_27",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_28",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P0_5",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_29",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_14",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_30",
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
                signal: "LP_FLEXCOMM1Rx",
                mux: "DMAMUX",
                request: 71,
            },
            DmaMux {
                signal: "LP_FLEXCOMM1Tx",
                mux: "DMAMUX",
                request: 72,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM2",
        address: 0x40094000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P4_0",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_12",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P4_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_13",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P4_2",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P4_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_17",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P4_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P4_5",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P4_6",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_23",
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
                signal: "LP_FLEXCOMM2Rx",
                mux: "DMAMUX",
                request: 73,
            },
            DmaMux {
                signal: "LP_FLEXCOMM2Tx",
                mux: "DMAMUX",
                request: 74,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM3",
        address: 0x40095000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
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
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P1_13",
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
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P1_15",
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
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
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
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
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
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LP_FLEXCOMM3Rx",
                mux: "DMAMUX",
                request: 75,
            },
            DmaMux {
                signal: "LP_FLEXCOMM3Tx",
                mux: "DMAMUX",
                request: 76,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM4",
        address: 0x400B4000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_21",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_22",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P1_11",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_23",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
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
                name: "P5",
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
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P1_14",
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
                signal: "LP_FLEXCOMM4Rx",
                mux: "DMAMUX",
                request: 77,
            },
            DmaMux {
                signal: "LP_FLEXCOMM4Tx",
                mux: "DMAMUX",
                request: 78,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM5",
        address: 0x400B5000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P1_16",
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
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P1_17",
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
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P1_18",
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
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P1_19",
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
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P1_8",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_20",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P1_9",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_21",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P1_10",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_22",
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
                signal: "LP_FLEXCOMM5Rx",
                mux: "DMAMUX",
                request: 79,
            },
            DmaMux {
                signal: "LP_FLEXCOMM5Tx",
                mux: "DMAMUX",
                request: 80,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM6",
        address: 0x400B6000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P3_20",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_1",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P3_21",
                        alt: 3u8,
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
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P3_22",
                        alt: 3u8,
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
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P3_23",
                        alt: 3u8,
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
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P3_12",
                        alt: 3u8,
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
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P3_13",
                        alt: 3u8,
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
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P3_18",
                        alt: 3u8,
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LP_FLEXCOMM6Rx",
                mux: "DMAMUX",
                request: 81,
            },
            DmaMux {
                signal: "LP_FLEXCOMM6Tx",
                mux: "DMAMUX",
                request: 82,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM7",
        address: 0x400B7000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[
                    SignalPin {
                        pin: "P3_8",
                        alt: 3u8,
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
                name: "P1",
                pins: &[
                    SignalPin {
                        pin: "P3_7",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_3",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[
                    SignalPin {
                        pin: "P3_9",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_4",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[
                    SignalPin {
                        pin: "P3_5",
                        alt: 2u8,
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
                name: "P4",
                pins: &[
                    SignalPin {
                        pin: "P3_12",
                        alt: 2u8,
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
                name: "P5",
                pins: &[
                    SignalPin {
                        pin: "P3_13",
                        alt: 2u8,
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
                name: "P6",
                pins: &[
                    SignalPin {
                        pin: "P3_19",
                        alt: 2u8,
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
                signal: "LP_FLEXCOMM7Rx",
                mux: "DMAMUX",
                request: 83,
            },
            DmaMux {
                signal: "LP_FLEXCOMM7Tx",
                mux: "DMAMUX",
                request: 84,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM8",
        address: 0x400B8000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[SignalPin {
                    pin: "P3_14",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[SignalPin {
                    pin: "P3_15",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[SignalPin {
                    pin: "P3_16",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[SignalPin {
                    pin: "P3_17",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
                pins: &[SignalPin {
                    pin: "P3_20",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[SignalPin {
                    pin: "P3_21",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[SignalPin {
                    pin: "P3_22",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LP_FLEXCOMM8Rx",
                mux: "DMAMUX",
                request: 85,
            },
            DmaMux {
                signal: "LP_FLEXCOMM8Tx",
                mux: "DMAMUX",
                request: 86,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LP_FLEXCOMM9",
        address: 0x400B9000,
        driver_name: "mcxn/LP_FLEXCOMM",
        signals: &[
            Signal {
                name: "P0",
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P1",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P2",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P3",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P4",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P5",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "P6",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "LP_FLEXCOMM9Rx",
                mux: "DMAMUX",
                request: 87,
            },
            DmaMux {
                signal: "LP_FLEXCOMM9Tx",
                mux: "DMAMUX",
                request: 88,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "LPI2C0",
        address: 0x40092800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C1",
        address: 0x40093800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C2",
        address: 0x40094800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C3",
        address: 0x40095800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C4",
        address: 0x400B4800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C5",
        address: 0x400B5800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C6",
        address: 0x400B6800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C7",
        address: 0x400B7800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C8",
        address: 0x400B8800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "LPI2C9",
        address: 0x400B9800,
        driver_name: "mcxn/LPI2C",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO0",
        address: 0x40096000,
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
                name: "8",
                pins: &[SignalPin {
                    pin: "P0_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "P0_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "P0_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "P0_11",
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
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "P0_28",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "P0_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "P0_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "P0_31",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "GPIO0PinEvent0",
                mux: "DMAMUX",
                request: 108,
            },
            DmaMux {
                signal: "GPIO0PinEvent1",
                mux: "DMAMUX",
                request: 109,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO0_ALIAS1",
        address: 0x40097000,
        driver_name: "mcx/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO1",
        address: 0x40098000,
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
                name: "20",
                pins: &[SignalPin {
                    pin: "P1_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "P1_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "P1_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "P1_23",
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
        dma_muxing: &[
            DmaMux {
                signal: "GPIO1PinEvent0",
                mux: "DMAMUX",
                request: 110,
            },
            DmaMux {
                signal: "GPIO1PinEvent1",
                mux: "DMAMUX",
                request: 111,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO1_ALIAS1",
        address: 0x40099000,
        driver_name: "mcx/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO2",
        address: 0x4009A000,
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "GPIO2PinEvent0",
                mux: "DMAMUX",
                request: 112,
            },
            DmaMux {
                signal: "GPIO2PinEvent1",
                mux: "DMAMUX",
                request: 113,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO2_ALIAS1",
        address: 0x4009B000,
        driver_name: "mcx/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO3",
        address: 0x4009C000,
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "GPIO3PinEvent0",
                mux: "DMAMUX",
                request: 114,
            },
            DmaMux {
                signal: "GPIO3PinEvent1",
                mux: "DMAMUX",
                request: 115,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO3_ALIAS1",
        address: 0x4009D000,
        driver_name: "mcx/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO4",
        address: 0x4009E000,
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
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "P4_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "P4_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "P4_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "P4_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "P4_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "P4_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "P4_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "P4_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "P4_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "P4_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "P4_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "P4_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "GPIO4PinEvent0",
                mux: "DMAMUX",
                request: 116,
            },
            DmaMux {
                signal: "GPIO4PinEvent1",
                mux: "DMAMUX",
                request: 117,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "GPIO4_ALIAS1",
        address: 0x4009F000,
        driver_name: "mcx/GPIO",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EDMA_1_TCD",
        address: 0x400A1000,
        driver_name: "mcxn/EDMA_TCD::TCD16",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SEMA42_0",
        address: 0x400B1000,
        driver_name: "mcxn/SEMA42",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "MAILBOX",
        address: 0x400B2000,
        driver_name: "mcxn/MAILBOX",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CDOG0",
        address: 0x400BB000,
        driver_name: "mcxn/CDOG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CDOG1",
        address: 0x400BC000,
        driver_name: "mcxn/CDOG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "DM0",
        address: 0x400BD000,
        driver_name: "mcxn/DM",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "POWERQUAD",
        address: 0x400BF000,
        driver_name: "mcxn/POWERQUAD",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EWM0",
        address: 0x400C0000,
        driver_name: "mcxn/EWM",
        signals: &[
            Signal {
                name: "IN",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_22",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT_b",
                pins: &[
                    SignalPin {
                        pin: "P0_5",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_19",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_23",
                        alt: 1u8,
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
        name: "CMX_PERFMON0",
        address: 0x400C1000,
        driver_name: "mcxn/CMX_PERFMON",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CMX_PERFMON1",
        address: 0x400C2000,
        driver_name: "mcxn/CMX_PERFMON",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "TRDC",
        address: 0x400C7000,
        driver_name: "mcxn/TRDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXSPI0",
        address: 0x400C8000,
        driver_name: "mcxn/FLEXSPI",
        signals: &[
            Signal {
                name: "A_DATA0",
                pins: &[SignalPin {
                    pin: "P3_8",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA1",
                pins: &[SignalPin {
                    pin: "P3_9",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA2",
                pins: &[SignalPin {
                    pin: "P3_10",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA3",
                pins: &[SignalPin {
                    pin: "P3_11",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA4",
                pins: &[SignalPin {
                    pin: "P3_12",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA5",
                pins: &[SignalPin {
                    pin: "P3_13",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA6",
                pins: &[SignalPin {
                    pin: "P3_14",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA7",
                pins: &[SignalPin {
                    pin: "P3_15",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DQS",
                pins: &[SignalPin {
                    pin: "P3_6",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SCLK",
                pins: &[SignalPin {
                    pin: "P3_7",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SS0_b",
                pins: &[SignalPin {
                    pin: "P3_0",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SS1_b",
                pins: &[SignalPin {
                    pin: "P3_1",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA0",
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA1",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA2",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA3",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA4",
                pins: &[SignalPin {
                    pin: "P2_8",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA5",
                pins: &[SignalPin {
                    pin: "P2_9",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA6",
                pins: &[SignalPin {
                    pin: "P2_10",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA7",
                pins: &[SignalPin {
                    pin: "P2_11",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DQS",
                pins: &[SignalPin {
                    pin: "P2_1",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SCLK",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS0_b",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS1_b",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXSPI0Rx",
                mux: "DMAMUX",
                request: 1,
            },
            DmaMux {
                signal: "FLEXSPI0Tx",
                mux: "DMAMUX",
                request: 2,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "OTPC0",
        address: 0x400C9000,
        driver_name: "mcxn/OTPC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "CRC0",
        address: 0x400CB000,
        driver_name: "mcxn/CRC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "NPX0",
        address: 0x400CC000,
        driver_name: "mcxn/NPX",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PWM0",
        address: 0x400CE000,
        driver_name: "mcxn/PWM",
        signals: &[
            Signal {
                name: "A0",
                pins: &[SignalPin {
                    pin: "P3_0",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A1",
                pins: &[SignalPin {
                    pin: "P3_6",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A2",
                pins: &[SignalPin {
                    pin: "P3_8",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A3",
                pins: &[SignalPin {
                    pin: "P3_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B0",
                pins: &[SignalPin {
                    pin: "P3_1",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B1",
                pins: &[SignalPin {
                    pin: "P3_7",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B2",
                pins: &[SignalPin {
                    pin: "P3_9",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B3",
                pins: &[SignalPin {
                    pin: "P3_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X0",
                pins: &[SignalPin {
                    pin: "P3_2",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X1",
                pins: &[SignalPin {
                    pin: "P3_3",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X2",
                pins: &[SignalPin {
                    pin: "P3_4",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "X3",
                pins: &[SignalPin {
                    pin: "P3_5",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FlexPWM0Mcapt0",
                mux: "DMAMUX",
                request: 39,
            },
            DmaMux {
                signal: "FlexPWM0Mcapt1",
                mux: "DMAMUX",
                request: 40,
            },
            DmaMux {
                signal: "FlexPWM0Mcapt2",
                mux: "DMAMUX",
                request: 41,
            },
            DmaMux {
                signal: "FlexPWM0Mcapt3",
                mux: "DMAMUX",
                request: 42,
            },
            DmaMux {
                signal: "FlexPWM0Mval0",
                mux: "DMAMUX",
                request: 43,
            },
            DmaMux {
                signal: "FlexPWM0Mval1",
                mux: "DMAMUX",
                request: 44,
            },
            DmaMux {
                signal: "FlexPWM0Mval2",
                mux: "DMAMUX",
                request: 45,
            },
            DmaMux {
                signal: "FlexPWM0Mval3",
                mux: "DMAMUX",
                request: 46,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "PWM1",
        address: 0x400D0000,
        driver_name: "mcxn/PWM",
        signals: &[
            Signal {
                name: "A0",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
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
            Signal {
                name: "A1",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
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
                name: "A2",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_16",
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
                        pin: "P2_0",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
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
                        pin: "P2_7",
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
                name: "B1",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_15",
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
                        pin: "P2_3",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_17",
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
                        pin: "P2_1",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
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
                        pin: "P2_8",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_18",
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
                        pin: "P2_9",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_19",
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
                        pin: "P2_10",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_22",
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
                        pin: "P2_11",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_23",
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
                signal: "FlexPWM1Mcapt0",
                mux: "DMAMUX",
                request: 47,
            },
            DmaMux {
                signal: "FlexPWM1Mcapt1",
                mux: "DMAMUX",
                request: 48,
            },
            DmaMux {
                signal: "FlexPWM1Mcapt2",
                mux: "DMAMUX",
                request: 49,
            },
            DmaMux {
                signal: "FlexPWM1Mcapt3",
                mux: "DMAMUX",
                request: 50,
            },
            DmaMux {
                signal: "FlexPWM1Mval0",
                mux: "DMAMUX",
                request: 51,
            },
            DmaMux {
                signal: "FlexPWM1Mval1",
                mux: "DMAMUX",
                request: 52,
            },
            DmaMux {
                signal: "FlexPWM1Mval2",
                mux: "DMAMUX",
                request: 53,
            },
            DmaMux {
                signal: "FlexPWM1Mval3",
                mux: "DMAMUX",
                request: 54,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "QDC0",
        address: 0x400CF000,
        driver_name: "mcxn/QDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "QDC1",
        address: 0x400D1000,
        driver_name: "mcxn/QDC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "EVTG0",
        address: 0x400D2000,
        driver_name: "mcxn/EVTG",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "EVTG0Out0A",
                mux: "DMAMUX",
                request: 31,
            },
            DmaMux {
                signal: "EVTG0Out0B",
                mux: "DMAMUX",
                request: 32,
            },
            DmaMux {
                signal: "EVTG0Out1A",
                mux: "DMAMUX",
                request: 33,
            },
            DmaMux {
                signal: "EVTG0Out1B",
                mux: "DMAMUX",
                request: 34,
            },
            DmaMux {
                signal: "EVTG0Out2A",
                mux: "DMAMUX",
                request: 35,
            },
            DmaMux {
                signal: "EVTG0Out2B",
                mux: "DMAMUX",
                request: 36,
            },
            DmaMux {
                signal: "EVTG0Out3A",
                mux: "DMAMUX",
                request: 37,
            },
            DmaMux {
                signal: "EVTG0Out3B",
                mux: "DMAMUX",
                request: 38,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "CAN0",
        address: 0x400D4000,
        driver_name: "mcxn/CAN",
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
                        pin: "P4_12",
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
                        pin: "P4_13",
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
            mux: "DMAMUX",
            request: 59,
        }],
        gate: None,
    },
    Peripheral {
        name: "CAN1",
        address: 0x400D8000,
        driver_name: "mcxn/CAN",
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
                        pin: "P1_21",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_15",
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
                        pin: "P1_20",
                        alt: 11u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_16",
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
            mux: "DMAMUX",
            request: 60,
        }],
        gate: None,
    },
    Peripheral {
        name: "USBDCD0",
        address: 0x400DC000,
        driver_name: "mcxn/USBDCD",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USBFS0",
        address: 0x400DD000,
        driver_name: "mcxn/USBFS",
        signals: &[Signal {
            name: "VBUS_DET",
            pins: &[SignalPin {
                pin: "P4_12",
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
        name: "ENET0",
        address: 0x40100000,
        driver_name: "mcxn/ENET",
        signals: &[
            Signal {
                name: "COL",
                pins: &[SignalPin {
                    pin: "P1_18",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CRS",
                pins: &[SignalPin {
                    pin: "P1_19",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MDC",
                pins: &[
                    SignalPin {
                        pin: "P1_20",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MDIO",
                pins: &[
                    SignalPin {
                        pin: "P1_21",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD0",
                pins: &[SignalPin {
                    pin: "P1_14",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD1",
                pins: &[SignalPin {
                    pin: "P1_15",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD2",
                pins: &[SignalPin {
                    pin: "P1_16",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD3",
                pins: &[SignalPin {
                    pin: "P1_17",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXDV",
                pins: &[SignalPin {
                    pin: "P1_13",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXER",
                pins: &[SignalPin {
                    pin: "P1_12",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RX_CLK",
                pins: &[SignalPin {
                    pin: "P1_11",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD0",
                pins: &[SignalPin {
                    pin: "P1_6",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD1",
                pins: &[SignalPin {
                    pin: "P1_7",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD2",
                pins: &[SignalPin {
                    pin: "P1_8",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD3",
                pins: &[SignalPin {
                    pin: "P1_9",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXEN",
                pins: &[SignalPin {
                    pin: "P1_5",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXER",
                pins: &[SignalPin {
                    pin: "P1_10",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TX_CLK",
                pins: &[SignalPin {
                    pin: "P1_4",
                    alt: 9u8,
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
        name: "EMVSIM0",
        address: 0x40103000,
        driver_name: "mcxn/EMVSIM",
        signals: &[
            Signal {
                name: "CLK",
                pins: &[
                    SignalPin {
                        pin: "P3_16",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IO",
                pins: &[
                    SignalPin {
                        pin: "P3_17",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PD",
                pins: &[
                    SignalPin {
                        pin: "P3_20",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RST",
                pins: &[
                    SignalPin {
                        pin: "P3_21",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "VCCEN",
                pins: &[
                    SignalPin {
                        pin: "P3_22",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "EMVSIM0Rx",
                mux: "DMAMUX",
                request: 91,
            },
            DmaMux {
                signal: "EMVSIM0Tx",
                mux: "DMAMUX",
                request: 92,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "EMVSIM1",
        address: 0x40104000,
        driver_name: "mcxn/EMVSIM",
        signals: &[
            Signal {
                name: "CLK",
                pins: &[SignalPin {
                    pin: "P3_4",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "IO",
                pins: &[SignalPin {
                    pin: "P3_5",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "PD",
                pins: &[SignalPin {
                    pin: "P3_2",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RST",
                pins: &[SignalPin {
                    pin: "P3_3",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "VCCEN",
                pins: &[SignalPin {
                    pin: "P3_6",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "EMVSIM1Rx",
                mux: "DMAMUX",
                request: 93,
            },
            DmaMux {
                signal: "EMVSIM1Tx",
                mux: "DMAMUX",
                request: 94,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "FLEXIO0",
        address: 0x40105000,
        driver_name: "mcxn/FLEXIO",
        signals: &[
            Signal {
                name: "D0",
                pins: &[
                    SignalPin {
                        pin: "P0_8",
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
                        pin: "P0_9",
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
                        pin: "P0_10",
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
                        pin: "P0_11",
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
                name: "D4",
                pins: &[
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
                name: "D10",
                pins: &[
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
                name: "D20",
                pins: &[
                    SignalPin {
                        pin: "P1_12",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_12",
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
                        pin: "P4_13",
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
                        pin: "P4_14",
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
                        pin: "P4_15",
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
                        pin: "P4_16",
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
                        pin: "P4_17",
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
                        pin: "P4_18",
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
                        pin: "P4_19",
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
                        pin: "P1_20",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_20",
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
                        pin: "P1_21",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_21",
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
                name: "D30",
                pins: &[
                    SignalPin {
                        pin: "P1_22",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_22",
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
                        pin: "P1_23",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_23",
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
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "FLEXIO0SR0",
                mux: "DMAMUX",
                request: 61,
            },
            DmaMux {
                signal: "FLEXIO0SR1",
                mux: "DMAMUX",
                request: 62,
            },
            DmaMux {
                signal: "FLEXIO0SR2",
                mux: "DMAMUX",
                request: 63,
            },
            DmaMux {
                signal: "FLEXIO0SR3",
                mux: "DMAMUX",
                request: 64,
            },
            DmaMux {
                signal: "FLEXIO0SR4",
                mux: "DMAMUX",
                request: 65,
            },
            DmaMux {
                signal: "FLEXIO0SR5",
                mux: "DMAMUX",
                request: 66,
            },
            DmaMux {
                signal: "FLEXIO0SR6",
                mux: "DMAMUX",
                request: 67,
            },
            DmaMux {
                signal: "FLEXIO0SR7",
                mux: "DMAMUX",
                request: 68,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "SAI0",
        address: 0x40106000,
        driver_name: "mcxn/SAI",
        signals: &[
            Signal {
                name: "MCLK",
                pins: &[
                    SignalPin {
                        pin: "P1_30",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_7",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD0",
                pins: &[
                    SignalPin {
                        pin: "P2_3",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_11",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD1",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_12",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_5",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RX_BCLK",
                pins: &[
                    SignalPin {
                        pin: "P2_0",
                        alt: 10u8,
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
                name: "RX_FS",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
                        alt: 10u8,
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
                name: "TXD0",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_10",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD1",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_13",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_4",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TX_BCLK",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_8",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TX_FS",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_9",
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
                signal: "SAI0Rx",
                mux: "DMAMUX",
                request: 99,
            },
            DmaMux {
                signal: "SAI0Tx",
                mux: "DMAMUX",
                request: 100,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x40107000,
        driver_name: "mcxn/SAI",
        signals: &[
            Signal {
                name: "MCLK",
                pins: &[
                    SignalPin {
                        pin: "P1_21",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_6",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD0",
                pins: &[
                    SignalPin {
                        pin: "P2_9",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_21",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_3",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD1",
                pins: &[
                    SignalPin {
                        pin: "P2_10",
                        alt: 10u8,
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
                name: "RX_BCLK",
                pins: &[
                    SignalPin {
                        pin: "P3_18",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_6",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RX_FS",
                pins: &[
                    SignalPin {
                        pin: "P3_19",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_7",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD0",
                pins: &[
                    SignalPin {
                        pin: "P2_8",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P3_20",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_2",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TXD1",
                pins: &[
                    SignalPin {
                        pin: "P2_11",
                        alt: 10u8,
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
                name: "TX_BCLK",
                pins: &[
                    SignalPin {
                        pin: "P3_16",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_0",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "TX_FS",
                pins: &[
                    SignalPin {
                        pin: "P3_17",
                        alt: 10u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P1_1",
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
                signal: "SAI1Rx",
                mux: "DMAMUX",
                request: 101,
            },
            DmaMux {
                signal: "SAI1Tx",
                mux: "DMAMUX",
                request: 102,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "SINC0",
        address: 0x40108000,
        driver_name: "mcxn/SINC",
        signals: &[
            Signal {
                name: "MBIT0",
                pins: &[
                    SignalPin {
                        pin: "P2_3",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_13",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MBIT1",
                pins: &[
                    SignalPin {
                        pin: "P2_5",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_17",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MBIT2",
                pins: &[
                    SignalPin {
                        pin: "P2_7",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_21",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MBIT3",
                pins: &[
                    SignalPin {
                        pin: "P2_9",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_2",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MBIT4",
                pins: &[
                    SignalPin {
                        pin: "P2_11",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_5",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK0",
                pins: &[
                    SignalPin {
                        pin: "P2_2",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_12",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK1",
                pins: &[
                    SignalPin {
                        pin: "P2_4",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_16",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK2",
                pins: &[
                    SignalPin {
                        pin: "P2_6",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_20",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK3",
                pins: &[
                    SignalPin {
                        pin: "P2_8",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_0",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK4",
                pins: &[
                    SignalPin {
                        pin: "P2_10",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_4",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK_OUT0",
                pins: &[
                    SignalPin {
                        pin: "P2_1",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P4_15",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK_OUT1",
                pins: &[SignalPin {
                    pin: "P4_19",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MCLK_OUT2",
                pins: &[SignalPin {
                    pin: "P4_23",
                    alt: 9u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "SINC0Ch0Request",
                mux: "DMAMUX",
                request: 103,
            },
            DmaMux {
                signal: "SINC0Ch1Request",
                mux: "DMAMUX",
                request: 104,
            },
            DmaMux {
                signal: "SINC0Ch2Request",
                mux: "DMAMUX",
                request: 105,
            },
            DmaMux {
                signal: "SINC0Ch3Request",
                mux: "DMAMUX",
                request: 106,
            },
            DmaMux {
                signal: "SINC0Ch4Request",
                mux: "DMAMUX",
                request: 107,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USDHC0",
        address: 0x40109000,
        driver_name: "mcxn/USDHC",
        signals: &[
            Signal {
                name: "CLK",
                pins: &[SignalPin {
                    pin: "P2_4",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CMD",
                pins: &[SignalPin {
                    pin: "P2_5",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D0",
                pins: &[SignalPin {
                    pin: "P2_3",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D1",
                pins: &[SignalPin {
                    pin: "P2_2",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D2",
                pins: &[SignalPin {
                    pin: "P2_7",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D3",
                pins: &[SignalPin {
                    pin: "P2_6",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D4",
                pins: &[SignalPin {
                    pin: "P2_1",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D5",
                pins: &[SignalPin {
                    pin: "P2_0",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D6",
                pins: &[SignalPin {
                    pin: "P2_9",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "D7",
                pins: &[SignalPin {
                    pin: "P2_8",
                    alt: 3u8,
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
        name: "USBPHY",
        address: 0x4010A000,
        driver_name: "mcxn/USBPHY",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USBHS1_PHY_DCD",
        address: 0x4010A800,
        driver_name: "mcxn/USBHS1_PHY_DCD",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USBHS1__USBC",
        address: 0x4010B000,
        driver_name: "mcxn/USBHS1__USBC::USBHS1_USBC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USBHS1__USBNC",
        address: 0x4010B200,
        driver_name: "mcxn/USBHS1__USBNC::USBHS1_USBNC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PDM",
        address: 0x4010C000,
        driver_name: "mcxn/PDM",
        signals: &[
            Signal {
                name: "CLK",
                pins: &[
                    SignalPin {
                        pin: "P0_4",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_16",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "DATA0",
                pins: &[
                    SignalPin {
                        pin: "P0_5",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_17",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "DATA1",
                pins: &[
                    SignalPin {
                        pin: "P0_6",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "P0_18",
                        alt: 9u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "PDMFifoRequest",
            mux: "DMAMUX",
            request: 18,
        }],
        gate: None,
    },
    Peripheral {
        name: "ADC0",
        address: 0x4010D000,
        driver_name: "mcxn/ADC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "ADC0FifoARequest",
                mux: "DMAMUX",
                request: 21,
            },
            DmaMux {
                signal: "ADC0FifoBRequest",
                mux: "DMAMUX",
                request: 22,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "ADC1",
        address: 0x4010E000,
        driver_name: "mcxn/ADC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[
            DmaMux {
                signal: "ADC1FifoARequest",
                mux: "DMAMUX",
                request: 23,
            },
            DmaMux {
                signal: "ADC1FifoBRequest",
                mux: "DMAMUX",
                request: 24,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "DAC0",
        address: 0x4010F000,
        driver_name: "mcx/DAC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "DAC0FifoRequest",
            mux: "DMAMUX",
            request: 25,
        }],
        gate: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x40112000,
        driver_name: "mcx/DAC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "DAC1FifoRequest",
            mux: "DMAMUX",
            request: 26,
        }],
        gate: None,
    },
    Peripheral {
        name: "OPAMP0",
        address: 0x40110000,
        driver_name: "mcxn/OPAMP",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OPAMP1",
        address: 0x40113000,
        driver_name: "mcxn/OPAMP",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "OPAMP2",
        address: 0x40115000,
        driver_name: "mcxn/OPAMP",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "VREF0",
        address: 0x40111000,
        driver_name: "mcxn/VREF",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "DAC2",
        address: 0x40114000,
        driver_name: "mcxn/DAC2",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[DmaMux {
            signal: "DAC2FifoRequest",
            mux: "DMAMUX",
            request: 27,
        }],
        gate: None,
    },
    Peripheral {
        name: "PORT0",
        address: 0x40116000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PORT1",
        address: 0x40117000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PORT2",
        address: 0x40118000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PORT3",
        address: 0x40119000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "PORT4",
        address: 0x4011A000,
        driver_name: "mcx/PORT",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "AHBSC",
        address: 0x40120000,
        driver_name: "mcxn/AHBSC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "AHBSC_ALIAS1",
        address: 0x40121000,
        driver_name: "mcxn/AHBSC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "AHBSC_ALIAS2",
        address: 0x40122000,
        driver_name: "mcxn/AHBSC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "AHBSC_ALIAS3",
        address: 0x40123000,
        driver_name: "mcxn/AHBSC",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCnSCB",
        address: 0xE000E000,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SysTick0",
        address: 0xE000E010,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "NVIC",
        address: 0xE000E100,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCB",
        address: 0xE000ED00,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "MPU",
        address: 0xE000ED90,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SAU",
        address: 0xE000EDD0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
];
pub const INTERRUPTS: &[(&str, u32)] = &[
    ("ETB0", 149u32),
    ("CTI0", 155u32),
    ("PINT0", 47u32),
    ("CTIMER0", 31u32),
    ("CTIMER1", 32u32),
    ("CTIMER2", 34u32),
    ("CTIMER3", 55u32),
    ("CTIMER4", 56u32),
    ("FREQME", 71u32),
    ("UTICK0", 29u32),
    ("MRT0", 30u32),
    ("WWDT0", 152u32),
    ("WWDT1", 153u32),
    ("I3C0", 95u32),
    ("I3C1", 96u32),
    ("GDET", 98u32),
    ("ITRC0", 132u32),
    ("PKC", 74u32),
    ("PKC_ERR", 135u32),
    ("PUF", 75u32),
    ("BSP32", 133u32),
    ("SMARTDMA", 53u32),
    ("PLU", 70u32),
    ("GPIO50", 27u32),
    ("GPIO51", 28u32),
    ("PORT_EFT", 148u32),
    ("FMU0", 138u32),
    ("SCG", 145u32),
    ("SPC", 146u32),
    ("WUU", 147u32),
    ("CMC0", 154u32),
    ("OS_EVENT", 57u32),
    ("LPTMR0", 143u32),
    ("LPTMR1", 144u32),
    ("RTC", 52u32),
    ("TSI_END_OF_SCAN", 101u32),
    ("TSI_OUT_OF_SCAN", 102u32),
    ("HSCMP0", 109u32),
    ("HSCMP1", 110u32),
    ("HSCMP2", 111u32),
    ("ELS", 73u32),
    ("ELS_ERR", 134u32),
    ("VBAT0", 99u32),
    ("ERM_SINGLE_BIT_ERROR", 136u32),
    ("ERM_MULTI_BIT_ERROR", 137u32),
    ("EDMA_0_CH0", 1u32),
    ("EDMA_0_CH1", 2u32),
    ("EDMA_0_CH2", 3u32),
    ("EDMA_0_CH3", 4u32),
    ("EDMA_0_CH4", 5u32),
    ("EDMA_0_CH5", 6u32),
    ("EDMA_0_CH6", 7u32),
    ("EDMA_0_CH7", 8u32),
    ("EDMA_0_CH8", 9u32),
    ("EDMA_0_CH9", 10u32),
    ("EDMA_0_CH10", 11u32),
    ("EDMA_0_CH11", 12u32),
    ("EDMA_0_CH12", 13u32),
    ("EDMA_0_CH13", 14u32),
    ("EDMA_0_CH14", 15u32),
    ("EDMA_0_CH15", 16u32),
    ("EDMA_1_CH0", 77u32),
    ("EDMA_1_CH1", 78u32),
    ("EDMA_1_CH2", 79u32),
    ("EDMA_1_CH3", 80u32),
    ("EDMA_1_CH4", 81u32),
    ("EDMA_1_CH5", 82u32),
    ("EDMA_1_CH6", 83u32),
    ("EDMA_1_CH7", 84u32),
    ("EDMA_1_CH8", 85u32),
    ("EDMA_1_CH9", 86u32),
    ("EDMA_1_CH10", 87u32),
    ("EDMA_1_CH11", 88u32),
    ("EDMA_1_CH12", 89u32),
    ("EDMA_1_CH13", 90u32),
    ("EDMA_1_CH14", 91u32),
    ("EDMA_1_CH15", 92u32),
    ("SCT0", 33u32),
    ("LP_FLEXCOMM0", 35u32),
    ("LP_FLEXCOMM1", 36u32),
    ("LP_FLEXCOMM2", 37u32),
    ("LP_FLEXCOMM3", 38u32),
    ("LP_FLEXCOMM4", 39u32),
    ("LP_FLEXCOMM5", 40u32),
    ("LP_FLEXCOMM6", 41u32),
    ("LP_FLEXCOMM7", 42u32),
    ("LP_FLEXCOMM8", 43u32),
    ("LP_FLEXCOMM9", 44u32),
    ("GPIO00", 17u32),
    ("GPIO01", 18u32),
    ("GPIO10", 19u32),
    ("GPIO11", 20u32),
    ("GPIO20", 21u32),
    ("GPIO21", 22u32),
    ("GPIO30", 23u32),
    ("GPIO31", 24u32),
    ("GPIO40", 25u32),
    ("GPIO41", 26u32),
    ("MAILBOX", 54u32),
    ("CDOG0", 93u32),
    ("CDOG1", 94u32),
    ("PQ", 76u32),
    ("EWM0", 100u32),
    ("SEC_VIO", 72u32),
    ("FLEXSPI0", 58u32),
    ("FLEXPWM0_RELOAD_ERROR", 112u32),
    ("FLEXPWM0_FAULT", 113u32),
    ("FLEXPWM0_SUBMODULE0", 114u32),
    ("FLEXPWM0_SUBMODULE1", 115u32),
    ("FLEXPWM0_SUBMODULE2", 116u32),
    ("FLEXPWM0_SUBMODULE3", 117u32),
    ("FLEXPWM1_RELOAD_ERROR", 118u32),
    ("FLEXPWM1_FAULT", 119u32),
    ("FLEXPWM1_SUBMODULE0", 120u32),
    ("FLEXPWM1_SUBMODULE1", 121u32),
    ("FLEXPWM1_SUBMODULE2", 122u32),
    ("FLEXPWM1_SUBMODULE3", 123u32),
    ("QDC0_COMPARE", 124u32),
    ("QDC0_HOME", 125u32),
    ("QDC0_WDG_SAB", 126u32),
    ("QDC0_IDX", 127u32),
    ("QDC1_COMPARE", 128u32),
    ("QDC1_HOME", 129u32),
    ("QDC1_WDG_SAB", 130u32),
    ("QDC1_IDX", 131u32),
    ("CAN0", 62u32),
    ("CAN1", 63u32),
    ("USB0_DCD", 51u32),
    ("USB0_FS", 50u32),
    ("ETHERNET", 139u32),
    ("ETHERNET_PMT", 140u32),
    ("ETHERNET_MACLP", 141u32),
    ("EMVSIM0", 103u32),
    ("EMVSIM1", 104u32),
    ("FLEXIO", 105u32),
    ("SAI0", 59u32),
    ("SAI1", 60u32),
    ("SINC_FILTER", 142u32),
    ("USDHC0", 61u32),
    ("USB1_HS_PHY", 66u32),
    ("USB1_HS", 67u32),
    ("PDM_EVENT", 48u32),
    ("ADC0", 45u32),
    ("ADC1", 46u32),
    ("DAC0", 106u32),
    ("DAC1", 107u32),
    ("DAC2", 108u32),
    ("SEC_HYPERVISOR_CALL", 68u32),
    ("OR", 0u32),
    ("RESERVED65", 49u32),
    ("RESERVED80", 64u32),
    ("RESERVED81", 65u32),
    ("RESERVED85", 69u32),
    ("NPU", 97u32),
    ("RESERVED166", 150u32),
    ("RESERVED167", 151u32),
];
