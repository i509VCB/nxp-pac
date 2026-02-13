#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0b476f2 2026-01-01))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WDT_BOD"]
    WDT_BOD = 0,
    #[doc = "1 - DMA0"]
    DMA0 = 1,
    #[doc = "2 - GINT0"]
    GINT0 = 2,
    #[doc = "3 - GINT1"]
    GINT1 = 3,
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
    #[doc = "12 - SCT0"]
    SCT0 = 12,
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
    #[doc = "20 - FLEXCOMM6"]
    FLEXCOMM6 = 20,
    #[doc = "21 - FLEXCOMM7"]
    FLEXCOMM7 = 21,
    #[doc = "22 - ADC0"]
    ADC0 = 22,
    #[doc = "24 - ACMP"]
    ACMP = 24,
    #[doc = "27 - USB0_NEEDCLK"]
    USB0_NEEDCLK = 27,
    #[doc = "28 - USB0"]
    USB0 = 28,
    #[doc = "29 - RTC"]
    RTC = 29,
    #[doc = "31 - MAILBOX"]
    MAILBOX = 31,
    #[doc = "32 - PIN_INT4"]
    PIN_INT4 = 32,
    #[doc = "33 - PIN_INT5"]
    PIN_INT5 = 33,
    #[doc = "34 - PIN_INT6"]
    PIN_INT6 = 34,
    #[doc = "35 - PIN_INT7"]
    PIN_INT7 = 35,
    #[doc = "36 - CTIMER2"]
    CTIMER2 = 36,
    #[doc = "37 - CTIMER4"]
    CTIMER4 = 37,
    #[doc = "38 - OS_EVENT"]
    OS_EVENT = 38,
    #[doc = "42 - SDIO"]
    SDIO = 42,
    #[doc = "46 - USB1_PHY"]
    USB1_PHY = 46,
    #[doc = "47 - USB1"]
    USB1 = 47,
    #[doc = "48 - USB1_NEEDCLK"]
    USB1_NEEDCLK = 48,
    #[doc = "49 - SEC_HYPERVISOR_CALL"]
    SEC_HYPERVISOR_CALL = 49,
    #[doc = "50 - SEC_GPIO_INT0_IRQ0"]
    SEC_GPIO_INT0_IRQ0 = 50,
    #[doc = "51 - SEC_GPIO_INT0_IRQ1"]
    SEC_GPIO_INT0_IRQ1 = 51,
    #[doc = "52 - PLU"]
    PLU = 52,
    #[doc = "53 - SEC_VIO"]
    SEC_VIO = 53,
    #[doc = "54 - HASHCRYPT"]
    HASHCRYPT = 54,
    #[doc = "55 - CASER"]
    CASER = 55,
    #[doc = "56 - PUF"]
    PUF = 56,
    #[doc = "57 - PQ"]
    PQ = 57,
    #[doc = "58 - DMA1"]
    DMA1 = 58,
    #[doc = "59 - FLEXCOMM8"]
    FLEXCOMM8 = 59,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "FLASH_CFPA"]
pub const FLASH_CFPA_SCRATCH: flash_cfpa::FlashCfpa =
    unsafe { flash_cfpa::FlashCfpa::from_ptr(0x0009_de00usize as _) };
#[doc = "FLASH_CFPA"]
pub const FLASH_CFPA0: flash_cfpa::FlashCfpa =
    unsafe { flash_cfpa::FlashCfpa::from_ptr(0x0009_e000usize as _) };
#[doc = "FLASH_CFPA"]
pub const FLASH_CFPA1: flash_cfpa::FlashCfpa =
    unsafe { flash_cfpa::FlashCfpa::from_ptr(0x0009_e200usize as _) };
#[doc = "FLASH_CMPA"]
pub const FLASH_CMPA: flash_cmpa::FlashCmpa =
    unsafe { flash_cmpa::FlashCmpa::from_ptr(0x0009_e400usize as _) };
#[doc = "FLASH_KEY_STORE"]
pub const FLASH_KEY_STORE: flash_key_store::FlashKeyStore =
    unsafe { flash_key_store::FlashKeyStore::from_ptr(0x0009_e600usize as _) };
