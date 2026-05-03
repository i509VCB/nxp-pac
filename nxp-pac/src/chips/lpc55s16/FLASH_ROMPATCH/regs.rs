#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HEADER(pub u32);
impl HEADER {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn ENTRIES(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_ENTRIES(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SUB_TYPE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SUB_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn IDENTIFIER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_IDENTIFIER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for HEADER {
    #[inline(always)]
    fn default() -> HEADER {
        HEADER(0)
    }
}
impl core::fmt::Debug for HEADER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HEADER")
            .field("ENTRIES", &self.ENTRIES())
            .field("SUB_TYPE", &self.SUB_TYPE())
            .field("TYPE", &self.TYPE())
            .field("IDENTIFIER", &self.IDENTIFIER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HEADER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HEADER {{ ENTRIES: {=u8:?}, SUB_TYPE: {=u8:?}, TYPE: {=u8:?}, IDENTIFIER: {=u8:?} }}",
            self.ENTRIES(),
            self.SUB_TYPE(),
            self.TYPE(),
            self.IDENTIFIER()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PATCH(pub u32);
impl PATCH {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn PATCH(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_PATCH(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PATCH {
    #[inline(always)]
    fn default() -> PATCH {
        PATCH(0)
    }
}
impl core::fmt::Debug for PATCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PATCH")
            .field("PATCH", &self.PATCH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PATCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PATCH {{ PATCH: {=u32:?} }}", self.PATCH())
    }
}
