#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - RESERVED16"]
    RESERVED16 = 0,
    #[doc = "1 - CMC"]
    CMC = 1,
    #[doc = "2 - DMA_CH0"]
    DMA_CH0 = 2,
    #[doc = "3 - DMA_CH1"]
    DMA_CH1 = 3,
    #[doc = "4 - DMA_CH2"]
    DMA_CH2 = 4,
    #[doc = "5 - DMA_CH3"]
    DMA_CH3 = 5,
    #[doc = "6 - DMA_CH4"]
    DMA_CH4 = 6,
    #[doc = "7 - DMA_CH5"]
    DMA_CH5 = 7,
    #[doc = "8 - DMA_CH6"]
    DMA_CH6 = 8,
    #[doc = "9 - DMA_CH7"]
    DMA_CH7 = 9,
    #[doc = "10 - ERM0_SINGLE_BIT"]
    ERM0_SINGLE_BIT = 10,
    #[doc = "11 - ERM0_MULTI_BIT"]
    ERM0_MULTI_BIT = 11,
    #[doc = "12 - FMU0"]
    FMU0 = 12,
    #[doc = "13 - GLIKEY0"]
    GLIKEY0 = 13,
    #[doc = "14 - MBC0"]
    MBC0 = 14,
    #[doc = "15 - SCG0"]
    SCG0 = 15,
    #[doc = "16 - SPC0"]
    SPC0 = 16,
    #[doc = "17 - TDET"]
    TDET = 17,
    #[doc = "18 - WUU0"]
    WUU0 = 18,
    #[doc = "19 - CAN0"]
    CAN0 = 19,
    #[doc = "20 - CAN1"]
    CAN1 = 20,
    #[doc = "23 - FLEXIO"]
    FLEXIO = 23,
    #[doc = "24 - I3C0"]
    I3C0 = 24,
    #[doc = "26 - LPI2C0"]
    LPI2C0 = 26,
    #[doc = "27 - LPI2C1"]
    LPI2C1 = 27,
    #[doc = "28 - LPSPI0"]
    LPSPI0 = 28,
    #[doc = "29 - LPSPI1"]
    LPSPI1 = 29,
    #[doc = "31 - LPUART0"]
    LPUART0 = 31,
    #[doc = "32 - LPUART1"]
    LPUART1 = 32,
    #[doc = "33 - LPUART2"]
    LPUART2 = 33,
    #[doc = "34 - LPUART3"]
    LPUART3 = 34,
    #[doc = "35 - LPUART4"]
    LPUART4 = 35,
    #[doc = "36 - USB0"]
    USB0 = 36,
    #[doc = "38 - CDOG0"]
    CDOG0 = 38,
    #[doc = "39 - CTIMER0"]
    CTIMER0 = 39,
    #[doc = "40 - CTIMER1"]
    CTIMER1 = 40,
    #[doc = "41 - CTIMER2"]
    CTIMER2 = 41,
    #[doc = "42 - CTIMER3"]
    CTIMER3 = 42,
    #[doc = "43 - CTIMER4"]
    CTIMER4 = 43,
    #[doc = "44 - FLEXPWM0_RELOAD_ERROR"]
    FLEXPWM0_RELOAD_ERROR = 44,
    #[doc = "45 - FLEXPWM0_FAULT"]
    FLEXPWM0_FAULT = 45,
    #[doc = "46 - FLEXPWM0_SUBMODULE0"]
    FLEXPWM0_SUBMODULE0 = 46,
    #[doc = "47 - FLEXPWM0_SUBMODULE1"]
    FLEXPWM0_SUBMODULE1 = 47,
    #[doc = "48 - FLEXPWM0_SUBMODULE2"]
    FLEXPWM0_SUBMODULE2 = 48,
    #[doc = "49 - FLEXPWM0_SUBMODULE3"]
    FLEXPWM0_SUBMODULE3 = 49,
    #[doc = "50 - EQDC0_COMPARE"]
    EQDC0_COMPARE = 50,
    #[doc = "51 - EQDC0_HOME"]
    EQDC0_HOME = 51,
    #[doc = "52 - EQDC0_WATCHDOG"]
    EQDC0_WATCHDOG = 52,
    #[doc = "53 - EQDC0_INDEX"]
    EQDC0_INDEX = 53,
    #[doc = "54 - FREQME0"]
    FREQME0 = 54,
    #[doc = "55 - LPTMR0"]
    LPTMR0 = 55,
    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,
    #[doc = "58 - WAKETIMER0"]
    WAKETIMER0 = 58,
    #[doc = "59 - UTICK0"]
    UTICK0 = 59,
    #[doc = "60 - WWDT0"]
    WWDT0 = 60,
    #[doc = "62 - ADC0"]
    ADC0 = 62,
    #[doc = "63 - ADC1"]
    ADC1 = 63,
    #[doc = "64 - CMP0"]
    CMP0 = 64,
    #[doc = "65 - CMP1"]
    CMP1 = 65,
    #[doc = "66 - CMP2"]
    CMP2 = 66,
    #[doc = "67 - DAC0"]
    DAC0 = 67,
    #[doc = "71 - GPIO0"]
    GPIO0 = 71,
    #[doc = "72 - GPIO1"]
    GPIO1 = 72,
    #[doc = "73 - GPIO2"]
    GPIO2 = 73,
    #[doc = "74 - GPIO3"]
    GPIO3 = 74,
    #[doc = "75 - GPIO4"]
    GPIO4 = 75,
    #[doc = "77 - LPI2C2"]
    LPI2C2 = 77,
    #[doc = "78 - LPI2C3"]
    LPI2C3 = 78,
    #[doc = "79 - FLEXPWM1_RELOAD_ERROR"]
    FLEXPWM1_RELOAD_ERROR = 79,
    #[doc = "80 - FLEXPWM1_FAULT"]
    FLEXPWM1_FAULT = 80,
    #[doc = "81 - FLEXPWM1_SUBMODULE0"]
    FLEXPWM1_SUBMODULE0 = 81,
    #[doc = "82 - FLEXPWM1_SUBMODULE1"]
    FLEXPWM1_SUBMODULE1 = 82,
    #[doc = "83 - FLEXPWM1_SUBMODULE2"]
    FLEXPWM1_SUBMODULE2 = 83,
    #[doc = "84 - FLEXPWM1_SUBMODULE3"]
    FLEXPWM1_SUBMODULE3 = 84,
    #[doc = "85 - EQDC1_COMPARE"]
    EQDC1_COMPARE = 85,
    #[doc = "86 - EQDC1_HOME"]
    EQDC1_HOME = 86,
    #[doc = "87 - EQDC1_WATCHDOG"]
    EQDC1_WATCHDOG = 87,
    #[doc = "88 - EQDC1_INDEX"]
    EQDC1_INDEX = 88,
    #[doc = "95 - LPUART5"]
    LPUART5 = 95,
    #[doc = "107 - MAU"]
    MAU = 107,
    #[doc = "108 - SMARTDMA"]
    SMARTDMA = 108,
    #[doc = "109 - CDOG1"]
    CDOG1 = 109,
    #[doc = "110 - PKC"]
    PKC = 110,
    #[doc = "111 - SGI"]
    SGI = 111,
    #[doc = "113 - TRNG0"]
    TRNG0 = 113,
    #[doc = "116 - ADC2"]
    ADC2 = 116,
    #[doc = "117 - ADC3"]
    ADC3 = 117,
    #[doc = "119 - RTC"]
    RTC = 119,
    #[doc = "120 - RTC_1HZ"]
    RTC_1HZ = 120,
    #[doc = "121 - SLCD"]
    SLCD = 121,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "INPUTMUX"]