#[doc = "SYSCON"]
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x4000_0000usize as _) };
#[doc = "I/O pin configuration (IOCON)"]
pub const IOCON: iocon::Iocon = unsafe { iocon::Iocon::from_ptr(0x4000_1000usize as _) };
#[doc = "Group GPIO input interrupt (GINT0/1)"]
pub const GINT0: gint::Gint = unsafe { gint::Gint::from_ptr(0x4000_2000usize as _) };
#[doc = "Group GPIO input interrupt (GINT0/1)"]
pub const GINT1: gint::Gint = unsafe { gint::Gint::from_ptr(0x4000_3000usize as _) };
#[doc = "Pin interrupt and pattern match (PINT)"]
pub const PINT: pint::Pint = unsafe { pint::Pint::from_ptr(0x4000_4000usize as _) };
#[doc = "Pin interrupt and pattern match (PINT)"]
pub const SECPINT: pint::Pint = unsafe { pint::Pint::from_ptr(0x4000_5000usize as _) };
#[doc = "Input multiplexing (INPUT MUX)"]
pub const INPUTMUX: inputmux::Inputmux =
    unsafe { inputmux::Inputmux::from_ptr(0x4000_6000usize as _) };
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_8000usize as _) };
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_9000usize as _) };
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub const WWDT: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_c000usize as _) };
#[doc = "Multi-Rate Timer (MRT)"]
pub const MRT0: mrt0::Mrt0 = unsafe { mrt0::Mrt0::from_ptr(0x4000_d000usize as _) };
#[doc = "Micro-tick Timer (UTICK)"]
pub const UTICK0: utick0::Utick0 = unsafe { utick0::Utick0::from_ptr(0x4000_e000usize as _) };
#[doc = "ANALOGCTRL"]
pub const ANACTRL: anactrl::Anactrl = unsafe { anactrl::Anactrl::from_ptr(0x4001_3000usize as _) };
#[doc = "PMC"]
pub const PMC: pmc::Pmc = unsafe { pmc::Pmc::from_ptr(0x4002_0000usize as _) };
#[doc = "system controller"]
pub const SYSCTL: sysctl::Sysctl = unsafe { sysctl::Sysctl::from_ptr(0x4002_3000usize as _) };
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_8000usize as _) };
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_9000usize as _) };
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4002_a000usize as _) };
#[doc = "Real-Time Clock (RTC)"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4002_c000usize as _) };
#[doc = "Synchronous OS/Event timer with Wakeup Timer"]
pub const OSTIMER: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x4002_d000usize as _) };
#[doc = "FLASH"]
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4003_4000usize as _) };
#[doc = "PRINCE"]
pub const PRINCE: prince::Prince = unsafe { prince::Prince::from_ptr(0x4003_5000usize as _) };
#[doc = "Universal System Bus Physical Layer"]
pub const USBPHY: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x4003_8000usize as _) };
#[doc = "RNG"]
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4003_a000usize as _) };
#[doc = "PUFCTRL"]
pub const PUF: puf::Puf = unsafe { puf::Puf::from_ptr(0x4003_b000usize as _) };
#[doc = "LPC80X Programmable Logic Unit (PLU)"]
pub const PLU: plu::Plu = unsafe { plu::Plu::from_ptr(0x4003_d000usize as _) };
#[doc = "DMA controller"]
pub const DMA0: dma::Dma = unsafe { dma::Dma::from_ptr(0x4008_2000usize as _) };
#[doc = "USB 2.0 Device Controller"]
pub const USB0: usb0::Usb0 = unsafe { usb0::Usb0::from_ptr(0x4008_4000usize as _) };
#[doc = "SCTimer/PWM (SCT)"]
pub const SCT0: sct0::Sct0 = unsafe { sct0::Sct0::from_ptr(0x4008_5000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM0: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4008_6000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_6000usize as _) };
#[doc = "I2S interface"]
pub const I2S0: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4008_6000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_6000usize as _) };
#[doc = "USARTs"]
pub const USART0: usart::Usart = unsafe { usart::Usart::from_ptr(0x4008_6000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM1: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4008_7000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_7000usize as _) };
#[doc = "I2S interface"]
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4008_7000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_7000usize as _) };
#[doc = "USARTs"]
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4008_7000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM2: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4008_8000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_8000usize as _) };
#[doc = "I2S interface"]
pub const I2S2: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4008_8000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_8000usize as _) };
#[doc = "USARTs"]
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4008_8000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM3: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4008_9000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_9000usize as _) };
#[doc = "I2S interface"]
pub const I2S3: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4008_9000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_9000usize as _) };
#[doc = "USARTs"]
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4008_9000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM4: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4008_a000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4008_a000usize as _) };
#[doc = "I2S interface"]
pub const I2S4: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4008_a000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_a000usize as _) };
#[doc = "USARTs"]
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4008_a000usize as _) };
#[doc = "Mailbox"]
pub const MAILBOX: mailbox::Mailbox = unsafe { mailbox::Mailbox::from_ptr(0x4008_b000usize as _) };
#[doc = "General Purpose I/O (GPIO)"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4008_c000usize as _) };
#[doc = "USB1 High-speed Device Controller"]
pub const USBHSD: usbhsd::Usbhsd = unsafe { usbhsd::Usbhsd::from_ptr(0x4009_4000usize as _) };
#[doc = "CRC engine"]
pub const CRC_ENGINE: crc_engine::CrcEngine =
    unsafe { crc_engine::CrcEngine::from_ptr(0x4009_5000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM5: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4009_6000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C5: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4009_6000usize as _) };
