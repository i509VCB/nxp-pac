#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0b476f2 2026-01-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - OR"]
    OR = 0,
    #[doc = "1 - EDMA_0_CH0"]
    EDMA_0_CH0 = 1,
    #[doc = "2 - EDMA_0_CH1"]
    EDMA_0_CH1 = 2,
    #[doc = "3 - EDMA_0_CH2"]
    EDMA_0_CH2 = 3,
    #[doc = "4 - EDMA_0_CH3"]
    EDMA_0_CH3 = 4,
    #[doc = "5 - EDMA_0_CH4"]
    EDMA_0_CH4 = 5,
    #[doc = "6 - EDMA_0_CH5"]
    EDMA_0_CH5 = 6,
    #[doc = "7 - EDMA_0_CH6"]
    EDMA_0_CH6 = 7,
    #[doc = "8 - EDMA_0_CH7"]
    EDMA_0_CH7 = 8,
    #[doc = "9 - EDMA_0_CH8"]
    EDMA_0_CH8 = 9,
    #[doc = "10 - EDMA_0_CH9"]
    EDMA_0_CH9 = 10,
    #[doc = "11 - EDMA_0_CH10"]
    EDMA_0_CH10 = 11,
    #[doc = "12 - EDMA_0_CH11"]
    EDMA_0_CH11 = 12,
    #[doc = "13 - EDMA_0_CH12"]
    EDMA_0_CH12 = 13,
    #[doc = "14 - EDMA_0_CH13"]
    EDMA_0_CH13 = 14,
    #[doc = "15 - EDMA_0_CH14"]
    EDMA_0_CH14 = 15,
    #[doc = "16 - EDMA_0_CH15"]
    EDMA_0_CH15 = 16,
    #[doc = "17 - GPIO00"]
    GPIO00 = 17,
    #[doc = "18 - GPIO01"]
    GPIO01 = 18,
    #[doc = "19 - GPIO10"]
    GPIO10 = 19,
    #[doc = "20 - GPIO11"]
    GPIO11 = 20,
    #[doc = "21 - GPIO20"]
    GPIO20 = 21,
    #[doc = "22 - GPIO21"]
    GPIO21 = 22,
    #[doc = "23 - GPIO30"]
    GPIO30 = 23,
    #[doc = "24 - GPIO31"]
    GPIO31 = 24,
    #[doc = "25 - GPIO40"]
    GPIO40 = 25,
    #[doc = "26 - GPIO41"]
    GPIO41 = 26,
    #[doc = "27 - GPIO50"]
    GPIO50 = 27,
    #[doc = "28 - GPIO51"]
    GPIO51 = 28,
    #[doc = "29 - UTICK0"]
    UTICK0 = 29,
    #[doc = "30 - MRT0"]
    MRT0 = 30,
    #[doc = "31 - CTIMER0"]
    CTIMER0 = 31,
    #[doc = "32 - CTIMER1"]
    CTIMER1 = 32,
    #[doc = "33 - SCT0"]
    SCT0 = 33,
    #[doc = "34 - CTIMER2"]
    CTIMER2 = 34,
    #[doc = "35 - LP_FLEXCOMM0"]
    LP_FLEXCOMM0 = 35,
    #[doc = "36 - LP_FLEXCOMM1"]
    LP_FLEXCOMM1 = 36,
    #[doc = "37 - LP_FLEXCOMM2"]
    LP_FLEXCOMM2 = 37,
    #[doc = "38 - LP_FLEXCOMM3"]
    LP_FLEXCOMM3 = 38,
    #[doc = "39 - LP_FLEXCOMM4"]
    LP_FLEXCOMM4 = 39,
    #[doc = "40 - LP_FLEXCOMM5"]
    LP_FLEXCOMM5 = 40,
    #[doc = "41 - LP_FLEXCOMM6"]
    LP_FLEXCOMM6 = 41,
    #[doc = "42 - LP_FLEXCOMM7"]
    LP_FLEXCOMM7 = 42,
    #[doc = "43 - LP_FLEXCOMM8"]
    LP_FLEXCOMM8 = 43,
    #[doc = "44 - LP_FLEXCOMM9"]
    LP_FLEXCOMM9 = 44,
    #[doc = "45 - ADC0"]
    ADC0 = 45,
    #[doc = "46 - ADC1"]
    ADC1 = 46,
    #[doc = "47 - PINT0"]
    PINT0 = 47,
    #[doc = "48 - PDM_EVENT"]
    PDM_EVENT = 48,
    #[doc = "49 - RESERVED65"]
    RESERVED65 = 49,
    #[doc = "50 - USB0_FS"]
    USB0_FS = 50,
    #[doc = "51 - USB0_DCD"]
    USB0_DCD = 51,
    #[doc = "52 - RTC"]
    RTC = 52,
    #[doc = "53 - SMARTDMA"]
    SMARTDMA = 53,
    #[doc = "54 - MAILBOX"]
    MAILBOX = 54,
    #[doc = "55 - CTIMER3"]
    CTIMER3 = 55,
    #[doc = "56 - CTIMER4"]
    CTIMER4 = 56,
    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,
    #[doc = "58 - FLEXSPI0"]
    FLEXSPI0 = 58,
    #[doc = "59 - SAI0"]
    SAI0 = 59,
    #[doc = "60 - SAI1"]
    SAI1 = 60,
    #[doc = "61 - USDHC0"]
    USDHC0 = 61,
    #[doc = "62 - CAN0"]
    CAN0 = 62,
    #[doc = "63 - CAN1"]
    CAN1 = 63,
    #[doc = "64 - RESERVED80"]
    RESERVED80 = 64,
    #[doc = "65 - RESERVED81"]
    RESERVED81 = 65,
    #[doc = "66 - USB1_HS_PHY"]
    USB1_HS_PHY = 66,
    #[doc = "67 - USB1_HS"]
    USB1_HS = 67,
    #[doc = "68 - SEC_HYPERVISOR_CALL"]
    SEC_HYPERVISOR_CALL = 68,
    #[doc = "69 - RESERVED85"]
    RESERVED85 = 69,
    #[doc = "70 - PLU"]
    PLU = 70,
    #[doc = "71 - FREQME"]
    FREQME = 71,
    #[doc = "72 - SEC_VIO"]
    SEC_VIO = 72,
    #[doc = "73 - ELS"]
    ELS = 73,
    #[doc = "74 - PKC"]
    PKC = 74,
    #[doc = "75 - PUF"]
    PUF = 75,
    #[doc = "76 - PQ"]
    PQ = 76,
    #[doc = "77 - EDMA_1_CH0"]
    EDMA_1_CH0 = 77,
    #[doc = "78 - EDMA_1_CH1"]
    EDMA_1_CH1 = 78,
    #[doc = "79 - EDMA_1_CH2"]
    EDMA_1_CH2 = 79,
    #[doc = "80 - EDMA_1_CH3"]
    EDMA_1_CH3 = 80,
    #[doc = "81 - EDMA_1_CH4"]
    EDMA_1_CH4 = 81,
    #[doc = "82 - EDMA_1_CH5"]
    EDMA_1_CH5 = 82,
    #[doc = "83 - EDMA_1_CH6"]
    EDMA_1_CH6 = 83,
    #[doc = "84 - EDMA_1_CH7"]
    EDMA_1_CH7 = 84,
    #[doc = "85 - EDMA_1_CH8"]
    EDMA_1_CH8 = 85,
    #[doc = "86 - EDMA_1_CH9"]
    EDMA_1_CH9 = 86,
    #[doc = "87 - EDMA_1_CH10"]
    EDMA_1_CH10 = 87,
    #[doc = "88 - EDMA_1_CH11"]
    EDMA_1_CH11 = 88,
    #[doc = "89 - EDMA_1_CH12"]
    EDMA_1_CH12 = 89,
    #[doc = "90 - EDMA_1_CH13"]
    EDMA_1_CH13 = 90,
    #[doc = "91 - EDMA_1_CH14"]
    EDMA_1_CH14 = 91,
    #[doc = "92 - EDMA_1_CH15"]
    EDMA_1_CH15 = 92,
    #[doc = "93 - CDOG0"]
    CDOG0 = 93,
    #[doc = "94 - CDOG1"]
    CDOG1 = 94,
    #[doc = "95 - I3C0"]
    I3C0 = 95,
    #[doc = "96 - I3C1"]
    I3C1 = 96,
    #[doc = "97 - NPU"]
    NPU = 97,
    #[doc = "98 - GDET"]
    GDET = 98,
    #[doc = "99 - VBAT0"]
    VBAT0 = 99,
    #[doc = "100 - EWM0"]
    EWM0 = 100,
    #[doc = "101 - TSI_END_OF_SCAN"]
    TSI_END_OF_SCAN = 101,
    #[doc = "102 - TSI_OUT_OF_SCAN"]
    TSI_OUT_OF_SCAN = 102,
    #[doc = "103 - EMVSIM0"]
    EMVSIM0 = 103,
    #[doc = "104 - EMVSIM1"]
    EMVSIM1 = 104,
    #[doc = "105 - FLEXIO"]
    FLEXIO = 105,
    #[doc = "106 - DAC0"]
    DAC0 = 106,
    #[doc = "107 - DAC1"]
    DAC1 = 107,
    #[doc = "108 - DAC2"]
    DAC2 = 108,
    #[doc = "109 - HSCMP0"]
    HSCMP0 = 109,
    #[doc = "110 - HSCMP1"]
    HSCMP1 = 110,
    #[doc = "111 - HSCMP2"]
    HSCMP2 = 111,
    #[doc = "112 - FLEXPWM0_RELOAD_ERROR"]
    FLEXPWM0_RELOAD_ERROR = 112,
    #[doc = "113 - FLEXPWM0_FAULT"]
    FLEXPWM0_FAULT = 113,
    #[doc = "114 - FLEXPWM0_SUBMODULE0"]
    FLEXPWM0_SUBMODULE0 = 114,
    #[doc = "115 - FLEXPWM0_SUBMODULE1"]
    FLEXPWM0_SUBMODULE1 = 115,
    #[doc = "116 - FLEXPWM0_SUBMODULE2"]
    FLEXPWM0_SUBMODULE2 = 116,
    #[doc = "117 - FLEXPWM0_SUBMODULE3"]
    FLEXPWM0_SUBMODULE3 = 117,
    #[doc = "118 - FLEXPWM1_RELOAD_ERROR"]
    FLEXPWM1_RELOAD_ERROR = 118,
    #[doc = "119 - FLEXPWM1_FAULT"]
    FLEXPWM1_FAULT = 119,
    #[doc = "120 - FLEXPWM1_SUBMODULE0"]
    FLEXPWM1_SUBMODULE0 = 120,
    #[doc = "121 - FLEXPWM1_SUBMODULE1"]
    FLEXPWM1_SUBMODULE1 = 121,
    #[doc = "122 - FLEXPWM1_SUBMODULE2"]
    FLEXPWM1_SUBMODULE2 = 122,
    #[doc = "123 - FLEXPWM1_SUBMODULE3"]
    FLEXPWM1_SUBMODULE3 = 123,
    #[doc = "124 - QDC0_COMPARE"]
    QDC0_COMPARE = 124,
    #[doc = "125 - QDC0_HOME"]
    QDC0_HOME = 125,
    #[doc = "126 - QDC0_WDG_SAB"]
    QDC0_WDG_SAB = 126,
    #[doc = "127 - QDC0_IDX"]
    QDC0_IDX = 127,
    #[doc = "128 - QDC1_COMPARE"]
    QDC1_COMPARE = 128,
    #[doc = "129 - QDC1_HOME"]
    QDC1_HOME = 129,
    #[doc = "130 - QDC1_WDG_SAB"]
    QDC1_WDG_SAB = 130,
    #[doc = "131 - QDC1_IDX"]
    QDC1_IDX = 131,
    #[doc = "132 - ITRC0"]
    ITRC0 = 132,
    #[doc = "133 - BSP32"]
    BSP32 = 133,
    #[doc = "134 - ELS_ERR"]
    ELS_ERR = 134,
    #[doc = "135 - PKC_ERR"]
    PKC_ERR = 135,
    #[doc = "136 - ERM_SINGLE_BIT_ERROR"]
    ERM_SINGLE_BIT_ERROR = 136,
    #[doc = "137 - ERM_MULTI_BIT_ERROR"]
    ERM_MULTI_BIT_ERROR = 137,
    #[doc = "138 - FMU0"]
    FMU0 = 138,
    #[doc = "139 - ETHERNET"]
    ETHERNET = 139,
    #[doc = "140 - ETHERNET_PMT"]
    ETHERNET_PMT = 140,
    #[doc = "141 - ETHERNET_MACLP"]
    ETHERNET_MACLP = 141,
    #[doc = "142 - SINC_FILTER"]
    SINC_FILTER = 142,
    #[doc = "143 - LPTMR0"]
    LPTMR0 = 143,
    #[doc = "144 - LPTMR1"]
    LPTMR1 = 144,
    #[doc = "145 - SCG"]
    SCG = 145,
    #[doc = "146 - SPC"]
    SPC = 146,
    #[doc = "147 - WUU"]
    WUU = 147,
    #[doc = "148 - PORT_EFT"]
    PORT_EFT = 148,
    #[doc = "149 - ETB0"]
    ETB0 = 149,
    #[doc = "150 - RESERVED166"]
    RESERVED166 = 150,
    #[doc = "151 - RESERVED167"]
    RESERVED167 = 151,
    #[doc = "152 - WWDT0"]
    WWDT0 = 152,
    #[doc = "153 - WWDT1"]
    WWDT1 = 153,
    #[doc = "154 - CMC0"]
    CMC0 = 154,
    #[doc = "155 - CTI0"]
    CTI0 = 155,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "SYSCON"]
