#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0b476f2 2026-01-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA16"]
    DMA0_DMA16 = 0,
    #[doc = "1 - DMA1_DMA17"]
    DMA1_DMA17 = 1,
    #[doc = "2 - DMA2_DMA18"]
    DMA2_DMA18 = 2,
    #[doc = "3 - DMA3_DMA19"]
    DMA3_DMA19 = 3,
    #[doc = "4 - DMA4_DMA20"]
    DMA4_DMA20 = 4,
    #[doc = "5 - DMA5_DMA21"]
    DMA5_DMA21 = 5,
    #[doc = "6 - DMA6_DMA22"]
    DMA6_DMA22 = 6,
    #[doc = "7 - DMA7_DMA23"]
    DMA7_DMA23 = 7,
    #[doc = "8 - DMA8_DMA24"]
    DMA8_DMA24 = 8,
    #[doc = "9 - DMA9_DMA25"]
    DMA9_DMA25 = 9,
    #[doc = "10 - DMA10_DMA26"]
    DMA10_DMA26 = 10,
    #[doc = "11 - DMA11_DMA27"]
    DMA11_DMA27 = 11,
    #[doc = "12 - DMA12_DMA28"]
    DMA12_DMA28 = 12,
    #[doc = "13 - DMA13_DMA29"]
    DMA13_DMA29 = 13,
    #[doc = "14 - DMA14_DMA30"]
    DMA14_DMA30 = 14,
    #[doc = "15 - DMA15_DMA31"]
    DMA15_DMA31 = 15,
    #[doc = "16 - DMA_ERROR"]
    DMA_ERROR = 16,
    #[doc = "17 - CTI0_ERROR"]
    CTI0_ERROR = 17,
    #[doc = "18 - CTI1_ERROR"]
    CTI1_ERROR = 18,
    #[doc = "19 - CORE"]
    CORE = 19,
    #[doc = "20 - LPUART1"]
    LPUART1 = 20,
    #[doc = "21 - LPUART2"]
    LPUART2 = 21,
    #[doc = "22 - LPUART3"]
    LPUART3 = 22,
    #[doc = "23 - LPUART4"]
    LPUART4 = 23,
    #[doc = "24 - LPUART5"]
    LPUART5 = 24,
    #[doc = "25 - LPUART6"]
    LPUART6 = 25,
    #[doc = "26 - LPUART7"]
    LPUART7 = 26,
    #[doc = "27 - LPUART8"]
    LPUART8 = 27,
    #[doc = "28 - LPI2C1"]
    LPI2C1 = 28,
    #[doc = "29 - LPI2C2"]
    LPI2C2 = 29,
    #[doc = "30 - LPI2C3"]
    LPI2C3 = 30,
    #[doc = "31 - LPI2C4"]
    LPI2C4 = 31,
    #[doc = "32 - LPSPI1"]
    LPSPI1 = 32,
    #[doc = "33 - LPSPI2"]
    LPSPI2 = 33,
    #[doc = "34 - LPSPI3"]
    LPSPI3 = 34,
    #[doc = "35 - LPSPI4"]
    LPSPI4 = 35,
    #[doc = "36 - CAN1"]
    CAN1 = 36,
    #[doc = "37 - CAN2"]
    CAN2 = 37,
    #[doc = "38 - FLEXRAM"]
    FLEXRAM = 38,
    #[doc = "39 - KPP"]
    KPP = 39,
    #[doc = "40 - TSC_DIG"]
    TSC_DIG = 40,
    #[doc = "41 - GPR_IRQ"]
    GPR_IRQ = 41,
    #[doc = "42 - LCDIF"]
    LCDIF = 42,
    #[doc = "43 - CSI"]
    CSI = 43,
    #[doc = "44 - PXP"]
    PXP = 44,
    #[doc = "45 - WDOG2"]
    WDOG2 = 45,
    #[doc = "46 - SNVS_HP_WRAPPER"]
    SNVS_HP_WRAPPER = 46,
    #[doc = "47 - SNVS_HP_WRAPPER_TZ"]
    SNVS_HP_WRAPPER_TZ = 47,
    #[doc = "48 - SNVS_LP_WRAPPER"]
    SNVS_LP_WRAPPER = 48,
    #[doc = "49 - CSU"]
    CSU = 49,
    #[doc = "50 - DCP"]
    DCP = 50,
    #[doc = "51 - DCP_VMI"]
    DCP_VMI = 51,
    #[doc = "52 - RESERVED68"]
    RESERVED68 = 52,
    #[doc = "53 - TRNG"]
    TRNG = 53,
    #[doc = "54 - SJC"]
    SJC = 54,
    #[doc = "55 - BEE"]
    BEE = 55,
    #[doc = "56 - SAI1"]
    SAI1 = 56,
    #[doc = "57 - SAI2"]
    SAI2 = 57,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX = 58,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX = 59,
    #[doc = "60 - SPDIF"]
    SPDIF = 60,
    #[doc = "61 - PMU_EVENT"]
    PMU_EVENT = 61,
    #[doc = "62 - RESERVED78"]
    RESERVED78 = 62,
    #[doc = "63 - TEMP_LOW_HIGH"]
    TEMP_LOW_HIGH = 63,
    #[doc = "64 - TEMP_PANIC"]
    TEMP_PANIC = 64,
    #[doc = "65 - USB_PHY1"]
    USB_PHY1 = 65,
    #[doc = "66 - USB_PHY2"]
    USB_PHY2 = 66,
    #[doc = "67 - ADC1"]
    ADC1 = 67,
    #[doc = "68 - ADC2"]
    ADC2 = 68,
    #[doc = "69 - DCDC"]
    DCDC = 69,
    #[doc = "70 - RESERVED86"]
    RESERVED86 = 70,
    #[doc = "71 - GPIO10"]
    GPIO10 = 71,
    #[doc = "72 - GPIO1_INT0"]
    GPIO1_INT0 = 72,
    #[doc = "73 - GPIO1_INT1"]
    GPIO1_INT1 = 73,
    #[doc = "74 - GPIO1_INT2"]
    GPIO1_INT2 = 74,
    #[doc = "75 - GPIO1_INT3"]
    GPIO1_INT3 = 75,
    #[doc = "76 - GPIO1_INT4"]
    GPIO1_INT4 = 76,
    #[doc = "77 - GPIO1_INT5"]
    GPIO1_INT5 = 77,
    #[doc = "78 - GPIO1_INT6"]
    GPIO1_INT6 = 78,
    #[doc = "79 - GPIO1_INT7"]
    GPIO1_INT7 = 79,
    #[doc = "80 - GPIO1_COMBINED_0_15"]
    GPIO1_COMBINED_0_15 = 80,
    #[doc = "81 - GPIO1_COMBINED_16_31"]
    GPIO1_COMBINED_16_31 = 81,
    #[doc = "82 - GPIO2_COMBINED_0_15"]
    GPIO2_COMBINED_0_15 = 82,
    #[doc = "83 - GPIO2_COMBINED_16_31"]
    GPIO2_COMBINED_16_31 = 83,
    #[doc = "84 - GPIO3_COMBINED_0_15"]
    GPIO3_COMBINED_0_15 = 84,
    #[doc = "85 - GPIO3_COMBINED_16_31"]
    GPIO3_COMBINED_16_31 = 85,
    #[doc = "86 - GPIO4_COMBINED_0_15"]
    GPIO4_COMBINED_0_15 = 86,
    #[doc = "87 - GPIO4_COMBINED_16_31"]
    GPIO4_COMBINED_16_31 = 87,
    #[doc = "88 - GPIO5_COMBINED_0_15"]
    GPIO5_COMBINED_0_15 = 88,
    #[doc = "89 - GPIO5_COMBINED_16_31"]
    GPIO5_COMBINED_16_31 = 89,
    #[doc = "90 - FLEXIO1"]
    FLEXIO1 = 90,
    #[doc = "91 - FLEXIO2"]
    FLEXIO2 = 91,
    #[doc = "92 - WDOG1"]
    WDOG1 = 92,
    #[doc = "93 - RTWDOG"]
    RTWDOG = 93,
    #[doc = "94 - EWM"]
    EWM = 94,
    #[doc = "95 - CCM_1"]
    CCM_1 = 95,
    #[doc = "96 - CCM_2"]
    CCM_2 = 96,
    #[doc = "97 - GPC"]
    GPC = 97,
    #[doc = "98 - SRC"]
    SRC = 98,
    #[doc = "99 - RESERVED115"]
    RESERVED115 = 99,
    #[doc = "100 - GPT1"]
    GPT1 = 100,
    #[doc = "101 - GPT2"]
    GPT2 = 101,
    #[doc = "102 - PWM1_0"]
    PWM1_0 = 102,
    #[doc = "103 - PWM1_1"]
    PWM1_1 = 103,
    #[doc = "104 - PWM1_2"]
    PWM1_2 = 104,
    #[doc = "105 - PWM1_3"]
    PWM1_3 = 105,
    #[doc = "106 - PWM1_FAULT"]
    PWM1_FAULT = 106,
    #[doc = "107 - FLEXSPI2"]
    FLEXSPI2 = 107,
    #[doc = "108 - FLEXSPI"]
    FLEXSPI = 108,
    #[doc = "109 - SEMC"]
    SEMC = 109,
    #[doc = "110 - USDHC1"]
    USDHC1 = 110,
    #[doc = "111 - USDHC2"]
    USDHC2 = 111,
    #[doc = "112 - USB_OTG2"]
    USB_OTG2 = 112,
    #[doc = "113 - USB_OTG1"]
    USB_OTG1 = 113,
    #[doc = "114 - ENET"]
    ENET = 114,
    #[doc = "115 - ENET_1588_TIMER"]
    ENET_1588_TIMER = 115,
    #[doc = "116 - XBAR1_IRQ_0_1"]
    XBAR1_IRQ_0_1 = 116,
    #[doc = "117 - XBAR1_IRQ_2_3"]
    XBAR1_IRQ_2_3 = 117,
    #[doc = "118 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0 = 118,
    #[doc = "119 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1 = 119,
    #[doc = "120 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2 = 120,
    #[doc = "121 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ = 121,
    #[doc = "122 - PIT"]
    PIT = 122,
    #[doc = "123 - ACMP1"]
    ACMP1 = 123,
    #[doc = "124 - ACMP2"]
    ACMP2 = 124,
    #[doc = "125 - ACMP3"]
    ACMP3 = 125,
    #[doc = "126 - ACMP4"]
    ACMP4 = 126,
    #[doc = "127 - RESERVED143"]
    RESERVED143 = 127,
    #[doc = "128 - RESERVED144"]
    RESERVED144 = 128,
    #[doc = "129 - ENC1"]
    ENC1 = 129,
    #[doc = "130 - ENC2"]
    ENC2 = 130,
    #[doc = "131 - ENC3"]
    ENC3 = 131,
    #[doc = "132 - ENC4"]
    ENC4 = 132,
    #[doc = "133 - TMR1"]
    TMR1 = 133,
    #[doc = "134 - TMR2"]
    TMR2 = 134,
    #[doc = "135 - TMR3"]
    TMR3 = 135,
    #[doc = "136 - TMR4"]
    TMR4 = 136,
    #[doc = "137 - PWM2_0"]
    PWM2_0 = 137,
    #[doc = "138 - PWM2_1"]
    PWM2_1 = 138,
    #[doc = "139 - PWM2_2"]
    PWM2_2 = 139,
    #[doc = "140 - PWM2_3"]
    PWM2_3 = 140,
    #[doc = "141 - PWM2_FAULT"]
    PWM2_FAULT = 141,
    #[doc = "142 - PWM3_0"]
    PWM3_0 = 142,
    #[doc = "143 - PWM3_1"]
    PWM3_1 = 143,
    #[doc = "144 - PWM3_2"]
    PWM3_2 = 144,
    #[doc = "145 - PWM3_3"]
    PWM3_3 = 145,
    #[doc = "146 - PWM3_FAULT"]
    PWM3_FAULT = 146,
    #[doc = "147 - PWM4_0"]
    PWM4_0 = 147,
    #[doc = "148 - PWM4_1"]
    PWM4_1 = 148,
    #[doc = "149 - PWM4_2"]
    PWM4_2 = 149,
    #[doc = "150 - PWM4_3"]
    PWM4_3 = 150,
    #[doc = "151 - PWM4_FAULT"]
    PWM4_FAULT = 151,
    #[doc = "152 - ENET2"]
    ENET2 = 152,
    #[doc = "153 - ENET2_1588_TIMER"]
    ENET2_1588_TIMER = 153,
    #[doc = "154 - CAN3"]
    CAN3 = 154,
    #[doc = "155 - RESERVED171"]
    RESERVED171 = 155,
    #[doc = "156 - FLEXIO3"]
    FLEXIO3 = 156,
    #[doc = "157 - GPIO6_7_8_9"]
    GPIO6_7_8_9 = 157,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "AIPSTZ Control Registers"]
