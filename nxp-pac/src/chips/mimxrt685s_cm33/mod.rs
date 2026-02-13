#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0b476f2 2026-01-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WDT0"]
    WDT0 = 0,
    #[doc = "1 - DMA0"]
    DMA0 = 1,
    #[doc = "2 - GPIO_INTA"]
    GPIO_INTA = 2,
    #[doc = "3 - GPIO_INTB"]
    GPIO_INTB = 3,
    #[doc = "4 - PIN_INT0"]
    PIN_INT0 = 4,
    #[doc = "5 - PIN_INT1"]
    PIN_INT1 = 5,
    #[doc = "6 - PIN_INT2"]
    PIN_INT2 = 6,
    #[doc = "7 - PIN_INT3"]
    PIN_INT3 = 7,
    #[doc = "8 - UTICK0"]
    UTICK0 = 8,
    #[doc = "9 - MRT0"]
    MRT0 = 9,
    #[doc = "10 - CTIMER0"]
    CTIMER0 = 10,
    #[doc = "11 - CTIMER1"]
    CTIMER1 = 11,
    #[doc = "12 - SCT"]
    SCT = 12,
    #[doc = "13 - CTIMER3"]
    CTIMER3 = 13,
    #[doc = "14 - FLEXCOMM0"]
    FLEXCOMM0 = 14,
    #[doc = "15 - FLEXCOMM1"]
    FLEXCOMM1 = 15,
    #[doc = "16 - FLEXCOMM2"]
    FLEXCOMM2 = 16,
    #[doc = "17 - FLEXCOMM3"]
    FLEXCOMM3 = 17,
    #[doc = "18 - FLEXCOMM4"]
    FLEXCOMM4 = 18,
    #[doc = "19 - FLEXCOMM5"]
    FLEXCOMM5 = 19,
    #[doc = "20 - FLEXCOMM14"]
    FLEXCOMM14 = 20,
    #[doc = "21 - FLEXCOMM15"]
    FLEXCOMM15 = 21,
    #[doc = "22 - ADC"]
    ADC = 22,
    #[doc = "23 - RESERVED39"]
    RESERVED39 = 23,
    #[doc = "24 - ACMP"]
    ACMP = 24,
    #[doc = "25 - DMIC"]
    DMIC = 25,
    #[doc = "26 - RESERVED42"]
    RESERVED42 = 26,
    #[doc = "27 - HYPERVISOR"]
    HYPERVISOR = 27,
    #[doc = "28 - SECUREVIOLATION"]
    SECUREVIOLATION = 28,
    #[doc = "29 - HWVAD0"]
    HWVAD0 = 29,
    #[doc = "30 - ESPI"]
    ESPI = 30,
    #[doc = "31 - RNG"]
    RNG = 31,
    #[doc = "32 - RTC"]
    RTC = 32,
    #[doc = "33 - DSPWAKE"]
    DSPWAKE = 33,
    #[doc = "34 - MU_A"]
    MU_A = 34,
    #[doc = "35 - PIN_INT4"]
    PIN_INT4 = 35,
    #[doc = "36 - PIN_INT5"]
    PIN_INT5 = 36,
    #[doc = "37 - PIN_INT6"]
    PIN_INT6 = 37,
    #[doc = "38 - PIN_INT7"]
    PIN_INT7 = 38,
    #[doc = "39 - CTIMER2"]
    CTIMER2 = 39,
    #[doc = "40 - CTIMER4"]
    CTIMER4 = 40,
    #[doc = "41 - OS_EVENT"]
    OS_EVENT = 41,
    #[doc = "42 - FLEXSPI"]
    FLEXSPI = 42,
    #[doc = "43 - FLEXCOMM6"]
    FLEXCOMM6 = 43,
    #[doc = "44 - FLEXCOMM7"]
    FLEXCOMM7 = 44,
    #[doc = "45 - USDHC0"]
    USDHC0 = 45,
    #[doc = "46 - USDHC1"]
    USDHC1 = 46,
    #[doc = "47 - SGPIO_INTA"]
    SGPIO_INTA = 47,
    #[doc = "48 - SGPIO_INTB"]
    SGPIO_INTB = 48,
    #[doc = "49 - I3C"]
    I3C = 49,
    #[doc = "50 - USB"]
    USB = 50,
    #[doc = "51 - USB_WAKEUP"]
    USB_WAKEUP = 51,
    #[doc = "52 - WDT1"]
    WDT1 = 52,
    #[doc = "53 - USBPHY_DCD"]
    USBPHY_DCD = 53,
    #[doc = "54 - DMA1"]
    DMA1 = 54,
    #[doc = "55 - PUF"]
    PUF = 55,
    #[doc = "56 - POWERQUAD"]
    POWERQUAD = 56,
    #[doc = "57 - CASPER"]
    CASPER = 57,
    #[doc = "58 - PMC_PMIC"]
    PMC_PMIC = 58,
    #[doc = "59 - HASHCRYPT"]
    HASHCRYPT = 59,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "reset ccontroller 0"]