pub const SYSCON0: syscon0::Syscon0 = unsafe { syscon0::Syscon0::from_ptr(0x4000_0000usize as _) };
#[doc = "Pin Interrupts and Pattern Match"]
pub const PINT0: pint0::Pint0 = unsafe { pint0::Pint0::from_ptr(0x4000_4000usize as _) };
#[doc = "INPUTMUX"]
pub const INPUTMUX0: inputmux0::Inputmux0 =
    unsafe { inputmux0::Inputmux0::from_ptr(0x4000_6000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_c000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_d000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_e000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_f000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4001_0000usize as _) };
#[doc = "FREQME"]
pub const FREQME0: freqme0::Freqme0 = unsafe { freqme0::Freqme0::from_ptr(0x4001_1000usize as _) };
#[doc = "UTICK"]
pub const UTICK0: utick0::Utick0 = unsafe { utick0::Utick0::from_ptr(0x4001_2000usize as _) };
#[doc = "Multi-Rate Timer (MRT)"]
pub const MRT0: mrt0::Mrt0 = unsafe { mrt0::Mrt0::from_ptr(0x4001_3000usize as _) };
#[doc = "WWDT"]
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4001_6000usize as _) };
#[doc = "WWDT"]
pub const WWDT1: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4001_7000usize as _) };
#[doc = "CACHE64_CTRL"]
pub const CACHE64_CTRL0: cache64_ctrl0::Cache64Ctrl0 =
    unsafe { cache64_ctrl0::Cache64Ctrl0::from_ptr(0x4001_b000usize as _) };