pub const AIPSTZ1: aipstz::Aipstz = unsafe { aipstz::Aipstz::from_ptr(0x4007_c000usize as _) };
#[doc = "DCDC"]
pub const DCDC: dcdc::Dcdc = unsafe { dcdc::Dcdc::from_ptr(0x4008_0000usize as _) };
#[doc = "PIT"]
pub const PIT: pit::Pit = unsafe { pit::Pit::from_ptr(0x4008_4000usize as _) };
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub const CMP1: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4009_4000usize as _) };
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub const CMP2: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4009_4008usize as _) };
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub const CMP3: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4009_4010usize as _) };
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub const CMP4: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0x4009_4018usize as _) };
#[doc = "IOMUXC"]
pub const IOMUXC_SNVS_GPR: iomuxc_snvs_gpr::IomuxcSnvsGpr =
    unsafe { iomuxc_snvs_gpr::IomuxcSnvsGpr::from_ptr(0x400a_4000usize as _) };
#[doc = "IOMUXC_SNVS"]
pub const IOMUXC_SNVS: iomuxc_snvs::IomuxcSnvs =
    unsafe { iomuxc_snvs::IomuxcSnvs::from_ptr(0x400a_8000usize as _) };
#[doc = "IOMUXC_GPR"]
pub const IOMUXC_GPR: iomuxc_gpr::IomuxcGpr =
    unsafe { iomuxc_gpr::IomuxcGpr::from_ptr(0x400a_c000usize as _) };
