#[doc = "ADC0 Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0clkdiv(pub u32);
impl Adc0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Adc0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Adc0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Adc0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Adc0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Adc0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Adc0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Adc0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Adc0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Adc0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Adc0clkdiv {
    #[inline(always)]
    fn default() -> Adc0clkdiv {
        Adc0clkdiv(0)
    }
}
impl core::fmt::Debug for Adc0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC0 Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0clksel(pub u32);
impl Adc0clksel {
    #[doc = "Selects the ADC0 clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Adc0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the ADC0 clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Adc0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc0clksel {
    #[inline(always)]
    fn default() -> Adc0clksel {
        Adc0clksel(0)
    }
}
impl core::fmt::Debug for Adc0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "ADC1 Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1clkdiv(pub u32);
impl Adc1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Adc1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Adc1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Adc1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Adc1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Adc1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Adc1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Adc1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Adc1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Adc1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Adc1clkdiv {
    #[inline(always)]
    fn default() -> Adc1clkdiv {
        Adc1clkdiv(0)
    }
}
impl core::fmt::Debug for Adc1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC1 Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1clksel(pub u32);
impl Adc1clksel {
    #[doc = "Selects the ADC1 clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Adc1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the ADC1 clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Adc1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc1clksel {
    #[inline(always)]
    fn default() -> Adc1clksel {
        Adc1clksel(0)
    }
}
impl core::fmt::Debug for Adc1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "AHB Clock Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl0(pub u32);
impl Ahbclkctrl0 {
    #[doc = "Enables the clock for the ROM"]
    #[must_use]
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the ROM"]
    #[inline(always)]
    pub const fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the RAMB Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMB Controller"]
    #[inline(always)]
    pub const fn set_ramb_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for the RAMC Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_ctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMC Controller"]
    #[inline(always)]
    pub const fn set_ramc_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for the RAMD Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramd_ctrl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMD Controller"]
    #[inline(always)]
    pub const fn set_ramd_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for the RAME Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn rame_ctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAME Controller"]
    #[inline(always)]
    pub const fn set_rame_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for the RAMF Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramf_ctrl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMF Controller"]
    #[inline(always)]
    pub const fn set_ramf_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for the RAMG Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramg_ctrl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMG Controller"]
    #[inline(always)]
    pub const fn set_ramg_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the RAMH Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramh_ctrl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMH Controller"]
    #[inline(always)]
    pub const fn set_ramh_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for the Flash Management Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn fmu(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Flash Management Unit"]
    #[inline(always)]
    pub const fn set_fmu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enables the clock for the Flash Memory Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Flash Memory Controller"]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables the clock for FlexSPI"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for FlexSPI"]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for INPUTMUX"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for INPUTMUX"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock for PORT0 controller"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PORT0 controller"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for PINT"]
    #[must_use]
    #[inline(always)]
    pub const fn pint(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PINT"]
    #[inline(always)]
    pub const fn set_pint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CRC"]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enables the clock for WWDT0"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for WWDT0"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enables the clock for WWDT1"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for WWDT1"]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub const fn set_mailbox(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ramb_ctrl", &self.ramb_ctrl())
            .field("ramc_ctrl", &self.ramc_ctrl())
            .field("ramd_ctrl", &self.ramd_ctrl())
            .field("rame_ctrl", &self.rame_ctrl())
            .field("ramf_ctrl", &self.ramf_ctrl())
            .field("ramg_ctrl", &self.ramg_ctrl())
            .field("ramh_ctrl", &self.ramh_ctrl())
            .field("fmu", &self.fmu())
            .field("fmc", &self.fmc())
            .field("flexspi", &self.flexspi())
            .field("mux", &self.mux())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("pint", &self.pint())
            .field("dma0", &self.dma0())
            .field("crc", &self.crc())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("mailbox", &self.mailbox())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl0 {{ rom: {=bool:?}, ramb_ctrl: {=bool:?}, ramc_ctrl: {=bool:?}, ramd_ctrl: {=bool:?}, rame_ctrl: {=bool:?}, ramf_ctrl: {=bool:?}, ramg_ctrl: {=bool:?}, ramh_ctrl: {=bool:?}, fmu: {=bool:?}, fmc: {=bool:?}, flexspi: {=bool:?}, mux: {=bool:?}, port0: {=bool:?}, port1: {=bool:?}, port2: {=bool:?}, port3: {=bool:?}, port4: {=bool:?}, gpio0: {=bool:?}, gpio1: {=bool:?}, gpio2: {=bool:?}, gpio3: {=bool:?}, gpio4: {=bool:?}, pint: {=bool:?}, dma0: {=bool:?}, crc: {=bool:?}, wwdt0: {=bool:?}, wwdt1: {=bool:?}, mailbox: {=bool:?} }}",
            self.rom(),
            self.ramb_ctrl(),
            self.ramc_ctrl(),
            self.ramd_ctrl(),
            self.rame_ctrl(),
            self.ramf_ctrl(),
            self.ramg_ctrl(),
            self.ramh_ctrl(),
            self.fmu(),
            self.fmc(),
            self.flexspi(),
            self.mux(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4(),
            self.gpio0(),
            self.gpio1(),
            self.gpio2(),
            self.gpio3(),
            self.gpio4(),
            self.pint(),
            self.dma0(),
            self.crc(),
            self.wwdt0(),
            self.wwdt1(),
            self.mailbox()
        )
    }
}
#[doc = "AHB Clock Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl1(pub u32);
impl Ahbclkctrl1 {
    #[doc = "Enables the clock for MRT"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for MRT"]
    #[inline(always)]
    pub const fn set_mrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the clock for the OS Event Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the OS Event Timer"]
    #[inline(always)]
    pub const fn set_ostimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for SCT"]
    #[must_use]
    #[inline(always)]
    pub const fn sct(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SCT"]
    #[inline(always)]
    pub const fn set_sct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for RTC"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for RTC"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for EVSIM0"]
    #[must_use]
    #[inline(always)]
    pub const fn evsim0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EVSIM0"]
    #[inline(always)]
    pub const fn set_evsim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for EVSIM1"]
    #[must_use]
    #[inline(always)]
    pub const fn evsim1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EVSIM1"]
    #[inline(always)]
    pub const fn set_evsim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enables the clock for UTICK"]
    #[must_use]
    #[inline(always)]
    pub const fn utick(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for UTICK"]
    #[inline(always)]
    pub const fn set_utick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM0"]
    #[must_use]
    #[inline(always)]
    pub const fn fc0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM0"]
    #[inline(always)]
    pub const fn set_fc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM1"]
    #[must_use]
    #[inline(always)]
    pub const fn fc1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM1"]
    #[inline(always)]
    pub const fn set_fc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM2"]
    #[must_use]
    #[inline(always)]
    pub const fn fc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM2"]
    #[inline(always)]
    pub const fn set_fc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM3"]
    #[must_use]
    #[inline(always)]
    pub const fn fc3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM3"]
    #[inline(always)]
    pub const fn set_fc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM4"]
    #[must_use]
    #[inline(always)]
    pub const fn fc4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM4"]
    #[inline(always)]
    pub const fn set_fc4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM5"]
    #[must_use]
    #[inline(always)]
    pub const fn fc5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM5"]
    #[inline(always)]
    pub const fn set_fc5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM6"]
    #[must_use]
    #[inline(always)]
    pub const fn fc6(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM6"]
    #[inline(always)]
    pub const fn set_fc6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM7"]
    #[must_use]
    #[inline(always)]
    pub const fn fc7(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM7"]
    #[inline(always)]
    pub const fn set_fc7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM8"]
    #[must_use]
    #[inline(always)]
    pub const fn fc8(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM8"]
    #[inline(always)]
    pub const fn set_fc8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM9"]
    #[must_use]
    #[inline(always)]
    pub const fn fc9(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM9"]
    #[inline(always)]
    pub const fn set_fc9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for MICFIL"]
    #[must_use]
    #[inline(always)]
    pub const fn micfil(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for MICFIL"]
    #[inline(always)]
    pub const fn set_micfil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn timer2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER2"]
    #[inline(always)]
    pub const fn set_timer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for USB-FS DCD"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs_dcd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB-FS DCD"]
    #[inline(always)]
    pub const fn set_usb0_fs_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for USB-FS"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB-FS"]
    #[inline(always)]
    pub const fn set_usb0_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER0"]
    #[inline(always)]
    pub const fn set_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER1"]
    #[inline(always)]
    pub const fn set_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enables the clock for PKC RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_ram(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PKC RAM"]
    #[inline(always)]
    pub const fn set_pkc_ram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for SmartDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SmartDMA"]
    #[inline(always)]
    pub const fn set_smart_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("dac0", &self.dac0())
            .field("rtc", &self.rtc())
            .field("evsim0", &self.evsim0())
            .field("evsim1", &self.evsim1())
            .field("utick", &self.utick())
            .field("fc0", &self.fc0())
            .field("fc1", &self.fc1())
            .field("fc2", &self.fc2())
            .field("fc3", &self.fc3())
            .field("fc4", &self.fc4())
            .field("fc5", &self.fc5())
            .field("fc6", &self.fc6())
            .field("fc7", &self.fc7())
            .field("fc8", &self.fc8())
            .field("fc9", &self.fc9())
            .field("micfil", &self.micfil())
            .field("timer2", &self.timer2())
            .field("usb0_fs_dcd", &self.usb0_fs_dcd())
            .field("usb0_fs", &self.usb0_fs())
            .field("timer0", &self.timer0())
            .field("timer1", &self.timer1())
            .field("pkc_ram", &self.pkc_ram())
            .field("smart_dma", &self.smart_dma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl1 {{ mrt: {=bool:?}, ostimer: {=bool:?}, sct: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, dac0: {=bool:?}, rtc: {=bool:?}, evsim0: {=bool:?}, evsim1: {=bool:?}, utick: {=bool:?}, fc0: {=bool:?}, fc1: {=bool:?}, fc2: {=bool:?}, fc3: {=bool:?}, fc4: {=bool:?}, fc5: {=bool:?}, fc6: {=bool:?}, fc7: {=bool:?}, fc8: {=bool:?}, fc9: {=bool:?}, micfil: {=bool:?}, timer2: {=bool:?}, usb0_fs_dcd: {=bool:?}, usb0_fs: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, pkc_ram: {=bool:?}, smart_dma: {=bool:?} }}",
            self.mrt(),
            self.ostimer(),
            self.sct(),
            self.adc0(),
            self.adc1(),
            self.dac0(),
            self.rtc(),
            self.evsim0(),
            self.evsim1(),
            self.utick(),
            self.fc0(),
            self.fc1(),
            self.fc2(),
            self.fc3(),
            self.fc4(),
            self.fc5(),
            self.fc6(),
            self.fc7(),
            self.fc8(),
            self.fc9(),
            self.micfil(),
            self.timer2(),
            self.usb0_fs_dcd(),
            self.usb0_fs(),
            self.timer0(),
            self.timer1(),
            self.pkc_ram(),
            self.smart_dma()
        )
    }
}
#[doc = "AHB Clock Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl2(pub u32);
impl Ahbclkctrl2 {
    #[doc = "Enables the clock for DMA1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DMA1"]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for Ethernet"]
    #[must_use]
    #[inline(always)]
    pub const fn enet(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Ethernet"]
    #[inline(always)]
    pub const fn set_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for uSDHC"]
    #[must_use]
    #[inline(always)]
    pub const fn u_sdhc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for uSDHC"]
    #[inline(always)]
    pub const fn set_u_sdhc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for Flexio"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Flexio"]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for SAI0"]
    #[must_use]
    #[inline(always)]
    pub const fn sai0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SAI0"]
    #[inline(always)]
    pub const fn set_sai0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for SAI1"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SAI1"]
    #[inline(always)]
    pub const fn set_sai1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for TRO"]
    #[must_use]
    #[inline(always)]
    pub const fn tro(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for TRO"]
    #[inline(always)]
    pub const fn set_tro(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the Frequency meter"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Frequency meter"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for TRNG"]
    #[must_use]
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for TRNG"]
    #[inline(always)]
    pub const fn set_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for FLEXCAN0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for FLEXCAN0"]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for FLEXCAN1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for FLEXCAN1"]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for USB HS"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB HS"]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_phy(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB HS PHY"]
    #[inline(always)]
    pub const fn set_usb_hs_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for ELS"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ELS"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for Powerquad"]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Powerquad"]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for PLU_LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn plu_lut(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PLU_LUT"]
    #[inline(always)]
    pub const fn set_plu_lut(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn timer3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER3"]
    #[inline(always)]
    pub const fn set_timer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn timer4(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER4"]
    #[inline(always)]
    pub const fn set_timer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for PUF"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PUF"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for PKC"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PKC"]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for SCG"]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SCG"]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for GDET0 and GDET1"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GDET0 and GDET1"]
    #[inline(always)]
    pub const fn set_gdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for SM3"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SM3"]
    #[inline(always)]
    pub const fn set_sm3(&mut self, val: bool) {
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
            .field("enet", &self.enet())
            .field("u_sdhc", &self.u_sdhc())
            .field("flexio", &self.flexio())
            .field("sai0", &self.sai0())
            .field("sai1", &self.sai1())
            .field("tro", &self.tro())
            .field("freqme", &self.freqme())
            .field("trng", &self.trng())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("usb_hs", &self.usb_hs())
            .field("usb_hs_phy", &self.usb_hs_phy())
            .field("els", &self.els())
            .field("pq", &self.pq())
            .field("plu_lut", &self.plu_lut())
            .field("timer3", &self.timer3())
            .field("timer4", &self.timer4())
            .field("puf", &self.puf())
            .field("pkc", &self.pkc())
            .field("scg", &self.scg())
            .field("gdet", &self.gdet())
            .field("sm3", &self.sm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl2 {{ dma1: {=bool:?}, enet: {=bool:?}, u_sdhc: {=bool:?}, flexio: {=bool:?}, sai0: {=bool:?}, sai1: {=bool:?}, tro: {=bool:?}, freqme: {=bool:?}, trng: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, usb_hs: {=bool:?}, usb_hs_phy: {=bool:?}, els: {=bool:?}, pq: {=bool:?}, plu_lut: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, puf: {=bool:?}, pkc: {=bool:?}, scg: {=bool:?}, gdet: {=bool:?}, sm3: {=bool:?} }}",
            self.dma1(),
            self.enet(),
            self.u_sdhc(),
            self.flexio(),
            self.sai0(),
            self.sai1(),
            self.tro(),
            self.freqme(),
            self.trng(),
            self.flexcan0(),
            self.flexcan1(),
            self.usb_hs(),
            self.usb_hs_phy(),
            self.els(),
            self.pq(),
            self.plu_lut(),
            self.timer3(),
            self.timer4(),
            self.puf(),
            self.pkc(),
            self.scg(),
            self.gdet(),
            self.sm3()
        )
    }
}
#[doc = "AHB Clock Control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl3(pub u32);
impl Ahbclkctrl3 {
    #[doc = "Enables the clock for I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the clock for I3C1"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for I3C1"]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for SINC"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SINC"]
    #[inline(always)]
    pub const fn set_sinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for CoolFlux"]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CoolFlux"]
    #[inline(always)]
    pub const fn set_coolflux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for QDC0"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for QDC0"]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for QDC1"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for QDC1"]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PWM0"]
    #[inline(always)]
    pub const fn set_pwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for PWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PWM1"]
    #[inline(always)]
    pub const fn set_pwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for EVTG"]
    #[must_use]
    #[inline(always)]
    pub const fn evtg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EVTG"]
    #[inline(always)]
    pub const fn set_evtg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for DAC1"]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DAC1"]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for DAC2"]
    #[must_use]
    #[inline(always)]
    pub const fn dac2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DAC2"]
    #[inline(always)]
    pub const fn set_dac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock for OPAMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for OPAMP0"]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for OPAMP1"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for OPAMP1"]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for OPAMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for OPAMP2"]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for CMP2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CMP2"]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for VREF"]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for VREF"]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for CoolFlux APB"]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_apb(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CoolFlux APB"]
    #[inline(always)]
    pub const fn set_coolflux_apb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for NPU"]
    #[must_use]
    #[inline(always)]
    pub const fn npu(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for NPU"]
    #[inline(always)]
    pub const fn set_npu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for TSI"]
    #[must_use]
    #[inline(always)]
    pub const fn tsi(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for TSI"]
    #[inline(always)]
    pub const fn set_tsi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for EWM"]
    #[must_use]
    #[inline(always)]
    pub const fn ewm(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EWM"]
    #[inline(always)]
    pub const fn set_ewm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for EIM"]
    #[must_use]
    #[inline(always)]
    pub const fn eim(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EIM"]
    #[inline(always)]
    pub const fn set_eim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for ERM"]
    #[must_use]
    #[inline(always)]
    pub const fn erm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ERM"]
    #[inline(always)]
    pub const fn set_erm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for INTM"]
    #[must_use]
    #[inline(always)]
    pub const fn intm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for INTM"]
    #[inline(always)]
    pub const fn set_intm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for Semaphore"]
    #[must_use]
    #[inline(always)]
    pub const fn sema42(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Semaphore"]
    #[inline(always)]
    pub const fn set_sema42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Ahbclkctrl3 {
    #[inline(always)]
    fn default() -> Ahbclkctrl3 {
        Ahbclkctrl3(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl3")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("sinc", &self.sinc())
            .field("coolflux", &self.coolflux())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("pwm0", &self.pwm0())
            .field("pwm1", &self.pwm1())
            .field("evtg", &self.evtg())
            .field("dac1", &self.dac1())
            .field("dac2", &self.dac2())
            .field("opamp0", &self.opamp0())
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("cmp2", &self.cmp2())
            .field("vref", &self.vref())
            .field("coolflux_apb", &self.coolflux_apb())
            .field("npu", &self.npu())
            .field("tsi", &self.tsi())
            .field("ewm", &self.ewm())
            .field("eim", &self.eim())
            .field("erm", &self.erm())
            .field("intm", &self.intm())
            .field("sema42", &self.sema42())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl3 {{ i3c0: {=bool:?}, i3c1: {=bool:?}, sinc: {=bool:?}, coolflux: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, pwm0: {=bool:?}, pwm1: {=bool:?}, evtg: {=bool:?}, dac1: {=bool:?}, dac2: {=bool:?}, opamp0: {=bool:?}, opamp1: {=bool:?}, opamp2: {=bool:?}, cmp2: {=bool:?}, vref: {=bool:?}, coolflux_apb: {=bool:?}, npu: {=bool:?}, tsi: {=bool:?}, ewm: {=bool:?}, eim: {=bool:?}, erm: {=bool:?}, intm: {=bool:?}, sema42: {=bool:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.sinc(),
            self.coolflux(),
            self.qdc0(),
            self.qdc1(),
            self.pwm0(),
            self.pwm1(),
            self.evtg(),
            self.dac1(),
            self.dac2(),
            self.opamp0(),
            self.opamp1(),
            self.opamp2(),
            self.cmp2(),
            self.vref(),
            self.coolflux_apb(),
            self.npu(),
            self.tsi(),
            self.ewm(),
            self.eim(),
            self.erm(),
            self.intm(),
            self.sema42()
        )
    }
}
#[doc = "AHB Clock Control Clear"]
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
#[doc = "AHB Clock Control Set"]
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
#[doc = "System Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::AhbclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AhbclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::AhbclkdivUnstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "AHB Matrix Priority Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu0_cbus(&self) -> super::vals::PriCpu0Cbus {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PriCpu0Cbus::from_bits(val as u8)
    }
    #[doc = "CPU0 C-AHB bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_cpu0_cbus(&mut self, val: super::vals::PriCpu0Cbus) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu0_sbus(&self) -> super::vals::PriCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PriCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_cpu0_sbus(&mut self, val: super::vals::PriCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU1 S-AHB/SmartDMA-D bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu1_sbus_smart_dma_d(&self) -> super::vals::PriCpu1SbusSmartDmaD {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PriCpu1SbusSmartDmaD::from_bits(val as u8)
    }
    #[doc = "CPU1 S-AHB/SmartDMA-D bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_cpu1_sbus_smart_dma_d(&mut self, val: super::vals::PriCpu1SbusSmartDmaD) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU1 C-AHB/SmartDMA-I bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu1_cbus_smart_dma_i(&self) -> super::vals::PriCpu1CbusSmartDmaI {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::PriCpu1CbusSmartDmaI::from_bits(val as u8)
    }
    #[doc = "CPU1 C-AHB/SmartDMA-I bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_cpu1_cbus_smart_dma_i(&mut self, val: super::vals::PriCpu1CbusSmartDmaI) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 controller bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::AhbmatprioDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AhbmatprioDma0::from_bits(val as u8)
    }
    #[doc = "DMA0 controller bus master priority level"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::AhbmatprioDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA1 controller bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> super::vals::AhbmatprioDma1 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::AhbmatprioDma1::from_bits(val as u8)
    }
    #[doc = "DMA1 controller bus master priority level"]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: super::vals::AhbmatprioDma1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "PKC and ELS bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_pkc_els(&self) -> super::vals::PriPkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::PriPkcEls::from_bits(val as u8)
    }
    #[doc = "PKC and ELS bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_pkc_els(&mut self, val: super::vals::PriPkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "NPU O bus and Powerquad bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_npu_pq(&self) -> super::vals::PriNpuPq {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PriNpuPq::from_bits(val as u8)
    }
    #[doc = "NPU O bus and Powerquad bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_npu_pq(&mut self, val: super::vals::PriNpuPq) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CoolFlux I bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_coolflux_i(&self) -> super::vals::PriCoolfluxI {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PriCoolfluxI::from_bits(val as u8)
    }
    #[doc = "CoolFlux I bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_coolflux_i(&mut self, val: super::vals::PriCoolfluxI) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CoolFlux X bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_coolflux_x(&self) -> super::vals::PriCoolfluxX {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::PriCoolfluxX::from_bits(val as u8)
    }
    #[doc = "CoolFlux X bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_coolflux_x(&mut self, val: super::vals::PriCoolfluxX) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "CoolFlux Y bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_coolflux_y_espi(&self) -> super::vals::PriCoolfluxYEspi {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::PriCoolfluxYEspi::from_bits(val as u8)
    }
    #[doc = "CoolFlux Y bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_coolflux_y_espi(&mut self, val: super::vals::PriCoolfluxYEspi) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "NPU D bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_npu_d(&self) -> super::vals::PriNpuD {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::PriNpuD::from_bits(val as u8)
    }
    #[doc = "NPU D bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_npu_d(&mut self, val: super::vals::PriNpuD) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "USB-FS and ENET bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usb_fs_enet(&self) -> super::vals::PriUsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::PriUsbFsEnet::from_bits(val as u8)
    }
    #[doc = "USB-FS and ENET bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_usb_fs_enet(&mut self, val: super::vals::PriUsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB-HS bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usb_hs(&self) -> super::vals::PriUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::PriUsbHs::from_bits(val as u8)
    }
    #[doc = "USB-HS bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_usb_hs(&mut self, val: super::vals::PriUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "USDHC bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usdhc(&self) -> super::vals::PriUsdhc {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::PriUsdhc::from_bits(val as u8)
    }
    #[doc = "USDHC bus master priority level"]
    #[inline(always)]
    pub const fn set_pri_usdhc(&mut self, val: super::vals::PriUsdhc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
            .field(
                "pri_cpu1_sbus_smart_dma_d",
                &self.pri_cpu1_sbus_smart_dma_d(),
            )
            .field(
                "pri_cpu1_cbus_smart_dma_i",
                &self.pri_cpu1_cbus_smart_dma_i(),
            )
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pri_pkc_els", &self.pri_pkc_els())
            .field("pri_npu_pq", &self.pri_npu_pq())
            .field("pri_coolflux_i", &self.pri_coolflux_i())
            .field("pri_coolflux_x", &self.pri_coolflux_x())
            .field("pri_coolflux_y_espi", &self.pri_coolflux_y_espi())
            .field("pri_npu_d", &self.pri_npu_d())
            .field("pri_usb_fs_enet", &self.pri_usb_fs_enet())
            .field("pri_usb_hs", &self.pri_usb_hs())
            .field("pri_usdhc", &self.pri_usdhc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ pri_cpu0_cbus: {:?}, pri_cpu0_sbus: {:?}, pri_cpu1_sbus_smart_dma_d: {:?}, pri_cpu1_cbus_smart_dma_i: {:?}, dma0: {:?}, dma1: {:?}, pri_pkc_els: {:?}, pri_npu_pq: {:?}, pri_coolflux_i: {:?}, pri_coolflux_x: {:?}, pri_coolflux_y_espi: {:?}, pri_npu_d: {:?}, pri_usb_fs_enet: {:?}, pri_usb_hs: {:?}, pri_usdhc: {:?} }}",
            self.pri_cpu0_cbus(),
            self.pri_cpu0_sbus(),
            self.pri_cpu1_sbus_smart_dma_d(),
            self.pri_cpu1_cbus_smart_dma_i(),
            self.dma0(),
            self.dma1(),
            self.pri_pkc_els(),
            self.pri_npu_pq(),
            self.pri_coolflux_i(),
            self.pri_coolflux_x(),
            self.pri_coolflux_y_espi(),
            self.pri_npu_d(),
            self.pri_usb_fs_enet(),
            self.pri_usb_hs(),
            self.pri_usdhc()
        )
    }
}
#[doc = "Control Automatic Clock Gating"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverride(pub u32);
impl Autoclkgateoverride {
    #[doc = "Controls automatic clock gating for the RAMB Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMB Controller"]
    #[inline(always)]
    pub const fn set_ramb_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Controls automatic clock gating for the RAMC Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_ctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMC Controller"]
    #[inline(always)]
    pub const fn set_ramc_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramd_ctrl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller"]
    #[inline(always)]
    pub const fn set_ramd_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn rame_ctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    #[inline(always)]
    pub const fn set_rame_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Controls automatic clock gating for the RAMF Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramf_ctrl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMF Controller"]
    #[inline(always)]
    pub const fn set_ramf_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramg_ctrl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller"]
    #[inline(always)]
    pub const fn set_ramg_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramh_ctrl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller"]
    #[inline(always)]
    pub const fn set_ramh_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("ramb_ctrl", &self.ramb_ctrl())
            .field("ramc_ctrl", &self.ramc_ctrl())
            .field("ramd_ctrl", &self.ramd_ctrl())
            .field("rame_ctrl", &self.rame_ctrl())
            .field("ramf_ctrl", &self.ramf_ctrl())
            .field("ramg_ctrl", &self.ramg_ctrl())
            .field("ramh_ctrl", &self.ramh_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverride {{ ramb_ctrl: {=bool:?}, ramc_ctrl: {=bool:?}, ramd_ctrl: {=bool:?}, rame_ctrl: {=bool:?}, ramf_ctrl: {=bool:?}, ramg_ctrl: {=bool:?}, ramh_ctrl: {=bool:?} }}",
            self.ramb_ctrl(),
            self.ramc_ctrl(),
            self.ramd_ctrl(),
            self.rame_ctrl(),
            self.ramf_ctrl(),
            self.ramg_ctrl(),
            self.ramh_ctrl()
        )
    }
}
#[doc = "Control Automatic Clock Gating C"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverridec(pub u32);
impl Autoclkgateoverridec {
    #[doc = "Controls automatic clock gating of the RAMX controller"]
    #[must_use]
    #[inline(always)]
    pub const fn ramx(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating of the RAMX controller"]
    #[inline(always)]
    pub const fn set_ramx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Controls automatic clock gating of the RAMA controller"]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating of the RAMA controller"]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Autoclkgateoverridec {
    #[inline(always)]
    fn default() -> Autoclkgateoverridec {
        Autoclkgateoverridec(0)
    }
}
impl core::fmt::Debug for Autoclkgateoverridec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoclkgateoverridec")
            .field("ramx", &self.ramx())
            .field("rama", &self.rama())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverridec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverridec {{ ramx: {=bool:?}, rama: {=bool:?} }}",
            self.ramx(),
            self.rama()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[31:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeLsb(pub u32);
impl BinaryCodeLsb {
    #[doc = "Binary code \\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Binary code \\[31:0\\]"]
    #[inline(always)]
    pub const fn set_code_bin_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BinaryCodeLsb {
    #[inline(always)]
    fn default() -> BinaryCodeLsb {
        BinaryCodeLsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeLsb")
            .field("code_bin_31_0", &self.code_bin_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeLsb {{ code_bin_31_0: {=u32:?} }}",
            self.code_bin_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeMsb(pub u32);
impl BinaryCodeMsb {
    #[doc = "Binary code \\[41:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Binary code \\[41:32\\]"]
    #[inline(always)]
    pub const fn set_code_bin_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BinaryCodeMsb {
    #[inline(always)]
    fn default() -> BinaryCodeMsb {
        BinaryCodeMsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeMsb")
            .field("code_bin_41_32", &self.code_bin_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeMsb {{ code_bin_41_32: {=u16:?} }}",
            self.code_bin_41_32()
        )
    }
}
#[doc = "CLKOUT FRG Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkoutFrgctrl(pub u32);
impl ClkoutFrgctrl {
    #[doc = "Divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator value"]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator value"]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ClkoutFrgctrl {
    #[inline(always)]
    fn default() -> ClkoutFrgctrl {
        ClkoutFrgctrl(0)
    }
}
impl core::fmt::Debug for ClkoutFrgctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkoutFrgctrl")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkoutFrgctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkoutFrgctrl {{ div: {=u8:?}, mult: {=u8:?} }}",
            self.div(),
            self.mult()
        )
    }
}
#[doc = "CLKOUT Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutdiv(pub u32);
impl Clkoutdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::ClkoutdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::ClkoutdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::ClkoutdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::ClkoutdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::ClkoutdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::ClkoutdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::ClkoutdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkoutdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::ClkoutdivUnstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkoutdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CLKOUT Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutsel(pub u32);
impl Clkoutsel {
    #[doc = "Selects the CLKOUT clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::ClkoutselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::ClkoutselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CLKOUT clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::ClkoutselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
#[doc = "Clock Configuration Unlock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkunlock(pub u32);
impl Clkunlock {
    #[doc = "Controls clock configuration registers access (for example, xxxDIV, xxxSEL)"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> super::vals::Unlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Unlock::from_bits(val as u8)
    }
    #[doc = "Controls clock configuration registers access (for example, xxxDIV, xxxSEL)"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: super::vals::Unlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clkunlock {
    #[inline(always)]
    fn default() -> Clkunlock {
        Clkunlock(0)
    }
}
impl core::fmt::Debug for Clkunlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkunlock")
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkunlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkunlock {{ unlock: {:?} }}", self.unlock())
    }
}
#[doc = "Clock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClockCtrl(pub u32);
impl ClockCtrl {
    #[doc = "Enables the clk_in clock for the Frequency Measurement, USB HS and LPTMR0/1 modules."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin_ena_fm_usbh_lpt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk_in clock for the Frequency Measurement, USB HS and LPTMR0/1 modules."]
    #[inline(always)]
    pub const fn set_clkin_ena_fm_usbh_lpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the FRO_1MHz clock for RTC module and for UTICK"]
    #[must_use]
    #[inline(always)]
    pub const fn fro1mhz_ena(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the FRO_1MHz clock for RTC module and for UTICK"]
    #[inline(always)]
    pub const fn set_fro1mhz_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the FRO_12MHz clock for the Flash, LPTMR0/1, and Frequency Measurement modules"]
    #[must_use]
    #[inline(always)]
    pub const fn fro12mhz_ena(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the FRO_12MHz clock for the Flash, LPTMR0/1, and Frequency Measurement modules"]
    #[inline(always)]
    pub const fn set_fro12mhz_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables FRO HF clock for the Frequency Measure module"]
    #[must_use]
    #[inline(always)]
    pub const fn fro_hf_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables FRO HF clock for the Frequency Measure module"]
    #[inline(always)]
    pub const fn set_fro_hf_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables clk_in clock for MICFIL, CAN0/1, I3C0/1, SAI0/1, clkout."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin_ena(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables clk_in clock for MICFIL, CAN0/1, I3C0/1, SAI0/1, clkout."]
    #[inline(always)]
    pub const fn set_clkin_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables FRO_1MHz clock for clock muxing in clock gen"]
    #[must_use]
    #[inline(always)]
    pub const fn fro1mhz_clk_ena(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables FRO_1MHz clock for clock muxing in clock gen"]
    #[inline(always)]
    pub const fn set_fro1mhz_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[must_use]
    #[inline(always)]
    pub const fn plu_deglitch_clk_ena(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
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
            .field("clkin_ena_fm_usbh_lpt", &self.clkin_ena_fm_usbh_lpt())
            .field("fro1mhz_ena", &self.fro1mhz_ena())
            .field("fro12mhz_ena", &self.fro12mhz_ena())
            .field("fro_hf_ena", &self.fro_hf_ena())
            .field("clkin_ena", &self.clkin_ena())
            .field("fro1mhz_clk_ena", &self.fro1mhz_clk_ena())
            .field("plu_deglitch_clk_ena", &self.plu_deglitch_clk_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClockCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClockCtrl {{ clkin_ena_fm_usbh_lpt: {=bool:?}, fro1mhz_ena: {=bool:?}, fro12mhz_ena: {=bool:?}, fro_hf_ena: {=bool:?}, clkin_ena: {=bool:?}, fro1mhz_clk_ena: {=bool:?}, plu_deglitch_clk_ena: {=bool:?} }}",
            self.clkin_ena_fm_usbh_lpt(),
            self.fro1mhz_ena(),
            self.fro12mhz_ena(),
            self.fro_hf_ena(),
            self.clkin_ena(),
            self.fro1mhz_clk_ena(),
            self.plu_deglitch_clk_ena()
        )
    }
}
#[doc = "CMP0 Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0fclkdiv(pub u32);
impl Cmp0fclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Cmp0fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cmp0fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Cmp0fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Cmp0fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp0fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Cmp0fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Cmp0fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cmp0fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Cmp0fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmp0fclkdiv {
    #[inline(always)]
    fn default() -> Cmp0fclkdiv {
        Cmp0fclkdiv(0)
    }
}
impl core::fmt::Debug for Cmp0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmp0fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0 Function Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0fclksel(pub u32);
impl Cmp0fclksel {
    #[doc = "Selects the CMP0 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Cmp0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmp0fclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP0 function clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Cmp0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp0fclksel {
    #[inline(always)]
    fn default() -> Cmp0fclksel {
        Cmp0fclksel(0)
    }
}
impl core::fmt::Debug for Cmp0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP0 Round Robin Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0rrclkdiv(pub u32);
impl Cmp0rrclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Cmp0rrclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cmp0rrclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Cmp0rrclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Cmp0rrclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp0rrclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Cmp0rrclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Cmp0rrclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cmp0rrclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Cmp0rrclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmp0rrclkdiv {
    #[inline(always)]
    fn default() -> Cmp0rrclkdiv {
        Cmp0rrclkdiv(0)
    }
}
impl core::fmt::Debug for Cmp0rrclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp0rrclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp0rrclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmp0rrclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP0 Round Robin Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0rrclksel(pub u32);
impl Cmp0rrclksel {
    #[doc = "Selects the CMP0 round robin clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Cmp0rrclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmp0rrclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP0 round robin clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Cmp0rrclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp0rrclksel {
    #[inline(always)]
    fn default() -> Cmp0rrclksel {
        Cmp0rrclksel(0)
    }
}
impl core::fmt::Debug for Cmp0rrclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp0rrclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp0rrclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp0rrclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP1 Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1fclkdiv(pub u32);
impl Cmp1fclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Cmp1fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cmp1fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Cmp1fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Cmp1fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp1fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Cmp1fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Cmp1fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cmp1fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Cmp1fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmp1fclkdiv {
    #[inline(always)]
    fn default() -> Cmp1fclkdiv {
        Cmp1fclkdiv(0)
    }
}
impl core::fmt::Debug for Cmp1fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp1fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp1fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmp1fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1 Function Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1fclksel(pub u32);
impl Cmp1fclksel {
    #[doc = "Selects the CMP1 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Cmp1fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmp1fclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP1 function clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Cmp1fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp1fclksel {
    #[inline(always)]
    fn default() -> Cmp1fclksel {
        Cmp1fclksel(0)
    }
}
impl core::fmt::Debug for Cmp1fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp1fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp1fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp1fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP1 Round Robin Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1rrclkdiv(pub u32);
impl Cmp1rrclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Cmp1rrclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cmp1rrclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Cmp1rrclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Cmp1rrclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp1rrclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Cmp1rrclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Cmp1rrclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cmp1rrclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Cmp1rrclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmp1rrclkdiv {
    #[inline(always)]
    fn default() -> Cmp1rrclkdiv {
        Cmp1rrclkdiv(0)
    }
}
impl core::fmt::Debug for Cmp1rrclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp1rrclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp1rrclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmp1rrclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP1 Round Robin Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1rrclksel(pub u32);
impl Cmp1rrclksel {
    #[doc = "Selects the CMP1 round robin clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Cmp1rrclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmp1rrclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP1 round robin clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Cmp1rrclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp1rrclksel {
    #[inline(always)]
    fn default() -> Cmp1rrclksel {
        Cmp1rrclksel(0)
    }
}
impl core::fmt::Debug for Cmp1rrclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp1rrclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp1rrclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp1rrclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP2 Function Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2fclkdiv(pub u32);
impl Cmp2fclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Cmp2fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cmp2fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Cmp2fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Cmp2fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp2fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Cmp2fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Cmp2fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cmp2fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Cmp2fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmp2fclkdiv {
    #[inline(always)]
    fn default() -> Cmp2fclkdiv {
        Cmp2fclkdiv(0)
    }
}
impl core::fmt::Debug for Cmp2fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp2fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp2fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmp2fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP2 Function Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2fclksel(pub u32);
impl Cmp2fclksel {
    #[doc = "Selects the CMP2 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Cmp2fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmp2fclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP2 function clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Cmp2fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp2fclksel {
    #[inline(always)]
    fn default() -> Cmp2fclksel {
        Cmp2fclksel(0)
    }
}
impl core::fmt::Debug for Cmp2fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp2fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp2fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp2fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP2 Round Robin Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2rrclkdiv(pub u32);
impl Cmp2rrclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Cmp2rrclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cmp2rrclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Cmp2rrclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Cmp2rrclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp2rrclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Cmp2rrclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Cmp2rrclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cmp2rrclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Cmp2rrclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmp2rrclkdiv {
    #[inline(always)]
    fn default() -> Cmp2rrclkdiv {
        Cmp2rrclkdiv(0)
    }
}
impl core::fmt::Debug for Cmp2rrclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp2rrclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp2rrclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmp2rrclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP2 Round Robin Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2rrclksel(pub u32);
impl Cmp2rrclksel {
    #[doc = "Selects the CMP2 round robin clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Cmp2rrclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmp2rrclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP2 round robin clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Cmp2rrclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp2rrclksel {
    #[inline(always)]
    fn default() -> Cmp2rrclksel {
        Cmp2rrclksel(0)
    }
}
impl core::fmt::Debug for Cmp2rrclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp2rrclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp2rrclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp2rrclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Coprocessor Boot Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpboot(pub u32);
impl Cpboot {
    #[doc = "Coprocessor Boot VTOR Address \\[31:7\\] for CPU1"]
    #[must_use]
    #[inline(always)]
    pub const fn cpboot(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Coprocessor Boot VTOR Address \\[31:7\\] for CPU1"]
    #[inline(always)]
    pub const fn set_cpboot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
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
#[doc = "Non-Secure CPU0 System Tick Calibration"]
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
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Cpu0nstckcalSkew {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Cpu0nstckcalSkew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Cpu0nstckcalSkew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Cpu0nstckcalNoref {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Cpu0nstckcalNoref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Cpu0nstckcalNoref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "Secure CPU0 System Tick Calibration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0stckcal(pub u32);
impl Cpu0stckcal {
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
    #[doc = "Whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Cpu0stckcalSkew {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Cpu0stckcalSkew::from_bits(val as u8)
    }
    #[doc = "Whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Cpu0stckcalSkew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Cpu0stckcalNoref {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Cpu0stckcalNoref::from_bits(val as u8)
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Cpu0stckcalNoref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
            "Cpu0stckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
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
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Cpu1stckcalSkew {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Cpu1stckcalSkew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Cpu1stckcalSkew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Cpu1stckcalNoref {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Cpu1stckcalNoref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Cpu1stckcalNoref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
            "Cpu1stckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPU Control for Multiple Processors"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuctrl(pub u32);
impl Cpuctrl {
    #[doc = "Enables the CPU1 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1clken(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CPU1 clock"]
    #[inline(always)]
    pub const fn set_cpu1clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CPU1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1rsten(&self) -> super::vals::Cpu1rsten {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cpu1rsten::from_bits(val as u8)
    }
    #[doc = "CPU1 reset"]
    #[inline(always)]
    pub const fn set_cpu1rsten(&mut self, val: super::vals::Cpu1rsten) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Write Protect"]
    #[must_use]
    #[inline(always)]
    pub const fn prot(&self) -> super::vals::Prot {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Prot::from_bits(val as u16)
    }
    #[doc = "Write Protect"]
    #[inline(always)]
    pub const fn set_prot(&mut self, val: super::vals::Prot) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
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
            .field("prot", &self.prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpuctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpuctrl {{ cpu1clken: {=bool:?}, cpu1rsten: {:?}, prot: {:?} }}",
            self.cpu1clken(),
            self.cpu1rsten(),
            self.prot()
        )
    }
}
#[doc = "CPU Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpustat(pub u32);
impl Cpustat {
    #[doc = "CPU0 sleeping state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> super::vals::Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "CPU0 sleeping state"]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: super::vals::Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU1 sleeping state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1sleeping(&self) -> super::vals::Cpu1sleeping {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cpu1sleeping::from_bits(val as u8)
    }
    #[doc = "CPU1 sleeping state"]
    #[inline(always)]
    pub const fn set_cpu1sleeping(&mut self, val: super::vals::Cpu1sleeping) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU0 lockup state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> super::vals::Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "CPU0 lockup state"]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: super::vals::Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CPU1 lockup state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1lockup(&self) -> super::vals::Cpu1lockup {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cpu1lockup::from_bits(val as u8)
    }
    #[doc = "CPU1 lockup state"]
    #[inline(always)]
    pub const fn set_cpu1lockup(&mut self, val: super::vals::Cpu1lockup) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Cpustat {
    #[inline(always)]
    fn default() -> Cpustat {
        Cpustat(0)
    }
}
impl core::fmt::Debug for Cpustat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpustat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu1sleeping", &self.cpu1sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .field("cpu1lockup", &self.cpu1lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpustat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpustat {{ cpu0sleeping: {:?}, cpu1sleeping: {:?}, cpu0lockup: {:?}, cpu1lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu1sleeping(),
            self.cpu0lockup(),
            self.cpu1lockup()
        )
    }
}
#[doc = "CTimer Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkdiv(pub u32);
impl Ctimerclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::CtimerclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CtimerclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::CtimerclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::CtimerclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtimerclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::CtimerclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctimerclkdiv {
    #[inline(always)]
    fn default() -> Ctimerclkdiv {
        Ctimerclkdiv(0)
    }
}
impl core::fmt::Debug for Ctimerclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel(pub u32);
impl Ctimerclksel {
    #[doc = "Selects the CTIMER clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::CtimerclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::CtimerclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CTIMER clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::CtimerclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ctimerclksel {
    #[inline(always)]
    fn default() -> Ctimerclksel {
        Ctimerclksel(0)
    }
}
impl core::fmt::Debug for Ctimerclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CTIMER Global Start Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerglobalstarten(pub u32);
impl Ctimerglobalstarten {
    #[doc = "Enables the CTIMER0 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_clk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER0 function clock"]
    #[inline(always)]
    pub const fn set_ctimer0_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the CTIMER1 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_clk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER1 function clock"]
    #[inline(always)]
    pub const fn set_ctimer1_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the CTIMER2 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_clk_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER2 function clock"]
    #[inline(always)]
    pub const fn set_ctimer2_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the CTIMER3 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_clk_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER3 function clock"]
    #[inline(always)]
    pub const fn set_ctimer3_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the CTIMER4 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_clk_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER4 function clock"]
    #[inline(always)]
    pub const fn set_ctimer4_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctimerglobalstarten {
    #[inline(always)]
    fn default() -> Ctimerglobalstarten {
        Ctimerglobalstarten(0)
    }
}
impl core::fmt::Debug for Ctimerglobalstarten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerglobalstarten")
            .field("ctimer0_clk_en", &self.ctimer0_clk_en())
            .field("ctimer1_clk_en", &self.ctimer1_clk_en())
            .field("ctimer2_clk_en", &self.ctimer2_clk_en())
            .field("ctimer3_clk_en", &self.ctimer3_clk_en())
            .field("ctimer4_clk_en", &self.ctimer4_clk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerglobalstarten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerglobalstarten {{ ctimer0_clk_en: {=bool:?}, ctimer1_clk_en: {=bool:?}, ctimer2_clk_en: {=bool:?}, ctimer3_clk_en: {=bool:?}, ctimer4_clk_en: {=bool:?} }}",
            self.ctimer0_clk_en(),
            self.ctimer1_clk_en(),
            self.ctimer2_clk_en(),
            self.ctimer3_clk_en(),
            self.ctimer4_clk_en()
        )
    }
}
#[doc = "DAC0 functional clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac0clkdiv(pub u32);
impl Dac0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Dac0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dac0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Dac0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Dac0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dac0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Dac0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Dac0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dac0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Dac0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dac0clkdiv {
    #[inline(always)]
    fn default() -> Dac0clkdiv {
        Dac0clkdiv(0)
    }
}
impl core::fmt::Debug for Dac0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dac0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC0 Functional Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac0clksel(pub u32);
impl Dac0clksel {
    #[doc = "Selects the DAC clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Dac0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dac0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the DAC clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Dac0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Dac0clksel {
    #[inline(always)]
    fn default() -> Dac0clksel {
        Dac0clksel(0)
    }
}
impl core::fmt::Debug for Dac0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dac0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "DAC1 functional clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac1clkdiv(pub u32);
impl Dac1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Dac1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dac1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Dac1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Dac1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dac1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Dac1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Dac1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dac1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Dac1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dac1clkdiv {
    #[inline(always)]
    fn default() -> Dac1clkdiv {
        Dac1clkdiv(0)
    }
}
impl core::fmt::Debug for Dac1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dac1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC1 Functional Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac1clksel(pub u32);
impl Dac1clksel {
    #[doc = "Selects the DAC clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Dac1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dac1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the DAC clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Dac1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Dac1clksel {
    #[inline(always)]
    fn default() -> Dac1clksel {
        Dac1clksel(0)
    }
}
impl core::fmt::Debug for Dac1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dac1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "DAC2 functional clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac2clkdiv(pub u32);
impl Dac2clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Dac2clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dac2clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Dac2clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Dac2clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dac2clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Dac2clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Dac2clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dac2clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Dac2clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dac2clkdiv {
    #[inline(always)]
    fn default() -> Dac2clkdiv {
        Dac2clkdiv(0)
    }
}
impl core::fmt::Debug for Dac2clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac2clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac2clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dac2clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC2 Functional Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac2clksel(pub u32);
impl Dac2clksel {
    #[doc = "Selects the DAC clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Dac2clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dac2clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the DAC clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Dac2clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Dac2clksel {
    #[inline(always)]
    fn default() -> Dac2clksel {
        Dac2clksel(0)
    }
}
impl core::fmt::Debug for Dac2clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac2clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac2clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dac2clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Debug Authentication BEACON"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugAuthBeacon(pub u32);
impl DebugAuthBeacon {
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[must_use]
    #[inline(always)]
    pub const fn beacon(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
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
#[doc = "Cortex Debug Features Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeatures(pub u32);
impl DebugFeatures {
    #[doc = "CPU0 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> super::vals::DebugFeaturesCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DebugFeaturesCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: super::vals::DebugFeaturesCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> super::vals::DebugFeaturesCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DebugFeaturesCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: super::vals::DebugFeaturesCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CPU1 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> super::vals::DebugFeaturesCpu1Dbgen {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DebugFeaturesCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: super::vals::DebugFeaturesCpu1Dbgen) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU1 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> super::vals::DebugFeaturesCpu1Niden {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::DebugFeaturesCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: super::vals::DebugFeaturesCpu1Niden) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "DSP invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_dbgden(&self) -> super::vals::DspDbgden {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::DspDbgden::from_bits(val as u8)
    }
    #[doc = "DSP invasive debug control"]
    #[inline(always)]
    pub const fn set_dsp_dbgden(&mut self, val: super::vals::DspDbgden) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
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
            .field("dsp_dbgden", &self.dsp_dbgden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeatures {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?}, cpu1_dbgen: {:?}, cpu1_niden: {:?}, dsp_dbgden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden(),
            self.cpu1_dbgen(),
            self.cpu1_niden(),
            self.dsp_dbgden()
        )
    }
}
#[doc = "Cortex Debug Features Control (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeaturesDp(pub u32);
impl DebugFeaturesDp {
    #[doc = "CPU0 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesDpCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesDpCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesDpCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesDpCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> super::vals::DebugFeaturesDpCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: super::vals::DebugFeaturesDpCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> super::vals::DebugFeaturesDpCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: super::vals::DebugFeaturesDpCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CPU1 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> super::vals::DebugFeaturesDpCpu1Dbgen {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DebugFeaturesDpCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: super::vals::DebugFeaturesDpCpu1Dbgen) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU1 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> super::vals::DebugFeaturesDpCpu1Niden {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::DebugFeaturesDpCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: super::vals::DebugFeaturesDpCpu1Niden) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "DSP invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_dbgen(&self) -> super::vals::DspDbgen {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::DspDbgen::from_bits(val as u8)
    }
    #[doc = "DSP invasive debug control"]
    #[inline(always)]
    pub const fn set_dsp_dbgen(&mut self, val: super::vals::DspDbgen) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
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
            .field("dsp_dbgen", &self.dsp_dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeaturesDp {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?}, cpu1_dbgen: {:?}, cpu1_niden: {:?}, dsp_dbgen: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden(),
            self.cpu1_dbgen(),
            self.cpu1_niden(),
            self.dsp_dbgen()
        )
    }
}
#[doc = "Control Write Access to Security"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLockEn(pub u32);
impl DebugLockEn {
    #[doc = "Controls write access to the security registers"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_all(&self) -> super::vals::LockAll {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LockAll::from_bits(val as u8)
    }
    #[doc = "Controls write access to the security registers"]
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
#[doc = "Device Type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Indicates DEVICE TYPE."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates DEVICE TYPE."]
    #[inline(always)]
    pub const fn set_device_type(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type", &self.device_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type: {=u32:?} }}",
            self.device_type()
        )
    }
}
#[doc = "Chip Revision ID and Number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieid(pub u32);
impl Dieid {
    #[doc = "Chip minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn minor_revision(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip minor revision"]
    #[inline(always)]
    pub const fn set_minor_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Chip major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn major_revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip major revision"]
    #[inline(always)]
    pub const fn set_major_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Chip number"]
    #[must_use]
    #[inline(always)]
    pub const fn mco_num_in_die_id(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Chip number"]
    #[inline(always)]
    pub const fn set_mco_num_in_die_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 8usize)) | (((val as u32) & 0x000f_ffff) << 8usize);
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
            .field("minor_revision", &self.minor_revision())
            .field("major_revision", &self.major_revision())
            .field("mco_num_in_die_id", &self.mco_num_in_die_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieid {{ minor_revision: {=u8:?}, major_revision: {=u8:?}, mco_num_in_die_id: {=u32:?} }}",
            self.minor_revision(),
            self.major_revision(),
            self.mco_num_in_die_id()
        )
    }
}
#[doc = "RAM ECC Enable Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccEnableCtrl(pub u32);
impl EccEnableCtrl {
    #[doc = "RAMA ECC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rama_ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA ECC enable"]
    #[inline(always)]
    pub const fn set_rama_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMB and RAMX ECC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_ramx_ecc_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB and RAMX ECC enable"]
    #[inline(always)]
    pub const fn set_ramb_ramx_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMD and RAMC ECC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ramd_ramc_ecc_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMD and RAMC ECC enable"]
    #[inline(always)]
    pub const fn set_ramd_ramc_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMF and RAME ECC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ramf_rame_ecc_enable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMF and RAME ECC enable"]
    #[inline(always)]
    pub const fn set_ramf_rame_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for EccEnableCtrl {
    #[inline(always)]
    fn default() -> EccEnableCtrl {
        EccEnableCtrl(0)
    }
}
impl core::fmt::Debug for EccEnableCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EccEnableCtrl")
            .field("rama_ecc_enable", &self.rama_ecc_enable())
            .field("ramb_ramx_ecc_enable", &self.ramb_ramx_ecc_enable())
            .field("ramd_ramc_ecc_enable", &self.ramd_ramc_ecc_enable())
            .field("ramf_rame_ecc_enable", &self.ramf_rame_ecc_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EccEnableCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EccEnableCtrl {{ rama_ecc_enable: {=bool:?}, ramb_ramx_ecc_enable: {=bool:?}, ramd_ramc_ecc_enable: {=bool:?}, ramf_rame_ecc_enable: {=bool:?} }}",
            self.rama_ecc_enable(),
            self.ramb_ramx_ecc_enable(),
            self.ramd_ramc_ecc_enable(),
            self.ramf_rame_ecc_enable()
        )
    }
}
#[doc = "Boot state captured during boot: Main ROM log"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog0(pub u32);
impl ElsAsBootLog0 {
    #[doc = "Boot image source used during this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_image(&self) -> super::vals::BootImage {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::BootImage::from_bits(val as u8)
    }
    #[doc = "Boot image source used during this boot."]
    #[inline(always)]
    pub const fn set_boot_image(&mut self, val: super::vals::BootImage) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "CMAC verify is used instead of ECDSA verify on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn cmac(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC verify is used instead of ECDSA verify on this boot."]
    #[inline(always)]
    pub const fn set_cmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ECDSA P-384 verification is done on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn ecdsa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ECDSA P-384 verification is done on this boot."]
    #[inline(always)]
    pub const fn set_ecdsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Off-chip Prince is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn off_chip(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Off-chip Prince is enabled during boot."]
    #[inline(always)]
    pub const fn set_off_chip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "On-chip Prince is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn on_chip(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "On-chip Prince is enabled during boot."]
    #[inline(always)]
    pub const fn set_on_chip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CDI based device keys are derived for CSR harvesting on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn cdi_csr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CDI based device keys are derived for CSR harvesting on this boot."]
    #[inline(always)]
    pub const fn set_cdi_csr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CDI per DICE specification is computed on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn cdi_dice(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CDI per DICE specification is computed on this boot."]
    #[inline(always)]
    pub const fn set_cdi_dice(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TrustZone preset data is loaded during this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn trustzone(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TrustZone preset data is loaded during this boot."]
    #[inline(always)]
    pub const fn set_trustzone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Debug authentication done in this session prior to boot."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_auth(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Debug authentication done in this session prior to boot."]
    #[inline(always)]
    pub const fn set_debug_auth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ITRC zeroize event is handled in this session of boot."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC zeroize event is handled in this session of boot."]
    #[inline(always)]
    pub const fn set_itrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Digital glitch detector is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn dig_gdet(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Digital glitch detector is enabled during boot."]
    #[inline(always)]
    pub const fn set_dig_gdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog glitch detector is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn ana_gdet(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog glitch detector is enabled during boot."]
    #[inline(always)]
    pub const fn set_ana_gdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Boot from deep-power down state."]
    #[must_use]
    #[inline(always)]
    pub const fn deep_pd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Boot from deep-power down state."]
    #[inline(always)]
    pub const fn set_deep_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Last low-power mode value. ROM copies SPC_LP_MODE field from SPC->SC\\[7:4\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn low_power(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Last low-power mode value. ROM copies SPC_LP_MODE field from SPC->SC\\[7:4\\]."]
    #[inline(always)]
    pub const fn set_low_power(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "ISP pin state at boot time. ROM copies CMC->MR0\\[0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn isp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ISP pin state at boot time. ROM copies CMC->MR0\\[0\\]."]
    #[inline(always)]
    pub const fn set_isp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ElsAsBootLog0 {
    #[inline(always)]
    fn default() -> ElsAsBootLog0 {
        ElsAsBootLog0(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog0")
            .field("boot_image", &self.boot_image())
            .field("cmac", &self.cmac())
            .field("ecdsa", &self.ecdsa())
            .field("off_chip", &self.off_chip())
            .field("on_chip", &self.on_chip())
            .field("cdi_csr", &self.cdi_csr())
            .field("cdi_dice", &self.cdi_dice())
            .field("trustzone", &self.trustzone())
            .field("debug_auth", &self.debug_auth())
            .field("itrc", &self.itrc())
            .field("dig_gdet", &self.dig_gdet())
            .field("ana_gdet", &self.ana_gdet())
            .field("deep_pd", &self.deep_pd())
            .field("low_power", &self.low_power())
            .field("isp", &self.isp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog0 {{ boot_image: {:?}, cmac: {=bool:?}, ecdsa: {=bool:?}, off_chip: {=bool:?}, on_chip: {=bool:?}, cdi_csr: {=bool:?}, cdi_dice: {=bool:?}, trustzone: {=bool:?}, debug_auth: {=bool:?}, itrc: {=bool:?}, dig_gdet: {=bool:?}, ana_gdet: {=bool:?}, deep_pd: {=bool:?}, low_power: {=u8:?}, isp: {=bool:?} }}",
            self.boot_image(),
            self.cmac(),
            self.ecdsa(),
            self.off_chip(),
            self.on_chip(),
            self.cdi_csr(),
            self.cdi_dice(),
            self.trustzone(),
            self.debug_auth(),
            self.itrc(),
            self.dig_gdet(),
            self.ana_gdet(),
            self.deep_pd(),
            self.low_power(),
            self.isp()
        )
    }
}
#[doc = "Boot state captured during boot: Library log"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog1(pub u32);
impl ElsAsBootLog1 {
    #[doc = "RoTK index used for this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn ro_tk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "RoTK index used for this boot."]
    #[inline(always)]
    pub const fn set_ro_tk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "FIPS self-test is executed and PASS during this boot. When a bit is set, means self-test is executed and it FAILS. When a bit is clear, means corresponding self-test is executed and PASS or it is not executed."]
    #[must_use]
    #[inline(always)]
    pub const fn fips(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0xff;
        val as u8
    }
    #[doc = "FIPS self-test is executed and PASS during this boot. When a bit is set, means self-test is executed and it FAILS. When a bit is clear, means corresponding self-test is executed and PASS or it is not executed."]
    #[inline(always)]
    pub const fn set_fips(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 2usize)) | (((val as u32) & 0xff) << 2usize);
    }
    #[doc = "SB3 type (valid after nboot_sb3_load_manifest())."]
    #[must_use]
    #[inline(always)]
    pub const fn sb3(&self) -> super::vals::Sb3 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sb3::from_bits(val as u8)
    }
    #[doc = "SB3 type (valid after nboot_sb3_load_manifest())."]
    #[inline(always)]
    pub const fn set_sb3(&mut self, val: super::vals::Sb3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for ElsAsBootLog1 {
    #[inline(always)]
    fn default() -> ElsAsBootLog1 {
        ElsAsBootLog1(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog1")
            .field("ro_tk", &self.ro_tk())
            .field("fips", &self.fips())
            .field("sb3", &self.sb3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog1 {{ ro_tk: {=u8:?}, fips: {=u8:?}, sb3: {:?} }}",
            self.ro_tk(),
            self.fips(),
            self.sb3()
        )
    }
}
#[doc = "Boot state captured during boot: Hardware status signals log"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog2(pub u32);
impl ElsAsBootLog2 {
    #[doc = "CMC->SRS\\[5:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn cmc_srs0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CMC->SRS\\[5:0\\]"]
    #[inline(always)]
    pub const fn set_cmc_srs0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "VBAT->STATUSA\\[1:0\\] | ~VBAT->STATUSB\\[1:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat_status0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "VBAT->STATUSA\\[1:0\\] | ~VBAT->STATUSB\\[1:0\\]"]
    #[inline(always)]
    pub const fn set_vbat_status0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "CMC->SRS\\[16:8\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn cmc_srs1(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x01ff;
        val as u16
    }
    #[doc = "CMC->SRS\\[16:8\\]"]
    #[inline(always)]
    pub const fn set_cmc_srs1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
    }
    #[doc = "VBAT->STATUSA\\[11:6\\] | ~VBAT->STATUSB\\[11:6\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat_status1(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "VBAT->STATUSA\\[11:6\\] | ~VBAT->STATUSB\\[11:6\\]"]
    #[inline(always)]
    pub const fn set_vbat_status1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "CMC->SRS\\[31:24\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn cmc_srs2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "CMC->SRS\\[31:24\\]"]
    #[inline(always)]
    pub const fn set_cmc_srs2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ElsAsBootLog2 {
    #[inline(always)]
    fn default() -> ElsAsBootLog2 {
        ElsAsBootLog2(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog2")
            .field("cmc_srs0", &self.cmc_srs0())
            .field("vbat_status0", &self.vbat_status0())
            .field("cmc_srs1", &self.cmc_srs1())
            .field("vbat_status1", &self.vbat_status1())
            .field("cmc_srs2", &self.cmc_srs2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog2 {{ cmc_srs0: {=u8:?}, vbat_status0: {=u8:?}, cmc_srs1: {=u16:?}, vbat_status1: {=u8:?}, cmc_srs2: {=u8:?} }}",
            self.cmc_srs0(),
            self.vbat_status0(),
            self.cmc_srs1(),
            self.vbat_status1(),
            self.cmc_srs2()
        )
    }
}
#[doc = "Boot state captured during boot: Security log"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog3(pub u32);
impl ElsAsBootLog3 {
    #[doc = "CFPA->ERR_AUTH_FAIL_COUNT\\[7:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn err_auth_fail_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CFPA->ERR_AUTH_FAIL_COUNT\\[7:0\\]"]
    #[inline(always)]
    pub const fn set_err_auth_fail_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "CFPA->ERR_ITRC_COUNT\\[7:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn err_itrc_count(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "CFPA->ERR_ITRC_COUNT\\[7:0\\]"]
    #[inline(always)]
    pub const fn set_err_itrc_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ElsAsBootLog3 {
    #[inline(always)]
    fn default() -> ElsAsBootLog3 {
        ElsAsBootLog3(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog3")
            .field("err_auth_fail_count", &self.err_auth_fail_count())
            .field("err_itrc_count", &self.err_itrc_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog3 {{ err_auth_fail_count: {=u8:?}, err_itrc_count: {=u8:?} }}",
            self.err_auth_fail_count(),
            self.err_itrc_count()
        )
    }
}
#[doc = "ELS AS Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg0(pub u32);
impl ElsAsCfg0 {
    #[doc = "LC state configuration bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LC state configuration bit"]
    #[inline(always)]
    pub const fn set_cfg_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_core_reset_enabled(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_core_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD IRQ are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_core_irq_enabled(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD IRQ are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_core_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When WatchDog Timer 0 is activated, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_wdt0_enabled(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When WatchDog Timer 0 is activated, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_wdt0_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When Code WatchDog Timer 0 is activated, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_cwdt0_enabled(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "When Code WatchDog Timer 0 is activated, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_cwdt0_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "When either GDET is enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_els_gdet_enabled(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "When either GDET is enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_els_gdet_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When SPC analog glitch detect reset is enabled, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_ana_gdet_reset_enabled(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC analog glitch detect reset is enabled, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_ana_gdet_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When SPC analog glitch detect IRQ is enabled, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_ana_gdet_irq_enabled(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC analog glitch detect IRQ is enabled, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_ana_gdet_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When tamper detector is enabled in TDET, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_tamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When tamper detector is enabled in TDET, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_tamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vsys_reset_enabled(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vsys_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vddio_reset_enabled(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vddio_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vsys_irq_enabled(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vsys_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vddio_irq_enabled(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vddio_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "When WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_wdt1_enabled(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "When WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_wdt1_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When Code WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_cwdt1_enabled(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "When Code WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_cwdt1_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "When temperature tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_temptamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When temperature tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_temptamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "When voltage tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_voltamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "When voltage tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_voltamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "When light tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lhttamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "When light tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lhttamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When clk tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_clktamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "When clk tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_clktamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "When QK PUF \"qk_disable_enroll\" input is driven 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_qk_disable_enroll(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When QK PUF \"qk_disable_enroll\" input is driven 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_qk_disable_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "When QK PUF \"qk_disable_wrap\" input is driven 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_qk_disable_wrap(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "When QK PUF \"qk_disable_wrap\" input is driven 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_qk_disable_wrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for ElsAsCfg0 {
    #[inline(always)]
    fn default() -> ElsAsCfg0 {
        ElsAsCfg0(0)
    }
}
impl core::fmt::Debug for ElsAsCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg0")
            .field("cfg_lc_state", &self.cfg_lc_state())
            .field(
                "cfg_lvd_core_reset_enabled",
                &self.cfg_lvd_core_reset_enabled(),
            )
            .field("cfg_lvd_core_irq_enabled", &self.cfg_lvd_core_irq_enabled())
            .field("cfg_wdt0_enabled", &self.cfg_wdt0_enabled())
            .field("cfg_cwdt0_enabled", &self.cfg_cwdt0_enabled())
            .field("cfg_els_gdet_enabled", &self.cfg_els_gdet_enabled())
            .field(
                "cfg_ana_gdet_reset_enabled",
                &self.cfg_ana_gdet_reset_enabled(),
            )
            .field("cfg_ana_gdet_irq_enabled", &self.cfg_ana_gdet_irq_enabled())
            .field("cfg_tamper_det_enabled", &self.cfg_tamper_det_enabled())
            .field(
                "cfg_lvd_vsys_reset_enabled",
                &self.cfg_lvd_vsys_reset_enabled(),
            )
            .field(
                "cfg_lvd_vddio_reset_enabled",
                &self.cfg_lvd_vddio_reset_enabled(),
            )
            .field("cfg_lvd_vsys_irq_enabled", &self.cfg_lvd_vsys_irq_enabled())
            .field(
                "cfg_lvd_vddio_irq_enabled",
                &self.cfg_lvd_vddio_irq_enabled(),
            )
            .field("cfg_wdt1_enabled", &self.cfg_wdt1_enabled())
            .field("cfg_cwdt1_enabled", &self.cfg_cwdt1_enabled())
            .field(
                "cfg_temptamper_det_enabled",
                &self.cfg_temptamper_det_enabled(),
            )
            .field(
                "cfg_voltamper_det_enabled",
                &self.cfg_voltamper_det_enabled(),
            )
            .field(
                "cfg_lhttamper_det_enabled",
                &self.cfg_lhttamper_det_enabled(),
            )
            .field(
                "cfg_clktamper_det_enabled",
                &self.cfg_clktamper_det_enabled(),
            )
            .field("cfg_qk_disable_enroll", &self.cfg_qk_disable_enroll())
            .field("cfg_qk_disable_wrap", &self.cfg_qk_disable_wrap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg0 {{ cfg_lc_state: {=u8:?}, cfg_lvd_core_reset_enabled: {=bool:?}, cfg_lvd_core_irq_enabled: {=bool:?}, cfg_wdt0_enabled: {=bool:?}, cfg_cwdt0_enabled: {=bool:?}, cfg_els_gdet_enabled: {=bool:?}, cfg_ana_gdet_reset_enabled: {=bool:?}, cfg_ana_gdet_irq_enabled: {=bool:?}, cfg_tamper_det_enabled: {=bool:?}, cfg_lvd_vsys_reset_enabled: {=bool:?}, cfg_lvd_vddio_reset_enabled: {=bool:?}, cfg_lvd_vsys_irq_enabled: {=bool:?}, cfg_lvd_vddio_irq_enabled: {=bool:?}, cfg_wdt1_enabled: {=bool:?}, cfg_cwdt1_enabled: {=bool:?}, cfg_temptamper_det_enabled: {=bool:?}, cfg_voltamper_det_enabled: {=bool:?}, cfg_lhttamper_det_enabled: {=bool:?}, cfg_clktamper_det_enabled: {=bool:?}, cfg_qk_disable_enroll: {=bool:?}, cfg_qk_disable_wrap: {=bool:?} }}",
            self.cfg_lc_state(),
            self.cfg_lvd_core_reset_enabled(),
            self.cfg_lvd_core_irq_enabled(),
            self.cfg_wdt0_enabled(),
            self.cfg_cwdt0_enabled(),
            self.cfg_els_gdet_enabled(),
            self.cfg_ana_gdet_reset_enabled(),
            self.cfg_ana_gdet_irq_enabled(),
            self.cfg_tamper_det_enabled(),
            self.cfg_lvd_vsys_reset_enabled(),
            self.cfg_lvd_vddio_reset_enabled(),
            self.cfg_lvd_vsys_irq_enabled(),
            self.cfg_lvd_vddio_irq_enabled(),
            self.cfg_wdt1_enabled(),
            self.cfg_cwdt1_enabled(),
            self.cfg_temptamper_det_enabled(),
            self.cfg_voltamper_det_enabled(),
            self.cfg_lhttamper_det_enabled(),
            self.cfg_clktamper_det_enabled(),
            self.cfg_qk_disable_enroll(),
            self.cfg_qk_disable_wrap()
        )
    }
}
#[doc = "ELS AS Configuration1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg1(pub u32);
impl ElsAsCfg1 {
    #[doc = "When CFG_SEC_ENA_SEC_CHK indicates state 0 or when DISABLE_STRICT_MODE bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_dis_strict_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When CFG_SEC_ENA_SEC_CHK indicates state 0 or when DISABLE_STRICT_MODE bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_dis_strict_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When the DISABLE_VIOLATION_ABORT bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_dis_viol_abort(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When the DISABLE_VIOLATION_ABORT bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_dis_viol_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When the ENABLE_NS_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_ena_ns_priv_chk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When the ENABLE_NS_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_ena_ns_priv_chk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When the ENABLE_S_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_ena_s_priv_chk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When the ENABLE_S_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_ena_s_priv_chk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When the ENABLE_SECURE_CHECKING bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_ena_sec_chk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When the ENABLE_SECURE_CHECKING bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_ena_sec_chk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When the IDAU_ALL_NS bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_idau_allns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When the IDAU_ALL_NS bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_idau_allns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When the LOCK_NS_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_ns_mpu(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_NS_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_ns_mpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When the LOCK_NS_VTOR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_ns_vtor(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_NS_VTOR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_ns_vtor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When the LOCK_S_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_s_mpu(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_S_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_s_mpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When the LOCK_S_VTAIRCR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_s_vtaircr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_S_VTAIRCR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_s_vtaircr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When the LOCK_SAU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_sau(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_SAU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_sau(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "metal version"]
    #[must_use]
    #[inline(always)]
    pub const fn metal_version(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0xff;
        val as u8
    }
    #[doc = "metal version"]
    #[inline(always)]
    pub const fn set_metal_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 13usize)) | (((val as u32) & 0xff) << 13usize);
    }
    #[doc = "ROM patch version"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_patch_version(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM patch version"]
    #[inline(always)]
    pub const fn set_rom_patch_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_core_reset_enabled(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_core_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD IRQ are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_core_irq_enabled(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD IRQ are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_core_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vsys_reset_enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vsys_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vddio_reset_enabled(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vddio_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vsys_irq_enabled(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vsys_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vddio_irq_enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vddio_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ElsAsCfg1 {
    #[inline(always)]
    fn default() -> ElsAsCfg1 {
        ElsAsCfg1(0)
    }
}
impl core::fmt::Debug for ElsAsCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg1")
            .field("cfg_sec_dis_strict_mode", &self.cfg_sec_dis_strict_mode())
            .field("cfg_sec_dis_viol_abort", &self.cfg_sec_dis_viol_abort())
            .field("cfg_sec_ena_ns_priv_chk", &self.cfg_sec_ena_ns_priv_chk())
            .field("cfg_sec_ena_s_priv_chk", &self.cfg_sec_ena_s_priv_chk())
            .field("cfg_sec_ena_sec_chk", &self.cfg_sec_ena_sec_chk())
            .field("cfg_sec_idau_allns", &self.cfg_sec_idau_allns())
            .field("cfg_sec_lock_ns_mpu", &self.cfg_sec_lock_ns_mpu())
            .field("cfg_sec_lock_ns_vtor", &self.cfg_sec_lock_ns_vtor())
            .field("cfg_sec_lock_s_mpu", &self.cfg_sec_lock_s_mpu())
            .field("cfg_sec_lock_s_vtaircr", &self.cfg_sec_lock_s_vtaircr())
            .field("cfg_sec_lock_sau", &self.cfg_sec_lock_sau())
            .field("metal_version", &self.metal_version())
            .field("rom_patch_version", &self.rom_patch_version())
            .field(
                "cfg_hvd_core_reset_enabled",
                &self.cfg_hvd_core_reset_enabled(),
            )
            .field("cfg_hvd_core_irq_enabled", &self.cfg_hvd_core_irq_enabled())
            .field(
                "cfg_hvd_vsys_reset_enabled",
                &self.cfg_hvd_vsys_reset_enabled(),
            )
            .field(
                "cfg_hvd_vddio_reset_enabled",
                &self.cfg_hvd_vddio_reset_enabled(),
            )
            .field("cfg_hvd_vsys_irq_enabled", &self.cfg_hvd_vsys_irq_enabled())
            .field(
                "cfg_hvd_vddio_irq_enabled",
                &self.cfg_hvd_vddio_irq_enabled(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg1 {{ cfg_sec_dis_strict_mode: {=bool:?}, cfg_sec_dis_viol_abort: {=bool:?}, cfg_sec_ena_ns_priv_chk: {=bool:?}, cfg_sec_ena_s_priv_chk: {=bool:?}, cfg_sec_ena_sec_chk: {=bool:?}, cfg_sec_idau_allns: {=bool:?}, cfg_sec_lock_ns_mpu: {=bool:?}, cfg_sec_lock_ns_vtor: {=bool:?}, cfg_sec_lock_s_mpu: {=bool:?}, cfg_sec_lock_s_vtaircr: {=bool:?}, cfg_sec_lock_sau: {=bool:?}, metal_version: {=u8:?}, rom_patch_version: {=u8:?}, cfg_hvd_core_reset_enabled: {=bool:?}, cfg_hvd_core_irq_enabled: {=bool:?}, cfg_hvd_vsys_reset_enabled: {=bool:?}, cfg_hvd_vddio_reset_enabled: {=bool:?}, cfg_hvd_vsys_irq_enabled: {=bool:?}, cfg_hvd_vddio_irq_enabled: {=bool:?} }}",
            self.cfg_sec_dis_strict_mode(),
            self.cfg_sec_dis_viol_abort(),
            self.cfg_sec_ena_ns_priv_chk(),
            self.cfg_sec_ena_s_priv_chk(),
            self.cfg_sec_ena_sec_chk(),
            self.cfg_sec_idau_allns(),
            self.cfg_sec_lock_ns_mpu(),
            self.cfg_sec_lock_ns_vtor(),
            self.cfg_sec_lock_s_mpu(),
            self.cfg_sec_lock_s_vtaircr(),
            self.cfg_sec_lock_sau(),
            self.metal_version(),
            self.rom_patch_version(),
            self.cfg_hvd_core_reset_enabled(),
            self.cfg_hvd_core_irq_enabled(),
            self.cfg_hvd_vsys_reset_enabled(),
            self.cfg_hvd_vddio_reset_enabled(),
            self.cfg_hvd_vsys_irq_enabled(),
            self.cfg_hvd_vddio_irq_enabled()
        )
    }
}
#[doc = "ELS AS Configuration2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg2(pub u32);
impl ElsAsCfg2 {
    #[doc = "ELS configuration command enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_els_cmd_en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ELS configuration command enable bit"]
    #[inline(always)]
    pub const fn set_cfg_els_cmd_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsAsCfg2 {
    #[inline(always)]
    fn default() -> ElsAsCfg2 {
        ElsAsCfg2(0)
    }
}
impl core::fmt::Debug for ElsAsCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg2")
            .field("cfg_els_cmd_en", &self.cfg_els_cmd_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg2 {{ cfg_els_cmd_en: {=u32:?} }}",
            self.cfg_els_cmd_en()
        )
    }
}
#[doc = "ELS AS Configuration3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg3(pub u32);
impl ElsAsCfg3 {
    #[doc = "Device type identification data"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Device type identification data"]
    #[inline(always)]
    pub const fn set_device_type(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsAsCfg3 {
    #[inline(always)]
    fn default() -> ElsAsCfg3 {
        ElsAsCfg3(0)
    }
}
impl core::fmt::Debug for ElsAsCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg3")
            .field("device_type", &self.device_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg3 {{ device_type: {=u32:?} }}",
            self.device_type()
        )
    }
}
#[doc = "ELS AS Flag0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsFlag0(pub u32);
impl ElsAsFlag0 {
    #[doc = "This flag bit is set as 1 when DAP enables AP0 for CPU0 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ap_enable_cpu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP0 for CPU0 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ap_enable_cpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP1 for CPU1 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ap_enable_cpu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP1 for CPU1 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ap_enable_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP3 for DSP (CoolFlux) debug access. The register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ap_enable_dsp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP3 for DSP (CoolFlux) debug access. The register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ap_enable_dsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OTPC can output attack_detect signal when it detects attack when load shadow registers. The output will be cleared by reset. ELS_AS_FLAG is reset by PoR, so the status can be recorded."]
    #[must_use]
    #[inline(always)]
    pub const fn efuse_attack_detect(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OTPC can output attack_detect signal when it detects attack when load shadow registers. The output will be cleared by reset. ELS_AS_FLAG is reset by PoR, so the status can be recorded."]
    #[inline(always)]
    pub const fn set_efuse_attack_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This flag register is set 1 when VDD_CORE LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lvd_core_occured(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This flag register is set 1 when VDD_CORE LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lvd_core_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt0_reset_occured(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_wdt0_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt0_reset_occured(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_cwdt0_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt0_irq_occured(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_wdt0_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt0_irq_occured(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cwdt0_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This flag bit is set as 1 when QK_ERROR is flagged from QK PUF block. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_qk_error(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when QK_ERROR is flagged from QK PUF block. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_qk_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This flag bit is set as 1 when GDET error is flagged. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_els_glitch_detected(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when GDET error is flagged. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_els_glitch_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This flag bit is set as 1 when ANALOG GDET error is flagged in SYSCON block. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ana_glitch_detected(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when ANALOG GDET error is flagged in SYSCON block. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ana_glitch_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This flag bit is set as 1 when tamper event is flagged from TDET. This register is cleared 0 by AO domain POR or by PMC reset event, if tamper detection event is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_tamper_event_detected(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when tamper event is flagged from TDET. This register is cleared 0 by AO domain POR or by PMC reset event, if tamper detection event is cleared by software."]
    #[inline(always)]
    pub const fn set_flag_tamper_event_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This flag bit is set as 1 when FLASH controller indicates ECC error. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_flash_ecc_invalid(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when FLASH controller indicates ECC error. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_flash_ecc_invalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This flag bit is set as 1 when security violation is indicated from FLASH sub-system or AHB bus matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_sec_viol_irq_ocurred(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when security violation is indicated from FLASH sub-system or AHB bus matrix."]
    #[inline(always)]
    pub const fn set_flag_sec_viol_irq_ocurred(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure code transactions. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cpu0_ns_c_acc_occured(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure code transactions. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cpu0_ns_c_acc_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure data transactions. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cpu0_ns_d_acc_occured(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure data transactions. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cpu0_ns_d_acc_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "This flag register is set 1 when VDD_SYS LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lvd_vsys_occured(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "This flag register is set 1 when VDD_SYS LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lvd_vsys_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This flag register is set 1 when VDD LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lvd_vddio_occured(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This flag register is set 1 when VDD LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lvd_vddio_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt1_reset_occured(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_wdt1_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt1_reset_occured(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_cwdt1_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt1_irq_occured(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_wdt1_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt1_irq_occured(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cwdt1_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This flag bit is set as 1 when temperature temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_temptamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when temperature temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_temptamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This flag bit is set as 1 when voltage temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_voltamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when voltage temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_voltamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This flag bit is set as 1 when light temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lhttamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when light temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lhttamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This flag bit is set as 1 when clock temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_clktamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when clock temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_clktamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for ElsAsFlag0 {
    #[inline(always)]
    fn default() -> ElsAsFlag0 {
        ElsAsFlag0(0)
    }
}
impl core::fmt::Debug for ElsAsFlag0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsFlag0")
            .field("flag_ap_enable_cpu0", &self.flag_ap_enable_cpu0())
            .field("flag_ap_enable_cpu1", &self.flag_ap_enable_cpu1())
            .field("flag_ap_enable_dsp", &self.flag_ap_enable_dsp())
            .field("efuse_attack_detect", &self.efuse_attack_detect())
            .field("flag_lvd_core_occured", &self.flag_lvd_core_occured())
            .field("flag_wdt0_reset_occured", &self.flag_wdt0_reset_occured())
            .field("flag_cwdt0_reset_occured", &self.flag_cwdt0_reset_occured())
            .field("flag_wdt0_irq_occured", &self.flag_wdt0_irq_occured())
            .field("flag_cwdt0_irq_occured", &self.flag_cwdt0_irq_occured())
            .field("flag_qk_error", &self.flag_qk_error())
            .field("flag_els_glitch_detected", &self.flag_els_glitch_detected())
            .field("flag_ana_glitch_detected", &self.flag_ana_glitch_detected())
            .field(
                "flag_tamper_event_detected",
                &self.flag_tamper_event_detected(),
            )
            .field("flag_flash_ecc_invalid", &self.flag_flash_ecc_invalid())
            .field(
                "flag_sec_viol_irq_ocurred",
                &self.flag_sec_viol_irq_ocurred(),
            )
            .field(
                "flag_cpu0_ns_c_acc_occured",
                &self.flag_cpu0_ns_c_acc_occured(),
            )
            .field(
                "flag_cpu0_ns_d_acc_occured",
                &self.flag_cpu0_ns_d_acc_occured(),
            )
            .field("flag_lvd_vsys_occured", &self.flag_lvd_vsys_occured())
            .field("flag_lvd_vddio_occured", &self.flag_lvd_vddio_occured())
            .field("flag_wdt1_reset_occured", &self.flag_wdt1_reset_occured())
            .field("flag_cwdt1_reset_occured", &self.flag_cwdt1_reset_occured())
            .field("flag_wdt1_irq_occured", &self.flag_wdt1_irq_occured())
            .field("flag_cwdt1_irq_occured", &self.flag_cwdt1_irq_occured())
            .field(
                "flag_temptamper_det_irq_occured",
                &self.flag_temptamper_det_irq_occured(),
            )
            .field(
                "flag_voltamper_det_irq_occured",
                &self.flag_voltamper_det_irq_occured(),
            )
            .field(
                "flag_lhttamper_det_irq_occured",
                &self.flag_lhttamper_det_irq_occured(),
            )
            .field(
                "flag_clktamper_det_irq_occured",
                &self.flag_clktamper_det_irq_occured(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsFlag0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsFlag0 {{ flag_ap_enable_cpu0: {=bool:?}, flag_ap_enable_cpu1: {=bool:?}, flag_ap_enable_dsp: {=bool:?}, efuse_attack_detect: {=bool:?}, flag_lvd_core_occured: {=bool:?}, flag_wdt0_reset_occured: {=bool:?}, flag_cwdt0_reset_occured: {=bool:?}, flag_wdt0_irq_occured: {=bool:?}, flag_cwdt0_irq_occured: {=bool:?}, flag_qk_error: {=bool:?}, flag_els_glitch_detected: {=bool:?}, flag_ana_glitch_detected: {=bool:?}, flag_tamper_event_detected: {=bool:?}, flag_flash_ecc_invalid: {=bool:?}, flag_sec_viol_irq_ocurred: {=bool:?}, flag_cpu0_ns_c_acc_occured: {=bool:?}, flag_cpu0_ns_d_acc_occured: {=bool:?}, flag_lvd_vsys_occured: {=bool:?}, flag_lvd_vddio_occured: {=bool:?}, flag_wdt1_reset_occured: {=bool:?}, flag_cwdt1_reset_occured: {=bool:?}, flag_wdt1_irq_occured: {=bool:?}, flag_cwdt1_irq_occured: {=bool:?}, flag_temptamper_det_irq_occured: {=bool:?}, flag_voltamper_det_irq_occured: {=bool:?}, flag_lhttamper_det_irq_occured: {=bool:?}, flag_clktamper_det_irq_occured: {=bool:?} }}",
            self.flag_ap_enable_cpu0(),
            self.flag_ap_enable_cpu1(),
            self.flag_ap_enable_dsp(),
            self.efuse_attack_detect(),
            self.flag_lvd_core_occured(),
            self.flag_wdt0_reset_occured(),
            self.flag_cwdt0_reset_occured(),
            self.flag_wdt0_irq_occured(),
            self.flag_cwdt0_irq_occured(),
            self.flag_qk_error(),
            self.flag_els_glitch_detected(),
            self.flag_ana_glitch_detected(),
            self.flag_tamper_event_detected(),
            self.flag_flash_ecc_invalid(),
            self.flag_sec_viol_irq_ocurred(),
            self.flag_cpu0_ns_c_acc_occured(),
            self.flag_cpu0_ns_d_acc_occured(),
            self.flag_lvd_vsys_occured(),
            self.flag_lvd_vddio_occured(),
            self.flag_wdt1_reset_occured(),
            self.flag_cwdt1_reset_occured(),
            self.flag_wdt1_irq_occured(),
            self.flag_cwdt1_irq_occured(),
            self.flag_temptamper_det_irq_occured(),
            self.flag_voltamper_det_irq_occured(),
            self.flag_lhttamper_det_irq_occured(),
            self.flag_clktamper_det_irq_occured()
        )
    }
}
#[doc = "ELS AS Flag1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsFlag1(pub u32);
impl ElsAsFlag1 {
    #[doc = "This flag bit is set as 1 when HVD from VDD_CORE power domain is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_hvd_core_occured(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_CORE power domain is triggered."]
    #[inline(always)]
    pub const fn set_flag_hvd_core_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_SYS power domain is triggered"]
    #[must_use]
    #[inline(always)]
    pub const fn flag_hvd_vsys_occured(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_SYS power domain is triggered"]
    #[inline(always)]
    pub const fn set_flag_hvd_vsys_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD power domain is triggered"]
    #[must_use]
    #[inline(always)]
    pub const fn flag_hvd_vddio_occured(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD power domain is triggered"]
    #[inline(always)]
    pub const fn set_flag_hvd_vddio_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ElsAsFlag1 {
    #[inline(always)]
    fn default() -> ElsAsFlag1 {
        ElsAsFlag1(0)
    }
}
impl core::fmt::Debug for ElsAsFlag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsFlag1")
            .field("flag_hvd_core_occured", &self.flag_hvd_core_occured())
            .field("flag_hvd_vsys_occured", &self.flag_hvd_vsys_occured())
            .field("flag_hvd_vddio_occured", &self.flag_hvd_vddio_occured())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsFlag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsFlag1 {{ flag_hvd_core_occured: {=bool:?}, flag_hvd_vsys_occured: {=bool:?}, flag_hvd_vddio_occured: {=bool:?} }}",
            self.flag_hvd_core_occured(),
            self.flag_hvd_vsys_occured(),
            self.flag_hvd_vddio_occured()
        )
    }
}
#[doc = "ELS AS State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsSt0(pub u32);
impl ElsAsSt0 {
    #[doc = "TEMPORAL_STATE\\[3:0\\] in the ELS_TEMPORAL_STATE register reflects this register"]
    #[must_use]
    #[inline(always)]
    pub const fn st_temporal_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "TEMPORAL_STATE\\[3:0\\] in the ELS_TEMPORAL_STATE register reflects this register"]
    #[inline(always)]
    pub const fn set_st_temporal_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "When CPU0 (CM33) \"deben\" input is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_dbgen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"deben\" input is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_cpu0_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When CPU0 (CM33) \"niden\" input is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_niden(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"niden\" input is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_cpu0_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When CPU0 (CM33) \"spiden\" input is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_spiden(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"spiden\" input is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_cpu0_spiden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When CPU0 (CM33) \"spniden\" input is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_spniden(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"spniden\" input is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_cpu0_spniden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "When CPU1 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu1_dbgen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU1 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu1_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When CPU1 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu1_niden(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU1 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu1_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When DAP to AP0 for CPU0 (CM33) debug access is allowed, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_dap_enable_cpu0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When DAP to AP0 for CPU0 (CM33) debug access is allowed, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_dap_enable_cpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When DAP to AP1 for CPU1 (CM33) debug access is allowed, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dap_enable_cpu1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When DAP to AP1 for CPU1 (CM33) debug access is allowed, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_dap_enable_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When DAP to AP3 for DSP (CoolFlux) debug access is allowed, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_dap_enable_dsp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When DAP to AP3 for DSP (CoolFlux) debug access is allowed, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_dap_enable_dsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When JTAG TAP access is allowed, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_allow_test_access(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "When JTAG TAP access is allowed, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_allow_test_access(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When XO32K oscillation fail flag is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_xo32k_failed(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When XO32K oscillation fail flag is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_xo32k_failed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When XO40M oscillation fail flag is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_xo40m_failed(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When XO40M oscillation fail flag is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_xo40m_failed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When IFR load fail flag is state 1, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_ifr_load_failed(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When IFR load fail flag is state 1, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_ifr_load_failed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GLITCH_DETECT_FLAG is state of 4-bit Glitch Ripple Counter output."]
    #[must_use]
    #[inline(always)]
    pub const fn st_glitch_detect_flag(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "GLITCH_DETECT_FLAG is state of 4-bit Glitch Ripple Counter output."]
    #[inline(always)]
    pub const fn set_st_glitch_detect_flag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
}
impl Default for ElsAsSt0 {
    #[inline(always)]
    fn default() -> ElsAsSt0 {
        ElsAsSt0(0)
    }
}
impl core::fmt::Debug for ElsAsSt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsSt0")
            .field("st_temporal_state", &self.st_temporal_state())
            .field("st_cpu0_dbgen", &self.st_cpu0_dbgen())
            .field("st_cpu0_niden", &self.st_cpu0_niden())
            .field("st_cpu0_spiden", &self.st_cpu0_spiden())
            .field("st_cpu0_spniden", &self.st_cpu0_spniden())
            .field("st_cpu1_dbgen", &self.st_cpu1_dbgen())
            .field("st_cpu1_niden", &self.st_cpu1_niden())
            .field("st_dap_enable_cpu0", &self.st_dap_enable_cpu0())
            .field("st_dap_enable_cpu1", &self.st_dap_enable_cpu1())
            .field("st_dap_enable_dsp", &self.st_dap_enable_dsp())
            .field("st_allow_test_access", &self.st_allow_test_access())
            .field("st_xo32k_failed", &self.st_xo32k_failed())
            .field("st_xo40m_failed", &self.st_xo40m_failed())
            .field("st_ifr_load_failed", &self.st_ifr_load_failed())
            .field("st_glitch_detect_flag", &self.st_glitch_detect_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsSt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsSt0 {{ st_temporal_state: {=u8:?}, st_cpu0_dbgen: {=bool:?}, st_cpu0_niden: {=bool:?}, st_cpu0_spiden: {=bool:?}, st_cpu0_spniden: {=bool:?}, st_cpu1_dbgen: {=bool:?}, st_cpu1_niden: {=bool:?}, st_dap_enable_cpu0: {=bool:?}, st_dap_enable_cpu1: {=bool:?}, st_dap_enable_dsp: {=bool:?}, st_allow_test_access: {=bool:?}, st_xo32k_failed: {=bool:?}, st_xo40m_failed: {=bool:?}, st_ifr_load_failed: {=bool:?}, st_glitch_detect_flag: {=u8:?} }}",
            self.st_temporal_state(),
            self.st_cpu0_dbgen(),
            self.st_cpu0_niden(),
            self.st_cpu0_spiden(),
            self.st_cpu0_spniden(),
            self.st_cpu1_dbgen(),
            self.st_cpu1_niden(),
            self.st_dap_enable_cpu0(),
            self.st_dap_enable_cpu1(),
            self.st_dap_enable_dsp(),
            self.st_allow_test_access(),
            self.st_xo32k_failed(),
            self.st_xo40m_failed(),
            self.st_ifr_load_failed(),
            self.st_glitch_detect_flag()
        )
    }
}
#[doc = "ELS AS State1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsSt1(pub u32);
impl ElsAsSt1 {
    #[doc = "These register bits indicate the state of \"qk_puf_score\\[3:0\\]\" outputs from QK PUF block"]
    #[must_use]
    #[inline(always)]
    pub const fn st_qk_puf_score(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These register bits indicate the state of \"qk_puf_score\\[3:0\\]\" outputs from QK PUF block"]
    #[inline(always)]
    pub const fn set_st_qk_puf_score(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This register bit indicates the state of \"qk_zeroized\" output from QK PUF block"]
    #[must_use]
    #[inline(always)]
    pub const fn st_qk_zeroized(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This register bit indicates the state of \"qk_zeroized\" output from QK PUF block"]
    #[inline(always)]
    pub const fn set_st_qk_zeroized(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When MAIN_CLK is running from external clock source either XO32M, XO32K or GPIO CLKIN, this bit indicates state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn st_main_clk_is_ext(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When MAIN_CLK is running from external clock source either XO32M, XO32K or GPIO CLKIN, this bit indicates state 1"]
    #[inline(always)]
    pub const fn set_st_main_clk_is_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "VOUT\\[1:0\\] setting on DCDC0 register in SPC block will reflect to this register. Default is 1.0V"]
    #[must_use]
    #[inline(always)]
    pub const fn st_dcdc_vout(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "VOUT\\[1:0\\] setting on DCDC0 register in SPC block will reflect to this register. Default is 1.0V"]
    #[inline(always)]
    pub const fn set_st_dcdc_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "DCDC drive strength setting. Default is normal drive."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dcdc_ds(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "DCDC drive strength setting. Default is normal drive."]
    #[inline(always)]
    pub const fn set_st_dcdc_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "ISP pin status during boot. By default ISP pin is pulled up. If want to enter ISP mode during boot, ISP pin should be pull down when out of reset."]
    #[must_use]
    #[inline(always)]
    pub const fn st_boot_mode(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "ISP pin status during boot. By default ISP pin is pulled up. If want to enter ISP mode during boot, ISP pin should be pull down when out of reset."]
    #[inline(always)]
    pub const fn set_st_boot_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "BOOT_RETRY_CNT\\[3:0\\] in the ELS_BOOT_RETRY_CNT register reflects this register"]
    #[must_use]
    #[inline(always)]
    pub const fn st_boot_retry_cnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "BOOT_RETRY_CNT\\[3:0\\] in the ELS_BOOT_RETRY_CNT register reflects this register"]
    #[inline(always)]
    pub const fn set_st_boot_retry_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "VOUT\\[1:0\\] setting on LDO Core register in SPC block will reflect to this register. Default is 1.0V"]
    #[must_use]
    #[inline(always)]
    pub const fn st_ldo_core_vout(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "VOUT\\[1:0\\] setting on LDO Core register in SPC block will reflect to this register. Default is 1.0V"]
    #[inline(always)]
    pub const fn set_st_ldo_core_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "LDO_CORE drive strength setting. Default is normal drive."]
    #[must_use]
    #[inline(always)]
    pub const fn st_ldo_core_ds(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "LDO_CORE drive strength setting. Default is normal drive."]
    #[inline(always)]
    pub const fn set_st_ldo_core_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for ElsAsSt1 {
    #[inline(always)]
    fn default() -> ElsAsSt1 {
        ElsAsSt1(0)
    }
}
impl core::fmt::Debug for ElsAsSt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsSt1")
            .field("st_qk_puf_score", &self.st_qk_puf_score())
            .field("st_qk_zeroized", &self.st_qk_zeroized())
            .field("st_main_clk_is_ext", &self.st_main_clk_is_ext())
            .field("st_dcdc_vout", &self.st_dcdc_vout())
            .field("st_dcdc_ds", &self.st_dcdc_ds())
            .field("st_boot_mode", &self.st_boot_mode())
            .field("st_boot_retry_cnt", &self.st_boot_retry_cnt())
            .field("st_ldo_core_vout", &self.st_ldo_core_vout())
            .field("st_ldo_core_ds", &self.st_ldo_core_ds())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsSt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsSt1 {{ st_qk_puf_score: {=u8:?}, st_qk_zeroized: {=bool:?}, st_main_clk_is_ext: {=bool:?}, st_dcdc_vout: {=u8:?}, st_dcdc_ds: {=u8:?}, st_boot_mode: {=u8:?}, st_boot_retry_cnt: {=u8:?}, st_ldo_core_vout: {=u8:?}, st_ldo_core_ds: {=u8:?} }}",
            self.st_qk_puf_score(),
            self.st_qk_zeroized(),
            self.st_main_clk_is_ext(),
            self.st_dcdc_vout(),
            self.st_dcdc_ds(),
            self.st_boot_mode(),
            self.st_boot_retry_cnt(),
            self.st_ldo_core_vout(),
            self.st_ldo_core_ds()
        )
    }
}
#[doc = "ELS Asset Protection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAssetProt(pub u32);
impl ElsAssetProt {
    #[doc = "ELS asset protection. This field controls the asset protection port to the ELS module. Refer to the ELS chapter in the SRM for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn asset_protection(&self) -> super::vals::AssetProtection {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AssetProtection::from_bits(val as u8)
    }
    #[doc = "ELS asset protection. This field controls the asset protection port to the ELS module. Refer to the ELS chapter in the SRM for more details."]
    #[inline(always)]
    pub const fn set_asset_protection(&mut self, val: super::vals::AssetProtection) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for ElsAssetProt {
    #[inline(always)]
    fn default() -> ElsAssetProt {
        ElsAssetProt(0)
    }
}
impl core::fmt::Debug for ElsAssetProt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAssetProt")
            .field("asset_protection", &self.asset_protection())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAssetProt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAssetProt {{ asset_protection: {:?} }}",
            self.asset_protection()
        )
    }
}
#[doc = "Key Derivation Function Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKdfMask(pub u32);
impl ElsKdfMask {
    #[doc = "Key derivation function mask"]
    #[must_use]
    #[inline(always)]
    pub const fn kdf_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key derivation function mask"]
    #[inline(always)]
    pub const fn set_kdf_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsKdfMask {
    #[inline(always)]
    fn default() -> ElsKdfMask {
        ElsKdfMask(0)
    }
}
impl core::fmt::Debug for ElsKdfMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKdfMask")
            .field("kdf_mask", &self.kdf_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKdfMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ElsKdfMask {{ kdf_mask: {=u32:?} }}", self.kdf_mask())
    }
}
#[doc = "ELS Lock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsLockCtrl(pub u32);
impl ElsLockCtrl {
    #[doc = "ELS Lock Control"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ELS Lock Control"]
    #[inline(always)]
    pub const fn set_lock_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for ElsLockCtrl {
    #[inline(always)]
    fn default() -> ElsLockCtrl {
        ElsLockCtrl(0)
    }
}
impl core::fmt::Debug for ElsLockCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsLockCtrl")
            .field("lock_ctrl", &self.lock_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsLockCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ElsLockCtrl {{ lock_ctrl: {=u8:?} }}", self.lock_ctrl())
    }
}
#[doc = "ELS Lock Control DP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsLockCtrlDp(pub u32);
impl ElsLockCtrlDp {
    #[doc = "Refer to ELS_LOCK_CTRL\\[1:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ctrl_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Refer to ELS_LOCK_CTRL\\[1:0\\]"]
    #[inline(always)]
    pub const fn set_lock_ctrl_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for ElsLockCtrlDp {
    #[inline(always)]
    fn default() -> ElsLockCtrlDp {
        ElsLockCtrlDp(0)
    }
}
impl core::fmt::Debug for ElsLockCtrlDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsLockCtrlDp")
            .field("lock_ctrl_dp", &self.lock_ctrl_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsLockCtrlDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsLockCtrlDp {{ lock_ctrl_dp: {=u8:?} }}",
            self.lock_ctrl_dp()
        )
    }
}
#[doc = "Life Cycle State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcState(pub u32);
impl ElsOtpLcState {
    #[doc = "OTP life cycle state"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state"]
    #[inline(always)]
    pub const fn set_otp_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcState {
    #[inline(always)]
    fn default() -> ElsOtpLcState {
        ElsOtpLcState(0)
    }
}
impl core::fmt::Debug for ElsOtpLcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcState")
            .field("otp_lc_state", &self.otp_lc_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcState {{ otp_lc_state: {=u8:?} }}",
            self.otp_lc_state()
        )
    }
}
#[doc = "Life Cycle State Register (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcStateDp(pub u32);
impl ElsOtpLcStateDp {
    #[doc = "OTP life cycle state"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state"]
    #[inline(always)]
    pub const fn set_otp_lc_state_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcStateDp {
    #[inline(always)]
    fn default() -> ElsOtpLcStateDp {
        ElsOtpLcStateDp(0)
    }
}
impl core::fmt::Debug for ElsOtpLcStateDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcStateDp")
            .field("otp_lc_state_dp", &self.otp_lc_state_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcStateDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcStateDp {{ otp_lc_state_dp: {=u8:?} }}",
            self.otp_lc_state_dp()
        )
    }
}
#[doc = "ELS Temporal State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsTemporalState(pub u32);
impl ElsTemporalState {
    #[doc = "Temporal state"]
    #[must_use]
    #[inline(always)]
    pub const fn temporal_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Temporal state"]
    #[inline(always)]
    pub const fn set_temporal_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for ElsTemporalState {
    #[inline(always)]
    fn default() -> ElsTemporalState {
        ElsTemporalState(0)
    }
}
impl core::fmt::Debug for ElsTemporalState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsTemporalState")
            .field("temporal_state", &self.temporal_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsTemporalState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsTemporalState {{ temporal_state: {=u8:?} }}",
            self.temporal_state()
        )
    }
}
#[doc = "EMVSIM0 Function Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim0clkdiv(pub u32);
impl Emvsim0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Emvsim0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Emvsim0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Emvsim0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Emvsim0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Emvsim0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Emvsim0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Emvsim0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Emvsim0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Emvsim0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Emvsim0clkdiv {
    #[inline(always)]
    fn default() -> Emvsim0clkdiv {
        Emvsim0clkdiv(0)
    }
}
impl core::fmt::Debug for Emvsim0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emvsim0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "EMVSIM0 Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim0clksel(pub u32);
impl Emvsim0clksel {
    #[doc = "Selects the EMVSIM0 function clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Emvsim0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Emvsim0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the EMVSIM0 function clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Emvsim0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Emvsim0clksel {
    #[inline(always)]
    fn default() -> Emvsim0clksel {
        Emvsim0clksel(0)
    }
}
impl core::fmt::Debug for Emvsim0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Emvsim0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "EMVSIM1 Function Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim1clkdiv(pub u32);
impl Emvsim1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Emvsim1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Emvsim1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Emvsim1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Emvsim1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Emvsim1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Emvsim1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Emvsim1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Emvsim1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Emvsim1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Emvsim1clkdiv {
    #[inline(always)]
    fn default() -> Emvsim1clkdiv {
        Emvsim1clkdiv(0)
    }
}
impl core::fmt::Debug for Emvsim1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emvsim1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "EMVSIM1 Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim1clksel(pub u32);
impl Emvsim1clksel {
    #[doc = "Selects the EMVSIM1 function clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Emvsim1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Emvsim1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the EMVSIM1 function clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Emvsim1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Emvsim1clksel {
    #[inline(always)]
    fn default() -> Emvsim1clksel {
        Emvsim1clksel(0)
    }
}
impl core::fmt::Debug for Emvsim1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Emvsim1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Ethernet PHY Interface Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetPhyIntfSel(pub u32);
impl EnetPhyIntfSel {
    #[doc = "Selects the PHY interface"]
    #[must_use]
    #[inline(always)]
    pub const fn phy_sel(&self) -> super::vals::PhySel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PhySel::from_bits(val as u8)
    }
    #[doc = "Selects the PHY interface"]
    #[inline(always)]
    pub const fn set_phy_sel(&mut self, val: super::vals::PhySel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for EnetPhyIntfSel {
    #[inline(always)]
    fn default() -> EnetPhyIntfSel {
        EnetPhyIntfSel(0)
    }
}
impl core::fmt::Debug for EnetPhyIntfSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetPhyIntfSel")
            .field("phy_sel", &self.phy_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetPhyIntfSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetPhyIntfSel {{ phy_sel: {:?} }}", self.phy_sel())
    }
}
#[doc = "Sideband Flow Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetSbdFlowCtrl(pub u32);
impl EnetSbdFlowCtrl {
    #[doc = "Sideband Flow Control for channel0"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sideband Flow Control for channel0"]
    #[inline(always)]
    pub const fn set_sel_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sideband Flow Control for channel1"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sideband Flow Control for channel1"]
    #[inline(always)]
    pub const fn set_sel_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for EnetSbdFlowCtrl {
    #[inline(always)]
    fn default() -> EnetSbdFlowCtrl {
        EnetSbdFlowCtrl(0)
    }
}
impl core::fmt::Debug for EnetSbdFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetSbdFlowCtrl")
            .field("sel_ch0", &self.sel_ch0())
            .field("sel_ch1", &self.sel_ch1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetSbdFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetSbdFlowCtrl {{ sel_ch0: {=bool:?}, sel_ch1: {=bool:?} }}",
            self.sel_ch0(),
            self.sel_ch1()
        )
    }
}
#[doc = "Ethernet PTP REF Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetptprefclkdiv(pub u32);
impl Enetptprefclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::EnetptprefclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::EnetptprefclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::EnetptprefclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::EnetptprefclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::EnetptprefclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::EnetptprefclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::EnetptprefclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::EnetptprefclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::EnetptprefclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Enetptprefclkdiv {
    #[inline(always)]
    fn default() -> Enetptprefclkdiv {
        Enetptprefclkdiv(0)
    }
}
impl core::fmt::Debug for Enetptprefclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetptprefclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetptprefclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enetptprefclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Ethernet PTP REF Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetptprefclksel(pub u32);
impl Enetptprefclksel {
    #[doc = "Selects the Ethernet PTP REF clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::EnetptprefclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::EnetptprefclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the Ethernet PTP REF clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::EnetptprefclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Enetptprefclksel {
    #[inline(always)]
    fn default() -> Enetptprefclksel {
        Enetptprefclksel(0)
    }
}
impl core::fmt::Debug for Enetptprefclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetptprefclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetptprefclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enetptprefclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Ethernet RMII Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetrmiiclkdiv(pub u32);
impl Enetrmiiclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::EnetrmiiclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::EnetrmiiclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::EnetrmiiclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::EnetrmiiclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::EnetrmiiclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::EnetrmiiclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::EnetrmiiclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::EnetrmiiclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::EnetrmiiclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Enetrmiiclkdiv {
    #[inline(always)]
    fn default() -> Enetrmiiclkdiv {
        Enetrmiiclkdiv(0)
    }
}
impl core::fmt::Debug for Enetrmiiclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetrmiiclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetrmiiclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enetrmiiclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Ethernet RMII Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetrmiiclksel(pub u32);
impl Enetrmiiclksel {
    #[doc = "Selects the Ethernet RMII clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::EnetrmiiclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::EnetrmiiclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the Ethernet RMII clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::EnetrmiiclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Enetrmiiclksel {
    #[inline(always)]
    fn default() -> Enetrmiiclksel {
        Enetrmiiclksel(0)
    }
}
impl core::fmt::Debug for Enetrmiiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetrmiiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetrmiiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enetrmiiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "ETB Counter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbCounterCtrl(pub u32);
impl EtbCounterCtrl {
    #[doc = "Enables the ETB counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cnten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the ETB counter"]
    #[inline(always)]
    pub const fn set_cnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rspt(&self) -> super::vals::Rspt {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Rspt::from_bits(val as u8)
    }
    #[doc = "Response Type"]
    #[inline(always)]
    pub const fn set_rspt(&mut self, val: super::vals::Rspt) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Reload request"]
    #[must_use]
    #[inline(always)]
    pub const fn rlrq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reload request"]
    #[inline(always)]
    pub const fn set_rlrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for EtbCounterCtrl {
    #[inline(always)]
    fn default() -> EtbCounterCtrl {
        EtbCounterCtrl(0)
    }
}
impl core::fmt::Debug for EtbCounterCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbCounterCtrl")
            .field("cnten", &self.cnten())
            .field("rspt", &self.rspt())
            .field("rlrq", &self.rlrq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbCounterCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EtbCounterCtrl {{ cnten: {=bool:?}, rspt: {:?}, rlrq: {=bool:?} }}",
            self.cnten(),
            self.rspt(),
            self.rlrq()
        )
    }
}
#[doc = "ETB Counter Reload Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbCounterReload(pub u32);
impl EtbCounterReload {
    #[doc = "Byte count reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn reload(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Byte count reload value"]
    #[inline(always)]
    pub const fn set_reload(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for EtbCounterReload {
    #[inline(always)]
    fn default() -> EtbCounterReload {
        EtbCounterReload(0)
    }
}
impl core::fmt::Debug for EtbCounterReload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbCounterReload")
            .field("reload", &self.reload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbCounterReload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EtbCounterReload {{ reload: {=u16:?} }}", self.reload())
    }
}
#[doc = "ETB Counter Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbCounterValue(pub u32);
impl EtbCounterValue {
    #[doc = "Byte count counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn counter_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Byte count counter value"]
    #[inline(always)]
    pub const fn set_counter_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for EtbCounterValue {
    #[inline(always)]
    fn default() -> EtbCounterValue {
        EtbCounterValue(0)
    }
}
impl core::fmt::Debug for EtbCounterValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbCounterValue")
            .field("counter_value", &self.counter_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbCounterValue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EtbCounterValue {{ counter_value: {=u16:?} }}",
            self.counter_value()
        )
    }
}
#[doc = "ETB Counter Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbStatus(pub u32);
impl EtbStatus {
    #[doc = "ETB Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETB Interrupt"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ETB NMI"]
    #[must_use]
    #[inline(always)]
    pub const fn nmi(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ETB NMI"]
    #[inline(always)]
    pub const fn set_nmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug halt request"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_halt_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug halt request"]
    #[inline(always)]
    pub const fn set_dbg_halt_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for EtbStatus {
    #[inline(always)]
    fn default() -> EtbStatus {
        EtbStatus(0)
    }
}
impl core::fmt::Debug for EtbStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbStatus")
            .field("irq", &self.irq())
            .field("nmi", &self.nmi())
            .field("dbg_halt_req", &self.dbg_halt_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EtbStatus {{ irq: {=bool:?}, nmi: {=bool:?}, dbg_halt_req: {=bool:?} }}",
            self.irq(),
            self.nmi(),
            self.dbg_halt_req()
        )
    }
}
#[doc = "EWM0 Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ewm0clksel(pub u32);
impl Ewm0clksel {
    #[doc = "Selects the EWM0 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ewm0clkselSel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ewm0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the EWM0 clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ewm0clkselSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ewm0clksel {
    #[inline(always)]
    fn default() -> Ewm0clksel {
        Ewm0clksel(0)
    }
}
impl core::fmt::Debug for Ewm0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ewm0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ewm0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ewm0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcclksel(pub u32);
impl Fcclksel {
    #[doc = "Selects the LP_FLEXCOMM clock source for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FcclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FcclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the LP_FLEXCOMM clock source for Fractional Rate Divider."]
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
#[doc = "FlexSPI Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexSpiclkdiv(pub u32);
impl FlexSpiclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FlexSpiclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FlexSpiclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FlexSpiclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FlexSpiclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FlexSpiclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FlexSpiclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FlexSpiclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FlexSpiclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FlexSpiclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FlexSpiclkdiv {
    #[inline(always)]
    fn default() -> FlexSpiclkdiv {
        FlexSpiclkdiv(0)
    }
}
impl core::fmt::Debug for FlexSpiclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexSpiclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexSpiclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexSpiclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FlexSPI Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexSpiclksel(pub u32);
impl FlexSpiclksel {
    #[doc = "Selects the FlexSPI clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FlexSpiclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::FlexSpiclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FlexSPI clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::FlexSpiclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for FlexSpiclksel {
    #[inline(always)]
    fn default() -> FlexSpiclksel {
        FlexSpiclksel(0)
    }
}
impl core::fmt::Debug for FlexSpiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexSpiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexSpiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexSpiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FLEXCAN0 Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan0clkdiv(pub u32);
impl Flexcan0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Flexcan0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Flexcan0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Flexcan0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Flexcan0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Flexcan0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Flexcan0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Flexcan0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Flexcan0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Flexcan0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexcan0clkdiv {
    #[inline(always)]
    fn default() -> Flexcan0clkdiv {
        Flexcan0clkdiv(0)
    }
}
impl core::fmt::Debug for Flexcan0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexcan0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN0 Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan0clksel(pub u32);
impl Flexcan0clksel {
    #[doc = "Selects the FLEXCAN0 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Flexcan0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Flexcan0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FLEXCAN0 clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Flexcan0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexcan0clksel {
    #[inline(always)]
    fn default() -> Flexcan0clksel {
        Flexcan0clksel(0)
    }
}
impl core::fmt::Debug for Flexcan0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcan0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FLEXCAN1 Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan1clkdiv(pub u32);
impl Flexcan1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Flexcan1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Flexcan1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Flexcan1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Flexcan1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Flexcan1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Flexcan1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Flexcan1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Flexcan1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Flexcan1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexcan1clkdiv {
    #[inline(always)]
    fn default() -> Flexcan1clkdiv {
        Flexcan1clkdiv(0)
    }
}
impl core::fmt::Debug for Flexcan1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexcan1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN1 Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan1clksel(pub u32);
impl Flexcan1clksel {
    #[doc = "Selects the FLEXCAN1 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Flexcan1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Flexcan1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FLEXCAN1 clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Flexcan1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexcan1clksel {
    #[inline(always)]
    fn default() -> Flexcan1clksel {
        Flexcan1clksel(0)
    }
}
impl core::fmt::Debug for Flexcan1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcan1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "LP_FLEXCOMM Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcommclkdiv(pub u32);
impl Flexcommclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FlexcommclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FlexcommclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FlexcommclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FlexcommclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FlexcommclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FlexcommclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FlexcommclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FlexcommclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FlexcommclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexcommclkdiv {
    #[inline(always)]
    fn default() -> Flexcommclkdiv {
        Flexcommclkdiv(0)
    }
}
impl core::fmt::Debug for Flexcommclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcommclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcommclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexcommclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexioclkdiv(pub u32);
impl Flexioclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FlexioclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FlexioclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FlexioclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FlexioclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FlexioclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FlexioclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FlexioclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FlexioclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FlexioclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexioclkdiv {
    #[inline(always)]
    fn default() -> Flexioclkdiv {
        Flexioclkdiv(0)
    }
}
impl core::fmt::Debug for Flexioclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexioclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexioclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexioclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexioclksel(pub u32);
impl Flexioclksel {
    #[doc = "Selects the FLEXIO clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FlexioclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FlexioclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FLEXIO clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::FlexioclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexioclksel {
    #[inline(always)]
    fn default() -> Flexioclksel {
        Flexioclksel(0)
    }
}
impl core::fmt::Debug for Flexioclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexioclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexioclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexioclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FRO_HF_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FrohfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FrohfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FrohfdivUnstab) {
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
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "GDET Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetCtrl(pub u32);
impl GdetCtrl {
    #[doc = "Controls the GDET clean event counter"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_evtcnt_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the GDET clean event counter"]
    #[inline(always)]
    pub const fn set_gdet_evtcnt_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears GDET error status"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_err_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears GDET error status"]
    #[inline(always)]
    pub const fn set_gdet_err_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GDET isolation control"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_iso_sw(&self) -> super::vals::GdetIsoSw {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::GdetIsoSw::from_bits(val as u8)
    }
    #[doc = "GDET isolation control"]
    #[inline(always)]
    pub const fn set_gdet_iso_sw(&mut self, val: super::vals::GdetIsoSw) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Event count value"]
    #[must_use]
    #[inline(always)]
    pub const fn event_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Event count value"]
    #[inline(always)]
    pub const fn set_event_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Positive glitch detected"]
    #[must_use]
    #[inline(always)]
    pub const fn pos_sync(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Positive glitch detected"]
    #[inline(always)]
    pub const fn set_pos_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Negative glitch detected"]
    #[must_use]
    #[inline(always)]
    pub const fn neg_sync(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Negative glitch detected"]
    #[inline(always)]
    pub const fn set_neg_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event counter cleared"]
    #[must_use]
    #[inline(always)]
    pub const fn event_clr_flag(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event counter cleared"]
    #[inline(always)]
    pub const fn set_event_clr_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for GdetCtrl {
    #[inline(always)]
    fn default() -> GdetCtrl {
        GdetCtrl(0)
    }
}
impl core::fmt::Debug for GdetCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetCtrl")
            .field("gdet_evtcnt_clr", &self.gdet_evtcnt_clr())
            .field("gdet_err_clr", &self.gdet_err_clr())
            .field("gdet_iso_sw", &self.gdet_iso_sw())
            .field("event_cnt", &self.event_cnt())
            .field("pos_sync", &self.pos_sync())
            .field("neg_sync", &self.neg_sync())
            .field("event_clr_flag", &self.event_clr_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetCtrl {{ gdet_evtcnt_clr: {=bool:?}, gdet_err_clr: {=bool:?}, gdet_iso_sw: {:?}, event_cnt: {=u8:?}, pos_sync: {=bool:?}, neg_sync: {=bool:?}, event_clr_flag: {=bool:?} }}",
            self.gdet_evtcnt_clr(),
            self.gdet_err_clr(),
            self.gdet_iso_sw(),
            self.event_cnt(),
            self.pos_sync(),
            self.neg_sync(),
            self.event_clr_flag()
        )
    }
}
#[doc = "Gray to Binary Converter Gray code_gray\\[31:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeLsb(pub u32);
impl GrayCodeLsb {
    #[doc = "Gray code \\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Gray code \\[31:0\\]"]
    #[inline(always)]
    pub const fn set_code_gray_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GrayCodeLsb {
    #[inline(always)]
    fn default() -> GrayCodeLsb {
        GrayCodeLsb(0)
    }
}
impl core::fmt::Debug for GrayCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeLsb")
            .field("code_gray_31_0", &self.code_gray_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeLsb {{ code_gray_31_0: {=u32:?} }}",
            self.code_gray_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Gray code_gray\\[41:32\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeMsb(pub u32);
impl GrayCodeMsb {
    #[doc = "Gray code \\[41:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Gray code \\[41:32\\]"]
    #[inline(always)]
    pub const fn set_code_gray_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for GrayCodeMsb {
    #[inline(always)]
    fn default() -> GrayCodeMsb {
        GrayCodeMsb(0)
    }
}
impl core::fmt::Debug for GrayCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeMsb")
            .field("code_gray_41_32", &self.code_gray_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeMsb {{ code_gray_41_32: {=u16:?} }}",
            self.code_gray_41_32()
        )
    }
}
#[doc = "I3C0 Functional Clock FCLK Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkdiv(pub u32);
impl I3c0fclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::I3c0fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::I3c0fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::I3c0fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::I3c0fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I3c0fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::I3c0fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::I3c0fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I3c0fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::I3c0fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclkdiv {
    #[inline(always)]
    fn default() -> I3c0fclkdiv {
        I3c0fclkdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0 FCLK Slow Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclksdiv(pub u32);
impl I3c0fclksdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::I3c0fclksdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::I3c0fclksdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::I3c0fclksdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::I3c0fclksdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I3c0fclksdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::I3c0fclksdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::I3c0fclksdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I3c0fclksdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::I3c0fclksdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclksdiv {
    #[inline(always)]
    fn default() -> I3c0fclksdiv {
        I3c0fclksdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclksdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclksdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclksdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclksdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0 Functional Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclksel(pub u32);
impl I3c0fclksel {
    #[doc = "Selects the I3C0 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c0fclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the I3C0 clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclksel {
    #[inline(always)]
    fn default() -> I3c0fclksel {
        I3c0fclksel(0)
    }
}
impl core::fmt::Debug for I3c0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C0 FCLK Slow Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkssel(pub u32);
impl I3c0fclkssel {
    #[doc = "Selects the I3C FCLK Slow clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c0fclksselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c0fclksselSel::from_bits(val as u8)
    }
    #[doc = "Selects the I3C FCLK Slow clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c0fclksselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclkssel {
    #[inline(always)]
    fn default() -> I3c0fclkssel {
        I3c0fclkssel(0)
    }
}
impl core::fmt::Debug for I3c0fclkssel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkssel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkssel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclkssel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C0 FCLK_STC Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkstcdiv(pub u32);
impl I3c0fclkstcdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::I3c0fclkstcdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::I3c0fclkstcdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::I3c0fclkstcdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::I3c0fclkstcdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I3c0fclkstcdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::I3c0fclkstcdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::I3c0fclkstcdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I3c0fclkstcdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::I3c0fclkstcdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclkstcdiv {
    #[inline(always)]
    fn default() -> I3c0fclkstcdiv {
        I3c0fclkstcdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclkstcdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkstcdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkstcdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclkstcdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0 FCLK_STC Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkstcsel(pub u32);
impl I3c0fclkstcsel {
    #[doc = "Selects the I3C0 Time Control clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c0fclkstcselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c0fclkstcselSel::from_bits(val as u8)
    }
    #[doc = "Selects the I3C0 Time Control clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c0fclkstcselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclkstcsel {
    #[inline(always)]
    fn default() -> I3c0fclkstcsel {
        I3c0fclkstcsel(0)
    }
}
impl core::fmt::Debug for I3c0fclkstcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkstcsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkstcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclkstcsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C1 Functional Clock FCLK Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkdiv(pub u32);
impl I3c1fclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::I3c1fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::I3c1fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::I3c1fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::I3c1fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I3c1fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::I3c1fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::I3c1fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I3c1fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::I3c1fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c1fclkdiv {
    #[inline(always)]
    fn default() -> I3c1fclkdiv {
        I3c1fclkdiv(0)
    }
}
impl core::fmt::Debug for I3c1fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c1fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C1 FCLK Slow clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclksdiv(pub u32);
impl I3c1fclksdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::I3c1fclksdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::I3c1fclksdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::I3c1fclksdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::I3c1fclksdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I3c1fclksdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::I3c1fclksdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::I3c1fclksdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I3c1fclksdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::I3c1fclksdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c1fclksdiv {
    #[inline(always)]
    fn default() -> I3c1fclksdiv {
        I3c1fclksdiv(0)
    }
}
impl core::fmt::Debug for I3c1fclksdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclksdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclksdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c1fclksdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C1 Functional Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclksel(pub u32);
impl I3c1fclksel {
    #[doc = "I3C1 clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c1fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c1fclkselSel::from_bits(val as u8)
    }
    #[doc = "I3C1 clock select"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c1fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c1fclksel {
    #[inline(always)]
    fn default() -> I3c1fclksel {
        I3c1fclksel(0)
    }
}
impl core::fmt::Debug for I3c1fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c1fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C1 FCLK Slow Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkssel(pub u32);
impl I3c1fclkssel {
    #[doc = "I3C1 FCLK Slow Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c1fclksselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c1fclksselSel::from_bits(val as u8)
    }
    #[doc = "I3C1 FCLK Slow Clock Select"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c1fclksselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c1fclkssel {
    #[inline(always)]
    fn default() -> I3c1fclkssel {
        I3c1fclkssel(0)
    }
}
impl core::fmt::Debug for I3c1fclkssel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkssel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkssel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c1fclkssel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C1 FCLK_STC Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkstcdiv(pub u32);
impl I3c1fclkstcdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::I3c1fclkstcdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::I3c1fclkstcdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::I3c1fclkstcdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::I3c1fclkstcdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::I3c1fclkstcdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::I3c1fclkstcdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::I3c1fclkstcdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::I3c1fclkstcdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::I3c1fclkstcdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c1fclkstcdiv {
    #[inline(always)]
    fn default() -> I3c1fclkstcdiv {
        I3c1fclkstcdiv(0)
    }
}
impl core::fmt::Debug for I3c1fclkstcdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkstcdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkstcdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c1fclkstcdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Selects the I3C1 Time Control clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkstcsel(pub u32);
impl I3c1fclkstcsel {
    #[doc = "I3C1 FCLK_STC clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c1fclkstcselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c1fclkstcselSel::from_bits(val as u8)
    }
    #[doc = "I3C1 FCLK_STC clock select"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c1fclkstcselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c1fclkstcsel {
    #[inline(always)]
    fn default() -> I3c1fclkstcsel {
        I3c1fclkstcsel(0)
    }
}
impl core::fmt::Debug for I3c1fclkstcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkstcsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkstcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c1fclkstcsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "JTAG Chip ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "Indicates the device ID"]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the device ID"]
    #[inline(always)]
    pub const fn set_jtag_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("jtag_id", &self.jtag_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "JtagId {{ jtag_id: {=u32:?} }}", self.jtag_id())
    }
}
#[doc = "Key Retain Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyRetainCtrl(pub u32);
impl KeyRetainCtrl {
    #[doc = "Indicates if the PUF key has been retained in the VBAT domain and has not been reset or otherwise invalidated by software."]
    #[must_use]
    #[inline(always)]
    pub const fn key_retain_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if the PUF key has been retained in the VBAT domain and has not been reset or otherwise invalidated by software."]
    #[inline(always)]
    pub const fn set_key_retain_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the successful completion of the key_save or key_load routine. Once set, to clear the key_retain_done flag, both key_save and key_load should be cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn key_retain_done(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the successful completion of the key_save or key_load routine. Once set, to clear the key_retain_done flag, both key_save and key_load should be cleared by software."]
    #[inline(always)]
    pub const fn set_key_retain_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time"]
    #[must_use]
    #[inline(always)]
    pub const fn key_save(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time"]
    #[inline(always)]
    pub const fn set_key_save(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time"]
    #[must_use]
    #[inline(always)]
    pub const fn key_load(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time"]
    #[inline(always)]
    pub const fn set_key_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for KeyRetainCtrl {
    #[inline(always)]
    fn default() -> KeyRetainCtrl {
        KeyRetainCtrl(0)
    }
}
impl core::fmt::Debug for KeyRetainCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeyRetainCtrl")
            .field("key_retain_valid", &self.key_retain_valid())
            .field("key_retain_done", &self.key_retain_done())
            .field("key_save", &self.key_save())
            .field("key_load", &self.key_load())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeyRetainCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KeyRetainCtrl {{ key_retain_valid: {=bool:?}, key_retain_done: {=bool:?}, key_save: {=bool:?}, key_load: {=bool:?} }}",
            self.key_retain_valid(),
            self.key_retain_done(),
            self.key_save(),
            self.key_load()
        )
    }
}
#[doc = "LPCAC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpcacCtrl(pub u32);
impl LpcacCtrl {
    #[doc = "Disables/enables the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac(&self) -> super::vals::DisLpcac {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DisLpcac::from_bits(val as u8)
    }
    #[doc = "Disables/enables the cache function."]
    #[inline(always)]
    pub const fn set_dis_lpcac(&mut self, val: super::vals::DisLpcac) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_lpcac(&self) -> super::vals::ClrLpcac {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ClrLpcac::from_bits(val as u8)
    }
    #[doc = "Clears the cache function."]
    #[inline(always)]
    pub const fn set_clr_lpcac(&mut self, val: super::vals::ClrLpcac) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces no allocation."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_no_alloc(&self) -> super::vals::FrcNoAlloc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FrcNoAlloc::from_bits(val as u8)
    }
    #[doc = "Forces no allocation."]
    #[inline(always)]
    pub const fn set_frc_no_alloc(&mut self, val: super::vals::FrcNoAlloc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables parity miss."]
    #[must_use]
    #[inline(always)]
    pub const fn parity_miss_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables parity miss."]
    #[inline(always)]
    pub const fn set_parity_miss_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_dis_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lim_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_lim_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable parity error report."]
    #[must_use]
    #[inline(always)]
    pub const fn parity_fault_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable parity error report."]
    #[inline(always)]
    pub const fn set_parity_fault_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_xom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    #[inline(always)]
    pub const fn set_lpcac_xom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for LpcacCtrl {
    #[inline(always)]
    fn default() -> LpcacCtrl {
        LpcacCtrl(0)
    }
}
impl core::fmt::Debug for LpcacCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpcacCtrl")
            .field("dis_lpcac", &self.dis_lpcac())
            .field("clr_lpcac", &self.clr_lpcac())
            .field("frc_no_alloc", &self.frc_no_alloc())
            .field("parity_miss_en", &self.parity_miss_en())
            .field("dis_lpcac_wtbf", &self.dis_lpcac_wtbf())
            .field("lim_lpcac_wtbf", &self.lim_lpcac_wtbf())
            .field("parity_fault_en", &self.parity_fault_en())
            .field("lpcac_xom", &self.lpcac_xom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpcacCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpcacCtrl {{ dis_lpcac: {:?}, clr_lpcac: {:?}, frc_no_alloc: {:?}, parity_miss_en: {=bool:?}, dis_lpcac_wtbf: {=bool:?}, lim_lpcac_wtbf: {=bool:?}, parity_fault_en: {=bool:?}, lpcac_xom: {=bool:?} }}",
            self.dis_lpcac(),
            self.clr_lpcac(),
            self.frc_no_alloc(),
            self.parity_miss_en(),
            self.dis_lpcac_wtbf(),
            self.lim_lpcac_wtbf(),
            self.parity_fault_en(),
            self.lpcac_xom()
        )
    }
}
#[doc = "MICFIL Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Micfilfclkdiv(pub u32);
impl Micfilfclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::MicfilfclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MicfilfclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::MicfilfclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::MicfilfclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::MicfilfclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::MicfilfclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::MicfilfclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MicfilfclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::MicfilfclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Micfilfclkdiv {
    #[inline(always)]
    fn default() -> Micfilfclkdiv {
        Micfilfclkdiv(0)
    }
}
impl core::fmt::Debug for Micfilfclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Micfilfclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Micfilfclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Micfilfclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "MICFIL Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Micfilfclksel(pub u32);
impl Micfilfclksel {
    #[doc = "Selects the MICFIL clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MicfilfclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::MicfilfclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the MICFIL clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::MicfilfclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Micfilfclksel {
    #[inline(always)]
    fn default() -> Micfilfclksel {
        Micfilfclksel(0)
    }
}
impl core::fmt::Debug for Micfilfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Micfilfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Micfilfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Micfilfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "NMI Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU1, if enabled by NMIENCPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    pub const fn set_irqcpu1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    pub const fn set_nmiencpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
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
#[doc = "NVM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmCtrl(pub u32);
impl NvmCtrl {
    #[doc = "Flash speculation control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_spec(&self) -> super::vals::DisFlashSpec {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DisFlashSpec::from_bits(val as u8)
    }
    #[doc = "Flash speculation control"]
    #[inline(always)]
    pub const fn set_dis_flash_spec(&mut self, val: super::vals::DisFlashSpec) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash data speculation control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_data_spec(&self) -> super::vals::DisDataSpec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DisDataSpec::from_bits(val as u8)
    }
    #[doc = "Flash data speculation control"]
    #[inline(always)]
    pub const fn set_dis_data_spec(&mut self, val: super::vals::DisDataSpec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash cache control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_cache(&self) -> super::vals::DisFlashCache {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DisFlashCache::from_bits(val as u8)
    }
    #[doc = "Flash cache control"]
    #[inline(always)]
    pub const fn set_dis_flash_cache(&mut self, val: super::vals::DisFlashCache) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Flash instruction cache control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_inst(&self) -> super::vals::DisFlashInst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DisFlashInst::from_bits(val as u8)
    }
    #[doc = "Flash instruction cache control"]
    #[inline(always)]
    pub const fn set_dis_flash_inst(&mut self, val: super::vals::DisFlashInst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Flash data cache control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_data(&self) -> super::vals::DisFlashData {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DisFlashData::from_bits(val as u8)
    }
    #[doc = "Flash data cache control"]
    #[inline(always)]
    pub const fn set_dis_flash_data(&mut self, val: super::vals::DisFlashData) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Clear flash cache control"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_flash_cache(&self) -> super::vals::ClrFlashCache {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ClrFlashCache::from_bits(val as u8)
    }
    #[doc = "Clear flash cache control"]
    #[inline(always)]
    pub const fn set_clr_flash_cache(&mut self, val: super::vals::ClrFlashCache) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLASH stall on busy control"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stall_en(&self) -> super::vals::FlashStallEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FlashStallEn::from_bits(val as u8)
    }
    #[doc = "FLASH stall on busy control"]
    #[inline(always)]
    pub const fn set_flash_stall_en(&mut self, val: super::vals::FlashStallEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus error on instruction multi-bit ECC error control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_inst(&self) -> super::vals::DisMbeccErrInst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DisMbeccErrInst::from_bits(val as u8)
    }
    #[doc = "Bus error on instruction multi-bit ECC error control"]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_inst(&mut self, val: super::vals::DisMbeccErrInst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_data(&self) -> super::vals::DisMbeccErrData {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DisMbeccErrData::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control"]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_data(&mut self, val: super::vals::DisMbeccErrData) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for NvmCtrl {
    #[inline(always)]
    fn default() -> NvmCtrl {
        NvmCtrl(0)
    }
}
impl core::fmt::Debug for NvmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NvmCtrl")
            .field("dis_flash_spec", &self.dis_flash_spec())
            .field("dis_data_spec", &self.dis_data_spec())
            .field("dis_flash_cache", &self.dis_flash_cache())
            .field("dis_flash_inst", &self.dis_flash_inst())
            .field("dis_flash_data", &self.dis_flash_data())
            .field("clr_flash_cache", &self.clr_flash_cache())
            .field("flash_stall_en", &self.flash_stall_en())
            .field("dis_mbecc_err_inst", &self.dis_mbecc_err_inst())
            .field("dis_mbecc_err_data", &self.dis_mbecc_err_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NvmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NvmCtrl {{ dis_flash_spec: {:?}, dis_data_spec: {:?}, dis_flash_cache: {:?}, dis_flash_inst: {:?}, dis_flash_data: {:?}, clr_flash_cache: {:?}, flash_stall_en: {:?}, dis_mbecc_err_inst: {:?}, dis_mbecc_err_data: {:?} }}",
            self.dis_flash_spec(),
            self.dis_data_spec(),
            self.dis_flash_cache(),
            self.dis_flash_inst(),
            self.dis_flash_data(),
            self.clr_flash_cache(),
            self.flash_stall_en(),
            self.dis_mbecc_err_inst(),
            self.dis_mbecc_err_data()
        )
    }
}
#[doc = "OSTIMER Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostimerclksel(pub u32);
impl Ostimerclksel {
    #[doc = "Selects the OS Event Timer clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::OstimerclkselSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::OstimerclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the OS Event Timer clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::OstimerclkselSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Ostimerclksel {
    #[inline(always)]
    fn default() -> Ostimerclksel {
        Ostimerclksel(0)
    }
}
impl core::fmt::Debug for Ostimerclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ostimerclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ostimerclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ostimerclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "PLL1 Clock 0 Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clk0div(pub u32);
impl Pll1clk0div {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Pll1clk0divReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pll1clk0divReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Pll1clk0divReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Pll1clk0divHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pll1clk0divHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Pll1clk0divHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Pll1clk0divUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pll1clk0divUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Pll1clk0divUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clk0div {
    #[inline(always)]
    fn default() -> Pll1clk0div {
        Pll1clk0div(0)
    }
}
impl core::fmt::Debug for Pll1clk0div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clk0div")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clk0div {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clk0div {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "PLL1 Clock 1 Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clk1div(pub u32);
impl Pll1clk1div {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Pll1clk1divReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pll1clk1divReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Pll1clk1divReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Pll1clk1divHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pll1clk1divHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Pll1clk1divHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Pll1clk1divUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pll1clk1divUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Pll1clk1divUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clk1div {
    #[inline(always)]
    fn default() -> Pll1clk1div {
        Pll1clk1div(0)
    }
}
impl core::fmt::Debug for Pll1clk1div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clk1div")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clk1div {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clk1div {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "PLL Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllclkdiv(pub u32);
impl Pllclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::PllclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::PllclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::PllclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::PllclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::PllclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::PllclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::PllclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::PllclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pllclkdiv {
    #[inline(always)]
    fn default() -> Pllclkdiv {
        Pllclkdiv(0)
    }
}
impl core::fmt::Debug for Pllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pllclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "PLL Clock Divider Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllclkdivsel(pub u32);
impl Pllclkdivsel {
    #[doc = "Selects the PLL Clock Divider source clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::PllclkdivselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::PllclkdivselSel::from_bits(val as u8)
    }
    #[doc = "Selects the PLL Clock Divider source clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::PllclkdivselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Pllclkdivsel {
    #[inline(always)]
    fn default() -> Pllclkdivsel {
        Pllclkdivsel(0)
    }
}
impl core::fmt::Debug for Pllclkdivsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pllclkdivsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pllclkdivsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pllclkdivsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Peripheral Reset Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl0(pub u32);
impl Presetctrl0 {
    #[doc = "Flash management unit reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_rst(&self) -> super::vals::FmuRst {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::FmuRst::from_bits(val as u8)
    }
    #[doc = "Flash management unit reset control"]
    #[inline(always)]
    pub const fn set_fmu_rst(&mut self, val: super::vals::FmuRst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FlexSPI reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_rst(&self) -> super::vals::FlexspiRst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FlexspiRst::from_bits(val as u8)
    }
    #[doc = "FlexSPI reset control"]
    #[inline(always)]
    pub const fn set_flexspi_rst(&mut self, val: super::vals::FlexspiRst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "INPUTMUX reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_rst(&self) -> super::vals::MuxRst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::MuxRst::from_bits(val as u8)
    }
    #[doc = "INPUTMUX reset control"]
    #[inline(always)]
    pub const fn set_mux_rst(&mut self, val: super::vals::MuxRst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT0 controller reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn port0_rst(&self) -> super::vals::Port0Rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Port0Rst::from_bits(val as u8)
    }
    #[doc = "PORT0 controller reset control"]
    #[inline(always)]
    pub const fn set_port0_rst(&mut self, val: super::vals::Port0Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "PORT1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn port1_rst(&self) -> super::vals::Port1Rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Port1Rst::from_bits(val as u8)
    }
    #[doc = "PORT1 reset control"]
    #[inline(always)]
    pub const fn set_port1_rst(&mut self, val: super::vals::Port1Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "PORT2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn port2_rst(&self) -> super::vals::Port2Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Port2Rst::from_bits(val as u8)
    }
    #[doc = "PORT2 reset control"]
    #[inline(always)]
    pub const fn set_port2_rst(&mut self, val: super::vals::Port2Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "PORT3 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn port3_rst(&self) -> super::vals::Port3Rst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Port3Rst::from_bits(val as u8)
    }
    #[doc = "PORT3 reset control"]
    #[inline(always)]
    pub const fn set_port3_rst(&mut self, val: super::vals::Port3Rst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "PORT4 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn port4_rst(&self) -> super::vals::Port4Rst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Port4Rst::from_bits(val as u8)
    }
    #[doc = "PORT4 reset control"]
    #[inline(always)]
    pub const fn set_port4_rst(&mut self, val: super::vals::Port4Rst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "GPIO0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_rst(&self) -> super::vals::Gpio0Rst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpio0Rst::from_bits(val as u8)
    }
    #[doc = "GPIO0 reset control"]
    #[inline(always)]
    pub const fn set_gpio0_rst(&mut self, val: super::vals::Gpio0Rst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "GPIO1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_rst(&self) -> super::vals::Gpio1Rst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpio1Rst::from_bits(val as u8)
    }
    #[doc = "GPIO1 reset control"]
    #[inline(always)]
    pub const fn set_gpio1_rst(&mut self, val: super::vals::Gpio1Rst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_rst(&self) -> super::vals::Gpio2Rst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gpio2Rst::from_bits(val as u8)
    }
    #[doc = "GPIO2 reset control"]
    #[inline(always)]
    pub const fn set_gpio2_rst(&mut self, val: super::vals::Gpio2Rst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "GPIO3 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_rst(&self) -> super::vals::Gpio3Rst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gpio3Rst::from_bits(val as u8)
    }
    #[doc = "GPIO3 reset control"]
    #[inline(always)]
    pub const fn set_gpio3_rst(&mut self, val: super::vals::Gpio3Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "GPIO4 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_rst(&self) -> super::vals::Gpio4Rst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpio4Rst::from_bits(val as u8)
    }
    #[doc = "GPIO4 reset control"]
    #[inline(always)]
    pub const fn set_gpio4_rst(&mut self, val: super::vals::Gpio4Rst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "PINT reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn pint_rst(&self) -> super::vals::PintRst {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::PintRst::from_bits(val as u8)
    }
    #[doc = "PINT reset control"]
    #[inline(always)]
    pub const fn set_pint_rst(&mut self, val: super::vals::PintRst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_rst(&self) -> super::vals::Dma0Rst {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dma0Rst::from_bits(val as u8)
    }
    #[doc = "DMA0 reset control"]
    #[inline(always)]
    pub const fn set_dma0_rst(&mut self, val: super::vals::Dma0Rst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "CRC reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_rst(&self) -> super::vals::CrcRst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::CrcRst::from_bits(val as u8)
    }
    #[doc = "CRC reset control"]
    #[inline(always)]
    pub const fn set_crc_rst(&mut self, val: super::vals::CrcRst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Inter-CPU communication Mailbox reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox_rst(&self) -> super::vals::MailboxRst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MailboxRst::from_bits(val as u8)
    }
    #[doc = "Inter-CPU communication Mailbox reset control"]
    #[inline(always)]
    pub const fn set_mailbox_rst(&mut self, val: super::vals::MailboxRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("fmu_rst", &self.fmu_rst())
            .field("flexspi_rst", &self.flexspi_rst())
            .field("mux_rst", &self.mux_rst())
            .field("port0_rst", &self.port0_rst())
            .field("port1_rst", &self.port1_rst())
            .field("port2_rst", &self.port2_rst())
            .field("port3_rst", &self.port3_rst())
            .field("port4_rst", &self.port4_rst())
            .field("gpio0_rst", &self.gpio0_rst())
            .field("gpio1_rst", &self.gpio1_rst())
            .field("gpio2_rst", &self.gpio2_rst())
            .field("gpio3_rst", &self.gpio3_rst())
            .field("gpio4_rst", &self.gpio4_rst())
            .field("pint_rst", &self.pint_rst())
            .field("dma0_rst", &self.dma0_rst())
            .field("crc_rst", &self.crc_rst())
            .field("mailbox_rst", &self.mailbox_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl0 {{ fmu_rst: {:?}, flexspi_rst: {:?}, mux_rst: {:?}, port0_rst: {:?}, port1_rst: {:?}, port2_rst: {:?}, port3_rst: {:?}, port4_rst: {:?}, gpio0_rst: {:?}, gpio1_rst: {:?}, gpio2_rst: {:?}, gpio3_rst: {:?}, gpio4_rst: {:?}, pint_rst: {:?}, dma0_rst: {:?}, crc_rst: {:?}, mailbox_rst: {:?} }}",
            self.fmu_rst(),
            self.flexspi_rst(),
            self.mux_rst(),
            self.port0_rst(),
            self.port1_rst(),
            self.port2_rst(),
            self.port3_rst(),
            self.port4_rst(),
            self.gpio0_rst(),
            self.gpio1_rst(),
            self.gpio2_rst(),
            self.gpio3_rst(),
            self.gpio4_rst(),
            self.pint_rst(),
            self.dma0_rst(),
            self.crc_rst(),
            self.mailbox_rst()
        )
    }
}
#[doc = "Peripheral Reset Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl1(pub u32);
impl Presetctrl1 {
    #[doc = "MRT reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt_rst(&self) -> super::vals::MrtRst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MrtRst::from_bits(val as u8)
    }
    #[doc = "MRT reset control"]
    #[inline(always)]
    pub const fn set_mrt_rst(&mut self, val: super::vals::MrtRst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "OS Event Timer reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer_rst(&self) -> super::vals::OstimerRst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::OstimerRst::from_bits(val as u8)
    }
    #[doc = "OS Event Timer reset control"]
    #[inline(always)]
    pub const fn set_ostimer_rst(&mut self, val: super::vals::OstimerRst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SCT reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sct_rst(&self) -> super::vals::SctRst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SctRst::from_bits(val as u8)
    }
    #[doc = "SCT reset control"]
    #[inline(always)]
    pub const fn set_sct_rst(&mut self, val: super::vals::SctRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0_rst(&self) -> super::vals::Adc0Rst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Adc0Rst::from_bits(val as u8)
    }
    #[doc = "ADC0 reset control"]
    #[inline(always)]
    pub const fn set_adc0_rst(&mut self, val: super::vals::Adc0Rst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1_rst(&self) -> super::vals::Adc1Rst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Adc1Rst::from_bits(val as u8)
    }
    #[doc = "ADC1 reset control"]
    #[inline(always)]
    pub const fn set_adc1_rst(&mut self, val: super::vals::Adc1Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0_rst(&self) -> super::vals::Dac0Rst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dac0Rst::from_bits(val as u8)
    }
    #[doc = "DAC0 reset control"]
    #[inline(always)]
    pub const fn set_dac0_rst(&mut self, val: super::vals::Dac0Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RTC reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_rst(&self) -> super::vals::RtcRst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RtcRst::from_bits(val as u8)
    }
    #[doc = "RTC reset control"]
    #[inline(always)]
    pub const fn set_rtc_rst(&mut self, val: super::vals::RtcRst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "EVSIM0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn evsim0_rst(&self) -> super::vals::Evsim0Rst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Evsim0Rst::from_bits(val as u8)
    }
    #[doc = "EVSIM0 reset control"]
    #[inline(always)]
    pub const fn set_evsim0_rst(&mut self, val: super::vals::Evsim0Rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "EVSIM1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn evsim1_rst(&self) -> super::vals::Evsim1Rst {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Evsim1Rst::from_bits(val as u8)
    }
    #[doc = "EVSIM1 reset control"]
    #[inline(always)]
    pub const fn set_evsim1_rst(&mut self, val: super::vals::Evsim1Rst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "UTICK reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn utick_rst(&self) -> super::vals::UtickRst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::UtickRst::from_bits(val as u8)
    }
    #[doc = "UTICK reset control"]
    #[inline(always)]
    pub const fn set_utick_rst(&mut self, val: super::vals::UtickRst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "LP_FLEXCOMM0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc0_rst(&self) -> super::vals::Fc0Rst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fc0Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM0 reset control"]
    #[inline(always)]
    pub const fn set_fc0_rst(&mut self, val: super::vals::Fc0Rst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "LP_FLEXCOMM1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc1_rst(&self) -> super::vals::Fc1Rst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Fc1Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM1 reset control"]
    #[inline(always)]
    pub const fn set_fc1_rst(&mut self, val: super::vals::Fc1Rst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "LP_FLEXCOMM2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc2_rst(&self) -> super::vals::Fc2Rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Fc2Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM2 reset control"]
    #[inline(always)]
    pub const fn set_fc2_rst(&mut self, val: super::vals::Fc2Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "LP_FLEXCOMM3 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc3_rst(&self) -> super::vals::Fc3Rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Fc3Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM3 reset control"]
    #[inline(always)]
    pub const fn set_fc3_rst(&mut self, val: super::vals::Fc3Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "LP_FLEXCOMM4 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc4_rst(&self) -> super::vals::Fc4Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Fc4Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM4 reset control"]
    #[inline(always)]
    pub const fn set_fc4_rst(&mut self, val: super::vals::Fc4Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "LP_FLEXCOMM5 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc5_rst(&self) -> super::vals::Fc5Rst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fc5Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM5 reset control"]
    #[inline(always)]
    pub const fn set_fc5_rst(&mut self, val: super::vals::Fc5Rst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "LP_FLEXCOMM6 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc6_rst(&self) -> super::vals::Fc6Rst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Fc6Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM6 reset control"]
    #[inline(always)]
    pub const fn set_fc6_rst(&mut self, val: super::vals::Fc6Rst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "LP_FLEXCOMM7 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc7_rst(&self) -> super::vals::Fc7Rst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Fc7Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM7 reset control"]
    #[inline(always)]
    pub const fn set_fc7_rst(&mut self, val: super::vals::Fc7Rst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "LP_FLEXCOMM8 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc8_rst(&self) -> super::vals::Fc8Rst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Fc8Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM8 reset control"]
    #[inline(always)]
    pub const fn set_fc8_rst(&mut self, val: super::vals::Fc8Rst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "LP_FLEXCOMM9 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc9_rst(&self) -> super::vals::Fc9Rst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Fc9Rst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM9 reset control"]
    #[inline(always)]
    pub const fn set_fc9_rst(&mut self, val: super::vals::Fc9Rst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "MICFIL reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn micfil_rst(&self) -> super::vals::MicfilRst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::MicfilRst::from_bits(val as u8)
    }
    #[doc = "MICFIL reset control"]
    #[inline(always)]
    pub const fn set_micfil_rst(&mut self, val: super::vals::MicfilRst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "CTIMER2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn timer2_rst(&self) -> super::vals::Timer2Rst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Timer2Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER2 reset control"]
    #[inline(always)]
    pub const fn set_timer2_rst(&mut self, val: super::vals::Timer2Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB FS DCD reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs_dcd_rst(&self) -> super::vals::Usb0FsDcdRst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Usb0FsDcdRst::from_bits(val as u8)
    }
    #[doc = "USB FS DCD reset control"]
    #[inline(always)]
    pub const fn set_usb0_fs_dcd_rst(&mut self, val: super::vals::Usb0FsDcdRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "USB FS reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs_rst(&self) -> super::vals::Usb0FsRst {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Usb0FsRst::from_bits(val as u8)
    }
    #[doc = "USB FS reset control"]
    #[inline(always)]
    pub const fn set_usb0_fs_rst(&mut self, val: super::vals::Usb0FsRst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "CTIMER0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_rst(&self) -> super::vals::Timer0Rst {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Timer0Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER0 reset control"]
    #[inline(always)]
    pub const fn set_timer0_rst(&mut self, val: super::vals::Timer0Rst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "CTIMER1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_rst(&self) -> super::vals::Timer1Rst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Timer1Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER1 reset control"]
    #[inline(always)]
    pub const fn set_timer1_rst(&mut self, val: super::vals::Timer1Rst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "SmartDMA reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_rst(&self) -> super::vals::SmartDmaRst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SmartDmaRst::from_bits(val as u8)
    }
    #[doc = "SmartDMA reset control"]
    #[inline(always)]
    pub const fn set_smart_dma_rst(&mut self, val: super::vals::SmartDmaRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("adc0_rst", &self.adc0_rst())
            .field("adc1_rst", &self.adc1_rst())
            .field("dac0_rst", &self.dac0_rst())
            .field("rtc_rst", &self.rtc_rst())
            .field("evsim0_rst", &self.evsim0_rst())
            .field("evsim1_rst", &self.evsim1_rst())
            .field("utick_rst", &self.utick_rst())
            .field("fc0_rst", &self.fc0_rst())
            .field("fc1_rst", &self.fc1_rst())
            .field("fc2_rst", &self.fc2_rst())
            .field("fc3_rst", &self.fc3_rst())
            .field("fc4_rst", &self.fc4_rst())
            .field("fc5_rst", &self.fc5_rst())
            .field("fc6_rst", &self.fc6_rst())
            .field("fc7_rst", &self.fc7_rst())
            .field("fc8_rst", &self.fc8_rst())
            .field("fc9_rst", &self.fc9_rst())
            .field("micfil_rst", &self.micfil_rst())
            .field("timer2_rst", &self.timer2_rst())
            .field("usb0_fs_dcd_rst", &self.usb0_fs_dcd_rst())
            .field("usb0_fs_rst", &self.usb0_fs_rst())
            .field("timer0_rst", &self.timer0_rst())
            .field("timer1_rst", &self.timer1_rst())
            .field("smart_dma_rst", &self.smart_dma_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl1 {{ mrt_rst: {:?}, ostimer_rst: {:?}, sct_rst: {:?}, adc0_rst: {:?}, adc1_rst: {:?}, dac0_rst: {:?}, rtc_rst: {:?}, evsim0_rst: {:?}, evsim1_rst: {:?}, utick_rst: {:?}, fc0_rst: {:?}, fc1_rst: {:?}, fc2_rst: {:?}, fc3_rst: {:?}, fc4_rst: {:?}, fc5_rst: {:?}, fc6_rst: {:?}, fc7_rst: {:?}, fc8_rst: {:?}, fc9_rst: {:?}, micfil_rst: {:?}, timer2_rst: {:?}, usb0_fs_dcd_rst: {:?}, usb0_fs_rst: {:?}, timer0_rst: {:?}, timer1_rst: {:?}, smart_dma_rst: {:?} }}",
            self.mrt_rst(),
            self.ostimer_rst(),
            self.sct_rst(),
            self.adc0_rst(),
            self.adc1_rst(),
            self.dac0_rst(),
            self.rtc_rst(),
            self.evsim0_rst(),
            self.evsim1_rst(),
            self.utick_rst(),
            self.fc0_rst(),
            self.fc1_rst(),
            self.fc2_rst(),
            self.fc3_rst(),
            self.fc4_rst(),
            self.fc5_rst(),
            self.fc6_rst(),
            self.fc7_rst(),
            self.fc8_rst(),
            self.fc9_rst(),
            self.micfil_rst(),
            self.timer2_rst(),
            self.usb0_fs_dcd_rst(),
            self.usb0_fs_rst(),
            self.timer0_rst(),
            self.timer1_rst(),
            self.smart_dma_rst()
        )
    }
}
#[doc = "Peripheral Reset Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl2(pub u32);
impl Presetctrl2 {
    #[doc = "DMA1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_rst(&self) -> super::vals::Dma1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dma1Rst::from_bits(val as u8)
    }
    #[doc = "DMA1 reset control"]
    #[inline(always)]
    pub const fn set_dma1_rst(&mut self, val: super::vals::Dma1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Ethernet reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_rst(&self) -> super::vals::EnetRst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EnetRst::from_bits(val as u8)
    }
    #[doc = "Ethernet reset control"]
    #[inline(always)]
    pub const fn set_enet_rst(&mut self, val: super::vals::EnetRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "uSDHC reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc_rst(&self) -> super::vals::UsdhcRst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::UsdhcRst::from_bits(val as u8)
    }
    #[doc = "uSDHC reset control"]
    #[inline(always)]
    pub const fn set_usdhc_rst(&mut self, val: super::vals::UsdhcRst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXIO reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio_rst(&self) -> super::vals::FlexioRst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::FlexioRst::from_bits(val as u8)
    }
    #[doc = "FLEXIO reset control"]
    #[inline(always)]
    pub const fn set_flexio_rst(&mut self, val: super::vals::FlexioRst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SAI0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sai0_rst(&self) -> super::vals::Sai0Rst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sai0Rst::from_bits(val as u8)
    }
    #[doc = "SAI0 reset control"]
    #[inline(always)]
    pub const fn set_sai0_rst(&mut self, val: super::vals::Sai0Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SAI1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_rst(&self) -> super::vals::Sai1Rst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sai1Rst::from_bits(val as u8)
    }
    #[doc = "SAI1 reset control"]
    #[inline(always)]
    pub const fn set_sai1_rst(&mut self, val: super::vals::Sai1Rst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "TRO reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn tro_rst(&self) -> super::vals::TroRst {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TroRst::from_bits(val as u8)
    }
    #[doc = "TRO reset control"]
    #[inline(always)]
    pub const fn set_tro_rst(&mut self, val: super::vals::TroRst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FREQME reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_rst(&self) -> super::vals::FreqmeRst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FreqmeRst::from_bits(val as u8)
    }
    #[doc = "FREQME reset control"]
    #[inline(always)]
    pub const fn set_freqme_rst(&mut self, val: super::vals::FreqmeRst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TRNG reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_rst(&self) -> super::vals::TrngRst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::TrngRst::from_bits(val as u8)
    }
    #[doc = "TRNG reset control"]
    #[inline(always)]
    pub const fn set_trng_rst(&mut self, val: super::vals::TrngRst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "CAN0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0_rst(&self) -> super::vals::Flexcan0Rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Flexcan0Rst::from_bits(val as u8)
    }
    #[doc = "CAN0 reset control"]
    #[inline(always)]
    pub const fn set_flexcan0_rst(&mut self, val: super::vals::Flexcan0Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "CAN1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1_rst(&self) -> super::vals::Flexcan1Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Flexcan1Rst::from_bits(val as u8)
    }
    #[doc = "CAN1 reset control"]
    #[inline(always)]
    pub const fn set_flexcan1_rst(&mut self, val: super::vals::Flexcan1Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USB HS reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_rst(&self) -> super::vals::UsbHsRst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::UsbHsRst::from_bits(val as u8)
    }
    #[doc = "USB HS reset control"]
    #[inline(always)]
    pub const fn set_usb_hs_rst(&mut self, val: super::vals::UsbHsRst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB HS PHY reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_phy_rst(&self) -> super::vals::UsbHsPhyRst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::UsbHsPhyRst::from_bits(val as u8)
    }
    #[doc = "USB HS PHY reset control"]
    #[inline(always)]
    pub const fn set_usb_hs_phy_rst(&mut self, val: super::vals::UsbHsPhyRst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "PowerQuad reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_rst(&self) -> super::vals::PqRst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PqRst::from_bits(val as u8)
    }
    #[doc = "PowerQuad reset control"]
    #[inline(always)]
    pub const fn set_pq_rst(&mut self, val: super::vals::PqRst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "PLU reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn plu_rst(&self) -> super::vals::PluRst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PluRst::from_bits(val as u8)
    }
    #[doc = "PLU reset control"]
    #[inline(always)]
    pub const fn set_plu_rst(&mut self, val: super::vals::PluRst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "CTIMER3 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn timer3_rst(&self) -> super::vals::Timer3Rst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Timer3Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER3 reset control"]
    #[inline(always)]
    pub const fn set_timer3_rst(&mut self, val: super::vals::Timer3Rst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "CTIMER4 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn timer4_rst(&self) -> super::vals::Timer4Rst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Timer4Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER4 reset control"]
    #[inline(always)]
    pub const fn set_timer4_rst(&mut self, val: super::vals::Timer4Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "PUF reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_rst(&self) -> super::vals::PufRst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PufRst::from_bits(val as u8)
    }
    #[doc = "PUF reset control"]
    #[inline(always)]
    pub const fn set_puf_rst(&mut self, val: super::vals::PufRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "PKC reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_rst(&self) -> super::vals::PkcRst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::PkcRst::from_bits(val as u8)
    }
    #[doc = "PKC reset control"]
    #[inline(always)]
    pub const fn set_pkc_rst(&mut self, val: super::vals::PkcRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SM3 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sm3_rst(&self) -> super::vals::Sm3Rst {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Sm3Rst::from_bits(val as u8)
    }
    #[doc = "SM3 reset control"]
    #[inline(always)]
    pub const fn set_sm3_rst(&mut self, val: super::vals::Sm3Rst) {
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
            .field("enet_rst", &self.enet_rst())
            .field("usdhc_rst", &self.usdhc_rst())
            .field("flexio_rst", &self.flexio_rst())
            .field("sai0_rst", &self.sai0_rst())
            .field("sai1_rst", &self.sai1_rst())
            .field("tro_rst", &self.tro_rst())
            .field("freqme_rst", &self.freqme_rst())
            .field("trng_rst", &self.trng_rst())
            .field("flexcan0_rst", &self.flexcan0_rst())
            .field("flexcan1_rst", &self.flexcan1_rst())
            .field("usb_hs_rst", &self.usb_hs_rst())
            .field("usb_hs_phy_rst", &self.usb_hs_phy_rst())
            .field("pq_rst", &self.pq_rst())
            .field("plu_rst", &self.plu_rst())
            .field("timer3_rst", &self.timer3_rst())
            .field("timer4_rst", &self.timer4_rst())
            .field("puf_rst", &self.puf_rst())
            .field("pkc_rst", &self.pkc_rst())
            .field("sm3_rst", &self.sm3_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl2 {{ dma1_rst: {:?}, enet_rst: {:?}, usdhc_rst: {:?}, flexio_rst: {:?}, sai0_rst: {:?}, sai1_rst: {:?}, tro_rst: {:?}, freqme_rst: {:?}, trng_rst: {:?}, flexcan0_rst: {:?}, flexcan1_rst: {:?}, usb_hs_rst: {:?}, usb_hs_phy_rst: {:?}, pq_rst: {:?}, plu_rst: {:?}, timer3_rst: {:?}, timer4_rst: {:?}, puf_rst: {:?}, pkc_rst: {:?}, sm3_rst: {:?} }}",
            self.dma1_rst(),
            self.enet_rst(),
            self.usdhc_rst(),
            self.flexio_rst(),
            self.sai0_rst(),
            self.sai1_rst(),
            self.tro_rst(),
            self.freqme_rst(),
            self.trng_rst(),
            self.flexcan0_rst(),
            self.flexcan1_rst(),
            self.usb_hs_rst(),
            self.usb_hs_phy_rst(),
            self.pq_rst(),
            self.plu_rst(),
            self.timer3_rst(),
            self.timer4_rst(),
            self.puf_rst(),
            self.pkc_rst(),
            self.sm3_rst()
        )
    }
}
#[doc = "Peripheral Reset Control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl3(pub u32);
impl Presetctrl3 {
    #[doc = "I3C0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_rst(&self) -> super::vals::I3c0Rst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3c0Rst::from_bits(val as u8)
    }
    #[doc = "I3C0 reset control"]
    #[inline(always)]
    pub const fn set_i3c0_rst(&mut self, val: super::vals::I3c0Rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1_rst(&self) -> super::vals::I3c1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::I3c1Rst::from_bits(val as u8)
    }
    #[doc = "I3C1 reset control"]
    #[inline(always)]
    pub const fn set_i3c1_rst(&mut self, val: super::vals::I3c1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SINC reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_rst(&self) -> super::vals::SincRst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SincRst::from_bits(val as u8)
    }
    #[doc = "SINC reset control"]
    #[inline(always)]
    pub const fn set_sinc_rst(&mut self, val: super::vals::SincRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CoolFlux reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_rst(&self) -> super::vals::CoolfluxRst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CoolfluxRst::from_bits(val as u8)
    }
    #[doc = "CoolFlux reset control"]
    #[inline(always)]
    pub const fn set_coolflux_rst(&mut self, val: super::vals::CoolfluxRst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "QDC0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0_rst(&self) -> super::vals::Qdc0Rst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Qdc0Rst::from_bits(val as u8)
    }
    #[doc = "QDC0 reset control"]
    #[inline(always)]
    pub const fn set_qdc0_rst(&mut self, val: super::vals::Qdc0Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "QDC1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1_rst(&self) -> super::vals::Qdc1Rst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Qdc1Rst::from_bits(val as u8)
    }
    #[doc = "QDC1 reset control"]
    #[inline(always)]
    pub const fn set_qdc1_rst(&mut self, val: super::vals::Qdc1Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "PWM0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm0_rst(&self) -> super::vals::Pwm0Rst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pwm0Rst::from_bits(val as u8)
    }
    #[doc = "PWM0 reset control"]
    #[inline(always)]
    pub const fn set_pwm0_rst(&mut self, val: super::vals::Pwm0Rst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "PWM1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn pwm1_rst(&self) -> super::vals::Pwm1Rst {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pwm1Rst::from_bits(val as u8)
    }
    #[doc = "PWM1 reset control"]
    #[inline(always)]
    pub const fn set_pwm1_rst(&mut self, val: super::vals::Pwm1Rst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AOI0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0_rst(&self) -> super::vals::Aoi0Rst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Aoi0Rst::from_bits(val as u8)
    }
    #[doc = "AOI0 reset control"]
    #[inline(always)]
    pub const fn set_aoi0_rst(&mut self, val: super::vals::Aoi0Rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DAC1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn dac1_rst(&self) -> super::vals::Dac1Rst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dac1Rst::from_bits(val as u8)
    }
    #[doc = "DAC1 reset control"]
    #[inline(always)]
    pub const fn set_dac1_rst(&mut self, val: super::vals::Dac1Rst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DAC2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn dac2_rst(&self) -> super::vals::Dac2Rst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dac2Rst::from_bits(val as u8)
    }
    #[doc = "DAC2 reset control"]
    #[inline(always)]
    pub const fn set_dac2_rst(&mut self, val: super::vals::Dac2Rst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "OPAMP0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0_rst(&self) -> super::vals::Opamp0Rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Opamp0Rst::from_bits(val as u8)
    }
    #[doc = "OPAMP0 reset control"]
    #[inline(always)]
    pub const fn set_opamp0_rst(&mut self, val: super::vals::Opamp0Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "OPAMP1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1_rst(&self) -> super::vals::Opamp1Rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Opamp1Rst::from_bits(val as u8)
    }
    #[doc = "OPAMP1 reset control"]
    #[inline(always)]
    pub const fn set_opamp1_rst(&mut self, val: super::vals::Opamp1Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "OPAMP2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2_rst(&self) -> super::vals::Opamp2Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Opamp2Rst::from_bits(val as u8)
    }
    #[doc = "OPAMP2 reset control"]
    #[inline(always)]
    pub const fn set_opamp2_rst(&mut self, val: super::vals::Opamp2Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "CMP2 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2_rst(&self) -> super::vals::Cmp2Rst {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cmp2Rst::from_bits(val as u8)
    }
    #[doc = "CMP2 reset control"]
    #[inline(always)]
    pub const fn set_cmp2_rst(&mut self, val: super::vals::Cmp2Rst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "VREF reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_rst(&self) -> super::vals::VrefRst {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::VrefRst::from_bits(val as u8)
    }
    #[doc = "VREF reset control"]
    #[inline(always)]
    pub const fn set_vref_rst(&mut self, val: super::vals::VrefRst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "CoolFlux APB reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_apb_rst(&self) -> super::vals::CoolfluxApbRst {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::CoolfluxApbRst::from_bits(val as u8)
    }
    #[doc = "CoolFlux APB reset control"]
    #[inline(always)]
    pub const fn set_coolflux_apb_rst(&mut self, val: super::vals::CoolfluxApbRst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "NPU reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn npu_rst(&self) -> super::vals::NpuRst {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::NpuRst::from_bits(val as u8)
    }
    #[doc = "NPU reset control"]
    #[inline(always)]
    pub const fn set_npu_rst(&mut self, val: super::vals::NpuRst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TSI reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn tsi_rst(&self) -> super::vals::TsiRst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::TsiRst::from_bits(val as u8)
    }
    #[doc = "TSI reset control"]
    #[inline(always)]
    pub const fn set_tsi_rst(&mut self, val: super::vals::TsiRst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "EWM reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn ewm_rst(&self) -> super::vals::EwmRst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::EwmRst::from_bits(val as u8)
    }
    #[doc = "EWM reset control"]
    #[inline(always)]
    pub const fn set_ewm_rst(&mut self, val: super::vals::EwmRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "EIM reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn eim_rst(&self) -> super::vals::EimRst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::EimRst::from_bits(val as u8)
    }
    #[doc = "EIM reset control"]
    #[inline(always)]
    pub const fn set_eim_rst(&mut self, val: super::vals::EimRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Semaphore reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sema42_rst(&self) -> super::vals::Sema42Rst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Sema42Rst::from_bits(val as u8)
    }
    #[doc = "Semaphore reset control"]
    #[inline(always)]
    pub const fn set_sema42_rst(&mut self, val: super::vals::Sema42Rst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Presetctrl3 {
    #[inline(always)]
    fn default() -> Presetctrl3 {
        Presetctrl3(0)
    }
}
impl core::fmt::Debug for Presetctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl3")
            .field("i3c0_rst", &self.i3c0_rst())
            .field("i3c1_rst", &self.i3c1_rst())
            .field("sinc_rst", &self.sinc_rst())
            .field("coolflux_rst", &self.coolflux_rst())
            .field("qdc0_rst", &self.qdc0_rst())
            .field("qdc1_rst", &self.qdc1_rst())
            .field("pwm0_rst", &self.pwm0_rst())
            .field("pwm1_rst", &self.pwm1_rst())
            .field("aoi0_rst", &self.aoi0_rst())
            .field("dac1_rst", &self.dac1_rst())
            .field("dac2_rst", &self.dac2_rst())
            .field("opamp0_rst", &self.opamp0_rst())
            .field("opamp1_rst", &self.opamp1_rst())
            .field("opamp2_rst", &self.opamp2_rst())
            .field("cmp2_rst", &self.cmp2_rst())
            .field("vref_rst", &self.vref_rst())
            .field("coolflux_apb_rst", &self.coolflux_apb_rst())
            .field("npu_rst", &self.npu_rst())
            .field("tsi_rst", &self.tsi_rst())
            .field("ewm_rst", &self.ewm_rst())
            .field("eim_rst", &self.eim_rst())
            .field("sema42_rst", &self.sema42_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl3 {{ i3c0_rst: {:?}, i3c1_rst: {:?}, sinc_rst: {:?}, coolflux_rst: {:?}, qdc0_rst: {:?}, qdc1_rst: {:?}, pwm0_rst: {:?}, pwm1_rst: {:?}, aoi0_rst: {:?}, dac1_rst: {:?}, dac2_rst: {:?}, opamp0_rst: {:?}, opamp1_rst: {:?}, opamp2_rst: {:?}, cmp2_rst: {:?}, vref_rst: {:?}, coolflux_apb_rst: {:?}, npu_rst: {:?}, tsi_rst: {:?}, ewm_rst: {:?}, eim_rst: {:?}, sema42_rst: {:?} }}",
            self.i3c0_rst(),
            self.i3c1_rst(),
            self.sinc_rst(),
            self.coolflux_rst(),
            self.qdc0_rst(),
            self.qdc1_rst(),
            self.pwm0_rst(),
            self.pwm1_rst(),
            self.aoi0_rst(),
            self.dac1_rst(),
            self.dac2_rst(),
            self.opamp0_rst(),
            self.opamp1_rst(),
            self.opamp2_rst(),
            self.cmp2_rst(),
            self.vref_rst(),
            self.coolflux_apb_rst(),
            self.npu_rst(),
            self.tsi_rst(),
            self.ewm_rst(),
            self.eim_rst(),
            self.sema42_rst()
        )
    }
}
#[doc = "Peripheral Reset Control Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrlclr(pub u32);
impl Presetctrlclr {
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
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
#[doc = "Peripheral Reset Control Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrlset(pub u32);
impl Presetctrlset {
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
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
#[doc = "PWM0 Submodule Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0subctl(pub u32);
impl Pwm0subctl {
    #[doc = "Enables PWM0 SUB Clock0"]
    #[must_use]
    #[inline(always)]
    pub const fn clk0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock0"]
    #[inline(always)]
    pub const fn set_clk0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables PWM0 SUB Clock1"]
    #[must_use]
    #[inline(always)]
    pub const fn clk1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock1"]
    #[inline(always)]
    pub const fn set_clk1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables PWM0 SUB Clock2"]
    #[must_use]
    #[inline(always)]
    pub const fn clk2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock2"]
    #[inline(always)]
    pub const fn set_clk2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables PWM0 SUB Clock3"]
    #[must_use]
    #[inline(always)]
    pub const fn clk3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock3"]
    #[inline(always)]
    pub const fn set_clk3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PWM0 submodule 0 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM0 submodule 0 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PWM0 submodule 1 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM0 submodule 1 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PWM0 submodule 2 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM0 submodule 2 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PWM0 submodule 3 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM0 submodule 3 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Pwm0subctl {
    #[inline(always)]
    fn default() -> Pwm0subctl {
        Pwm0subctl(0)
    }
}
impl core::fmt::Debug for Pwm0subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm0subctl")
            .field("clk0_en", &self.clk0_en())
            .field("clk1_en", &self.clk1_en())
            .field("clk2_en", &self.clk2_en())
            .field("clk3_en", &self.clk3_en())
            .field("dmavalm0", &self.dmavalm0())
            .field("dmavalm1", &self.dmavalm1())
            .field("dmavalm2", &self.dmavalm2())
            .field("dmavalm3", &self.dmavalm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm0subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm0subctl {{ clk0_en: {=bool:?}, clk1_en: {=bool:?}, clk2_en: {=bool:?}, clk3_en: {=bool:?}, dmavalm0: {=bool:?}, dmavalm1: {=bool:?}, dmavalm2: {=bool:?}, dmavalm3: {=bool:?} }}",
            self.clk0_en(),
            self.clk1_en(),
            self.clk2_en(),
            self.clk3_en(),
            self.dmavalm0(),
            self.dmavalm1(),
            self.dmavalm2(),
            self.dmavalm3()
        )
    }
}
#[doc = "PWM1 Submodule Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1subctl(pub u32);
impl Pwm1subctl {
    #[doc = "Enables PWM1 SUB Clock0"]
    #[must_use]
    #[inline(always)]
    pub const fn clk0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock0"]
    #[inline(always)]
    pub const fn set_clk0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables PWM1 SUB Clock1"]
    #[must_use]
    #[inline(always)]
    pub const fn clk1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock1"]
    #[inline(always)]
    pub const fn set_clk1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables PWM1 SUB Clock2"]
    #[must_use]
    #[inline(always)]
    pub const fn clk2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock2"]
    #[inline(always)]
    pub const fn set_clk2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables PWM1 SUB Clock3"]
    #[must_use]
    #[inline(always)]
    pub const fn clk3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock3"]
    #[inline(always)]
    pub const fn set_clk3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PWM1 submodule 0 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM1 submodule 0 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PWM1 submodule 1 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM1 submodule 1 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PWM1 submodule 2 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM1 submodule 2 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PWM1 submodule 3 DMA compare value done mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM1 submodule 3 DMA compare value done mask"]
    #[inline(always)]
    pub const fn set_dmavalm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Pwm1subctl {
    #[inline(always)]
    fn default() -> Pwm1subctl {
        Pwm1subctl(0)
    }
}
impl core::fmt::Debug for Pwm1subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm1subctl")
            .field("clk0_en", &self.clk0_en())
            .field("clk1_en", &self.clk1_en())
            .field("clk2_en", &self.clk2_en())
            .field("clk3_en", &self.clk3_en())
            .field("dmavalm0", &self.dmavalm0())
            .field("dmavalm1", &self.dmavalm1())
            .field("dmavalm2", &self.dmavalm2())
            .field("dmavalm3", &self.dmavalm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm1subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm1subctl {{ clk0_en: {=bool:?}, clk1_en: {=bool:?}, clk2_en: {=bool:?}, clk3_en: {=bool:?}, dmavalm0: {=bool:?}, dmavalm1: {=bool:?}, dmavalm2: {=bool:?}, dmavalm3: {=bool:?} }}",
            self.clk0_en(),
            self.clk1_en(),
            self.clk2_en(),
            self.clk3_en(),
            self.dmavalm0(),
            self.dmavalm1(),
            self.dmavalm2(),
            self.dmavalm3()
        )
    }
}
#[doc = "Control PKC RAM Interleave Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamInterleave(pub u32);
impl RamInterleave {
    #[doc = "Controls PKC RAM access for PKC RAM 0 and PKC RAM 1"]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> super::vals::Interleave {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Interleave::from_bits(val as u8)
    }
    #[doc = "Controls PKC RAM access for PKC RAM 0 and PKC RAM 1"]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: super::vals::Interleave) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RamInterleave {
    #[inline(always)]
    fn default() -> RamInterleave {
        RamInterleave(0)
    }
}
impl core::fmt::Debug for RamInterleave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamInterleave")
            .field("interleave", &self.interleave())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamInterleave {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RamInterleave {{ interleave: {:?} }}", self.interleave())
    }
}
#[doc = "FRO 48MHz Reference Clock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefClkCtrl(pub u32);
impl RefClkCtrl {
    #[doc = "GDET reference clock enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_refclk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET reference clock enable bit"]
    #[inline(always)]
    pub const fn set_gdet_refclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ELS TRNG reference clock enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_refclk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ELS TRNG reference clock enable bit"]
    #[inline(always)]
    pub const fn set_trng_refclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RefClkCtrl {
    #[inline(always)]
    fn default() -> RefClkCtrl {
        RefClkCtrl(0)
    }
}
impl core::fmt::Debug for RefClkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefClkCtrl")
            .field("gdet_refclk_en", &self.gdet_refclk_en())
            .field("trng_refclk_en", &self.trng_refclk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefClkCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RefClkCtrl {{ gdet_refclk_en: {=bool:?}, trng_refclk_en: {=bool:?} }}",
            self.gdet_refclk_en(),
            self.trng_refclk_en()
        )
    }
}
#[doc = "FRO 48MHz Reference Clock Control Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefClkCtrlClr(pub u32);
impl RefClkCtrlClr {
    #[doc = "GDET reference clock enable clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_refclk_en_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET reference clock enable clear bit"]
    #[inline(always)]
    pub const fn set_gdet_refclk_en_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ELS TRNG reference clock enable clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_refclk_en_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ELS TRNG reference clock enable clear bit"]
    #[inline(always)]
    pub const fn set_trng_refclk_en_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RefClkCtrlClr {
    #[inline(always)]
    fn default() -> RefClkCtrlClr {
        RefClkCtrlClr(0)
    }
}
impl core::fmt::Debug for RefClkCtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefClkCtrlClr")
            .field("gdet_refclk_en_clr", &self.gdet_refclk_en_clr())
            .field("trng_refclk_en_clr", &self.trng_refclk_en_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefClkCtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RefClkCtrlClr {{ gdet_refclk_en_clr: {=bool:?}, trng_refclk_en_clr: {=bool:?} }}",
            self.gdet_refclk_en_clr(),
            self.trng_refclk_en_clr()
        )
    }
}
#[doc = "FRO 48MHz Reference Clock Control Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefClkCtrlSet(pub u32);
impl RefClkCtrlSet {
    #[doc = "GDET reference clock enable set bit"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_refclk_en_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET reference clock enable set bit"]
    #[inline(always)]
    pub const fn set_gdet_refclk_en_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ELS TRNG reference clock enable set bit"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_refclk_en_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ELS TRNG reference clock enable set bit"]
    #[inline(always)]
    pub const fn set_trng_refclk_en_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RefClkCtrlSet {
    #[inline(always)]
    fn default() -> RefClkCtrlSet {
        RefClkCtrlSet(0)
    }
}
impl core::fmt::Debug for RefClkCtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefClkCtrlSet")
            .field("gdet_refclk_en_set", &self.gdet_refclk_en_set())
            .field("trng_refclk_en_set", &self.trng_refclk_en_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefClkCtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RefClkCtrlSet {{ gdet_refclk_en_set: {=bool:?}, trng_refclk_en_set: {=bool:?} }}",
            self.gdet_refclk_en_set(),
            self.trng_refclk_en_set()
        )
    }
}
#[doc = "ROM Wait State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Romcr(pub u32);
impl Romcr {
    #[doc = "ROM waiting Arm core and other masters for one cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_wait(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ROM waiting Arm core and other masters for one cycle"]
    #[inline(always)]
    pub const fn set_rom_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Romcr {
    #[inline(always)]
    fn default() -> Romcr {
        Romcr(0)
    }
}
impl core::fmt::Debug for Romcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Romcr")
            .field("rom_wait", &self.rom_wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Romcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Romcr {{ rom_wait: {=bool:?} }}", self.rom_wait())
    }
}
#[doc = "SAI0 Function Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai0clkdiv(pub u32);
impl Sai0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Sai0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Sai0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Sai0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Sai0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Sai0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Sai0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Sai0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Sai0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Sai0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sai0clkdiv {
    #[inline(always)]
    fn default() -> Sai0clkdiv {
        Sai0clkdiv(0)
    }
}
impl core::fmt::Debug for Sai0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SAI0 Function Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai0clksel(pub u32);
impl Sai0clksel {
    #[doc = "Selects the clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sai0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sai0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Sai0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sai0clksel {
    #[inline(always)]
    fn default() -> Sai0clksel {
        Sai0clksel(0)
    }
}
impl core::fmt::Debug for Sai0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SAI1 Function Clock Division"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1clkdiv(pub u32);
impl Sai1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Sai1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Sai1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Sai1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Sai1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Sai1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Sai1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Sai1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Sai1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Sai1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sai1clkdiv {
    #[inline(always)]
    fn default() -> Sai1clkdiv {
        Sai1clkdiv(0)
    }
}
impl core::fmt::Debug for Sai1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SAI1 Function Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1clksel(pub u32);
impl Sai1clksel {
    #[doc = "Selects the clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sai1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sai1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Sai1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sai1clksel {
    #[inline(always)]
    fn default() -> Sai1clksel {
        Sai1clksel(0)
    }
}
impl core::fmt::Debug for Sai1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SCT/PWM Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctclkdiv(pub u32);
impl Sctclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::SctclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SctclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::SctclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::SctclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SctclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::SctclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::SctclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SctclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::SctclkdivUnstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SCTimer/PWM Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctclksel(pub u32);
impl Sctclksel {
    #[doc = "Selects the SCTimer/PWM clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SctclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SctclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the SCTimer/PWM clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SctclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
#[doc = "SINC FILTER Function Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sincfiltclksel(pub u32);
impl Sincfiltclksel {
    #[doc = "Selects the SINC FILTER function clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SincfiltclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SincfiltclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the SINC FILTER function clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SincfiltclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sincfiltclksel {
    #[inline(always)]
    fn default() -> Sincfiltclksel {
        Sincfiltclksel(0)
    }
}
impl core::fmt::Debug for Sincfiltclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sincfiltclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sincfiltclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sincfiltclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SLOW_CLK Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slowclkdiv(pub u32);
impl Slowclkdiv {
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::SlowclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SlowclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::SlowclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::SlowclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SlowclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::SlowclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::SlowclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SlowclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::SlowclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Slowclkdiv {
    #[inline(always)]
    fn default() -> Slowclkdiv {
        Slowclkdiv(0)
    }
}
impl core::fmt::Debug for Slowclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slowclkdiv")
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slowclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slowclkdiv {{ reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SmartDMA Interrupt Hijack"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaint(pub u32);
impl SmartDmaint {
    #[doc = "SmartDMA hijack NVIC IRQ1"]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ1"]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ17"]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ17"]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ18"]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ18"]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ30"]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ30"]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ35"]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ35"]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ36"]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ36"]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ37"]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ37"]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ38"]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ38"]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ42"]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ42"]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ45"]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ45"]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ47"]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ47"]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ50"]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ50"]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ51"]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ51"]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ66"]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ66"]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ67"]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ67"]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ77"]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ77"]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SmartDmaint {
    #[inline(always)]
    fn default() -> SmartDmaint {
        SmartDmaint(0)
    }
}
impl core::fmt::Debug for SmartDmaint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaint")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmartDmaint {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23()
        )
    }
}
#[doc = "CPU0 Software Debug Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu0(pub u32);
impl SwdAccessCpu0 {
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> super::vals::SwdAccessCpu0SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SwdAccessCpu0SecCode::from_bits(val as u32)
    }
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: super::vals::SwdAccessCpu0SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu0 {
    #[inline(always)]
    fn default() -> SwdAccessCpu0 {
        SwdAccessCpu0(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu0")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu0 {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[doc = "CPU1 Software Debug Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu1(pub u32);
impl SwdAccessCpu1 {
    #[doc = "Security code to allow CPU1 DAP: 0x12345678"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> super::vals::SwdAccessCpu1SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SwdAccessCpu1SecCode::from_bits(val as u32)
    }
    #[doc = "Security code to allow CPU1 DAP: 0x12345678"]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: super::vals::SwdAccessCpu1SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu1 {
    #[inline(always)]
    fn default() -> SwdAccessCpu1 {
        SwdAccessCpu1(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu1")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu1 {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[doc = "DSP Software Debug Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessDsp(pub u32);
impl SwdAccessDsp {
    #[doc = "DSP SWD-AP: 0x12345678"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> super::vals::SwdAccessDspSecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SwdAccessDspSecCode::from_bits(val as u32)
    }
    #[doc = "DSP SWD-AP: 0x12345678"]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: super::vals::SwdAccessDspSecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessDsp {
    #[inline(always)]
    fn default() -> SwdAccessDsp {
        SwdAccessDsp(0)
    }
}
impl core::fmt::Debug for SwdAccessDsp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessDsp")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessDsp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessDsp {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[doc = "CPU0 System Tick Timer Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkdiv0(pub u32);
impl Systickclkdiv0 {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Systickclkdiv0Reset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Systickclkdiv0Reset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Systickclkdiv0Reset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Systickclkdiv0Halt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Systickclkdiv0Halt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Systickclkdiv0Halt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Systickclkdiv0Unstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Systickclkdiv0Unstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Systickclkdiv0Unstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkdiv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickclkdiv0 {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CPU1 System Tick Timer Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkdiv1(pub u32);
impl Systickclkdiv1 {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Systickclkdiv1Reset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Systickclkdiv1Reset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Systickclkdiv1Reset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Systickclkdiv1Halt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Systickclkdiv1Halt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Systickclkdiv1Halt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Systickclkdiv1Unstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Systickclkdiv1Unstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Systickclkdiv1Unstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkdiv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickclkdiv1 {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CPU0 System Tick Timer Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclksel0(pub u32);
impl Systickclksel0 {
    #[doc = "Selects the System Tick Timer for CPU0 source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Systickclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Systickclksel0Sel::from_bits(val as u8)
    }
    #[doc = "Selects the System Tick Timer for CPU0 source"]
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
#[doc = "CPU1 System Tick Timer Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclksel1(pub u32);
impl Systickclksel1 {
    #[doc = "Selects the System Tick Timer for CPU1 source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Systickclksel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Systickclksel1Sel::from_bits(val as u8)
    }
    #[doc = "Selects the System Tick Timer for CPU1 source."]
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
#[doc = "TRACE Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclkdiv(pub u32);
impl Traceclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::TraceclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::TraceclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::TraceclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::TraceclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::TraceclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::TraceclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::TraceclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TraceclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::TraceclkdivUnstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Traceclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Traceclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Trace Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclksel(pub u32);
impl Traceclksel {
    #[doc = "Selects the trace clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::TraceclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TraceclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the trace clock source."]
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
#[doc = "TSI Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsiclkdiv(pub u32);
impl Tsiclkdiv {
    #[doc = "Clock divider value:"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value:"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::TsiclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::TsiclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::TsiclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::TsiclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::TsiclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::TsiclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::TsiclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TsiclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::TsiclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tsiclkdiv {
    #[inline(always)]
    fn default() -> Tsiclkdiv {
        Tsiclkdiv(0)
    }
}
impl core::fmt::Debug for Tsiclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsiclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsiclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsiclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "TSI Function Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsiclksel(pub u32);
impl Tsiclksel {
    #[doc = "Selects the TSI function clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::TsiclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TsiclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the TSI function clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::TsiclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Tsiclksel {
    #[inline(always)]
    fn default() -> Tsiclksel {
        Tsiclksel(0)
    }
}
impl core::fmt::Debug for Tsiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "uSDHC Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USdhcclkdiv(pub u32);
impl USdhcclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::USdhcclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::USdhcclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::USdhcclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::USdhcclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::USdhcclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::USdhcclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::USdhcclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::USdhcclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::USdhcclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for USdhcclkdiv {
    #[inline(always)]
    fn default() -> USdhcclkdiv {
        USdhcclkdiv(0)
    }
}
impl core::fmt::Debug for USdhcclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USdhcclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USdhcclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USdhcclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "uSDHC Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USdhcclksel(pub u32);
impl USdhcclksel {
    #[doc = "Selects the uSDHC clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::USdhcclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::USdhcclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the uSDHC clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::USdhcclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for USdhcclksel {
    #[inline(always)]
    fn default() -> USdhcclksel {
        USdhcclksel(0)
    }
}
impl core::fmt::Debug for USdhcclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USdhcclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USdhcclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USdhcclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "USB-FS Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0clkdiv(pub u32);
impl Usb0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Usb0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Usb0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Usb0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Usb0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Usb0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Usb0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Usb0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usb0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Usb0clkdivUnstab) {
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
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "USB-FS Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0clksel(pub u32);
impl Usb0clksel {
    #[doc = "Selects the USB-FS clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Usb0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the USB-FS clock source."]
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
#[doc = "UTICK Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utickclkdiv(pub u32);
impl Utickclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::UtickclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::UtickclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::UtickclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::UtickclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::UtickclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::UtickclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::UtickclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::UtickclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::UtickclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Utickclkdiv {
    #[inline(always)]
    fn default() -> Utickclkdiv {
        Utickclkdiv(0)
    }
}
impl core::fmt::Debug for Utickclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utickclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utickclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Utickclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "UTICK Function Clock Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utickclksel(pub u32);
impl Utickclksel {
    #[doc = "Selects the clock source"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::UtickclkselSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::UtickclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the clock source"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::UtickclkselSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Utickclksel {
    #[inline(always)]
    fn default() -> Utickclksel {
        Utickclksel(0)
    }
}
impl core::fmt::Debug for Utickclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utickclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utickclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Utickclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "WDT0 Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt0clkdiv(pub u32);
impl Wdt0clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Wdt0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Wdt0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Wdt0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Wdt0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Wdt0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Wdt0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Wdt0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Wdt0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Wdt0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Wdt0clkdiv {
    #[inline(always)]
    fn default() -> Wdt0clkdiv {
        Wdt0clkdiv(0)
    }
}
impl core::fmt::Debug for Wdt0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdt0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "WDT1 Function Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt1clkdiv(pub u32);
impl Wdt1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Wdt1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Wdt1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Wdt1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Wdt1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Wdt1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Wdt1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Wdt1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Wdt1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Wdt1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Wdt1clkdiv {
    #[inline(always)]
    fn default() -> Wdt1clkdiv {
        Wdt1clkdiv(0)
    }
}
impl core::fmt::Debug for Wdt1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdt1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "WDT1 Clock Selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt1clksel(pub u32);
impl Wdt1clksel {
    #[doc = "Selects the WDT1 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Wdt1clkselSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Wdt1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the WDT1 clock"]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Wdt1clkselSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Wdt1clksel {
    #[inline(always)]
    fn default() -> Wdt1clksel {
        Wdt1clksel(0)
    }
}
impl core::fmt::Debug for Wdt1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdt1clksel {{ sel: {:?} }}", self.sel())
    }
}