#[doc = "CACHE64_POLSEL"]
pub const CACHE64_POLSEL0: cache64_polsel0::Cache64Polsel0 =
    unsafe { cache64_polsel0::Cache64Polsel0::from_ptr(0x4001_b000usize as _) };
#[doc = "I3C"]
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4002_1000usize as _) };
#[doc = "I3C"]
pub const I3C1: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4002_2000usize as _) };
#[doc = "no description available"]
pub const GDET0: gdet0::Gdet0 = unsafe { gdet0::Gdet0::from_ptr(0x4002_4000usize as _) };
#[doc = "no description available"]
pub const GDET1: gdet1::Gdet1 = unsafe { gdet1::Gdet1::from_ptr(0x4002_5000usize as _) };
#[doc = "Intrusion and Tamper Response Controller"]
pub const ITRC0: itrc0::Itrc0 = unsafe { itrc0::Itrc0::from_ptr(0x4002_6000usize as _) };
#[doc = "no description available"]
pub const PKC0: pkc0::Pkc0 = unsafe { pkc0::Pkc0::from_ptr(0x4002_b000usize as _) };
#[doc = "PUF"]
pub const PUF: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002_c000usize as _) };
#[doc = "PUF Key Context Management"]
pub const PUF_CTRL: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002_c000usize as _) };
#[doc = "PUF"]
pub const PUF_ALIAS1: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002_d000usize as _) };
#[doc = "PUF Key Context Management"]
pub const PUF_CTRL_ALIAS1: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002_d000usize as _) };
#[doc = "PUF"]
pub const PUF_ALIAS2: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002_e000usize as _) };
#[doc = "PUF Key Context Management"]
pub const PUF_CTRL_ALIAS2: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002_e000usize as _) };
#[doc = "PUF"]
pub const PUF_ALIAS3: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002_f000usize as _) };
#[doc = "PUF Key Context Management"]
pub const PUF_CTRL_ALIAS3: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002_f000usize as _) };
#[doc = "CoolFlux BSP32"]
pub const BSP32_0: bsp32_0::Bsp320 = unsafe { bsp32_0::Bsp320::from_ptr(0x4003_2000usize as _) };
#[doc = "SmartDMA"]
pub const SMARTDMA0: smartdma0::Smartdma0 =
    unsafe { smartdma0::Smartdma0::from_ptr(0x4003_3000usize as _) };
