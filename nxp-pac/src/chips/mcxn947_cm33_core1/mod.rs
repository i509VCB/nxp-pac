#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "149 - ETB0"]
    ETB0 = 149,
    #[doc = "155 - CTI0"]
    CTI0 = 155,
    #[doc = "47 - PINT0"]
    PINT0 = 47,
    #[doc = "31 - CTIMER0"]
    CTIMER0 = 31,
    #[doc = "32 - CTIMER1"]
    CTIMER1 = 32,
    #[doc = "34 - CTIMER2"]
    CTIMER2 = 34,
    #[doc = "55 - CTIMER3"]
    CTIMER3 = 55,
    #[doc = "56 - CTIMER4"]
    CTIMER4 = 56,
    #[doc = "71 - FREQME"]
    FREQME = 71,
    #[doc = "29 - UTICK0"]
    UTICK0 = 29,
    #[doc = "30 - MRT0"]
    MRT0 = 30,
    #[doc = "152 - WWDT0"]
    WWDT0 = 152,
    #[doc = "153 - WWDT1"]
    WWDT1 = 153,
    #[doc = "95 - I3C0"]
    I3C0 = 95,
    #[doc = "96 - I3C1"]
    I3C1 = 96,
    #[doc = "98 - GDET"]
    GDET = 98,
    #[doc = "132 - ITRC0"]
    ITRC0 = 132,
    #[doc = "74 - PKC"]
    PKC = 74,
    #[doc = "135 - PKC_ERR"]
    PKC_ERR = 135,
    #[doc = "75 - PUF"]
    PUF = 75,
    #[doc = "133 - BSP32"]
    BSP32 = 133,
    #[doc = "53 - SMARTDMA"]
    SMARTDMA = 53,
    #[doc = "70 - PLU"]
    PLU = 70,
    #[doc = "27 - GPIO50"]
    GPIO50 = 27,
    #[doc = "28 - GPIO51"]
    GPIO51 = 28,
    #[doc = "148 - PORT_EFT"]
    PORT_EFT = 148,
    #[doc = "138 - FMU0"]
    FMU0 = 138,
    #[doc = "145 - SCG"]
    SCG = 145,
    #[doc = "146 - SPC"]
    SPC = 146,
    #[doc = "147 - WUU"]
    WUU = 147,
    #[doc = "154 - CMC0"]
    CMC0 = 154,
    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,
    #[doc = "143 - LPTMR0"]
    LPTMR0 = 143,
    #[doc = "144 - LPTMR1"]
    LPTMR1 = 144,
    #[doc = "52 - RTC"]
    RTC = 52,
    #[doc = "101 - TSI_END_OF_SCAN"]
    TSI_END_OF_SCAN = 101,
    #[doc = "102 - TSI_OUT_OF_SCAN"]
    TSI_OUT_OF_SCAN = 102,
    #[doc = "109 - HSCMP0"]
    HSCMP0 = 109,
    #[doc = "110 - HSCMP1"]
    HSCMP1 = 110,
    #[doc = "111 - HSCMP2"]
    HSCMP2 = 111,
    #[doc = "73 - ELS"]
    ELS = 73,
    #[doc = "134 - ELS_ERR"]
    ELS_ERR = 134,
    #[doc = "99 - VBAT0"]
    VBAT0 = 99,
    #[doc = "136 - ERM_SINGLE_BIT_ERROR"]
    ERM_SINGLE_BIT_ERROR = 136,
    #[doc = "137 - ERM_MULTI_BIT_ERROR"]
    ERM_MULTI_BIT_ERROR = 137,
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
    #[doc = "33 - SCT0"]
    SCT0 = 33,
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
    #[doc = "54 - MAILBOX"]
    MAILBOX = 54,
    #[doc = "93 - CDOG0"]
    CDOG0 = 93,
    #[doc = "94 - CDOG1"]
    CDOG1 = 94,
    #[doc = "76 - PQ"]
    PQ = 76,
    #[doc = "100 - EWM0"]
    EWM0 = 100,
    #[doc = "72 - SEC_VIO"]
    SEC_VIO = 72,
    #[doc = "58 - FLEXSPI0"]
    FLEXSPI0 = 58,
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
    #[doc = "62 - CAN0"]
    CAN0 = 62,
    #[doc = "63 - CAN1"]
    CAN1 = 63,
    #[doc = "51 - USB0_DCD"]
    USB0_DCD = 51,
    #[doc = "50 - USB0_FS"]
    USB0_FS = 50,
    #[doc = "139 - ETHERNET"]
    ETHERNET = 139,
    #[doc = "140 - ETHERNET_PMT"]
    ETHERNET_PMT = 140,
    #[doc = "141 - ETHERNET_MACLP"]
    ETHERNET_MACLP = 141,
    #[doc = "103 - EMVSIM0"]
    EMVSIM0 = 103,
    #[doc = "104 - EMVSIM1"]
    EMVSIM1 = 104,
    #[doc = "105 - FLEXIO"]
    FLEXIO = 105,
    #[doc = "59 - SAI0"]
    SAI0 = 59,
    #[doc = "60 - SAI1"]
    SAI1 = 60,
    #[doc = "142 - SINC_FILTER"]
    SINC_FILTER = 142,
    #[doc = "61 - USDHC0"]
    USDHC0 = 61,
    #[doc = "66 - USB1_HS_PHY"]
    USB1_HS_PHY = 66,
    #[doc = "67 - USB1_HS"]
    USB1_HS = 67,
    #[doc = "48 - PDM_EVENT"]
    PDM_EVENT = 48,
    #[doc = "45 - ADC0"]
    ADC0 = 45,
    #[doc = "46 - ADC1"]
    ADC1 = 46,
    #[doc = "106 - DAC0"]
    DAC0 = 106,
    #[doc = "107 - DAC1"]
    DAC1 = 107,
    #[doc = "108 - DAC2"]
    DAC2 = 108,
    #[doc = "68 - SEC_HYPERVISOR_CALL"]
    SEC_HYPERVISOR_CALL = 68,
    #[doc = "0 - OR"]
    OR = 0,
    #[doc = "49 - RESERVED65"]
    RESERVED65 = 49,
    #[doc = "64 - RESERVED80"]
    RESERVED80 = 64,
    #[doc = "65 - RESERVED81"]
    RESERVED81 = 65,
    #[doc = "69 - RESERVED85"]
    RESERVED85 = 69,
    #[doc = "97 - NPU"]
    NPU = 97,
    #[doc = "150 - RESERVED166"]
    RESERVED166 = 150,
    #[doc = "151 - RESERVED167"]
    RESERVED167 = 151,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub const SYSCON0: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x40000000 as _) };
