#[doc = "GPIO data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "DR data bits"]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DR data bits"]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr").field("dr", &self.dr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr {{ dr: {=u32:?} }}", self.dr())
    }
}
#[doc = "GPIO data register CLEAR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrClear(pub u32);
impl DrClear {
    #[doc = "Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dr_clear(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub const fn set_dr_clear(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DrClear {
    #[inline(always)]
    fn default() -> DrClear {
        DrClear(0)
    }
}
impl core::fmt::Debug for DrClear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DrClear")
            .field("dr_clear", &self.dr_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DrClear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DrClear {{ dr_clear: {=u32:?} }}", self.dr_clear())
    }
}
#[doc = "GPIO data register SET"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrSet(pub u32);
impl DrSet {
    #[doc = "Set"]
    #[must_use]
    #[inline(always)]
    pub const fn dr_set(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set"]
    #[inline(always)]
    pub const fn set_dr_set(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DrSet {
    #[inline(always)]
    fn default() -> DrSet {
        DrSet(0)
    }
}
impl core::fmt::Debug for DrSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DrSet")
            .field("dr_set", &self.dr_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DrSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DrSet {{ dr_set: {=u32:?} }}", self.dr_set())
    }
}
#[doc = "GPIO data register TOGGLE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrToggle(pub u32);
impl DrToggle {
    #[doc = "Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn dr_toggle(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub const fn set_dr_toggle(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DrToggle {
    #[inline(always)]
    fn default() -> DrToggle {
        DrToggle(0)
    }
}
impl core::fmt::Debug for DrToggle {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DrToggle")
            .field("dr_toggle", &self.dr_toggle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DrToggle {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DrToggle {{ dr_toggle: {=u32:?} }}", self.dr_toggle())
    }
}
#[doc = "GPIO edge select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgeSel(pub u32);
impl EdgeSel {
    #[doc = "Edge select"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_edge_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Edge select"]
    #[inline(always)]
    pub const fn set_gpio_edge_sel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EdgeSel {
    #[inline(always)]
    fn default() -> EdgeSel {
        EdgeSel(0)
    }
}
impl core::fmt::Debug for EdgeSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgeSel")
            .field("gpio_edge_sel", &self.gpio_edge_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgeSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EdgeSel {{ gpio_edge_sel: {=u32:?} }}",
            self.gpio_edge_sel()
        )
    }
}
#[doc = "GPIO direction register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gdir(pub u32);
impl Gdir {
    #[doc = "GPIO direction bits"]
    #[must_use]
    #[inline(always)]
    pub const fn gdir(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO direction bits"]
    #[inline(always)]
    pub const fn set_gdir(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gdir {
    #[inline(always)]
    fn default() -> Gdir {
        Gdir(0)
    }
}
impl core::fmt::Debug for Gdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gdir").field("gdir", &self.gdir()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gdir {{ gdir: {=u32:?} }}", self.gdir())
    }
}
#[doc = "GPIO interrupt configuration register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt configuration field for GPIO interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> super::vals::Icr {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::Icr::from_bits(val as u8)
    }
    #[doc = "Interrupt configuration field for GPIO interrupt 0"]
    #[inline(always)]
    pub const fn set_pin(&mut self, n: usize, val: super::vals::Icr) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("pin[0]", &self.pin(0usize))
            .field("pin[1]", &self.pin(1usize))
            .field("pin[2]", &self.pin(2usize))
            .field("pin[3]", &self.pin(3usize))
            .field("pin[4]", &self.pin(4usize))
            .field("pin[5]", &self.pin(5usize))
            .field("pin[6]", &self.pin(6usize))
            .field("pin[7]", &self.pin(7usize))
            .field("pin[8]", &self.pin(8usize))
            .field("pin[9]", &self.pin(9usize))
            .field("pin[10]", &self.pin(10usize))
            .field("pin[11]", &self.pin(11usize))
            .field("pin[12]", &self.pin(12usize))
            .field("pin[13]", &self.pin(13usize))
            .field("pin[14]", &self.pin(14usize))
            .field("pin[15]", &self.pin(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ pin[0]: {:?}, pin[1]: {:?}, pin[2]: {:?}, pin[3]: {:?}, pin[4]: {:?}, pin[5]: {:?}, pin[6]: {:?}, pin[7]: {:?}, pin[8]: {:?}, pin[9]: {:?}, pin[10]: {:?}, pin[11]: {:?}, pin[12]: {:?}, pin[13]: {:?}, pin[14]: {:?}, pin[15]: {:?} }}",
            self.pin(0usize),
            self.pin(1usize),
            self.pin(2usize),
            self.pin(3usize),
            self.pin(4usize),
            self.pin(5usize),
            self.pin(6usize),
            self.pin(7usize),
            self.pin(8usize),
            self.pin(9usize),
            self.pin(10usize),
            self.pin(11usize),
            self.pin(12usize),
            self.pin(13usize),
            self.pin(14usize),
            self.pin(15usize)
        )
    }
}
#[doc = "GPIO interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Interrupt Mask bits"]
    #[must_use]
    #[inline(always)]
    pub const fn imr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Mask bits"]
    #[inline(always)]
    pub const fn set_imr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr").field("imr", &self.imr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr {{ imr: {=u32:?} }}", self.imr())
    }
}
#[doc = "GPIO interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Interrupt status bits"]
    #[must_use]
    #[inline(always)]
    pub const fn isr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt status bits"]
    #[inline(always)]
    pub const fn set_isr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr").field("isr", &self.isr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr {{ isr: {=u32:?} }}", self.isr())
    }
}
#[doc = "GPIO pad status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "GPIO pad status bits"]
    #[must_use]
    #[inline(always)]
    pub const fn psr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO pad status bits"]
    #[inline(always)]
    pub const fn set_psr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr").field("psr", &self.psr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Psr {{ psr: {=u32:?} }}", self.psr())
    }
}
