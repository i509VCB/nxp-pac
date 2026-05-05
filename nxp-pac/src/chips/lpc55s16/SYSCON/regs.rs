#[doc = "ADC clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCCLKDIV(pub u32);
impl ADCCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::ADCCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ADCCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::ADCCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::ADCCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ADCCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::ADCCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::ADCCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ADCCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::ADCCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ADCCLKDIV {
    #[inline(always)]
    fn default() -> ADCCLKDIV {
        ADCCLKDIV(0)
    }
}
impl core::fmt::Debug for ADCCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "ADC clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCCLKSEL(pub u32);
impl ADCCLKSEL {
    #[doc = "ADC clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::ADCCLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ADCCLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "ADC clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::ADCCLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for ADCCLKSEL {
    #[inline(always)]
    fn default() -> ADCCLKSEL {
        ADCCLKSEL(0)
    }
}
impl core::fmt::Debug for ADCCLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCCLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADCCLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "AHB Clock control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRL0(pub u32);
impl AHBCLKCTRL0 {
    #[doc = "Enables the clock for the ROM."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the ROM."]
    #[inline(always)]
    pub const fn set_ROM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the SRAM Controller 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_CTRL1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub const fn set_SRAM_CTRL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for the SRAM Controller 2."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_CTRL2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub const fn set_SRAM_CTRL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for the Flash controller."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Flash controller."]
    #[inline(always)]
    pub const fn set_FLASH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the FMC controller."]
    #[must_use]
    #[inline(always)]
    pub const fn FMC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FMC controller."]
    #[inline(always)]
    pub const fn set_FMC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for the Input Mux."]
    #[must_use]
    #[inline(always)]
    pub const fn MUX(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Input Mux."]
    #[inline(always)]
    pub const fn set_MUX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for the I/O controller."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCON(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the I/O controller."]
    #[inline(always)]
    pub const fn set_IOCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for the GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO0."]
    #[inline(always)]
    pub const fn set_GPIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for the GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO1."]
    #[inline(always)]
    pub const fn set_GPIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for the Pin interrupt (PINT)."]
    #[must_use]
    #[inline(always)]
    pub const fn PINT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub const fn set_PINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for the Group interrupt (GINT)."]
    #[must_use]
    #[inline(always)]
    pub const fn GINT(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub const fn set_GINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for the DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the DMA0."]
    #[inline(always)]
    pub const fn set_DMA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for the CRCGEN."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCGEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub const fn set_CRCGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for the Watchdog Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn WWDT(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub const fn set_WWDT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for the Real Time Clock (RTC)."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub const fn set_RTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn MAILBOX(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub const fn set_MAILBOX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for the ADC."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the ADC."]
    #[inline(always)]
    pub const fn set_ADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for AHBCLKCTRL0 {
    #[inline(always)]
    fn default() -> AHBCLKCTRL0 {
        AHBCLKCTRL0(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRL0")
            .field("ROM", &self.ROM())
            .field("SRAM_CTRL1", &self.SRAM_CTRL1())
            .field("SRAM_CTRL2", &self.SRAM_CTRL2())
            .field("FLASH", &self.FLASH())
            .field("FMC", &self.FMC())
            .field("MUX", &self.MUX())
            .field("IOCON", &self.IOCON())
            .field("GPIO0", &self.GPIO0())
            .field("GPIO1", &self.GPIO1())
            .field("PINT", &self.PINT())
            .field("GINT", &self.GINT())
            .field("DMA0", &self.DMA0())
            .field("CRCGEN", &self.CRCGEN())
            .field("WWDT", &self.WWDT())
            .field("RTC", &self.RTC())
            .field("MAILBOX", &self.MAILBOX())
            .field("ADC", &self.ADC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AHBCLKCTRL0 {{ ROM: {=bool:?}, SRAM_CTRL1: {=bool:?}, SRAM_CTRL2: {=bool:?}, FLASH: {=bool:?}, FMC: {=bool:?}, MUX: {=bool:?}, IOCON: {=bool:?}, GPIO0: {=bool:?}, GPIO1: {=bool:?}, PINT: {=bool:?}, GINT: {=bool:?}, DMA0: {=bool:?}, CRCGEN: {=bool:?}, WWDT: {=bool:?}, RTC: {=bool:?}, MAILBOX: {=bool:?}, ADC: {=bool:?} }}",
            self.ROM(),
            self.SRAM_CTRL1(),
            self.SRAM_CTRL2(),
            self.FLASH(),
            self.FMC(),
            self.MUX(),
            self.IOCON(),
            self.GPIO0(),
            self.GPIO1(),
            self.PINT(),
            self.GINT(),
            self.DMA0(),
            self.CRCGEN(),
            self.WWDT(),
            self.RTC(),
            self.MAILBOX(),
            self.ADC()
        )
    }
}
#[doc = "AHB Clock control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRL1(pub u32);
impl AHBCLKCTRL1 {
    #[doc = "Enables the clock for the MRT."]
    #[must_use]
    #[inline(always)]
    pub const fn MRT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the MRT."]
    #[inline(always)]
    pub const fn set_MRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the clock for the OS Event Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn OSTIMER(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub const fn set_OSTIMER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn SCT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SCT."]
    #[inline(always)]
    pub const fn set_SCT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for the CAN."]
    #[must_use]
    #[inline(always)]
    pub const fn CAN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the CAN."]
    #[inline(always)]
    pub const fn set_CAN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the UTICK."]
    #[must_use]
    #[inline(always)]
    pub const fn UTICK(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the UTICK."]
    #[inline(always)]
    pub const fn set_UTICK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables the clock for the FC0."]
    #[must_use]
    #[inline(always)]
    pub const fn FC0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC0."]
    #[inline(always)]
    pub const fn set_FC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for the FC1."]
    #[must_use]
    #[inline(always)]
    pub const fn FC1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC1."]
    #[inline(always)]
    pub const fn set_FC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock for the FC2."]
    #[must_use]
    #[inline(always)]
    pub const fn FC2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC2."]
    #[inline(always)]
    pub const fn set_FC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for the FC3."]
    #[must_use]
    #[inline(always)]
    pub const fn FC3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC3."]
    #[inline(always)]
    pub const fn set_FC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for the FC4."]
    #[must_use]
    #[inline(always)]
    pub const fn FC4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC4."]
    #[inline(always)]
    pub const fn set_FC4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for the FC5."]
    #[must_use]
    #[inline(always)]
    pub const fn FC5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC5."]
    #[inline(always)]
    pub const fn set_FC5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for the FC6."]
    #[must_use]
    #[inline(always)]
    pub const fn FC6(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC6."]
    #[inline(always)]
    pub const fn set_FC6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for the FC7."]
    #[must_use]
    #[inline(always)]
    pub const fn FC7(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC7."]
    #[inline(always)]
    pub const fn set_FC7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for the Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 2."]
    #[inline(always)]
    pub const fn set_TIMER2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for the USB0-FS device."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_DEV(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB0-FS device."]
    #[inline(always)]
    pub const fn set_USB0_DEV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for the Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 0."]
    #[inline(always)]
    pub const fn set_TIMER0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for the Timer 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 1."]
    #[inline(always)]
    pub const fn set_TIMER1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for AHBCLKCTRL1 {
    #[inline(always)]
    fn default() -> AHBCLKCTRL1 {
        AHBCLKCTRL1(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRL1")
            .field("MRT", &self.MRT())
            .field("OSTIMER", &self.OSTIMER())
            .field("SCT", &self.SCT())
            .field("CAN", &self.CAN())
            .field("UTICK", &self.UTICK())
            .field("FC0", &self.FC0())
            .field("FC1", &self.FC1())
            .field("FC2", &self.FC2())
            .field("FC3", &self.FC3())
            .field("FC4", &self.FC4())
            .field("FC5", &self.FC5())
            .field("FC6", &self.FC6())
            .field("FC7", &self.FC7())
            .field("TIMER2", &self.TIMER2())
            .field("USB0_DEV", &self.USB0_DEV())
            .field("TIMER0", &self.TIMER0())
            .field("TIMER1", &self.TIMER1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AHBCLKCTRL1 {{ MRT: {=bool:?}, OSTIMER: {=bool:?}, SCT: {=bool:?}, CAN: {=bool:?}, UTICK: {=bool:?}, FC0: {=bool:?}, FC1: {=bool:?}, FC2: {=bool:?}, FC3: {=bool:?}, FC4: {=bool:?}, FC5: {=bool:?}, FC6: {=bool:?}, FC7: {=bool:?}, TIMER2: {=bool:?}, USB0_DEV: {=bool:?}, TIMER0: {=bool:?}, TIMER1: {=bool:?} }}",
            self.MRT(),
            self.OSTIMER(),
            self.SCT(),
            self.CAN(),
            self.UTICK(),
            self.FC0(),
            self.FC1(),
            self.FC2(),
            self.FC3(),
            self.FC4(),
            self.FC5(),
            self.FC6(),
            self.FC7(),
            self.TIMER2(),
            self.USB0_DEV(),
            self.TIMER0(),
            self.TIMER1()
        )
    }
}
#[doc = "AHB Clock control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRL2(pub u32);
impl AHBCLKCTRL2 {
    #[doc = "Enables the clock for the DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the DMA1."]
    #[inline(always)]
    pub const fn set_DMA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Comparator."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for the USB1-HS Host."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_HOST(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1-HS Host."]
    #[inline(always)]
    pub const fn set_USB1_HOST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for the USB1-HS device."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_DEV(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1-HS device."]
    #[inline(always)]
    pub const fn set_USB1_DEV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for the USB1-HS RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_RAM(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1-HS RAM."]
    #[inline(always)]
    pub const fn set_USB1_RAM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for the USB1-HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_PHY(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1-HS PHY."]
    #[inline(always)]
    pub const fn set_USB1_PHY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the Frequency meter."]
    #[must_use]
    #[inline(always)]
    pub const fn FREQME(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub const fn set_FREQME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for the code watchdog."]
    #[must_use]
    #[inline(always)]
    pub const fn CDOG(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the code watchdog."]
    #[inline(always)]
    pub const fn set_CDOG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for the RNG."]
    #[must_use]
    #[inline(always)]
    pub const fn RNG(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RNG."]
    #[inline(always)]
    pub const fn set_RNG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SYSCTL block clock."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCTL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SYSCTL block clock."]
    #[inline(always)]
    pub const fn set_SYSCTL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for the USB0-FS Host Master."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_HOSTM(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB0-FS Host Master."]
    #[inline(always)]
    pub const fn set_USB0_HOSTM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for the USB0-FS Host Slave."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_HOSTS(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB0-FS Host Slave."]
    #[inline(always)]
    pub const fn set_USB0_HOSTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for the HASH_AES."]
    #[must_use]
    #[inline(always)]
    pub const fn HASH_AES(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub const fn set_HASH_AES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for the PLU LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn PLULUT(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub const fn set_PLULUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for the Timer 3."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 3."]
    #[inline(always)]
    pub const fn set_TIMER3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for the Timer 4."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER4(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 4."]
    #[inline(always)]
    pub const fn set_TIMER4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for the PUF reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn PUF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub const fn set_PUF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for the Casper."]
    #[must_use]
    #[inline(always)]
    pub const fn CASPER(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Casper."]
    #[inline(always)]
    pub const fn set_CASPER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for the analog control."]
    #[must_use]
    #[inline(always)]
    pub const fn ANALOG_CTRL(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the analog control."]
    #[inline(always)]
    pub const fn set_ANALOG_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enables the clock for the HS LSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn HS_LSPI(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub const fn set_HS_LSPI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enables the clock for the GPIO secure."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_SEC(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub const fn set_GPIO_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for the GPIO secure int."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_SEC_INT(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub const fn set_GPIO_SEC_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for AHBCLKCTRL2 {
    #[inline(always)]
    fn default() -> AHBCLKCTRL2 {
        AHBCLKCTRL2(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRL2")
            .field("DMA1", &self.DMA1())
            .field("COMP", &self.COMP())
            .field("USB1_HOST", &self.USB1_HOST())
            .field("USB1_DEV", &self.USB1_DEV())
            .field("USB1_RAM", &self.USB1_RAM())
            .field("USB1_PHY", &self.USB1_PHY())
            .field("FREQME", &self.FREQME())
            .field("CDOG", &self.CDOG())
            .field("RNG", &self.RNG())
            .field("SYSCTL", &self.SYSCTL())
            .field("USB0_HOSTM", &self.USB0_HOSTM())
            .field("USB0_HOSTS", &self.USB0_HOSTS())
            .field("HASH_AES", &self.HASH_AES())
            .field("PLULUT", &self.PLULUT())
            .field("TIMER3", &self.TIMER3())
            .field("TIMER4", &self.TIMER4())
            .field("PUF", &self.PUF())
            .field("CASPER", &self.CASPER())
            .field("ANALOG_CTRL", &self.ANALOG_CTRL())
            .field("HS_LSPI", &self.HS_LSPI())
            .field("GPIO_SEC", &self.GPIO_SEC())
            .field("GPIO_SEC_INT", &self.GPIO_SEC_INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AHBCLKCTRL2 {{ DMA1: {=bool:?}, COMP: {=bool:?}, USB1_HOST: {=bool:?}, USB1_DEV: {=bool:?}, USB1_RAM: {=bool:?}, USB1_PHY: {=bool:?}, FREQME: {=bool:?}, CDOG: {=bool:?}, RNG: {=bool:?}, SYSCTL: {=bool:?}, USB0_HOSTM: {=bool:?}, USB0_HOSTS: {=bool:?}, HASH_AES: {=bool:?}, PLULUT: {=bool:?}, TIMER3: {=bool:?}, TIMER4: {=bool:?}, PUF: {=bool:?}, CASPER: {=bool:?}, ANALOG_CTRL: {=bool:?}, HS_LSPI: {=bool:?}, GPIO_SEC: {=bool:?}, GPIO_SEC_INT: {=bool:?} }}",
            self.DMA1(),
            self.COMP(),
            self.USB1_HOST(),
            self.USB1_DEV(),
            self.USB1_RAM(),
            self.USB1_PHY(),
            self.FREQME(),
            self.CDOG(),
            self.RNG(),
            self.SYSCTL(),
            self.USB0_HOSTM(),
            self.USB0_HOSTS(),
            self.HASH_AES(),
            self.PLULUT(),
            self.TIMER3(),
            self.TIMER4(),
            self.PUF(),
            self.CASPER(),
            self.ANALOG_CTRL(),
            self.HS_LSPI(),
            self.GPIO_SEC(),
            self.GPIO_SEC_INT()
        )
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRLCLR(pub u32);
impl AHBCLKCTRLCLR {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AHBCLKCTRLCLR {
    #[inline(always)]
    fn default() -> AHBCLKCTRLCLR {
        AHBCLKCTRLCLR(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRLCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRLCLR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRLCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AHBCLKCTRLCLR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRLSET(pub u32);
impl AHBCLKCTRLSET {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AHBCLKCTRLSET {
    #[inline(always)]
    fn default() -> AHBCLKCTRLSET {
        AHBCLKCTRLSET(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRLSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRLSET")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRLSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AHBCLKCTRLSET {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRLX0(pub u32);
impl AHBCLKCTRLX0 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AHBCLKCTRLX0 {
    #[inline(always)]
    fn default() -> AHBCLKCTRLX0 {
        AHBCLKCTRLX0(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRLX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRLX0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRLX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AHBCLKCTRLX0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRLX1(pub u32);
impl AHBCLKCTRLX1 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AHBCLKCTRLX1 {
    #[inline(always)]
    fn default() -> AHBCLKCTRLX1 {
        AHBCLKCTRLX1(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRLX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRLX1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRLX1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AHBCLKCTRLX1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKCTRLX2(pub u32);
impl AHBCLKCTRLX2 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AHBCLKCTRLX2 {
    #[inline(always)]
    fn default() -> AHBCLKCTRLX2 {
        AHBCLKCTRLX2(0)
    }
}
impl core::fmt::Debug for AHBCLKCTRLX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKCTRLX2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKCTRLX2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AHBCLKCTRLX2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "System clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBCLKDIV(pub u32);
impl AHBCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::AHBCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::AHBCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::AHBCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::AHBCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::AHBCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::AHBCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::AHBCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AHBCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::AHBCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for AHBCLKDIV {
    #[inline(always)]
    fn default() -> AHBCLKDIV {
        AHBCLKDIV(0)
    }
}
impl core::fmt::Debug for AHBCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AHBCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBMATPRIO(pub u32);
impl AHBMATPRIO {
    #[doc = "CPU0 C-AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_CPU0_CBUS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CPU0 C-AHB bus."]
    #[inline(always)]
    pub const fn set_PRI_CPU0_CBUS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_CPU0_SBUS(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "CPU0 S-AHB bus."]
    #[inline(always)]
    pub const fn set_PRI_CPU0_SBUS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "DMA0 controller priority."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_SDMA0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "DMA0 controller priority."]
    #[inline(always)]
    pub const fn set_PRI_SDMA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA1 controller priority."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_SDMA1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "DMA1 controller priority."]
    #[inline(always)]
    pub const fn set_PRI_SDMA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "USB0-FS Device.(USB0)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_USB_FSD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "USB0-FS Device.(USB0)."]
    #[inline(always)]
    pub const fn set_PRI_USB_FSD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "USB0-FS host.(USB0)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_USB_FSH(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "USB0-FS host.(USB0)."]
    #[inline(always)]
    pub const fn set_PRI_USB_FSH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "HASH_AES."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_HASH_AES(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "HASH_AES."]
    #[inline(always)]
    pub const fn set_PRI_HASH_AES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "CANFD."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_CANFD(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "CANFD."]
    #[inline(always)]
    pub const fn set_PRI_CANFD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for AHBMATPRIO {
    #[inline(always)]
    fn default() -> AHBMATPRIO {
        AHBMATPRIO(0)
    }
}
impl core::fmt::Debug for AHBMATPRIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBMATPRIO")
            .field("PRI_CPU0_CBUS", &self.PRI_CPU0_CBUS())
            .field("PRI_CPU0_SBUS", &self.PRI_CPU0_SBUS())
            .field("PRI_SDMA0", &self.PRI_SDMA0())
            .field("PRI_SDMA1", &self.PRI_SDMA1())
            .field("PRI_USB_FSD", &self.PRI_USB_FSD())
            .field("PRI_USB_FSH", &self.PRI_USB_FSH())
            .field("PRI_HASH_AES", &self.PRI_HASH_AES())
            .field("PRI_CANFD", &self.PRI_CANFD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AHBMATPRIO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AHBMATPRIO {{ PRI_CPU0_CBUS: {=u8:?}, PRI_CPU0_SBUS: {=u8:?}, PRI_SDMA0: {=u8:?}, PRI_SDMA1: {=u8:?}, PRI_USB_FSD: {=u8:?}, PRI_USB_FSH: {=u8:?}, PRI_HASH_AES: {=u8:?}, PRI_CANFD: {=u8:?} }}",
            self.PRI_CPU0_CBUS(),
            self.PRI_CPU0_SBUS(),
            self.PRI_SDMA0(),
            self.PRI_SDMA1(),
            self.PRI_USB_FSD(),
            self.PRI_USB_FSH(),
            self.PRI_HASH_AES(),
            self.PRI_CANFD()
        )
    }
}
#[doc = "Control automatic clock gating."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUTOCLKGATEOVERRIDE(pub u32);
impl AUTOCLKGATEOVERRIDE {
    #[doc = "Control automatic clock gating of ROM controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub const fn set_ROM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control automatic clock gating of RAMX controller."]
    #[must_use]
    #[inline(always)]
    pub const fn RAMX_CTRL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub const fn set_RAMX_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control automatic clock gating of RAM0 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM0_CTRL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub const fn set_RAM0_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control automatic clock gating of RAM1 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM1_CTRL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub const fn set_RAM1_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control automatic clock gating of RAM2 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM2_CTRL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub const fn set_RAM2_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNC0_APB(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub const fn set_SYNC0_APB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNC1_APB(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub const fn set_SYNC1_APB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Control automatic clock gating of CRCGEN controller."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCGEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub const fn set_CRCGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Control automatic clock gating of DMA0 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub const fn set_SDMA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Control automatic clock gating of DMA1 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn SDMA1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub const fn set_SDMA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Control automatic clock gating of USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub const fn set_USB0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Control automatic clock gating of synchronous system controller registers bank."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCON(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub const fn set_SYSCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLEUPDATE(&self) -> super::vals::ENABLEUPDATE {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::ENABLEUPDATE::from_bits(val as u16)
    }
    #[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[inline(always)]
    pub const fn set_ENABLEUPDATE(&mut self, val: super::vals::ENABLEUPDATE) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUTOCLKGATEOVERRIDE {
    #[inline(always)]
    fn default() -> AUTOCLKGATEOVERRIDE {
        AUTOCLKGATEOVERRIDE(0)
    }
}
impl core::fmt::Debug for AUTOCLKGATEOVERRIDE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCLKGATEOVERRIDE")
            .field("ROM", &self.ROM())
            .field("RAMX_CTRL", &self.RAMX_CTRL())
            .field("RAM0_CTRL", &self.RAM0_CTRL())
            .field("RAM1_CTRL", &self.RAM1_CTRL())
            .field("RAM2_CTRL", &self.RAM2_CTRL())
            .field("SYNC0_APB", &self.SYNC0_APB())
            .field("SYNC1_APB", &self.SYNC1_APB())
            .field("CRCGEN", &self.CRCGEN())
            .field("SDMA0", &self.SDMA0())
            .field("SDMA1", &self.SDMA1())
            .field("USB0", &self.USB0())
            .field("SYSCON", &self.SYSCON())
            .field("ENABLEUPDATE", &self.ENABLEUPDATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUTOCLKGATEOVERRIDE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUTOCLKGATEOVERRIDE {{ ROM: {=bool:?}, RAMX_CTRL: {=bool:?}, RAM0_CTRL: {=bool:?}, RAM1_CTRL: {=bool:?}, RAM2_CTRL: {=bool:?}, SYNC0_APB: {=bool:?}, SYNC1_APB: {=bool:?}, CRCGEN: {=bool:?}, SDMA0: {=bool:?}, SDMA1: {=bool:?}, USB0: {=bool:?}, SYSCON: {=bool:?}, ENABLEUPDATE: {:?} }}",
            self.ROM(),
            self.RAMX_CTRL(),
            self.RAM0_CTRL(),
            self.RAM1_CTRL(),
            self.RAM2_CTRL(),
            self.SYNC0_APB(),
            self.SYNC1_APB(),
            self.CRCGEN(),
            self.SDMA0(),
            self.SDMA1(),
            self.USB0(),
            self.SYSCON(),
            self.ENABLEUPDATE()
        )
    }
}
#[doc = "Control write access to boot seed security registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_LOCK(pub u32);
impl BOOT_LOCK {
    #[doc = "Control write access to BOOT_SEED_REG registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_BOOT_SEED(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control write access to BOOT_SEED_REG registers."]
    #[inline(always)]
    pub const fn set_LOCK_BOOT_SEED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control write access to HMAC_REG registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_HMAC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control write access to HMAC_REG registers."]
    #[inline(always)]
    pub const fn set_LOCK_HMAC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for BOOT_LOCK {
    #[inline(always)]
    fn default() -> BOOT_LOCK {
        BOOT_LOCK(0)
    }
}
impl core::fmt::Debug for BOOT_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_LOCK")
            .field("LOCK_BOOT_SEED", &self.LOCK_BOOT_SEED())
            .field("LOCK_HMAC", &self.LOCK_HMAC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_LOCK {{ LOCK_BOOT_SEED: {=bool:?}, LOCK_HMAC: {=bool:?} }}",
            self.LOCK_BOOT_SEED(),
            self.LOCK_HMAC()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG0(pub u32);
impl BOOT_SEED_REG0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG0 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG0 {
        BOOT_SEED_REG0(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG0")
            .field("BOOT_SEED_REG0", &self.BOOT_SEED_REG0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG0 {{ BOOT_SEED_REG0: {=u32:?} }}",
            self.BOOT_SEED_REG0()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG1(pub u32);
impl BOOT_SEED_REG1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG1 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG1 {
        BOOT_SEED_REG1(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG1")
            .field("BOOT_SEED_REG1", &self.BOOT_SEED_REG1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG1 {{ BOOT_SEED_REG1: {=u32:?} }}",
            self.BOOT_SEED_REG1()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG2(pub u32);
impl BOOT_SEED_REG2 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG2 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG2 {
        BOOT_SEED_REG2(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG2")
            .field("BOOT_SEED_REG2", &self.BOOT_SEED_REG2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG2 {{ BOOT_SEED_REG2: {=u32:?} }}",
            self.BOOT_SEED_REG2()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG3(pub u32);
impl BOOT_SEED_REG3 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG3 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG3 {
        BOOT_SEED_REG3(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG3")
            .field("BOOT_SEED_REG3", &self.BOOT_SEED_REG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG3 {{ BOOT_SEED_REG3: {=u32:?} }}",
            self.BOOT_SEED_REG3()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG4(pub u32);
impl BOOT_SEED_REG4 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG4 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG4 {
        BOOT_SEED_REG4(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG4")
            .field("BOOT_SEED_REG4", &self.BOOT_SEED_REG4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG4 {{ BOOT_SEED_REG4: {=u32:?} }}",
            self.BOOT_SEED_REG4()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG5(pub u32);
impl BOOT_SEED_REG5 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG5 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG5 {
        BOOT_SEED_REG5(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG5")
            .field("BOOT_SEED_REG5", &self.BOOT_SEED_REG5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG5 {{ BOOT_SEED_REG5: {=u32:?} }}",
            self.BOOT_SEED_REG5()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG6(pub u32);
impl BOOT_SEED_REG6 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG6 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG6 {
        BOOT_SEED_REG6(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG6")
            .field("BOOT_SEED_REG6", &self.BOOT_SEED_REG6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG6 {{ BOOT_SEED_REG6: {=u32:?} }}",
            self.BOOT_SEED_REG6()
        )
    }
}
#[doc = "boot seed (256-bit random value)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_SEED_REG7(pub u32);
impl BOOT_SEED_REG7 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_REG7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_REG7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BOOT_SEED_REG7 {
    #[inline(always)]
    fn default() -> BOOT_SEED_REG7 {
        BOOT_SEED_REG7(0)
    }
}
impl core::fmt::Debug for BOOT_SEED_REG7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_SEED_REG7")
            .field("BOOT_SEED_REG7", &self.BOOT_SEED_REG7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_SEED_REG7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_SEED_REG7 {{ BOOT_SEED_REG7: {=u32:?} }}",
            self.BOOT_SEED_REG7()
        )
    }
}
#[doc = "CAN clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CANCLKDIV(pub u32);
impl CANCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::CANCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::CANCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::CANCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::CANCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CANCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::CANCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::CANCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CANCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::CANCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CANCLKDIV {
    #[inline(always)]
    fn default() -> CANCLKDIV {
        CANCLKDIV(0)
    }
}
impl core::fmt::Debug for CANCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CANCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CANCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "CAN clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CANCLKSEL(pub u32);
impl CANCLKSEL {
    #[doc = "CAN clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CANCLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CANCLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "CAN clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CANCLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CANCLKSEL {
    #[inline(always)]
    fn default() -> CANCLKSEL {
        CANCLKSEL(0)
    }
}
impl core::fmt::Debug for CANCLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANCLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CANCLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CANCLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Control CASPER integration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CASPER_CTRL(pub u32);
impl CASPER_CTRL {
    #[doc = "Control RAM access for RAMX0 and RAMX1."]
    #[must_use]
    #[inline(always)]
    pub const fn INTERLEAVE(&self) -> super::vals::INTERLEAVE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::INTERLEAVE::from_bits(val as u8)
    }
    #[doc = "Control RAM access for RAMX0 and RAMX1."]
    #[inline(always)]
    pub const fn set_INTERLEAVE(&mut self, val: super::vals::INTERLEAVE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CASPER_CTRL {
    #[inline(always)]
    fn default() -> CASPER_CTRL {
        CASPER_CTRL(0)
    }
}
impl core::fmt::Debug for CASPER_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CASPER_CTRL")
            .field("INTERLEAVE", &self.INTERLEAVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CASPER_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CASPER_CTRL {{ INTERLEAVE: {:?} }}", self.INTERLEAVE())
    }
}
#[doc = "clock low speed source select for HS USB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLK32KCLKSEL(pub u32);
impl CLK32KCLKSEL {
    #[doc = "clock low speed source select for HS USB."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CLK32KCLKSEL_SEL {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CLK32KCLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "clock low speed source select for HS USB."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CLK32KCLKSEL_SEL) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for CLK32KCLKSEL {
    #[inline(always)]
    fn default() -> CLK32KCLKSEL {
        CLK32KCLKSEL(0)
    }
}
impl core::fmt::Debug for CLK32KCLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK32KCLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLK32KCLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLK32KCLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "CLKOUT clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKOUTDIV(pub u32);
impl CLKOUTDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::CLKOUTDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::CLKOUTDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::CLKOUTDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::CLKOUTDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CLKOUTDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::CLKOUTDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::CLKOUTDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CLKOUTDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::CLKOUTDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CLKOUTDIV {
    #[inline(always)]
    fn default() -> CLKOUTDIV {
        CLKOUTDIV(0)
    }
}
impl core::fmt::Debug for CLKOUTDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKOUTDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKOUTDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLKOUTDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "CLKOUT clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKOUTSEL(pub u32);
impl CLKOUTSEL {
    #[doc = "CLKOUT clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CLKOUTSEL_SEL {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::CLKOUTSEL_SEL::from_bits(val as u8)
    }
    #[doc = "CLKOUT clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CLKOUTSEL_SEL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for CLKOUTSEL {
    #[inline(always)]
    fn default() -> CLKOUTSEL {
        CLKOUTSEL(0)
    }
}
impl core::fmt::Debug for CLKOUTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKOUTSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKOUTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLKOUTSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLOCKGENUPDATELOCKOUT(pub u32);
impl CLOCKGENUPDATELOCKOUT {
    #[doc = "Control clock configuration registers access (for example, xxxDIV, xxxSEL)."]
    #[must_use]
    #[inline(always)]
    pub const fn CLOCKGENUPDATELOCKOUT(&self) -> super::vals::CLOCKGENUPDATELOCKOUT {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::CLOCKGENUPDATELOCKOUT::from_bits(val as u32)
    }
    #[doc = "Control clock configuration registers access (for example, xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub const fn set_CLOCKGENUPDATELOCKOUT(&mut self, val: super::vals::CLOCKGENUPDATELOCKOUT) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLOCKGENUPDATELOCKOUT {
    #[inline(always)]
    fn default() -> CLOCKGENUPDATELOCKOUT {
        CLOCKGENUPDATELOCKOUT(0)
    }
}
impl core::fmt::Debug for CLOCKGENUPDATELOCKOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCKGENUPDATELOCKOUT")
            .field("CLOCKGENUPDATELOCKOUT", &self.CLOCKGENUPDATELOCKOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLOCKGENUPDATELOCKOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLOCKGENUPDATELOCKOUT {{ CLOCKGENUPDATELOCKOUT: {:?} }}",
            self.CLOCKGENUPDATELOCKOUT()
        )
    }
}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLOCK_CTRL(pub u32);
impl CLOCK_CTRL {
    #[doc = "Enable XTAL32MHz clock for Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn XTAL32MHZ_FREQM_ENA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub const fn set_XTAL32MHZ_FREQM_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO1MHZ_UTICK_ENA(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub const fn set_FRO1MHZ_UTICK_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable FRO 12MHz clock for Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO12MHZ_FREQM_ENA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub const fn set_FRO12MHZ_FREQM_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable FRO 96MHz clock for Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO_HF_FREQM_ENA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub const fn set_FRO_HF_FREQM_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable clock_in clock for clock module."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKIN_ENA(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clock_in clock for clock module."]
    #[inline(always)]
    pub const fn set_CLKIN_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO1MHZ_CLK_ENA(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub const fn set_FRO1MHZ_CLK_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn ANA_FRO12M_CLK_ENA(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub const fn set_ANA_FRO12M_CLK_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable clock for cristal oscilator calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn XO_CAL_CLK_ENA(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub const fn set_XO_CAL_CLK_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[must_use]
    #[inline(always)]
    pub const fn PLU_DEGLITCH_CLK_ENA(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub const fn set_PLU_DEGLITCH_CLK_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CLOCK_CTRL {
    #[inline(always)]
    fn default() -> CLOCK_CTRL {
        CLOCK_CTRL(0)
    }
}
impl core::fmt::Debug for CLOCK_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_CTRL")
            .field("XTAL32MHZ_FREQM_ENA", &self.XTAL32MHZ_FREQM_ENA())
            .field("FRO1MHZ_UTICK_ENA", &self.FRO1MHZ_UTICK_ENA())
            .field("FRO12MHZ_FREQM_ENA", &self.FRO12MHZ_FREQM_ENA())
            .field("FRO_HF_FREQM_ENA", &self.FRO_HF_FREQM_ENA())
            .field("CLKIN_ENA", &self.CLKIN_ENA())
            .field("FRO1MHZ_CLK_ENA", &self.FRO1MHZ_CLK_ENA())
            .field("ANA_FRO12M_CLK_ENA", &self.ANA_FRO12M_CLK_ENA())
            .field("XO_CAL_CLK_ENA", &self.XO_CAL_CLK_ENA())
            .field("PLU_DEGLITCH_CLK_ENA", &self.PLU_DEGLITCH_CLK_ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLOCK_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLOCK_CTRL {{ XTAL32MHZ_FREQM_ENA: {=bool:?}, FRO1MHZ_UTICK_ENA: {=bool:?}, FRO12MHZ_FREQM_ENA: {=bool:?}, FRO_HF_FREQM_ENA: {=bool:?}, CLKIN_ENA: {=bool:?}, FRO1MHZ_CLK_ENA: {=bool:?}, ANA_FRO12M_CLK_ENA: {=bool:?}, XO_CAL_CLK_ENA: {=bool:?}, PLU_DEGLITCH_CLK_ENA: {=bool:?} }}",
            self.XTAL32MHZ_FREQM_ENA(),
            self.FRO1MHZ_UTICK_ENA(),
            self.FRO12MHZ_FREQM_ENA(),
            self.FRO_HF_FREQM_ENA(),
            self.CLKIN_ENA(),
            self.FRO1MHZ_CLK_ENA(),
            self.ANA_FRO12M_CLK_ENA(),
            self.XO_CAL_CLK_ENA(),
            self.PLU_DEGLITCH_CLK_ENA()
        )
    }
}
#[doc = "Comparator Interrupt control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP_INT_CTRL(pub u32);
impl COMP_INT_CTRL {
    #[doc = "Analog Comparator interrupt enable control:."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_ENABLE(&self) -> super::vals::INT_ENABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::INT_ENABLE::from_bits(val as u8)
    }
    #[doc = "Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub const fn set_INT_ENABLE(&mut self, val: super::vals::INT_ENABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_CLEAR(&self) -> super::vals::INT_CLEAR {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::INT_CLEAR::from_bits(val as u8)
    }
    #[doc = "Analog Comparator interrupt clear."]
    #[inline(always)]
    pub const fn set_INT_CLEAR(&mut self, val: super::vals::INT_CLEAR) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator interrupt type selector:."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_CTRL(&self) -> super::vals::INT_CTRL {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::INT_CTRL::from_bits(val as u8)
    }
    #[doc = "Comparator interrupt type selector:."]
    #[inline(always)]
    pub const fn set_INT_CTRL(&mut self, val: super::vals::INT_CTRL) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_SOURCE(&self) -> super::vals::INT_SOURCE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::INT_SOURCE::from_bits(val as u8)
    }
    #[doc = "Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub const fn set_INT_SOURCE(&mut self, val: super::vals::INT_SOURCE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for COMP_INT_CTRL {
    #[inline(always)]
    fn default() -> COMP_INT_CTRL {
        COMP_INT_CTRL(0)
    }
}
impl core::fmt::Debug for COMP_INT_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_INT_CTRL")
            .field("INT_ENABLE", &self.INT_ENABLE())
            .field("INT_CLEAR", &self.INT_CLEAR())
            .field("INT_CTRL", &self.INT_CTRL())
            .field("INT_SOURCE", &self.INT_SOURCE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP_INT_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP_INT_CTRL {{ INT_ENABLE: {:?}, INT_CLEAR: {:?}, INT_CTRL: {:?}, INT_SOURCE: {:?} }}",
            self.INT_ENABLE(),
            self.INT_CLEAR(),
            self.INT_CTRL(),
            self.INT_SOURCE()
        )
    }
}
#[doc = "Comparator Interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP_INT_STATUS(pub u32);
impl COMP_INT_STATUS {
    #[doc = "Interrupt status BEFORE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn STATUS(&self) -> super::vals::STATUS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::STATUS::from_bits(val as u8)
    }
    #[doc = "Interrupt status BEFORE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_STATUS(&mut self, val: super::vals::STATUS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status AFTER Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn INT_STATUS(&self) -> super::vals::INT_STATUS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::INT_STATUS::from_bits(val as u8)
    }
    #[doc = "Interrupt status AFTER Interrupt Enable."]
    #[inline(always)]
    pub const fn set_INT_STATUS(&mut self, val: super::vals::INT_STATUS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "comparator analog output."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> super::vals::VAL {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::VAL::from_bits(val as u8)
    }
    #[doc = "comparator analog output."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: super::vals::VAL) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for COMP_INT_STATUS {
    #[inline(always)]
    fn default() -> COMP_INT_STATUS {
        COMP_INT_STATUS(0)
    }
}
impl core::fmt::Debug for COMP_INT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_INT_STATUS")
            .field("STATUS", &self.STATUS())
            .field("INT_STATUS", &self.INT_STATUS())
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP_INT_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP_INT_STATUS {{ STATUS: {:?}, INT_STATUS: {:?}, VAL: {:?} }}",
            self.STATUS(),
            self.INT_STATUS(),
            self.VAL()
        )
    }
}
#[doc = "CPU Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPSTAT(pub u32);
impl CPSTAT {
    #[doc = "The CPU0 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0SLEEPING(&self) -> super::vals::CPU0SLEEPING {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CPU0SLEEPING::from_bits(val as u8)
    }
    #[doc = "The CPU0 sleeping state."]
    #[inline(always)]
    pub const fn set_CPU0SLEEPING(&mut self, val: super::vals::CPU0SLEEPING) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The CPU0 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0LOCKUP(&self) -> super::vals::CPU0LOCKUP {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CPU0LOCKUP::from_bits(val as u8)
    }
    #[doc = "The CPU0 lockup state."]
    #[inline(always)]
    pub const fn set_CPU0LOCKUP(&mut self, val: super::vals::CPU0LOCKUP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for CPSTAT {
    #[inline(always)]
    fn default() -> CPSTAT {
        CPSTAT(0)
    }
}
impl core::fmt::Debug for CPSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPSTAT")
            .field("CPU0SLEEPING", &self.CPU0SLEEPING())
            .field("CPU0LOCKUP", &self.CPU0LOCKUP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPSTAT {{ CPU0SLEEPING: {:?}, CPU0LOCKUP: {:?} }}",
            self.CPU0SLEEPING(),
            self.CPU0LOCKUP()
        )
    }
}
#[doc = "System tick calibration for non-secure part of CPU0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU0NSTCKCAL(pub u32);
impl CPU0NSTCKCAL {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn TENMS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_TENMS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[must_use]
    #[inline(always)]
    pub const fn SKEW(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub const fn set_SKEW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Initial value for the Systick timer."]
    #[must_use]
    #[inline(always)]
    pub const fn NOREF(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Initial value for the Systick timer."]
    #[inline(always)]
    pub const fn set_NOREF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for CPU0NSTCKCAL {
    #[inline(always)]
    fn default() -> CPU0NSTCKCAL {
        CPU0NSTCKCAL(0)
    }
}
impl core::fmt::Debug for CPU0NSTCKCAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU0NSTCKCAL")
            .field("TENMS", &self.TENMS())
            .field("SKEW", &self.SKEW())
            .field("NOREF", &self.NOREF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPU0NSTCKCAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPU0NSTCKCAL {{ TENMS: {=u32:?}, SKEW: {=bool:?}, NOREF: {=bool:?} }}",
            self.TENMS(),
            self.SKEW(),
            self.NOREF()
        )
    }
}
#[doc = "System tick calibration for secure part of CPU0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU0STCKCAL(pub u32);
impl CPU0STCKCAL {
    #[doc = "Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn TENMS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_TENMS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Initial value for the Systick timer."]
    #[must_use]
    #[inline(always)]
    pub const fn SKEW(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Initial value for the Systick timer."]
    #[inline(always)]
    pub const fn set_SKEW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[must_use]
    #[inline(always)]
    pub const fn NOREF(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub const fn set_NOREF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for CPU0STCKCAL {
    #[inline(always)]
    fn default() -> CPU0STCKCAL {
        CPU0STCKCAL(0)
    }
}
impl core::fmt::Debug for CPU0STCKCAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU0STCKCAL")
            .field("TENMS", &self.TENMS())
            .field("SKEW", &self.SKEW())
            .field("NOREF", &self.NOREF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPU0STCKCAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPU0STCKCAL {{ TENMS: {=u32:?}, SKEW: {=bool:?}, NOREF: {=bool:?} }}",
            self.TENMS(),
            self.SKEW(),
            self.NOREF()
        )
    }
}
#[doc = "CTimer 0 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSEL0(pub u32);
impl CTIMERCLKSEL0 {
    #[doc = "CTimer 0 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CTIMERCLKSEL0_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CTIMERCLKSEL0_SEL::from_bits(val as u8)
    }
    #[doc = "CTimer 0 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CTIMERCLKSEL0_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CTIMERCLKSEL0 {
    #[inline(always)]
    fn default() -> CTIMERCLKSEL0 {
        CTIMERCLKSEL0(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSEL0")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSEL0 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "CTimer 1 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSEL1(pub u32);
impl CTIMERCLKSEL1 {
    #[doc = "CTimer 1 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CTIMERCLKSEL1_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CTIMERCLKSEL1_SEL::from_bits(val as u8)
    }
    #[doc = "CTimer 1 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CTIMERCLKSEL1_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CTIMERCLKSEL1 {
    #[inline(always)]
    fn default() -> CTIMERCLKSEL1 {
        CTIMERCLKSEL1(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSEL1")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSEL1 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "CTimer 2 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSEL2(pub u32);
impl CTIMERCLKSEL2 {
    #[doc = "CTimer 2 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CTIMERCLKSEL2_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CTIMERCLKSEL2_SEL::from_bits(val as u8)
    }
    #[doc = "CTimer 2 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CTIMERCLKSEL2_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CTIMERCLKSEL2 {
    #[inline(always)]
    fn default() -> CTIMERCLKSEL2 {
        CTIMERCLKSEL2(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSEL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSEL2")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSEL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSEL2 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "CTimer 3 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSEL3(pub u32);
impl CTIMERCLKSEL3 {
    #[doc = "CTimer 3 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CTIMERCLKSEL3_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CTIMERCLKSEL3_SEL::from_bits(val as u8)
    }
    #[doc = "CTimer 3 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CTIMERCLKSEL3_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CTIMERCLKSEL3 {
    #[inline(always)]
    fn default() -> CTIMERCLKSEL3 {
        CTIMERCLKSEL3(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSEL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSEL3")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSEL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSEL3 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "CTimer 4 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSEL4(pub u32);
impl CTIMERCLKSEL4 {
    #[doc = "CTimer 4 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::CTIMERCLKSEL4_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CTIMERCLKSEL4_SEL::from_bits(val as u8)
    }
    #[doc = "CTimer 4 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::CTIMERCLKSEL4_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CTIMERCLKSEL4 {
    #[inline(always)]
    fn default() -> CTIMERCLKSEL4 {
        CTIMERCLKSEL4(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSEL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSEL4")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSEL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSEL4 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSELX0(pub u32);
impl CTIMERCLKSELX0 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CTIMERCLKSELX0 {
    #[inline(always)]
    fn default() -> CTIMERCLKSELX0 {
        CTIMERCLKSELX0(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSELX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSELX0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSELX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSELX0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSELX1(pub u32);
impl CTIMERCLKSELX1 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CTIMERCLKSELX1 {
    #[inline(always)]
    fn default() -> CTIMERCLKSELX1 {
        CTIMERCLKSELX1(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSELX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSELX1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSELX1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSELX1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSELX2(pub u32);
impl CTIMERCLKSELX2 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CTIMERCLKSELX2 {
    #[inline(always)]
    fn default() -> CTIMERCLKSELX2 {
        CTIMERCLKSELX2(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSELX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSELX2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSELX2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSELX2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSELX3(pub u32);
impl CTIMERCLKSELX3 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CTIMERCLKSELX3 {
    #[inline(always)]
    fn default() -> CTIMERCLKSELX3 {
        CTIMERCLKSELX3(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSELX3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSELX3")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSELX3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSELX3 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMERCLKSELX4(pub u32);
impl CTIMERCLKSELX4 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CTIMERCLKSELX4 {
    #[inline(always)]
    fn default() -> CTIMERCLKSELX4 {
        CTIMERCLKSELX4(0)
    }
}
impl core::fmt::Debug for CTIMERCLKSELX4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIMERCLKSELX4")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTIMERCLKSELX4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CTIMERCLKSELX4 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Debug authentication BEACON register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG_AUTH_BEACON(pub u32);
impl DEBUG_AUTH_BEACON {
    #[doc = "Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[must_use]
    #[inline(always)]
    pub const fn BEACON(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub const fn set_BEACON(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DEBUG_AUTH_BEACON {
    #[inline(always)]
    fn default() -> DEBUG_AUTH_BEACON {
        DEBUG_AUTH_BEACON(0)
    }
}
impl core::fmt::Debug for DEBUG_AUTH_BEACON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_AUTH_BEACON")
            .field("BEACON", &self.BEACON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG_AUTH_BEACON {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DEBUG_AUTH_BEACON {{ BEACON: {=u32:?} }}", self.BEACON())
    }
}
#[doc = "Cortex debug features control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG_FEATURES(pub u32);
impl DEBUG_FEATURES {
    #[doc = "CPU0 Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_DBGEN(&self) -> super::vals::DEBUG_FEATURES_CPU0_DBGEN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DEBUG_FEATURES_CPU0_DBGEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_DBGEN(&mut self, val: super::vals::DEBUG_FEATURES_CPU0_DBGEN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_NIDEN(&self) -> super::vals::DEBUG_FEATURES_CPU0_NIDEN {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DEBUG_FEATURES_CPU0_NIDEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_NIDEN(&mut self, val: super::vals::DEBUG_FEATURES_CPU0_NIDEN) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_SPIDEN(&self) -> super::vals::DEBUG_FEATURES_CPU0_SPIDEN {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DEBUG_FEATURES_CPU0_SPIDEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_SPIDEN(&mut self, val: super::vals::DEBUG_FEATURES_CPU0_SPIDEN) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_SPNIDEN(&self) -> super::vals::DEBUG_FEATURES_CPU0_SPNIDEN {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DEBUG_FEATURES_CPU0_SPNIDEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_SPNIDEN(&mut self, val: super::vals::DEBUG_FEATURES_CPU0_SPNIDEN) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DEBUG_FEATURES {
    #[inline(always)]
    fn default() -> DEBUG_FEATURES {
        DEBUG_FEATURES(0)
    }
}
impl core::fmt::Debug for DEBUG_FEATURES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_FEATURES")
            .field("CPU0_DBGEN", &self.CPU0_DBGEN())
            .field("CPU0_NIDEN", &self.CPU0_NIDEN())
            .field("CPU0_SPIDEN", &self.CPU0_SPIDEN())
            .field("CPU0_SPNIDEN", &self.CPU0_SPNIDEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG_FEATURES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBUG_FEATURES {{ CPU0_DBGEN: {:?}, CPU0_NIDEN: {:?}, CPU0_SPIDEN: {:?}, CPU0_SPNIDEN: {:?} }}",
            self.CPU0_DBGEN(),
            self.CPU0_NIDEN(),
            self.CPU0_SPIDEN(),
            self.CPU0_SPNIDEN()
        )
    }
}
#[doc = "Cortex debug features control. (duplicate)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG_FEATURES_DP(pub u32);
impl DEBUG_FEATURES_DP {
    #[doc = "CPU0 (CPU0) Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_DBGEN(&self) -> super::vals::DEBUG_FEATURES_DP_CPU0_DBGEN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DEBUG_FEATURES_DP_CPU0_DBGEN::from_bits(val as u8)
    }
    #[doc = "CPU0 (CPU0) Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_DBGEN(&mut self, val: super::vals::DEBUG_FEATURES_DP_CPU0_DBGEN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_NIDEN(&self) -> super::vals::DEBUG_FEATURES_DP_CPU0_NIDEN {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DEBUG_FEATURES_DP_CPU0_NIDEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_NIDEN(&mut self, val: super::vals::DEBUG_FEATURES_DP_CPU0_NIDEN) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_SPIDEN(&self) -> super::vals::DEBUG_FEATURES_DP_CPU0_SPIDEN {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DEBUG_FEATURES_DP_CPU0_SPIDEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_SPIDEN(&mut self, val: super::vals::DEBUG_FEATURES_DP_CPU0_SPIDEN) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_SPNIDEN(&self) -> super::vals::DEBUG_FEATURES_DP_CPU0_SPNIDEN {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DEBUG_FEATURES_DP_CPU0_SPNIDEN::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_CPU0_SPNIDEN(&mut self, val: super::vals::DEBUG_FEATURES_DP_CPU0_SPNIDEN) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DEBUG_FEATURES_DP {
    #[inline(always)]
    fn default() -> DEBUG_FEATURES_DP {
        DEBUG_FEATURES_DP(0)
    }
}
impl core::fmt::Debug for DEBUG_FEATURES_DP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_FEATURES_DP")
            .field("CPU0_DBGEN", &self.CPU0_DBGEN())
            .field("CPU0_NIDEN", &self.CPU0_NIDEN())
            .field("CPU0_SPIDEN", &self.CPU0_SPIDEN())
            .field("CPU0_SPNIDEN", &self.CPU0_SPNIDEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG_FEATURES_DP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBUG_FEATURES_DP {{ CPU0_DBGEN: {:?}, CPU0_NIDEN: {:?}, CPU0_SPIDEN: {:?}, CPU0_SPNIDEN: {:?} }}",
            self.CPU0_DBGEN(),
            self.CPU0_NIDEN(),
            self.CPU0_SPIDEN(),
            self.CPU0_SPNIDEN()
        )
    }
}
#[doc = "Control write access to security registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG_LOCK_EN(pub u32);
impl DEBUG_LOCK_EN {
    #[doc = "Control write access to security registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_ALL(&self) -> super::vals::LOCK_ALL {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LOCK_ALL::from_bits(val as u8)
    }
    #[doc = "Control write access to security registers."]
    #[inline(always)]
    pub const fn set_LOCK_ALL(&mut self, val: super::vals::LOCK_ALL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DEBUG_LOCK_EN {
    #[inline(always)]
    fn default() -> DEBUG_LOCK_EN {
        DEBUG_LOCK_EN(0)
    }
}
impl core::fmt::Debug for DEBUG_LOCK_EN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_LOCK_EN")
            .field("LOCK_ALL", &self.LOCK_ALL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG_LOCK_EN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DEBUG_LOCK_EN {{ LOCK_ALL: {:?} }}", self.LOCK_ALL())
    }
}
#[doc = "Device ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEVICE_ID0(pub u32);
impl DEVICE_ID0 {
    #[doc = "ROM revision."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM_REV_MINOR(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM revision."]
    #[inline(always)]
    pub const fn set_ROM_REV_MINOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for DEVICE_ID0 {
    #[inline(always)]
    fn default() -> DEVICE_ID0 {
        DEVICE_ID0(0)
    }
}
impl core::fmt::Debug for DEVICE_ID0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_ID0")
            .field("ROM_REV_MINOR", &self.ROM_REV_MINOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEVICE_ID0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEVICE_ID0 {{ ROM_REV_MINOR: {=u8:?} }}",
            self.ROM_REV_MINOR()
        )
    }
}
#[doc = "Chip revision ID and Number."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIEID(pub u32);
impl DIEID {
    #[doc = "Chip Metal Revision ID."]
    #[must_use]
    #[inline(always)]
    pub const fn REV_ID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip Metal Revision ID."]
    #[inline(always)]
    pub const fn set_REV_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Chip Number 0x426B."]
    #[must_use]
    #[inline(always)]
    pub const fn MCO_NUM_IN_DIE_ID(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Chip Number 0x426B."]
    #[inline(always)]
    pub const fn set_MCO_NUM_IN_DIE_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 4usize)) | (((val as u32) & 0x000f_ffff) << 4usize);
    }
}
impl Default for DIEID {
    #[inline(always)]
    fn default() -> DIEID {
        DIEID(0)
    }
}
impl core::fmt::Debug for DIEID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEID")
            .field("REV_ID", &self.REV_ID())
            .field("MCO_NUM_IN_DIE_ID", &self.MCO_NUM_IN_DIE_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIEID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DIEID {{ REV_ID: {=u8:?}, MCO_NUM_IN_DIE_ID: {=u32:?} }}",
            self.REV_ID(),
            self.MCO_NUM_IN_DIE_ID()
        )
    }
}
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL0(pub u32);
impl FCCLKSEL0 {
    #[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL0_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL0_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL0_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL0 {
    #[inline(always)]
    fn default() -> FCCLKSEL0 {
        FCCLKSEL0(0)
    }
}
impl core::fmt::Debug for FCCLKSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL0")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL0 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL1(pub u32);
impl FCCLKSEL1 {
    #[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL1_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL1_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL1_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL1 {
    #[inline(always)]
    fn default() -> FCCLKSEL1 {
        FCCLKSEL1(0)
    }
}
impl core::fmt::Debug for FCCLKSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL1")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL1 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL2(pub u32);
impl FCCLKSEL2 {
    #[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL2_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL2_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL2_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL2 {
    #[inline(always)]
    fn default() -> FCCLKSEL2 {
        FCCLKSEL2(0)
    }
}
impl core::fmt::Debug for FCCLKSEL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL2")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL2 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL3(pub u32);
impl FCCLKSEL3 {
    #[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL3_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL3_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL3_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL3 {
    #[inline(always)]
    fn default() -> FCCLKSEL3 {
        FCCLKSEL3(0)
    }
}
impl core::fmt::Debug for FCCLKSEL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL3")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL3 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL4(pub u32);
impl FCCLKSEL4 {
    #[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL4_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL4_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL4_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL4 {
    #[inline(always)]
    fn default() -> FCCLKSEL4 {
        FCCLKSEL4(0)
    }
}
impl core::fmt::Debug for FCCLKSEL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL4")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL4 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL5(pub u32);
impl FCCLKSEL5 {
    #[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL5_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL5_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL5_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL5 {
    #[inline(always)]
    fn default() -> FCCLKSEL5 {
        FCCLKSEL5(0)
    }
}
impl core::fmt::Debug for FCCLKSEL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL5")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL5 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL6(pub u32);
impl FCCLKSEL6 {
    #[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL6_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL6_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL6_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL6 {
    #[inline(always)]
    fn default() -> FCCLKSEL6 {
        FCCLKSEL6(0)
    }
}
impl core::fmt::Debug for FCCLKSEL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL6")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL6 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSEL7(pub u32);
impl FCCLKSEL7 {
    #[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::FCCLKSEL7_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FCCLKSEL7_SEL::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::FCCLKSEL7_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for FCCLKSEL7 {
    #[inline(always)]
    fn default() -> FCCLKSEL7 {
        FCCLKSEL7(0)
    }
}
impl core::fmt::Debug for FCCLKSEL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSEL7")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSEL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSEL7 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX0(pub u32);
impl FCCLKSELX0 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX0 {
    #[inline(always)]
    fn default() -> FCCLKSELX0 {
        FCCLKSELX0(0)
    }
}
impl core::fmt::Debug for FCCLKSELX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX1(pub u32);
impl FCCLKSELX1 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX1 {
    #[inline(always)]
    fn default() -> FCCLKSELX1 {
        FCCLKSELX1(0)
    }
}
impl core::fmt::Debug for FCCLKSELX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX2(pub u32);
impl FCCLKSELX2 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX2 {
    #[inline(always)]
    fn default() -> FCCLKSELX2 {
        FCCLKSELX2(0)
    }
}
impl core::fmt::Debug for FCCLKSELX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX3(pub u32);
impl FCCLKSELX3 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX3 {
    #[inline(always)]
    fn default() -> FCCLKSELX3 {
        FCCLKSELX3(0)
    }
}
impl core::fmt::Debug for FCCLKSELX3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX3")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX3 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX4(pub u32);
impl FCCLKSELX4 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX4 {
    #[inline(always)]
    fn default() -> FCCLKSELX4 {
        FCCLKSELX4(0)
    }
}
impl core::fmt::Debug for FCCLKSELX4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX4")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX4 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX5(pub u32);
impl FCCLKSELX5 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX5 {
    #[inline(always)]
    fn default() -> FCCLKSELX5 {
        FCCLKSELX5(0)
    }
}
impl core::fmt::Debug for FCCLKSELX5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX5")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX5 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX6(pub u32);
impl FCCLKSELX6 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX6 {
    #[inline(always)]
    fn default() -> FCCLKSELX6 {
        FCCLKSELX6(0)
    }
}
impl core::fmt::Debug for FCCLKSELX6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX6")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX6 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCLKSELX7(pub u32);
impl FCCLKSELX7 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCCLKSELX7 {
    #[inline(always)]
    fn default() -> FCCLKSELX7 {
        FCCLKSELX7(0)
    }
}
impl core::fmt::Debug for FCCLKSELX7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCLKSELX7")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCLKSELX7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCCLKSELX7 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHREMAP_LOCK(pub u32);
impl FLASHREMAP_LOCK {
    #[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> super::vals::LOCK {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::LOCK::from_bits(val as u32)
    }
    #[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: super::vals::LOCK) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASHREMAP_LOCK {
    #[inline(always)]
    fn default() -> FLASHREMAP_LOCK {
        FLASHREMAP_LOCK(0)
    }
}
impl core::fmt::Debug for FLASHREMAP_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHREMAP_LOCK")
            .field("LOCK", &self.LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHREMAP_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLASHREMAP_LOCK {{ LOCK: {:?} }}", self.LOCK())
    }
}
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHREMAP_OFFSET(pub u32);
impl FLASHREMAP_OFFSET {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHREMAP_OFFSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FLASHREMAP_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASHREMAP_OFFSET {
    #[inline(always)]
    fn default() -> FLASHREMAP_OFFSET {
        FLASHREMAP_OFFSET(0)
    }
}
impl core::fmt::Debug for FLASHREMAP_OFFSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHREMAP_OFFSET")
            .field("FLASHREMAP_OFFSET", &self.FLASHREMAP_OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHREMAP_OFFSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASHREMAP_OFFSET {{ FLASHREMAP_OFFSET: {=u32:?} }}",
            self.FLASHREMAP_OFFSET()
        )
    }
}
#[doc = "This 32-bit register is a duplicate of FLASHREMAPOFFSET for increased security."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHREMAP_OFFSET_DP(pub u32);
impl FLASHREMAP_OFFSET_DP {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHREMAP_OFFSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FLASHREMAP_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASHREMAP_OFFSET_DP {
    #[inline(always)]
    fn default() -> FLASHREMAP_OFFSET_DP {
        FLASHREMAP_OFFSET_DP(0)
    }
}
impl core::fmt::Debug for FLASHREMAP_OFFSET_DP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHREMAP_OFFSET_DP")
            .field("FLASHREMAP_OFFSET", &self.FLASHREMAP_OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHREMAP_OFFSET_DP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASHREMAP_OFFSET_DP {{ FLASHREMAP_OFFSET: {=u32:?} }}",
            self.FLASHREMAP_OFFSET()
        )
    }
}
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHREMAP_SIZE(pub u32);
impl FLASHREMAP_SIZE {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHREMAP_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FLASHREMAP_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASHREMAP_SIZE {
    #[inline(always)]
    fn default() -> FLASHREMAP_SIZE {
        FLASHREMAP_SIZE(0)
    }
}
impl core::fmt::Debug for FLASHREMAP_SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHREMAP_SIZE")
            .field("FLASHREMAP_SIZE", &self.FLASHREMAP_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHREMAP_SIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASHREMAP_SIZE {{ FLASHREMAP_SIZE: {=u32:?} }}",
            self.FLASHREMAP_SIZE()
        )
    }
}
#[doc = "This 32-bit register is a duplicate of FLASHREMAPSIZE for increased security."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHREMAP_SIZE_DP(pub u32);
impl FLASHREMAP_SIZE_DP {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHREMAP_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FLASHREMAP_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASHREMAP_SIZE_DP {
    #[inline(always)]
    fn default() -> FLASHREMAP_SIZE_DP {
        FLASHREMAP_SIZE_DP(0)
    }
}
impl core::fmt::Debug for FLASHREMAP_SIZE_DP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHREMAP_SIZE_DP")
            .field("FLASHREMAP_SIZE", &self.FLASHREMAP_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHREMAP_SIZE_DP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASHREMAP_SIZE_DP {{ FLASHREMAP_SIZE: {=u32:?} }}",
            self.FLASHREMAP_SIZE()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG0CTRL(pub u32);
impl FLEXFRG0CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG0CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG0CTRL {
        FLEXFRG0CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG0CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG0CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG0CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG0CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG1CTRL(pub u32);
impl FLEXFRG1CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG1CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG1CTRL {
        FLEXFRG1CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG1CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG1CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG1CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG1CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG2CTRL(pub u32);
impl FLEXFRG2CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG2CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG2CTRL {
        FLEXFRG2CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG2CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG2CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG2CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG2CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG3CTRL(pub u32);
impl FLEXFRG3CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG3CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG3CTRL {
        FLEXFRG3CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG3CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG3CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG3CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG3CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG4CTRL(pub u32);
impl FLEXFRG4CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG4CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG4CTRL {
        FLEXFRG4CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG4CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG4CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG4CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG4CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG5CTRL(pub u32);
impl FLEXFRG5CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG5CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG5CTRL {
        FLEXFRG5CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG5CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG5CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG5CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG5CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG6CTRL(pub u32);
impl FLEXFRG6CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG6CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG6CTRL {
        FLEXFRG6CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG6CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG6CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG6CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG6CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Fractional rate divider for flexcomm 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRG7CTRL(pub u32);
impl FLEXFRG7CTRL {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn MULT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_MULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FLEXFRG7CTRL {
    #[inline(always)]
    fn default() -> FLEXFRG7CTRL {
        FLEXFRG7CTRL(0)
    }
}
impl core::fmt::Debug for FLEXFRG7CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRG7CTRL")
            .field("DIV", &self.DIV())
            .field("MULT", &self.MULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRG7CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXFRG7CTRL {{ DIV: {=u8:?}, MULT: {=u8:?} }}",
            self.DIV(),
            self.MULT()
        )
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL0(pub u32);
impl FLEXFRGXCTRL0 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL0 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL0 {
        FLEXFRGXCTRL0(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL1(pub u32);
impl FLEXFRGXCTRL1 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL1 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL1 {
        FLEXFRGXCTRL1(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL2(pub u32);
impl FLEXFRGXCTRL2 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL2 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL2 {
        FLEXFRGXCTRL2(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL3(pub u32);
impl FLEXFRGXCTRL3 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL3 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL3 {
        FLEXFRGXCTRL3(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL3")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL3 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL4(pub u32);
impl FLEXFRGXCTRL4 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL4 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL4 {
        FLEXFRGXCTRL4(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL4")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL4 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL5(pub u32);
impl FLEXFRGXCTRL5 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL5 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL5 {
        FLEXFRGXCTRL5(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL5")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL5 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL6(pub u32);
impl FLEXFRGXCTRL6 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL6 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL6 {
        FLEXFRGXCTRL6(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL6")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL6 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXFRGXCTRL7(pub u32);
impl FLEXFRGXCTRL7 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXFRGXCTRL7 {
    #[inline(always)]
    fn default() -> FLEXFRGXCTRL7 {
        FLEXFRGXCTRL7(0)
    }
}
impl core::fmt::Debug for FLEXFRGXCTRL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXFRGXCTRL7")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXFRGXCTRL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXFRGXCTRL7 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "FMC configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMCCR(pub u32);
impl FMCCR {
    #[doc = "Instruction fetch configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn FETCHCFG(&self) -> super::vals::FETCHCFG {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FETCHCFG::from_bits(val as u8)
    }
    #[doc = "Instruction fetch configuration."]
    #[inline(always)]
    pub const fn set_FETCHCFG(&mut self, val: super::vals::FETCHCFG) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Data read configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn DATACFG(&self) -> super::vals::DATACFG {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DATACFG::from_bits(val as u8)
    }
    #[doc = "Data read configuration."]
    #[inline(always)]
    pub const fn set_DATACFG(&mut self, val: super::vals::DATACFG) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Acceleration enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ACCEL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Acceleration enable."]
    #[inline(always)]
    pub const fn set_ACCEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Prefetch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PREFEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable."]
    #[inline(always)]
    pub const fn set_PREFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Prefetch override."]
    #[must_use]
    #[inline(always)]
    pub const fn PREFOVR(&self) -> super::vals::PREFOVR {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PREFOVR::from_bits(val as u8)
    }
    #[doc = "Prefetch override."]
    #[inline(always)]
    pub const fn set_PREFOVR(&mut self, val: super::vals::PREFOVR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Flash memory access time."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASHTIM(&self) -> super::vals::FLASHTIM {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::FLASHTIM::from_bits(val as u8)
    }
    #[doc = "Flash memory access time."]
    #[inline(always)]
    pub const fn set_FLASHTIM(&mut self, val: super::vals::FLASHTIM) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for FMCCR {
    #[inline(always)]
    fn default() -> FMCCR {
        FMCCR(0)
    }
}
impl core::fmt::Debug for FMCCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMCCR")
            .field("FETCHCFG", &self.FETCHCFG())
            .field("DATACFG", &self.DATACFG())
            .field("ACCEL", &self.ACCEL())
            .field("PREFEN", &self.PREFEN())
            .field("PREFOVR", &self.PREFOVR())
            .field("FLASHTIM", &self.FLASHTIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FMCCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FMCCR {{ FETCHCFG: {:?}, DATACFG: {:?}, ACCEL: {=bool:?}, PREFEN: {=bool:?}, PREFOVR: {:?}, FLASHTIM: {:?} }}",
            self.FETCHCFG(),
            self.DATACFG(),
            self.ACCEL(),
            self.PREFEN(),
            self.PREFOVR(),
            self.FLASHTIM()
        )
    }
}
#[doc = "FMCflush control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMCFLUSH(pub u32);
impl FMCFLUSH {
    #[doc = "Flush control."]
    #[must_use]
    #[inline(always)]
    pub const fn FLUSH(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush control."]
    #[inline(always)]
    pub const fn set_FLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for FMCFLUSH {
    #[inline(always)]
    fn default() -> FMCFLUSH {
        FMCFLUSH(0)
    }
}
impl core::fmt::Debug for FMCFLUSH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMCFLUSH")
            .field("FLUSH", &self.FLUSH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FMCFLUSH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FMCFLUSH {{ FLUSH: {=bool:?} }}", self.FLUSH())
    }
}
#[doc = "FRO1MHz Clock divider (FRO1M_divided)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRO1MCLKDIV(pub u32);
impl FRO1MCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::FRO1MCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FRO1MCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::FRO1MCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::FRO1MCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FRO1MCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::FRO1MCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::FRO1MCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FRO1MCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::FRO1MCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FRO1MCLKDIV {
    #[inline(always)]
    fn default() -> FRO1MCLKDIV {
        FRO1MCLKDIV(0)
    }
}
impl core::fmt::Debug for FRO1MCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRO1MCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRO1MCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRO1MCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "FRO_HF (96MHz) clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FROHFDIV(pub u32);
impl FROHFDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::FROHFDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FROHFDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::FROHFDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::FROHFDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FROHFDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::FROHFDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::FROHFDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FROHFDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::FROHFDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FROHFDIV {
    #[inline(always)]
    fn default() -> FROHFDIV {
        FROHFDIV(0)
    }
}
impl core::fmt::Debug for FROHFDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FROHFDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FROHFDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FROHFDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "Functional retention control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FUNCRETENTIONCTRL(pub u32);
impl FUNCRETENTIONCTRL {
    #[doc = "functional retention in power down only."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNCRETENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "functional retention in power down only."]
    #[inline(always)]
    pub const fn set_FUNCRETENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start address divided by 4 inside SRAMX bank."]
    #[must_use]
    #[inline(always)]
    pub const fn RET_START(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x1fff;
        val as u16
    }
    #[doc = "Start address divided by 4 inside SRAMX bank."]
    #[inline(always)]
    pub const fn set_RET_START(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 1usize)) | (((val as u32) & 0x1fff) << 1usize);
    }
    #[doc = "lenth of Scan chains to save."]
    #[must_use]
    #[inline(always)]
    pub const fn RET_LENTH(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x03ff;
        val as u16
    }
    #[doc = "lenth of Scan chains to save."]
    #[inline(always)]
    pub const fn set_RET_LENTH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
    }
}
impl Default for FUNCRETENTIONCTRL {
    #[inline(always)]
    fn default() -> FUNCRETENTIONCTRL {
        FUNCRETENTIONCTRL(0)
    }
}
impl core::fmt::Debug for FUNCRETENTIONCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNCRETENTIONCTRL")
            .field("FUNCRETENA", &self.FUNCRETENA())
            .field("RET_START", &self.RET_START())
            .field("RET_LENTH", &self.RET_LENTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FUNCRETENTIONCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FUNCRETENTIONCTRL {{ FUNCRETENA: {=bool:?}, RET_START: {=u16:?}, RET_LENTH: {=u16:?} }}",
            self.FUNCRETENA(),
            self.RET_START(),
            self.RET_LENTH()
        )
    }
}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIOPSYNC(pub u32);
impl GPIOPSYNC {
    #[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[must_use]
    #[inline(always)]
    pub const fn PSYNC(&self) -> super::vals::PSYNC {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PSYNC::from_bits(val as u8)
    }
    #[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub const fn set_PSYNC(&mut self, val: super::vals::PSYNC) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for GPIOPSYNC {
    #[inline(always)]
    fn default() -> GPIOPSYNC {
        GPIOPSYNC(0)
    }
}
impl core::fmt::Debug for GPIOPSYNC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOPSYNC")
            .field("PSYNC", &self.PSYNC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIOPSYNC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIOPSYNC {{ PSYNC: {:?} }}", self.PSYNC())
    }
}
#[doc = "Controls whether the HASH AES hardware secret key is restricted to use by secure code."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HASHRESTHWKEY(pub u32);
impl HASHRESTHWKEY {
    #[doc = "Code value that controls whether HASH AES hardware secret key is unlocked."]
    #[must_use]
    #[inline(always)]
    pub const fn UNLOCKCODE(&self) -> super::vals::UNLOCKCODE {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UNLOCKCODE::from_bits(val as u32)
    }
    #[doc = "Code value that controls whether HASH AES hardware secret key is unlocked."]
    #[inline(always)]
    pub const fn set_UNLOCKCODE(&mut self, val: super::vals::UNLOCKCODE) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HASHRESTHWKEY {
    #[inline(always)]
    fn default() -> HASHRESTHWKEY {
        HASHRESTHWKEY(0)
    }
}
impl core::fmt::Debug for HASHRESTHWKEY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASHRESTHWKEY")
            .field("UNLOCKCODE", &self.UNLOCKCODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HASHRESTHWKEY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HASHRESTHWKEY {{ UNLOCKCODE: {:?} }}", self.UNLOCKCODE())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG0(pub u32);
impl HMAC_REG0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG0 {
    #[inline(always)]
    fn default() -> HMAC_REG0 {
        HMAC_REG0(0)
    }
}
impl core::fmt::Debug for HMAC_REG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG0")
            .field("HMAC_REG0", &self.HMAC_REG0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG0 {{ HMAC_REG0: {=u32:?} }}", self.HMAC_REG0())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG1(pub u32);
impl HMAC_REG1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG1 {
    #[inline(always)]
    fn default() -> HMAC_REG1 {
        HMAC_REG1(0)
    }
}
impl core::fmt::Debug for HMAC_REG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG1")
            .field("HMAC_REG1", &self.HMAC_REG1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG1 {{ HMAC_REG1: {=u32:?} }}", self.HMAC_REG1())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG2(pub u32);
impl HMAC_REG2 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG2 {
    #[inline(always)]
    fn default() -> HMAC_REG2 {
        HMAC_REG2(0)
    }
}
impl core::fmt::Debug for HMAC_REG2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG2")
            .field("HMAC_REG2", &self.HMAC_REG2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG2 {{ HMAC_REG2: {=u32:?} }}", self.HMAC_REG2())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG3(pub u32);
impl HMAC_REG3 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG3 {
    #[inline(always)]
    fn default() -> HMAC_REG3 {
        HMAC_REG3(0)
    }
}
impl core::fmt::Debug for HMAC_REG3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG3")
            .field("HMAC_REG3", &self.HMAC_REG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG3 {{ HMAC_REG3: {=u32:?} }}", self.HMAC_REG3())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG4(pub u32);
impl HMAC_REG4 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG4 {
    #[inline(always)]
    fn default() -> HMAC_REG4 {
        HMAC_REG4(0)
    }
}
impl core::fmt::Debug for HMAC_REG4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG4")
            .field("HMAC_REG4", &self.HMAC_REG4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG4 {{ HMAC_REG4: {=u32:?} }}", self.HMAC_REG4())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG5(pub u32);
impl HMAC_REG5 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG5 {
    #[inline(always)]
    fn default() -> HMAC_REG5 {
        HMAC_REG5(0)
    }
}
impl core::fmt::Debug for HMAC_REG5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG5")
            .field("HMAC_REG5", &self.HMAC_REG5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG5 {{ HMAC_REG5: {=u32:?} }}", self.HMAC_REG5())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG6(pub u32);
impl HMAC_REG6 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG6 {
    #[inline(always)]
    fn default() -> HMAC_REG6 {
        HMAC_REG6(0)
    }
}
impl core::fmt::Debug for HMAC_REG6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG6")
            .field("HMAC_REG6", &self.HMAC_REG6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG6 {{ HMAC_REG6: {=u32:?} }}", self.HMAC_REG6())
    }
}
#[doc = "HMAC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HMAC_REG7(pub u32);
impl HMAC_REG7 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn HMAC_REG7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_HMAC_REG7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HMAC_REG7 {
    #[inline(always)]
    fn default() -> HMAC_REG7 {
        HMAC_REG7(0)
    }
}
impl core::fmt::Debug for HMAC_REG7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC_REG7")
            .field("HMAC_REG7", &self.HMAC_REG7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HMAC_REG7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HMAC_REG7 {{ HMAC_REG7: {=u32:?} }}", self.HMAC_REG7())
    }
}
#[doc = "HS LSPI clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSLSPICLKSEL(pub u32);
impl HSLSPICLKSEL {
    #[doc = "HS LSPI clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::HSLSPICLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::HSLSPICLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "HS LSPI clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::HSLSPICLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for HSLSPICLKSEL {
    #[inline(always)]
    fn default() -> HSLSPICLKSEL {
        HSLSPICLKSEL(0)
    }
}
impl core::fmt::Debug for HSLSPICLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSLSPICLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSLSPICLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSLSPICLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "block quiddikey/PUF all index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEY_BLOCK(pub u32);
impl KEY_BLOCK {
    #[doc = "Write a value to block quiddikey/PUF all index."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_BLOCK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write a value to block quiddikey/PUF all index."]
    #[inline(always)]
    pub const fn set_KEY_BLOCK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KEY_BLOCK {
    #[inline(always)]
    fn default() -> KEY_BLOCK {
        KEY_BLOCK(0)
    }
}
impl core::fmt::Debug for KEY_BLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_BLOCK")
            .field("KEY_BLOCK", &self.KEY_BLOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEY_BLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEY_BLOCK {{ KEY_BLOCK: {=u32:?} }}", self.KEY_BLOCK())
    }
}
#[doc = "Main clock A source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAINCLKSELA(pub u32);
impl MAINCLKSELA {
    #[doc = "Main clock A source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::MAINCLKSELA_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MAINCLKSELA_SEL::from_bits(val as u8)
    }
    #[doc = "Main clock A source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::MAINCLKSELA_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MAINCLKSELA {
    #[inline(always)]
    fn default() -> MAINCLKSELA {
        MAINCLKSELA(0)
    }
}
impl core::fmt::Debug for MAINCLKSELA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAINCLKSELA")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAINCLKSELA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MAINCLKSELA {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Main clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAINCLKSELB(pub u32);
impl MAINCLKSELB {
    #[doc = "Main clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::MAINCLKSELB_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MAINCLKSELB_SEL::from_bits(val as u8)
    }
    #[doc = "Main clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::MAINCLKSELB_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MAINCLKSELB {
    #[inline(always)]
    fn default() -> MAINCLKSELB {
        MAINCLKSELB(0)
    }
}
impl core::fmt::Debug for MAINCLKSELB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAINCLKSELB")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAINCLKSELB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MAINCLKSELB {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "MCLK clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCLKCLKSEL(pub u32);
impl MCLKCLKSEL {
    #[doc = "MCLK clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::MCLKCLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MCLKCLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "MCLK clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::MCLKCLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for MCLKCLKSEL {
    #[inline(always)]
    fn default() -> MCLKCLKSEL {
        MCLKCLKSEL(0)
    }
}
impl core::fmt::Debug for MCLKCLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCLKCLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCLKCLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MCLKCLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "I2S MCLK clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCLKDIV(pub u32);
impl MCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::MCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::MCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::MCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::MCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::MCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::MCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MCLKDIV {
    #[inline(always)]
    fn default() -> MCLKDIV {
        MCLKDIV(0)
    }
}
impl core::fmt::Debug for MCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "MCLK control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCLKIO(pub u32);
impl MCLKIO {
    #[doc = "MCLK control."]
    #[must_use]
    #[inline(always)]
    pub const fn MCLKIO(&self) -> super::vals::MCLKIO {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MCLKIO::from_bits(val as u8)
    }
    #[doc = "MCLK control."]
    #[inline(always)]
    pub const fn set_MCLKIO(&mut self, val: super::vals::MCLKIO) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for MCLKIO {
    #[inline(always)]
    fn default() -> MCLKIO {
        MCLKIO(0)
    }
}
impl core::fmt::Debug for MCLKIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCLKIO")
            .field("MCLKIO", &self.MCLKIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCLKIO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MCLKIO {{ MCLKIO: {:?} }}", self.MCLKIO())
    }
}
#[doc = "Memory Remap control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MEMORYREMAP(pub u32);
impl MEMORYREMAP {
    #[doc = "Select the location of the vector table :."]
    #[must_use]
    #[inline(always)]
    pub const fn MAP(&self) -> super::vals::MAP {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MAP::from_bits(val as u8)
    }
    #[doc = "Select the location of the vector table :."]
    #[inline(always)]
    pub const fn set_MAP(&mut self, val: super::vals::MAP) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MEMORYREMAP {
    #[inline(always)]
    fn default() -> MEMORYREMAP {
        MEMORYREMAP(0)
    }
}
impl core::fmt::Debug for MEMORYREMAP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMORYREMAP")
            .field("MAP", &self.MAP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MEMORYREMAP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MEMORYREMAP {{ MAP: {:?} }}", self.MAP())
    }
}
#[doc = "NMI Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NMISRC(pub u32);
impl NMISRC {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQCPU0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_IRQCPU0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn NMIENCPU0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_NMIENCPU0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NMISRC {
    #[inline(always)]
    fn default() -> NMISRC {
        NMISRC(0)
    }
}
impl core::fmt::Debug for NMISRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMISRC")
            .field("IRQCPU0", &self.IRQCPU0())
            .field("NMIENCPU0", &self.NMIENCPU0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NMISRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NMISRC {{ IRQCPU0: {=u8:?}, NMIENCPU0: {=bool:?} }}",
            self.IRQCPU0(),
            self.NMIENCPU0()
        )
    }
}
#[doc = "PLL0 clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0CLKDIV(pub u32);
impl PLL0CLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::PLL0CLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PLL0CLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::PLL0CLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::PLL0CLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PLL0CLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::PLL0CLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::PLL0CLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PLL0CLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::PLL0CLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL0CLKDIV {
    #[inline(always)]
    fn default() -> PLL0CLKDIV {
        PLL0CLKDIV(0)
    }
}
impl core::fmt::Debug for PLL0CLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0CLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0CLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL0CLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "PLL0 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0CLKSEL(pub u32);
impl PLL0CLKSEL {
    #[doc = "PLL0 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::PLL0CLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::PLL0CLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "PLL0 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::PLL0CLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for PLL0CLKSEL {
    #[inline(always)]
    fn default() -> PLL0CLKSEL {
        PLL0CLKSEL(0)
    }
}
impl core::fmt::Debug for PLL0CLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0CLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0CLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PLL0CLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "PLL0 550m control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0CTRL(pub u32);
impl PLL0CTRL {
    #[doc = "Bandwidth select R value."]
    #[must_use]
    #[inline(always)]
    pub const fn SELR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R value."]
    #[inline(always)]
    pub const fn set_SELR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I value."]
    #[must_use]
    #[inline(always)]
    pub const fn SELI(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I value."]
    #[inline(always)]
    pub const fn set_SELI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P value."]
    #[must_use]
    #[inline(always)]
    pub const fn SELP(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P value."]
    #[inline(always)]
    pub const fn set_SELP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPLL(&self) -> super::vals::PLL0CTRL_BYPASSPLL {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PLL0CTRL_BYPASSPLL::from_bits(val as u8)
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub const fn set_BYPASSPLL(&mut self, val: super::vals::PLL0CTRL_BYPASSPLL) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPOSTDIV2(&self) -> super::vals::PLL0CTRL_BYPASSPOSTDIV2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PLL0CTRL_BYPASSPOSTDIV2::from_bits(val as u8)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub const fn set_BYPASSPOSTDIV2(&mut self, val: super::vals::PLL0CTRL_BYPASSPOSTDIV2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMUPOFF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub const fn set_LIMUPOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn BWDIRECT(&self) -> super::vals::PLL0CTRL_BWDIRECT {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PLL0CTRL_BWDIRECT::from_bits(val as u8)
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_BWDIRECT(&mut self, val: super::vals::PLL0CTRL_BWDIRECT) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "bypass of the pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPREDIV(&self) -> super::vals::PLL0CTRL_BYPASSPREDIV {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PLL0CTRL_BYPASSPREDIV::from_bits(val as u8)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub const fn set_BYPASSPREDIV(&mut self, val: super::vals::PLL0CTRL_BYPASSPREDIV) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "bypass of the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPOSTDIV(&self) -> super::vals::PLL0CTRL_BYPASSPOSTDIV {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PLL0CTRL_BYPASSPOSTDIV::from_bits(val as u8)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub const fn set_BYPASSPOSTDIV(&mut self, val: super::vals::PLL0CTRL_BYPASSPOSTDIV) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "enable the output clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable the output clock."]
    #[inline(always)]
    pub const fn set_CLKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "free running mode."]
    #[must_use]
    #[inline(always)]
    pub const fn FRMEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "free running mode."]
    #[inline(always)]
    pub const fn set_FRMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn FRMCLKSTABLE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub const fn set_FRMCLKSTABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "skew mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SKEWEN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "skew mode."]
    #[inline(always)]
    pub const fn set_SKEWEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for PLL0CTRL {
    #[inline(always)]
    fn default() -> PLL0CTRL {
        PLL0CTRL(0)
    }
}
impl core::fmt::Debug for PLL0CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0CTRL")
            .field("SELR", &self.SELR())
            .field("SELI", &self.SELI())
            .field("SELP", &self.SELP())
            .field("BYPASSPLL", &self.BYPASSPLL())
            .field("BYPASSPOSTDIV2", &self.BYPASSPOSTDIV2())
            .field("LIMUPOFF", &self.LIMUPOFF())
            .field("BWDIRECT", &self.BWDIRECT())
            .field("BYPASSPREDIV", &self.BYPASSPREDIV())
            .field("BYPASSPOSTDIV", &self.BYPASSPOSTDIV())
            .field("CLKEN", &self.CLKEN())
            .field("FRMEN", &self.FRMEN())
            .field("FRMCLKSTABLE", &self.FRMCLKSTABLE())
            .field("SKEWEN", &self.SKEWEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL0CTRL {{ SELR: {=u8:?}, SELI: {=u8:?}, SELP: {=u8:?}, BYPASSPLL: {:?}, BYPASSPOSTDIV2: {:?}, LIMUPOFF: {=bool:?}, BWDIRECT: {:?}, BYPASSPREDIV: {:?}, BYPASSPOSTDIV: {:?}, CLKEN: {=bool:?}, FRMEN: {=bool:?}, FRMCLKSTABLE: {=bool:?}, SKEWEN: {=bool:?} }}",
            self.SELR(),
            self.SELI(),
            self.SELP(),
            self.BYPASSPLL(),
            self.BYPASSPOSTDIV2(),
            self.LIMUPOFF(),
            self.BWDIRECT(),
            self.BYPASSPREDIV(),
            self.BYPASSPOSTDIV(),
            self.CLKEN(),
            self.FRMEN(),
            self.FRMCLKSTABLE(),
            self.SKEWEN()
        )
    }
}
#[doc = "PLL0 550m N divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0NDEC(pub u32);
impl PLL0NDEC {
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn NDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_NDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pre-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn NREQ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change request."]
    #[inline(always)]
    pub const fn set_NREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for PLL0NDEC {
    #[inline(always)]
    fn default() -> PLL0NDEC {
        PLL0NDEC(0)
    }
}
impl core::fmt::Debug for PLL0NDEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0NDEC")
            .field("NDIV", &self.NDIV())
            .field("NREQ", &self.NREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0NDEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL0NDEC {{ NDIV: {=u8:?}, NREQ: {=bool:?} }}",
            self.NDIV(),
            self.NREQ()
        )
    }
}
#[doc = "PLL0 550m P divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0PDEC(pub u32);
impl PLL0PDEC {
    #[doc = "post-divider divider ratio (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn PDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "post-divider divider ratio (P-divider)."]
    #[inline(always)]
    pub const fn set_PDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn PREQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "feedback ratio change request."]
    #[inline(always)]
    pub const fn set_PREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for PLL0PDEC {
    #[inline(always)]
    fn default() -> PLL0PDEC {
        PLL0PDEC(0)
    }
}
impl core::fmt::Debug for PLL0PDEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0PDEC")
            .field("PDIV", &self.PDIV())
            .field("PREQ", &self.PREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0PDEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL0PDEC {{ PDIV: {=u8:?}, PREQ: {=bool:?} }}",
            self.PDIV(),
            self.PREQ()
        )
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0SSCG0(pub u32);
impl PLL0SSCG0 {
    #[doc = "input word of the wrapper bit 31 to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn MD_LBS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub const fn set_MD_LBS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PLL0SSCG0 {
    #[inline(always)]
    fn default() -> PLL0SSCG0 {
        PLL0SSCG0(0)
    }
}
impl core::fmt::Debug for PLL0SSCG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0SSCG0")
            .field("MD_LBS", &self.MD_LBS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0SSCG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PLL0SSCG0 {{ MD_LBS: {=u32:?} }}", self.MD_LBS())
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0SSCG1(pub u32);
impl PLL0SSCG1 {
    #[doc = "input word of the wrapper bit 32."]
    #[must_use]
    #[inline(always)]
    pub const fn MD_MBS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "input word of the wrapper bit 32."]
    #[inline(always)]
    pub const fn set_MD_MBS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "md change request."]
    #[must_use]
    #[inline(always)]
    pub const fn MD_REQ(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "md change request."]
    #[inline(always)]
    pub const fn set_MD_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[must_use]
    #[inline(always)]
    pub const fn MF(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub const fn set_MF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[must_use]
    #[inline(always)]
    pub const fn MR(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[inline(always)]
    pub const fn set_MR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[must_use]
    #[inline(always)]
    pub const fn MC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub const fn set_MC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "to select an external mdiv value."]
    #[must_use]
    #[inline(always)]
    pub const fn MDIV_EXT(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0xffff;
        val as u16
    }
    #[doc = "to select an external mdiv value."]
    #[inline(always)]
    pub const fn set_MDIV_EXT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 10usize)) | (((val as u32) & 0xffff) << 10usize);
    }
    #[doc = "to select an external mreq value."]
    #[must_use]
    #[inline(always)]
    pub const fn MREQ(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "to select an external mreq value."]
    #[inline(always)]
    pub const fn set_MREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[must_use]
    #[inline(always)]
    pub const fn DITHER(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub const fn set_DITHER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL_EXT(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub const fn set_SEL_EXT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PLL0SSCG1 {
    #[inline(always)]
    fn default() -> PLL0SSCG1 {
        PLL0SSCG1(0)
    }
}
impl core::fmt::Debug for PLL0SSCG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0SSCG1")
            .field("MD_MBS", &self.MD_MBS())
            .field("MD_REQ", &self.MD_REQ())
            .field("MF", &self.MF())
            .field("MR", &self.MR())
            .field("MC", &self.MC())
            .field("MDIV_EXT", &self.MDIV_EXT())
            .field("MREQ", &self.MREQ())
            .field("DITHER", &self.DITHER())
            .field("SEL_EXT", &self.SEL_EXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0SSCG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL0SSCG1 {{ MD_MBS: {=bool:?}, MD_REQ: {=bool:?}, MF: {=u8:?}, MR: {=u8:?}, MC: {=u8:?}, MDIV_EXT: {=u16:?}, MREQ: {=bool:?}, DITHER: {=bool:?}, SEL_EXT: {=bool:?} }}",
            self.MD_MBS(),
            self.MD_REQ(),
            self.MF(),
            self.MR(),
            self.MC(),
            self.MDIV_EXT(),
            self.MREQ(),
            self.DITHER(),
            self.SEL_EXT()
        )
    }
}
#[doc = "PLL0 550m status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL0STAT(pub u32);
impl PLL0STAT {
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn PREDIVACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_PREDIVACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn FEEDDIVACK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_FEEDDIVACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIVACK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_POSTDIVACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "free running detector output (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn FRMDET(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "free running detector output (active high)."]
    #[inline(always)]
    pub const fn set_FRMDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for PLL0STAT {
    #[inline(always)]
    fn default() -> PLL0STAT {
        PLL0STAT(0)
    }
}
impl core::fmt::Debug for PLL0STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0STAT")
            .field("LOCK", &self.LOCK())
            .field("PREDIVACK", &self.PREDIVACK())
            .field("FEEDDIVACK", &self.FEEDDIVACK())
            .field("POSTDIVACK", &self.POSTDIVACK())
            .field("FRMDET", &self.FRMDET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL0STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL0STAT {{ LOCK: {=bool:?}, PREDIVACK: {=bool:?}, FEEDDIVACK: {=bool:?}, POSTDIVACK: {=bool:?}, FRMDET: {=bool:?} }}",
            self.LOCK(),
            self.PREDIVACK(),
            self.FEEDDIVACK(),
            self.POSTDIVACK(),
            self.FRMDET()
        )
    }
}
#[doc = "PLL1 clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL1CLKSEL(pub u32);
impl PLL1CLKSEL {
    #[doc = "PLL1 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::PLL1CLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::PLL1CLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "PLL1 clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::PLL1CLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for PLL1CLKSEL {
    #[inline(always)]
    fn default() -> PLL1CLKSEL {
        PLL1CLKSEL(0)
    }
}
impl core::fmt::Debug for PLL1CLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL1CLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PLL1CLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "PLL1 550m control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL1CTRL(pub u32);
impl PLL1CTRL {
    #[doc = "Bandwidth select R value."]
    #[must_use]
    #[inline(always)]
    pub const fn SELR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R value."]
    #[inline(always)]
    pub const fn set_SELR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I value."]
    #[must_use]
    #[inline(always)]
    pub const fn SELI(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I value."]
    #[inline(always)]
    pub const fn set_SELI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P value."]
    #[must_use]
    #[inline(always)]
    pub const fn SELP(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P value."]
    #[inline(always)]
    pub const fn set_SELP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPLL(&self) -> super::vals::PLL1CTRL_BYPASSPLL {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PLL1CTRL_BYPASSPLL::from_bits(val as u8)
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub const fn set_BYPASSPLL(&mut self, val: super::vals::PLL1CTRL_BYPASSPLL) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPOSTDIV2(&self) -> super::vals::PLL1CTRL_BYPASSPOSTDIV2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PLL1CTRL_BYPASSPOSTDIV2::from_bits(val as u8)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub const fn set_BYPASSPOSTDIV2(&mut self, val: super::vals::PLL1CTRL_BYPASSPOSTDIV2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMUPOFF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub const fn set_LIMUPOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn BWDIRECT(&self) -> super::vals::PLL1CTRL_BWDIRECT {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PLL1CTRL_BWDIRECT::from_bits(val as u8)
    }
    #[doc = "control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_BWDIRECT(&mut self, val: super::vals::PLL1CTRL_BWDIRECT) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "bypass of the pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPREDIV(&self) -> super::vals::PLL1CTRL_BYPASSPREDIV {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PLL1CTRL_BYPASSPREDIV::from_bits(val as u8)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub const fn set_BYPASSPREDIV(&mut self, val: super::vals::PLL1CTRL_BYPASSPREDIV) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "bypass of the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASSPOSTDIV(&self) -> super::vals::PLL1CTRL_BYPASSPOSTDIV {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PLL1CTRL_BYPASSPOSTDIV::from_bits(val as u8)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub const fn set_BYPASSPOSTDIV(&mut self, val: super::vals::PLL1CTRL_BYPASSPOSTDIV) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "enable the output clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable the output clock."]
    #[inline(always)]
    pub const fn set_CLKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "1: free running mode."]
    #[must_use]
    #[inline(always)]
    pub const fn FRMEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "1: free running mode."]
    #[inline(always)]
    pub const fn set_FRMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn FRMCLKSTABLE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub const fn set_FRMCLKSTABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Skew mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SKEWEN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Skew mode."]
    #[inline(always)]
    pub const fn set_SKEWEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for PLL1CTRL {
    #[inline(always)]
    fn default() -> PLL1CTRL {
        PLL1CTRL(0)
    }
}
impl core::fmt::Debug for PLL1CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CTRL")
            .field("SELR", &self.SELR())
            .field("SELI", &self.SELI())
            .field("SELP", &self.SELP())
            .field("BYPASSPLL", &self.BYPASSPLL())
            .field("BYPASSPOSTDIV2", &self.BYPASSPOSTDIV2())
            .field("LIMUPOFF", &self.LIMUPOFF())
            .field("BWDIRECT", &self.BWDIRECT())
            .field("BYPASSPREDIV", &self.BYPASSPREDIV())
            .field("BYPASSPOSTDIV", &self.BYPASSPOSTDIV())
            .field("CLKEN", &self.CLKEN())
            .field("FRMEN", &self.FRMEN())
            .field("FRMCLKSTABLE", &self.FRMCLKSTABLE())
            .field("SKEWEN", &self.SKEWEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL1CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL1CTRL {{ SELR: {=u8:?}, SELI: {=u8:?}, SELP: {=u8:?}, BYPASSPLL: {:?}, BYPASSPOSTDIV2: {:?}, LIMUPOFF: {=bool:?}, BWDIRECT: {:?}, BYPASSPREDIV: {:?}, BYPASSPOSTDIV: {:?}, CLKEN: {=bool:?}, FRMEN: {=bool:?}, FRMCLKSTABLE: {=bool:?}, SKEWEN: {=bool:?} }}",
            self.SELR(),
            self.SELI(),
            self.SELP(),
            self.BYPASSPLL(),
            self.BYPASSPOSTDIV2(),
            self.LIMUPOFF(),
            self.BWDIRECT(),
            self.BYPASSPREDIV(),
            self.BYPASSPOSTDIV(),
            self.CLKEN(),
            self.FRMEN(),
            self.FRMCLKSTABLE(),
            self.SKEWEN()
        )
    }
}
#[doc = "PLL1 550m M divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL1MDEC(pub u32);
impl PLL1MDEC {
    #[doc = "feedback divider divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn MDIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_MDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn MREQ(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "feedback ratio change request."]
    #[inline(always)]
    pub const fn set_MREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for PLL1MDEC {
    #[inline(always)]
    fn default() -> PLL1MDEC {
        PLL1MDEC(0)
    }
}
impl core::fmt::Debug for PLL1MDEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1MDEC")
            .field("MDIV", &self.MDIV())
            .field("MREQ", &self.MREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL1MDEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL1MDEC {{ MDIV: {=u16:?}, MREQ: {=bool:?} }}",
            self.MDIV(),
            self.MREQ()
        )
    }
}
#[doc = "PLL1 550m N divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL1NDEC(pub u32);
impl PLL1NDEC {
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn NDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_NDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pre-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn NREQ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change request."]
    #[inline(always)]
    pub const fn set_NREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for PLL1NDEC {
    #[inline(always)]
    fn default() -> PLL1NDEC {
        PLL1NDEC(0)
    }
}
impl core::fmt::Debug for PLL1NDEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1NDEC")
            .field("NDIV", &self.NDIV())
            .field("NREQ", &self.NREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL1NDEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL1NDEC {{ NDIV: {=u8:?}, NREQ: {=bool:?} }}",
            self.NDIV(),
            self.NREQ()
        )
    }
}
#[doc = "PLL1 550m P divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL1PDEC(pub u32);
impl PLL1PDEC {
    #[doc = "post-divider divider ratio (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn PDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "post-divider divider ratio (P-divider)."]
    #[inline(always)]
    pub const fn set_PDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn PREQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "feedback ratio change request."]
    #[inline(always)]
    pub const fn set_PREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for PLL1PDEC {
    #[inline(always)]
    fn default() -> PLL1PDEC {
        PLL1PDEC(0)
    }
}
impl core::fmt::Debug for PLL1PDEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1PDEC")
            .field("PDIV", &self.PDIV())
            .field("PREQ", &self.PREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL1PDEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL1PDEC {{ PDIV: {=u8:?}, PREQ: {=bool:?} }}",
            self.PDIV(),
            self.PREQ()
        )
    }
}
#[doc = "PLL1 550m status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL1STAT(pub u32);
impl PLL1STAT {
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn PREDIVACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_PREDIVACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn FEEDDIVACK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_FEEDDIVACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIVACK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_POSTDIVACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "free running detector output (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn FRMDET(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "free running detector output (active high)."]
    #[inline(always)]
    pub const fn set_FRMDET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for PLL1STAT {
    #[inline(always)]
    fn default() -> PLL1STAT {
        PLL1STAT(0)
    }
}
impl core::fmt::Debug for PLL1STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1STAT")
            .field("LOCK", &self.LOCK())
            .field("PREDIVACK", &self.PREDIVACK())
            .field("FEEDDIVACK", &self.FEEDDIVACK())
            .field("POSTDIVACK", &self.POSTDIVACK())
            .field("FRMDET", &self.FRMDET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL1STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL1STAT {{ LOCK: {=bool:?}, PREDIVACK: {=bool:?}, FEEDDIVACK: {=bool:?}, POSTDIVACK: {=bool:?}, FRMDET: {=bool:?} }}",
            self.LOCK(),
            self.PREDIVACK(),
            self.FEEDDIVACK(),
            self.POSTDIVACK(),
            self.FRMDET()
        )
    }
}
#[doc = "Peripheral reset control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRL0(pub u32);
impl PRESETCTRL0 {
    #[doc = "ROM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM_RST(&self) -> super::vals::ROM_RST {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ROM_RST::from_bits(val as u8)
    }
    #[doc = "ROM reset control."]
    #[inline(always)]
    pub const fn set_ROM_RST(&mut self, val: super::vals::ROM_RST) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SRAM Controller 1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_CTRL1_RST(&self) -> super::vals::SRAM_CTRL1_RST {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SRAM_CTRL1_RST::from_bits(val as u8)
    }
    #[doc = "SRAM Controller 1 reset control."]
    #[inline(always)]
    pub const fn set_SRAM_CTRL1_RST(&mut self, val: super::vals::SRAM_CTRL1_RST) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SRAM Controller 2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_CTRL2_RST(&self) -> super::vals::SRAM_CTRL2_RST {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SRAM_CTRL2_RST::from_bits(val as u8)
    }
    #[doc = "SRAM Controller 2 reset control."]
    #[inline(always)]
    pub const fn set_SRAM_CTRL2_RST(&mut self, val: super::vals::SRAM_CTRL2_RST) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Flash controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_RST(&self) -> super::vals::FLASH_RST {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::FLASH_RST::from_bits(val as u8)
    }
    #[doc = "Flash controller reset control."]
    #[inline(always)]
    pub const fn set_FLASH_RST(&mut self, val: super::vals::FLASH_RST) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FMC controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FMC_RST(&self) -> super::vals::FMC_RST {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FMC_RST::from_bits(val as u8)
    }
    #[doc = "FMC controller reset control."]
    #[inline(always)]
    pub const fn set_FMC_RST(&mut self, val: super::vals::FMC_RST) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Input Mux reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn MUX_RST(&self) -> super::vals::MUX_RST {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MUX_RST::from_bits(val as u8)
    }
    #[doc = "Input Mux reset control."]
    #[inline(always)]
    pub const fn set_MUX_RST(&mut self, val: super::vals::MUX_RST) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "I/O controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCON_RST(&self) -> super::vals::IOCON_RST {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::IOCON_RST::from_bits(val as u8)
    }
    #[doc = "I/O controller reset control."]
    #[inline(always)]
    pub const fn set_IOCON_RST(&mut self, val: super::vals::IOCON_RST) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO0_RST(&self) -> super::vals::GPIO0_RST {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::GPIO0_RST::from_bits(val as u8)
    }
    #[doc = "GPIO0 reset control."]
    #[inline(always)]
    pub const fn set_GPIO0_RST(&mut self, val: super::vals::GPIO0_RST) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO1_RST(&self) -> super::vals::GPIO1_RST {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::GPIO1_RST::from_bits(val as u8)
    }
    #[doc = "GPIO1 reset control."]
    #[inline(always)]
    pub const fn set_GPIO1_RST(&mut self, val: super::vals::GPIO1_RST) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Pin interrupt (PINT) reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn PINT_RST(&self) -> super::vals::PINT_RST {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PINT_RST::from_bits(val as u8)
    }
    #[doc = "Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub const fn set_PINT_RST(&mut self, val: super::vals::PINT_RST) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Group interrupt (GINT) reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn GINT_RST(&self) -> super::vals::GINT_RST {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::GINT_RST::from_bits(val as u8)
    }
    #[doc = "Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub const fn set_GINT_RST(&mut self, val: super::vals::GINT_RST) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA0_RST(&self) -> super::vals::DMA0_RST {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DMA0_RST::from_bits(val as u8)
    }
    #[doc = "DMA0 reset control."]
    #[inline(always)]
    pub const fn set_DMA0_RST(&mut self, val: super::vals::DMA0_RST) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "CRCGEN reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn CRCGEN_RST(&self) -> super::vals::CRCGEN_RST {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::CRCGEN_RST::from_bits(val as u8)
    }
    #[doc = "CRCGEN reset control."]
    #[inline(always)]
    pub const fn set_CRCGEN_RST(&mut self, val: super::vals::CRCGEN_RST) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Watchdog Timer reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn WWDT_RST(&self) -> super::vals::WWDT_RST {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::WWDT_RST::from_bits(val as u8)
    }
    #[doc = "Watchdog Timer reset control."]
    #[inline(always)]
    pub const fn set_WWDT_RST(&mut self, val: super::vals::WWDT_RST) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Real Time Clock (RTC) reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_RST(&self) -> super::vals::RTC_RST {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RTC_RST::from_bits(val as u8)
    }
    #[doc = "Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub const fn set_RTC_RST(&mut self, val: super::vals::RTC_RST) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Inter CPU communication Mailbox reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn MAILBOX_RST(&self) -> super::vals::MAILBOX_RST {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MAILBOX_RST::from_bits(val as u8)
    }
    #[doc = "Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub const fn set_MAILBOX_RST(&mut self, val: super::vals::MAILBOX_RST) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_RST(&self) -> super::vals::ADC_RST {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ADC_RST::from_bits(val as u8)
    }
    #[doc = "ADC reset control."]
    #[inline(always)]
    pub const fn set_ADC_RST(&mut self, val: super::vals::ADC_RST) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for PRESETCTRL0 {
    #[inline(always)]
    fn default() -> PRESETCTRL0 {
        PRESETCTRL0(0)
    }
}
impl core::fmt::Debug for PRESETCTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRL0")
            .field("ROM_RST", &self.ROM_RST())
            .field("SRAM_CTRL1_RST", &self.SRAM_CTRL1_RST())
            .field("SRAM_CTRL2_RST", &self.SRAM_CTRL2_RST())
            .field("FLASH_RST", &self.FLASH_RST())
            .field("FMC_RST", &self.FMC_RST())
            .field("MUX_RST", &self.MUX_RST())
            .field("IOCON_RST", &self.IOCON_RST())
            .field("GPIO0_RST", &self.GPIO0_RST())
            .field("GPIO1_RST", &self.GPIO1_RST())
            .field("PINT_RST", &self.PINT_RST())
            .field("GINT_RST", &self.GINT_RST())
            .field("DMA0_RST", &self.DMA0_RST())
            .field("CRCGEN_RST", &self.CRCGEN_RST())
            .field("WWDT_RST", &self.WWDT_RST())
            .field("RTC_RST", &self.RTC_RST())
            .field("MAILBOX_RST", &self.MAILBOX_RST())
            .field("ADC_RST", &self.ADC_RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRESETCTRL0 {{ ROM_RST: {:?}, SRAM_CTRL1_RST: {:?}, SRAM_CTRL2_RST: {:?}, FLASH_RST: {:?}, FMC_RST: {:?}, MUX_RST: {:?}, IOCON_RST: {:?}, GPIO0_RST: {:?}, GPIO1_RST: {:?}, PINT_RST: {:?}, GINT_RST: {:?}, DMA0_RST: {:?}, CRCGEN_RST: {:?}, WWDT_RST: {:?}, RTC_RST: {:?}, MAILBOX_RST: {:?}, ADC_RST: {:?} }}",
            self.ROM_RST(),
            self.SRAM_CTRL1_RST(),
            self.SRAM_CTRL2_RST(),
            self.FLASH_RST(),
            self.FMC_RST(),
            self.MUX_RST(),
            self.IOCON_RST(),
            self.GPIO0_RST(),
            self.GPIO1_RST(),
            self.PINT_RST(),
            self.GINT_RST(),
            self.DMA0_RST(),
            self.CRCGEN_RST(),
            self.WWDT_RST(),
            self.RTC_RST(),
            self.MAILBOX_RST(),
            self.ADC_RST()
        )
    }
}
#[doc = "Peripheral reset control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRL1(pub u32);
impl PRESETCTRL1 {
    #[doc = "MRT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn MRT_RST(&self) -> super::vals::MRT_RST {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MRT_RST::from_bits(val as u8)
    }
    #[doc = "MRT reset control."]
    #[inline(always)]
    pub const fn set_MRT_RST(&mut self, val: super::vals::MRT_RST) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "OS Event Timer reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn OSTIMER_RST(&self) -> super::vals::OSTIMER_RST {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OSTIMER_RST::from_bits(val as u8)
    }
    #[doc = "OS Event Timer reset control."]
    #[inline(always)]
    pub const fn set_OSTIMER_RST(&mut self, val: super::vals::OSTIMER_RST) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SCT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn SCT_RST(&self) -> super::vals::SCT_RST {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SCT_RST::from_bits(val as u8)
    }
    #[doc = "SCT reset control."]
    #[inline(always)]
    pub const fn set_SCT_RST(&mut self, val: super::vals::SCT_RST) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CAN reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn CAN_RST(&self) -> super::vals::CAN_RST {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CAN_RST::from_bits(val as u8)
    }
    #[doc = "CAN reset control."]
    #[inline(always)]
    pub const fn set_CAN_RST(&mut self, val: super::vals::CAN_RST) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "UTICK reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn UTICK_RST(&self) -> super::vals::UTICK_RST {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::UTICK_RST::from_bits(val as u8)
    }
    #[doc = "UTICK reset control."]
    #[inline(always)]
    pub const fn set_UTICK_RST(&mut self, val: super::vals::UTICK_RST) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FC0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC0_RST(&self) -> super::vals::FC0_RST {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FC0_RST::from_bits(val as u8)
    }
    #[doc = "FC0 reset control."]
    #[inline(always)]
    pub const fn set_FC0_RST(&mut self, val: super::vals::FC0_RST) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FC1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC1_RST(&self) -> super::vals::FC1_RST {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::FC1_RST::from_bits(val as u8)
    }
    #[doc = "FC1 reset control."]
    #[inline(always)]
    pub const fn set_FC1_RST(&mut self, val: super::vals::FC1_RST) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FC2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC2_RST(&self) -> super::vals::FC2_RST {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::FC2_RST::from_bits(val as u8)
    }
    #[doc = "FC2 reset control."]
    #[inline(always)]
    pub const fn set_FC2_RST(&mut self, val: super::vals::FC2_RST) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FC3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC3_RST(&self) -> super::vals::FC3_RST {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::FC3_RST::from_bits(val as u8)
    }
    #[doc = "FC3 reset control."]
    #[inline(always)]
    pub const fn set_FC3_RST(&mut self, val: super::vals::FC3_RST) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FC4 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC4_RST(&self) -> super::vals::FC4_RST {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::FC4_RST::from_bits(val as u8)
    }
    #[doc = "FC4 reset control."]
    #[inline(always)]
    pub const fn set_FC4_RST(&mut self, val: super::vals::FC4_RST) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "FC5 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC5_RST(&self) -> super::vals::FC5_RST {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::FC5_RST::from_bits(val as u8)
    }
    #[doc = "FC5 reset control."]
    #[inline(always)]
    pub const fn set_FC5_RST(&mut self, val: super::vals::FC5_RST) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "FC6 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC6_RST(&self) -> super::vals::FC6_RST {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::FC6_RST::from_bits(val as u8)
    }
    #[doc = "FC6 reset control."]
    #[inline(always)]
    pub const fn set_FC6_RST(&mut self, val: super::vals::FC6_RST) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "FC7 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FC7_RST(&self) -> super::vals::FC7_RST {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::FC7_RST::from_bits(val as u8)
    }
    #[doc = "FC7 reset control."]
    #[inline(always)]
    pub const fn set_FC7_RST(&mut self, val: super::vals::FC7_RST) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Timer 2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER2_RST(&self) -> super::vals::TIMER2_RST {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::TIMER2_RST::from_bits(val as u8)
    }
    #[doc = "Timer 2 reset control."]
    #[inline(always)]
    pub const fn set_TIMER2_RST(&mut self, val: super::vals::TIMER2_RST) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB0-FS DEV reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_DEV_RST(&self) -> super::vals::USB0_DEV_RST {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::USB0_DEV_RST::from_bits(val as u8)
    }
    #[doc = "USB0-FS DEV reset control."]
    #[inline(always)]
    pub const fn set_USB0_DEV_RST(&mut self, val: super::vals::USB0_DEV_RST) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Timer 0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_RST(&self) -> super::vals::TIMER0_RST {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::TIMER0_RST::from_bits(val as u8)
    }
    #[doc = "Timer 0 reset control."]
    #[inline(always)]
    pub const fn set_TIMER0_RST(&mut self, val: super::vals::TIMER0_RST) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Timer 1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_RST(&self) -> super::vals::TIMER1_RST {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::TIMER1_RST::from_bits(val as u8)
    }
    #[doc = "Timer 1 reset control."]
    #[inline(always)]
    pub const fn set_TIMER1_RST(&mut self, val: super::vals::TIMER1_RST) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for PRESETCTRL1 {
    #[inline(always)]
    fn default() -> PRESETCTRL1 {
        PRESETCTRL1(0)
    }
}
impl core::fmt::Debug for PRESETCTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRL1")
            .field("MRT_RST", &self.MRT_RST())
            .field("OSTIMER_RST", &self.OSTIMER_RST())
            .field("SCT_RST", &self.SCT_RST())
            .field("CAN_RST", &self.CAN_RST())
            .field("UTICK_RST", &self.UTICK_RST())
            .field("FC0_RST", &self.FC0_RST())
            .field("FC1_RST", &self.FC1_RST())
            .field("FC2_RST", &self.FC2_RST())
            .field("FC3_RST", &self.FC3_RST())
            .field("FC4_RST", &self.FC4_RST())
            .field("FC5_RST", &self.FC5_RST())
            .field("FC6_RST", &self.FC6_RST())
            .field("FC7_RST", &self.FC7_RST())
            .field("TIMER2_RST", &self.TIMER2_RST())
            .field("USB0_DEV_RST", &self.USB0_DEV_RST())
            .field("TIMER0_RST", &self.TIMER0_RST())
            .field("TIMER1_RST", &self.TIMER1_RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRESETCTRL1 {{ MRT_RST: {:?}, OSTIMER_RST: {:?}, SCT_RST: {:?}, CAN_RST: {:?}, UTICK_RST: {:?}, FC0_RST: {:?}, FC1_RST: {:?}, FC2_RST: {:?}, FC3_RST: {:?}, FC4_RST: {:?}, FC5_RST: {:?}, FC6_RST: {:?}, FC7_RST: {:?}, TIMER2_RST: {:?}, USB0_DEV_RST: {:?}, TIMER0_RST: {:?}, TIMER1_RST: {:?} }}",
            self.MRT_RST(),
            self.OSTIMER_RST(),
            self.SCT_RST(),
            self.CAN_RST(),
            self.UTICK_RST(),
            self.FC0_RST(),
            self.FC1_RST(),
            self.FC2_RST(),
            self.FC3_RST(),
            self.FC4_RST(),
            self.FC5_RST(),
            self.FC6_RST(),
            self.FC7_RST(),
            self.TIMER2_RST(),
            self.USB0_DEV_RST(),
            self.TIMER0_RST(),
            self.TIMER1_RST()
        )
    }
}
#[doc = "Peripheral reset control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRL2(pub u32);
impl PRESETCTRL2 {
    #[doc = "DMA1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA1_RST(&self) -> super::vals::DMA1_RST {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DMA1_RST::from_bits(val as u8)
    }
    #[doc = "DMA1 reset control."]
    #[inline(always)]
    pub const fn set_DMA1_RST(&mut self, val: super::vals::DMA1_RST) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP_RST(&self) -> super::vals::COMP_RST {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::COMP_RST::from_bits(val as u8)
    }
    #[doc = "Comparator reset control."]
    #[inline(always)]
    pub const fn set_COMP_RST(&mut self, val: super::vals::COMP_RST) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "USB1-HS Host reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_HOST_RST(&self) -> super::vals::USB1_HOST_RST {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::USB1_HOST_RST::from_bits(val as u8)
    }
    #[doc = "USB1-HS Host reset control."]
    #[inline(always)]
    pub const fn set_USB1_HOST_RST(&mut self, val: super::vals::USB1_HOST_RST) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "USB1-HS dev reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_DEV_RST(&self) -> super::vals::USB1_DEV_RST {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::USB1_DEV_RST::from_bits(val as u8)
    }
    #[doc = "USB1-HS dev reset control."]
    #[inline(always)]
    pub const fn set_USB1_DEV_RST(&mut self, val: super::vals::USB1_DEV_RST) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "USB1-HS RAM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_RAM_RST(&self) -> super::vals::USB1_RAM_RST {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::USB1_RAM_RST::from_bits(val as u8)
    }
    #[doc = "USB1-HS RAM reset control."]
    #[inline(always)]
    pub const fn set_USB1_RAM_RST(&mut self, val: super::vals::USB1_RAM_RST) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "USB1-HS PHY reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB1_PHY_RST(&self) -> super::vals::USB1_PHY_RST {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::USB1_PHY_RST::from_bits(val as u8)
    }
    #[doc = "USB1-HS PHY reset control."]
    #[inline(always)]
    pub const fn set_USB1_PHY_RST(&mut self, val: super::vals::USB1_PHY_RST) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Frequency meter reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn FREQME_RST(&self) -> super::vals::FREQME_RST {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FREQME_RST::from_bits(val as u8)
    }
    #[doc = "Frequency meter reset control."]
    #[inline(always)]
    pub const fn set_FREQME_RST(&mut self, val: super::vals::FREQME_RST) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Code Watchdog reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn CDOG_RST(&self) -> super::vals::CDOG_RST {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::CDOG_RST::from_bits(val as u8)
    }
    #[doc = "Code Watchdog reset control."]
    #[inline(always)]
    pub const fn set_CDOG_RST(&mut self, val: super::vals::CDOG_RST) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn RNG_RST(&self) -> super::vals::RNG_RST {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::RNG_RST::from_bits(val as u8)
    }
    #[doc = "RNG reset control."]
    #[inline(always)]
    pub const fn set_RNG_RST(&mut self, val: super::vals::RNG_RST) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "SYSCTL Block reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCTL_RST(&self) -> super::vals::SYSCTL_RST {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SYSCTL_RST::from_bits(val as u8)
    }
    #[doc = "SYSCTL Block reset."]
    #[inline(always)]
    pub const fn set_SYSCTL_RST(&mut self, val: super::vals::SYSCTL_RST) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USB0-FS Host Master reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_HOSTM_RST(&self) -> super::vals::USB0_HOSTM_RST {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::USB0_HOSTM_RST::from_bits(val as u8)
    }
    #[doc = "USB0-FS Host Master reset control."]
    #[inline(always)]
    pub const fn set_USB0_HOSTM_RST(&mut self, val: super::vals::USB0_HOSTM_RST) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB0-FS Host Slave reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn USB0_HOSTS_RST(&self) -> super::vals::USB0_HOSTS_RST {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::USB0_HOSTS_RST::from_bits(val as u8)
    }
    #[doc = "USB0-FS Host Slave reset control."]
    #[inline(always)]
    pub const fn set_USB0_HOSTS_RST(&mut self, val: super::vals::USB0_HOSTS_RST) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "HASH_AES reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn HASH_AES_RST(&self) -> super::vals::HASH_AES_RST {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HASH_AES_RST::from_bits(val as u8)
    }
    #[doc = "HASH_AES reset control."]
    #[inline(always)]
    pub const fn set_HASH_AES_RST(&mut self, val: super::vals::HASH_AES_RST) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "PLU LUT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn PLULUT_RST(&self) -> super::vals::PLULUT_RST {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PLULUT_RST::from_bits(val as u8)
    }
    #[doc = "PLU LUT reset control."]
    #[inline(always)]
    pub const fn set_PLULUT_RST(&mut self, val: super::vals::PLULUT_RST) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Timer 3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER3_RST(&self) -> super::vals::TIMER3_RST {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::TIMER3_RST::from_bits(val as u8)
    }
    #[doc = "Timer 3 reset control."]
    #[inline(always)]
    pub const fn set_TIMER3_RST(&mut self, val: super::vals::TIMER3_RST) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Timer 4 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER4_RST(&self) -> super::vals::TIMER4_RST {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::TIMER4_RST::from_bits(val as u8)
    }
    #[doc = "Timer 4 reset control."]
    #[inline(always)]
    pub const fn set_TIMER4_RST(&mut self, val: super::vals::TIMER4_RST) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "PUF reset control reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn PUF_RST(&self) -> super::vals::PUF_RST {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PUF_RST::from_bits(val as u8)
    }
    #[doc = "PUF reset control reset control."]
    #[inline(always)]
    pub const fn set_PUF_RST(&mut self, val: super::vals::PUF_RST) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Casper reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn CASPER_RST(&self) -> super::vals::CASPER_RST {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::CASPER_RST::from_bits(val as u8)
    }
    #[doc = "Casper reset control."]
    #[inline(always)]
    pub const fn set_CASPER_RST(&mut self, val: super::vals::CASPER_RST) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "analog control reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn ANALOG_CTRL_RST(&self) -> super::vals::ANALOG_CTRL_RST {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::ANALOG_CTRL_RST::from_bits(val as u8)
    }
    #[doc = "analog control reset control."]
    #[inline(always)]
    pub const fn set_ANALOG_CTRL_RST(&mut self, val: super::vals::ANALOG_CTRL_RST) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "HS LSPI reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn HS_LSPI_RST(&self) -> super::vals::HS_LSPI_RST {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::HS_LSPI_RST::from_bits(val as u8)
    }
    #[doc = "HS LSPI reset control."]
    #[inline(always)]
    pub const fn set_HS_LSPI_RST(&mut self, val: super::vals::HS_LSPI_RST) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO secure reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_SEC_RST(&self) -> super::vals::GPIO_SEC_RST {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::GPIO_SEC_RST::from_bits(val as u8)
    }
    #[doc = "GPIO secure reset control."]
    #[inline(always)]
    pub const fn set_GPIO_SEC_RST(&mut self, val: super::vals::GPIO_SEC_RST) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "GPIO secure int reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_SEC_INT_RST(&self) -> super::vals::GPIO_SEC_INT_RST {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GPIO_SEC_INT_RST::from_bits(val as u8)
    }
    #[doc = "GPIO secure int reset control."]
    #[inline(always)]
    pub const fn set_GPIO_SEC_INT_RST(&mut self, val: super::vals::GPIO_SEC_INT_RST) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for PRESETCTRL2 {
    #[inline(always)]
    fn default() -> PRESETCTRL2 {
        PRESETCTRL2(0)
    }
}
impl core::fmt::Debug for PRESETCTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRL2")
            .field("DMA1_RST", &self.DMA1_RST())
            .field("COMP_RST", &self.COMP_RST())
            .field("USB1_HOST_RST", &self.USB1_HOST_RST())
            .field("USB1_DEV_RST", &self.USB1_DEV_RST())
            .field("USB1_RAM_RST", &self.USB1_RAM_RST())
            .field("USB1_PHY_RST", &self.USB1_PHY_RST())
            .field("FREQME_RST", &self.FREQME_RST())
            .field("CDOG_RST", &self.CDOG_RST())
            .field("RNG_RST", &self.RNG_RST())
            .field("SYSCTL_RST", &self.SYSCTL_RST())
            .field("USB0_HOSTM_RST", &self.USB0_HOSTM_RST())
            .field("USB0_HOSTS_RST", &self.USB0_HOSTS_RST())
            .field("HASH_AES_RST", &self.HASH_AES_RST())
            .field("PLULUT_RST", &self.PLULUT_RST())
            .field("TIMER3_RST", &self.TIMER3_RST())
            .field("TIMER4_RST", &self.TIMER4_RST())
            .field("PUF_RST", &self.PUF_RST())
            .field("CASPER_RST", &self.CASPER_RST())
            .field("ANALOG_CTRL_RST", &self.ANALOG_CTRL_RST())
            .field("HS_LSPI_RST", &self.HS_LSPI_RST())
            .field("GPIO_SEC_RST", &self.GPIO_SEC_RST())
            .field("GPIO_SEC_INT_RST", &self.GPIO_SEC_INT_RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRESETCTRL2 {{ DMA1_RST: {:?}, COMP_RST: {:?}, USB1_HOST_RST: {:?}, USB1_DEV_RST: {:?}, USB1_RAM_RST: {:?}, USB1_PHY_RST: {:?}, FREQME_RST: {:?}, CDOG_RST: {:?}, RNG_RST: {:?}, SYSCTL_RST: {:?}, USB0_HOSTM_RST: {:?}, USB0_HOSTS_RST: {:?}, HASH_AES_RST: {:?}, PLULUT_RST: {:?}, TIMER3_RST: {:?}, TIMER4_RST: {:?}, PUF_RST: {:?}, CASPER_RST: {:?}, ANALOG_CTRL_RST: {:?}, HS_LSPI_RST: {:?}, GPIO_SEC_RST: {:?}, GPIO_SEC_INT_RST: {:?} }}",
            self.DMA1_RST(),
            self.COMP_RST(),
            self.USB1_HOST_RST(),
            self.USB1_DEV_RST(),
            self.USB1_RAM_RST(),
            self.USB1_PHY_RST(),
            self.FREQME_RST(),
            self.CDOG_RST(),
            self.RNG_RST(),
            self.SYSCTL_RST(),
            self.USB0_HOSTM_RST(),
            self.USB0_HOSTS_RST(),
            self.HASH_AES_RST(),
            self.PLULUT_RST(),
            self.TIMER3_RST(),
            self.TIMER4_RST(),
            self.PUF_RST(),
            self.CASPER_RST(),
            self.ANALOG_CTRL_RST(),
            self.HS_LSPI_RST(),
            self.GPIO_SEC_RST(),
            self.GPIO_SEC_INT_RST()
        )
    }
}
#[doc = "Peripheral reset control clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRLCLR(pub u32);
impl PRESETCTRLCLR {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRESETCTRLCLR {
    #[inline(always)]
    fn default() -> PRESETCTRLCLR {
        PRESETCTRLCLR(0)
    }
}
impl core::fmt::Debug for PRESETCTRLCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRLCLR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRLCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRESETCTRLCLR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control set register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRLSET(pub u32);
impl PRESETCTRLSET {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRESETCTRLSET {
    #[inline(always)]
    fn default() -> PRESETCTRLSET {
        PRESETCTRLSET(0)
    }
}
impl core::fmt::Debug for PRESETCTRLSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRLSET")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRLSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRESETCTRLSET {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRLX0(pub u32);
impl PRESETCTRLX0 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRESETCTRLX0 {
    #[inline(always)]
    fn default() -> PRESETCTRLX0 {
        PRESETCTRLX0(0)
    }
}
impl core::fmt::Debug for PRESETCTRLX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRLX0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRLX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRESETCTRLX0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRLX1(pub u32);
impl PRESETCTRLX1 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRESETCTRLX1 {
    #[inline(always)]
    fn default() -> PRESETCTRLX1 {
        PRESETCTRLX1(0)
    }
}
impl core::fmt::Debug for PRESETCTRLX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRLX1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRLX1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRESETCTRLX1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRESETCTRLX2(pub u32);
impl PRESETCTRLX2 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRESETCTRLX2 {
    #[inline(always)]
    fn default() -> PRESETCTRLX2 {
        PRESETCTRLX2(0)
    }
}
impl core::fmt::Debug for PRESETCTRLX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESETCTRLX2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRESETCTRLX2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRESETCTRLX2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "SCT/PWM clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCTCLKDIV(pub u32);
impl SCTCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::SCTCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SCTCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::SCTCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::SCTCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SCTCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::SCTCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::SCTCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SCTCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::SCTCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SCTCLKDIV {
    #[inline(always)]
    fn default() -> SCTCLKDIV {
        SCTCLKDIV(0)
    }
}
impl core::fmt::Debug for SCTCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCTCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SCTCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "SCTimer/PWM clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCTCLKSEL(pub u32);
impl SCTCLKSEL {
    #[doc = "SCTimer/PWM clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::SCTCLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SCTCLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "SCTimer/PWM clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::SCTCLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for SCTCLKSEL {
    #[inline(always)]
    fn default() -> SCTCLKSEL {
        SCTCLKSEL(0)
    }
}
impl core::fmt::Debug for SCTCLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTCLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCTCLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SCTCLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "This register is used by ROM during DEBUG authentication mechanism to enable debug access port for CPU0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWD_ACCESS_CPU0(pub u32);
impl SWD_ACCESS_CPU0 {
    #[doc = "CPU0 SWD-AP: 0x12345678."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_CODE(&self) -> super::vals::SEC_CODE {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SEC_CODE::from_bits(val as u32)
    }
    #[doc = "CPU0 SWD-AP: 0x12345678."]
    #[inline(always)]
    pub const fn set_SEC_CODE(&mut self, val: super::vals::SEC_CODE) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SWD_ACCESS_CPU0 {
    #[inline(always)]
    fn default() -> SWD_ACCESS_CPU0 {
        SWD_ACCESS_CPU0(0)
    }
}
impl core::fmt::Debug for SWD_ACCESS_CPU0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_ACCESS_CPU0")
            .field("SEC_CODE", &self.SEC_CODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWD_ACCESS_CPU0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SWD_ACCESS_CPU0 {{ SEC_CODE: {:?} }}", self.SEC_CODE())
    }
}
#[doc = "generate a software_reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWR_RESET(pub u32);
impl SWR_RESET {
    #[doc = "Write 0x5A00_0001 to generate a software_reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SWR_RESET(&self) -> super::vals::SWR_RESET {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SWR_RESET::from_bits(val as u32)
    }
    #[doc = "Write 0x5A00_0001 to generate a software_reset."]
    #[inline(always)]
    pub const fn set_SWR_RESET(&mut self, val: super::vals::SWR_RESET) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SWR_RESET {
    #[inline(always)]
    fn default() -> SWR_RESET {
        SWR_RESET(0)
    }
}
impl core::fmt::Debug for SWR_RESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWR_RESET")
            .field("SWR_RESET", &self.SWR_RESET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWR_RESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SWR_RESET {{ SWR_RESET: {:?} }}", self.SWR_RESET())
    }
}
#[doc = "System Tick Timer divider for CPU0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSTICKCLKDIV0(pub u32);
impl SYSTICKCLKDIV0 {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::SYSTICKCLKDIV0_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SYSTICKCLKDIV0_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::SYSTICKCLKDIV0_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::SYSTICKCLKDIV0_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SYSTICKCLKDIV0_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::SYSTICKCLKDIV0_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::SYSTICKCLKDIV0_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SYSTICKCLKDIV0_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::SYSTICKCLKDIV0_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SYSTICKCLKDIV0 {
    #[inline(always)]
    fn default() -> SYSTICKCLKDIV0 {
        SYSTICKCLKDIV0(0)
    }
}
impl core::fmt::Debug for SYSTICKCLKDIV0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTICKCLKDIV0")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSTICKCLKDIV0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYSTICKCLKDIV0 {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "System Tick Timer for CPU0 source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSTICKCLKSEL0(pub u32);
impl SYSTICKCLKSEL0 {
    #[doc = "System Tick Timer for CPU0 source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::SYSTICKCLKSEL0_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SYSTICKCLKSEL0_SEL::from_bits(val as u8)
    }
    #[doc = "System Tick Timer for CPU0 source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::SYSTICKCLKSEL0_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for SYSTICKCLKSEL0 {
    #[inline(always)]
    fn default() -> SYSTICKCLKSEL0 {
        SYSTICKCLKSEL0(0)
    }
}
impl core::fmt::Debug for SYSTICKCLKSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTICKCLKSEL0")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSTICKCLKSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SYSTICKCLKSEL0 {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "Peripheral reset control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSTICKCLKSELX0(pub u32);
impl SYSTICKCLKSELX0 {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SYSTICKCLKSELX0 {
    #[inline(always)]
    fn default() -> SYSTICKCLKSELX0 {
        SYSTICKCLKSELX0(0)
    }
}
impl core::fmt::Debug for SYSTICKCLKSELX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTICKCLKSELX0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSTICKCLKSELX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SYSTICKCLKSELX0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "TRACE clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRACECLKDIV(pub u32);
impl TRACECLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::TRACECLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::TRACECLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::TRACECLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::TRACECLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::TRACECLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::TRACECLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::TRACECLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TRACECLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::TRACECLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TRACECLKDIV {
    #[inline(always)]
    fn default() -> TRACECLKDIV {
        TRACECLKDIV(0)
    }
}
impl core::fmt::Debug for TRACECLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACECLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRACECLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRACECLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "Trace clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRACECLKSEL(pub u32);
impl TRACECLKSEL {
    #[doc = "Trace clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::TRACECLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TRACECLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "Trace clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::TRACECLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for TRACECLKSEL {
    #[inline(always)]
    fn default() -> TRACECLKSEL {
        TRACECLKSEL(0)
    }
}
impl core::fmt::Debug for TRACECLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACECLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRACECLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TRACECLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "USB0-FS Clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB0CLKDIV(pub u32);
impl USB0CLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::USB0CLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::USB0CLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::USB0CLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::USB0CLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::USB0CLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::USB0CLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::USB0CLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::USB0CLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::USB0CLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for USB0CLKDIV {
    #[inline(always)]
    fn default() -> USB0CLKDIV {
        USB0CLKDIV(0)
    }
}
impl core::fmt::Debug for USB0CLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB0CLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB0CLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB0CLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
#[doc = "FS USB clock source select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB0CLKSEL(pub u32);
impl USB0CLKSEL {
    #[doc = "FS USB clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::USB0CLKSEL_SEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::USB0CLKSEL_SEL::from_bits(val as u8)
    }
    #[doc = "FS USB clock source select."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::USB0CLKSEL_SEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for USB0CLKSEL {
    #[inline(always)]
    fn default() -> USB0CLKSEL {
        USB0CLKSEL(0)
    }
}
impl core::fmt::Debug for USB0CLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB0CLKSEL")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB0CLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USB0CLKSEL {{ SEL: {:?} }}", self.SEL())
    }
}
#[doc = "USB0-FS need clock control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB0NEEDCLKCTRL(pub u32);
impl USB0NEEDCLKCTRL {
    #[doc = "USB0-FS Device USB0_NEEDCLK signal control:."]
    #[must_use]
    #[inline(always)]
    pub const fn AP_FS_DEV_NEEDCLK(&self) -> super::vals::AP_FS_DEV_NEEDCLK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AP_FS_DEV_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB0-FS Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub const fn set_AP_FS_DEV_NEEDCLK(&mut self, val: super::vals::AP_FS_DEV_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[must_use]
    #[inline(always)]
    pub const fn POL_FS_DEV_NEEDCLK(&self) -> super::vals::POL_FS_DEV_NEEDCLK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::POL_FS_DEV_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB0-FS Device USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[inline(always)]
    pub const fn set_POL_FS_DEV_NEEDCLK(&mut self, val: super::vals::POL_FS_DEV_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "USB0-FS Host USB0_NEEDCLK signal control:."]
    #[must_use]
    #[inline(always)]
    pub const fn AP_FS_HOST_NEEDCLK(&self) -> super::vals::AP_FS_HOST_NEEDCLK {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AP_FS_HOST_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB0-FS Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub const fn set_AP_FS_HOST_NEEDCLK(&mut self, val: super::vals::AP_FS_HOST_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[must_use]
    #[inline(always)]
    pub const fn POL_FS_HOST_NEEDCLK(&self) -> super::vals::POL_FS_HOST_NEEDCLK {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::POL_FS_HOST_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB0-FS Host USB0_NEEDCLK polarity for triggering the USB0-FS wake-up interrupt:."]
    #[inline(always)]
    pub const fn set_POL_FS_HOST_NEEDCLK(&mut self, val: super::vals::POL_FS_HOST_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for USB0NEEDCLKCTRL {
    #[inline(always)]
    fn default() -> USB0NEEDCLKCTRL {
        USB0NEEDCLKCTRL(0)
    }
}
impl core::fmt::Debug for USB0NEEDCLKCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB0NEEDCLKCTRL")
            .field("AP_FS_DEV_NEEDCLK", &self.AP_FS_DEV_NEEDCLK())
            .field("POL_FS_DEV_NEEDCLK", &self.POL_FS_DEV_NEEDCLK())
            .field("AP_FS_HOST_NEEDCLK", &self.AP_FS_HOST_NEEDCLK())
            .field("POL_FS_HOST_NEEDCLK", &self.POL_FS_HOST_NEEDCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB0NEEDCLKCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB0NEEDCLKCTRL {{ AP_FS_DEV_NEEDCLK: {:?}, POL_FS_DEV_NEEDCLK: {:?}, AP_FS_HOST_NEEDCLK: {:?}, POL_FS_HOST_NEEDCLK: {:?} }}",
            self.AP_FS_DEV_NEEDCLK(),
            self.POL_FS_DEV_NEEDCLK(),
            self.AP_FS_HOST_NEEDCLK(),
            self.POL_FS_HOST_NEEDCLK()
        )
    }
}
#[doc = "USB0-FS need clock status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB0NEEDCLKSTAT(pub u32);
impl USB0NEEDCLKSTAT {
    #[doc = "USB0-FS Device USB0_NEEDCLK signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_NEEDCLK(&self) -> super::vals::USB0NEEDCLKSTAT_DEV_NEEDCLK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::USB0NEEDCLKSTAT_DEV_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB0-FS Device USB0_NEEDCLK signal status:."]
    #[inline(always)]
    pub const fn set_DEV_NEEDCLK(&mut self, val: super::vals::USB0NEEDCLKSTAT_DEV_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB0-FS Host USB0_NEEDCLK signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_NEEDCLK(&self) -> super::vals::USB0NEEDCLKSTAT_HOST_NEEDCLK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::USB0NEEDCLKSTAT_HOST_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB0-FS Host USB0_NEEDCLK signal status:."]
    #[inline(always)]
    pub const fn set_HOST_NEEDCLK(&mut self, val: super::vals::USB0NEEDCLKSTAT_HOST_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for USB0NEEDCLKSTAT {
    #[inline(always)]
    fn default() -> USB0NEEDCLKSTAT {
        USB0NEEDCLKSTAT(0)
    }
}
impl core::fmt::Debug for USB0NEEDCLKSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB0NEEDCLKSTAT")
            .field("DEV_NEEDCLK", &self.DEV_NEEDCLK())
            .field("HOST_NEEDCLK", &self.HOST_NEEDCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB0NEEDCLKSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB0NEEDCLKSTAT {{ DEV_NEEDCLK: {:?}, HOST_NEEDCLK: {:?} }}",
            self.DEV_NEEDCLK(),
            self.HOST_NEEDCLK()
        )
    }
}
#[doc = "USB1-HS need clock control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1NEEDCLKCTRL(pub u32);
impl USB1NEEDCLKCTRL {
    #[doc = "USB1-HS Device need_clock signal control:."]
    #[must_use]
    #[inline(always)]
    pub const fn AP_HS_DEV_NEEDCLK(&self) -> super::vals::AP_HS_DEV_NEEDCLK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AP_HS_DEV_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB1-HS Device need_clock signal control:."]
    #[inline(always)]
    pub const fn set_AP_HS_DEV_NEEDCLK(&mut self, val: super::vals::AP_HS_DEV_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:."]
    #[must_use]
    #[inline(always)]
    pub const fn POL_HS_DEV_NEEDCLK(&self) -> super::vals::POL_HS_DEV_NEEDCLK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::POL_HS_DEV_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB1-HS device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:."]
    #[inline(always)]
    pub const fn set_POL_HS_DEV_NEEDCLK(&mut self, val: super::vals::POL_HS_DEV_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "USB1-HS Host need clock signal control:."]
    #[must_use]
    #[inline(always)]
    pub const fn AP_HS_HOST_NEEDCLK(&self) -> super::vals::AP_HS_HOST_NEEDCLK {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AP_HS_HOST_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB1-HS Host need clock signal control:."]
    #[inline(always)]
    pub const fn set_AP_HS_HOST_NEEDCLK(&mut self, val: super::vals::AP_HS_HOST_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn POL_HS_HOST_NEEDCLK(&self) -> super::vals::POL_HS_HOST_NEEDCLK {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::POL_HS_HOST_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB1-HS host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub const fn set_POL_HS_HOST_NEEDCLK(&mut self, val: super::vals::POL_HS_HOST_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Software override of device controller PHY wake up logic."]
    #[must_use]
    #[inline(always)]
    pub const fn HS_DEV_WAKEUP_N(&self) -> super::vals::HS_DEV_WAKEUP_N {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HS_DEV_WAKEUP_N::from_bits(val as u8)
    }
    #[doc = "Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub const fn set_HS_DEV_WAKEUP_N(&mut self, val: super::vals::HS_DEV_WAKEUP_N) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for USB1NEEDCLKCTRL {
    #[inline(always)]
    fn default() -> USB1NEEDCLKCTRL {
        USB1NEEDCLKCTRL(0)
    }
}
impl core::fmt::Debug for USB1NEEDCLKCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1NEEDCLKCTRL")
            .field("AP_HS_DEV_NEEDCLK", &self.AP_HS_DEV_NEEDCLK())
            .field("POL_HS_DEV_NEEDCLK", &self.POL_HS_DEV_NEEDCLK())
            .field("AP_HS_HOST_NEEDCLK", &self.AP_HS_HOST_NEEDCLK())
            .field("POL_HS_HOST_NEEDCLK", &self.POL_HS_HOST_NEEDCLK())
            .field("HS_DEV_WAKEUP_N", &self.HS_DEV_WAKEUP_N())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1NEEDCLKCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1NEEDCLKCTRL {{ AP_HS_DEV_NEEDCLK: {:?}, POL_HS_DEV_NEEDCLK: {:?}, AP_HS_HOST_NEEDCLK: {:?}, POL_HS_HOST_NEEDCLK: {:?}, HS_DEV_WAKEUP_N: {:?} }}",
            self.AP_HS_DEV_NEEDCLK(),
            self.POL_HS_DEV_NEEDCLK(),
            self.AP_HS_HOST_NEEDCLK(),
            self.POL_HS_HOST_NEEDCLK(),
            self.HS_DEV_WAKEUP_N()
        )
    }
}
#[doc = "USB1-HS need clock status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1NEEDCLKSTAT(pub u32);
impl USB1NEEDCLKSTAT {
    #[doc = "USB1-HS Device need_clock signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_NEEDCLK(&self) -> super::vals::USB1NEEDCLKSTAT_DEV_NEEDCLK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::USB1NEEDCLKSTAT_DEV_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB1-HS Device need_clock signal status:."]
    #[inline(always)]
    pub const fn set_DEV_NEEDCLK(&mut self, val: super::vals::USB1NEEDCLKSTAT_DEV_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB1-HS Host need_clock signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_NEEDCLK(&self) -> super::vals::USB1NEEDCLKSTAT_HOST_NEEDCLK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::USB1NEEDCLKSTAT_HOST_NEEDCLK::from_bits(val as u8)
    }
    #[doc = "USB1-HS Host need_clock signal status:."]
    #[inline(always)]
    pub const fn set_HOST_NEEDCLK(&mut self, val: super::vals::USB1NEEDCLKSTAT_HOST_NEEDCLK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for USB1NEEDCLKSTAT {
    #[inline(always)]
    fn default() -> USB1NEEDCLKSTAT {
        USB1NEEDCLKSTAT(0)
    }
}
impl core::fmt::Debug for USB1NEEDCLKSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1NEEDCLKSTAT")
            .field("DEV_NEEDCLK", &self.DEV_NEEDCLK())
            .field("HOST_NEEDCLK", &self.HOST_NEEDCLK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1NEEDCLKSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1NEEDCLKSTAT {{ DEV_NEEDCLK: {:?}, HOST_NEEDCLK: {:?} }}",
            self.DEV_NEEDCLK(),
            self.HOST_NEEDCLK()
        )
    }
}
#[doc = "WDT clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDTCLKDIV(pub u32);
impl WDTCLKDIV {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> super::vals::WDTCLKDIV_RESET {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::WDTCLKDIV_RESET::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: super::vals::WDTCLKDIV_RESET) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> super::vals::WDTCLKDIV_HALT {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::WDTCLKDIV_HALT::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: super::vals::WDTCLKDIV_HALT) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn REQFLAG(&self) -> super::vals::WDTCLKDIV_REQFLAG {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::WDTCLKDIV_REQFLAG::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_REQFLAG(&mut self, val: super::vals::WDTCLKDIV_REQFLAG) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for WDTCLKDIV {
    #[inline(always)]
    fn default() -> WDTCLKDIV {
        WDTCLKDIV(0)
    }
}
impl core::fmt::Debug for WDTCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCLKDIV")
            .field("DIV", &self.DIV())
            .field("RESET", &self.RESET())
            .field("HALT", &self.HALT())
            .field("REQFLAG", &self.REQFLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDTCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WDTCLKDIV {{ DIV: {=u8:?}, RESET: {:?}, HALT: {:?}, REQFLAG: {:?} }}",
            self.DIV(),
            self.RESET(),
            self.HALT(),
            self.REQFLAG()
        )
    }
}
