#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
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
    #[doc = "17 - VBAT0"]
    VBAT0 = 17,
    #[doc = "18 - WUU0"]
    WUU0 = 18,
    #[doc = "19 - CAN0"]
    CAN0 = 19,
    #[doc = "20 - RESERVED36"]
    RESERVED36 = 20,
    #[doc = "21 - RESERVED37"]
    RESERVED37 = 21,
    #[doc = "22 - RESERVED38"]
    RESERVED38 = 22,
    #[doc = "23 - FLEXIO"]
    FLEXIO = 23,
    #[doc = "24 - I3C0"]
    I3C0 = 24,
    #[doc = "25 - RESERVED41"]
    RESERVED41 = 25,
    #[doc = "26 - LPI2C0"]
    LPI2C0 = 26,
    #[doc = "27 - LPI2C1"]
    LPI2C1 = 27,
    #[doc = "28 - LPSPI0"]
    LPSPI0 = 28,
    #[doc = "29 - LPSPI1"]
    LPSPI1 = 29,
    #[doc = "30 - RESERVED46"]
    RESERVED46 = 30,
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
    #[doc = "37 - RESERVED53"]
    RESERVED53 = 37,
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
    #[doc = "49 - RESERVED65"]
    RESERVED65 = 49,
    #[doc = "50 - QDC0_COMPARE"]
    QDC0_COMPARE = 50,
    #[doc = "51 - QDC0_HOME"]
    QDC0_HOME = 51,
    #[doc = "52 - QDC0_WATCHDOG"]
    QDC0_WATCHDOG = 52,
    #[doc = "53 - QDC0_INDEX"]
    QDC0_INDEX = 53,
    #[doc = "54 - FREQME0"]
    FREQME0 = 54,
    #[doc = "55 - LPTMR0"]
    LPTMR0 = 55,
    #[doc = "56 - RESERVED72"]
    RESERVED72 = 56,
    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,
    #[doc = "58 - WAKETIMER0"]
    WAKETIMER0 = 58,
    #[doc = "59 - UTICK0"]
    UTICK0 = 59,
    #[doc = "60 - WWDT0"]
    WWDT0 = 60,
    #[doc = "61 - RESERVED77"]
    RESERVED77 = 61,
    #[doc = "62 - ADC0"]
    ADC0 = 62,
    #[doc = "63 - ADC1"]
    ADC1 = 63,
    #[doc = "64 - CMP0"]
    CMP0 = 64,
    #[doc = "65 - CMP1"]
    CMP1 = 65,
    #[doc = "66 - RESERVED82"]
    RESERVED82 = 66,
    #[doc = "67 - DAC0"]
    DAC0 = 67,
    #[doc = "68 - RESERVED84"]
    RESERVED84 = 68,
    #[doc = "69 - RESERVED85"]
    RESERVED85 = 69,
    #[doc = "70 - RESERVED86"]
    RESERVED86 = 70,
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
    #[doc = "76 - RESERVED92"]
    RESERVED92 = 76,
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
    #[doc = "84 - RESERVED100"]
    RESERVED100 = 84,
    #[doc = "85 - QDC1_COMPARE"]
    QDC1_COMPARE = 85,
    #[doc = "86 - QDC1_HOME"]
    QDC1_HOME = 86,
    #[doc = "87 - QDC1_WATCHDOG"]
    QDC1_WATCHDOG = 87,
    #[doc = "88 - QDC1_INDEX"]
    QDC1_INDEX = 88,
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
#[doc = "I3C"]
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4000_2000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_4000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_5000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_6000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_7000usize as _) };
#[doc = "CTIMER"]
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_8000usize as _) };
#[doc = "FREQME"]
pub const FREQME0: freqme::Freqme = unsafe { freqme::Freqme::from_ptr(0x4000_9000usize as _) };
#[doc = "UTICK"]
pub const UTICK0: utick::Utick = unsafe { utick::Utick::from_ptr(0x4000_b000usize as _) };
#[doc = "WWDT"]
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_c000usize as _) };
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
#[doc = "EIM"]
pub const EIM0: eim::Eim = unsafe { eim::Eim::from_ptr(0x4008_c000usize as _) };
#[doc = "ERM"]
pub const ERM0: erm::Erm = unsafe { erm::Erm::from_ptr(0x4008_d000usize as _) };
#[doc = "TRDC"]
pub const MBC0: mbc::Mbc = unsafe { mbc::Mbc::from_ptr(0x4008_e000usize as _) };
#[doc = "SCG"]
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
#[doc = "FlashTest"]
pub const FMU0TEST: fmu0test::Fmu0test =
    unsafe { fmu0test::Fmu0test::from_ptr(0x4009_6000usize as _) };