#[doc = "FLEXRAM"]
pub const FLEXRAM: flexram::Flexram = unsafe { flexram::Flexram::from_ptr(0x400b_0000usize as _) };
#[doc = "EWM"]
pub const EWM: ewm::Ewm = unsafe { ewm::Ewm::from_ptr(0x400b_4000usize as _) };
#[doc = "WDOG"]
pub const WDOG1: wdog::Wdog = unsafe { wdog::Wdog::from_ptr(0x400b_8000usize as _) };
#[doc = "WDOG"]
pub const RTWDOG: rtwdog::Rtwdog = unsafe { rtwdog::Rtwdog::from_ptr(0x400b_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO5: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400c_0000usize as _) };
#[doc = "Analog-to-Digital Converter"]
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400c_4000usize as _) };
#[doc = "Analog-to-Digital Converter"]
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x400c_8000usize as _) };
#[doc = "TRNG"]
pub const TRNG: trng::Trng = unsafe { trng::Trng::from_ptr(0x400c_c000usize as _) };
#[doc = "WDOG"]
pub const WDOG2: wdog::Wdog = unsafe { wdog::Wdog::from_ptr(0x400d_0000usize as _) };
#[doc = "SNVS"]
pub const SNVS: snvs::Snvs = unsafe { snvs::Snvs::from_ptr(0x400d_4000usize as _) };
#[doc = "CCM_ANALOG"]
pub const CCM_ANALOG: ccm_analog::CcmAnalog =
    unsafe { ccm_analog::CcmAnalog::from_ptr(0x400d_8000usize as _) };
