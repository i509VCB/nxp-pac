use crate::metadata::*;
pub const METADATA: Metadata = Metadata {
    name: "MIMXRT1064",
    pins: PINS,
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
};
pub const PINS: &[Pin] = &[
    Pin {
        name: "GPIO_AD_B0_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806396u32),
            pad: 1075806892u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806400u32),
            pad: 1075806896u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806404u32),
            pad: 1075806900u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806408u32),
            pad: 1075806904u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806412u32),
            pad: 1075806908u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806416u32),
            pad: 1075806912u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806420u32),
            pad: 1075806916u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806424u32),
            pad: 1075806920u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806428u32),
            pad: 1075806924u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806432u32),
            pad: 1075806928u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806436u32),
            pad: 1075806932u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806440u32),
            pad: 1075806936u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806444u32),
            pad: 1075806940u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806448u32),
            pad: 1075806944u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_14",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806452u32),
            pad: 1075806948u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B0_15",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806456u32),
            pad: 1075806952u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806460u32),
            pad: 1075806956u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806464u32),
            pad: 1075806960u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806468u32),
            pad: 1075806964u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806472u32),
            pad: 1075806968u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806476u32),
            pad: 1075806972u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806480u32),
            pad: 1075806976u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806484u32),
            pad: 1075806980u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806488u32),
            pad: 1075806984u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806492u32),
            pad: 1075806988u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806496u32),
            pad: 1075806992u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806500u32),
            pad: 1075806996u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806504u32),
            pad: 1075807000u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806508u32),
            pad: 1075807004u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806512u32),
            pad: 1075807008u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_14",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806516u32),
            pad: 1075807012u32,
        }),
    },
    Pin {
        name: "GPIO_AD_B1_15",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806520u32),
            pad: 1075807016u32,
        }),
    },
    Pin {
        name: "GPIO_B0_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806524u32),
            pad: 1075806524u32,
        }),
    },
    Pin {
        name: "GPIO_B0_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806528u32),
            pad: 1075806528u32,
        }),
    },
    Pin {
        name: "GPIO_B0_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806532u32),
            pad: 1075806532u32,
        }),
    },
    Pin {
        name: "GPIO_B0_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806536u32),
            pad: 1075806536u32,
        }),
    },
    Pin {
        name: "GPIO_B0_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806540u32),
            pad: 1075806540u32,
        }),
    },
    Pin {
        name: "GPIO_B0_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806544u32),
            pad: 1075806544u32,
        }),
    },
    Pin {
        name: "GPIO_B0_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806548u32),
            pad: 1075806548u32,
        }),
    },
    Pin {
        name: "GPIO_B0_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806552u32),
            pad: 1075806552u32,
        }),
    },
    Pin {
        name: "GPIO_B0_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806556u32),
            pad: 1075806556u32,
        }),
    },
    Pin {
        name: "GPIO_B0_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806560u32),
            pad: 1075806560u32,
        }),
    },
    Pin {
        name: "GPIO_B0_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806564u32),
            pad: 1075806564u32,
        }),
    },
    Pin {
        name: "GPIO_B0_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806568u32),
            pad: 1075806568u32,
        }),
    },
    Pin {
        name: "GPIO_B0_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806572u32),
            pad: 1075806572u32,
        }),
    },
    Pin {
        name: "GPIO_B0_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806576u32),
            pad: 1075806576u32,
        }),
    },
    Pin {
        name: "GPIO_B0_14",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806580u32),
            pad: 1075806580u32,
        }),
    },
    Pin {
        name: "GPIO_B0_15",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806584u32),
            pad: 1075806584u32,
        }),
    },
    Pin {
        name: "GPIO_B1_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806588u32),
            pad: 1075806588u32,
        }),
    },
    Pin {
        name: "GPIO_B1_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806592u32),
            pad: 1075806592u32,
        }),
    },
    Pin {
        name: "GPIO_B1_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806596u32),
            pad: 1075806596u32,
        }),
    },
    Pin {
        name: "GPIO_B1_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806600u32),
            pad: 1075806600u32,
        }),
    },
    Pin {
        name: "GPIO_B1_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806604u32),
            pad: 1075806604u32,
        }),
    },
    Pin {
        name: "GPIO_B1_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806608u32),
            pad: 1075806608u32,
        }),
    },
    Pin {
        name: "GPIO_B1_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806612u32),
            pad: 1075806612u32,
        }),
    },
    Pin {
        name: "GPIO_B1_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806616u32),
            pad: 1075806616u32,
        }),
    },
    Pin {
        name: "GPIO_B1_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806620u32),
            pad: 1075806620u32,
        }),
    },
    Pin {
        name: "GPIO_B1_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806624u32),
            pad: 1075806624u32,
        }),
    },
    Pin {
        name: "GPIO_B1_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806628u32),
            pad: 1075806628u32,
        }),
    },
    Pin {
        name: "GPIO_B1_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806632u32),
            pad: 1075806632u32,
        }),
    },
    Pin {
        name: "GPIO_B1_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806636u32),
            pad: 1075806636u32,
        }),
    },
    Pin {
        name: "GPIO_B1_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806640u32),
            pad: 1075806640u32,
        }),
    },
    Pin {
        name: "GPIO_B1_14",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806644u32),
            pad: 1075806644u32,
        }),
    },
    Pin {
        name: "GPIO_B1_15",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806648u32),
            pad: 1075806648u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806228u32),
            pad: 1075806724u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806232u32),
            pad: 1075806728u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806236u32),
            pad: 1075806732u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806240u32),
            pad: 1075806736u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806244u32),
            pad: 1075806740u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806248u32),
            pad: 1075806744u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806252u32),
            pad: 1075806748u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806256u32),
            pad: 1075806752u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806260u32),
            pad: 1075806756u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806264u32),
            pad: 1075806760u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806268u32),
            pad: 1075806764u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806272u32),
            pad: 1075806768u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806276u32),
            pad: 1075806772u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806280u32),
            pad: 1075806776u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_14",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806284u32),
            pad: 1075806780u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_15",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806288u32),
            pad: 1075806784u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_16",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806292u32),
            pad: 1075806788u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_17",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806296u32),
            pad: 1075806792u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_18",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806300u32),
            pad: 1075806796u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_19",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806304u32),
            pad: 1075806800u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_20",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806308u32),
            pad: 1075806804u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_21",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806312u32),
            pad: 1075806808u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_22",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806316u32),
            pad: 1075806812u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_23",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806320u32),
            pad: 1075806816u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_24",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806324u32),
            pad: 1075806820u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_25",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806328u32),
            pad: 1075806824u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_26",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806332u32),
            pad: 1075806828u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_27",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806336u32),
            pad: 1075806832u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_28",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806340u32),
            pad: 1075806836u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_29",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806344u32),
            pad: 1075806840u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_30",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806348u32),
            pad: 1075806844u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_31",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806352u32),
            pad: 1075806848u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_32",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806356u32),
            pad: 1075806852u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_33",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806360u32),
            pad: 1075806856u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_34",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806364u32),
            pad: 1075806860u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_35",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806368u32),
            pad: 1075806864u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_36",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806372u32),
            pad: 1075806868u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_37",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806376u32),
            pad: 1075806872u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_38",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806380u32),
            pad: 1075806876u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_39",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806384u32),
            pad: 1075806880u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_40",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806388u32),
            pad: 1075806884u32,
        }),
    },
    Pin {
        name: "GPIO_EMC_41",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806392u32),
            pad: 1075806888u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B0_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806652u32),
            pad: 1075807148u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B0_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806656u32),
            pad: 1075807152u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B0_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806660u32),
            pad: 1075807156u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B0_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806664u32),
            pad: 1075807160u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B0_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806668u32),
            pad: 1075807164u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B0_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806672u32),
            pad: 1075807168u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806676u32),
            pad: 1075807172u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806680u32),
            pad: 1075807176u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806684u32),
            pad: 1075807180u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806688u32),
            pad: 1075807184u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806692u32),
            pad: 1075807188u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806696u32),
            pad: 1075807192u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806700u32),
            pad: 1075807196u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806704u32),
            pad: 1075807200u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806708u32),
            pad: 1075807204u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806712u32),
            pad: 1075807208u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806716u32),
            pad: 1075807212u32,
        }),
    },
    Pin {
        name: "GPIO_SD_B1_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075806720u32),
            pad: 1075807216u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807836u32),
            pad: 1075807924u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807840u32),
            pad: 1075807928u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807844u32),
            pad: 1075807932u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807848u32),
            pad: 1075807936u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807852u32),
            pad: 1075807940u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807856u32),
            pad: 1075807944u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807860u32),
            pad: 1075807948u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807864u32),
            pad: 1075807952u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_08",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807868u32),
            pad: 1075807956u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_09",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807872u32),
            pad: 1075807960u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_10",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807876u32),
            pad: 1075807964u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_11",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807880u32),
            pad: 1075807968u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_12",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807884u32),
            pad: 1075807972u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B0_13",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807888u32),
            pad: 1075807976u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_00",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807892u32),
            pad: 1075807980u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_01",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807896u32),
            pad: 1075807984u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_02",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807900u32),
            pad: 1075807988u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_03",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807904u32),
            pad: 1075807992u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_04",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807908u32),
            pad: 1075807996u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_05",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807912u32),
            pad: 1075808000u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_06",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807916u32),
            pad: 1075808004u32,
        }),
    },
    Pin {
        name: "GPIO_SPI_B1_07",
        iomuxc: Some(PinIomuxc {
            mux: Some(1075807920u32),
            pad: 1075808008u32,
        }),
    },
    Pin {
        name: "WAKEUP",
        iomuxc: Some(PinIomuxc {
            mux: Some(1074429952u32),
            pad: 1074429976u32,
        }),
    },
    Pin {
        name: "PMIC_ON_REQ",
        iomuxc: Some(PinIomuxc {
            mux: Some(1074429956u32),
            pad: 1074429980u32,
        }),
    },
    Pin {
        name: "PMIC_STBY_REQ",
        iomuxc: Some(PinIomuxc {
            mux: Some(1074429960u32),
            pad: 1074429984u32,
        }),
    },
    Pin {
        name: "ONOFF",
        iomuxc: Some(PinIomuxc {
            mux: None,
            pad: 1074429972u32,
        }),
    },
    Pin {
        name: "POR_B",
        iomuxc: Some(PinIomuxc {
            mux: None,
            pad: 1074429968u32,
        }),
    },
    Pin {
        name: "TEST_MODE",
        iomuxc: Some(PinIomuxc {
            mux: None,
            pad: 1074429964u32,
        }),
    },
];
pub const PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "FLEXSPI1",
        signals: &[
            Signal {
                name: "A_DATA0",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_13",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_08",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807400u32),
            },
            Signal {
                name: "A_DATA1",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_12",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_09",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807404u32),
            },
            Signal {
                name: "A_DATA2",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_11",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_10",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807408u32),
            },
            Signal {
                name: "A_DATA3",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_10",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_11",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807412u32),
            },
            Signal {
                name: "A_SCLK",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_14",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_07",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807432u32),
            },
            Signal {
                name: "A_SS0_B",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_15",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_06",
                        alt: 1u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SS1_B",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_08",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_SD_B0_00",
                        alt: 6u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_04",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DQS",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_09",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_05",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807396u32),
            },
            Signal {
                name: "B_DATA0",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_07",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_03",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807416u32),
            },
            Signal {
                name: "B_DATA1",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_06",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_02",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807420u32),
            },
            Signal {
                name: "B_DATA2",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_05",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_01",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807424u32),
            },
            Signal {
                name: "B_DATA3",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_04",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_00",
                        alt: 1u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807428u32),
            },
            Signal {
                name: "B_SCLK",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_04",
                    alt: 1u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS0_B",
                pins: &[
                    SignalPin {
                        pin: "GPIO_SD_B0_04",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_05",
                        alt: 4u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS1_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_01",
                    alt: 6u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DQS",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_05",
                    alt: 4u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "FLEXSPI2",
        signals: &[
            Signal {
                name: "A_DATA0",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_26",
                        alt: 8u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_02",
                        alt: 0u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_04",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075808048u32),
            },
            Signal {
                name: "A_DATA1",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_27",
                        alt: 8u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_12",
                        alt: 0u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_03",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075808052u32),
            },
            Signal {
                name: "A_DATA2",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_28",
                        alt: 8u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_06",
                        alt: 0u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_02",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075808056u32),
            },
            Signal {
                name: "A_DATA3",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_29",
                        alt: 8u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_10",
                        alt: 0u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_01",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075808060u32),
            },
            Signal {
                name: "A_SCLK",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_25",
                        alt: 8u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_08",
                        alt: 0u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_05",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075808080u32),
            },
            Signal {
                name: "A_SS0_B",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_24",
                        alt: 8u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_05",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_06",
                        alt: 0u8,
                        iomuxc_daisy: None,
                    },
                ],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_SS1_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_22",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "A_DQS",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_23",
                        alt: 8u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_09",
                        alt: 0u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B1_00",
                        alt: 0u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075808044u32),
            },
            Signal {
                name: "B_DATA0",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_13",
                        alt: 8u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_11",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075808064u32),
            },
            Signal {
                name: "B_DATA1",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_14",
                        alt: 8u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_07",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075808068u32),
            },
            Signal {
                name: "B_DATA2",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_15",
                        alt: 8u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_03",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075808072u32),
            },
            Signal {
                name: "B_DATA3",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_16",
                        alt: 8u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_04",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075808076u32),
            },
            Signal {
                name: "B_SCLK",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_12",
                        alt: 8u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_SPI_B0_01",
                        alt: 0u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075808084u32),
            },
            Signal {
                name: "B_SS0_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_10",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_SS1_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_09",
                    alt: 8u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "B_DQS",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_11",
                    alt: 8u8,
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
                    pin: "GPIO_AD_B0_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_14",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_15",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_14",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_15",
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
                    pin: "GPIO_B0_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_B0_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_B0_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_B0_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_B0_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_B0_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_B0_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_B0_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_B0_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_B0_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_B0_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_B0_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_B0_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_B0_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "GPIO_B0_14",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "GPIO_B0_15",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "GPIO_B1_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "GPIO_B1_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "GPIO_B1_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "GPIO_B1_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "GPIO_B1_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "GPIO_B1_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "GPIO_B1_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "GPIO_B1_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "GPIO_B1_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "GPIO_B1_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "GPIO_B1_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "GPIO_B1_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "GPIO_B1_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "GPIO_B1_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "GPIO_B1_14",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "GPIO_B1_15",
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
        name: "GPIO3",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_32",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_33",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_34",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_35",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_36",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_37",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_38",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_39",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_40",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_41",
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
        name: "GPIO4",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_14",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_15",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_16",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_17",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_18",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_19",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_20",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_21",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "22",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_22",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "23",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_23",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "24",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_24",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "25",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_25",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "26",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_26",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "27",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_27",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "28",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_28",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "29",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_29",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "30",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_30",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "31",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_31",
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
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "WAKEUP",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "PMIC_ON_REQ",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "PMIC_STBY_REQ",
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
        name: "GPIO10",
        signals: &[
            Signal {
                name: "0",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "1",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "2",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "3",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "4",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "5",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "6",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "7",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_07",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "8",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_08",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "9",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_09",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "10",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_10",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "11",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_11",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "12",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_12",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "13",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B0_13",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "14",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_00",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "15",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_01",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "16",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_02",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "17",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_03",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "18",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_04",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "19",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_05",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "20",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_06",
                    alt: 5u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "21",
                pins: &[SignalPin {
                    pin: "GPIO_SPI_B1_07",
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
        name: "LPUART1",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_12",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RXD",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_13",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_14",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B0_15",
                    alt: 2u8,
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
                        pin: "GPIO_AD_B1_02",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_11",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807536u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_03",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_SD_B1_10",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807532u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_00",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_AD_B1_01",
                    alt: 2u8,
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
                        pin: "GPIO_AD_B1_06",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_B0_08",
                        alt: 3u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_13",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075807548u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B1_07",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_B0_09",
                        alt: 3u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_14",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075807544u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_15",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_B1_04",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075807540u32),
            },
            Signal {
                name: "RTS_B",
                pins: &[
                    SignalPin {
                        pin: "GPIO_EMC_16",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                    SignalPin {
                        pin: "GPIO_AD_B1_05",
                        alt: 2u8,
                        iomuxc_daisy: None,
                    },
                ],
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
                        pin: "GPIO_SD_B1_00",
                        alt: 4u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_B1_00",
                        alt: 2u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_19",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075807556u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_SD_B1_01",
                        alt: 4u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_B1_01",
                        alt: 2u8,
                        iomuxc_daisy: Some(2u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_20",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807552u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_17",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_18",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART5",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_B1_12",
                        alt: 1u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_23",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807564u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_B1_13",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_24",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807560u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_28",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_27",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART6",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B0_02",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_25",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807572u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_AD_B0_03",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_26",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                ],
                iomuxc_daisy: Some(1075807568u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_30",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_EMC_29",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART7",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_SD_B1_08",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_31",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075807580u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_SD_B1_09",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_32",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                ],
                iomuxc_daisy: Some(1075807576u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_06",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B1_07",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
        ],
        flexcomm: None,
        dma_muxing: &[],
    },
    Peripheral {
        name: "LPUART8",
        signals: &[
            Signal {
                name: "TXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_SD_B0_04",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_B1_10",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_38",
                        alt: 2u8,
                        iomuxc_daisy: Some(2u8),
                    },
                ],
                iomuxc_daisy: Some(1075807588u32),
            },
            Signal {
                name: "RXD",
                pins: &[
                    SignalPin {
                        pin: "GPIO_SD_B0_05",
                        alt: 2u8,
                        iomuxc_daisy: Some(0u8),
                    },
                    SignalPin {
                        pin: "GPIO_AD_B1_11",
                        alt: 2u8,
                        iomuxc_daisy: Some(1u8),
                    },
                    SignalPin {
                        pin: "GPIO_EMC_39",
                        alt: 2u8,
                        iomuxc_daisy: Some(2u8),
                    },
                ],
                iomuxc_daisy: Some(1075807584u32),
            },
            Signal {
                name: "CTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_02",
                    alt: 2u8,
                    iomuxc_daisy: None,
                }],
                iomuxc_daisy: None,
            },
            Signal {
                name: "RTS_B",
                pins: &[SignalPin {
                    pin: "GPIO_SD_B0_03",
                    alt: 2u8,
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
    "ACMP1",
    "ACMP2",
    "ACMP3",
    "ACMP4",
    "ADC1",
    "ADC2",
    "ADC_ETC_ERROR_IRQ",
    "ADC_ETC_IRQ0",
    "ADC_ETC_IRQ1",
    "ADC_ETC_IRQ2",
    "BEE",
    "CAN1",
    "CAN2",
    "CAN3",
    "CCM_1",
    "CCM_2",
    "CORE",
    "CSI",
    "CSU",
    "CTI0_ERROR",
    "CTI1_ERROR",
    "DCDC",
    "DCP",
    "DCP_VMI",
    "DMA0_DMA16",
    "DMA10_DMA26",
    "DMA11_DMA27",
    "DMA12_DMA28",
    "DMA13_DMA29",
    "DMA14_DMA30",
    "DMA15_DMA31",
    "DMA1_DMA17",
    "DMA2_DMA18",
    "DMA3_DMA19",
    "DMA4_DMA20",
    "DMA5_DMA21",
    "DMA6_DMA22",
    "DMA7_DMA23",
    "DMA8_DMA24",
    "DMA9_DMA25",
    "DMA_ERROR",
    "ENC1",
    "ENC2",
    "ENC3",
    "ENC4",
    "ENET",
    "ENET2",
    "ENET2_1588_TIMER",
    "ENET_1588_TIMER",
    "EWM",
    "FLEXIO1",
    "FLEXIO2",
    "FLEXIO3",
    "FLEXRAM",
    "FLEXSPI",
    "FLEXSPI2",
    "GPC",
    "GPIO10",
    "GPIO1_COMBINED_0_15",
    "GPIO1_COMBINED_16_31",
    "GPIO1_INT0",
    "GPIO1_INT1",
    "GPIO1_INT2",
    "GPIO1_INT3",
    "GPIO1_INT4",
    "GPIO1_INT5",
    "GPIO1_INT6",
    "GPIO1_INT7",
    "GPIO2_COMBINED_0_15",
    "GPIO2_COMBINED_16_31",
    "GPIO3_COMBINED_0_15",
    "GPIO3_COMBINED_16_31",
    "GPIO4_COMBINED_0_15",
    "GPIO4_COMBINED_16_31",
    "GPIO5_COMBINED_0_15",
    "GPIO5_COMBINED_16_31",
    "GPIO6_7_8_9",
    "GPR_IRQ",
    "GPT1",
    "GPT2",
    "KPP",
    "LCDIF",
    "LPI2C1",
    "LPI2C2",
    "LPI2C3",
    "LPI2C4",
    "LPSPI1",
    "LPSPI2",
    "LPSPI3",
    "LPSPI4",
    "LPUART1",
    "LPUART2",
    "LPUART3",
    "LPUART4",
    "LPUART5",
    "LPUART6",
    "LPUART7",
    "LPUART8",
    "PIT",
    "PMU_EVENT",
    "PWM1_0",
    "PWM1_1",
    "PWM1_2",
    "PWM1_3",
    "PWM1_FAULT",
    "PWM2_0",
    "PWM2_1",
    "PWM2_2",
    "PWM2_3",
    "PWM2_FAULT",
    "PWM3_0",
    "PWM3_1",
    "PWM3_2",
    "PWM3_3",
    "PWM3_FAULT",
    "PWM4_0",
    "PWM4_1",
    "PWM4_2",
    "PWM4_3",
    "PWM4_FAULT",
    "PXP",
    "RESERVED115",
    "RESERVED143",
    "RESERVED144",
    "RESERVED171",
    "RESERVED68",
    "RESERVED78",
    "RESERVED86",
    "RTWDOG",
    "SAI1",
    "SAI2",
    "SAI3_RX",
    "SAI3_TX",
    "SEMC",
    "SJC",
    "SNVS_HP_WRAPPER",
    "SNVS_HP_WRAPPER_TZ",
    "SNVS_LP_WRAPPER",
    "SPDIF",
    "SRC",
    "TEMP_LOW_HIGH",
    "TEMP_PANIC",
    "TMR1",
    "TMR2",
    "TMR3",
    "TMR4",
    "TRNG",
    "TSC_DIG",
    "USB_OTG1",
    "USB_OTG2",
    "USB_PHY1",
    "USB_PHY2",
    "USDHC1",
    "USDHC2",
    "WDOG1",
    "WDOG2",
    "XBAR1_IRQ_0_1",
    "XBAR1_IRQ_2_3",
];
