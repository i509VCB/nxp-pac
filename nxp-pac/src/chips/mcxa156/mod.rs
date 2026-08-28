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
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub const INPUTMUX0: inputmux::Inputmux = unsafe { inputmux::Inputmux::from_ptr(0x40001000 as _) };
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x40002000 as _) };
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40004000 as _) };
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40005000 as _) };
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40006000 as _) };
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40007000 as _) };
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40008000 as _) };
pub const FREQME0: freqme::Freqme = unsafe { freqme::Freqme::from_ptr(0x40009000 as _) };
pub const UTICK0: utick::Utick = unsafe { utick::Utick::from_ptr(0x4000B000 as _) };
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000C000 as _) };
pub const DMA0: dma::Dma8 = unsafe { dma::Dma8::from_ptr(0x40080000 as _) };
pub const EDMA_0_TCD: edma_0_tcd::Tcd8 = unsafe { edma_0_tcd::Tcd8::from_ptr(0x40081000 as _) };
pub const AOI0: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x40089000 as _) };
pub const AOI1: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x40097000 as _) };
pub const CRC0: crc::Crc = unsafe { crc::Crc::from_ptr(0x4008A000 as _) };
pub const CMC: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x4008B000 as _) };
pub const EIM0: eim::Eim = unsafe { eim::Eim::from_ptr(0x4008C000 as _) };
pub const ERM0: erm::Erm = unsafe { erm::Erm::from_ptr(0x4008D000 as _) };
pub const MBC0: mbc::Mbc = unsafe { mbc::Mbc::from_ptr(0x4008E000 as _) };
pub const SCG0: scg::Scg = unsafe { scg::Scg::from_ptr(0x4008F000 as _) };
pub const SPC0: spc::Spc = unsafe { spc::Spc::from_ptr(0x40090000 as _) };
pub const MRCC0: mrcc::Mrcc = unsafe { mrcc::Mrcc::from_ptr(0x40091000 as _) };
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x40091000 as _) };
pub const GLIKEY0: glikey::Glikey = unsafe { glikey::Glikey::from_ptr(0x40091D00 as _) };
pub const WUU0: wuu::Wuu = unsafe { wuu::Wuu::from_ptr(0x40092000 as _) };
pub const VBAT0: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x40093000 as _) };
pub const FMC0: fmc::Fmc = unsafe { fmc::Fmc::from_ptr(0x40094000 as _) };
pub const FMU0: fmu::Fmu = unsafe { fmu::Fmu::from_ptr(0x40095000 as _) };
pub const FMU0TEST: fmu0test::Fmu0test = unsafe { fmu0test::Fmu0test::from_ptr(0x40096000 as _) };
pub const FLEXIO0: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x40099000 as _) };
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009A000 as _) };
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009B000 as _) };
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D4000 as _) };
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D5000 as _) };
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009C000 as _) };
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009D000 as _) };
pub const LPUART0: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009F000 as _) };
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A0000 as _) };
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A1000 as _) };
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A2000 as _) };
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A3000 as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0x400A4000 as _) };
pub const EQDC0: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400A7000 as _) };
pub const EQDC1: qdc::Qdc = unsafe { qdc::Qdc::from_ptr(0x400A8000 as _) };
pub const FLEX_PWM0: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400A9000 as _) };
pub const FLEX_PWM1: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400AA000 as _) };
pub const LPTMR0: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x400AB000 as _) };
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x400AD000 as _) };
pub const WAKETIMER0: waketimer::Waketimer =
    unsafe { waketimer::Waketimer::from_ptr(0x400AE000 as _) };
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x400AF000 as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400B0000 as _) };
pub const CMP0: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400B1000 as _) };
pub const CMP1: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400B2000 as _) };
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x400B4000 as _) };
pub const OPAMP0: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0x400B7000 as _) };
pub const PORT0: port::Port = unsafe { port::Port::from_ptr(0x400BC000 as _) };
pub const PORT1: port::Port1 = unsafe { port::Port1::from_ptr(0x400BD000 as _) };
pub const PORT2: port::Port2 = unsafe { port::Port2::from_ptr(0x400BE000 as _) };
pub const PORT3: port::Port3 = unsafe { port::Port3::from_ptr(0x400BF000 as _) };
pub const PORT4: port::Port4 = unsafe { port::Port4::from_ptr(0x400C0000 as _) };
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400CC000 as _) };
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40100000 as _) };
pub const DBGMAILBOX: dbgmailbox::Dbgmailbox =
    unsafe { dbgmailbox::Dbgmailbox::from_ptr(0x40101000 as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40102000 as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40103000 as _) };
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40104000 as _) };
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40105000 as _) };
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40106000 as _) };
#[path = "../../meta_peripherals/MCXA156/ADC.rs"]
pub mod adc;
#[path = "../../meta_peripherals/MCXA156/AOI.rs"]
pub mod aoi;
#[path = "../../meta_peripherals/MCXA156/CAN.rs"]
pub mod can;
#[path = "../../meta_peripherals/MCXA156/CDOG.rs"]
pub mod cdog;
#[path = "../../meta_peripherals/MCXA156/CMC.rs"]
pub mod cmc;
#[path = "../../meta_peripherals/MCXA156/CMP.rs"]
pub mod cmp;
pub mod common;
#[path = "../../meta_peripherals/MCXA156/CRC.rs"]
pub mod crc;
#[path = "../../meta_peripherals/MCXA156/CTIMER.rs"]
pub mod ctimer;
#[path = "../../meta_peripherals/MCXA156/DAC.rs"]
pub mod dac;
#[path = "../../meta_peripherals/MCXA156/DBGMAILBOX.rs"]
pub mod dbgmailbox;
#[path = "../../meta_peripherals/MCXA156/DMA.rs"]
pub mod dma;
#[path = "../../meta_peripherals/MCXA156/EDMA_0_TCD.rs"]
pub mod edma_0_tcd;
#[path = "../../meta_peripherals/MCXA156/EIM.rs"]
pub mod eim;
#[path = "../../meta_peripherals/MCXA156/ERM.rs"]
pub mod erm;
#[path = "../../meta_peripherals/MCXA156/FLEXIO.rs"]
pub mod flexio;
#[path = "../../meta_peripherals/MCXA156/FLEXPWM.rs"]
pub mod flexpwm;
#[path = "../../meta_peripherals/MCXA156/FMC.rs"]
pub mod fmc;
#[path = "../../meta_peripherals/MCXA156/FMU.rs"]
pub mod fmu;
#[path = "../../meta_peripherals/MCXA156/FMU0TEST.rs"]
pub mod fmu0test;
#[path = "../../meta_peripherals/MCXA156/FREQME.rs"]
pub mod freqme;
#[path = "../../meta_peripherals/MCXA156/GLIKEY.rs"]
pub mod glikey;
#[path = "../../meta_peripherals/MCXA156/GPIO.rs"]
pub mod gpio;
#[path = "../../meta_peripherals/MCXA156/I3C.rs"]
pub mod i3c;
#[path = "../../meta_peripherals/MCXA156/INPUTMUX.rs"]
pub mod inputmux;
#[path = "../../meta_peripherals/MCXA156/LPI2C.rs"]
pub mod lpi2c;
#[path = "../../meta_peripherals/MCXA156/LPSPI.rs"]
pub mod lpspi;
#[path = "../../meta_peripherals/MCXA156/LPTMR.rs"]
pub mod lptmr;
#[path = "../../meta_peripherals/MCXA156/LPUART.rs"]
pub mod lpuart;
#[path = "../../meta_peripherals/MCXA156/MBC.rs"]
pub mod mbc;
#[path = "../../meta_peripherals/MCXA156/MRCC.rs"]
pub mod mrcc;
#[path = "../../meta_peripherals/MCXA156/OPAMP.rs"]
pub mod opamp;
#[path = "../../meta_peripherals/MCXA156/OSTIMER.rs"]
pub mod ostimer;
#[path = "../../meta_peripherals/MCXA156/PORT.rs"]
pub mod port;
#[path = "../../meta_peripherals/MCXA156/QDC.rs"]
pub mod qdc;
#[path = "../../meta_peripherals/MCXA156/SCG.rs"]
pub mod scg;
#[path = "../../meta_peripherals/MCXA156/SPC.rs"]
pub mod spc;
#[path = "../../meta_peripherals/MCXA156/SYSCON.rs"]
pub mod syscon;
#[path = "../../meta_peripherals/MCXA156/USB.rs"]
pub mod usb;
#[path = "../../meta_peripherals/MCXA156/UTICK.rs"]
pub mod utick;
#[path = "../../meta_peripherals/MCXA156/VBAT.rs"]
pub mod vbat;
#[path = "../../meta_peripherals/MCXA156/WAKETIMER.rs"]
pub mod waketimer;
#[path = "../../meta_peripherals/MCXA156/WUU.rs"]
pub mod wuu;
#[path = "../../meta_peripherals/MCXA156/WWDT.rs"]
pub mod wwdt;
