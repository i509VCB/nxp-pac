#[doc = "Clock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkctrl(pub u8);
impl Clkctrl {
    #[doc = "Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Clock Select"]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
    }
}
impl Default for Clkctrl {
    #[inline(always)]
    fn default() -> Clkctrl {
        Clkctrl(0)
    }
}
impl core::fmt::Debug for Clkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkctrl")
            .field("clksel", &self.clksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkctrl {{ clksel: {=u8:?} }}", self.clksel())
    }
}
#[doc = "Clock Prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkprescaler(pub u8);
impl Clkprescaler {
    #[doc = "Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Clkprescaler {
    #[inline(always)]
    fn default() -> Clkprescaler {
        Clkprescaler(0)
    }
}
impl core::fmt::Debug for Clkprescaler {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkprescaler")
            .field("clk_div", &self.clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkprescaler {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkprescaler {{ clk_div: {=u8:?} }}", self.clk_div())
    }
}
#[doc = "Compare High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmph(pub u8);
impl Cmph {
    #[doc = "Compare High"]
    #[must_use]
    #[inline(always)]
    pub const fn compareh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Compare High"]
    #[inline(always)]
    pub const fn set_compareh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Cmph {
    #[inline(always)]
    fn default() -> Cmph {
        Cmph(0)
    }
}
impl core::fmt::Debug for Cmph {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmph")
            .field("compareh", &self.compareh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmph {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmph {{ compareh: {=u8:?} }}", self.compareh())
    }
}
#[doc = "Compare Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpl(pub u8);
impl Cmpl {
    #[doc = "Compare Low"]
    #[must_use]
    #[inline(always)]
    pub const fn comparel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Compare Low"]
    #[inline(always)]
    pub const fn set_comparel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Cmpl {
    #[inline(always)]
    fn default() -> Cmpl {
        Cmpl(0)
    }
}
impl core::fmt::Debug for Cmpl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpl")
            .field("comparel", &self.comparel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmpl {{ comparel: {=u8:?} }}", self.comparel())
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u8);
impl Ctrl {
    #[doc = "EWM Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ewmen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EWM Enable"]
    #[inline(always)]
    pub const fn set_ewmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Assertion State Select"]
    #[must_use]
    #[inline(always)]
    pub const fn assin(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Assertion State Select"]
    #[inline(always)]
    pub const fn set_assin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn inen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input Enable"]
    #[inline(always)]
    pub const fn set_inen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn inten(&self) -> super::vals::Inten {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Inten::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn set_inten(&mut self, val: super::vals::Inten) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("ewmen", &self.ewmen())
            .field("assin", &self.assin())
            .field("inen", &self.inen())
            .field("inten", &self.inten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ ewmen: {=bool:?}, assin: {=bool:?}, inen: {=bool:?}, inten: {:?} }}",
            self.ewmen(),
            self.assin(),
            self.inen(),
            self.inten()
        )
    }
}
#[doc = "Service"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serv(pub u8);
impl Serv {
    #[doc = "Service"]
    #[must_use]
    #[inline(always)]
    pub const fn service(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Service"]
    #[inline(always)]
    pub const fn set_service(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Serv {
    #[inline(always)]
    fn default() -> Serv {
        Serv(0)
    }
}
impl core::fmt::Debug for Serv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serv")
            .field("service", &self.service())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Serv {{ service: {=u8:?} }}", self.service())
    }
}