pub const RSTCTL0: rstctl0::Rstctl0 = unsafe { rstctl0::Rstctl0::from_ptr(0x4000_0000usize as _) };
#[doc = "clock ccontroller 0"]
pub const CLKCTL0: clkctl0::Clkctl0 = unsafe { clkctl0::Clkctl0::from_ptr(0x4000_1000usize as _) };
#[doc = "system controller 0"]
pub const SYSCTL0: sysctl0::Sysctl0 = unsafe { sysctl0::Sysctl0::from_ptr(0x4000_2000usize as _) };
#[doc = "LPC-Next0 IO pad controller"]
pub const IOPCTL: iopctl::Iopctl = unsafe { iopctl::Iopctl::from_ptr(0x4000_4000usize as _) };
#[doc = "PUF Controller"]
pub const PUF: puf::Puf = unsafe { puf::Puf::from_ptr(0x4000_6000usize as _) };
#[doc = "LPC_Next0 Windowed Watchdog Timer (WWDT)"]
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_e000usize as _) };
#[doc = "LPC-Next0 Micro-tick Timer (UTICK)"]
pub const UTICK0: utick::Utick = unsafe { utick::Utick::from_ptr(0x4000_f000usize as _) };
#[doc = "reset ccontroller 1"]
pub const RSTCTL1: rstctl1::Rstctl1 = unsafe { rstctl1::Rstctl1::from_ptr(0x4002_0000usize as _) };
#[doc = "clock ccontroller 1"]
pub const CLKCTL1: clkctl1::Clkctl1 = unsafe { clkctl1::Clkctl1::from_ptr(0x4002_1000usize as _) };
#[doc = "system ccontroller 1"]
pub const SYSCTL1: sysctl1::Sysctl1 = unsafe { sysctl1::Sysctl1::from_ptr(0x4002_2000usize as _) };
#[doc = "LPC-Next0 Pin interrupt and pattern match (PINT)"]
pub const PINT: pint::Pint = unsafe { pint::Pint::from_ptr(0x4002_5000usize as _) };
#[doc = "LPC_Next0 Peripheral Input Multiplexers Controller"]
pub const INPUTMUX: inputmux::Inputmux =
    unsafe { inputmux::Inputmux::from_ptr(0x4002_6000usize as _) };
#[doc = "LPC-Next0 Standard async counter/timer"]
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_8000usize as _) };
#[doc = "LPC-Next0 Standard async counter/timer"]
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_9000usize as _) };
#[doc = "LPC-Next0 Standard async counter/timer"]
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_a000usize as _) };
#[doc = "LPC-Next0 Standard async counter/timer"]
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_b000usize as _) };
#[doc = "LPC-Next0 Standard async counter/timer"]
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_c000usize as _) };
#[doc = "LPC-Next0 Multi-Rate Timer (MRT)"]
pub const MRT0: mrt::Mrt = unsafe { mrt::Mrt::from_ptr(0x4002_d000usize as _) };
#[doc = "LPC_Next0 Windowed Watchdog Timer (WWDT)"]
pub const WWDT1: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4002_e000usize as _) };
#[doc = "LPC_Next0 Frequency Measurement (FREQME)"]
pub const FREQME: freqme::Freqme = unsafe { freqme::Freqme::from_ptr(0x4002_f000usize as _) };
#[doc = "LPC-Next0 Real-Time Clock (RTC)"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4003_0000usize as _) };
#[doc = "CACHE64"]
pub const CACHE64: cache64::Cache64 = unsafe { cache64::Cache64::from_ptr(0x4003_3000usize as _) };
#[doc = "CACHE64_POLSEL"]
pub const CACHE64_POLSEL: cache64_polsel::Cache64Polsel =
    unsafe { cache64_polsel::Cache64Polsel::from_ptr(0x4003_3000usize as _) };