pub const INPUTMUX0: inputmux::Inputmux =
    unsafe { inputmux::Inputmux::from_ptr(0x4000_1000usize as _) };
#[doc = "Improved Inter-Integrated Circuit"]
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4000_2000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_4000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_5000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_6000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_7000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_8000usize as _) };
#[doc = "FREQME"]
pub const FREQME0: freqme::Freqme = unsafe { freqme::Freqme::from_ptr(0x4000_9000usize as _) };
#[doc = "UTICK"]
pub const UTICK0: utick::Utick = unsafe { utick::Utick::from_ptr(0x4000_b000usize as _) };
#[doc = "WWDT"]
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_c000usize as _) };
#[doc = "Smart DMA Controller"]
pub const SMARTDMA0: smartdma::Smartdma =
    unsafe { smartdma::Smartdma::from_ptr(0x4000_e000usize as _) };
#[doc = "DMA MP"]
pub const DMA0: dma::Dma = unsafe { dma::Dma::from_ptr(0x4008_0000usize as _) };
#[doc = "DMA TCD"]
pub const EDMA_0_TCD0: edma_0_tcd::Edma0Tcd =
    unsafe { edma_0_tcd::Edma0Tcd::from_ptr(0x4008_1000usize as _) };
#[doc = "AOI"]
pub const AOI0: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4008_9000usize as _) };
#[doc = "CRC"]
pub const CRC0: crc::Crc = unsafe { crc::Crc::from_ptr(0x4008_a000usize as _) };
#[doc = "CMC"]
pub const CMC: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x4008_b000usize as _) };
#[doc = "Error Injection Module"]
pub const EIM0: eim::Eim = unsafe { eim::Eim::from_ptr(0x4008_c000usize as _) };
#[doc = "Error Reporting Module"]
pub const ERM0: erm::Erm = unsafe { erm::Erm::from_ptr(0x4008_d000usize as _) };
#[doc = "TRDC"]
pub const MBC0: mbc::Mbc = unsafe { mbc::Mbc::from_ptr(0x4008_e000usize as _) };
#[doc = "System Clock Generator"]
pub const SCG0: scg::Scg = unsafe { scg::Scg::from_ptr(0x4008_f000usize as _) };
#[doc = "SPC"]
pub const SPC0: spc::Spc = unsafe { spc::Spc::from_ptr(0x4009_0000usize as _) };
#[doc = "MRCC"]
pub const MRCC0: mrcc::Mrcc = unsafe { mrcc::Mrcc::from_ptr(0x4009_1000usize as _) };
#[doc = "SYSCON"]
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x4009_1000usize as _) };
#[doc = "GLIKEY"]
pub const GLIKEY0: glikey::Glikey = unsafe { glikey::Glikey::from_ptr(0x4009_1d00usize as _) };
#[doc = "Low-Leakage Wakeup Unit"]
pub const WUU0: wuu::Wuu = unsafe { wuu::Wuu::from_ptr(0x4009_2000usize as _) };
#[doc = "VBAT"]
pub const VBAT0: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x4009_3000usize as _) };
#[doc = "NPX"]
pub const FMC0: fmc::Fmc = unsafe { fmc::Fmc::from_ptr(0x4009_4000usize as _) };
#[doc = "Flash"]
pub const FMU0: fmu::Fmu = unsafe { fmu::Fmu::from_ptr(0x4009_5000usize as _) };
#[doc = "AOI"]
pub const AOI1: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4009_7000usize as _) };
#[doc = "Flexible I/O"]
pub const FLEXIO0: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x4009_9000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_a000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_b000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_c000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_d000usize as _) };
#[doc = "LPUART"]
pub const LPUART0: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009_f000usize as _) };
#[doc = "LPUART"]
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_1000usize as _) };
#[doc = "LPUART"]
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_2000usize as _) };
#[doc = "LPUART"]
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_3000usize as _) };
#[doc = "USBFS"]
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0x400a_4000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const EQDC0: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x400a_7000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const EQDC1: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x400a_8000usize as _) };
#[doc = "PWM"]
pub const FLEXPWM0: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400a_9000usize as _) };
#[doc = "PWM"]
pub const FLEXPWM1: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400a_a000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR0: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x400a_b000usize as _) };
#[doc = "OSTIMER"]
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x400a_d000usize as _) };
#[doc = "WAKE_TIMER"]
pub const WAKETIMER0: waketimer::Waketimer =
    unsafe { waketimer::Waketimer::from_ptr(0x400a_e000usize as _) };
