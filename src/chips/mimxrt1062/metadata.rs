pub mod iomuxc {
    pub struct IomuxcRegisters {
        pub name: &'static str,
        pub mux_ctl: u32,
        pub pad_ctl: u32,
    }
    pub const IOMUXC_REGISTERS: &[IomuxcRegisters] = &[
        IomuxcRegisters {
            name: "WAKEUP",
            mux_ctl: 1074429952,
            pad_ctl: 1074429976,
        },
        IomuxcRegisters {
            name: "PMIC_ON_REQ",
            mux_ctl: 1074429956,
            pad_ctl: 1074429980,
        },
        IomuxcRegisters {
            name: "PMIC_STBY_REQ",
            mux_ctl: 1074429960,
            pad_ctl: 1074429984,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_00",
            mux_ctl: 1075806228,
            pad_ctl: 1075806724,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_01",
            mux_ctl: 1075806232,
            pad_ctl: 1075806728,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_02",
            mux_ctl: 1075806236,
            pad_ctl: 1075806732,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_03",
            mux_ctl: 1075806240,
            pad_ctl: 1075806736,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_04",
            mux_ctl: 1075806244,
            pad_ctl: 1075806740,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_05",
            mux_ctl: 1075806248,
            pad_ctl: 1075806744,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_06",
            mux_ctl: 1075806252,
            pad_ctl: 1075806748,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_07",
            mux_ctl: 1075806256,
            pad_ctl: 1075806752,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_08",
            mux_ctl: 1075806260,
            pad_ctl: 1075806756,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_09",
            mux_ctl: 1075806264,
            pad_ctl: 1075806760,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_10",
            mux_ctl: 1075806268,
            pad_ctl: 1075806764,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_11",
            mux_ctl: 1075806272,
            pad_ctl: 1075806768,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_12",
            mux_ctl: 1075806276,
            pad_ctl: 1075806772,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_13",
            mux_ctl: 1075806280,
            pad_ctl: 1075806776,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_14",
            mux_ctl: 1075806284,
            pad_ctl: 1075806780,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_15",
            mux_ctl: 1075806288,
            pad_ctl: 1075806784,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_16",
            mux_ctl: 1075806292,
            pad_ctl: 1075806788,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_17",
            mux_ctl: 1075806296,
            pad_ctl: 1075806792,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_18",
            mux_ctl: 1075806300,
            pad_ctl: 1075806796,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_19",
            mux_ctl: 1075806304,
            pad_ctl: 1075806800,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_20",
            mux_ctl: 1075806308,
            pad_ctl: 1075806804,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_21",
            mux_ctl: 1075806312,
            pad_ctl: 1075806808,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_22",
            mux_ctl: 1075806316,
            pad_ctl: 1075806812,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_23",
            mux_ctl: 1075806320,
            pad_ctl: 1075806816,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_24",
            mux_ctl: 1075806324,
            pad_ctl: 1075806820,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_25",
            mux_ctl: 1075806328,
            pad_ctl: 1075806824,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_26",
            mux_ctl: 1075806332,
            pad_ctl: 1075806828,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_27",
            mux_ctl: 1075806336,
            pad_ctl: 1075806832,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_28",
            mux_ctl: 1075806340,
            pad_ctl: 1075806836,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_29",
            mux_ctl: 1075806344,
            pad_ctl: 1075806840,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_30",
            mux_ctl: 1075806348,
            pad_ctl: 1075806844,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_31",
            mux_ctl: 1075806352,
            pad_ctl: 1075806848,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_32",
            mux_ctl: 1075806356,
            pad_ctl: 1075806852,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_33",
            mux_ctl: 1075806360,
            pad_ctl: 1075806856,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_34",
            mux_ctl: 1075806364,
            pad_ctl: 1075806860,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_35",
            mux_ctl: 1075806368,
            pad_ctl: 1075806864,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_36",
            mux_ctl: 1075806372,
            pad_ctl: 1075806868,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_37",
            mux_ctl: 1075806376,
            pad_ctl: 1075806872,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_38",
            mux_ctl: 1075806380,
            pad_ctl: 1075806876,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_39",
            mux_ctl: 1075806384,
            pad_ctl: 1075806880,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_40",
            mux_ctl: 1075806388,
            pad_ctl: 1075806884,
        },
        IomuxcRegisters {
            name: "GPIO_EMC_41",
            mux_ctl: 1075806392,
            pad_ctl: 1075806888,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_00",
            mux_ctl: 1075806396,
            pad_ctl: 1075806892,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_01",
            mux_ctl: 1075806400,
            pad_ctl: 1075806896,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_02",
            mux_ctl: 1075806404,
            pad_ctl: 1075806900,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_03",
            mux_ctl: 1075806408,
            pad_ctl: 1075806904,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_04",
            mux_ctl: 1075806412,
            pad_ctl: 1075806908,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_05",
            mux_ctl: 1075806416,
            pad_ctl: 1075806912,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_06",
            mux_ctl: 1075806420,
            pad_ctl: 1075806916,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_07",
            mux_ctl: 1075806424,
            pad_ctl: 1075806920,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_08",
            mux_ctl: 1075806428,
            pad_ctl: 1075806924,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_09",
            mux_ctl: 1075806432,
            pad_ctl: 1075806928,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_10",
            mux_ctl: 1075806436,
            pad_ctl: 1075806932,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_11",
            mux_ctl: 1075806440,
            pad_ctl: 1075806936,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_12",
            mux_ctl: 1075806444,
            pad_ctl: 1075806940,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_13",
            mux_ctl: 1075806448,
            pad_ctl: 1075806944,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_14",
            mux_ctl: 1075806452,
            pad_ctl: 1075806948,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B0_15",
            mux_ctl: 1075806456,
            pad_ctl: 1075806952,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_00",
            mux_ctl: 1075806460,
            pad_ctl: 1075806956,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_01",
            mux_ctl: 1075806464,
            pad_ctl: 1075806960,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_02",
            mux_ctl: 1075806468,
            pad_ctl: 1075806964,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_03",
            mux_ctl: 1075806472,
            pad_ctl: 1075806968,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_04",
            mux_ctl: 1075806476,
            pad_ctl: 1075806972,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_05",
            mux_ctl: 1075806480,
            pad_ctl: 1075806976,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_06",
            mux_ctl: 1075806484,
            pad_ctl: 1075806980,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_07",
            mux_ctl: 1075806488,
            pad_ctl: 1075806984,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_08",
            mux_ctl: 1075806492,
            pad_ctl: 1075806988,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_09",
            mux_ctl: 1075806496,
            pad_ctl: 1075806992,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_10",
            mux_ctl: 1075806500,
            pad_ctl: 1075806996,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_11",
            mux_ctl: 1075806504,
            pad_ctl: 1075807000,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_12",
            mux_ctl: 1075806508,
            pad_ctl: 1075807004,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_13",
            mux_ctl: 1075806512,
            pad_ctl: 1075807008,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_14",
            mux_ctl: 1075806516,
            pad_ctl: 1075807012,
        },
        IomuxcRegisters {
            name: "GPIO_AD_B1_15",
            mux_ctl: 1075806520,
            pad_ctl: 1075807016,
        },
        IomuxcRegisters {
            name: "GPIO_B0_00",
            mux_ctl: 1075806524,
            pad_ctl: 1075807020,
        },
        IomuxcRegisters {
            name: "GPIO_B0_01",
            mux_ctl: 1075806528,
            pad_ctl: 1075807024,
        },
        IomuxcRegisters {
            name: "GPIO_B0_02",
            mux_ctl: 1075806532,
            pad_ctl: 1075807028,
        },
        IomuxcRegisters {
            name: "GPIO_B0_03",
            mux_ctl: 1075806536,
            pad_ctl: 1075807032,
        },
        IomuxcRegisters {
            name: "GPIO_B0_04",
            mux_ctl: 1075806540,
            pad_ctl: 1075807036,
        },
        IomuxcRegisters {
            name: "GPIO_B0_05",
            mux_ctl: 1075806544,
            pad_ctl: 1075807040,
        },
        IomuxcRegisters {
            name: "GPIO_B0_06",
            mux_ctl: 1075806548,
            pad_ctl: 1075807044,
        },
        IomuxcRegisters {
            name: "GPIO_B0_07",
            mux_ctl: 1075806552,
            pad_ctl: 1075807048,
        },
        IomuxcRegisters {
            name: "GPIO_B0_08",
            mux_ctl: 1075806556,
            pad_ctl: 1075807052,
        },
        IomuxcRegisters {
            name: "GPIO_B0_09",
            mux_ctl: 1075806560,
            pad_ctl: 1075807056,
        },
        IomuxcRegisters {
            name: "GPIO_B0_10",
            mux_ctl: 1075806564,
            pad_ctl: 1075807060,
        },
        IomuxcRegisters {
            name: "GPIO_B0_11",
            mux_ctl: 1075806568,
            pad_ctl: 1075807064,
        },
        IomuxcRegisters {
            name: "GPIO_B0_12",
            mux_ctl: 1075806572,
            pad_ctl: 1075807068,
        },
        IomuxcRegisters {
            name: "GPIO_B0_13",
            mux_ctl: 1075806576,
            pad_ctl: 1075807072,
        },
        IomuxcRegisters {
            name: "GPIO_B0_14",
            mux_ctl: 1075806580,
            pad_ctl: 1075807076,
        },
        IomuxcRegisters {
            name: "GPIO_B0_15",
            mux_ctl: 1075806584,
            pad_ctl: 1075807080,
        },
        IomuxcRegisters {
            name: "GPIO_B1_00",
            mux_ctl: 1075806588,
            pad_ctl: 1075807084,
        },
        IomuxcRegisters {
            name: "GPIO_B1_01",
            mux_ctl: 1075806592,
            pad_ctl: 1075807088,
        },
        IomuxcRegisters {
            name: "GPIO_B1_02",
            mux_ctl: 1075806596,
            pad_ctl: 1075807092,
        },
        IomuxcRegisters {
            name: "GPIO_B1_03",
            mux_ctl: 1075806600,
            pad_ctl: 1075807096,
        },
        IomuxcRegisters {
            name: "GPIO_B1_04",
            mux_ctl: 1075806604,
            pad_ctl: 1075807100,
        },
        IomuxcRegisters {
            name: "GPIO_B1_05",
            mux_ctl: 1075806608,
            pad_ctl: 1075807104,
        },
        IomuxcRegisters {
            name: "GPIO_B1_06",
            mux_ctl: 1075806612,
            pad_ctl: 1075807108,
        },
        IomuxcRegisters {
            name: "GPIO_B1_07",
            mux_ctl: 1075806616,
            pad_ctl: 1075807112,
        },
        IomuxcRegisters {
            name: "GPIO_B1_08",
            mux_ctl: 1075806620,
            pad_ctl: 1075807116,
        },
        IomuxcRegisters {
            name: "GPIO_B1_09",
            mux_ctl: 1075806624,
            pad_ctl: 1075807120,
        },
        IomuxcRegisters {
            name: "GPIO_B1_10",
            mux_ctl: 1075806628,
            pad_ctl: 1075807124,
        },
        IomuxcRegisters {
            name: "GPIO_B1_11",
            mux_ctl: 1075806632,
            pad_ctl: 1075807128,
        },
        IomuxcRegisters {
            name: "GPIO_B1_12",
            mux_ctl: 1075806636,
            pad_ctl: 1075807132,
        },
        IomuxcRegisters {
            name: "GPIO_B1_13",
            mux_ctl: 1075806640,
            pad_ctl: 1075807136,
        },
        IomuxcRegisters {
            name: "GPIO_B1_14",
            mux_ctl: 1075806644,
            pad_ctl: 1075807140,
        },
        IomuxcRegisters {
            name: "GPIO_B1_15",
            mux_ctl: 1075806648,
            pad_ctl: 1075807144,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B0_00",
            mux_ctl: 1075806652,
            pad_ctl: 1075807148,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B0_01",
            mux_ctl: 1075806656,
            pad_ctl: 1075807152,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B0_02",
            mux_ctl: 1075806660,
            pad_ctl: 1075807156,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B0_03",
            mux_ctl: 1075806664,
            pad_ctl: 1075807160,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B0_04",
            mux_ctl: 1075806668,
            pad_ctl: 1075807164,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B0_05",
            mux_ctl: 1075806672,
            pad_ctl: 1075807168,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_00",
            mux_ctl: 1075806676,
            pad_ctl: 1075807172,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_01",
            mux_ctl: 1075806680,
            pad_ctl: 1075807176,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_02",
            mux_ctl: 1075806684,
            pad_ctl: 1075807180,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_03",
            mux_ctl: 1075806688,
            pad_ctl: 1075807184,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_04",
            mux_ctl: 1075806692,
            pad_ctl: 1075807188,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_05",
            mux_ctl: 1075806696,
            pad_ctl: 1075807192,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_06",
            mux_ctl: 1075806700,
            pad_ctl: 1075807196,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_07",
            mux_ctl: 1075806704,
            pad_ctl: 1075807200,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_08",
            mux_ctl: 1075806708,
            pad_ctl: 1075807204,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_09",
            mux_ctl: 1075806712,
            pad_ctl: 1075807208,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_10",
            mux_ctl: 1075806716,
            pad_ctl: 1075807212,
        },
        IomuxcRegisters {
            name: "GPIO_SD_B1_11",
            mux_ctl: 1075806720,
            pad_ctl: 1075807216,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_00",
            mux_ctl: 1075807836,
            pad_ctl: 1075807924,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_01",
            mux_ctl: 1075807840,
            pad_ctl: 1075807928,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_02",
            mux_ctl: 1075807844,
            pad_ctl: 1075807932,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_03",
            mux_ctl: 1075807848,
            pad_ctl: 1075807936,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_04",
            mux_ctl: 1075807852,
            pad_ctl: 1075807940,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_05",
            mux_ctl: 1075807856,
            pad_ctl: 1075807944,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_06",
            mux_ctl: 1075807860,
            pad_ctl: 1075807948,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_07",
            mux_ctl: 1075807864,
            pad_ctl: 1075807952,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_08",
            mux_ctl: 1075807868,
            pad_ctl: 1075807956,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_09",
            mux_ctl: 1075807872,
            pad_ctl: 1075807960,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_10",
            mux_ctl: 1075807876,
            pad_ctl: 1075807964,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_11",
            mux_ctl: 1075807880,
            pad_ctl: 1075807968,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_12",
            mux_ctl: 1075807884,
            pad_ctl: 1075807972,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B0_13",
            mux_ctl: 1075807888,
            pad_ctl: 1075807976,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_00",
            mux_ctl: 1075807892,
            pad_ctl: 1075807980,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_01",
            mux_ctl: 1075807896,
            pad_ctl: 1075807984,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_02",
            mux_ctl: 1075807900,
            pad_ctl: 1075807988,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_03",
            mux_ctl: 1075807904,
            pad_ctl: 1075807992,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_04",
            mux_ctl: 1075807908,
            pad_ctl: 1075807996,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_05",
            mux_ctl: 1075807912,
            pad_ctl: 1075808000,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_06",
            mux_ctl: 1075807916,
            pad_ctl: 1075808004,
        },
        IomuxcRegisters {
            name: "GPIO_SPI_B1_07",
            mux_ctl: 1075807920,
            pad_ctl: 1075808008,
        },
    ];
}