#[doc = "AOI"]
pub const AOI1: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x4009_7000usize as _) };
#[doc = "FLEXIO"]
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
pub const QDC0: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400a_7000usize as _) };
#[doc = "Quadrature_Decoder"]
pub const QDC1: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400a_8000usize as _) };
#[doc = "PWM"]
pub const FLEXPWM0: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400a_9000usize as _) };
#[doc = "PWM"]
pub const FLEXPWM1: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400a_a000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR0: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x400a_b000usize as _) };
#[doc = "OSTIMER"]
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x400a_d000usize as _) };
#[doc = "WAKEUP_TIMER"]
pub const WAKETIMER0: waketimer::Waketimer =
    unsafe { waketimer::Waketimer::from_ptr(0x400a_e000usize as _) };
#[doc = "ADC"]
pub const ADC0: hsadc::Hsadc = unsafe { hsadc::Hsadc::from_ptr(0x400a_f000usize as _) };
#[doc = "ADC"]
pub const ADC1: hsadc::Hsadc = unsafe { hsadc::Hsadc::from_ptr(0x400b_0000usize as _) };
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
pub const PORT1: port1::Port1 = unsafe { port1::Port1::from_ptr(0x400b_d000usize as _) };
#[doc = "PORT"]
pub const PORT2: port2::Port2 = unsafe { port2::Port2::from_ptr(0x400b_e000usize as _) };
#[doc = "PORT"]
pub const PORT3: port3::Port3 = unsafe { port3::Port3::from_ptr(0x400b_f000usize as _) };
#[doc = "PORT"]
pub const PORT4: port4::Port4 = unsafe { port4::Port4::from_ptr(0x400c_0000usize as _) };
#[doc = "CAN"]
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400c_c000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_4000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_5000usize as _) };
#[doc = "CDOG"]
pub const CDOG: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x4010_0000usize as _) };
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
#[doc = "System Control not in System Control Block"]
pub const SCNSCB: s_cn_scb::SCnScb = unsafe { s_cn_scb::SCnScb::from_ptr(0xe000_e000usize as _) };
#[doc = "M33 Systick module"]
pub const SYSTICK: sys_tick::SysTick =
    unsafe { sys_tick::SysTick::from_ptr(0xe000_e010usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
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
pub mod erm;
pub mod flexio;
pub mod flexpwm;
pub mod fmc;
pub mod fmu;
pub mod fmu0test;
pub mod freqme;
pub mod glikey;
pub mod gpio;
pub mod hsadc;
pub mod i3c;
pub mod inputmux;
pub mod lpi2c;
pub mod lpspi;
pub mod lptmr;
pub mod lpuart;
pub mod mbc;
pub mod mrcc;
pub mod opamp;
pub mod ostimer;
pub mod port;
pub mod port1;
pub mod port2;
pub mod port3;
pub mod port4;
pub mod qdc;
pub mod s_cn_scb;
pub mod scg;
pub mod spc;
pub mod sys_tick;
pub mod syscon;
pub mod usb;
pub mod utick;
pub mod vbat;
pub mod waketimer;
pub mod wuu;
pub mod wwdt;
