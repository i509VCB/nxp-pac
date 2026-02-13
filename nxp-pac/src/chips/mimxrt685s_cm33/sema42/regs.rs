#[doc = "Semphores2 Gate n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate(pub u8);
impl Gate {
    #[doc = "ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gtfsm::from_bits(val as u8)
    }
    #[doc = "ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: super::vals::Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate {
    #[inline(always)]
    fn default() -> Gate {
        Gate(0)
    }
}
impl core::fmt::Debug for Gate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Reset Gate Read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtR(pub u16);
impl RstgtR {
    #[doc = "RSTGTN"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RSTGTN"]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "RSTGMS"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "RSTGMS"]
    #[inline(always)]
    pub const fn set_rstgms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "RSTGSM"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgsm(&self) -> super::vals::Rstgsm {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Rstgsm::from_bits(val as u8)
    }
    #[doc = "RSTGSM"]
    #[inline(always)]
    pub const fn set_rstgsm(&mut self, val: super::vals::Rstgsm) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "ROZ"]
    #[must_use]
    #[inline(always)]
    pub const fn roz(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "ROZ"]
    #[inline(always)]
    pub const fn set_roz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for RstgtR {
    #[inline(always)]
    fn default() -> RstgtR {
        RstgtR(0)
    }
}
impl core::fmt::Debug for RstgtR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtR")
            .field("rstgtn", &self.rstgtn())
            .field("rstgms", &self.rstgms())
            .field("rstgsm", &self.rstgsm())
            .field("roz", &self.roz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RstgtR {{ rstgtn: {=u8:?}, rstgms: {=u8:?}, rstgsm: {:?}, roz: {=u8:?} }}",
            self.rstgtn(),
            self.rstgms(),
            self.rstgsm(),
            self.roz()
        )
    }
}
#[doc = "Reset Gate Write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtW(pub u16);
impl RstgtW {
    #[doc = "RSTGTN"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RSTGTN"]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "RSTGDP"]
    #[must_use]
    #[inline(always)]
    pub const fn rstgdp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RSTGDP"]
    #[inline(always)]
    pub const fn set_rstgdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RstgtW {
    #[inline(always)]
    fn default() -> RstgtW {
        RstgtW(0)
    }
}
impl core::fmt::Debug for RstgtW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtW")
            .field("rstgtn", &self.rstgtn())
            .field("rstgdp", &self.rstgdp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RstgtW {{ rstgtn: {=u8:?}, rstgdp: {=u8:?} }}",
            self.rstgtn(),
            self.rstgdp()
        )
    }
}