#[doc = "Programmable Logic Unit (PLU)"]
pub const PLU0: plu0::Plu0 = unsafe { plu0::Plu0::from_ptr(0x4003_4000usize as _) };
#[doc = "GPIO"]
pub const GPIO5: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_0000usize as _) };
#[doc = "GPIO"]
pub const GPIO5_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_1000usize as _) };
#[doc = "PORT"]
pub const PORT5: port::Port = unsafe { port::Port::from_ptr(0x4004_2000usize as _) };
#[doc = "Flash"]
pub const FMU0: fmu0::Fmu0 = unsafe { fmu0::Fmu0::from_ptr(0x4004_3000usize as _) };
#[doc = "FlashTest"]
pub const FMU0TEST: fmu0test::Fmu0test =
    unsafe { fmu0test::Fmu0test::from_ptr(0x4004_3000usize as _) };
#[doc = "SCG"]
pub const SCG0: scg0::Scg0 = unsafe { scg0::Scg0::from_ptr(0x4004_4000usize as _) };
#[doc = "SPC"]
pub const SPC0: spc0::Spc0 = unsafe { spc0::Spc0::from_ptr(0x4004_5000usize as _) };
#[doc = "Low-Leakage Wakeup Unit"]
pub const WUU0: wuu0::Wuu0 = unsafe { wuu0::Wuu0::from_ptr(0x4004_6000usize as _) };
#[doc = "CMC"]
pub const CMC0: cmc0::Cmc0 = unsafe { cmc0::Cmc0::from_ptr(0x4004_8000usize as _) };
#[doc = "OSTIMER"]
pub const OSTIMER0: ostimer0::Ostimer0 =
    unsafe { ostimer0::Ostimer0::from_ptr(0x4004_9000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR0: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x4004_a000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR1: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x4004_b000usize as _) };
