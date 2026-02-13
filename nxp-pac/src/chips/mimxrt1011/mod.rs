#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0b476f2 2026-01-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - DMA0"]
    DMA0 = 0,
    #[doc = "1 - DMA1"]
    DMA1 = 1,
    #[doc = "2 - DMA2"]
    DMA2 = 2,
    #[doc = "3 - DMA3"]
    DMA3 = 3,
    #[doc = "4 - DMA4"]
    DMA4 = 4,
    #[doc = "5 - DMA5"]
    DMA5 = 5,
    #[doc = "6 - DMA6"]
    DMA6 = 6,
    #[doc = "7 - DMA7"]
    DMA7 = 7,
    #[doc = "8 - DMA8"]
    DMA8 = 8,
    #[doc = "9 - DMA9"]
    DMA9 = 9,
    #[doc = "10 - DMA10"]
    DMA10 = 10,
    #[doc = "11 - DMA11"]
    DMA11 = 11,
    #[doc = "12 - DMA12"]
    DMA12 = 12,
    #[doc = "13 - DMA13"]
    DMA13 = 13,
    #[doc = "14 - DMA14"]
    DMA14 = 14,
    #[doc = "15 - DMA15"]
    DMA15 = 15,
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
    #[doc = "24 - PIT"]
    PIT = 24,
    #[doc = "25 - USB_OTG1"]
    USB_OTG1 = 25,
    #[doc = "26 - FLEXSPI"]
    FLEXSPI = 26,
    #[doc = "27 - FLEXRAM"]
    FLEXRAM = 27,
    #[doc = "28 - LPI2C1"]
    LPI2C1 = 28,
    #[doc = "29 - LPI2C2"]
    LPI2C2 = 29,
    #[doc = "30 - GPT1"]
    GPT1 = 30,
    #[doc = "31 - GPT2"]
    GPT2 = 31,
    #[doc = "32 - LPSPI1"]
    LPSPI1 = 32,
    #[doc = "33 - LPSPI2"]
    LPSPI2 = 33,
    #[doc = "34 - PWM1_0"]
    PWM1_0 = 34,
    #[doc = "35 - PWM1_1"]
    PWM1_1 = 35,
    #[doc = "36 - PWM1_2"]
    PWM1_2 = 36,
    #[doc = "37 - PWM1_3"]
    PWM1_3 = 37,
    #[doc = "38 - PWM1_FAULT"]
    PWM1_FAULT = 38,
    #[doc = "39 - KPP"]
    KPP = 39,
    #[doc = "40 - SRC"]
    SRC = 40,
    #[doc = "41 - GPR_IRQ"]
    GPR_IRQ = 41,
    #[doc = "42 - CCM_1"]
    CCM_1 = 42,
    #[doc = "43 - CCM_2"]
    CCM_2 = 43,
    #[doc = "44 - EWM"]
    EWM = 44,
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
    #[doc = "54 - RESERVED70"]
    RESERVED70 = 54,
    #[doc = "55 - RESERVED71"]
    RESERVED71 = 55,
    #[doc = "56 - SAI1"]
    SAI1 = 56,
    #[doc = "57 - RTWDOG"]
    RTWDOG = 57,
    #[doc = "58 - SAI3_RX"]
    SAI3_RX = 58,
    #[doc = "59 - SAI3_TX"]
    SAI3_TX = 59,
    #[doc = "60 - SPDIF"]
    SPDIF = 60,
    #[doc = "61 - PMU"]
    PMU = 61,
    #[doc = "62 - XBAR1_IRQ_0_1_2_3"]
    XBAR1_IRQ_0_1_2_3 = 62,
    #[doc = "63 - TEMP_LOW_HIGH"]
    TEMP_LOW_HIGH = 63,
    #[doc = "64 - TEMP_PANIC"]
    TEMP_PANIC = 64,
    #[doc = "65 - USB_PHY"]
    USB_PHY = 65,
    #[doc = "66 - GPC"]
    GPC = 66,
    #[doc = "67 - ADC1"]
    ADC1 = 67,
    #[doc = "68 - FLEXIO1"]
    FLEXIO1 = 68,
    #[doc = "69 - DCDC"]
    DCDC = 69,
    #[doc = "70 - GPIO1_COMBINED_0_15"]
    GPIO1_COMBINED_0_15 = 70,
    #[doc = "71 - GPIO1_COMBINED_16_31"]
    GPIO1_COMBINED_16_31 = 71,
    #[doc = "72 - GPIO2_COMBINED_0_15"]
    GPIO2_COMBINED_0_15 = 72,
    #[doc = "73 - GPIO5_COMBINED_0_15"]
    GPIO5_COMBINED_0_15 = 73,
    #[doc = "74 - WDOG1"]
    WDOG1 = 74,
    #[doc = "75 - ADC_ETC_IRQ0"]
    ADC_ETC_IRQ0 = 75,
    #[doc = "76 - ADC_ETC_IRQ1"]
    ADC_ETC_IRQ1 = 76,
    #[doc = "77 - ADC_ETC_IRQ2"]
    ADC_ETC_IRQ2 = 77,
    #[doc = "78 - ADC_ETC_IRQ3"]
    ADC_ETC_IRQ3 = 78,
    #[doc = "79 - ADC_ETC_ERROR_IRQ"]
    ADC_ETC_ERROR_IRQ = 79,
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
#[doc = "ADC_ETC"]
pub const ADC_ETC: adc_etc::AdcEtc = unsafe { adc_etc::AdcEtc::from_ptr(0x4008_8000usize as _) };
#[doc = "AND/OR/INVERT module"]
pub const AOI: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4009_4000usize as _) };
#[doc = "Crossbar Switch"]
pub const XBARA: xbara::Xbara = unsafe { xbara::Xbara::from_ptr(0x4009_8000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x400a_0000usize as _) };
#[doc = "OTFAD"]
pub const OTFAD: otfad::Otfad = unsafe { otfad::Otfad::from_ptr(0x400a_0000usize as _) };
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
pub const USBPHY: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x400d_9000usize as _) };
#[doc = "CSU registers"]
pub const CSU: csu::Csu = unsafe { csu::Csu::from_ptr(0x400d_c000usize as _) };
#[doc = "USB"]
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x400e_4000usize as _) };
#[doc = "USB"]
pub const USBNC: usbnc::Usbnc = unsafe { usbnc::Usbnc::from_ptr(0x400e_4000usize as _) };
#[doc = "DMA"]
pub const DMA0: dma::Dma = unsafe { dma::Dma::from_ptr(0x400e_8000usize as _) };
#[doc = "DMAMUX"]
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x400e_c000usize as _) };
#[doc = "DCP register reference index"]
pub const DCP: dcp::Dcp = unsafe { dcp::Dcp::from_ptr(0x400f_0000usize as _) };
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
#[doc = "LPUART"]
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4018_4000usize as _) };
#[doc = "LPUART"]
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4018_8000usize as _) };
#[doc = "LPUART"]
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4018_c000usize as _) };
#[doc = "LPUART"]
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4019_0000usize as _) };
#[doc = "LPSPI"]
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4019_4000usize as _) };
#[doc = "LPSPI"]
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4019_8000usize as _) };
#[doc = "LPI2C"]
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x401a_4000usize as _) };
#[doc = "LPI2C"]
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x401a_8000usize as _) };
#[doc = "FLEXIO"]
pub const FLEXIO1: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x401a_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x401b_8000usize as _) };
#[doc = "PWM"]
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x401c_c000usize as _) };
#[doc = "SPDIF"]
pub const SPDIF: spdif::Spdif = unsafe { spdif::Spdif::from_ptr(0x401d_c000usize as _) };
#[doc = "SAI"]
pub const SAI1: sai::Sai = unsafe { sai::Sai::from_ptr(0x401e_0000usize as _) };
#[doc = "SAI"]
pub const SAI3: sai::Sai = unsafe { sai::Sai::from_ptr(0x401e_8000usize as _) };
#[doc = "GPT"]
pub const GPT1: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x401e_c000usize as _) };
#[doc = "GPT"]
pub const GPT2: gpt::Gpt = unsafe { gpt::Gpt::from_ptr(0x401f_0000usize as _) };
#[doc = "no description available"]
pub const OCOTP: ocotp::Ocotp = unsafe { ocotp::Ocotp::from_ptr(0x401f_4000usize as _) };
#[doc = "IOMUXC"]
pub const IOMUXC: iomuxc::Iomuxc = unsafe { iomuxc::Iomuxc::from_ptr(0x401f_8000usize as _) };
#[doc = "KPP Registers"]
pub const KPP: kpp::Kpp = unsafe { kpp::Kpp::from_ptr(0x401f_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4200_0000usize as _) };
#[doc = "CM7_MCM"]
pub const CM7_MCM: cm7_mcm::Cm7Mcm = unsafe { cm7_mcm::Cm7Mcm::from_ptr(0xe008_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod adc;
pub mod adc_etc;
pub mod aipstz;
pub mod aoi;
pub mod ccm;
pub mod ccm_analog;
pub mod cm7_mcm;
pub mod common;
pub mod csu;
pub mod dcdc;
pub mod dcp;
pub mod dma;
pub mod dmamux;
pub mod ewm;
pub mod flexio;
pub mod flexram;
pub mod flexspi;
pub mod gpc;
pub mod gpio;
pub mod gpt;
pub mod iomuxc;
pub mod iomuxc_gpr;
pub mod iomuxc_snvs;
pub mod iomuxc_snvs_gpr;
pub mod kpp;
pub mod lpi2c;
pub mod lpspi;
pub mod lpuart;
pub mod ocotp;
pub mod otfad;
pub mod pgc;
pub mod pit;
pub mod pmu;
pub mod pwm;
pub mod rtwdog;
pub mod sai;
pub mod snvs;
pub mod spdif;
pub mod src;
pub mod tempmon;
pub mod trng;
pub mod usb;
pub mod usb_analog;
pub mod usbnc;
pub mod usbphy;
pub mod wdog;
pub mod xbara;
pub mod xtalosc24m;
