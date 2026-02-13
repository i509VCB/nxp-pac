use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "MIMXRT1011",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "GPIO_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806396u32),
            pad: 1075806572u32,
        }),
    },
    Pin {
        name: "GPIO_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806392u32),
            pad: 1075806568u32,
        }),
    },
    Pin {
        name: "GPIO_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806388u32),
            pad: 1075806564u32,
        }),
    },
    Pin {
        name: "GPIO_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806384u32),
            pad: 1075806560u32,
        }),
    },
    Pin {
        name: "GPIO_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806380u32),
            pad: 1075806556u32,
        }),
    },
    Pin {
        name: "GPIO_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806376u32),
            pad: 1075806552u32,
        }),
    },
    Pin {
        name: "GPIO_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806372u32),
            pad: 1075806548u32,
        }),
    },
    Pin {
        name: "GPIO_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806368u32),
            pad: 1075806544u32,
        }),
    },
    Pin {
        name: "GPIO_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806364u32),
            pad: 1075806540u32,
        }),
    },
    Pin {
        name: "GPIO_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806360u32),
            pad: 1075806536u32,
        }),
    },
    Pin {
        name: "GPIO_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806356u32),
            pad: 1075806532u32,
        }),
    },
    Pin {
        name: "GPIO_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806352u32),
            pad: 1075806528u32,
        }),
    },
    Pin {
        name: "GPIO_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806348u32),
            pad: 1075806524u32,
        }),
    },
    Pin {
        name: "GPIO_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806344u32),
            pad: 1075806520u32,
        }),
    },
    Pin {
        name: "GPIO_AD_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806280u32),
            pad: 1075806456u32,
        }),
    },
    Pin {
        name: "GPIO_AD_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806276u32),
            pad: 1075806452u32,
        }),
    },
    Pin {
        name: "GPIO_AD_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806272u32),
            pad: 1075806448u32,
        }),
    },
    Pin {
        name: "GPIO_AD_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806268u32),
            pad: 1075806444u32,
        }),
    },
    Pin {
        name: "GPIO_AD_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806264u32),
            pad: 1075806440u32,
        }),
    },
    Pin {
        name: "GPIO_AD_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806260u32),
            pad: 1075806436u32,
        }),
    },
    Pin {
        name: "GPIO_AD_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806256u32),
            pad: 1075806432u32,
        }),
    },
    Pin {
        name: "GPIO_AD_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806252u32),
            pad: 1075806428u32,
        }),
    },
    Pin {
        name: "GPIO_AD_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806248u32),
            pad: 1075806424u32,
        }),
    },
    Pin {
        name: "GPIO_AD_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806244u32),
            pad: 1075806420u32,
        }),
    },
    Pin {
        name: "GPIO_AD_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806240u32),
            pad: 1075806416u32,
        }),
    },
    Pin {
        name: "GPIO_AD_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806236u32),
            pad: 1075806412u32,
        }),
    },
    Pin {
        name: "GPIO_AD_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806232u32),
            pad: 1075806408u32,
        }),
    },
    Pin {
        name: "GPIO_AD_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806228u32),
            pad: 1075806404u32,
        }),
    },
    Pin {
        name: "GPIO_AD_14",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806224u32),
            pad: 1075806400u32,
        }),
    },
    Pin {
        name: "GPIO_SD_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806340u32),
            pad: 1075806516u32,
        }),
    },
    Pin {
        name: "GPIO_SD_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806336u32),
            pad: 1075806512u32,
        }),
    },
    Pin {
        name: "GPIO_SD_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806332u32),
            pad: 1075806508u32,
        }),
    },
    Pin {
        name: "GPIO_SD_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806328u32),
            pad: 1075806504u32,
        }),
    },
    Pin {
        name: "GPIO_SD_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806324u32),
            pad: 1075806500u32,
        }),
    },
    Pin {
        name: "GPIO_SD_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806320u32),
            pad: 1075806496u32,
        }),
    },
    Pin {
        name: "GPIO_SD_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806316u32),
            pad: 1075806492u32,
        }),
    },
    Pin {
        name: "GPIO_SD_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806312u32),
            pad: 1075806488u32,
        }),
    },
    Pin {
        name: "GPIO_SD_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806308u32),
            pad: 1075806484u32,
        }),
    },
    Pin {
        name: "GPIO_SD_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806304u32),
            pad: 1075806480u32,
        }),
    },
    Pin {
        name: "GPIO_SD_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806300u32),
            pad: 1075806476u32,
        }),
    },
    Pin {
        name: "GPIO_SD_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806296u32),
            pad: 1075806472u32,
        }),
    },
    Pin {
        name: "GPIO_SD_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806292u32),
            pad: 1075806468u32,
        }),
    },
    Pin {
        name: "GPIO_SD_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806288u32),
            pad: 1075806464u32,
        }),
    },
    Pin {
        name: "PMIC_ON_REQ",
        iomuxc: Some(PinIomuxc {
            mux: Some(1074429952u32),
            pad: 1074429968u32,
        }),
    },
    Pin {
        name: "ONOFF",
        iomuxc: Some(PinIomuxc {
            mux: None,
            pad: 1074429964u32,
        }),
    },
    Pin {
        name: "TEST_MODE",
        iomuxc: Some(PinIomuxc {
            mux: None,
            pad: 1074429956u32,
        }),
    },
    Pin {
        name: "POR_B",
        iomuxc: Some(PinIomuxc {
            mux: None,
            pad: 1074429960u32,
        }),
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "FLEXSPI1",
        signals: &[
            Signal {
                name: "A_DATA0",
                pins: &[SignalPin {
                    pin: "GPIO_SD_09",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA1",
                pins: &[SignalPin {
                    pin: "GPIO_SD_07",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA2",
                pins: &[SignalPin {
                    pin: "GPIO_SD_08",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DATA3",
                pins: &[SignalPin {
                    pin: "GPIO_SD_11",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SCLK",
                pins: &[SignalPin {
                    pin: "GPIO_SD_10",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SS0_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_06",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SS1_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_05",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DQS",
                pins: &[SignalPin {
                    pin: "GPIO_SD_12",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA0",
                pins: &[SignalPin {
                    pin: "GPIO_SD_03",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA1",
                pins: &[SignalPin {
                    pin: "GPIO_SD_01",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA2",
                pins: &[SignalPin {
                    pin: "GPIO_SD_02",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DATA3",
                pins: &[SignalPin {
                    pin: "GPIO_SD_04",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SCLK",
                pins: &[SignalPin {
                    pin: "GPIO_SD_13",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS0_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_00",
                    alt: 0u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS1_B",
                pins: &[SignalPin {
                    pin: "GPIO_11",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DQS",
                pins: &[SignalPin {
                    pin: "GPIO_00",
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
                    pin: "GPIO_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "GPIO_AD_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "GPIO_AD_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "GPIO_AD_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "GPIO_AD_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "GPIO_AD_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "GPIO_AD_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "GPIO_AD_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "GPIO_AD_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "GPIO_AD_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "GPIO_AD_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "GPIO_AD_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "GPIO_AD_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "GPIO_AD_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "GPIO_AD_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "GPIO_AD_14",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "GPIO2",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "GPIO_SD_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_SD_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_SD_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_SD_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_SD_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_SD_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_SD_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_SD_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_SD_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_SD_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_SD_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_SD_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_SD_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_SD_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "GPIO5",
        signals: &[Signal {
            name: "0",
            pins: &[SignalPin {
                pin: "PMIC_ON_REQ",
                alt: 5u8,
                iomuxc_daisy: None,
            }],
            iomuxc_daisy: None,
        }],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART1",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_10",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_12",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806708u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_09",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_11",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806704u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_08",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_07",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART2",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_00",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_10",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075806716u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_13",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_09",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806712u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_08",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_07",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART3",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_08",
                        alt: 3u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_12",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_08",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806724u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_07",
                        alt: 3u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_11",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_07",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806720u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_14",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_13",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART4",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_06",
                        alt: 3u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_02",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806732u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_05",
                        alt: 3u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_01",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075806728u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_14",
                    alt: 3u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_13",
                    alt: 3u8,
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
    "ADC1",
    "ADC_ETC_ERROR_IRQ",
    "ADC_ETC_IRQ0",
    "ADC_ETC_IRQ1",
    "ADC_ETC_IRQ2",
    "ADC_ETC_IRQ3",
    "CCM_1",
    "CCM_2",
    "CORE",
    "CSU",
    "CTI0_ERROR",
    "CTI1_ERROR",
    "DCDC",
    "DCP",
    "DCP_VMI",
    "DMA0",
    "DMA1",
    "DMA10",
    "DMA11",
    "DMA12",
    "DMA13",
    "DMA14",
    "DMA15",
    "DMA2",
    "DMA3",
    "DMA4",
    "DMA5",
    "DMA6",
    "DMA7",
    "DMA8",
    "DMA9",
    "DMA_ERROR",
    "EWM",
    "FLEXIO1",
    "FLEXRAM",
    "FLEXSPI",
    "GPC",
    "GPIO1_COMBINED_0_15",
    "GPIO1_COMBINED_16_31",
    "GPIO2_COMBINED_0_15",
    "GPIO5_COMBINED_0_15",
    "GPR_IRQ",
    "GPT1",
    "GPT2",
    "KPP",
    "LPI2C1",
    "LPI2C2",
    "LPSPI1",
    "LPSPI2",
    "LPUART1",
    "LPUART2",
    "LPUART3",
    "LPUART4",
    "PIT",
    "PMU",
    "PWM1_0",
    "PWM1_1",
    "PWM1_2",
    "PWM1_3",
    "PWM1_FAULT",
    "RESERVED68",
    "RESERVED70",
    "RESERVED71",
    "RTWDOG",
    "SAI1",
    "SAI3_RX",
    "SAI3_TX",
    "SNVS_HP_WRAPPER",
    "SNVS_HP_WRAPPER_TZ",
    "SNVS_LP_WRAPPER",
    "SPDIF",
    "SRC",
    "TEMP_LOW_HIGH",
    "TEMP_PANIC",
    "TRNG",
    "USB_OTG1",
    "USB_PHY",
    "WDOG1",
    "WDOG2",
    "XBAR1_IRQ_0_1_2_3",
];
