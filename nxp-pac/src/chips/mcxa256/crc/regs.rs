#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "TCRC"]
    #[must_use]
    #[inline(always)]
    pub const fn tcrc(&self) -> super::vals::Tcrc {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Tcrc::from_bits(val as u8)
    }
    #[doc = "TCRC"]
    #[inline(always)]
    pub const fn set_tcrc(&mut self, val: super::vals::Tcrc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Write as Seed"]
    #[must_use]
    #[inline(always)]
    pub const fn was(&self) -> super::vals::Was {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Was::from_bits(val as u8)
    }
    #[doc = "Write as Seed"]
    #[inline(always)]
    pub const fn set_was(&mut self, val: super::vals::Was) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Complement Read of CRC Data Register"]
    #[must_use]
    #[inline(always)]
    pub const fn fxor(&self) -> super::vals::Fxor {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Fxor::from_bits(val as u8)
    }
    #[doc = "Complement Read of CRC Data Register"]
    #[inline(always)]
    pub const fn set_fxor(&mut self, val: super::vals::Fxor) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Transpose Type for Read"]
    #[must_use]
    #[inline(always)]
    pub const fn totr(&self) -> super::vals::Totr {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Totr::from_bits(val as u8)
    }
    #[doc = "Transpose Type for Read"]
    #[inline(always)]
    pub const fn set_totr(&mut self, val: super::vals::Totr) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Transpose Type for Write"]
    #[must_use]
    #[inline(always)]
    pub const fn tot(&self) -> super::vals::Tot {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Tot::from_bits(val as u8)
    }
    #[doc = "Transpose Type for Write"]
    #[inline(always)]
    pub const fn set_tot(&mut self, val: super::vals::Tot) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
            .field("tcrc", &self.tcrc())
            .field("was", &self.was())
            .field("fxor", &self.fxor())
            .field("totr", &self.totr())
            .field("tot", &self.tot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ tcrc: {:?}, was: {:?}, fxor: {:?}, totr: {:?}, tot: {:?} }}",
            self.tcrc(),
            self.was(),
            self.fxor(),
            self.totr(),
            self.tot()
        )
    }
}
#[doc = "Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "Lower Part of Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn ll(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Lower Part of Low Byte"]
    #[inline(always)]
    pub const fn set_ll(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Upper Part of Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn lu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Upper Part of Low Byte"]
    #[inline(always)]
    pub const fn set_lu(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Lower Part of High Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn hl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Lower Part of High Byte"]
    #[inline(always)]
    pub const fn set_hl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Upper Part of High Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Upper Part of High Byte"]
    #[inline(always)]
    pub const fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data")
            .field("ll", &self.ll())
            .field("lu", &self.lu())
            .field("hl", &self.hl())
            .field("hu", &self.hu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data {{ ll: {=u8:?}, lu: {=u8:?}, hl: {=u8:?}, hu: {=u8:?} }}",
            self.ll(),
            self.lu(),
            self.hl(),
            self.hu()
        )
    }
}
#[doc = "Polynomial"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpoly(pub u32);
impl Gpoly {
    #[doc = "Low Half-Word"]
    #[must_use]
    #[inline(always)]
    pub const fn low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low Half-Word"]
    #[inline(always)]
    pub const fn set_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High Half-Word"]
    #[must_use]
    #[inline(always)]
    pub const fn high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High Half-Word"]
    #[inline(always)]
    pub const fn set_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gpoly {
    #[inline(always)]
    fn default() -> Gpoly {
        Gpoly(0)
    }
}
impl core::fmt::Debug for Gpoly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpoly")
            .field("low", &self.low())
            .field("high", &self.high())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpoly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpoly {{ low: {=u16:?}, high: {=u16:?} }}",
            self.low(),
            self.high()
        )
    }
}