pub const PINT0: pint0::Pint0 = unsafe { pint0::Pint0::from_ptr(0x40004000 as _) };
pub const INPUTMUX0: inputmux0::Inputmux0 =
    unsafe { inputmux0::Inputmux0::from_ptr(0x40006000 as _) };
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000C000 as _) };
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000D000 as _) };
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000E000 as _) };
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000F000 as _) };
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40010000 as _) };
pub const FREQME0: freqme::Freqme = unsafe { freqme::Freqme::from_ptr(0x40011000 as _) };
pub const UTICK0: utick::Utick = unsafe { utick::Utick::from_ptr(0x40012000 as _) };
pub const MRT0: mrt::Mrt = unsafe { mrt::Mrt::from_ptr(0x40013000 as _) };
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x40016000 as _) };
pub const WWDT1: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x40017000 as _) };
pub const CACHE64_CTRL0: cache64ctrl::Cache64Ctrl =
    unsafe { cache64ctrl::Cache64Ctrl::from_ptr(0x4001B000 as _) };
pub const CACHE64_POLSEL0: cache64polsel::Cache64Polsel =
    unsafe { cache64polsel::Cache64Polsel::from_ptr(0x4001B000 as _) };
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x40021000 as _) };
pub const I3C1: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x40022000 as _) };
pub const GDET0: gdet::Gdet = unsafe { gdet::Gdet::from_ptr(0x40024000 as _) };
pub const GDET1: gdet::Gdet = unsafe { gdet::Gdet::from_ptr(0x40025000 as _) };
pub const ITRC0: itrc0::Itrc0 = unsafe { itrc0::Itrc0::from_ptr(0x40026000 as _) };
pub const PKC0: pkc0::Pkc0 = unsafe { pkc0::Pkc0::from_ptr(0x4002B000 as _) };
pub const PUF: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002C000 as _) };
pub const PUF_ALIAS1: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002D000 as _) };
pub const PUF_ALIAS2: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002E000 as _) };
pub const PUF_ALIAS3: puf::Puf = unsafe { puf::Puf::from_ptr(0x4002F000 as _) };
pub const PUF_CTRL: puf_ctrl::PufCtrl = unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002C000 as _) };
pub const PUF_CTRL_ALIAS1: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002D000 as _) };
pub const PUF_CTRL_ALIAS2: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002E000 as _) };
pub const PUF_CTRL_ALIAS3: puf_ctrl::PufCtrl =
    unsafe { puf_ctrl::PufCtrl::from_ptr(0x4002F000 as _) };