#[doc = "I3C"]
pub const I3C: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4003_6000usize as _) };
#[doc = "a variant of SPI used by Intel to communicate with its processors via the PCH (aka Southbridge)."]
pub const ESPI: espi::Espi = unsafe { espi::Espi::from_ptr(0x4003_7000usize as _) };
#[doc = "LPC-Next0 General Purpose I/O (GPIO)"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4010_0000usize as _) };
#[doc = "LPC-Next0 DMA controller"]
pub const DMA0: dma::Dma = unsafe { dma::Dma::from_ptr(0x4010_4000usize as _) };
#[doc = "LPC-Next0 DMA controller"]
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4010_5000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM0: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4010_6000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4010_6000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S0: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4010_6000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4010_6000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART0: usart::Usart = unsafe { usart::Usart::from_ptr(0x4010_6000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM1: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4010_7000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4010_7000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4010_7000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4010_7000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4010_7000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM2: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4010_8000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4010_8000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S2: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4010_8000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4010_8000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4010_8000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM3: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4010_9000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4010_9000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S3: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4010_9000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4010_9000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4010_9000usize as _) };
#[doc = "LPC-Next0 MUA"]
pub const MUA: mua::Mua = unsafe { mua::Mua::from_ptr(0x4011_0000usize as _) };
#[doc = "LPC-Next0 SEMA"]
pub const SEMA42: sema42::Sema42 = unsafe { sema42::Sema42::from_ptr(0x4011_2000usize as _) };
#[doc = "LPC_Next0 Synchronous OS/Event timer with Wakeup Timer"]
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x4011_3000usize as _) };
#[doc = "LPC_Next0 CRC engine"]
pub const CRC_ENGINE: crc::Crc = unsafe { crc::Crc::from_ptr(0x4012_0000usize as _) };
#[doc = "LPC_Next0 DMIC Subsystem (DMIC))"]
pub const DMIC0: dmic::Dmic = unsafe { dmic::Dmic::from_ptr(0x4012_1000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM4: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4012_2000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4012_2000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S4: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4012_2000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4012_2000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4012_2000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM5: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4012_3000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C5: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4012_3000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S5: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4012_3000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4012_3000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4012_3000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM6: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4012_4000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C6: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4012_4000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S6: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4012_4000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x4012_4000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4012_4000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM7: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4012_5000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C7: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4012_5000usize as _) };
#[doc = "LPC-Next0 I2S interface"]
pub const I2S7: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4012_5000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI7: spi::Spi = unsafe { spi::Spi::from_ptr(0x4012_5000usize as _) };
#[doc = "LPC_Next0 USARTs"]
pub const USART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4012_5000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM14: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4012_6000usize as _) };
#[doc = "LPC_Next0 Serial Peripheral Interfaces (SPI)"]
pub const SPI14: spi::Spi = unsafe { spi::Spi::from_ptr(0x4012_6000usize as _) };
#[doc = "LPC-Next0 Flexcomm serial communication"]
pub const FLEXCOMM15: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4012_7000usize as _) };
#[doc = "LPC-Next0 I2C-bus interfaces"]
pub const I2C15: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4012_7000usize as _) };
#[doc = "LPC-Next0 OTP controller"]
pub const OCOTP: ocotp::Ocotp = unsafe { ocotp::Ocotp::from_ptr(0x4013_0000usize as _) };
#[doc = "FlexSPI"]
pub const FLEXSPI: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x4013_4000usize as _) };
#[doc = "OTFAD"]
pub const OTFAD: otfad::Otfad = unsafe { otfad::Otfad::from_ptr(0x4013_4000usize as _) };
#[doc = "LPC-Next0 Power Management Controller"]
pub const PMC: pmc::Pmc = unsafe { pmc::Pmc::from_ptr(0x4013_5000usize as _) };
#[doc = "uSDHC"]
pub const USDHC0: usdhc::Usdhc = unsafe { usdhc::Usdhc::from_ptr(0x4013_6000usize as _) };
#[doc = "uSDHC"]
pub const USDHC1: usdhc::Usdhc = unsafe { usdhc::Usdhc::from_ptr(0x4013_7000usize as _) };
#[doc = "LPC-Next0 RNG"]
pub const TRNG: trng::Trng = unsafe { trng::Trng::from_ptr(0x4013_8000usize as _) };
#[doc = "CMP"]
pub const CMP: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x4013_9000usize as _) };
#[doc = "ADC"]
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x4013_a000usize as _) };
#[doc = "Universal System Bus Physical Layer"]
pub const USBPHY: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x4013_b000usize as _) };
#[doc = "USBDCD"]
pub const USBHSDCD: usbhsdcd::Usbhsdcd =
    unsafe { usbhsdcd::Usbhsdcd::from_ptr(0x4013_b800usize as _) };
