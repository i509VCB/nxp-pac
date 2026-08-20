#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - RESERVED16"]
    RESERVED16 = 0,
    #[doc = "1 - CMC"]
    CMC = 1,
    #[doc = "2 - DMA0_CH0"]
    DMA0_CH0 = 2,
    #[doc = "3 - DMA0_CH1"]
    DMA0_CH1 = 3,
    #[doc = "4 - DMA0_CH2"]
    DMA0_CH2 = 4,
    #[doc = "5 - DMA0_CH3"]
    DMA0_CH3 = 5,
    #[doc = "6 - DMA0_CH4"]
    DMA0_CH4 = 6,
    #[doc = "7 - DMA0_CH5"]
    DMA0_CH5 = 7,
    #[doc = "8 - DMA0_CH6"]
    DMA0_CH6 = 8,
    #[doc = "9 - DMA0_CH7"]
    DMA0_CH7 = 9,
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
    #[doc = "66 - RESERVED82"]
    RESERVED82 = 66,
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
    #[doc = "114 - SECURE_ERR"]
    SECURE_ERR = 114,
    #[doc = "116 - RESERVED132"]
    RESERVED132 = 116,
    #[doc = "117 - RESERVED133"]
    RESERVED133 = 117,
    #[doc = "119 - RTC"]
    RTC = 119,
    #[doc = "120 - RTC_1HZ"]
    RTC_1HZ = 120,
    #[doc = "121 - RESERVED137"]
    RESERVED137 = 121,
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
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x400AF000 as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400B0000 as _) };
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400CC000 as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x400D0000 as _) };
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40100000 as _) };
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40107000 as _) };
pub const CMC: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x4008B000 as _) };
pub const CRC0: crc::Crc = unsafe { crc::Crc::from_ptr(0x4008A000 as _) };
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40004000 as _) };
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40005000 as _) };
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40006000 as _) };
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40007000 as _) };
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40008000 as _) };
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x400B4000 as _) };
pub const DMA0: dma::Dma8 = unsafe { dma::Dma8::from_ptr(0x40080000 as _) };
pub const FLEXIO0: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x40099000 as _) };
pub const FMU0: fmu::Fmu = unsafe { fmu::Fmu::from_ptr(0x40095000 as _) };
pub const FLEX_PWM0: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400A9000 as _) };
pub const FLEX_PWM1: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400AA000 as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40102000 as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40103000 as _) };
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40104000 as _) };
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40105000 as _) };
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40106000 as _) };
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x40002000 as _) };
pub const INPUTMUX0: inputmux::Inputmux = unsafe { inputmux::Inputmux::from_ptr(0x40001000 as _) };
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
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400DA000 as _) };
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x400AD000 as _) };
pub const PORT0: port::Port = unsafe { port::Port::from_ptr(0x400BC000 as _) };
pub const PORT1: port::Port = unsafe { port::Port::from_ptr(0x400BD000 as _) };
pub const PORT2: port::Port = unsafe { port::Port::from_ptr(0x400BE000 as _) };
pub const PORT3: port::Port = unsafe { port::Port::from_ptr(0x400BF000 as _) };
pub const PORT4: port::Port = unsafe { port::Port::from_ptr(0x400C0000 as _) };
pub const RTC0: rtc2xx::Rtc = unsafe { rtc2xx::Rtc::from_ptr(0x400EE000 as _) };
pub const SCG0: scg::Scg = unsafe { scg::Scg::from_ptr(0x4008F000 as _) };
pub const SGI0: sgi::Sgi = unsafe { sgi::Sgi::from_ptr(0x400EB000 as _) };
pub const SPC0: spc::Spc = unsafe { spc::Spc::from_ptr(0x40090000 as _) };
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x40091000 as _) };
pub const TRNG0: trng::Trng = unsafe { trng::Trng::from_ptr(0x400EC000 as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0x400A4000 as _) };
pub const VBAT0: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x40093000 as _) };
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000C000 as _) };
pub const EDMA_0_TCD: edma_tcd::Tcd8 = unsafe { edma_tcd::Tcd8::from_ptr(0x40081000 as _) };
pub const MRCC0: mrcc::Mrcc = unsafe { mrcc::Mrcc::from_ptr(0x40091000 as _) };
#[path = "../../meta_peripherals/mcxa/ADC.rs"]
pub mod adc;
#[path = "../../meta_peripherals/mcxa/CAN.rs"]
pub mod can;
#[path = "../../meta_peripherals/mcxa/CDOG.rs"]
pub mod cdog;
#[path = "../../meta_peripherals/mcxa/CMC.rs"]
pub mod cmc;
pub mod common;
#[path = "../../meta_peripherals/mcxa/CRC.rs"]
pub mod crc;
#[path = "../../meta_peripherals/mcx/CTIMER.rs"]
pub mod ctimer;
#[path = "../../meta_peripherals/mcx/DAC.rs"]
pub mod dac;
#[path = "../../meta_peripherals/mcxa/DMA.rs"]
pub mod dma;
#[path = "../../meta_peripherals/mcxa/EDMA_TCD.rs"]
pub mod edma_tcd;
#[path = "../../meta_peripherals/mcxa/FLEXIO.rs"]
pub mod flexio;
#[path = "../../meta_peripherals/mcxa/FLEXPWM.rs"]
pub mod flexpwm;
#[path = "../../meta_peripherals/mcxa/FMU.rs"]
pub mod fmu;
#[path = "../../meta_peripherals/mcx/GPIO.rs"]
pub mod gpio;
#[path = "../../meta_peripherals/mcxa/I3C.rs"]
pub mod i3c;
#[path = "../../meta_peripherals/mcxa/INPUTMUX.rs"]
pub mod inputmux;
#[path = "../../meta_peripherals/mcxa/LPI2C.rs"]
pub mod lpi2c;
#[path = "../../meta_peripherals/mcxa/LPSPI.rs"]
pub mod lpspi;
#[path = "../../meta_peripherals/mcxa/LPUART.rs"]
pub mod lpuart;
#[path = "../../meta_peripherals/mcxa/MRCC2xx.rs"]
pub mod mrcc;
#[path = "../../meta_peripherals/mcxa/OSTIMER.rs"]
pub mod ostimer;
#[path = "../../meta_peripherals/mcx/PORT.rs"]
pub mod port;
#[path = "../../meta_peripherals/mcxa/RTC2xx.rs"]
pub mod rtc2xx;
#[path = "../../meta_peripherals/mcxa/SCG.rs"]
pub mod scg;
#[path = "../../meta_peripherals/mcxa/SGI.rs"]
pub mod sgi;
#[path = "../../meta_peripherals/mcxa/SPC.rs"]
pub mod spc;
#[path = "../../meta_peripherals/mcxa/SYSCON2xx.rs"]
pub mod syscon;
#[path = "../../meta_peripherals/mcxa/TRNG.rs"]
pub mod trng;
#[path = "../../meta_peripherals/mcxa/USB.rs"]
pub mod usb;
#[path = "../../meta_peripherals/mcxa/VBAT.rs"]
pub mod vbat;
#[path = "../../meta_peripherals/mcx/WWDT.rs"]
pub mod wwdt;