#[doc = "RTC"]
pub const RTC0: rtc0::Rtc0 = unsafe { rtc0::Rtc0::from_ptr(0x4004_c000usize as _) };
#[doc = "RTC_SUBSYSTEM"]
pub const RTC_SUBSYSTEM0: rtc_subsystem0::RtcSubsystem0 =
    unsafe { rtc_subsystem0::RtcSubsystem0::from_ptr(0x4004_c000usize as _) };
#[doc = "TSI"]
pub const TSI0: tsi0::Tsi0 = unsafe { tsi0::Tsi0::from_ptr(0x4005_0000usize as _) };
#[doc = "LPCMP"]
pub const CMP0: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x4005_1000usize as _) };
#[doc = "LPCMP"]
pub const CMP1: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x4005_2000usize as _) };
#[doc = "LPCMP"]
pub const CMP2: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x4005_3000usize as _) };
#[doc = "no description available"]
pub const ELS: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_4000usize as _) };
#[doc = "no description available"]
pub const ELS_ALIAS1: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_5000usize as _) };
#[doc = "no description available"]
pub const ELS_ALIAS2: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_6000usize as _) };
#[doc = "no description available"]
pub const ELS_ALIAS3: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_7000usize as _) };
#[doc = "TDET"]
pub const TDET0: tdet0::Tdet0 = unsafe { tdet0::Tdet0::from_ptr(0x4005_8000usize as _) };
#[doc = "VBAT"]
pub const VBAT0: vbat0::Vbat0 = unsafe { vbat0::Vbat0::from_ptr(0x4005_9000usize as _) };
#[doc = "EIM"]
pub const EIM0: eim0::Eim0 = unsafe { eim0::Eim0::from_ptr(0x4005_b000usize as _) };
#[doc = "ERM"]
pub const ERM0: erm0::Erm0 = unsafe { erm0::Erm0::from_ptr(0x4005_c000usize as _) };
#[doc = "INTM"]
pub const INTM0: intm0::Intm0 = unsafe { intm0::Intm0::from_ptr(0x4005_d000usize as _) };
#[doc = "DMA MP"]
pub const DMA0: dma::Dma = unsafe { dma::Dma::from_ptr(0x4008_0000usize as _) };
#[doc = "DMA TCD"]
pub const EDMA_0_TCD: edma_0_tcd::Edma0Tcd =
    unsafe { edma_0_tcd::Edma0Tcd::from_ptr(0x4008_1000usize as _) };
