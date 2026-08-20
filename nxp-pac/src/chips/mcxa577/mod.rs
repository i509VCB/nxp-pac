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
    #[doc = "25 - I3C1"]
    I3C1 = 25,
    #[doc = "26 - LPI2C0"]
    LPI2C0 = 26,
    #[doc = "27 - LPI2C1"]
    LPI2C1 = 27,
    #[doc = "28 - LPSPI0"]
    LPSPI0 = 28,
    #[doc = "29 - LPSPI1"]
    LPSPI1 = 29,
    #[doc = "30 - LPSPI2"]
    LPSPI2 = 30,
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
    #[doc = "61 - WWDT1"]
    WWDT1 = 61,
    #[doc = "62 - ADC0"]
    ADC0 = 62,
    #[doc = "63 - ADC1"]
    ADC1 = 63,
    #[doc = "64 - CMP0"]
    CMP0 = 64,
    #[doc = "67 - DAC0"]
    DAC0 = 67,
    #[doc = "68 - DAC1"]
    DAC1 = 68,
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
    #[doc = "76 - GPIO5"]
    GPIO5 = 76,
    #[doc = "77 - LPI2C2"]
    LPI2C2 = 77,
    #[doc = "78 - LPI2C3"]
    LPI2C3 = 78,
    #[doc = "89 - ESPI"]
    ESPI = 89,
    #[doc = "90 - ETHERNET"]
    ETHERNET = 90,
    #[doc = "91 - ETHERNET_PMT"]
    ETHERNET_PMT = 91,
    #[doc = "93 - TENBASET_PHY0"]
    TENBASET_PHY0 = 93,
    #[doc = "94 - I3C2"]
    I3C2 = 94,
    #[doc = "95 - LPUART5"]
    LPUART5 = 95,
    #[doc = "97 - LPSPI3"]
    LPSPI3 = 97,
    #[doc = "98 - LPSPI4"]
    LPSPI4 = 98,
    #[doc = "99 - LPSPI5"]
    LPSPI5 = 99,
    #[doc = "100 - LPI2C4"]
    LPI2C4 = 100,
    #[doc = "101 - I3C3"]
    I3C3 = 101,
    #[doc = "103 - USB1_HS"]
    USB1_HS = 103,
    #[doc = "104 - USB1_HS_PHY"]
    USB1_HS_PHY = 104,
    #[doc = "106 - FLEXSPI0"]
    FLEXSPI0 = 106,
    #[doc = "108 - SMARTDMA"]
    SMARTDMA = 108,
    #[doc = "109 - CDOG1"]
    CDOG1 = 109,
    #[doc = "110 - PKC"]
    PKC = 110,
    #[doc = "111 - SGI"]
    SGI = 111,
    #[doc = "112 - SPI_FILTER"]
    SPI_FILTER = 112,
    #[doc = "113 - TRNG0"]
    TRNG0 = 113,
    #[doc = "114 - SECURE_ERR"]
    SECURE_ERR = 114,
    #[doc = "115 - SEC_HYPERVISOR_CALL"]
    SEC_HYPERVISOR_CALL = 115,
    #[doc = "119 - RTC0"]
    RTC0 = 119,
    #[doc = "122 - GDET"]
    GDET = 122,
    #[doc = "123 - EWM0"]
    EWM0 = 123,
    #[doc = "124 - TSI_END_OF_SCAN"]
    TSI_END_OF_SCAN = 124,
    #[doc = "125 - TSI_OUT_OF_SCAN"]
    TSI_OUT_OF_SCAN = 125,
    #[doc = "126 - GPIO0_1"]
    GPIO0_1 = 126,
    #[doc = "127 - GPIO1_1"]
    GPIO1_1 = 127,
    #[doc = "128 - GPIO2_1"]
    GPIO2_1 = 128,
    #[doc = "129 - GPIO3_1"]
    GPIO3_1 = 129,
    #[doc = "130 - GPIO4_1"]
    GPIO4_1 = 130,
    #[doc = "131 - GPIO5_1"]
    GPIO5_1 = 131,
    #[doc = "133 - ITRC0"]
    ITRC0 = 133,
    #[doc = "134 - DMA0_CH8"]
    DMA0_CH8 = 134,
    #[doc = "135 - DMA0_CH9"]
    DMA0_CH9 = 135,
    #[doc = "136 - DMA0_CH10"]
    DMA0_CH10 = 136,
    #[doc = "137 - DMA0_CH11"]
    DMA0_CH11 = 137,
    #[doc = "142 - DMA1_CH0"]
    DMA1_CH0 = 142,
    #[doc = "143 - DMA1_CH1"]
    DMA1_CH1 = 143,
    #[doc = "144 - DMA1_CH2"]
    DMA1_CH2 = 144,
    #[doc = "145 - DMA1_CH3"]
    DMA1_CH3 = 145,
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
pub const AHBSC: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x40044000 as _) };
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x400AF000 as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400B0000 as _) };
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400CC000 as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x400D0000 as _) };
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40040000 as _) };
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40041000 as _) };
pub const CMC: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x400C6000 as _) };
pub const CRC0: crc::Crc = unsafe { crc::Crc::from_ptr(0x400C5000 as _) };
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40004000 as _) };
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40005000 as _) };
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40006000 as _) };
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40007000 as _) };
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40008000 as _) };
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x400B4000 as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x400B5000 as _) };
pub const DMA0: dma::Dma12 = unsafe { dma::Dma12::from_ptr(0x40080000 as _) };
pub const DMA1: dma::Dma4 = unsafe { dma::Dma4::from_ptr(0x40013000 as _) };
pub const FLEXSPI0: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x40020000 as _) };
pub const FMU0: fmu::Fmu = unsafe { fmu::Fmu::from_ptr(0x40095000 as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40048000 as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004A000 as _) };
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004C000 as _) };
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004E000 as _) };
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40050000 as _) };
pub const GPIO5: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400DF000 as _) };
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x40002000 as _) };
pub const I3C1: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x40003000 as _) };
pub const I3C2: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4009E000 as _) };
pub const I3C3: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x400DE000 as _) };
pub const INPUTMUX0: inputmux::Inputmux = unsafe { inputmux::Inputmux::from_ptr(0x40001000 as _) };
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009A000 as _) };
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009B000 as _) };
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D4000 as _) };
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D5000 as _) };
pub const LPI2C4: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D6000 as _) };
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009C000 as _) };
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009D000 as _) };
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40021000 as _) };
pub const LPSPI3: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40022000 as _) };
pub const LPSPI4: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40023000 as _) };
pub const LPSPI5: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x40024000 as _) };
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
pub const PORT5: port::Port = unsafe { port::Port::from_ptr(0x400E3000 as _) };
pub const RTC0: rtc5xx::Rtc = unsafe { rtc5xx::Rtc::from_ptr(0x400EE000 as _) };
pub const SCG0: scg::Scg = unsafe { scg::Scg::from_ptr(0x400CA000 as _) };
pub const SGI0: sgi::Sgi = unsafe { sgi::Sgi::from_ptr(0x400EB000 as _) };
pub const SPC0: spc::Spc = unsafe { spc::Spc::from_ptr(0x400CB000 as _) };
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x40091000 as _) };
pub const TRNG0: trng::Trng = unsafe { trng::Trng::from_ptr(0x400EC000 as _) };
pub const VBAT0: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x40093000 as _) };
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000C000 as _) };
pub const WWDT1: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000D000 as _) };
pub const EDMA_0_TCD: edma_tcd::Tcd12 = unsafe { edma_tcd::Tcd12::from_ptr(0x40081000 as _) };
pub const EDMA_1_TCD: edma_tcd::Tcd4 = unsafe { edma_tcd::Tcd4::from_ptr(0x40014000 as _) };
pub const MRCC0: mrcc::Mrcc = unsafe { mrcc::Mrcc::from_ptr(0x40091800 as _) };
#[path = "../../meta_peripherals/mcxa/ADC.rs"]
pub mod adc;
#[path = "../../meta_peripherals/mcxa/AHBSC.rs"]
pub mod ahbsc;
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
#[path = "../../meta_peripherals/mcxa/DAC.rs"]
pub mod dac;
#[path = "../../meta_peripherals/mcxa/DMA.rs"]
pub mod dma;
#[path = "../../meta_peripherals/mcxa/EDMA_TCD.rs"]
pub mod edma_tcd;
#[path = "../../meta_peripherals/mcxa/FLEXSPI.rs"]
pub mod flexspi;
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
#[path = "../../meta_peripherals/mcxa/MRCC5xx.rs"]
pub mod mrcc;
#[path = "../../meta_peripherals/mcxa/OSTIMER.rs"]
pub mod ostimer;
#[path = "../../meta_peripherals/mcx/PORT.rs"]
pub mod port;
#[path = "../../meta_peripherals/mcxa/RTC5xx.rs"]
pub mod rtc5xx;
#[path = "../../meta_peripherals/mcxa/SCG.rs"]
pub mod scg;
#[path = "../../meta_peripherals/mcxa/SGI.rs"]
pub mod sgi;
#[path = "../../meta_peripherals/mcxa/SPC.rs"]
pub mod spc;
#[path = "../../meta_peripherals/mcxa/SYSCON5xx.rs"]
pub mod syscon;
#[path = "../../meta_peripherals/mcxa/TRNG.rs"]
pub mod trng;
#[path = "../../meta_peripherals/mcxa/VBAT.rs"]
pub mod vbat;
#[path = "../../meta_peripherals/mcx/WWDT.rs"]
pub mod wwdt;