#[doc = "LPC54S60x/LPC5460x USB1 High-speed Device Controller"]
pub const USBHSD: usbhsd::Usbhsd = unsafe { usbhsd::Usbhsd::from_ptr(0x4014_4000usize as _) };
#[doc = "LPC-Next0 USB1 High-speed Host Controller"]
pub const USBHSH: usbhsh::Usbhsh = unsafe { usbhsh::Usbhsh::from_ptr(0x4014_5000usize as _) };
#[doc = "LPC84x SCTimer/PWM (SCT)"]
pub const SCT0: sct::Sct = unsafe { sct::Sct::from_ptr(0x4014_6000usize as _) };
#[doc = "LPC_Next0 AHB secure controller"]
pub const AHB_SECURE_CTRL: ahb_secure_ctrl::AhbSecureCtrl =
    unsafe { ahb_secure_ctrl::AhbSecureCtrl::from_ptr(0x4014_8000usize as _) };
#[doc = "LPC-Next0 Digital Signal Co-Processing companion to a Cortex-M v8M CPU core"]
pub const POWERQUAD: powerquad::Powerquad =
    unsafe { powerquad::Powerquad::from_ptr(0x4015_0000usize as _) };
#[doc = "LPC-Next0 CASPER"]
pub const CASPER: casper::Casper = unsafe { casper::Casper::from_ptr(0x4015_1000usize as _) };
#[doc = "LPC-Next0 General Purpose I/O (GPIO)"]
pub const SECGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4015_4000usize as _) };
#[doc = "LPC-Next0 Hash-Crypt peripheral"]
pub const HASHCRYPT: hashcrypt::Hashcrypt =
    unsafe { hashcrypt::Hashcrypt::from_ptr(0x4015_8000usize as _) };
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
pub mod ahb_secure_ctrl;
pub mod cache64;
pub mod cache64_polsel;
pub mod casper;
pub mod clkctl0;
pub mod clkctl1;
pub mod cmp;
pub mod common;
pub mod crc;
pub mod ctimer;
pub mod dma;
pub mod dmic;
pub mod espi;
pub mod flexcomm;
pub mod flexspi;
pub mod freqme;
pub mod gpio;
pub mod hashcrypt;
pub mod i2c;
pub mod i2s;
pub mod i3c;
pub mod inputmux;
pub mod iopctl;
pub mod mrt;
pub mod mua;
pub mod ocotp;
pub mod ostimer;
pub mod otfad;
pub mod pint;
pub mod pmc;
pub mod powerquad;
pub mod puf;
pub mod rstctl0;
pub mod rstctl1;
pub mod rtc;
pub mod scn_scb;
pub mod sct;
pub mod sema42;
pub mod spi;
pub mod sysctl0;
pub mod sysctl1;
pub mod trng;
pub mod usart;
pub mod usbhsd;
pub mod usbhsdcd;
pub mod usbhsh;
pub mod usbphy;
pub mod usdhc;
pub mod utick;
pub mod wwdt;