#[doc = "SCT"]
pub const SCT0: sct0::Sct0 = unsafe { sct0::Sct0::from_ptr(0x4009_1000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_2000usize as _) };
#[doc = "LPUART"]
pub const LPUART0: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009_2000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM0: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x4009_2000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_2800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_3000usize as _) };
#[doc = "LPUART"]
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009_3000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM1: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x4009_3000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_3800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_4000usize as _) };
#[doc = "LPUART"]
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009_4000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM2: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x4009_4000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_4800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI3: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_5000usize as _) };
#[doc = "LPUART"]
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009_5000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM3: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x4009_5000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_5800usize as _) };
#[doc = "GPIO"]
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_6000usize as _) };
#[doc = "GPIO"]
pub const GPIO0_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_7000usize as _) };
#[doc = "GPIO"]
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_8000usize as _) };
#[doc = "GPIO"]
pub const GPIO1_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_9000usize as _) };
#[doc = "GPIO"]
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_a000usize as _) };
#[doc = "GPIO"]
pub const GPIO2_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_b000usize as _) };
#[doc = "GPIO"]
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO3_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_d000usize as _) };
#[doc = "GPIO"]
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_e000usize as _) };
#[doc = "GPIO"]
pub const GPIO4_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009_f000usize as _) };
#[doc = "DMA MP"]
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x400a_0000usize as _) };
#[doc = "DMA TCD"]
pub const EDMA_1_TCD: edma_1_tcd::Edma1Tcd =
    unsafe { edma_1_tcd::Edma1Tcd::from_ptr(0x400a_1000usize as _) };
#[doc = "SEMA42"]
pub const SEMA42_0: sema42_0::Sema420 =
    unsafe { sema42_0::Sema420::from_ptr(0x400b_1000usize as _) };
#[doc = "MAILBOX"]
pub const MAILBOX: mailbox::Mailbox = unsafe { mailbox::Mailbox::from_ptr(0x400b_2000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI4: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400b_4000usize as _) };
#[doc = "LPUART"]
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400b_4000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM4: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400b_4000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C4: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400b_4800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI5: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400b_5000usize as _) };
#[doc = "LPUART"]
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400b_5000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM5: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400b_5000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C5: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400b_5800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI6: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400b_6000usize as _) };
#[doc = "LPUART"]
pub const LPUART6: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400b_6000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM6: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400b_6000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C6: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400b_6800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI7: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400b_7000usize as _) };
#[doc = "LPUART"]
pub const LPUART7: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400b_7000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM7: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400b_7000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C7: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400b_7800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI8: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400b_8000usize as _) };
#[doc = "LPUART"]
pub const LPUART8: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400b_8000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM8: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400b_8000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C8: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400b_8800usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI9: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400b_9000usize as _) };
#[doc = "LPUART"]
pub const LPUART9: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400b_9000usize as _) };
#[doc = "LP_FLEXCOMM"]
pub const LP_FLEXCOMM9: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400b_9000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C9: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400b_9800usize as _) };
#[doc = "CDOG"]
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x400b_b000usize as _) };
#[doc = "CDOG"]
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x400b_c000usize as _) };
#[doc = "DBGMB"]
pub const DM0: dm0::Dm0 = unsafe { dm0::Dm0::from_ptr(0x400b_d000usize as _) };
#[doc = "PowerQuad"]
pub const POWERQUAD: powerquad::Powerquad =
    unsafe { powerquad::Powerquad::from_ptr(0x400b_f000usize as _) };