pub const BSP32_0: bsp32::Bsp32 = unsafe { bsp32::Bsp32::from_ptr(0x40032000 as _) };
pub const SMARTDMA0: smartdma::Smartdma = unsafe { smartdma::Smartdma::from_ptr(0x40033000 as _) };
pub const PLU0: plu0::Plu0 = unsafe { plu0::Plu0::from_ptr(0x40034000 as _) };
pub const GPIO5: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40040000 as _) };
pub const GPIO5_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40041000 as _) };
pub const PORT5: port::Port = unsafe { port::Port::from_ptr(0x40042000 as _) };
pub const FMU0: fmu::Fmu = unsafe { fmu::Fmu::from_ptr(0x40043000 as _) };
pub const FMU0TEST: fmu0test::Fmu0test = unsafe { fmu0test::Fmu0test::from_ptr(0x40043000 as _) };
pub const SCG0: scg::Scg = unsafe { scg::Scg::from_ptr(0x40044000 as _) };
pub const SPC0: spc::Spc = unsafe { spc::Spc::from_ptr(0x40045000 as _) };
pub const WUU0: wuu::Wuu = unsafe { wuu::Wuu::from_ptr(0x40046000 as _) };
pub const CMC0: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x40048000 as _) };
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x40049000 as _) };
pub const LPTMR0: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x4004A000 as _) };
pub const LPTMR1: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x4004B000 as _) };
pub const RTC0: rtc5xx::Rtc = unsafe { rtc5xx::Rtc::from_ptr(0x4004C000 as _) };
pub const RTC_SUBSYSTEM0: rtc_subsystem::RtcSubsystem =
    unsafe { rtc_subsystem::RtcSubsystem::from_ptr(0x4004C000 as _) };
pub const TSI0: tsi::Tsi = unsafe { tsi::Tsi::from_ptr(0x40050000 as _) };
pub const CMP0: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x40051000 as _) };
pub const CMP1: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x40052000 as _) };
pub const CMP2: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x40053000 as _) };
pub const ELS: els::Els = unsafe { els::Els::from_ptr(0x40054000 as _) };
pub const ELS_ALIAS1: els::Els = unsafe { els::Els::from_ptr(0x40055000 as _) };
pub const ELS_ALIAS2: els::Els = unsafe { els::Els::from_ptr(0x40056000 as _) };
pub const ELS_ALIAS3: els::Els = unsafe { els::Els::from_ptr(0x40057000 as _) };
pub const TDET0: tdet::Tdet = unsafe { tdet::Tdet::from_ptr(0x40058000 as _) };
pub const VBAT0: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x40059000 as _) };
pub const EIM0: eim::Eim = unsafe { eim::Eim::from_ptr(0x4005B000 as _) };
pub const ERM0: erm::Erm = unsafe { erm::Erm::from_ptr(0x4005C000 as _) };
pub const INTM0: intm0::Intm0 = unsafe { intm0::Intm0::from_ptr(0x4005D000 as _) };
pub const EDMA_0_TCD: edma_0_tcd::Edma0Tcd =
    unsafe { edma_0_tcd::Edma0Tcd::from_ptr(0x40081000 as _) };
