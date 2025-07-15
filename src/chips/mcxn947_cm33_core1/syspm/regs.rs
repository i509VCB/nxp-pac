#[doc = "Performance Monitor Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcr(pub u32);
impl Pmcr {
    #[doc = "Module Is Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn menb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Is Enabled"]
    #[inline(always)]
    pub const fn set_menb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start and Stop Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc(&self) -> super::vals::Ssc {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Ssc::from_bits(val as u8)
    }
    #[doc = "Start and Stop Control"]
    #[inline(always)]
    pub const fn set_ssc(&mut self, val: super::vals::Ssc) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cmode(&self) -> super::vals::Cmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cmode::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cmode(&mut self, val: super::vals::Cmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Reset Event Counter 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rectr1(&self) -> super::vals::Rectr1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rectr1::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter 1"]
    #[inline(always)]
    pub const fn set_rectr1(&mut self, val: super::vals::Rectr1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Event Counter 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rectr2(&self) -> super::vals::Rectr2 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rectr2::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter 2"]
    #[inline(always)]
    pub const fn set_rectr2(&mut self, val: super::vals::Rectr2) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset Event Counter 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rectr3(&self) -> super::vals::Rectr3 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rectr3::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter 3"]
    #[inline(always)]
    pub const fn set_rectr3(&mut self, val: super::vals::Rectr3) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Event 1"]
    #[must_use]
    #[inline(always)]
    pub const fn selevt1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x7f;
        val as u8
    }
    #[doc = "Select Event 1"]
    #[inline(always)]
    pub const fn set_selevt1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 11usize)) | (((val as u32) & 0x7f) << 11usize);
    }
    #[doc = "Select Event 2"]
    #[must_use]
    #[inline(always)]
    pub const fn selevt2(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x7f;
        val as u8
    }
    #[doc = "Select Event 2"]
    #[inline(always)]
    pub const fn set_selevt2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
    }
    #[doc = "Select Event 3"]
    #[must_use]
    #[inline(always)]
    pub const fn selevt3(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Select Event 3"]
    #[inline(always)]
    pub const fn set_selevt3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Pmcr {
    #[inline(always)]
    fn default() -> Pmcr {
        Pmcr(0)
    }
}
impl core::fmt::Debug for Pmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcr")
            .field("menb", &self.menb())
            .field("ssc", &self.ssc())
            .field("cmode", &self.cmode())
            .field("rectr1", &self.rectr1())
            .field("rectr2", &self.rectr2())
            .field("rectr3", &self.rectr3())
            .field("selevt1", &self.selevt1())
            .field("selevt2", &self.selevt2())
            .field("selevt3", &self.selevt3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmcr {{ menb: {=bool:?}, ssc: {:?}, cmode: {:?}, rectr1: {:?}, rectr2: {:?}, rectr3: {:?}, selevt1: {=u8:?}, selevt2: {=u8:?}, selevt3: {=u8:?} }}",
            self.menb(),
            self.ssc(),
            self.cmode(),
            self.rectr1(),
            self.rectr2(),
            self.rectr3(),
            self.selevt1(),
            self.selevt2(),
            self.selevt3()
        )
    }
}
#[doc = "Performance Monitor Event Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmectr1Hi(pub u8);
impl Pmectr1Hi {
    #[doc = "Event Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Event Counter"]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Pmectr1Hi {
    #[inline(always)]
    fn default() -> Pmectr1Hi {
        Pmectr1Hi(0)
    }
}
impl core::fmt::Debug for Pmectr1Hi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmectr1Hi")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmectr1Hi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmectr1Hi {{ ectr: {=u8:?} }}", self.ectr())
    }
}
#[doc = "Performance Monitor Event Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmectr1Lo(pub u32);
impl Pmectr1Lo {
    #[doc = "Event Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Event Counter"]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pmectr1Lo {
    #[inline(always)]
    fn default() -> Pmectr1Lo {
        Pmectr1Lo(0)
    }
}
impl core::fmt::Debug for Pmectr1Lo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmectr1Lo")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmectr1Lo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmectr1Lo {{ ectr: {=u32:?} }}", self.ectr())
    }
}
#[doc = "Performance Monitor Event Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmectr2Hi(pub u8);
impl Pmectr2Hi {
    #[doc = "Event Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Event Counter"]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Pmectr2Hi {
    #[inline(always)]
    fn default() -> Pmectr2Hi {
        Pmectr2Hi(0)
    }
}
impl core::fmt::Debug for Pmectr2Hi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmectr2Hi")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmectr2Hi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmectr2Hi {{ ectr: {=u8:?} }}", self.ectr())
    }
}
#[doc = "Performance Monitor Event Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmectr2Lo(pub u32);
impl Pmectr2Lo {
    #[doc = "Event Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Event Counter"]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pmectr2Lo {
    #[inline(always)]
    fn default() -> Pmectr2Lo {
        Pmectr2Lo(0)
    }
}
impl core::fmt::Debug for Pmectr2Lo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmectr2Lo")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmectr2Lo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmectr2Lo {{ ectr: {=u32:?} }}", self.ectr())
    }
}
#[doc = "Performance Monitor Event Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmectr3Hi(pub u8);
impl Pmectr3Hi {
    #[doc = "Event Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Event Counter"]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Pmectr3Hi {
    #[inline(always)]
    fn default() -> Pmectr3Hi {
        Pmectr3Hi(0)
    }
}
impl core::fmt::Debug for Pmectr3Hi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmectr3Hi")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmectr3Hi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmectr3Hi {{ ectr: {=u8:?} }}", self.ectr())
    }
}
#[doc = "Performance Monitor Event Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmectr3Lo(pub u32);
impl Pmectr3Lo {
    #[doc = "Event Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Event Counter"]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pmectr3Lo {
    #[inline(always)]
    fn default() -> Pmectr3Lo {
        Pmectr3Lo(0)
    }
}
impl core::fmt::Debug for Pmectr3Lo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmectr3Lo")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmectr3Lo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmectr3Lo {{ ectr: {=u32:?} }}", self.ectr())
    }
}