#[doc = "EWM"]
pub const EWM0: ewm0::Ewm0 = unsafe { ewm0::Ewm0::from_ptr(0x400c_0000usize as _) };
#[doc = "CMX_PERFMON"]
pub const CMX_PERFMON0: syspm::Syspm = unsafe { syspm::Syspm::from_ptr(0x400c_1000usize as _) };
#[doc = "CMX_PERFMON"]
pub const CMX_PERFMON1: syspm::Syspm = unsafe { syspm::Syspm::from_ptr(0x400c_2000usize as _) };
#[doc = "TRDC"]
pub const TRDC: trdc::Trdc = unsafe { trdc::Trdc::from_ptr(0x400c_7000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI0: flexspi0::Flexspi0 =
    unsafe { flexspi0::Flexspi0::from_ptr(0x400c_8000usize as _) };
#[doc = "OTPC"]
pub const OTPC0: otpc0::Otpc0 = unsafe { otpc0::Otpc0::from_ptr(0x400c_9000usize as _) };
#[doc = "CRC"]
pub const CRC0: crc0::Crc0 = unsafe { crc0::Crc0::from_ptr(0x400c_b000usize as _) };
#[doc = "NPX"]
pub const NPX0: npx0::Npx0 = unsafe { npx0::Npx0::from_ptr(0x400c_c000usize as _) };
#[doc = "PWM"]
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x400c_e000usize as _) };
#[doc = "ENC"]
pub const QDC0: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400c_f000usize as _) };
#[doc = "PWM"]
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x400d_0000usize as _) };
#[doc = "ENC"]
pub const QDC1: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400d_1000usize as _) };
#[doc = "EVTG"]
pub const EVTG0: evtg0::Evtg0 = unsafe { evtg0::Evtg0::from_ptr(0x400d_2000usize as _) };
#[doc = "CAN"]
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400d_4000usize as _) };
#[doc = "CAN"]
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x400d_8000usize as _) };
#[doc = "USBDCD"]
pub const USBDCD0: usbdcd0::Usbdcd0 = unsafe { usbdcd0::Usbdcd0::from_ptr(0x400d_c000usize as _) };
#[doc = "USBFS"]
pub const USBFS0: usbfs0::Usbfs0 = unsafe { usbfs0::Usbfs0::from_ptr(0x400d_d000usize as _) };
#[doc = "ENET"]
pub const ENET0: enet0::Enet0 = unsafe { enet0::Enet0::from_ptr(0x4010_0000usize as _) };
#[doc = "EMVSIM"]
pub const EMVSIM0: emvsim::Emvsim = unsafe { emvsim::Emvsim::from_ptr(0x4010_3000usize as _) };
#[doc = "EMVSIM"]
pub const EMVSIM1: emvsim::Emvsim = unsafe { emvsim::Emvsim::from_ptr(0x4010_4000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO0: flexio0::Flexio0 = unsafe { flexio0::Flexio0::from_ptr(0x4010_5000usize as _) };
#[doc = "SAI"]
pub const SAI0: sai::Sai = unsafe { sai::Sai::from_ptr(0x4010_6000usize as _) };
#[doc = "SAI"]
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x4010_7000usize as _) };
#[doc = "SINC"]
pub const SINC0: sinc0::Sinc0 = unsafe { sinc0::Sinc0::from_ptr(0x4010_8000usize as _) };
#[doc = "uSDHC"]
pub const USDHC0: usdhc0::Usdhc0 = unsafe { usdhc0::Usdhc0::from_ptr(0x4010_9000usize as _) };
#[doc = "USBPHY"]
pub const USBPHY: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x4010_a000usize as _) };
#[doc = "USBDCD"]
pub const USBHS1_PHY_DCD: usbhs1_phy_dcd::Usbhs1PhyDcd =
    unsafe { usbhs1_phy_dcd::Usbhs1PhyDcd::from_ptr(0x4010_a800usize as _) };
#[doc = "USB"]
pub const USBHS1__USBC: usbhs1__usbc::Usbhs1_usbc =
    unsafe { usbhs1__usbc::Usbhs1_usbc::from_ptr(0x4010_b000usize as _) };
#[doc = "USBNC"]
pub const USBHS1__USBNC: usbhs1__usbnc::Usbhs1_usbnc =
    unsafe { usbhs1__usbnc::Usbhs1_usbnc::from_ptr(0x4010_b200usize as _) };