pub const SCT0: sct::Sct = unsafe { sct::Sct::from_ptr(0x40091000 as _) };
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40092000 as _) };
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40093000 as _) };
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40094000 as _) };
pub const LPSPI3: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40095000 as _) };
pub const LPSPI4: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400B4000 as _) };
pub const LPSPI5: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400B5000 as _) };
pub const LPSPI6: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400B6000 as _) };
pub const LPSPI7: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400B7000 as _) };
pub const LPSPI8: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400B8000 as _) };
pub const LPSPI9: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x400B9000 as _) };
pub const LPUART0: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x40092000 as _) };
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x40093000 as _) };
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x40094000 as _) };
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x40095000 as _) };
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400B4000 as _) };
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400B5000 as _) };
pub const LPUART6: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400B6000 as _) };
pub const LPUART7: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400B7000 as _) };
pub const LPUART8: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400B8000 as _) };
pub const LPUART9: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400B9000 as _) };
pub const LP_FLEXCOMM0: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x40092000 as _) };
pub const LP_FLEXCOMM1: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x40093000 as _) };
pub const LP_FLEXCOMM2: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x40094000 as _) };
pub const LP_FLEXCOMM3: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x40095000 as _) };
pub const LP_FLEXCOMM4: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400B4000 as _) };
pub const LP_FLEXCOMM5: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400B5000 as _) };
pub const LP_FLEXCOMM6: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400B6000 as _) };
pub const LP_FLEXCOMM7: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400B7000 as _) };
pub const LP_FLEXCOMM8: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400B8000 as _) };
pub const LP_FLEXCOMM9: lp_flexcomm::LpFlexcomm =
    unsafe { lp_flexcomm::LpFlexcomm::from_ptr(0x400B9000 as _) };
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x40092800 as _) };
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x40093800 as _) };
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x40094800 as _) };
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x40095800 as _) };
pub const LPI2C4: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400B4800 as _) };
pub const LPI2C5: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400B5800 as _) };
pub const LPI2C6: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400B6800 as _) };
pub const LPI2C7: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400B7800 as _) };
pub const LPI2C8: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400B8800 as _) };
pub const LPI2C9: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400B9800 as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40096000 as _) };
pub const GPIO0_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40097000 as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40098000 as _) };
pub const GPIO1_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40099000 as _) };
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009A000 as _) };
pub const GPIO2_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009B000 as _) };
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009C000 as _) };
pub const GPIO3_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009D000 as _) };
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009E000 as _) };
pub const GPIO4_ALIAS1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4009F000 as _) };
pub const EDMA_1_TCD: edma_1_tcd::Edma1Tcd =
    unsafe { edma_1_tcd::Edma1Tcd::from_ptr(0x400A1000 as _) };
pub const SEMA42_0: sema42::Sema42 = unsafe { sema42::Sema42::from_ptr(0x400B1000 as _) };
pub const MAILBOX: mailbox::Mailbox = unsafe { mailbox::Mailbox::from_ptr(0x400B2000 as _) };
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x400BB000 as _) };
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x400BC000 as _) };
pub const DM0: dm::Dm = unsafe { dm::Dm::from_ptr(0x400BD000 as _) };
pub const POWERQUAD: powerquad::Powerquad =
    unsafe { powerquad::Powerquad::from_ptr(0x400BF000 as _) };
pub const EWM0: ewm::Ewm = unsafe { ewm::Ewm::from_ptr(0x400C0000 as _) };
pub const CMX_PERFMON0: cmx_perfmon::CmxPerfmon =
    unsafe { cmx_perfmon::CmxPerfmon::from_ptr(0x400C1000 as _) };
pub const CMX_PERFMON1: cmx_perfmon::CmxPerfmon =
    unsafe { cmx_perfmon::CmxPerfmon::from_ptr(0x400C2000 as _) };
