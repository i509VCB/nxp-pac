#[doc = "ERM Memory 0 Correctable Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt0(pub u32);
impl CorrErrCnt0 {
    #[doc = "Memory n Correctable Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt0 {
    #[inline(always)]
    fn default() -> CorrErrCnt0 {
        CorrErrCnt0(0)
    }
}
impl core::fmt::Debug for CorrErrCnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt0")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CorrErrCnt0 {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "ERM Memory 1 Correctable Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt1(pub u32);
impl CorrErrCnt1 {
    #[doc = "Memory n Correctable Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt1 {
    #[inline(always)]
    fn default() -> CorrErrCnt1 {
        CorrErrCnt1(0)
    }
}
impl core::fmt::Debug for CorrErrCnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt1")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CorrErrCnt1 {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "ERM Configuration Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0(pub u32);
impl Cr0 {
    #[doc = "ENCIE1"]
    #[must_use]
    #[inline(always)]
    pub const fn encie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE1"]
    #[inline(always)]
    pub const fn set_encie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ESCIE1"]
    #[must_use]
    #[inline(always)]
    pub const fn escie1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE1"]
    #[inline(always)]
    pub const fn set_escie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ENCIE0"]
    #[must_use]
    #[inline(always)]
    pub const fn encie0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE0"]
    #[inline(always)]
    pub const fn set_encie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ESCIE0"]
    #[must_use]
    #[inline(always)]
    pub const fn escie0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE0"]
    #[inline(always)]
    pub const fn set_escie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        Cr0(0)
    }
}
impl core::fmt::Debug for Cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr0")
            .field("encie1", &self.encie1())
            .field("escie1", &self.escie1())
            .field("encie0", &self.encie0())
            .field("escie0", &self.escie0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr0 {{ encie1: {=bool:?}, escie1: {=bool:?}, encie0: {=bool:?}, escie0: {=bool:?} }}",
            self.encie1(),
            self.escie1(),
            self.encie0(),
            self.escie0()
        )
    }
}
#[doc = "ERM Memory 0 Error Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ear0(pub u32);
impl Ear0 {
    #[doc = "EAR"]
    #[must_use]
    #[inline(always)]
    pub const fn ear(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "EAR"]
    #[inline(always)]
    pub const fn set_ear(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ear0 {
    #[inline(always)]
    fn default() -> Ear0 {
        Ear0(0)
    }
}
impl core::fmt::Debug for Ear0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ear0").field("ear", &self.ear()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ear0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ear0 {{ ear: {=u32:?} }}", self.ear())
    }
}
#[doc = "ERM Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr0(pub u32);
impl Sr0 {
    #[doc = "NCE1"]
    #[must_use]
    #[inline(always)]
    pub const fn nce1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "NCE1"]
    #[inline(always)]
    pub const fn set_nce1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SBC1"]
    #[must_use]
    #[inline(always)]
    pub const fn sbc1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "SBC1"]
    #[inline(always)]
    pub const fn set_sbc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "NCE0"]
    #[must_use]
    #[inline(always)]
    pub const fn nce0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "NCE0"]
    #[inline(always)]
    pub const fn set_nce0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SBC0"]
    #[must_use]
    #[inline(always)]
    pub const fn sbc0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SBC0"]
    #[inline(always)]
    pub const fn set_sbc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sr0 {
    #[inline(always)]
    fn default() -> Sr0 {
        Sr0(0)
    }
}
impl core::fmt::Debug for Sr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr0")
            .field("nce1", &self.nce1())
            .field("sbc1", &self.sbc1())
            .field("nce0", &self.nce0())
            .field("sbc0", &self.sbc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr0 {{ nce1: {=bool:?}, sbc1: {=bool:?}, nce0: {=bool:?}, sbc0: {=bool:?} }}",
            self.nce1(),
            self.sbc1(),
            self.nce0(),
            self.sbc0()
        )
    }
}
#[doc = "ERM Memory 0 Syndrome Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syn0(pub u32);
impl Syn0 {
    #[doc = "SYNDROME"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SYNDROME"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Syn0 {
    #[inline(always)]
    fn default() -> Syn0 {
        Syn0(0)
    }
}
impl core::fmt::Debug for Syn0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syn0")
            .field("syndrome", &self.syndrome())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syn0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syn0 {{ syndrome: {=u8:?} }}", self.syndrome())
    }
}
