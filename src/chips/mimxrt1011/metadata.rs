pub mod iomuxc {
    pub struct IomuxcRegisters {
        pub name: &'static str,
        pub mux_ctl: u32,
        pub pad_ctl: u32,
    }
    pub const IOMUXC_REGISTERS: &[IomuxcRegisters] = &[
        IomuxcRegisters {
            name: "PMIC_ON_REQ",
            mux_ctl: 1074429952,
            pad_ctl: 1074429968,
        },
        IomuxcRegisters {
            name: "GPIO_AD_14",
            mux_ctl: 1075806224,
            pad_ctl: 1075806400,
        },
        IomuxcRegisters {
            name: "GPIO_AD_13",
            mux_ctl: 1075806228,
            pad_ctl: 1075806404,
        },
        IomuxcRegisters {
            name: "GPIO_AD_12",
            mux_ctl: 1075806232,
            pad_ctl: 1075806408,
        },
        IomuxcRegisters {
            name: "GPIO_AD_11",
            mux_ctl: 1075806236,
            pad_ctl: 1075806412,
        },
        IomuxcRegisters {
            name: "GPIO_AD_10",
            mux_ctl: 1075806240,
            pad_ctl: 1075806416,
        },
        IomuxcRegisters {
            name: "GPIO_AD_09",
            mux_ctl: 1075806244,
            pad_ctl: 1075806420,
        },
        IomuxcRegisters {
            name: "GPIO_AD_08",
            mux_ctl: 1075806248,
            pad_ctl: 1075806424,
        },
        IomuxcRegisters {
            name: "GPIO_AD_07",
            mux_ctl: 1075806252,
            pad_ctl: 1075806428,
        },
        IomuxcRegisters {
            name: "GPIO_AD_06",
            mux_ctl: 1075806256,
            pad_ctl: 1075806432,
        },
        IomuxcRegisters {
            name: "GPIO_AD_05",
            mux_ctl: 1075806260,
            pad_ctl: 1075806436,
        },
        IomuxcRegisters {
            name: "GPIO_AD_04",
            mux_ctl: 1075806264,
            pad_ctl: 1075806440,
        },
        IomuxcRegisters {
            name: "GPIO_AD_03",
            mux_ctl: 1075806268,
            pad_ctl: 1075806444,
        },
        IomuxcRegisters {
            name: "GPIO_AD_02",
            mux_ctl: 1075806272,
            pad_ctl: 1075806448,
        },
        IomuxcRegisters {
            name: "GPIO_AD_01",
            mux_ctl: 1075806276,
            pad_ctl: 1075806452,
        },
        IomuxcRegisters {
            name: "GPIO_AD_00",
            mux_ctl: 1075806280,
            pad_ctl: 1075806456,
        },
        IomuxcRegisters {
            name: "GPIO_SD_14",
            mux_ctl: 1075806284,
            pad_ctl: 1075806460,
        },
        IomuxcRegisters {
            name: "GPIO_SD_13",
            mux_ctl: 1075806288,
            pad_ctl: 1075806464,
        },
        IomuxcRegisters {
            name: "GPIO_SD_12",
            mux_ctl: 1075806292,
            pad_ctl: 1075806468,
        },
        IomuxcRegisters {
            name: "GPIO_SD_11",
            mux_ctl: 1075806296,
            pad_ctl: 1075806472,
        },
        IomuxcRegisters {
            name: "GPIO_SD_10",
            mux_ctl: 1075806300,
            pad_ctl: 1075806476,
        },
        IomuxcRegisters {
            name: "GPIO_SD_09",
            mux_ctl: 1075806304,
            pad_ctl: 1075806480,
        },
        IomuxcRegisters {
            name: "GPIO_SD_08",
            mux_ctl: 1075806308,
            pad_ctl: 1075806484,
        },
        IomuxcRegisters {
            name: "GPIO_SD_07",
            mux_ctl: 1075806312,
            pad_ctl: 1075806488,
        },
        IomuxcRegisters {
            name: "GPIO_SD_06",
            mux_ctl: 1075806316,
            pad_ctl: 1075806492,
        },
        IomuxcRegisters {
            name: "GPIO_SD_05",
            mux_ctl: 1075806320,
            pad_ctl: 1075806496,
        },
        IomuxcRegisters {
            name: "GPIO_SD_04",
            mux_ctl: 1075806324,
            pad_ctl: 1075806500,
        },
        IomuxcRegisters {
            name: "GPIO_SD_03",
            mux_ctl: 1075806328,
            pad_ctl: 1075806504,
        },
        IomuxcRegisters {
            name: "GPIO_SD_02",
            mux_ctl: 1075806332,
            pad_ctl: 1075806508,
        },
        IomuxcRegisters {
            name: "GPIO_SD_01",
            mux_ctl: 1075806336,
            pad_ctl: 1075806512,
        },
        IomuxcRegisters {
            name: "GPIO_SD_00",
            mux_ctl: 1075806340,
            pad_ctl: 1075806516,
        },
        IomuxcRegisters {
            name: "GPIO_13",
            mux_ctl: 1075806344,
            pad_ctl: 1075806520,
        },
        IomuxcRegisters {
            name: "GPIO_12",
            mux_ctl: 1075806348,
            pad_ctl: 1075806524,
        },
        IomuxcRegisters {
            name: "GPIO_11",
            mux_ctl: 1075806352,
            pad_ctl: 1075806528,
        },
        IomuxcRegisters {
            name: "GPIO_10",
            mux_ctl: 1075806356,
            pad_ctl: 1075806532,
        },
        IomuxcRegisters {
            name: "GPIO_09",
            mux_ctl: 1075806360,
            pad_ctl: 1075806536,
        },
        IomuxcRegisters {
            name: "GPIO_08",
            mux_ctl: 1075806364,
            pad_ctl: 1075806540,
        },
        IomuxcRegisters {
            name: "GPIO_07",
            mux_ctl: 1075806368,
            pad_ctl: 1075806544,
        },
        IomuxcRegisters {
            name: "GPIO_06",
            mux_ctl: 1075806372,
            pad_ctl: 1075806548,
        },
        IomuxcRegisters {
            name: "GPIO_05",
            mux_ctl: 1075806376,
            pad_ctl: 1075806552,
        },
        IomuxcRegisters {
            name: "GPIO_04",
            mux_ctl: 1075806380,
            pad_ctl: 1075806556,
        },
        IomuxcRegisters {
            name: "GPIO_03",
            mux_ctl: 1075806384,
            pad_ctl: 1075806560,
        },
        IomuxcRegisters {
            name: "GPIO_02",
            mux_ctl: 1075806388,
            pad_ctl: 1075806564,
        },
        IomuxcRegisters {
            name: "GPIO_01",
            mux_ctl: 1075806392,
            pad_ctl: 1075806568,
        },
        IomuxcRegisters {
            name: "GPIO_00",
            mux_ctl: 1075806396,
            pad_ctl: 1075806572,
        },
    ];
}