#[doc = "PMU"]
pub const PMU: pmu::Pmu = unsafe { pmu::Pmu::from_ptr(0x400d_8000usize as _) };
#[doc = "Temperature Monitor"]
pub const TEMPMON: tempmon::Tempmon = unsafe { tempmon::Tempmon::from_ptr(0x400d_8000usize as _) };
#[doc = "USB Analog"]
pub const USB_ANALOG: usb_analog::UsbAnalog =
    unsafe { usb_analog::UsbAnalog::from_ptr(0x400d_8000usize as _) };
#[doc = "XTALOSC24M"]
pub const XTALOSC24M: xtalosc24m::Xtalosc24m =
    unsafe { xtalosc24m::Xtalosc24m::from_ptr(0x400d_8000usize as _) };
#[doc = "USBPHY Register Reference Index"]
pub const USBPHY1: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x400d_9000usize as _) };
#[doc = "USBPHY Register Reference Index"]
pub const USBPHY2: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x400d_a000usize as _) };
#[doc = "CSU registers"]
pub const CSU: csu::Csu = unsafe { csu::Csu::from_ptr(0x400d_c000usize as _) };
#[doc = "Touch Screen Controller"]
pub const TSC: tsc::Tsc = unsafe { tsc::Tsc::from_ptr(0x400e_0000usize as _) };
#[doc = "DMA"]
pub const DMA0: dma0::Dma0 = unsafe { dma0::Dma0::from_ptr(0x400e_8000usize as _) };
#[doc = "DMAMUX"]
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x400e_c000usize as _) };
#[doc = "GPC"]
pub const GPC: gpc::Gpc = unsafe { gpc::Gpc::from_ptr(0x400f_4000usize as _) };
#[doc = "PGC"]
pub const PGC: pgc::Pgc = unsafe { pgc::Pgc::from_ptr(0x400f_4000usize as _) };
#[doc = "SRC"]
pub const SRC: src::Src = unsafe { src::Src::from_ptr(0x400f_8000usize as _) };
#[doc = "CCM"]
pub const CCM: ccm::Ccm = unsafe { ccm::Ccm::from_ptr(0x400f_c000usize as _) };
#[doc = "AIPSTZ Control Registers"]
pub const AIPSTZ2: aipstz::Aipstz = unsafe { aipstz::Aipstz::from_ptr(0x4017_c000usize as _) };
#[doc = "ROMC"]
pub const ROMC: romc::Romc = unsafe { romc::Romc::from_ptr(0x4018_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4018_4000usize as _) };
#[doc = "LPUART"]
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4018_8000usize as _) };
#[doc = "LPUART"]
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4018_c000usize as _) };
#[doc = "LPUART"]
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4019_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4019_4000usize as _) };
#[doc = "LPUART"]
pub const LPUART6: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4019_8000usize as _) };
#[doc = "LPUART"]
pub const LPUART7: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4019_c000usize as _) };
#[doc = "LPUART"]
pub const LPUART8: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x401a_0000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO1: flexio1::Flexio1 = unsafe { flexio1::Flexio1::from_ptr(0x401a_c000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO2: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x401b_0000usize as _) };
#[doc = "GPIO"]
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x401b_8000usize as _) };
#[doc = "GPIO"]
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x401b_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x401c_0000usize as _) };
#[doc = "GPIO"]
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x401c_4000usize as _) };
#[doc = "GPIO"]
pub const GPIO10: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x401c_8000usize as _) };
#[doc = "FLEXCAN"]
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x401d_0000usize as _) };
#[doc = "FLEXCAN"]
pub const CAN2: can::Can = unsafe { can::Can::from_ptr(0x401d_4000usize as _) };
#[doc = "CAN"]
pub const CAN3: can3::Can3 = unsafe { can3::Can3::from_ptr(0x401d_8000usize as _) };
#[doc = "TMR"]
pub const TMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x401d_c000usize as _) };
#[doc = "TMR"]
pub const TMR2: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x401e_0000usize as _) };
#[doc = "TMR"]
pub const TMR3: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x401e_4000usize as _) };
#[doc = "TMR"]
pub const TMR4: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0x401e_8000usize as _) };
#[doc = "GPT"]
pub const GPT1: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x401e_c000usize as _) };
#[doc = "GPT"]
pub const GPT2: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x401f_0000usize as _) };
#[doc = "OCOTP"]
pub const OCOTP: ocotp::Ocotp = unsafe { ocotp::Ocotp::from_ptr(0x401f_4000usize as _) };
#[doc = "IOMUXC"]
pub const IOMUXC: iomuxc::Iomuxc = unsafe { iomuxc::Iomuxc::from_ptr(0x401f_8000usize as _) };
#[doc = "KPP Registers"]
pub const KPP: kpp::Kpp = unsafe { kpp::Kpp::from_ptr(0x401f_c000usize as _) };
#[doc = "AIPSTZ Control Registers"]
pub const AIPSTZ3: aipstz::Aipstz = unsafe { aipstz::Aipstz::from_ptr(0x4027_c000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI2: flex_spi::FlexSpi =
    unsafe { flex_spi::FlexSpi::from_ptr(0x402a_4000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI: flex_spi::FlexSpi =
    unsafe { flex_spi::FlexSpi::from_ptr(0x402a_8000usize as _) };
#[doc = "PXP v2.0 Register Reference Index"]
pub const PXP: pxp::Pxp = unsafe { pxp::Pxp::from_ptr(0x402b_4000usize as _) };
#[doc = "LCDIF Register Reference Index"]
pub const LCDIF: lcdif::Lcdif = unsafe { lcdif::Lcdif::from_ptr(0x402b_8000usize as _) };
#[doc = "CSI"]
pub const CSI: csi::Csi = unsafe { csi::Csi::from_ptr(0x402b_c000usize as _) };
#[doc = "uSDHC"]
pub const USDHC1: usdhc::Usdhc = unsafe { usdhc::Usdhc::from_ptr(0x402c_0000usize as _) };
#[doc = "uSDHC"]
pub const USDHC2: usdhc::Usdhc = unsafe { usdhc::Usdhc::from_ptr(0x402c_4000usize as _) };
#[doc = "ENET"]
pub const ENET2: enet::Enet = unsafe { enet::Enet::from_ptr(0x402d_4000usize as _) };
#[doc = "ENET"]
pub const ENET: enet::Enet = unsafe { enet::Enet::from_ptr(0x402d_8000usize as _) };
#[doc = "USB"]
pub const USB1: usb::Usb = unsafe { usb::Usb::from_ptr(0x402e_0000usize as _) };
#[doc = "USB"]
pub const USBNC1: usbnc1::Usbnc1 = unsafe { usbnc1::Usbnc1::from_ptr(0x402e_0000usize as _) };
#[doc = "USB"]
pub const USBNC2: usbnc2::Usbnc2 = unsafe { usbnc2::Usbnc2::from_ptr(0x402e_0004usize as _) };
#[doc = "USB"]
pub const USB2: usb::Usb = unsafe { usb::Usb::from_ptr(0x402e_0200usize as _) };
#[doc = "SEMC"]
pub const SEMC: semc::Semc = unsafe { semc::Semc::from_ptr(0x402f_0000usize as _) };
#[doc = "DCP register reference index"]
pub const DCP: dcp::Dcp = unsafe { dcp::Dcp::from_ptr(0x402f_c000usize as _) };
#[doc = "AIPSTZ Control Registers"]
pub const AIPSTZ4: aipstz::Aipstz = unsafe { aipstz::Aipstz::from_ptr(0x4037_c000usize as _) };
#[doc = "SPDIF"]
pub const SPDIF: spdif::Spdif = unsafe { spdif::Spdif::from_ptr(0x4038_0000usize as _) };
#[doc = "SAI"]
pub const SAI1: sai1::Sai1 = unsafe { sai1::Sai1::from_ptr(0x4038_4000usize as _) };
#[doc = "SAI"]
pub const SAI2: sai::Sai = unsafe { sai::Sai::from_ptr(0x4038_8000usize as _) };
#[doc = "SAI"]
pub const SAI3: sai::Sai = unsafe { sai::Sai::from_ptr(0x4038_c000usize as _) };
#[doc = "LPSPI"]
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4039_4000usize as _) };
#[doc = "LPSPI"]
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4039_8000usize as _) };
#[doc = "LPSPI"]
pub const LPSPI3: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4039_c000usize as _) };
#[doc = "LPSPI"]
pub const LPSPI4: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x403a_0000usize as _) };
#[doc = "ADC_ETC"]
pub const ADC_ETC: adc_etc::AdcEtc = unsafe { adc_etc::AdcEtc::from_ptr(0x403b_0000usize as _) };
#[doc = "AND/OR/INVERT module"]
pub const AOI1: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x403b_4000usize as _) };
#[doc = "AND/OR/INVERT module"]
pub const AOI2: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x403b_8000usize as _) };
#[doc = "Crossbar Switch"]
pub const XBARA1: xbara1::Xbara1 = unsafe { xbara1::Xbara1::from_ptr(0x403b_c000usize as _) };
#[doc = "Crossbar Switch"]
pub const XBARB2: xbara::Xbara = unsafe { xbara::Xbara::from_ptr(0x403c_0000usize as _) };
#[doc = "Crossbar Switch"]
pub const XBARB3: xbara::Xbara = unsafe { xbara::Xbara::from_ptr(0x403c_4000usize as _) };
#[doc = "QDC"]
pub const ENC1: enc::Enc = unsafe { enc::Enc::from_ptr(0x403c_8000usize as _) };
#[doc = "QDC"]
pub const ENC2: enc::Enc = unsafe { enc::Enc::from_ptr(0x403c_c000usize as _) };
#[doc = "QDC"]
pub const ENC3: enc::Enc = unsafe { enc::Enc::from_ptr(0x403d_0000usize as _) };
#[doc = "QDC"]
pub const ENC4: enc::Enc = unsafe { enc::Enc::from_ptr(0x403d_4000usize as _) };
#[doc = "PWM"]
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x403d_c000usize as _) };
#[doc = "PWM"]
pub const PWM2: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x403e_0000usize as _) };
#[doc = "PWM"]
pub const PWM3: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x403e_4000usize as _) };
#[doc = "PWM"]
pub const PWM4: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x403e_8000usize as _) };
#[doc = "Bus Encryption Engine"]
pub const BEE: bee::Bee = unsafe { bee::Bee::from_ptr(0x403e_c000usize as _) };
#[doc = "LPI2C"]
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x403f_0000usize as _) };
#[doc = "LPI2C"]
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x403f_4000usize as _) };
#[doc = "LPI2C"]
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x403f_8000usize as _) };
#[doc = "LPI2C"]
pub const LPI2C4: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x403f_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO6: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4200_0000usize as _) };
#[doc = "GPIO"]
pub const GPIO7: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4200_4000usize as _) };
#[doc = "GPIO"]
pub const GPIO8: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4200_8000usize as _) };
#[doc = "GPIO"]
pub const GPIO9: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4200_c000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO3: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x4202_0000usize as _) };
#[doc = "CM7_MCM"]
pub const CM7_MCM: cm7_mcm::Cm7Mcm = unsafe { cm7_mcm::Cm7Mcm::from_ptr(0xe008_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod acmp;
pub mod adc;
pub mod adc_etc;
pub mod aipstz;
pub mod aoi;
pub mod bee;
pub mod can;
pub mod can3;
pub mod ccm;
pub mod ccm_analog;
pub mod cm7_mcm;
pub mod common;
pub mod csi;
pub mod csu;
pub mod dcdc;
pub mod dcp;
pub mod dma0;
pub mod dmamux;
pub mod enc;
pub mod enet;
pub mod ewm;
pub mod flex_spi;
pub mod flexio;
pub mod flexio1;
pub mod flexram;
pub mod gpc;
pub mod gpio;
pub mod gpt;
pub mod iomuxc;
pub mod iomuxc_gpr;
pub mod iomuxc_snvs;
pub mod iomuxc_snvs_gpr;
pub mod kpp;
pub mod lcdif;
pub mod lpi2c;
pub mod lpspi;
pub mod lpuart;
pub mod ocotp;
pub mod pgc;
pub mod pit;
pub mod pmu;
pub mod pwm;
pub mod pxp;
pub mod romc;
pub mod rtwdog;
pub mod sai;
pub mod sai1;
pub mod semc;
pub mod snvs;
pub mod spdif;
pub mod src;
pub mod tempmon;
pub mod tmr;
pub mod trng;
pub mod tsc;
pub mod usb;
pub mod usb_analog;
pub mod usbnc1;
pub mod usbnc2;
pub mod usbphy;
pub mod usdhc;
pub mod wdog;
pub mod xbara;
pub mod xbara1;
pub mod xtalosc24m;