pub const TRDC: trdc::Trdc = unsafe { trdc::Trdc::from_ptr(0x400C7000 as _) };
pub const FLEXSPI0: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x400C8000 as _) };
pub const OTPC0: otpc::Otpc = unsafe { otpc::Otpc::from_ptr(0x400C9000 as _) };
pub const CRC0: crc::Crc = unsafe { crc::Crc::from_ptr(0x400CB000 as _) };
pub const NPX0: npx::Npx = unsafe { npx::Npx::from_ptr(0x400CC000 as _) };
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x400CE000 as _) };
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x400D0000 as _) };
pub const QDC0: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400CF000 as _) };
pub const QDC1: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400D1000 as _) };
pub const EVTG0: evtg::Evtg = unsafe { evtg::Evtg::from_ptr(0x400D2000 as _) };
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400D4000 as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x400D8000 as _) };
pub const USBDCD0: usbdcd::Usbdcd = unsafe { usbdcd::Usbdcd::from_ptr(0x400DC000 as _) };
pub const USBFS0: usbfs::Usbfs = unsafe { usbfs::Usbfs::from_ptr(0x400DD000 as _) };
pub const ENET0: enet::Enet = unsafe { enet::Enet::from_ptr(0x40100000 as _) };
pub const EMVSIM0: emvsim::Emvsim = unsafe { emvsim::Emvsim::from_ptr(0x40103000 as _) };
pub const EMVSIM1: emvsim::Emvsim = unsafe { emvsim::Emvsim::from_ptr(0x40104000 as _) };
pub const FLEXIO0: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x40105000 as _) };
pub const SAI0: sai::Sai = unsafe { sai::Sai::from_ptr(0x40106000 as _) };
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x40107000 as _) };
pub const SINC0: sinc::Sinc = unsafe { sinc::Sinc::from_ptr(0x40108000 as _) };
pub const USDHC0: usdhc::Usdhc = unsafe { usdhc::Usdhc::from_ptr(0x40109000 as _) };
pub const USBPHY: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x4010A000 as _) };
pub const USBHS1_PHY_DCD: usbhs1_phy_dcd::Usbhs1PhyDcd =
    unsafe { usbhs1_phy_dcd::Usbhs1PhyDcd::from_ptr(0x4010A800 as _) };
pub const USBHS1__USBC: usbhs1__usbc::Usbhs1Usbc =
    unsafe { usbhs1__usbc::Usbhs1Usbc::from_ptr(0x4010B000 as _) };
