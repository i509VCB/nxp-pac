#[doc = "ADC clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcclkdiv(pub u32);
impl Adcclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::AdcclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::AdcclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::AdcclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::AdcclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::AdcclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::AdcclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::AdcclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AdcclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::AdcclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Adcclkdiv {
    #[inline(always)]
    fn default() -> Adcclkdiv {
        Adcclkdiv(0)
    }
}
impl core::fmt::Debug for Adcclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adcclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "ADC clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcclksel(pub u32);
impl Adcclksel {
    #[doc = "ADC clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "ADC clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Adcclksel {
    #[inline(always)]
    fn default() -> Adcclksel {
        Adcclksel(0)
    }
}
impl core::fmt::Debug for Adcclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adcclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adcclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adcclksel {{ sel: {=u8:?} }}", self.sel())
    }
}
#[doc = "AHB Clock control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl0(pub u32);
impl Ahbclkctrl0 {
    #[doc = "Enables the clock for the ROM."]
    #[must_use]
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the ROM."]
    #[inline(always)]
    pub const fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the SRAM Controller 1."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub const fn set_sram_ctrl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for the SRAM Controller 2."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub const fn set_sram_ctrl2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for the SRAM Controller 3."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub const fn set_sram_ctrl3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for the SRAM Controller 4."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    pub const fn set_sram_ctrl4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for the Flash controller."]
    #[must_use]
    #[inline(always)]
    pub const fn flash(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Flash controller."]
    #[inline(always)]
    pub const fn set_flash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the FMC controller."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FMC controller."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for the Input Mux."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Input Mux."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for the I/O controller."]
    #[must_use]
    #[inline(always)]
    pub const fn iocon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the I/O controller."]
    #[inline(always)]
    pub const fn set_iocon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for the GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for the GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for the GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for the GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for the Pin interrupt (PINT)."]
    #[must_use]
    #[inline(always)]
    pub const fn pint(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub const fn set_pint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for the Group interrupt (GINT)."]
    #[must_use]
    #[inline(always)]
    pub const fn gint(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub const fn set_gint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for the DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for the CRCGEN."]
    #[must_use]
    #[inline(always)]
    pub const fn crcgen(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub const fn set_crcgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for the Watchdog Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub const fn set_wwdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for the Real Time Clock (RTC)."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub const fn set_mailbox(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for the ADC."]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the ADC."]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Ahbclkctrl0 {
    #[inline(always)]
    fn default() -> Ahbclkctrl0 {
        Ahbclkctrl0(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl0")
            .field("rom", &self.rom())
            .field("sram_ctrl1", &self.sram_ctrl1())
            .field("sram_ctrl2", &self.sram_ctrl2())
            .field("sram_ctrl3", &self.sram_ctrl3())
            .field("sram_ctrl4", &self.sram_ctrl4())
            .field("flash", &self.flash())
            .field("fmc", &self.fmc())
            .field("mux", &self.mux())
            .field("iocon", &self.iocon())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("pint", &self.pint())
            .field("gint", &self.gint())
            .field("dma0", &self.dma0())
            .field("crcgen", &self.crcgen())
            .field("wwdt", &self.wwdt())
            .field("rtc", &self.rtc())
            .field("mailbox", &self.mailbox())
            .field("adc", &self.adc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl0 {{ rom: {=bool:?}, sram_ctrl1: {=bool:?}, sram_ctrl2: {=bool:?}, sram_ctrl3: {=bool:?}, sram_ctrl4: {=bool:?}, flash: {=bool:?}, fmc: {=bool:?}, mux: {=bool:?}, iocon: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, pint: {=bool:?}, gint: {=bool:?}, dma0: {=bool:?}, crcgen: {=bool:?}, wwdt: {=bool:?}, rtc: {=bool:?}, mailbox: {=bool:?}, adc: {=bool:?} }}",
            self.rom(),
            self.sram_ctrl1(),
            self.sram_ctrl2(),
            self.sram_ctrl3(),
            self.sram_ctrl4(),
            self.flash(),
            self.fmc(),
            self.mux(),
            self.iocon(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.pint(),
            self.gint(),
            self.dma0(),
            self.crcgen(),
            self.wwdt(),
            self.rtc(),
            self.mailbox(),
            self.adc()
        )
    }
}
#[doc = "AHB Clock control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl1(pub u32);
impl Ahbclkctrl1 {
    #[doc = "Enables the clock for the MRT."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the MRT."]
    #[inline(always)]
    pub const fn set_mrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the clock for the OS Event Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub const fn set_ostimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn sct(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SCT."]
    #[inline(always)]
    pub const fn set_sct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for the UTICK."]
    #[must_use]
    #[inline(always)]
    pub const fn utick(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the UTICK."]
    #[inline(always)]
    pub const fn set_utick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables the clock for the FC0."]
    #[must_use]
    #[inline(always)]
    pub const fn fc(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 11usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the FC0."]
    #[inline(always)]
    pub const fn set_fc(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 11usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Enables the clock for the Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn timer2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 2."]
    #[inline(always)]
    pub const fn set_timer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for the USB0 DEV."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_dev(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB0 DEV."]
    #[inline(always)]
    pub const fn set_usb0_dev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for the Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 0."]
    #[inline(always)]
    pub const fn set_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for the Timer 1."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 1."]
    #[inline(always)]
    pub const fn set_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Ahbclkctrl1 {
    #[inline(always)]
    fn default() -> Ahbclkctrl1 {
        Ahbclkctrl1(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl1")
            .field("mrt", &self.mrt())
            .field("ostimer", &self.ostimer())
            .field("sct", &self.sct())
            .field("utick", &self.utick())
            .field("fc[0]", &self.fc(0usize))
            .field("fc[1]", &self.fc(1usize))
            .field("fc[2]", &self.fc(2usize))
            .field("fc[3]", &self.fc(3usize))
            .field("fc[4]", &self.fc(4usize))
            .field("fc[5]", &self.fc(5usize))
            .field("fc[6]", &self.fc(6usize))
            .field("fc[7]", &self.fc(7usize))
            .field("timer2", &self.timer2())
            .field("usb0_dev", &self.usb0_dev())
            .field("timer0", &self.timer0())
            .field("timer1", &self.timer1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl1 {{ mrt: {=bool:?}, ostimer: {=bool:?}, sct: {=bool:?}, utick: {=bool:?}, fc[0]: {=bool:?}, fc[1]: {=bool:?}, fc[2]: {=bool:?}, fc[3]: {=bool:?}, fc[4]: {=bool:?}, fc[5]: {=bool:?}, fc[6]: {=bool:?}, fc[7]: {=bool:?}, timer2: {=bool:?}, usb0_dev: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?} }}",
            self.mrt(),
            self.ostimer(),
            self.sct(),
            self.utick(),
            self.fc(0usize),
            self.fc(1usize),
            self.fc(2usize),
            self.fc(3usize),
            self.fc(4usize),
            self.fc(5usize),
            self.fc(6usize),
            self.fc(7usize),
            self.timer2(),
            self.usb0_dev(),
            self.timer0(),
            self.timer1()
        )
    }
}
#[doc = "AHB Clock control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl2(pub u32);
impl Ahbclkctrl2 {
    #[doc = "Enables the clock for the DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Comparator."]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for the SDIO."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the SDIO."]
    #[inline(always)]
    pub const fn set_sdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for the USB1 Host."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_host(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1 Host."]
    #[inline(always)]
    pub const fn set_usb1_host(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for the USB1 dev."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_dev(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1 dev."]
    #[inline(always)]
    pub const fn set_usb1_dev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for the USB1 RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_ram(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1 RAM."]
    #[inline(always)]
    pub const fn set_usb1_ram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for the USB1 PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB1 PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the Frequency meter."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for the RNG."]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RNG."]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SYSCTL block clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sysctl(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SYSCTL block clock."]
    #[inline(always)]
    pub const fn set_sysctl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for the USB0 Host Master."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_hostm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB0 Host Master."]
    #[inline(always)]
    pub const fn set_usb0_hostm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for the USB0 Host Slave."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_hosts(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the USB0 Host Slave."]
    #[inline(always)]
    pub const fn set_usb0_hosts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for the HASH_AES."]
    #[must_use]
    #[inline(always)]
    pub const fn hash_aes(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub const fn set_hash_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for the Power Quad."]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Power Quad."]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for the PLU LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn plulut(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub const fn set_plulut(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for the Timer 3."]
    #[must_use]
    #[inline(always)]
    pub const fn timer3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 3."]
    #[inline(always)]
    pub const fn set_timer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for the Timer 4."]
    #[must_use]
    #[inline(always)]
    pub const fn timer4(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Timer 4."]
    #[inline(always)]
    pub const fn set_timer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for the PUF reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for the Casper."]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Casper."]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for the analog control."]
    #[must_use]
    #[inline(always)]
    pub const fn analog_ctrl(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the analog control."]
    #[inline(always)]
    pub const fn set_analog_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enables the clock for the HS LSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn hs_lspi(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub const fn set_hs_lspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enables the clock for the GPIO secure."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sec(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub const fn set_gpio_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for the GPIO secure int."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sec_int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub const fn set_gpio_sec_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ahbclkctrl2 {
    #[inline(always)]
    fn default() -> Ahbclkctrl2 {
        Ahbclkctrl2(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl2")
            .field("dma1", &self.dma1())
            .field("comp", &self.comp())
            .field("sdio", &self.sdio())
            .field("usb1_host", &self.usb1_host())
            .field("usb1_dev", &self.usb1_dev())
            .field("usb1_ram", &self.usb1_ram())
            .field("usb1_phy", &self.usb1_phy())
            .field("freqme", &self.freqme())
            .field("rng", &self.rng())
            .field("sysctl", &self.sysctl())
            .field("usb0_hostm", &self.usb0_hostm())
            .field("usb0_hosts", &self.usb0_hosts())
            .field("hash_aes", &self.hash_aes())
            .field("pq", &self.pq())
            .field("plulut", &self.plulut())
            .field("timer3", &self.timer3())
            .field("timer4", &self.timer4())
            .field("puf", &self.puf())
            .field("casper", &self.casper())
            .field("analog_ctrl", &self.analog_ctrl())
            .field("hs_lspi", &self.hs_lspi())
            .field("gpio_sec", &self.gpio_sec())
            .field("gpio_sec_int", &self.gpio_sec_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl2 {{ dma1: {=bool:?}, comp: {=bool:?}, sdio: {=bool:?}, usb1_host: {=bool:?}, usb1_dev: {=bool:?}, usb1_ram: {=bool:?}, usb1_phy: {=bool:?}, freqme: {=bool:?}, rng: {=bool:?}, sysctl: {=bool:?}, usb0_hostm: {=bool:?}, usb0_hosts: {=bool:?}, hash_aes: {=bool:?}, pq: {=bool:?}, plulut: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, puf: {=bool:?}, casper: {=bool:?}, analog_ctrl: {=bool:?}, hs_lspi: {=bool:?}, gpio_sec: {=bool:?}, gpio_sec_int: {=bool:?} }}",
            self.dma1(),
            self.comp(),
            self.sdio(),
            self.usb1_host(),
            self.usb1_dev(),
            self.usb1_ram(),
            self.usb1_phy(),
            self.freqme(),
            self.rng(),
            self.sysctl(),
            self.usb0_hostm(),
            self.usb0_hosts(),
            self.hash_aes(),
            self.pq(),
            self.plulut(),
            self.timer3(),
            self.timer4(),
            self.puf(),
            self.casper(),
            self.analog_ctrl(),
            self.hs_lspi(),
            self.gpio_sec(),
            self.gpio_sec_int()
        )
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrlclr(pub u32);
impl Ahbclkctrlclr {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ahbclkctrlclr {
    #[inline(always)]
    fn default() -> Ahbclkctrlclr {
        Ahbclkctrlclr(0)
    }
}
impl core::fmt::Debug for Ahbclkctrlclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrlclr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrlclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ahbclkctrlclr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrlset(pub u32);
impl Ahbclkctrlset {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ahbclkctrlset {
    #[inline(always)]
    fn default() -> Ahbclkctrlset {
        Ahbclkctrlset(0)
    }
}
impl core::fmt::Debug for Ahbclkctrlset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrlset")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrlset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ahbclkctrlset {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "System clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::AhbclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::AhbclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::AhbclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::AhbclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::AhbclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::AhbclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::AhbclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AhbclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::AhbclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkdiv {
    #[inline(always)]
    fn default() -> Ahbclkdiv {
        Ahbclkdiv(0)
    }
}
impl core::fmt::Debug for Ahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu0_cbus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CPU0 C-AHB bus."]
    #[inline(always)]
    pub const fn set_pri_cpu0_cbus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu0_sbus(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "CPU0 S-AHB bus."]
    #[inline(always)]
    pub const fn set_pri_cpu0_sbus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU1 C-AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu1_cbus(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "CPU1 C-AHB bus."]
    #[inline(always)]
    pub const fn set_pri_cpu1_cbus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU1 S-AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu1_sbus(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "CPU1 S-AHB bus."]
    #[inline(always)]
    pub const fn set_pri_cpu1_sbus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "USB-FS.(USB0)"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usb_fs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "USB-FS.(USB0)"]
    #[inline(always)]
    pub const fn set_pri_usb_fs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA0 controller priority."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_sdma0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "DMA0 controller priority."]
    #[inline(always)]
    pub const fn set_pri_sdma0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "SDIO."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_sdio(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "SDIO."]
    #[inline(always)]
    pub const fn set_pri_sdio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "PQ (HW Accelerator)."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_pq(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "PQ (HW Accelerator)."]
    #[inline(always)]
    pub const fn set_pri_pq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "HASH_AES."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_hash_aes(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "HASH_AES."]
    #[inline(always)]
    pub const fn set_pri_hash_aes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "USB-HS.(USB1)"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usb_hs(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "USB-HS.(USB1)"]
    #[inline(always)]
    pub const fn set_pri_usb_hs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "DMA1 controller priority."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_sdma1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "DMA1 controller priority."]
    #[inline(always)]
    pub const fn set_pri_sdma1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Ahbmatprio {
    #[inline(always)]
    fn default() -> Ahbmatprio {
        Ahbmatprio(0)
    }
}
impl core::fmt::Debug for Ahbmatprio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatprio")
            .field("pri_cpu0_cbus", &self.pri_cpu0_cbus())
            .field("pri_cpu0_sbus", &self.pri_cpu0_sbus())
            .field("pri_cpu1_cbus", &self.pri_cpu1_cbus())
            .field("pri_cpu1_sbus", &self.pri_cpu1_sbus())
            .field("pri_usb_fs", &self.pri_usb_fs())
            .field("pri_sdma0", &self.pri_sdma0())
            .field("pri_sdio", &self.pri_sdio())
            .field("pri_pq", &self.pri_pq())
            .field("pri_hash_aes", &self.pri_hash_aes())
            .field("pri_usb_hs", &self.pri_usb_hs())
            .field("pri_sdma1", &self.pri_sdma1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ pri_cpu0_cbus: {=u8:?}, pri_cpu0_sbus: {=u8:?}, pri_cpu1_cbus: {=u8:?}, pri_cpu1_sbus: {=u8:?}, pri_usb_fs: {=u8:?}, pri_sdma0: {=u8:?}, pri_sdio: {=u8:?}, pri_pq: {=u8:?}, pri_hash_aes: {=u8:?}, pri_usb_hs: {=u8:?}, pri_sdma1: {=u8:?} }}",
            self.pri_cpu0_cbus(),
            self.pri_cpu0_sbus(),
            self.pri_cpu1_cbus(),
            self.pri_cpu1_sbus(),
            self.pri_usb_fs(),
            self.pri_sdma0(),
            self.pri_sdio(),
            self.pri_pq(),
            self.pri_hash_aes(),
            self.pri_usb_hs(),
            self.pri_sdma1()
        )
    }
}
#[doc = "Control automatic clock gating"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverride(pub u32);
impl Autoclkgateoverride {
    #[doc = "Control automatic clock gating of ROM controller."]
    #[must_use]
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub const fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control automatic clock gating of RAMX controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx_ctrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub const fn set_ramx_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control automatic clock gating of RAM0 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ram0_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub const fn set_ram0_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control automatic clock gating of RAM1 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ram1_ctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub const fn set_ram1_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control automatic clock gating of RAM2 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ram2_ctrl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub const fn set_ram2_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control automatic clock gating of RAM3 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ram3_ctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM3 controller."]
    #[inline(always)]
    pub const fn set_ram3_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Control automatic clock gating of RAM4 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ram4_ctrl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of RAM4 controller."]
    #[inline(always)]
    pub const fn set_ram4_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sync0_apb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub const fn set_sync0_apb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 1."]
    #[must_use]
    #[inline(always)]
    pub const fn sync1_apb(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub const fn set_sync1_apb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Control automatic clock gating of CRCGEN controller."]
    #[must_use]
    #[inline(always)]
    pub const fn crcgen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub const fn set_crcgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Control automatic clock gating of DMA0 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn sdma0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub const fn set_sdma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Control automatic clock gating of DMA1 controller."]
    #[must_use]
    #[inline(always)]
    pub const fn sdma1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub const fn set_sdma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Control automatic clock gating of USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Control automatic clock gating of synchronous system controller registers bank."]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[must_use]
    #[inline(always)]
    pub const fn enableupdate(&self) -> super::vals::Enableupdate {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Enableupdate::from_bits(val as u16)
    }
    #[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[inline(always)]
    pub const fn set_enableupdate(&mut self, val: super::vals::Enableupdate) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Autoclkgateoverride {
    #[inline(always)]
    fn default() -> Autoclkgateoverride {
        Autoclkgateoverride(0)
    }
}
impl core::fmt::Debug for Autoclkgateoverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoclkgateoverride")
            .field("rom", &self.rom())
            .field("ramx_ctrl", &self.ramx_ctrl())
            .field("ram0_ctrl", &self.ram0_ctrl())
            .field("ram1_ctrl", &self.ram1_ctrl())
            .field("ram2_ctrl", &self.ram2_ctrl())
            .field("ram3_ctrl", &self.ram3_ctrl())
            .field("ram4_ctrl", &self.ram4_ctrl())
            .field("sync0_apb", &self.sync0_apb())
            .field("sync1_apb", &self.sync1_apb())
            .field("crcgen", &self.crcgen())
            .field("sdma0", &self.sdma0())
            .field("sdma1", &self.sdma1())
            .field("usb0", &self.usb0())
            .field("syscon", &self.syscon())
            .field("enableupdate", &self.enableupdate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverride {{ rom: {=bool:?}, ramx_ctrl: {=bool:?}, ram0_ctrl: {=bool:?}, ram1_ctrl: {=bool:?}, ram2_ctrl: {=bool:?}, ram3_ctrl: {=bool:?}, ram4_ctrl: {=bool:?}, sync0_apb: {=bool:?}, sync1_apb: {=bool:?}, crcgen: {=bool:?}, sdma0: {=bool:?}, sdma1: {=bool:?}, usb0: {=bool:?}, syscon: {=bool:?}, enableupdate: {:?} }}",
            self.rom(),
            self.ramx_ctrl(),
            self.ram0_ctrl(),
            self.ram1_ctrl(),
            self.ram2_ctrl(),
            self.ram3_ctrl(),
            self.ram4_ctrl(),
            self.sync0_apb(),
            self.sync1_apb(),
            self.crcgen(),
            self.sdma0(),
            self.sdma1(),
            self.usb0(),
            self.syscon(),
            self.enableupdate()
        )
    }
}
#[doc = "CLKOUT clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutdiv(pub u32);
impl Clkoutdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkoutdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkoutdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkoutdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkoutdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkoutdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkoutdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::ClkoutdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkoutdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::ClkoutdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Clkoutdiv {
    #[inline(always)]
    fn default() -> Clkoutdiv {
        Clkoutdiv(0)
    }
}
impl core::fmt::Debug for Clkoutdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkoutdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "CLKOUT clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutsel(pub u32);
impl Clkoutsel {
    #[doc = "CLKOUT clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::ClkoutselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::ClkoutselSel::from_bits(val as u8)
    }
    #[doc = "CLKOUT clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::ClkoutselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Clkoutsel {
    #[inline(always)]
    fn default() -> Clkoutsel {
        Clkoutsel(0)
    }
}
impl core::fmt::Debug for Clkoutsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkoutsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClockCtrl(pub u32);
impl ClockCtrl {
    #[doc = "Enable XTAL32MHz clock for Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal32mhz_freqm_ena(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub const fn set_xtal32mhz_freqm_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[must_use]
    #[inline(always)]
    pub const fn fro1mhz_utick_ena(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub const fn set_fro1mhz_utick_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable FRO 12MHz clock for Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn fro12mhz_freqm_ena(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub const fn set_fro12mhz_freqm_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable FRO 96MHz clock for Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn fro_hf_freqm_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub const fn set_fro_hf_freqm_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable clock_in clock for clock module."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin_ena(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clock_in clock for clock module."]
    #[inline(always)]
    pub const fn set_clkin_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[must_use]
    #[inline(always)]
    pub const fn fro1mhz_clk_ena(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub const fn set_fro1mhz_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn ana_fro12m_clk_ena(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub const fn set_ana_fro12m_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable clock for cristal oscilator calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn xo_cal_clk_ena(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub const fn set_xo_cal_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[must_use]
    #[inline(always)]
    pub const fn plu_deglitch_clk_ena(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub const fn set_plu_deglitch_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for ClockCtrl {
    #[inline(always)]
    fn default() -> ClockCtrl {
        ClockCtrl(0)
    }
}
impl core::fmt::Debug for ClockCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClockCtrl")
            .field("xtal32mhz_freqm_ena", &self.xtal32mhz_freqm_ena())
            .field("fro1mhz_utick_ena", &self.fro1mhz_utick_ena())
            .field("fro12mhz_freqm_ena", &self.fro12mhz_freqm_ena())
            .field("fro_hf_freqm_ena", &self.fro_hf_freqm_ena())
            .field("clkin_ena", &self.clkin_ena())
            .field("fro1mhz_clk_ena", &self.fro1mhz_clk_ena())
            .field("ana_fro12m_clk_ena", &self.ana_fro12m_clk_ena())
            .field("xo_cal_clk_ena", &self.xo_cal_clk_ena())
            .field("plu_deglitch_clk_ena", &self.plu_deglitch_clk_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClockCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClockCtrl {{ xtal32mhz_freqm_ena: {=bool:?}, fro1mhz_utick_ena: {=bool:?}, fro12mhz_freqm_ena: {=bool:?}, fro_hf_freqm_ena: {=bool:?}, clkin_ena: {=bool:?}, fro1mhz_clk_ena: {=bool:?}, ana_fro12m_clk_ena: {=bool:?}, xo_cal_clk_ena: {=bool:?}, plu_deglitch_clk_ena: {=bool:?} }}",
            self.xtal32mhz_freqm_ena(),
            self.fro1mhz_utick_ena(),
            self.fro12mhz_freqm_ena(),
            self.fro_hf_freqm_ena(),
            self.clkin_ena(),
            self.fro1mhz_clk_ena(),
            self.ana_fro12m_clk_ena(),
            self.xo_cal_clk_ena(),
            self.plu_deglitch_clk_ena()
        )
    }
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clockgenupdatelockout(pub u32);
impl Clockgenupdatelockout {
    #[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[must_use]
    #[inline(always)]
    pub const fn clockgenupdatelockout(&self) -> super::vals::Clockgenupdatelockout {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Clockgenupdatelockout::from_bits(val as u32)
    }
    #[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub const fn set_clockgenupdatelockout(&mut self, val: super::vals::Clockgenupdatelockout) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clockgenupdatelockout {
    #[inline(always)]
    fn default() -> Clockgenupdatelockout {
        Clockgenupdatelockout(0)
    }
}
impl core::fmt::Debug for Clockgenupdatelockout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clockgenupdatelockout")
            .field("clockgenupdatelockout", &self.clockgenupdatelockout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clockgenupdatelockout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clockgenupdatelockout {{ clockgenupdatelockout: {:?} }}",
            self.clockgenupdatelockout()
        )
    }
}
#[doc = "Comparator Interrupt control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompIntCtrl(pub u32);
impl CompIntCtrl {
    #[doc = "Analog Comparator interrupt enable control:."]
    #[must_use]
    #[inline(always)]
    pub const fn int_enable(&self) -> super::vals::IntEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntEnable::from_bits(val as u8)
    }
    #[doc = "Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub const fn set_int_enable(&mut self, val: super::vals::IntEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn int_clear(&self) -> super::vals::IntClear {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntClear::from_bits(val as u8)
    }
    #[doc = "Analog Comparator interrupt clear."]
    #[inline(always)]
    pub const fn set_int_clear(&mut self, val: super::vals::IntClear) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator interrupt type selector:."]
    #[must_use]
    #[inline(always)]
    pub const fn int_ctrl(&self) -> super::vals::IntCtrl {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::IntCtrl::from_bits(val as u8)
    }
    #[doc = "Comparator interrupt type selector:."]
    #[inline(always)]
    pub const fn set_int_ctrl(&mut self, val: super::vals::IntCtrl) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn int_source(&self) -> super::vals::IntSource {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::IntSource::from_bits(val as u8)
    }
    #[doc = "Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub const fn set_int_source(&mut self, val: super::vals::IntSource) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for CompIntCtrl {
    #[inline(always)]
    fn default() -> CompIntCtrl {
        CompIntCtrl(0)
    }
}
impl core::fmt::Debug for CompIntCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompIntCtrl")
            .field("int_enable", &self.int_enable())
            .field("int_clear", &self.int_clear())
            .field("int_ctrl", &self.int_ctrl())
            .field("int_source", &self.int_source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompIntCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CompIntCtrl {{ int_enable: {:?}, int_clear: {:?}, int_ctrl: {:?}, int_source: {:?} }}",
            self.int_enable(),
            self.int_clear(),
            self.int_ctrl(),
            self.int_source()
        )
    }
}
#[doc = "Comparator Interrupt status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompIntStatus(pub u32);
impl CompIntStatus {
    #[doc = "Interrupt status BEFORE Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::Status {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Status::from_bits(val as u8)
    }
    #[doc = "Interrupt status BEFORE Interrupt Enable."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: super::vals::Status) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status AFTER Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn int_status(&self) -> super::vals::IntStatus {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntStatus::from_bits(val as u8)
    }
    #[doc = "Interrupt status AFTER Interrupt Enable."]
    #[inline(always)]
    pub const fn set_int_status(&mut self, val: super::vals::IntStatus) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "comparator analog output."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> super::vals::Val {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Val::from_bits(val as u8)
    }
    #[doc = "comparator analog output."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: super::vals::Val) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for CompIntStatus {
    #[inline(always)]
    fn default() -> CompIntStatus {
        CompIntStatus(0)
    }
}
impl core::fmt::Debug for CompIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompIntStatus")
            .field("status", &self.status())
            .field("int_status", &self.int_status())
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CompIntStatus {{ status: {:?}, int_status: {:?}, val: {:?} }}",
            self.status(),
            self.int_status(),
            self.val()
        )
    }
}
#[doc = "Coprocessor Boot Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpboot(pub u32);
impl Cpboot {
    #[doc = "Coprocessor Boot Address for CPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn cpboot(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub const fn set_cpboot(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpboot {
    #[inline(always)]
    fn default() -> Cpboot {
        Cpboot(0)
    }
}
impl core::fmt::Debug for Cpboot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpboot")
            .field("cpboot", &self.cpboot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpboot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpboot {{ cpboot: {=u32:?} }}", self.cpboot())
    }
}
#[doc = "CPU Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpstat(pub u32);
impl Cpstat {
    #[doc = "The CPU0 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> super::vals::Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "The CPU0 sleeping state."]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: super::vals::Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The CPU1 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1sleeping(&self) -> super::vals::Cpu1sleeping {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cpu1sleeping::from_bits(val as u8)
    }
    #[doc = "The CPU1 sleeping state."]
    #[inline(always)]
    pub const fn set_cpu1sleeping(&mut self, val: super::vals::Cpu1sleeping) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "The CPU0 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> super::vals::Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "The CPU0 lockup state."]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: super::vals::Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "The CPU1 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1lockup(&self) -> super::vals::Cpu1lockup {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cpu1lockup::from_bits(val as u8)
    }
    #[doc = "The CPU1 lockup state."]
    #[inline(always)]
    pub const fn set_cpu1lockup(&mut self, val: super::vals::Cpu1lockup) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Cpstat {
    #[inline(always)]
    fn default() -> Cpstat {
        Cpstat(0)
    }
}
impl core::fmt::Debug for Cpstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpstat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu1sleeping", &self.cpu1sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .field("cpu1lockup", &self.cpu1lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpstat {{ cpu0sleeping: {:?}, cpu1sleeping: {:?}, cpu0lockup: {:?}, cpu1lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu1sleeping(),
            self.cpu0lockup(),
            self.cpu1lockup()
        )
    }
}
#[doc = "System tick calibration for non-secure part of CPU0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0nstckcal(pub u32);
impl Cpu0nstckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Initial value for the Systick timer."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Initial value for the Systick timer."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0nstckcal {
    #[inline(always)]
    fn default() -> Cpu0nstckcal {
        Cpu0nstckcal(0)
    }
}
impl core::fmt::Debug for Cpu0nstckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0nstckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0nstckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {=bool:?}, noref: {=bool:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "System tick calibration for secure part of CPU0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0stckcal(pub u32);
impl Cpu0stckcal {
    #[doc = "Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Initial value for the Systick timer."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Initial value for the Systick timer."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0stckcal {
    #[inline(always)]
    fn default() -> Cpu0stckcal {
        Cpu0stckcal(0)
    }
}
impl core::fmt::Debug for Cpu0stckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0stckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0stckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0stckcal {{ tenms: {=u32:?}, skew: {=bool:?}, noref: {=bool:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "System tick calibration for CPU1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1stckcal(pub u32);
impl Cpu1stckcal {
    #[doc = "Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu1stckcal {
    #[inline(always)]
    fn default() -> Cpu1stckcal {
        Cpu1stckcal(0)
    }
}
impl core::fmt::Debug for Cpu1stckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu1stckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu1stckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu1stckcal {{ tenms: {=u32:?}, skew: {=bool:?}, noref: {=bool:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPUs configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpucfg(pub u32);
impl Cpucfg {
    #[doc = "Enable CPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable CPU1."]
    #[inline(always)]
    pub const fn set_cpu1enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cpucfg {
    #[inline(always)]
    fn default() -> Cpucfg {
        Cpucfg(0)
    }
}
impl core::fmt::Debug for Cpucfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpucfg")
            .field("cpu1enable", &self.cpu1enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpucfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpucfg {{ cpu1enable: {=bool:?} }}", self.cpu1enable())
    }
}
#[doc = "CPU Control for multiple processors"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuctrl(pub u32);
impl Cpuctrl {
    #[doc = "CPU1 clock enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1clken(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 clock enable."]
    #[inline(always)]
    pub const fn set_cpu1clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CPU1 reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1rsten(&self) -> super::vals::Cpu1rsten {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cpu1rsten::from_bits(val as u8)
    }
    #[doc = "CPU1 reset."]
    #[inline(always)]
    pub const fn set_cpu1rsten(&mut self, val: super::vals::Cpu1rsten) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Cpuctrl {
    #[inline(always)]
    fn default() -> Cpuctrl {
        Cpuctrl(0)
    }
}
impl core::fmt::Debug for Cpuctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpuctrl")
            .field("cpu1clken", &self.cpu1clken())
            .field("cpu1rsten", &self.cpu1rsten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpuctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpuctrl {{ cpu1clken: {=bool:?}, cpu1rsten: {:?} }}",
            self.cpu1clken(),
            self.cpu1rsten()
        )
    }
}
#[doc = "CTimer 0 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel0(pub u32);
impl Ctimerclksel0 {
    #[doc = "CTimer 0 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ctimerclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctimerclksel0Sel::from_bits(val as u8)
    }
    #[doc = "CTimer 0 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ctimerclksel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Ctimerclksel0 {
    #[inline(always)]
    fn default() -> Ctimerclksel0 {
        Ctimerclksel0(0)
    }
}
impl core::fmt::Debug for Ctimerclksel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel0")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel0 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CTimer 1 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel1(pub u32);
impl Ctimerclksel1 {
    #[doc = "CTimer 1 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ctimerclksel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctimerclksel1Sel::from_bits(val as u8)
    }
    #[doc = "CTimer 1 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ctimerclksel1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Ctimerclksel1 {
    #[inline(always)]
    fn default() -> Ctimerclksel1 {
        Ctimerclksel1(0)
    }
}
impl core::fmt::Debug for Ctimerclksel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel1")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel1 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CTimer 2 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel2(pub u32);
impl Ctimerclksel2 {
    #[doc = "CTimer 2 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ctimerclksel2Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctimerclksel2Sel::from_bits(val as u8)
    }
    #[doc = "CTimer 2 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ctimerclksel2Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Ctimerclksel2 {
    #[inline(always)]
    fn default() -> Ctimerclksel2 {
        Ctimerclksel2(0)
    }
}
impl core::fmt::Debug for Ctimerclksel2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel2")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel2 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CTimer 3 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel3(pub u32);
impl Ctimerclksel3 {
    #[doc = "CTimer 3 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ctimerclksel3Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctimerclksel3Sel::from_bits(val as u8)
    }
    #[doc = "CTimer 3 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ctimerclksel3Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Ctimerclksel3 {
    #[inline(always)]
    fn default() -> Ctimerclksel3 {
        Ctimerclksel3(0)
    }
}
impl core::fmt::Debug for Ctimerclksel3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel3")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel3 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CTimer 4 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel4(pub u32);
impl Ctimerclksel4 {
    #[doc = "CTimer 4 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ctimerclksel4Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ctimerclksel4Sel::from_bits(val as u8)
    }
    #[doc = "CTimer 4 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ctimerclksel4Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Ctimerclksel4 {
    #[inline(always)]
    fn default() -> Ctimerclksel4 {
        Ctimerclksel4(0)
    }
}
impl core::fmt::Debug for Ctimerclksel4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel4")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel4 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkselx0(pub u32);
impl Ctimerclkselx0 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ctimerclkselx0 {
    #[inline(always)]
    fn default() -> Ctimerclkselx0 {
        Ctimerclkselx0(0)
    }
}
impl core::fmt::Debug for Ctimerclkselx0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkselx0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkselx0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclkselx0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkselx1(pub u32);
impl Ctimerclkselx1 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ctimerclkselx1 {
    #[inline(always)]
    fn default() -> Ctimerclkselx1 {
        Ctimerclkselx1(0)
    }
}
impl core::fmt::Debug for Ctimerclkselx1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkselx1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkselx1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclkselx1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkselx2(pub u32);
impl Ctimerclkselx2 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ctimerclkselx2 {
    #[inline(always)]
    fn default() -> Ctimerclkselx2 {
        Ctimerclkselx2(0)
    }
}
impl core::fmt::Debug for Ctimerclkselx2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkselx2")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkselx2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclkselx2 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkselx3(pub u32);
impl Ctimerclkselx3 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ctimerclkselx3 {
    #[inline(always)]
    fn default() -> Ctimerclkselx3 {
        Ctimerclkselx3(0)
    }
}
impl core::fmt::Debug for Ctimerclkselx3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkselx3")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkselx3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclkselx3 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkselx4(pub u32);
impl Ctimerclkselx4 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ctimerclkselx4 {
    #[inline(always)]
    fn default() -> Ctimerclkselx4 {
        Ctimerclkselx4(0)
    }
}
impl core::fmt::Debug for Ctimerclkselx4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkselx4")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkselx4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclkselx4 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Debug authentication BEACON register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugAuthBeacon(pub u32);
impl DebugAuthBeacon {
    #[doc = "Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[must_use]
    #[inline(always)]
    pub const fn beacon(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub const fn set_beacon(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DebugAuthBeacon {
    #[inline(always)]
    fn default() -> DebugAuthBeacon {
        DebugAuthBeacon(0)
    }
}
impl core::fmt::Debug for DebugAuthBeacon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugAuthBeacon")
            .field("beacon", &self.beacon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugAuthBeacon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugAuthBeacon {{ beacon: {=u32:?} }}", self.beacon())
    }
}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeatures(pub u32);
impl DebugFeatures {
    #[doc = "CPU0 Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> super::vals::DebugFeaturesCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DebugFeaturesCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: super::vals::DebugFeaturesCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> super::vals::DebugFeaturesCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DebugFeaturesCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: super::vals::DebugFeaturesCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CPU1 Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> super::vals::DebugFeaturesCpu1Dbgen {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DebugFeaturesCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: super::vals::DebugFeaturesCpu1Dbgen) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU1 Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> super::vals::DebugFeaturesCpu1Niden {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::DebugFeaturesCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: super::vals::DebugFeaturesCpu1Niden) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for DebugFeatures {
    #[inline(always)]
    fn default() -> DebugFeatures {
        DebugFeatures(0)
    }
}
impl core::fmt::Debug for DebugFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeatures")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .field("cpu0_spiden", &self.cpu0_spiden())
            .field("cpu0_spniden", &self.cpu0_spniden())
            .field("cpu1_dbgen", &self.cpu1_dbgen())
            .field("cpu1_niden", &self.cpu1_niden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeatures {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?}, cpu1_dbgen: {:?}, cpu1_niden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden(),
            self.cpu1_dbgen(),
            self.cpu1_niden()
        )
    }
}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeaturesDp(pub u32);
impl DebugFeaturesDp {
    #[doc = "CPU0 (CPU0) Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesDpCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 (CPU0) Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesDpCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesDpCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesDpCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> super::vals::DebugFeaturesDpCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: super::vals::DebugFeaturesDpCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> super::vals::DebugFeaturesDpCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: super::vals::DebugFeaturesDpCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CPU1 Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> super::vals::DebugFeaturesDpCpu1Dbgen {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DebugFeaturesDpCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: super::vals::DebugFeaturesDpCpu1Dbgen) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU1 Non Invasive debug control:."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> super::vals::DebugFeaturesDpCpu1Niden {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::DebugFeaturesDpCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 Non Invasive debug control:."]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: super::vals::DebugFeaturesDpCpu1Niden) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for DebugFeaturesDp {
    #[inline(always)]
    fn default() -> DebugFeaturesDp {
        DebugFeaturesDp(0)
    }
}
impl core::fmt::Debug for DebugFeaturesDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeaturesDp")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .field("cpu0_spiden", &self.cpu0_spiden())
            .field("cpu0_spniden", &self.cpu0_spniden())
            .field("cpu1_dbgen", &self.cpu1_dbgen())
            .field("cpu1_niden", &self.cpu1_niden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeaturesDp {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?}, cpu1_dbgen: {:?}, cpu1_niden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden(),
            self.cpu1_dbgen(),
            self.cpu1_niden()
        )
    }
}
#[doc = "Control write access to security registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLockEn(pub u32);
impl DebugLockEn {
    #[doc = "Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_all(&self) -> super::vals::LockAll {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LockAll::from_bits(val as u8)
    }
    #[doc = "Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline(always)]
    pub const fn set_lock_all(&mut self, val: super::vals::LockAll) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DebugLockEn {
    #[inline(always)]
    fn default() -> DebugLockEn {
        DebugLockEn(0)
    }
}
impl core::fmt::Debug for DebugLockEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugLockEn")
            .field("lock_all", &self.lock_all())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugLockEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugLockEn {{ lock_all: {:?} }}", self.lock_all())
    }
}
#[doc = "Device ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceId0(pub u32);
impl DeviceId0 {
    #[doc = "ROM revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rev_minor(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM revision."]
    #[inline(always)]
    pub const fn set_rom_rev_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for DeviceId0 {
    #[inline(always)]
    fn default() -> DeviceId0 {
        DeviceId0(0)
    }
}
impl core::fmt::Debug for DeviceId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceId0")
            .field("rom_rev_minor", &self.rom_rev_minor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceId0 {{ rom_rev_minor: {=u8:?} }}",
            self.rom_rev_minor()
        )
    }
}
#[doc = "Chip revision ID and Number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieid(pub u32);
impl Dieid {
    #[doc = "Chip Metal Revision ID."]
    #[must_use]
    #[inline(always)]
    pub const fn rev_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip Metal Revision ID."]
    #[inline(always)]
    pub const fn set_rev_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Chip Number 0x426B."]
    #[must_use]
    #[inline(always)]
    pub const fn mco_num_in_die_id(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Chip Number 0x426B."]
    #[inline(always)]
    pub const fn set_mco_num_in_die_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 4usize)) | (((val as u32) & 0x000f_ffff) << 4usize);
    }
}
impl Default for Dieid {
    #[inline(always)]
    fn default() -> Dieid {
        Dieid(0)
    }
}
impl core::fmt::Debug for Dieid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieid")
            .field("rev_id", &self.rev_id())
            .field("mco_num_in_die_id", &self.mco_num_in_die_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieid {{ rev_id: {=u8:?}, mco_num_in_die_id: {=u32:?} }}",
            self.rev_id(),
            self.mco_num_in_die_id()
        )
    }
}
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcclksel(pub u32);
impl Fcclksel {
    #[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FcclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FcclkselSel::from_bits(val as u8)
    }
    #[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::FcclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Fcclksel {
    #[inline(always)]
    fn default() -> Fcclksel {
        Fcclksel(0)
    }
}
impl core::fmt::Debug for Fcclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fcclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcclkselx(pub u32);
impl Fcclkselx {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fcclkselx {
    #[inline(always)]
    fn default() -> Fcclkselx {
        Fcclkselx(0)
    }
}
impl core::fmt::Debug for Fcclkselx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcclkselx")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcclkselx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fcclkselx {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Fractional rate divider for flexcomm 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexfrgctrl(pub u32);
impl Flexfrgctrl {
    #[doc = "Denominator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional rate divider."]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Flexfrgctrl {
    #[inline(always)]
    fn default() -> Flexfrgctrl {
        Flexfrgctrl(0)
    }
}
impl core::fmt::Debug for Flexfrgctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexfrgctrl")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexfrgctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexfrgctrl {{ div: {=u8:?}, mult: {=u8:?} }}",
            self.div(),
            self.mult()
        )
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexfrgxctrl(pub u32);
impl Flexfrgxctrl {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Flexfrgxctrl {
    #[inline(always)]
    fn default() -> Flexfrgxctrl {
        Flexfrgxctrl(0)
    }
}
impl core::fmt::Debug for Flexfrgxctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexfrgxctrl")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexfrgxctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexfrgxctrl {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "FMC configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmccr(pub u32);
impl Fmccr {
    #[doc = "Instruction fetch configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn fetchcfg(&self) -> super::vals::Fetchcfg {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Fetchcfg::from_bits(val as u8)
    }
    #[doc = "Instruction fetch configuration."]
    #[inline(always)]
    pub const fn set_fetchcfg(&mut self, val: super::vals::Fetchcfg) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Data read configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn datacfg(&self) -> super::vals::Datacfg {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Datacfg::from_bits(val as u8)
    }
    #[doc = "Data read configuration."]
    #[inline(always)]
    pub const fn set_datacfg(&mut self, val: super::vals::Datacfg) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Acceleration enable."]
    #[must_use]
    #[inline(always)]
    pub const fn accel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Acceleration enable."]
    #[inline(always)]
    pub const fn set_accel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Prefetch enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable."]
    #[inline(always)]
    pub const fn set_prefen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Prefetch override."]
    #[must_use]
    #[inline(always)]
    pub const fn prefovr(&self) -> super::vals::Prefovr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Prefovr::from_bits(val as u8)
    }
    #[doc = "Prefetch override."]
    #[inline(always)]
    pub const fn set_prefovr(&mut self, val: super::vals::Prefovr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Flash memory access time."]
    #[must_use]
    #[inline(always)]
    pub const fn flashtim(&self) -> super::vals::Flashtim {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Flashtim::from_bits(val as u8)
    }
    #[doc = "Flash memory access time."]
    #[inline(always)]
    pub const fn set_flashtim(&mut self, val: super::vals::Flashtim) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Fmccr {
    #[inline(always)]
    fn default() -> Fmccr {
        Fmccr(0)
    }
}
impl core::fmt::Debug for Fmccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmccr")
            .field("fetchcfg", &self.fetchcfg())
            .field("datacfg", &self.datacfg())
            .field("accel", &self.accel())
            .field("prefen", &self.prefen())
            .field("prefovr", &self.prefovr())
            .field("flashtim", &self.flashtim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fmccr {{ fetchcfg: {:?}, datacfg: {:?}, accel: {=bool:?}, prefen: {=bool:?}, prefovr: {:?}, flashtim: {:?} }}",
            self.fetchcfg(),
            self.datacfg(),
            self.accel(),
            self.prefen(),
            self.prefovr(),
            self.flashtim()
        )
    }
}
#[doc = "FMCflush control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmcflush(pub u32);
impl Fmcflush {
    #[doc = "Flush control"]
    #[must_use]
    #[inline(always)]
    pub const fn flush(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush control"]
    #[inline(always)]
    pub const fn set_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Fmcflush {
    #[inline(always)]
    fn default() -> Fmcflush {
        Fmcflush(0)
    }
}
impl core::fmt::Debug for Fmcflush {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmcflush")
            .field("flush", &self.flush())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmcflush {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fmcflush {{ flush: {=bool:?} }}", self.flush())
    }
}
#[doc = "FRO_HF (96MHz) clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FrohfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FrohfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FrohfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::FrohfdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FrohfdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::FrohfdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frohfdiv {
    #[inline(always)]
    fn default() -> Frohfdiv {
        Frohfdiv(0)
    }
}
impl core::fmt::Debug for Frohfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frohfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "Functional retention control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Funcretentionctrl(pub u32);
impl Funcretentionctrl {
    #[doc = "functional retention in power down only."]
    #[must_use]
    #[inline(always)]
    pub const fn funcretena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "functional retention in power down only."]
    #[inline(always)]
    pub const fn set_funcretena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start address divided by 4 inside SRAMX bank."]
    #[must_use]
    #[inline(always)]
    pub const fn ret_start(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x1fff;
        val as u16
    }
    #[doc = "Start address divided by 4 inside SRAMX bank."]
    #[inline(always)]
    pub const fn set_ret_start(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 1usize)) | (((val as u32) & 0x1fff) << 1usize);
    }
    #[doc = "lenth of Scan chains to save."]
    #[must_use]
    #[inline(always)]
    pub const fn ret_lenth(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x03ff;
        val as u16
    }
    #[doc = "lenth of Scan chains to save."]
    #[inline(always)]
    pub const fn set_ret_lenth(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
    }
}
impl Default for Funcretentionctrl {
    #[inline(always)]
    fn default() -> Funcretentionctrl {
        Funcretentionctrl(0)
    }
}
impl core::fmt::Debug for Funcretentionctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Funcretentionctrl")
            .field("funcretena", &self.funcretena())
            .field("ret_start", &self.ret_start())
            .field("ret_lenth", &self.ret_lenth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Funcretentionctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Funcretentionctrl {{ funcretena: {=bool:?}, ret_start: {=u16:?}, ret_lenth: {=u16:?} }}",
            self.funcretena(),
            self.ret_start(),
            self.ret_lenth()
        )
    }
}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpiopsync(pub u32);
impl Gpiopsync {
    #[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[must_use]
    #[inline(always)]
    pub const fn psync(&self) -> super::vals::Psync {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Psync::from_bits(val as u8)
    }
    #[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub const fn set_psync(&mut self, val: super::vals::Psync) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpiopsync {
    #[inline(always)]
    fn default() -> Gpiopsync {
        Gpiopsync(0)
    }
}
impl core::fmt::Debug for Gpiopsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpiopsync")
            .field("psync", &self.psync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpiopsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpiopsync {{ psync: {:?} }}", self.psync())
    }
}
#[doc = "HS LSPI clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hslspiclksel(pub u32);
impl Hslspiclksel {
    #[doc = "HS LSPI clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::HslspiclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::HslspiclkselSel::from_bits(val as u8)
    }
    #[doc = "HS LSPI clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::HslspiclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Hslspiclksel {
    #[inline(always)]
    fn default() -> Hslspiclksel {
        Hslspiclksel(0)
    }
}
impl core::fmt::Debug for Hslspiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hslspiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hslspiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hslspiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "block quiddikey/PUF all index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyBlock(pub u32);
impl KeyBlock {
    #[doc = "Write a value to block quiddikey/PUF all index."]
    #[must_use]
    #[inline(always)]
    pub const fn key_block(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write a value to block quiddikey/PUF all index."]
    #[inline(always)]
    pub const fn set_key_block(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KeyBlock {
    #[inline(always)]
    fn default() -> KeyBlock {
        KeyBlock(0)
    }
}
impl core::fmt::Debug for KeyBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeyBlock")
            .field("key_block", &self.key_block())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeyBlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KeyBlock {{ key_block: {=u32:?} }}", self.key_block())
    }
}
#[doc = "Main clock A source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclksela(pub u32);
impl Mainclksela {
    #[doc = "Main clock A source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MainclkselaSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MainclkselaSel::from_bits(val as u8)
    }
    #[doc = "Main clock A source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::MainclkselaSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Mainclksela {
    #[inline(always)]
    fn default() -> Mainclksela {
        Mainclksela(0)
    }
}
impl core::fmt::Debug for Mainclksela {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mainclksela")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mainclksela {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mainclksela {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Main clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclkselb(pub u32);
impl Mainclkselb {
    #[doc = "Main clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MainclkselbSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MainclkselbSel::from_bits(val as u8)
    }
    #[doc = "Main clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::MainclkselbSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Mainclkselb {
    #[inline(always)]
    fn default() -> Mainclkselb {
        Mainclkselb(0)
    }
}
impl core::fmt::Debug for Mainclkselb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mainclkselb")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mainclkselb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mainclkselb {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "MCLK clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mclkclksel(pub u32);
impl Mclkclksel {
    #[doc = "MCLK clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MclkclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MclkclkselSel::from_bits(val as u8)
    }
    #[doc = "MCLK clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::MclkclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Mclkclksel {
    #[inline(always)]
    fn default() -> Mclkclksel {
        Mclkclksel(0)
    }
}
impl core::fmt::Debug for Mclkclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mclkclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mclkclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mclkclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I2S MCLK clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mclkdiv(pub u32);
impl Mclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::MclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::MclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mclkdiv {
    #[inline(always)]
    fn default() -> Mclkdiv {
        Mclkdiv(0)
    }
}
impl core::fmt::Debug for Mclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "MCLK control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mclkio(pub u32);
impl Mclkio {
    #[doc = "MCLK control."]
    #[must_use]
    #[inline(always)]
    pub const fn mclkio(&self) -> super::vals::Mclkio {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mclkio::from_bits(val as u8)
    }
    #[doc = "MCLK control."]
    #[inline(always)]
    pub const fn set_mclkio(&mut self, val: super::vals::Mclkio) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mclkio {
    #[inline(always)]
    fn default() -> Mclkio {
        Mclkio(0)
    }
}
impl core::fmt::Debug for Mclkio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mclkio")
            .field("mclkio", &self.mclkio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mclkio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mclkio {{ mclkio: {:?} }}", self.mclkio())
    }
}
#[doc = "Memory Remap control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memoryremap(pub u32);
impl Memoryremap {
    #[doc = "Select the location of the vector table :."]
    #[must_use]
    #[inline(always)]
    pub const fn map(&self) -> super::vals::Map {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Map::from_bits(val as u8)
    }
    #[doc = "Select the location of the vector table :."]
    #[inline(always)]
    pub const fn set_map(&mut self, val: super::vals::Map) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Memoryremap {
    #[inline(always)]
    fn default() -> Memoryremap {
        Memoryremap(0)
    }
}
impl core::fmt::Debug for Memoryremap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Memoryremap")
            .field("map", &self.map())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Memoryremap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Memoryremap {{ map: {:?} }}", self.map())
    }
}
#[doc = "NMI Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    pub const fn set_irqcpu1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    pub const fn set_nmiencpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_nmiencpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nmisrc {
    #[inline(always)]
    fn default() -> Nmisrc {
        Nmisrc(0)
    }
}
impl core::fmt::Debug for Nmisrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisrc")
            .field("irqcpu0", &self.irqcpu0())
            .field("irqcpu1", &self.irqcpu1())
            .field("nmiencpu1", &self.nmiencpu1())
            .field("nmiencpu0", &self.nmiencpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmisrc {{ irqcpu0: {=u8:?}, irqcpu1: {=u8:?}, nmiencpu1: {=bool:?}, nmiencpu0: {=bool:?} }}",
            self.irqcpu0(),
            self.irqcpu1(),
            self.nmiencpu1(),
            self.nmiencpu0()
        )
    }
}
#[doc = "PLL0 clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0clkdiv(pub u32);
impl Pll0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Pll0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pll0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Pll0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Pll0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pll0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Pll0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::Pll0clkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pll0clkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::Pll0clkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll0clkdiv {
    #[inline(always)]
    fn default() -> Pll0clkdiv {
        Pll0clkdiv(0)
    }
}
impl core::fmt::Debug for Pll0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "PLL0 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0clksel(pub u32);
impl Pll0clksel {
    #[doc = "PLL0 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Pll0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pll0clkselSel::from_bits(val as u8)
    }
    #[doc = "PLL0 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Pll0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Pll0clksel {
    #[inline(always)]
    fn default() -> Pll0clksel {
        Pll0clksel(0)
    }
}
impl core::fmt::Debug for Pll0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pll0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "PLL0 550m control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0ctrl(pub u32);
impl Pll0ctrl {
    #[doc = "Bandwidth select R value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspll(&self) -> super::vals::Pll0ctrlBypasspll {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pll0ctrlBypasspll::from_bits(val as u8)
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub const fn set_bypasspll(&mut self, val: super::vals::Pll0ctrlBypasspll) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> super::vals::Pll0ctrlBypasspostdiv2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pll0ctrlBypasspostdiv2::from_bits(val as u8)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: super::vals::Pll0ctrlBypasspostdiv2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bwdirect(&self) -> super::vals::Pll0ctrlBwdirect {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pll0ctrlBwdirect::from_bits(val as u8)
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_bwdirect(&mut self, val: super::vals::Pll0ctrlBwdirect) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "bypass of the pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> super::vals::Pll0ctrlBypassprediv {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pll0ctrlBypassprediv::from_bits(val as u8)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: super::vals::Pll0ctrlBypassprediv) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "bypass of the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> super::vals::Pll0ctrlBypasspostdiv {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pll0ctrlBypasspostdiv::from_bits(val as u8)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: super::vals::Pll0ctrlBypasspostdiv) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "enable the output clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clken(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable the output clock."]
    #[inline(always)]
    pub const fn set_clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "free running mode."]
    #[must_use]
    #[inline(always)]
    pub const fn frmen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "free running mode."]
    #[inline(always)]
    pub const fn set_frmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn frmclkstable(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable =1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub const fn set_frmclkstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "skew mode."]
    #[must_use]
    #[inline(always)]
    pub const fn skewen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "skew mode."]
    #[inline(always)]
    pub const fn set_skewen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Pll0ctrl {
    #[inline(always)]
    fn default() -> Pll0ctrl {
        Pll0ctrl(0)
    }
}
impl core::fmt::Debug for Pll0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0ctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspll", &self.bypasspll())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("bwdirect", &self.bwdirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("clken", &self.clken())
            .field("frmen", &self.frmen())
            .field("frmclkstable", &self.frmclkstable())
            .field("skewen", &self.skewen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll0ctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspll: {:?}, bypasspostdiv2: {:?}, limupoff: {=bool:?}, bwdirect: {:?}, bypassprediv: {:?}, bypasspostdiv: {:?}, clken: {=bool:?}, frmen: {=bool:?}, frmclkstable: {=bool:?}, skewen: {=bool:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspll(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.bwdirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.clken(),
            self.frmen(),
            self.frmclkstable(),
            self.skewen()
        )
    }
}
#[doc = "PLL0 550m N divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0ndec(pub u32);
impl Pll0ndec {
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pre-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Pll0ndec {
    #[inline(always)]
    fn default() -> Pll0ndec {
        Pll0ndec(0)
    }
}
impl core::fmt::Debug for Pll0ndec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0ndec")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0ndec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll0ndec {{ ndiv: {=u8:?}, nreq: {=bool:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "PLL0 550m P divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0pdec(pub u32);
impl Pll0pdec {
    #[doc = "post-divider divider ratio (P-divider)"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "feedback ratio change request."]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Pll0pdec {
    #[inline(always)]
    fn default() -> Pll0pdec {
        Pll0pdec(0)
    }
}
impl core::fmt::Debug for Pll0pdec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0pdec")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0pdec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll0pdec {{ pdiv: {=u8:?}, preq: {=bool:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0sscg0(pub u32);
impl Pll0sscg0 {
    #[doc = "input word of the wrapper bit 31 to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn md_lbs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub const fn set_md_lbs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pll0sscg0 {
    #[inline(always)]
    fn default() -> Pll0sscg0 {
        Pll0sscg0(0)
    }
}
impl core::fmt::Debug for Pll0sscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0sscg0")
            .field("md_lbs", &self.md_lbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0sscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pll0sscg0 {{ md_lbs: {=u32:?} }}", self.md_lbs())
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0sscg1(pub u32);
impl Pll0sscg1 {
    #[doc = "input word of the wrapper bit 32."]
    #[must_use]
    #[inline(always)]
    pub const fn md_mbs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "input word of the wrapper bit 32."]
    #[inline(always)]
    pub const fn set_md_mbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "md change request."]
    #[must_use]
    #[inline(always)]
    pub const fn md_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "md change request."]
    #[inline(always)]
    pub const fn set_md_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "to select an external mdiv value."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv_ext(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0xffff;
        val as u16
    }
    #[doc = "to select an external mdiv value."]
    #[inline(always)]
    pub const fn set_mdiv_ext(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 10usize)) | (((val as u32) & 0xffff) << 10usize);
    }
    #[doc = "to select an external mreq value."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "to select an external mreq value."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ext(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub const fn set_sel_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Pll0sscg1 {
    #[inline(always)]
    fn default() -> Pll0sscg1 {
        Pll0sscg1(0)
    }
}
impl core::fmt::Debug for Pll0sscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0sscg1")
            .field("md_mbs", &self.md_mbs())
            .field("md_req", &self.md_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("mdiv_ext", &self.mdiv_ext())
            .field("mreq", &self.mreq())
            .field("dither", &self.dither())
            .field("sel_ext", &self.sel_ext())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0sscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll0sscg1 {{ md_mbs: {=bool:?}, md_req: {=bool:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {=u8:?}, mdiv_ext: {=u16:?}, mreq: {=bool:?}, dither: {=bool:?}, sel_ext: {=bool:?} }}",
            self.md_mbs(),
            self.md_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.mdiv_ext(),
            self.mreq(),
            self.dither(),
            self.sel_ext()
        )
    }
}
#[doc = "PLL0 550m status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll0stat(pub u32);
impl Pll0stat {
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn predivack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_predivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn feeddivack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_feeddivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn postdivack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_postdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "free running detector output (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn frmdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "free running detector output (active high)."]
    #[inline(always)]
    pub const fn set_frmdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Pll0stat {
    #[inline(always)]
    fn default() -> Pll0stat {
        Pll0stat(0)
    }
}
impl core::fmt::Debug for Pll0stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll0stat")
            .field("lock", &self.lock())
            .field("predivack", &self.predivack())
            .field("feeddivack", &self.feeddivack())
            .field("postdivack", &self.postdivack())
            .field("frmdet", &self.frmdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll0stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll0stat {{ lock: {=bool:?}, predivack: {=bool:?}, feeddivack: {=bool:?}, postdivack: {=bool:?}, frmdet: {=bool:?} }}",
            self.lock(),
            self.predivack(),
            self.feeddivack(),
            self.postdivack(),
            self.frmdet()
        )
    }
}
#[doc = "PLL1 clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clksel(pub u32);
impl Pll1clksel {
    #[doc = "PLL1 clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Pll1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pll1clkselSel::from_bits(val as u8)
    }
    #[doc = "PLL1 clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Pll1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Pll1clksel {
    #[inline(always)]
    fn default() -> Pll1clksel {
        Pll1clksel(0)
    }
}
impl core::fmt::Debug for Pll1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pll1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "PLL1 550m control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1ctrl(pub u32);
impl Pll1ctrl {
    #[doc = "Bandwidth select R value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspll(&self) -> super::vals::Pll1ctrlBypasspll {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pll1ctrlBypasspll::from_bits(val as u8)
    }
    #[doc = "Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub const fn set_bypasspll(&mut self, val: super::vals::Pll1ctrlBypasspll) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> super::vals::Pll1ctrlBypasspostdiv2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pll1ctrlBypasspostdiv2::from_bits(val as u8)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: super::vals::Pll1ctrlBypasspostdiv2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bwdirect(&self) -> super::vals::Pll1ctrlBwdirect {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pll1ctrlBwdirect::from_bits(val as u8)
    }
    #[doc = "control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_bwdirect(&mut self, val: super::vals::Pll1ctrlBwdirect) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "bypass of the pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> super::vals::Pll1ctrlBypassprediv {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pll1ctrlBypassprediv::from_bits(val as u8)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: super::vals::Pll1ctrlBypassprediv) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "bypass of the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> super::vals::Pll1ctrlBypasspostdiv {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pll1ctrlBypasspostdiv::from_bits(val as u8)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: super::vals::Pll1ctrlBypasspostdiv) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "enable the output clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clken(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable the output clock."]
    #[inline(always)]
    pub const fn set_clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "1: free running mode."]
    #[must_use]
    #[inline(always)]
    pub const fn frmen(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "1: free running mode."]
    #[inline(always)]
    pub const fn set_frmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn frmclkstable(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub const fn set_frmclkstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Skew mode."]
    #[must_use]
    #[inline(always)]
    pub const fn skewen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Skew mode."]
    #[inline(always)]
    pub const fn set_skewen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Pll1ctrl {
    #[inline(always)]
    fn default() -> Pll1ctrl {
        Pll1ctrl(0)
    }
}
impl core::fmt::Debug for Pll1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1ctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspll", &self.bypasspll())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("bwdirect", &self.bwdirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("clken", &self.clken())
            .field("frmen", &self.frmen())
            .field("frmclkstable", &self.frmclkstable())
            .field("skewen", &self.skewen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1ctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspll: {:?}, bypasspostdiv2: {:?}, limupoff: {=bool:?}, bwdirect: {:?}, bypassprediv: {:?}, bypasspostdiv: {:?}, clken: {=bool:?}, frmen: {=bool:?}, frmclkstable: {=bool:?}, skewen: {=bool:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspll(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.bwdirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.clken(),
            self.frmen(),
            self.frmclkstable(),
            self.skewen()
        )
    }
}
#[doc = "PLL1 550m M divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1mdec(pub u32);
impl Pll1mdec {
    #[doc = "feedback divider divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Pll1mdec {
    #[inline(always)]
    fn default() -> Pll1mdec {
        Pll1mdec(0)
    }
}
impl core::fmt::Debug for Pll1mdec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1mdec")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1mdec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1mdec {{ mdiv: {=u16:?}, mreq: {=bool:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "PLL1 550m N divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1ndec(pub u32);
impl Pll1ndec {
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "pre-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Pll1ndec {
    #[inline(always)]
    fn default() -> Pll1ndec {
        Pll1ndec(0)
    }
}
impl core::fmt::Debug for Pll1ndec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1ndec")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1ndec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1ndec {{ ndiv: {=u8:?}, nreq: {=bool:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "PLL1 550m P divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1pdec(pub u32);
impl Pll1pdec {
    #[doc = "post-divider divider ratio (P-divider)"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "feedback ratio change request."]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Pll1pdec {
    #[inline(always)]
    fn default() -> Pll1pdec {
        Pll1pdec(0)
    }
}
impl core::fmt::Debug for Pll1pdec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1pdec")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1pdec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1pdec {{ pdiv: {=u8:?}, preq: {=bool:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "PLL1 550m status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1stat(pub u32);
impl Pll1stat {
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\] :100 kHz to 20 MHz."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn predivack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_predivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn feeddivack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_feeddivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn postdivack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "post-divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_postdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "free running detector output (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn frmdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "free running detector output (active high)."]
    #[inline(always)]
    pub const fn set_frmdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Pll1stat {
    #[inline(always)]
    fn default() -> Pll1stat {
        Pll1stat(0)
    }
}
impl core::fmt::Debug for Pll1stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1stat")
            .field("lock", &self.lock())
            .field("predivack", &self.predivack())
            .field("feeddivack", &self.feeddivack())
            .field("postdivack", &self.postdivack())
            .field("frmdet", &self.frmdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1stat {{ lock: {=bool:?}, predivack: {=bool:?}, feeddivack: {=bool:?}, postdivack: {=bool:?}, frmdet: {=bool:?} }}",
            self.lock(),
            self.predivack(),
            self.feeddivack(),
            self.postdivack(),
            self.frmdet()
        )
    }
}
#[doc = "Peripheral reset control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl0(pub u32);
impl Presetctrl0 {
    #[doc = "ROM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rst(&self) -> super::vals::RomRst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RomRst::from_bits(val as u8)
    }
    #[doc = "ROM reset control."]
    #[inline(always)]
    pub const fn set_rom_rst(&mut self, val: super::vals::RomRst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SRAM Controller 1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl1_rst(&self) -> super::vals::SramCtrl1Rst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SramCtrl1Rst::from_bits(val as u8)
    }
    #[doc = "SRAM Controller 1 reset control."]
    #[inline(always)]
    pub const fn set_sram_ctrl1_rst(&mut self, val: super::vals::SramCtrl1Rst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SRAM Controller 2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl2_rst(&self) -> super::vals::SramCtrl2Rst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SramCtrl2Rst::from_bits(val as u8)
    }
    #[doc = "SRAM Controller 2 reset control."]
    #[inline(always)]
    pub const fn set_sram_ctrl2_rst(&mut self, val: super::vals::SramCtrl2Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SRAM Controller 3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl3_rst(&self) -> super::vals::SramCtrl3Rst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SramCtrl3Rst::from_bits(val as u8)
    }
    #[doc = "SRAM Controller 3 reset control."]
    #[inline(always)]
    pub const fn set_sram_ctrl3_rst(&mut self, val: super::vals::SramCtrl3Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SRAM Controller 4 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_ctrl4_rst(&self) -> super::vals::SramCtrl4Rst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SramCtrl4Rst::from_bits(val as u8)
    }
    #[doc = "SRAM Controller 4 reset control."]
    #[inline(always)]
    pub const fn set_sram_ctrl4_rst(&mut self, val: super::vals::SramCtrl4Rst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Flash controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_rst(&self) -> super::vals::FlashRst {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::FlashRst::from_bits(val as u8)
    }
    #[doc = "Flash controller reset control."]
    #[inline(always)]
    pub const fn set_flash_rst(&mut self, val: super::vals::FlashRst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FMC controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc_rst(&self) -> super::vals::FmcRst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FmcRst::from_bits(val as u8)
    }
    #[doc = "FMC controller reset control."]
    #[inline(always)]
    pub const fn set_fmc_rst(&mut self, val: super::vals::FmcRst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Input Mux reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_rst(&self) -> super::vals::MuxRst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MuxRst::from_bits(val as u8)
    }
    #[doc = "Input Mux reset control."]
    #[inline(always)]
    pub const fn set_mux_rst(&mut self, val: super::vals::MuxRst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "I/O controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn iocon_rst(&self) -> super::vals::IoconRst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::IoconRst::from_bits(val as u8)
    }
    #[doc = "I/O controller reset control."]
    #[inline(always)]
    pub const fn set_iocon_rst(&mut self, val: super::vals::IoconRst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_rst(&self) -> super::vals::Gpio0Rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Gpio0Rst::from_bits(val as u8)
    }
    #[doc = "GPIO0 reset control."]
    #[inline(always)]
    pub const fn set_gpio0_rst(&mut self, val: super::vals::Gpio0Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_rst(&self) -> super::vals::Gpio1Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Gpio1Rst::from_bits(val as u8)
    }
    #[doc = "GPIO1 reset control."]
    #[inline(always)]
    pub const fn set_gpio1_rst(&mut self, val: super::vals::Gpio1Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_rst(&self) -> super::vals::Gpio2Rst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpio2Rst::from_bits(val as u8)
    }
    #[doc = "GPIO2 reset control."]
    #[inline(always)]
    pub const fn set_gpio2_rst(&mut self, val: super::vals::Gpio2Rst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIO3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_rst(&self) -> super::vals::Gpio3Rst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gpio3Rst::from_bits(val as u8)
    }
    #[doc = "GPIO3 reset control."]
    #[inline(always)]
    pub const fn set_gpio3_rst(&mut self, val: super::vals::Gpio3Rst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Pin interrupt (PINT) reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pint_rst(&self) -> super::vals::PintRst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PintRst::from_bits(val as u8)
    }
    #[doc = "Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub const fn set_pint_rst(&mut self, val: super::vals::PintRst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Group interrupt (GINT) reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gint_rst(&self) -> super::vals::GintRst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::GintRst::from_bits(val as u8)
    }
    #[doc = "Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub const fn set_gint_rst(&mut self, val: super::vals::GintRst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_rst(&self) -> super::vals::Dma0Rst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dma0Rst::from_bits(val as u8)
    }
    #[doc = "DMA0 reset control."]
    #[inline(always)]
    pub const fn set_dma0_rst(&mut self, val: super::vals::Dma0Rst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "CRCGEN reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn crcgen_rst(&self) -> super::vals::CrcgenRst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::CrcgenRst::from_bits(val as u8)
    }
    #[doc = "CRCGEN reset control."]
    #[inline(always)]
    pub const fn set_crcgen_rst(&mut self, val: super::vals::CrcgenRst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Watchdog Timer reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt_rst(&self) -> super::vals::WwdtRst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::WwdtRst::from_bits(val as u8)
    }
    #[doc = "Watchdog Timer reset control."]
    #[inline(always)]
    pub const fn set_wwdt_rst(&mut self, val: super::vals::WwdtRst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Real Time Clock (RTC) reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_rst(&self) -> super::vals::RtcRst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RtcRst::from_bits(val as u8)
    }
    #[doc = "Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub const fn set_rtc_rst(&mut self, val: super::vals::RtcRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Inter CPU communication Mailbox reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox_rst(&self) -> super::vals::MailboxRst {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MailboxRst::from_bits(val as u8)
    }
    #[doc = "Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub const fn set_mailbox_rst(&mut self, val: super::vals::MailboxRst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_rst(&self) -> super::vals::AdcRst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::AdcRst::from_bits(val as u8)
    }
    #[doc = "ADC reset control."]
    #[inline(always)]
    pub const fn set_adc_rst(&mut self, val: super::vals::AdcRst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Presetctrl0 {
    #[inline(always)]
    fn default() -> Presetctrl0 {
        Presetctrl0(0)
    }
}
impl core::fmt::Debug for Presetctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl0")
            .field("rom_rst", &self.rom_rst())
            .field("sram_ctrl1_rst", &self.sram_ctrl1_rst())
            .field("sram_ctrl2_rst", &self.sram_ctrl2_rst())
            .field("sram_ctrl3_rst", &self.sram_ctrl3_rst())
            .field("sram_ctrl4_rst", &self.sram_ctrl4_rst())
            .field("flash_rst", &self.flash_rst())
            .field("fmc_rst", &self.fmc_rst())
            .field("mux_rst", &self.mux_rst())
            .field("iocon_rst", &self.iocon_rst())
            .field("gpio0_rst", &self.gpio0_rst())
            .field("gpio1_rst", &self.gpio1_rst())
            .field("gpio2_rst", &self.gpio2_rst())
            .field("gpio3_rst", &self.gpio3_rst())
            .field("pint_rst", &self.pint_rst())
            .field("gint_rst", &self.gint_rst())
            .field("dma0_rst", &self.dma0_rst())
            .field("crcgen_rst", &self.crcgen_rst())
            .field("wwdt_rst", &self.wwdt_rst())
            .field("rtc_rst", &self.rtc_rst())
            .field("mailbox_rst", &self.mailbox_rst())
            .field("adc_rst", &self.adc_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl0 {{ rom_rst: {:?}, sram_ctrl1_rst: {:?}, sram_ctrl2_rst: {:?}, sram_ctrl3_rst: {:?}, sram_ctrl4_rst: {:?}, flash_rst: {:?}, fmc_rst: {:?}, mux_rst: {:?}, iocon_rst: {:?}, gpio0_rst: {:?}, gpio1_rst: {:?}, gpio2_rst: {:?}, gpio3_rst: {:?}, pint_rst: {:?}, gint_rst: {:?}, dma0_rst: {:?}, crcgen_rst: {:?}, wwdt_rst: {:?}, rtc_rst: {:?}, mailbox_rst: {:?}, adc_rst: {:?} }}",
            self.rom_rst(),
            self.sram_ctrl1_rst(),
            self.sram_ctrl2_rst(),
            self.sram_ctrl3_rst(),
            self.sram_ctrl4_rst(),
            self.flash_rst(),
            self.fmc_rst(),
            self.mux_rst(),
            self.iocon_rst(),
            self.gpio0_rst(),
            self.gpio1_rst(),
            self.gpio2_rst(),
            self.gpio3_rst(),
            self.pint_rst(),
            self.gint_rst(),
            self.dma0_rst(),
            self.crcgen_rst(),
            self.wwdt_rst(),
            self.rtc_rst(),
            self.mailbox_rst(),
            self.adc_rst()
        )
    }
}
#[doc = "Peripheral reset control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl1(pub u32);
impl Presetctrl1 {
    #[doc = "MRT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt_rst(&self) -> super::vals::MrtRst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MrtRst::from_bits(val as u8)
    }
    #[doc = "MRT reset control."]
    #[inline(always)]
    pub const fn set_mrt_rst(&mut self, val: super::vals::MrtRst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "OS Event Timer reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer_rst(&self) -> super::vals::OstimerRst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OstimerRst::from_bits(val as u8)
    }
    #[doc = "OS Event Timer reset control."]
    #[inline(always)]
    pub const fn set_ostimer_rst(&mut self, val: super::vals::OstimerRst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SCT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sct_rst(&self) -> super::vals::SctRst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SctRst::from_bits(val as u8)
    }
    #[doc = "SCT reset control."]
    #[inline(always)]
    pub const fn set_sct_rst(&mut self, val: super::vals::SctRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SCTIPU reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sctipu_rst(&self) -> super::vals::SctipuRst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SctipuRst::from_bits(val as u8)
    }
    #[doc = "SCTIPU reset control."]
    #[inline(always)]
    pub const fn set_sctipu_rst(&mut self, val: super::vals::SctipuRst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "UTICK reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn utick_rst(&self) -> super::vals::UtickRst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::UtickRst::from_bits(val as u8)
    }
    #[doc = "UTICK reset control."]
    #[inline(always)]
    pub const fn set_utick_rst(&mut self, val: super::vals::UtickRst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FC0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn fc_rst(&self, n: usize) -> super::vals::FcRst {
        assert!(n < 8usize);
        let offs = 11usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::FcRst::from_bits(val as u8)
    }
    #[doc = "FC0 reset control."]
    #[inline(always)]
    pub const fn set_fc_rst(&mut self, n: usize, val: super::vals::FcRst) {
        assert!(n < 8usize);
        let offs = 11usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Timer 2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer2_rst(&self) -> super::vals::Timer2Rst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Timer2Rst::from_bits(val as u8)
    }
    #[doc = "Timer 2 reset control."]
    #[inline(always)]
    pub const fn set_timer2_rst(&mut self, val: super::vals::Timer2Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB0 DEV reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_dev_rst(&self) -> super::vals::Usb0DevRst {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Usb0DevRst::from_bits(val as u8)
    }
    #[doc = "USB0 DEV reset control."]
    #[inline(always)]
    pub const fn set_usb0_dev_rst(&mut self, val: super::vals::Usb0DevRst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Timer 0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_rst(&self) -> super::vals::Timer0Rst {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Timer0Rst::from_bits(val as u8)
    }
    #[doc = "Timer 0 reset control."]
    #[inline(always)]
    pub const fn set_timer0_rst(&mut self, val: super::vals::Timer0Rst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Timer 1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_rst(&self) -> super::vals::Timer1Rst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Timer1Rst::from_bits(val as u8)
    }
    #[doc = "Timer 1 reset control."]
    #[inline(always)]
    pub const fn set_timer1_rst(&mut self, val: super::vals::Timer1Rst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Presetctrl1 {
    #[inline(always)]
    fn default() -> Presetctrl1 {
        Presetctrl1(0)
    }
}
impl core::fmt::Debug for Presetctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl1")
            .field("mrt_rst", &self.mrt_rst())
            .field("ostimer_rst", &self.ostimer_rst())
            .field("sct_rst", &self.sct_rst())
            .field("sctipu_rst", &self.sctipu_rst())
            .field("utick_rst", &self.utick_rst())
            .field("fc_rst[0]", &self.fc_rst(0usize))
            .field("fc_rst[1]", &self.fc_rst(1usize))
            .field("fc_rst[2]", &self.fc_rst(2usize))
            .field("fc_rst[3]", &self.fc_rst(3usize))
            .field("fc_rst[4]", &self.fc_rst(4usize))
            .field("fc_rst[5]", &self.fc_rst(5usize))
            .field("fc_rst[6]", &self.fc_rst(6usize))
            .field("fc_rst[7]", &self.fc_rst(7usize))
            .field("timer2_rst", &self.timer2_rst())
            .field("usb0_dev_rst", &self.usb0_dev_rst())
            .field("timer0_rst", &self.timer0_rst())
            .field("timer1_rst", &self.timer1_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl1 {{ mrt_rst: {:?}, ostimer_rst: {:?}, sct_rst: {:?}, sctipu_rst: {:?}, utick_rst: {:?}, fc_rst[0]: {:?}, fc_rst[1]: {:?}, fc_rst[2]: {:?}, fc_rst[3]: {:?}, fc_rst[4]: {:?}, fc_rst[5]: {:?}, fc_rst[6]: {:?}, fc_rst[7]: {:?}, timer2_rst: {:?}, usb0_dev_rst: {:?}, timer0_rst: {:?}, timer1_rst: {:?} }}",
            self.mrt_rst(),
            self.ostimer_rst(),
            self.sct_rst(),
            self.sctipu_rst(),
            self.utick_rst(),
            self.fc_rst(0usize),
            self.fc_rst(1usize),
            self.fc_rst(2usize),
            self.fc_rst(3usize),
            self.fc_rst(4usize),
            self.fc_rst(5usize),
            self.fc_rst(6usize),
            self.fc_rst(7usize),
            self.timer2_rst(),
            self.usb0_dev_rst(),
            self.timer0_rst(),
            self.timer1_rst()
        )
    }
}
#[doc = "Peripheral reset control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl2(pub u32);
impl Presetctrl2 {
    #[doc = "DMA1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_rst(&self) -> super::vals::Dma1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dma1Rst::from_bits(val as u8)
    }
    #[doc = "DMA1 reset control."]
    #[inline(always)]
    pub const fn set_dma1_rst(&mut self, val: super::vals::Dma1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn comp_rst(&self) -> super::vals::CompRst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CompRst::from_bits(val as u8)
    }
    #[doc = "Comparator reset control."]
    #[inline(always)]
    pub const fn set_comp_rst(&mut self, val: super::vals::CompRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_rst(&self) -> super::vals::SdioRst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SdioRst::from_bits(val as u8)
    }
    #[doc = "SDIO reset control."]
    #[inline(always)]
    pub const fn set_sdio_rst(&mut self, val: super::vals::SdioRst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "USB1 Host reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_host_rst(&self) -> super::vals::Usb1HostRst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Usb1HostRst::from_bits(val as u8)
    }
    #[doc = "USB1 Host reset control."]
    #[inline(always)]
    pub const fn set_usb1_host_rst(&mut self, val: super::vals::Usb1HostRst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "USB1 dev reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_dev_rst(&self) -> super::vals::Usb1DevRst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Usb1DevRst::from_bits(val as u8)
    }
    #[doc = "USB1 dev reset control."]
    #[inline(always)]
    pub const fn set_usb1_dev_rst(&mut self, val: super::vals::Usb1DevRst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "USB1 RAM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_ram_rst(&self) -> super::vals::Usb1RamRst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Usb1RamRst::from_bits(val as u8)
    }
    #[doc = "USB1 RAM reset control."]
    #[inline(always)]
    pub const fn set_usb1_ram_rst(&mut self, val: super::vals::Usb1RamRst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "USB1 PHY reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy_rst(&self) -> super::vals::Usb1PhyRst {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Usb1PhyRst::from_bits(val as u8)
    }
    #[doc = "USB1 PHY reset control."]
    #[inline(always)]
    pub const fn set_usb1_phy_rst(&mut self, val: super::vals::Usb1PhyRst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Frequency meter reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_rst(&self) -> super::vals::FreqmeRst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FreqmeRst::from_bits(val as u8)
    }
    #[doc = "Frequency meter reset control."]
    #[inline(always)]
    pub const fn set_freqme_rst(&mut self, val: super::vals::FreqmeRst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RNG reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn rng_rst(&self) -> super::vals::RngRst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::RngRst::from_bits(val as u8)
    }
    #[doc = "RNG reset control."]
    #[inline(always)]
    pub const fn set_rng_rst(&mut self, val: super::vals::RngRst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "SYSCTL Block reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sysctl_rst(&self) -> super::vals::SysctlRst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SysctlRst::from_bits(val as u8)
    }
    #[doc = "SYSCTL Block reset."]
    #[inline(always)]
    pub const fn set_sysctl_rst(&mut self, val: super::vals::SysctlRst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USB0 Host Master reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_hostm_rst(&self) -> super::vals::Usb0HostmRst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Usb0HostmRst::from_bits(val as u8)
    }
    #[doc = "USB0 Host Master reset control."]
    #[inline(always)]
    pub const fn set_usb0_hostm_rst(&mut self, val: super::vals::Usb0HostmRst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB0 Host Slave reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_hosts_rst(&self) -> super::vals::Usb0HostsRst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Usb0HostsRst::from_bits(val as u8)
    }
    #[doc = "USB0 Host Slave reset control."]
    #[inline(always)]
    pub const fn set_usb0_hosts_rst(&mut self, val: super::vals::Usb0HostsRst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "HASH_AES reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn hash_aes_rst(&self) -> super::vals::HashAesRst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HashAesRst::from_bits(val as u8)
    }
    #[doc = "HASH_AES reset control."]
    #[inline(always)]
    pub const fn set_hash_aes_rst(&mut self, val: super::vals::HashAesRst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Quad reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pq_rst(&self) -> super::vals::PqRst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PqRst::from_bits(val as u8)
    }
    #[doc = "Power Quad reset control."]
    #[inline(always)]
    pub const fn set_pq_rst(&mut self, val: super::vals::PqRst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "PLU LUT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn plulut_rst(&self) -> super::vals::PlulutRst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PlulutRst::from_bits(val as u8)
    }
    #[doc = "PLU LUT reset control."]
    #[inline(always)]
    pub const fn set_plulut_rst(&mut self, val: super::vals::PlulutRst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Timer 3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer3_rst(&self) -> super::vals::Timer3Rst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Timer3Rst::from_bits(val as u8)
    }
    #[doc = "Timer 3 reset control."]
    #[inline(always)]
    pub const fn set_timer3_rst(&mut self, val: super::vals::Timer3Rst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Timer 4 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer4_rst(&self) -> super::vals::Timer4Rst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Timer4Rst::from_bits(val as u8)
    }
    #[doc = "Timer 4 reset control."]
    #[inline(always)]
    pub const fn set_timer4_rst(&mut self, val: super::vals::Timer4Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "PUF reset control reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_rst(&self) -> super::vals::PufRst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PufRst::from_bits(val as u8)
    }
    #[doc = "PUF reset control reset control."]
    #[inline(always)]
    pub const fn set_puf_rst(&mut self, val: super::vals::PufRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Casper reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn casper_rst(&self) -> super::vals::CasperRst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::CasperRst::from_bits(val as u8)
    }
    #[doc = "Casper reset control."]
    #[inline(always)]
    pub const fn set_casper_rst(&mut self, val: super::vals::CasperRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "analog control reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn analog_ctrl_rst(&self) -> super::vals::AnalogCtrlRst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::AnalogCtrlRst::from_bits(val as u8)
    }
    #[doc = "analog control reset control."]
    #[inline(always)]
    pub const fn set_analog_ctrl_rst(&mut self, val: super::vals::AnalogCtrlRst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "HS LSPI reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn hs_lspi_rst(&self) -> super::vals::HsLspiRst {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::HsLspiRst::from_bits(val as u8)
    }
    #[doc = "HS LSPI reset control."]
    #[inline(always)]
    pub const fn set_hs_lspi_rst(&mut self, val: super::vals::HsLspiRst) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPIO secure reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sec_rst(&self) -> super::vals::GpioSecRst {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::GpioSecRst::from_bits(val as u8)
    }
    #[doc = "GPIO secure reset control."]
    #[inline(always)]
    pub const fn set_gpio_sec_rst(&mut self, val: super::vals::GpioSecRst) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "GPIO secure int reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sec_int_rst(&self) -> super::vals::GpioSecIntRst {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpioSecIntRst::from_bits(val as u8)
    }
    #[doc = "GPIO secure int reset control."]
    #[inline(always)]
    pub const fn set_gpio_sec_int_rst(&mut self, val: super::vals::GpioSecIntRst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Presetctrl2 {
    #[inline(always)]
    fn default() -> Presetctrl2 {
        Presetctrl2(0)
    }
}
impl core::fmt::Debug for Presetctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl2")
            .field("dma1_rst", &self.dma1_rst())
            .field("comp_rst", &self.comp_rst())
            .field("sdio_rst", &self.sdio_rst())
            .field("usb1_host_rst", &self.usb1_host_rst())
            .field("usb1_dev_rst", &self.usb1_dev_rst())
            .field("usb1_ram_rst", &self.usb1_ram_rst())
            .field("usb1_phy_rst", &self.usb1_phy_rst())
            .field("freqme_rst", &self.freqme_rst())
            .field("rng_rst", &self.rng_rst())
            .field("sysctl_rst", &self.sysctl_rst())
            .field("usb0_hostm_rst", &self.usb0_hostm_rst())
            .field("usb0_hosts_rst", &self.usb0_hosts_rst())
            .field("hash_aes_rst", &self.hash_aes_rst())
            .field("pq_rst", &self.pq_rst())
            .field("plulut_rst", &self.plulut_rst())
            .field("timer3_rst", &self.timer3_rst())
            .field("timer4_rst", &self.timer4_rst())
            .field("puf_rst", &self.puf_rst())
            .field("casper_rst", &self.casper_rst())
            .field("analog_ctrl_rst", &self.analog_ctrl_rst())
            .field("hs_lspi_rst", &self.hs_lspi_rst())
            .field("gpio_sec_rst", &self.gpio_sec_rst())
            .field("gpio_sec_int_rst", &self.gpio_sec_int_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl2 {{ dma1_rst: {:?}, comp_rst: {:?}, sdio_rst: {:?}, usb1_host_rst: {:?}, usb1_dev_rst: {:?}, usb1_ram_rst: {:?}, usb1_phy_rst: {:?}, freqme_rst: {:?}, rng_rst: {:?}, sysctl_rst: {:?}, usb0_hostm_rst: {:?}, usb0_hosts_rst: {:?}, hash_aes_rst: {:?}, pq_rst: {:?}, plulut_rst: {:?}, timer3_rst: {:?}, timer4_rst: {:?}, puf_rst: {:?}, casper_rst: {:?}, analog_ctrl_rst: {:?}, hs_lspi_rst: {:?}, gpio_sec_rst: {:?}, gpio_sec_int_rst: {:?} }}",
            self.dma1_rst(),
            self.comp_rst(),
            self.sdio_rst(),
            self.usb1_host_rst(),
            self.usb1_dev_rst(),
            self.usb1_ram_rst(),
            self.usb1_phy_rst(),
            self.freqme_rst(),
            self.rng_rst(),
            self.sysctl_rst(),
            self.usb0_hostm_rst(),
            self.usb0_hosts_rst(),
            self.hash_aes_rst(),
            self.pq_rst(),
            self.plulut_rst(),
            self.timer3_rst(),
            self.timer4_rst(),
            self.puf_rst(),
            self.casper_rst(),
            self.analog_ctrl_rst(),
            self.hs_lspi_rst(),
            self.gpio_sec_rst(),
            self.gpio_sec_int_rst()
        )
    }
}
#[doc = "Peripheral reset control clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrlclr(pub u32);
impl Presetctrlclr {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Presetctrlclr {
    #[inline(always)]
    fn default() -> Presetctrlclr {
        Presetctrlclr(0)
    }
}
impl core::fmt::Debug for Presetctrlclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrlclr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrlclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Presetctrlclr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrlset(pub u32);
impl Presetctrlset {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Presetctrlset {
    #[inline(always)]
    fn default() -> Presetctrlset {
        Presetctrlset(0)
    }
}
impl core::fmt::Debug for Presetctrlset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrlset")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrlset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Presetctrlset {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "SCT/PWM clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctclkdiv(pub u32);
impl Sctclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::SctclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SctclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::SctclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::SctclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SctclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::SctclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::SctclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SctclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::SctclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sctclkdiv {
    #[inline(always)]
    fn default() -> Sctclkdiv {
        Sctclkdiv(0)
    }
}
impl core::fmt::Debug for Sctclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "SCTimer/PWM clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctclksel(pub u32);
impl Sctclksel {
    #[doc = "SCTimer/PWM clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SctclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SctclkselSel::from_bits(val as u8)
    }
    #[doc = "SCTimer/PWM clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SctclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sctclksel {
    #[inline(always)]
    fn default() -> Sctclksel {
        Sctclksel(0)
    }
}
impl core::fmt::Debug for Sctclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sctclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SDIO CCLKIN phase and delay control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdioclkctrl(pub u32);
impl Sdioclkctrl {
    #[doc = "Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk_drv_phase(&self) -> super::vals::CclkDrvPhase {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CclkDrvPhase::from_bits(val as u8)
    }
    #[doc = "Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub const fn set_cclk_drv_phase(&mut self, val: super::vals::CclkDrvPhase) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk_sample_phase(&self) -> super::vals::CclkSamplePhase {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CclkSamplePhase::from_bits(val as u8)
    }
    #[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub const fn set_cclk_sample_phase(&mut self, val: super::vals::CclkSamplePhase) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[must_use]
    #[inline(always)]
    pub const fn phase_active(&self) -> super::vals::PhaseActive {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PhaseActive::from_bits(val as u8)
    }
    #[doc = "Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub const fn set_phase_active(&mut self, val: super::vals::PhaseActive) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk_drv_delay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub const fn set_cclk_drv_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk_drv_delay_active(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub const fn set_cclk_drv_delay_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk_sample_delay(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub const fn set_cclk_sample_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[must_use]
    #[inline(always)]
    pub const fn cclk_sample_delay_active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub const fn set_cclk_sample_delay_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdioclkctrl {
    #[inline(always)]
    fn default() -> Sdioclkctrl {
        Sdioclkctrl(0)
    }
}
impl core::fmt::Debug for Sdioclkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdioclkctrl")
            .field("cclk_drv_phase", &self.cclk_drv_phase())
            .field("cclk_sample_phase", &self.cclk_sample_phase())
            .field("phase_active", &self.phase_active())
            .field("cclk_drv_delay", &self.cclk_drv_delay())
            .field("cclk_drv_delay_active", &self.cclk_drv_delay_active())
            .field("cclk_sample_delay", &self.cclk_sample_delay())
            .field("cclk_sample_delay_active", &self.cclk_sample_delay_active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdioclkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdioclkctrl {{ cclk_drv_phase: {:?}, cclk_sample_phase: {:?}, phase_active: {:?}, cclk_drv_delay: {=u8:?}, cclk_drv_delay_active: {=bool:?}, cclk_sample_delay: {=u8:?}, cclk_sample_delay_active: {=bool:?} }}",
            self.cclk_drv_phase(),
            self.cclk_sample_phase(),
            self.phase_active(),
            self.cclk_drv_delay(),
            self.cclk_drv_delay_active(),
            self.cclk_sample_delay(),
            self.cclk_sample_delay_active()
        )
    }
}
#[doc = "SDIO clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdioclkdiv(pub u32);
impl Sdioclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::SdioclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SdioclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::SdioclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::SdioclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SdioclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::SdioclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::SdioclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdioclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::SdioclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdioclkdiv {
    #[inline(always)]
    fn default() -> Sdioclkdiv {
        Sdioclkdiv(0)
    }
}
impl core::fmt::Debug for Sdioclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdioclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdioclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdioclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "SDIO clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdioclksel(pub u32);
impl Sdioclksel {
    #[doc = "SDIO clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SdioclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SdioclkselSel::from_bits(val as u8)
    }
    #[doc = "SDIO clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SdioclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sdioclksel {
    #[inline(always)]
    fn default() -> Sdioclksel {
        Sdioclksel(0)
    }
}
impl core::fmt::Debug for Sdioclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdioclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdioclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sdioclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "generate a software_reset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwrReset(pub u32);
impl SwrReset {
    #[doc = "Write 0x5A00_0001 to generate a software_reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swr_reset(&self) -> super::vals::SwrReset {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SwrReset::from_bits(val as u32)
    }
    #[doc = "Write 0x5A00_0001 to generate a software_reset."]
    #[inline(always)]
    pub const fn set_swr_reset(&mut self, val: super::vals::SwrReset) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwrReset {
    #[inline(always)]
    fn default() -> SwrReset {
        SwrReset(0)
    }
}
impl core::fmt::Debug for SwrReset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwrReset")
            .field("swr_reset", &self.swr_reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwrReset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwrReset {{ swr_reset: {:?} }}", self.swr_reset())
    }
}
#[doc = "System Tick Timer divider for CPU0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkdiv0(pub u32);
impl Systickclkdiv0 {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Systickclkdiv0Reset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Systickclkdiv0Reset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Systickclkdiv0Reset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Systickclkdiv0Halt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Systickclkdiv0Halt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Systickclkdiv0Halt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::Systickclkdiv0Reqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Systickclkdiv0Reqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::Systickclkdiv0Reqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Systickclkdiv0 {
    #[inline(always)]
    fn default() -> Systickclkdiv0 {
        Systickclkdiv0(0)
    }
}
impl core::fmt::Debug for Systickclkdiv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclkdiv0")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkdiv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickclkdiv0 {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "System Tick Timer divider for CPU1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkdiv1(pub u32);
impl Systickclkdiv1 {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Systickclkdiv1Reset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Systickclkdiv1Reset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Systickclkdiv1Reset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Systickclkdiv1Halt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Systickclkdiv1Halt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Systickclkdiv1Halt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::Systickclkdiv1Reqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Systickclkdiv1Reqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::Systickclkdiv1Reqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Systickclkdiv1 {
    #[inline(always)]
    fn default() -> Systickclkdiv1 {
        Systickclkdiv1(0)
    }
}
impl core::fmt::Debug for Systickclkdiv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclkdiv1")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkdiv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickclkdiv1 {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "System Tick Timer for CPU0 source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclksel0(pub u32);
impl Systickclksel0 {
    #[doc = "System Tick Timer for CPU0 source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Systickclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Systickclksel0Sel::from_bits(val as u8)
    }
    #[doc = "System Tick Timer for CPU0 source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Systickclksel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Systickclksel0 {
    #[inline(always)]
    fn default() -> Systickclksel0 {
        Systickclksel0(0)
    }
}
impl core::fmt::Debug for Systickclksel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclksel0")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclksel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickclksel0 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "System Tick Timer for CPU1 source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclksel1(pub u32);
impl Systickclksel1 {
    #[doc = "System Tick Timer for CPU1 source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Systickclksel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Systickclksel1Sel::from_bits(val as u8)
    }
    #[doc = "System Tick Timer for CPU1 source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Systickclksel1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Systickclksel1 {
    #[inline(always)]
    fn default() -> Systickclksel1 {
        Systickclksel1(0)
    }
}
impl core::fmt::Debug for Systickclksel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclksel1")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclksel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickclksel1 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkselx0(pub u32);
impl Systickclkselx0 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Systickclkselx0 {
    #[inline(always)]
    fn default() -> Systickclkselx0 {
        Systickclkselx0(0)
    }
}
impl core::fmt::Debug for Systickclkselx0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclkselx0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkselx0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickclkselx0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral reset control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkselx1(pub u32);
impl Systickclkselx1 {
    #[doc = "Data array value"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Systickclkselx1 {
    #[inline(always)]
    fn default() -> Systickclkselx1 {
        Systickclkselx1(0)
    }
}
impl core::fmt::Debug for Systickclkselx1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclkselx1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkselx1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickclkselx1 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "TRACE clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclkdiv(pub u32);
impl Traceclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::TraceclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::TraceclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::TraceclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::TraceclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::TraceclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::TraceclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::TraceclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TraceclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::TraceclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Traceclkdiv {
    #[inline(always)]
    fn default() -> Traceclkdiv {
        Traceclkdiv(0)
    }
}
impl core::fmt::Debug for Traceclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Traceclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Traceclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Traceclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "Trace clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclksel(pub u32);
impl Traceclksel {
    #[doc = "Trace clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::TraceclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TraceclkselSel::from_bits(val as u8)
    }
    #[doc = "Trace clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::TraceclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Traceclksel {
    #[inline(always)]
    fn default() -> Traceclksel {
        Traceclksel(0)
    }
}
impl core::fmt::Debug for Traceclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Traceclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Traceclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Traceclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "USB0 Clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0clkdiv(pub u32);
impl Usb0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Usb0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Usb0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Usb0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Usb0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Usb0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Usb0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::Usb0clkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usb0clkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::Usb0clkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb0clkdiv {
    #[inline(always)]
    fn default() -> Usb0clkdiv {
        Usb0clkdiv(0)
    }
}
impl core::fmt::Debug for Usb0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "FS USB clock source select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0clksel(pub u32);
impl Usb0clksel {
    #[doc = "FS USB clock source select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Usb0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb0clkselSel::from_bits(val as u8)
    }
    #[doc = "FS USB clock source select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Usb0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Usb0clksel {
    #[inline(always)]
    fn default() -> Usb0clksel {
        Usb0clksel(0)
    }
}
impl core::fmt::Debug for Usb0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usb0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "USB0 need clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0needclkctrl(pub u32);
impl Usb0needclkctrl {
    #[doc = "USB0 Device USB0_NEEDCLK signal control:."]
    #[must_use]
    #[inline(always)]
    pub const fn ap_fs_dev_needclk(&self) -> super::vals::ApFsDevNeedclk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ApFsDevNeedclk::from_bits(val as u8)
    }
    #[doc = "USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub const fn set_ap_fs_dev_needclk(&mut self, val: super::vals::ApFsDevNeedclk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[must_use]
    #[inline(always)]
    pub const fn pol_fs_dev_needclk(&self) -> super::vals::PolFsDevNeedclk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PolFsDevNeedclk::from_bits(val as u8)
    }
    #[doc = "USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub const fn set_pol_fs_dev_needclk(&mut self, val: super::vals::PolFsDevNeedclk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "USB0 Host USB0_NEEDCLK signal control:."]
    #[must_use]
    #[inline(always)]
    pub const fn ap_fs_host_needclk(&self) -> super::vals::ApFsHostNeedclk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ApFsHostNeedclk::from_bits(val as u8)
    }
    #[doc = "USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub const fn set_ap_fs_host_needclk(&mut self, val: super::vals::ApFsHostNeedclk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[must_use]
    #[inline(always)]
    pub const fn pol_fs_host_needclk(&self) -> super::vals::PolFsHostNeedclk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PolFsHostNeedclk::from_bits(val as u8)
    }
    #[doc = "USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub const fn set_pol_fs_host_needclk(&mut self, val: super::vals::PolFsHostNeedclk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Usb0needclkctrl {
    #[inline(always)]
    fn default() -> Usb0needclkctrl {
        Usb0needclkctrl(0)
    }
}
impl core::fmt::Debug for Usb0needclkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb0needclkctrl")
            .field("ap_fs_dev_needclk", &self.ap_fs_dev_needclk())
            .field("pol_fs_dev_needclk", &self.pol_fs_dev_needclk())
            .field("ap_fs_host_needclk", &self.ap_fs_host_needclk())
            .field("pol_fs_host_needclk", &self.pol_fs_host_needclk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0needclkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb0needclkctrl {{ ap_fs_dev_needclk: {:?}, pol_fs_dev_needclk: {:?}, ap_fs_host_needclk: {:?}, pol_fs_host_needclk: {:?} }}",
            self.ap_fs_dev_needclk(),
            self.pol_fs_dev_needclk(),
            self.ap_fs_host_needclk(),
            self.pol_fs_host_needclk()
        )
    }
}
#[doc = "USB0 need clock status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0needclkstat(pub u32);
impl Usb0needclkstat {
    #[doc = "USB0 Device USB0_NEEDCLK signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_needclk(&self) -> super::vals::Usb0needclkstatDevNeedclk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usb0needclkstatDevNeedclk::from_bits(val as u8)
    }
    #[doc = "USB0 Device USB0_NEEDCLK signal status:."]
    #[inline(always)]
    pub const fn set_dev_needclk(&mut self, val: super::vals::Usb0needclkstatDevNeedclk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB0 Host USB0_NEEDCLK signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn host_needclk(&self) -> super::vals::Usb0needclkstatHostNeedclk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Usb0needclkstatHostNeedclk::from_bits(val as u8)
    }
    #[doc = "USB0 Host USB0_NEEDCLK signal status:."]
    #[inline(always)]
    pub const fn set_host_needclk(&mut self, val: super::vals::Usb0needclkstatHostNeedclk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Usb0needclkstat {
    #[inline(always)]
    fn default() -> Usb0needclkstat {
        Usb0needclkstat(0)
    }
}
impl core::fmt::Debug for Usb0needclkstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb0needclkstat")
            .field("dev_needclk", &self.dev_needclk())
            .field("host_needclk", &self.host_needclk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0needclkstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb0needclkstat {{ dev_needclk: {:?}, host_needclk: {:?} }}",
            self.dev_needclk(),
            self.host_needclk()
        )
    }
}
#[doc = "USB1 need clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1needclkctrl(pub u32);
impl Usb1needclkctrl {
    #[doc = "USB1 Device need_clock signal control:"]
    #[must_use]
    #[inline(always)]
    pub const fn ap_hs_dev_needclk(&self) -> super::vals::ApHsDevNeedclk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ApHsDevNeedclk::from_bits(val as u8)
    }
    #[doc = "USB1 Device need_clock signal control:"]
    #[inline(always)]
    pub const fn set_ap_hs_dev_needclk(&mut self, val: super::vals::ApHsDevNeedclk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[must_use]
    #[inline(always)]
    pub const fn pol_hs_dev_needclk(&self) -> super::vals::PolHsDevNeedclk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PolHsDevNeedclk::from_bits(val as u8)
    }
    #[doc = "USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    pub const fn set_pol_hs_dev_needclk(&mut self, val: super::vals::PolHsDevNeedclk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "USB1 Host need clock signal control:"]
    #[must_use]
    #[inline(always)]
    pub const fn ap_hs_host_needclk(&self) -> super::vals::ApHsHostNeedclk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ApHsHostNeedclk::from_bits(val as u8)
    }
    #[doc = "USB1 Host need clock signal control:"]
    #[inline(always)]
    pub const fn set_ap_hs_host_needclk(&mut self, val: super::vals::ApHsHostNeedclk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn pol_hs_host_needclk(&self) -> super::vals::PolHsHostNeedclk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PolHsHostNeedclk::from_bits(val as u8)
    }
    #[doc = "USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub const fn set_pol_hs_host_needclk(&mut self, val: super::vals::PolHsHostNeedclk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Software override of device controller PHY wake up logic."]
    #[must_use]
    #[inline(always)]
    pub const fn hs_dev_wakeup_n(&self) -> super::vals::HsDevWakeupN {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HsDevWakeupN::from_bits(val as u8)
    }
    #[doc = "Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub const fn set_hs_dev_wakeup_n(&mut self, val: super::vals::HsDevWakeupN) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1needclkctrl {
    #[inline(always)]
    fn default() -> Usb1needclkctrl {
        Usb1needclkctrl(0)
    }
}
impl core::fmt::Debug for Usb1needclkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1needclkctrl")
            .field("ap_hs_dev_needclk", &self.ap_hs_dev_needclk())
            .field("pol_hs_dev_needclk", &self.pol_hs_dev_needclk())
            .field("ap_hs_host_needclk", &self.ap_hs_host_needclk())
            .field("pol_hs_host_needclk", &self.pol_hs_host_needclk())
            .field("hs_dev_wakeup_n", &self.hs_dev_wakeup_n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1needclkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1needclkctrl {{ ap_hs_dev_needclk: {:?}, pol_hs_dev_needclk: {:?}, ap_hs_host_needclk: {:?}, pol_hs_host_needclk: {:?}, hs_dev_wakeup_n: {:?} }}",
            self.ap_hs_dev_needclk(),
            self.pol_hs_dev_needclk(),
            self.ap_hs_host_needclk(),
            self.pol_hs_host_needclk(),
            self.hs_dev_wakeup_n()
        )
    }
}
#[doc = "USB1 need clock status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1needclkstat(pub u32);
impl Usb1needclkstat {
    #[doc = "USB1 Device need_clock signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_needclk(&self) -> super::vals::Usb1needclkstatDevNeedclk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usb1needclkstatDevNeedclk::from_bits(val as u8)
    }
    #[doc = "USB1 Device need_clock signal status:."]
    #[inline(always)]
    pub const fn set_dev_needclk(&mut self, val: super::vals::Usb1needclkstatDevNeedclk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB1 Host need_clock signal status:."]
    #[must_use]
    #[inline(always)]
    pub const fn host_needclk(&self) -> super::vals::Usb1needclkstatHostNeedclk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Usb1needclkstatHostNeedclk::from_bits(val as u8)
    }
    #[doc = "USB1 Host need_clock signal status:."]
    #[inline(always)]
    pub const fn set_host_needclk(&mut self, val: super::vals::Usb1needclkstatHostNeedclk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Usb1needclkstat {
    #[inline(always)]
    fn default() -> Usb1needclkstat {
        Usb1needclkstat(0)
    }
}
impl core::fmt::Debug for Usb1needclkstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1needclkstat")
            .field("dev_needclk", &self.dev_needclk())
            .field("host_needclk", &self.host_needclk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1needclkstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1needclkstat {{ dev_needclk: {:?}, host_needclk: {:?} }}",
            self.dev_needclk(),
            self.host_needclk()
        )
    }
}
#[doc = "WDT clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtclkdiv(pub u32);
impl Wdtclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::WdtclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::WdtclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::WdtclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::WdtclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::WdtclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::WdtclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> super::vals::WdtclkdivReqflag {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::WdtclkdivReqflag::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: super::vals::WdtclkdivReqflag) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Wdtclkdiv {
    #[inline(always)]
    fn default() -> Wdtclkdiv {
        Wdtclkdiv(0)
    }
}
impl core::fmt::Debug for Wdtclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdtclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdtclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdtclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, reqflag: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