#[doc = "ADC"]
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x400a_f000usize as _) };
#[doc = "ADC"]
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400b_0000usize as _) };
#[doc = "LPCMP"]
pub const CMP0: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400b_1000usize as _) };
#[doc = "LPCMP"]
pub const CMP1: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400b_2000usize as _) };
#[doc = "12-bit DAC"]
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x400b_4000usize as _) };
#[doc = "OPAMP"]
pub const OPAMP0: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x400b_7000usize as _) };
#[doc = "PORT"]
pub const PORT0: port::Port = unsafe { port::Port::from_ptr(0x400b_c000usize as _) };
#[doc = "PORT"]
pub const PORT1: port::Port = unsafe { port::Port::from_ptr(0x400b_d000usize as _) };
#[doc = "PORT"]
pub const PORT2: port::Port = unsafe { port::Port::from_ptr(0x400b_e000usize as _) };
#[doc = "PORT"]
pub const PORT3: port::Port = unsafe { port::Port::from_ptr(0x400b_f000usize as _) };
#[doc = "PORT"]
pub const PORT4: port::Port = unsafe { port::Port::from_ptr(0x400c_0000usize as _) };
#[doc = "CAN"]
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400c_c000usize as _) };
#[doc = "CAN"]
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x400d_0000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_4000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_5000usize as _) };
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400d_a000usize as _) };
#[doc = "TDET"]
pub const TDET0: tdet::Tdet = unsafe { tdet::Tdet::from_ptr(0x400e_9000usize as _) };
#[doc = "no description available"]
pub const PKC0: pkc::Pkc = unsafe { pkc::Pkc::from_ptr(0x400e_a000usize as _) };
#[doc = "no description available"]
pub const SGI0: sgi::Sgi = unsafe { sgi::Sgi::from_ptr(0x400e_b000usize as _) };
#[doc = "pd_main.trng0"]
pub const TRNG0: trng::Trng = unsafe { trng::Trng::from_ptr(0x400e_c000usize as _) };
#[doc = "no description available"]
pub const UDF0: udf::Udf = unsafe { udf::Udf::from_ptr(0x400e_d000usize as _) };
#[doc = "RTC"]
pub const RTC0: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x400e_e000usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x400f_0000usize as _) };
pub const ADC3: adc::Adc = unsafe { adc::Adc::from_ptr(0x400f_1000usize as _) };
#[doc = "CDOG"]
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x4010_0000usize as _) };
#[doc = "DBGMB"]
pub const DBGMAILBOX: dbgmailbox::Dbgmailbox =
    unsafe { dbgmailbox::Dbgmailbox::from_ptr(0x4010_1000usize as _) };