#[doc = "MICFIL"]
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x4010_c000usize as _) };
#[doc = "ADC"]
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x4010_d000usize as _) };
#[doc = "ADC"]
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4010_e000usize as _) };
#[doc = "12-bit DAC"]
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x4010_f000usize as _) };
#[doc = "OPAMP"]
pub const OPAMP0: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4011_0000usize as _) };
#[doc = "VREF"]
pub const VREF0: vref0::Vref0 = unsafe { vref0::Vref0::from_ptr(0x4011_1000usize as _) };
#[doc = "12-bit DAC"]
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4011_2000usize as _) };
#[doc = "OPAMP"]
pub const OPAMP1: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4011_3000usize as _) };
#[doc = "14-bit DAC"]
pub const DAC2: dac2::Dac2 = unsafe { dac2::Dac2::from_ptr(0x4011_4000usize as _) };
#[doc = "OPAMP"]
pub const OPAMP2: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x4011_5000usize as _) };
#[doc = "PORT"]
pub const PORT0: port::Port = unsafe { port::Port::from_ptr(0x4011_6000usize as _) };
#[doc = "PORT"]
pub const PORT1: port::Port = unsafe { port::Port::from_ptr(0x4011_7000usize as _) };
#[doc = "PORT"]
pub const PORT2: port::Port = unsafe { port::Port::from_ptr(0x4011_8000usize as _) };
#[doc = "PORT"]
pub const PORT3: port::Port = unsafe { port::Port::from_ptr(0x4011_9000usize as _) };
#[doc = "PORT"]
pub const PORT4: port::Port = unsafe { port::Port::from_ptr(0x4011_a000usize as _) };
#[doc = "AHBSC"]
pub const AHBSC: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x4012_0000usize as _) };
#[doc = "AHBSC"]
pub const AHBSC_ALIAS1: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x4012_1000usize as _) };
#[doc = "AHBSC"]
pub const AHBSC_ALIAS2: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x4012_2000usize as _) };
#[doc = "AHBSC"]
pub const AHBSC_ALIAS3: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x4012_3000usize as _) };
#[doc = "System Control not in System Control Block"]
pub const SCNSCB: scn_scb::ScnScb = unsafe { scn_scb::ScnScb::from_ptr(0xe000_e000usize as _) };
#[doc = "M33 Systick module"]
pub const SYSTICK0: sys_tick0::SysTick0 =
    unsafe { sys_tick0::SysTick0::from_ptr(0xe000_e010usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod adc;
pub mod ahbsc;
pub mod bsp32_0;
pub mod cache64_ctrl0;
pub mod cache64_polsel0;
pub mod can;
pub mod cdog;
pub mod cmc0;
pub mod cmp;
pub mod common;
pub mod crc0;
pub mod ctimer;
pub mod dac;
pub mod dac2;
pub mod dm0;
pub mod dma;
pub mod edma_0_tcd;
pub mod edma_1_tcd;
pub mod eim0;
pub mod emvsim;
pub mod enet0;
pub mod erm0;
pub mod evtg0;
pub mod ewm0;
pub mod flexio0;
pub mod flexspi0;
pub mod fmu0;
pub mod fmu0test;
pub mod freqme0;
pub mod gdet0;
pub mod gdet1;
pub mod gpio;
pub mod i3c;
pub mod inputmux0;
pub mod intm0;
pub mod itrc0;
pub mod lp_flexcomm;
pub mod lpi2c;
pub mod lpspi;
pub mod lptmr;
pub mod lpuart;
pub mod mailbox;
pub mod mrt0;
pub mod npx0;
pub mod opamp;
pub mod ostimer0;
pub mod otpc0;
pub mod pdm;
pub mod pint0;
pub mod pkc0;
pub mod plu0;
pub mod port;
pub mod powerquad;
pub mod puf;
pub mod puf_ctrl;
pub mod pwm;
pub mod qdc;
pub mod rtc0;
pub mod rtc_subsystem0;
pub mod s50;
pub mod sai;
pub mod scg0;
pub mod scn_scb;
pub mod sct0;
pub mod sema42_0;
pub mod sinc0;
pub mod smartdma0;
pub mod spc0;
pub mod sys_tick0;
pub mod syscon0;
pub mod syspm;
pub mod tdet0;
pub mod trdc;
pub mod tsi0;
pub mod usbdcd0;
pub mod usbfs0;
pub mod usbhs1__usbc;
pub mod usbhs1__usbnc;
pub mod usbhs1_phy_dcd;
pub mod usbphy;
pub mod usdhc0;
pub mod utick0;
pub mod vbat0;
pub mod vref0;
pub mod wuu0;
pub mod wwdt;