pub const USBHS1__USBNC: usbhs1__usbnc::Usbhs1Usbnc =
    unsafe { usbhs1__usbnc::Usbhs1Usbnc::from_ptr(0x4010B200 as _) };
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x4010C000 as _) };
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x4010D000 as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4010E000 as _) };
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x4010F000 as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x40112000 as _) };
pub const OPAMP0: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x40110000 as _) };
pub const OPAMP1: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x40113000 as _) };
pub const OPAMP2: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x40115000 as _) };
pub const VREF0: vref::Vref = unsafe { vref::Vref::from_ptr(0x40111000 as _) };
pub const DAC2: dac2::Dac2 = unsafe { dac2::Dac2::from_ptr(0x40114000 as _) };
pub const PORT0: port::Port = unsafe { port::Port::from_ptr(0x40116000 as _) };
pub const PORT1: port::Port = unsafe { port::Port::from_ptr(0x40117000 as _) };
pub const PORT2: port::Port = unsafe { port::Port::from_ptr(0x40118000 as _) };
pub const PORT3: port::Port = unsafe { port::Port::from_ptr(0x40119000 as _) };
pub const PORT4: port::Port = unsafe { port::Port::from_ptr(0x4011A000 as _) };
pub const AHBSC: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x40120000 as _) };
pub const AHBSC_ALIAS1: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x40121000 as _) };
pub const AHBSC_ALIAS2: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x40122000 as _) };
pub const AHBSC_ALIAS3: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x40123000 as _) };
#[path = "../../meta_peripherals/mcxn/ADC.rs"]
pub mod adc;
#[path = "../../meta_peripherals/mcxn/AHBSC.rs"]
pub mod ahbsc;
#[path = "../../meta_peripherals/mcxn/BSP32.rs"]
pub mod bsp32;
#[path = "../../meta_peripherals/mcxn/CACHE64Ctrl.rs"]
pub mod cache64ctrl;
#[path = "../../meta_peripherals/mcxn/CACHE64Polsel.rs"]
pub mod cache64polsel;
#[path = "../../meta_peripherals/mcxn/CAN.rs"]
pub mod can;
#[path = "../../meta_peripherals/mcxn/CDOG.rs"]
pub mod cdog;
#[path = "../../meta_peripherals/mcxn/CMC.rs"]
pub mod cmc;
#[path = "../../meta_peripherals/mcxn/CMP.rs"]
pub mod cmp;
#[path = "../../meta_peripherals/mcxn/CMX_PERFMON.rs"]
pub mod cmx_perfmon;
pub mod common;
#[path = "../../meta_peripherals/mcxn/CRC.rs"]
pub mod crc;
#[path = "../../meta_peripherals/mcx/CTIMER.rs"]
pub mod ctimer;
#[path = "../../meta_peripherals/mcx/DAC.rs"]
pub mod dac;
#[path = "../../meta_peripherals/mcxn/DAC2.rs"]
pub mod dac2;
#[path = "../../meta_peripherals/mcxn/DM.rs"]
pub mod dm;
#[path = "../../meta_peripherals/mcxn/EDMA_0_TCD.rs"]
pub mod edma_0_tcd;
#[path = "../../meta_peripherals/mcxn/EDMA_1_TCD.rs"]
pub mod edma_1_tcd;
#[path = "../../meta_peripherals/mcxn/EIM.rs"]
pub mod eim;
#[path = "../../meta_peripherals/mcxn/ELS.rs"]
pub mod els;
#[path = "../../meta_peripherals/mcxn/EMVSIM.rs"]
pub mod emvsim;
#[path = "../../meta_peripherals/mcxn/ENET.rs"]
pub mod enet;
#[path = "../../meta_peripherals/mcxn/ERM.rs"]
pub mod erm;
#[path = "../../meta_peripherals/mcxn/EVTG.rs"]
pub mod evtg;
#[path = "../../meta_peripherals/mcxn/EWM.rs"]
pub mod ewm;
#[path = "../../meta_peripherals/mcxn/FLEXIO.rs"]
pub mod flexio;
#[path = "../../meta_peripherals/mcxn/FLEXSPI.rs"]
pub mod flexspi;
#[path = "../../meta_peripherals/mcxn/FMU.rs"]
pub mod fmu;
#[path = "../../meta_peripherals/mcxn/FMU0TEST.rs"]
pub mod fmu0test;
#[path = "../../meta_peripherals/mcxn/FREQME.rs"]
pub mod freqme;
#[path = "../../meta_peripherals/mcxn/GDET.rs"]
pub mod gdet;
#[path = "../../meta_peripherals/mcx/GPIO.rs"]
pub mod gpio;
#[path = "../../meta_peripherals/mcxn/I3C.rs"]
pub mod i3c;
#[path = "../../meta_peripherals/mcxn/INPUTMUX0.rs"]
pub mod inputmux0;
#[path = "../../meta_peripherals/mcxn/INTM0.rs"]
pub mod intm0;
#[path = "../../meta_peripherals/mcxn/ITRC0.rs"]
pub mod itrc0;
#[path = "../../meta_peripherals/mcxn/LP_FLEXCOMM.rs"]
pub mod lp_flexcomm;
#[path = "../../meta_peripherals/mcxn/LPI2C.rs"]
pub mod lpi2c;
#[path = "../../meta_peripherals/mcxn/LPSPI.rs"]
pub mod lpspi;
#[path = "../../meta_peripherals/mcxn/LPTMR.rs"]
pub mod lptmr;
#[path = "../../meta_peripherals/mcxn/LPUART.rs"]
pub mod lpuart;
#[path = "../../meta_peripherals/mcxn/MAILBOX.rs"]
pub mod mailbox;
#[path = "../../meta_peripherals/mcxn/MRT.rs"]
pub mod mrt;
#[path = "../../meta_peripherals/mcxn/NPX.rs"]
pub mod npx;
#[path = "../../meta_peripherals/mcxn/OPAMP.rs"]
pub mod opamp;
#[path = "../../meta_peripherals/mcxn/OSTIMER.rs"]
pub mod ostimer;
#[path = "../../meta_peripherals/mcxn/OTPC.rs"]
pub mod otpc;
#[path = "../../meta_peripherals/mcxn/PDM.rs"]
pub mod pdm;
#[path = "../../meta_peripherals/mcxn/PINT0.rs"]
pub mod pint0;
#[path = "../../meta_peripherals/mcxn/PKC0.rs"]
pub mod pkc0;
#[path = "../../meta_peripherals/mcxn/PLU0.rs"]
pub mod plu0;
#[path = "../../meta_peripherals/mcx/PORT.rs"]
pub mod port;
#[path = "../../meta_peripherals/mcxn/POWERQUAD.rs"]
pub mod powerquad;
#[path = "../../meta_peripherals/mcxn/PUF.rs"]
pub mod puf;
#[path = "../../meta_peripherals/mcxn/PUF_CTRL.rs"]
pub mod puf_ctrl;
#[path = "../../meta_peripherals/mcxn/PWM.rs"]
pub mod pwm;
#[path = "../../meta_peripherals/mcxn/QDC.rs"]
pub mod qdc;
#[path = "../../meta_peripherals/mcx/RTC5xx.rs"]
pub mod rtc5xx;
#[path = "../../meta_peripherals/mcxn/RTC_SUBSYSTEM.rs"]
pub mod rtc_subsystem;
#[path = "../../meta_peripherals/mcxn/SAI.rs"]
pub mod sai;
#[path = "../../meta_peripherals/mcxn/SCG.rs"]
pub mod scg;
#[path = "../../meta_peripherals/mcxn/SCT.rs"]
pub mod sct;
#[path = "../../meta_peripherals/mcxn/SEMA42.rs"]
pub mod sema42;
#[path = "../../meta_peripherals/mcxn/SINC.rs"]
pub mod sinc;
#[path = "../../meta_peripherals/mcxn/SMARTDMA.rs"]
pub mod smartdma;
#[path = "../../meta_peripherals/mcxn/SPC.rs"]
pub mod spc;
#[path = "../../meta_peripherals/mcxn/SYSCON.rs"]
pub mod syscon;
#[path = "../../meta_peripherals/mcxn/TDET.rs"]
pub mod tdet;
#[path = "../../meta_peripherals/mcxn/TRDC.rs"]
pub mod trdc;
#[path = "../../meta_peripherals/mcxn/TSI.rs"]
pub mod tsi;
#[path = "../../meta_peripherals/mcxn/USBDCD.rs"]
pub mod usbdcd;
#[path = "../../meta_peripherals/mcxn/USBFS.rs"]
pub mod usbfs;
#[path = "../../meta_peripherals/mcxn/USBHS1__USBC.rs"]
pub mod usbhs1__usbc;
#[path = "../../meta_peripherals/mcxn/USBHS1__USBNC.rs"]
pub mod usbhs1__usbnc;
#[path = "../../meta_peripherals/mcxn/USBHS1_PHY_DCD.rs"]
pub mod usbhs1_phy_dcd;
#[path = "../../meta_peripherals/mcxn/USBPHY.rs"]
pub mod usbphy;
#[path = "../../meta_peripherals/mcxn/USDHC.rs"]
pub mod usdhc;
#[path = "../../meta_peripherals/mcxn/UTICK.rs"]
pub mod utick;
#[path = "../../meta_peripherals/mcxn/VBAT.rs"]
pub mod vbat;
#[path = "../../meta_peripherals/mcxn/VREF.rs"]
pub mod vref;
#[path = "../../meta_peripherals/mcxn/WUU.rs"]
pub mod wuu;
#[path = "../../meta_peripherals/mcx/WWDT.rs"]
pub mod wwdt;
