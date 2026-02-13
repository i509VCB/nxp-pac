use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "LPC55S69_cm33_core1",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "PIO0_0",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_1",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_2",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_3",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_4",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_5",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_6",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_7",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_8",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_9",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_10",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_11",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_12",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_13",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_14",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_15",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_16",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_17",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_18",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_19",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_20",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_21",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_22",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_23",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_24",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_25",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_26",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_27",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_28",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_29",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_30",
        iomuxc: None,
    },
    Pin {
        name: "PIO0_31",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_0",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_1",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_2",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_3",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_4",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_5",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_6",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_7",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_8",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_9",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_10",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_11",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_12",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_13",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_14",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_15",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_16",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_17",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_18",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_19",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_20",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_21",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_22",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_23",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_24",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_25",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_26",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_27",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_28",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_29",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_30",
        iomuxc: None,
    },
    Pin {
        name: "PIO1_31",
        iomuxc: None,
    },
    Pin {
        name: "VREFP",
        iomuxc: None,
    },
    Pin {
        name: "VREFN",
        iomuxc: None,
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "DMA0",
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
    },
    Peripheral {
        name: "GPIO0",
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
    },
    Peripheral {
        name: "GPIO1",
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
    },
    Peripheral {
        name: "FLEXCOMM0",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM1",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM2",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM3",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM4",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM5",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM6",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXCOMM7",
        signals: &[],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "USART0",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "PIO1_6",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "PIO1_5",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
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
    },
    Peripheral {
        name: "USART1",
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
    },
    Peripheral {
        name: "USART2",
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
    },
    Peripheral {
        name: "USART3",
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
    },
    Peripheral {
        name: "USART4",
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
    },
    Peripheral {
        name: "USART5",
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
    },
    Peripheral {
        name: "USART6",
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
    },
    Peripheral {
        name: "USART7",
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
    },
    Peripheral {
        name: "SCT0",
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
    },
];
pub const INTERRUPTS: &[&str] = &[
    "ACMP",
    "ADC0",
    "CASER",
    "CTIMER0",
    "CTIMER1",
    "CTIMER2",
    "CTIMER3",
    "CTIMER4",
    "DMA0",
    "DMA1",
    "FLEXCOMM0",
    "FLEXCOMM1",
    "FLEXCOMM2",
    "FLEXCOMM3",
    "FLEXCOMM4",
    "FLEXCOMM5",
    "FLEXCOMM6",
    "FLEXCOMM7",
    "FLEXCOMM8",
    "GINT0",
    "GINT1",
    "HASHCRYPT",
    "MAILBOX",
    "MRT0",
    "OS_EVENT",
    "PIN_INT0",
    "PIN_INT1",
    "PIN_INT2",
    "PIN_INT3",
    "PIN_INT4",
    "PIN_INT5",
    "PIN_INT6",
    "PIN_INT7",
    "PLU",
    "PQ",
    "PUF",
    "RTC",
    "SCT0",
    "SDIO",
    "SEC_GPIO_INT0_IRQ0",
    "SEC_GPIO_INT0_IRQ1",
    "SEC_HYPERVISOR_CALL",
    "SEC_VIO",
    "USB0",
    "USB0_NEEDCLK",
    "USB1",
    "USB1_NEEDCLK",
    "USB1_PHY",
    "UTICK0",
    "WDT_BOD",
];
