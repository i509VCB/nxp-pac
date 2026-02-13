#[doc = "FRO16K Clock Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froclke(pub u32);
impl Froclke {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Froclke {
    #[inline(always)]
    fn default() -> Froclke {
        Froclke(0)
    }
}
impl core::fmt::Debug for Froclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "FRO16K Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctla(pub u32);
impl Froctla {
    #[doc = "FRO16K Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fro_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FRO16K Enable"]
    #[inline(always)]
    pub const fn set_fro_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Froctla {
    #[inline(always)]
    fn default() -> Froctla {
        Froctla(0)
    }
}
impl core::fmt::Debug for Froctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froctla")
            .field("fro_en", &self.fro_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froctla {{ fro_en: {=bool:?} }}", self.fro_en())
    }
}
#[doc = "FRO16K Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolcka(pub u32);
impl Frolcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Frolcka {
    #[inline(always)]
    fn default() -> Frolcka {
        Frolcka(0)
    }
}
impl core::fmt::Debug for Frolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[doc = "Wakeup 0 Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupa(pub u32);
impl Wakeupa {
    #[doc = "Register"]
    #[must_use]
    #[inline(always)]
    pub const fn reg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register"]
    #[inline(always)]
    pub const fn set_reg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wakeupa {
    #[inline(always)]
    fn default() -> Wakeupa {
        Wakeupa(0)
    }
}
impl core::fmt::Debug for Wakeupa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupa").field("reg", &self.reg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeupa {{ reg: {=u32:?} }}", self.reg())
    }
}
#[doc = "Wakeup Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklcka(pub u32);
impl Waklcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Waklcka {
    #[inline(always)]
    fn default() -> Waklcka {
        Waklcka(0)
    }
}
impl core::fmt::Debug for Waklcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Waklcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Waklcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Waklcka {{ lock: {=bool:?} }}", self.lock())
    }
}