#[doc = "I2S interface"]
pub const I2S5: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4009_6000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI5: spi::Spi = unsafe { spi::Spi::from_ptr(0x4009_6000usize as _) };
#[doc = "USARTs"]
pub const USART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4009_6000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM6: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4009_7000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C6: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4009_7000usize as _) };
#[doc = "I2S interface"]
pub const I2S6: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4009_7000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI6: spi::Spi = unsafe { spi::Spi::from_ptr(0x4009_7000usize as _) };
#[doc = "USARTs"]
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4009_7000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM7: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4009_8000usize as _) };
#[doc = "I2C-bus interfaces"]
pub const I2C7: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4009_8000usize as _) };
#[doc = "I2S interface"]
pub const I2S7: i2s::I2s = unsafe { i2s::I2s::from_ptr(0x4009_8000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI7: spi::Spi = unsafe { spi::Spi::from_ptr(0x4009_8000usize as _) };
#[doc = "USARTs"]
pub const USART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4009_8000usize as _) };
#[doc = "SDMMC"]
pub const SDIF: sdif::Sdif = unsafe { sdif::Sdif::from_ptr(0x4009_b000usize as _) };
#[doc = "MCU Debugger Mailbox"]
pub const DBGMAILBOX: dbgmailbox::Dbgmailbox =
    unsafe { dbgmailbox::Dbgmailbox::from_ptr(0x4009_c000usize as _) };
#[doc = "Flexcomm serial communication"]
pub const FLEXCOMM8: flexcomm::Flexcomm =
    unsafe { flexcomm::Flexcomm::from_ptr(0x4009_f000usize as _) };
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub const SPI8: spi::Spi = unsafe { spi::Spi::from_ptr(0x4009_f000usize as _) };
#[doc = "ADC"]
pub const ADC0: adc0::Adc0 = unsafe { adc0::Adc0::from_ptr(0x400a_0000usize as _) };
#[doc = "USB0 Full-speed Host controller"]
pub const USBFSH: usbfsh::Usbfsh = unsafe { usbfsh::Usbfsh::from_ptr(0x400a_2000usize as _) };
#[doc = "USB1 High-speed Host Controller"]
pub const USBHSH: usbhsh::Usbhsh = unsafe { usbhsh::Usbhsh::from_ptr(0x400a_3000usize as _) };
#[doc = "Hash-Crypt peripheral"]
pub const HASHCRYPT: hashcrypt::Hashcrypt =
    unsafe { hashcrypt::Hashcrypt::from_ptr(0x400a_4000usize as _) };
#[doc = "CASPER"]
pub const CASPER: casper::Casper = unsafe { casper::Casper::from_ptr(0x400a_5000usize as _) };
#[doc = "Digital Signal Co-Processing companion to a Cortex-M v8M CPU core"]
pub const POWERQUAD: powerquad::Powerquad =
    unsafe { powerquad::Powerquad::from_ptr(0x400a_6000usize as _) };
#[doc = "DMA controller"]
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x400a_7000usize as _) };
#[doc = "General Purpose I/O (GPIO)"]
pub const SECGPIO: secgpio::Secgpio = unsafe { secgpio::Secgpio::from_ptr(0x400a_8000usize as _) };
#[doc = "AHB secure controller"]
pub const AHB_SECURE_CTRL: ahb_secure_ctrl::AhbSecureCtrl =
    unsafe { ahb_secure_ctrl::AhbSecureCtrl::from_ptr(0x400a_c000usize as _) };
#[doc = "no description available"]
pub const SCNSCB: scn_scb::ScnScb = unsafe { scn_scb::ScnScb::from_ptr(0xe000_e000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod adc0;
pub mod ahb_secure_ctrl;
pub mod anactrl;
pub mod casper;
pub mod common;
pub mod crc_engine;
pub mod ctimer;
pub mod dbgmailbox;
pub mod dma;
pub mod flash;
pub mod flash_cfpa;
pub mod flash_cmpa;
pub mod flash_key_store;
pub mod flexcomm;
pub mod gint;
pub mod gpio;
pub mod hashcrypt;
pub mod i2c;
pub mod i2s;
pub mod inputmux;
pub mod iocon;
pub mod mailbox;
pub mod mrt0;
pub mod ostimer;
pub mod pint;
pub mod plu;
pub mod pmc;
pub mod powerquad;
pub mod prince;
pub mod puf;
pub mod rng;
pub mod rtc;
pub mod scn_scb;
pub mod sct0;
pub mod sdif;
pub mod secgpio;
pub mod spi;
pub mod syscon;
pub mod sysctl;
pub mod usart;
pub mod usb0;
pub mod usbfsh;
pub mod usbhsd;
pub mod usbhsh;
pub mod usbphy;
pub mod utick0;
pub mod wwdt;
