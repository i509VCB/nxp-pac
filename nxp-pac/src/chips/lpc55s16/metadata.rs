use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "LPC55S16",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "PIO0_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_24",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_25",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_26",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_27",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_28",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_29",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO0_31",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_0",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_1",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_2",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_3",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_4",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_5",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_6",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_7",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_8",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_9",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_10",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_11",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_12",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_13",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_14",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_15",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_16",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_17",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_18",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_19",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_20",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_21",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_22",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_23",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_24",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_25",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_26",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_27",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_28",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_29",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_30",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "PIO1_31",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "VREFP",
        iomuxc: None,
        feature: None,
    },
    Pin {
        name: "VREFN",
        iomuxc: None,
        feature: None,
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "DMA0",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "0",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "GPIO0",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "PIO0_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "PIO0_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "PIO0_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "PIO0_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "PIO0_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "PIO0_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "PIO0_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "PIO0_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "PIO0_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "PIO0_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "PIO0_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "PIO0_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "PIO0_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "PIO0_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "PIO0_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "PIO0_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "PIO0_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "PIO0_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "PIO0_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "PIO0_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "PIO0_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "PIO0_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "PIO0_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "PIO0_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "PIO0_24",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "PIO0_25",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "PIO0_26",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "PIO0_27",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "PIO0_28",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "PIO0_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "PIO0_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "PIO0_31",
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
        name: "GPIO1",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "PIO1_0",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "PIO1_1",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "PIO1_2",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "PIO1_3",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "PIO1_4",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "PIO1_5",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "PIO1_6",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "PIO1_7",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "PIO1_8",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "PIO1_9",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "PIO1_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "PIO1_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "PIO1_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "PIO1_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "PIO1_14",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "PIO1_15",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "PIO1_16",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "PIO1_17",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "PIO1_18",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "PIO1_19",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "PIO1_20",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "PIO1_21",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "PIO1_22",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "PIO1_23",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "PIO1_24",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "PIO1_25",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "PIO1_26",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "PIO1_27",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "PIO1_28",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "PIO1_29",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "PIO1_30",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "PIO1_31",
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
        name: "FLEXCOMM0",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM1",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM2",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM3",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM4",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM5",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM6",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "FLEXCOMM7",
        address: 0,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USART0",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "PIO0_25",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_30",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_6",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "PIO0_24",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_29",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_5",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS",
                pins: &[
                    SignalPin {
                        pin: "PIO1_0",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_7",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CTS",
                pins: &[
                    SignalPin {
                        pin: "PIO0_23",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_31",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_8",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[
                    SignalPin {
                        pin: "PIO0_28",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_4",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM0"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 4,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 5,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART1",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO1_11",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO1_10",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM1"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 6,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 7,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART2",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO0_27",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO1_24",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM2"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 10,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 11,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART3",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO0_2",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO0_3",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM3"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 8,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 9,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART4",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO0_16",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO0_5",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM4"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 12,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 13,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART5",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO0_9",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO0_8",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM5"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 14,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 15,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART6",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO1_16",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO1_13",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM6"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 16,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 17,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "USART7",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO0_19",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO0_20",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM7"),
        dma_muxing: &[
            DmaMux {
                signal: "RX",
                mux: "DMA0",
                request: 18,
            },
            DmaMux {
                signal: "TX",
                mux: "DMA0",
                request: 19,
            },
        ],
        gate: None,
    },
    Peripheral {
        name: "SPI0",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO1_6",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO1_5",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO1_4",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM0"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO1_11",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO1_10",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO1_9",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM1"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI2",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO1_25",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO1_24",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO1_23",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM2"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO0_2",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO0_3",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO0_0",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM3"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI4",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO1_20",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO1_21",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO1_19",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM4"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI5",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO0_9",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO0_8",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO0_7",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM5"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI6",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO1_16",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO1_13",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO1_12",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM6"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SPI7",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "MISO",
                pins: &[SignalPin {
                    pin: "PIO0_19",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "MOSI",
                pins: &[SignalPin {
                    pin: "PIO0_20",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "SCK",
                pins: &[SignalPin {
                    pin: "PIO0_21",
                    alt: 7u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: Some("FLEXCOMM7"),
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "SCT0",
        address: 0,
        driver_name: "",
        signals: &[
            Signal {
                name: "OUT0",
                pins: &[
                    SignalPin {
                        pin: "PIO0_2",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_17",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_4",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_23",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT1",
                pins: &[
                    SignalPin {
                        pin: "PIO0_3",
                        alt: 3u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_18",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_8",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_24",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT2",
                pins: &[
                    SignalPin {
                        pin: "PIO0_10",
                        alt: 5u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_15",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_19",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_9",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_25",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT3",
                pins: &[
                    SignalPin {
                        pin: "PIO0_22",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO0_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_10",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_26",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT4",
                pins: &[
                    SignalPin {
                        pin: "PIO0_23",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_3",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_17",
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
                        pin: "PIO0_26",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_18",
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
                        pin: "PIO0_27",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_31",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT7",
                pins: &[
                    SignalPin {
                        pin: "PIO0_28",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "PIO1_19",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT8",
                pins: &[SignalPin {
                    pin: "PIO0_29",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "OUT9",
                pins: &[SignalPin {
                    pin: "PIO0_30",
                    alt: 4u8,
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
        name: "USBHSD",
        address: 0x4009_4000,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "USB0",
        address: 0x4008_4000,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
    Peripheral {
        name: "HASHCRYPT",
        address: 0x400A_4000,
        driver_name: "",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
        gate: None,
    },
];
pub const INTERRUPTS: &[(&str, u32)] = &[
    ("WDT_BOD", 0u32),
    ("DMA0", 1u32),
    ("GINT0", 2u32),
    ("GINT1", 3u32),
    ("PIN_INT0", 4u32),
    ("PIN_INT1", 5u32),
    ("PIN_INT2", 6u32),
    ("PIN_INT3", 7u32),
    ("UTICK0", 8u32),
    ("MRT0", 9u32),
    ("CTIMER0", 10u32),
    ("CTIMER1", 11u32),
    ("SCT0", 12u32),
    ("CTIMER3", 13u32),
    ("FLEXCOMM0", 14u32),
    ("FLEXCOMM1", 15u32),
    ("FLEXCOMM2", 16u32),
    ("FLEXCOMM3", 17u32),
    ("FLEXCOMM4", 18u32),
    ("FLEXCOMM5", 19u32),
    ("FLEXCOMM6", 20u32),
    ("FLEXCOMM7", 21u32),
    ("ADC0", 22u32),
    ("ACMP", 24u32),
    ("USB0_NEEDCLK", 27u32),
    ("USB0", 28u32),
    ("RTC", 29u32),
    ("PIN_INT4", 32u32),
    ("PIN_INT5", 33u32),
    ("PIN_INT6", 34u32),
    ("PIN_INT7", 35u32),
    ("CTIMER2", 36u32),
    ("CTIMER4", 37u32),
    ("OS_EVENT", 38u32),
    ("CAN0_IRQ0", 43u32),
    ("CAN0_IRQ1", 44u32),
    ("USB1_PHY", 46u32),
    ("USB1", 47u32),
    ("USB1_NEEDCLK", 48u32),
    ("SEC_HYPERVISOR_CALL", 49u32),
    ("SEC_GPIO_INT0_IRQ0", 50u32),
    ("SEC_GPIO_INT0_IRQ1", 51u32),
    ("PLU", 52u32),
    ("SEC_VIO", 53u32),
    ("HASHCRYPT", 54u32),
    ("CASER", 55u32),
    ("PUF", 56u32),
    ("DMA1", 58u32),
    ("FLEXCOMM8", 59u32),
    ("CDOG", 60u32),
];