#[doc = "GPIO"]
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4010_2000usize as _) };
#[doc = "GPIO"]
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4010_3000usize as _) };
#[doc = "GPIO"]
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4010_4000usize as _) };
#[doc = "GPIO"]
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4010_5000usize as _) };
#[doc = "GPIO"]
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4010_6000usize as _) };
#[doc = "CDOG"]
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x4010_7000usize as _) };
#[doc = "MAUWRAP"]
pub const MAU0: mau::Mau = unsafe { mau::Mau::from_ptr(0x4010_8000usize as _) };
#[doc = "System Control not in System Control Block"]
pub const SCNSCB: scn_scb::ScnScb = unsafe { scn_scb::ScnScb::from_ptr(0xe000_e000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod adc;
pub mod aoi;
pub mod can;
pub mod cdog;
pub mod cmc;
pub mod cmp;
pub mod common;
pub mod crc;
pub mod ctimer;
pub mod dac;
pub mod dbgmailbox;
pub mod dma;
pub mod edma_0_tcd;
pub mod eim;
pub mod eqdc;
pub mod erm;
pub mod flexio;
pub mod flexpwm;
pub mod fmc;
pub mod fmu;
pub mod freqme;
pub mod glikey;
pub mod gpio;
pub mod i3c;
pub mod inputmux;
pub mod lpi2c;
pub mod lpspi;
pub mod lptmr;
pub mod lpuart;
pub mod mau;
pub mod mbc;
pub mod mrcc;
pub mod opamp;
pub mod ostimer;
pub mod pkc;
pub mod port;
pub mod rtc;
pub mod scg;
pub mod scn_scb;
pub mod sgi;
pub mod smartdma;
pub mod spc;
pub mod syscon;
pub mod tdet;
pub mod trng;
pub mod udf;
pub mod usb;
pub mod utick;
pub mod vbat;
pub mod waketimer;
pub mod wuu;
pub mod wwdt;
