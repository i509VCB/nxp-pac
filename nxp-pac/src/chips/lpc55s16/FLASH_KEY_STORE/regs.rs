#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ACTIVATION_CODE(pub u32);
impl ACTIVATION_CODE {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ACTIVATION_CODE {
    #[inline(always)]
    fn default() -> ACTIVATION_CODE {
        ACTIVATION_CODE(0)
    }
}
impl core::fmt::Debug for ACTIVATION_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTIVATION_CODE")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ACTIVATION_CODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ACTIVATION_CODE {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "Valid Key Sore Header : 0x95959595."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HEADER(pub u32);
impl HEADER {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HEADER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HEADER {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY0(pub u32);
impl PRINCE_REGION0_BODY0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY0 {
        PRINCE_REGION0_BODY0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY1(pub u32);
impl PRINCE_REGION0_BODY1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY1 {
        PRINCE_REGION0_BODY1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY10(pub u32);
impl PRINCE_REGION0_BODY10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY10 {
        PRINCE_REGION0_BODY10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY11(pub u32);
impl PRINCE_REGION0_BODY11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY11 {
        PRINCE_REGION0_BODY11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY2(pub u32);
impl PRINCE_REGION0_BODY2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY2 {
        PRINCE_REGION0_BODY2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY3(pub u32);
impl PRINCE_REGION0_BODY3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY3 {
        PRINCE_REGION0_BODY3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY4(pub u32);
impl PRINCE_REGION0_BODY4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY4 {
        PRINCE_REGION0_BODY4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY5(pub u32);
impl PRINCE_REGION0_BODY5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY5 {
        PRINCE_REGION0_BODY5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY6(pub u32);
impl PRINCE_REGION0_BODY6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY6 {
        PRINCE_REGION0_BODY6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY7(pub u32);
impl PRINCE_REGION0_BODY7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY7 {
        PRINCE_REGION0_BODY7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY8(pub u32);
impl PRINCE_REGION0_BODY8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY8 {
        PRINCE_REGION0_BODY8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_BODY9(pub u32);
impl PRINCE_REGION0_BODY9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_BODY9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_BODY9 {
        PRINCE_REGION0_BODY9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_BODY9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_HEADER0(pub u32);
impl PRINCE_REGION0_HEADER0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_HEADER0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_HEADER0 {
        PRINCE_REGION0_HEADER0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_HEADER0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_HEADER1(pub u32);
impl PRINCE_REGION0_HEADER1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PRINCE_REGION0_HEADER1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_HEADER1 {
        PRINCE_REGION0_HEADER1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE0(pub u32);
impl PRINCE_REGION0_KEY_CODE0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE0 {
        PRINCE_REGION0_KEY_CODE0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE1(pub u32);
impl PRINCE_REGION0_KEY_CODE1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE1 {
        PRINCE_REGION0_KEY_CODE1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE10(pub u32);
impl PRINCE_REGION0_KEY_CODE10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE10 {
        PRINCE_REGION0_KEY_CODE10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE11(pub u32);
impl PRINCE_REGION0_KEY_CODE11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE11 {
        PRINCE_REGION0_KEY_CODE11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE12(pub u32);
impl PRINCE_REGION0_KEY_CODE12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE12 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE12 {
        PRINCE_REGION0_KEY_CODE12(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE12 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE13(pub u32);
impl PRINCE_REGION0_KEY_CODE13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE13 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE13 {
        PRINCE_REGION0_KEY_CODE13(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE13 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE2(pub u32);
impl PRINCE_REGION0_KEY_CODE2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE2 {
        PRINCE_REGION0_KEY_CODE2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE3(pub u32);
impl PRINCE_REGION0_KEY_CODE3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE3 {
        PRINCE_REGION0_KEY_CODE3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE4(pub u32);
impl PRINCE_REGION0_KEY_CODE4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE4 {
        PRINCE_REGION0_KEY_CODE4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE5(pub u32);
impl PRINCE_REGION0_KEY_CODE5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE5 {
        PRINCE_REGION0_KEY_CODE5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE6(pub u32);
impl PRINCE_REGION0_KEY_CODE6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE6 {
        PRINCE_REGION0_KEY_CODE6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE7(pub u32);
impl PRINCE_REGION0_KEY_CODE7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE7 {
        PRINCE_REGION0_KEY_CODE7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE8(pub u32);
impl PRINCE_REGION0_KEY_CODE8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE8 {
        PRINCE_REGION0_KEY_CODE8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_KEY_CODE9(pub u32);
impl PRINCE_REGION0_KEY_CODE9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION0_KEY_CODE9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_KEY_CODE9 {
        PRINCE_REGION0_KEY_CODE9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_KEY_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_KEY_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_KEY_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_KEY_CODE9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY0(pub u32);
impl PRINCE_REGION1_BODY0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY0 {
        PRINCE_REGION1_BODY0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY1(pub u32);
impl PRINCE_REGION1_BODY1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY1 {
        PRINCE_REGION1_BODY1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY10(pub u32);
impl PRINCE_REGION1_BODY10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY10 {
        PRINCE_REGION1_BODY10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY11(pub u32);
impl PRINCE_REGION1_BODY11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY11 {
        PRINCE_REGION1_BODY11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY2(pub u32);
impl PRINCE_REGION1_BODY2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY2 {
        PRINCE_REGION1_BODY2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY3(pub u32);
impl PRINCE_REGION1_BODY3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY3 {
        PRINCE_REGION1_BODY3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY4(pub u32);
impl PRINCE_REGION1_BODY4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY4 {
        PRINCE_REGION1_BODY4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY5(pub u32);
impl PRINCE_REGION1_BODY5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY5 {
        PRINCE_REGION1_BODY5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY6(pub u32);
impl PRINCE_REGION1_BODY6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY6 {
        PRINCE_REGION1_BODY6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY7(pub u32);
impl PRINCE_REGION1_BODY7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY7 {
        PRINCE_REGION1_BODY7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY8(pub u32);
impl PRINCE_REGION1_BODY8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY8 {
        PRINCE_REGION1_BODY8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_BODY9(pub u32);
impl PRINCE_REGION1_BODY9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_BODY9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_BODY9 {
        PRINCE_REGION1_BODY9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_BODY9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_HEADER0(pub u32);
impl PRINCE_REGION1_HEADER0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_HEADER0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_HEADER0 {
        PRINCE_REGION1_HEADER0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_HEADER0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_HEADER1(pub u32);
impl PRINCE_REGION1_HEADER1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PRINCE_REGION1_HEADER1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_HEADER1 {
        PRINCE_REGION1_HEADER1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE0(pub u32);
impl PRINCE_REGION1_KEY_CODE0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE0 {
        PRINCE_REGION1_KEY_CODE0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE1(pub u32);
impl PRINCE_REGION1_KEY_CODE1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE1 {
        PRINCE_REGION1_KEY_CODE1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE10(pub u32);
impl PRINCE_REGION1_KEY_CODE10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE10 {
        PRINCE_REGION1_KEY_CODE10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE11(pub u32);
impl PRINCE_REGION1_KEY_CODE11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE11 {
        PRINCE_REGION1_KEY_CODE11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE12(pub u32);
impl PRINCE_REGION1_KEY_CODE12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE12 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE12 {
        PRINCE_REGION1_KEY_CODE12(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE12 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE13(pub u32);
impl PRINCE_REGION1_KEY_CODE13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE13 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE13 {
        PRINCE_REGION1_KEY_CODE13(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE13 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE2(pub u32);
impl PRINCE_REGION1_KEY_CODE2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE2 {
        PRINCE_REGION1_KEY_CODE2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE3(pub u32);
impl PRINCE_REGION1_KEY_CODE3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE3 {
        PRINCE_REGION1_KEY_CODE3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE4(pub u32);
impl PRINCE_REGION1_KEY_CODE4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE4 {
        PRINCE_REGION1_KEY_CODE4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE5(pub u32);
impl PRINCE_REGION1_KEY_CODE5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE5 {
        PRINCE_REGION1_KEY_CODE5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE6(pub u32);
impl PRINCE_REGION1_KEY_CODE6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE6 {
        PRINCE_REGION1_KEY_CODE6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE7(pub u32);
impl PRINCE_REGION1_KEY_CODE7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE7 {
        PRINCE_REGION1_KEY_CODE7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE8(pub u32);
impl PRINCE_REGION1_KEY_CODE8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE8 {
        PRINCE_REGION1_KEY_CODE8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_KEY_CODE9(pub u32);
impl PRINCE_REGION1_KEY_CODE9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION1_KEY_CODE9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_KEY_CODE9 {
        PRINCE_REGION1_KEY_CODE9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_KEY_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_KEY_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_KEY_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_KEY_CODE9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY0(pub u32);
impl PRINCE_REGION2_BODY0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY0 {
        PRINCE_REGION2_BODY0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY1(pub u32);
impl PRINCE_REGION2_BODY1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY1 {
        PRINCE_REGION2_BODY1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY10(pub u32);
impl PRINCE_REGION2_BODY10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY10 {
        PRINCE_REGION2_BODY10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY11(pub u32);
impl PRINCE_REGION2_BODY11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY11 {
        PRINCE_REGION2_BODY11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY2(pub u32);
impl PRINCE_REGION2_BODY2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY2 {
        PRINCE_REGION2_BODY2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY3(pub u32);
impl PRINCE_REGION2_BODY3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY3 {
        PRINCE_REGION2_BODY3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY4(pub u32);
impl PRINCE_REGION2_BODY4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY4 {
        PRINCE_REGION2_BODY4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY5(pub u32);
impl PRINCE_REGION2_BODY5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY5 {
        PRINCE_REGION2_BODY5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY6(pub u32);
impl PRINCE_REGION2_BODY6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY6 {
        PRINCE_REGION2_BODY6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY7(pub u32);
impl PRINCE_REGION2_BODY7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY7 {
        PRINCE_REGION2_BODY7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY8(pub u32);
impl PRINCE_REGION2_BODY8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY8 {
        PRINCE_REGION2_BODY8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_BODY9(pub u32);
impl PRINCE_REGION2_BODY9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_BODY9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_BODY9 {
        PRINCE_REGION2_BODY9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_BODY9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_HEADER0(pub u32);
impl PRINCE_REGION2_HEADER0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_HEADER0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_HEADER0 {
        PRINCE_REGION2_HEADER0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_HEADER0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_HEADER1(pub u32);
impl PRINCE_REGION2_HEADER1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PRINCE_REGION2_HEADER1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_HEADER1 {
        PRINCE_REGION2_HEADER1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE0(pub u32);
impl PRINCE_REGION2_KEY_CODE0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE0 {
        PRINCE_REGION2_KEY_CODE0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE1(pub u32);
impl PRINCE_REGION2_KEY_CODE1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE1 {
        PRINCE_REGION2_KEY_CODE1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE10(pub u32);
impl PRINCE_REGION2_KEY_CODE10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE10 {
        PRINCE_REGION2_KEY_CODE10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE11(pub u32);
impl PRINCE_REGION2_KEY_CODE11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE11 {
        PRINCE_REGION2_KEY_CODE11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE12(pub u32);
impl PRINCE_REGION2_KEY_CODE12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE12 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE12 {
        PRINCE_REGION2_KEY_CODE12(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE12 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE13(pub u32);
impl PRINCE_REGION2_KEY_CODE13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE13 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE13 {
        PRINCE_REGION2_KEY_CODE13(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE13 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE2(pub u32);
impl PRINCE_REGION2_KEY_CODE2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE2 {
        PRINCE_REGION2_KEY_CODE2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE3(pub u32);
impl PRINCE_REGION2_KEY_CODE3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE3 {
        PRINCE_REGION2_KEY_CODE3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE4(pub u32);
impl PRINCE_REGION2_KEY_CODE4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE4 {
        PRINCE_REGION2_KEY_CODE4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE5(pub u32);
impl PRINCE_REGION2_KEY_CODE5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE5 {
        PRINCE_REGION2_KEY_CODE5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE6(pub u32);
impl PRINCE_REGION2_KEY_CODE6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE6 {
        PRINCE_REGION2_KEY_CODE6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE7(pub u32);
impl PRINCE_REGION2_KEY_CODE7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE7 {
        PRINCE_REGION2_KEY_CODE7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE8(pub u32);
impl PRINCE_REGION2_KEY_CODE8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE8 {
        PRINCE_REGION2_KEY_CODE8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_KEY_CODE9(pub u32);
impl PRINCE_REGION2_KEY_CODE9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_REGION2_KEY_CODE9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_KEY_CODE9 {
        PRINCE_REGION2_KEY_CODE9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_KEY_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_KEY_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_KEY_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_KEY_CODE9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY0(pub u32);
impl SBKEY_BODY0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY0 {
    #[inline(always)]
    fn default() -> SBKEY_BODY0 {
        SBKEY_BODY0(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY1(pub u32);
impl SBKEY_BODY1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY1 {
    #[inline(always)]
    fn default() -> SBKEY_BODY1 {
        SBKEY_BODY1(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY10(pub u32);
impl SBKEY_BODY10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY10 {
    #[inline(always)]
    fn default() -> SBKEY_BODY10 {
        SBKEY_BODY10(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY10 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY11(pub u32);
impl SBKEY_BODY11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY11 {
    #[inline(always)]
    fn default() -> SBKEY_BODY11 {
        SBKEY_BODY11(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY11 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY2(pub u32);
impl SBKEY_BODY2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY2 {
    #[inline(always)]
    fn default() -> SBKEY_BODY2 {
        SBKEY_BODY2(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY3(pub u32);
impl SBKEY_BODY3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY3 {
    #[inline(always)]
    fn default() -> SBKEY_BODY3 {
        SBKEY_BODY3(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY4(pub u32);
impl SBKEY_BODY4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY4 {
    #[inline(always)]
    fn default() -> SBKEY_BODY4 {
        SBKEY_BODY4(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY4 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY5(pub u32);
impl SBKEY_BODY5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY5 {
    #[inline(always)]
    fn default() -> SBKEY_BODY5 {
        SBKEY_BODY5(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY5 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY6(pub u32);
impl SBKEY_BODY6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY6 {
    #[inline(always)]
    fn default() -> SBKEY_BODY6 {
        SBKEY_BODY6(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY6 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY7(pub u32);
impl SBKEY_BODY7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY7 {
    #[inline(always)]
    fn default() -> SBKEY_BODY7 {
        SBKEY_BODY7(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY7 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY8(pub u32);
impl SBKEY_BODY8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY8 {
    #[inline(always)]
    fn default() -> SBKEY_BODY8 {
        SBKEY_BODY8(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY8 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_BODY9(pub u32);
impl SBKEY_BODY9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_BODY9 {
    #[inline(always)]
    fn default() -> SBKEY_BODY9 {
        SBKEY_BODY9(0)
    }
}
impl core::fmt::Debug for SBKEY_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_BODY9 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_HEADER0(pub u32);
impl SBKEY_HEADER0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_HEADER0 {
    #[inline(always)]
    fn default() -> SBKEY_HEADER0 {
        SBKEY_HEADER0(0)
    }
}
impl core::fmt::Debug for SBKEY_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_HEADER0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_HEADER1(pub u32);
impl SBKEY_HEADER1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for SBKEY_HEADER1 {
    #[inline(always)]
    fn default() -> SBKEY_HEADER1 {
        SBKEY_HEADER1(0)
    }
}
impl core::fmt::Debug for SBKEY_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SBKEY_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE0(pub u32);
impl SBKEY_KEY_CODE0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE0 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE0 {
        SBKEY_KEY_CODE0(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE1(pub u32);
impl SBKEY_KEY_CODE1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE1 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE1 {
        SBKEY_KEY_CODE1(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE10(pub u32);
impl SBKEY_KEY_CODE10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE10 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE10 {
        SBKEY_KEY_CODE10(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE10 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE11(pub u32);
impl SBKEY_KEY_CODE11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE11 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE11 {
        SBKEY_KEY_CODE11(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE11 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE12(pub u32);
impl SBKEY_KEY_CODE12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE12 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE12 {
        SBKEY_KEY_CODE12(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE12 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE13(pub u32);
impl SBKEY_KEY_CODE13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE13 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE13 {
        SBKEY_KEY_CODE13(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE13 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE2(pub u32);
impl SBKEY_KEY_CODE2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE2 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE2 {
        SBKEY_KEY_CODE2(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE3(pub u32);
impl SBKEY_KEY_CODE3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE3 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE3 {
        SBKEY_KEY_CODE3(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE4(pub u32);
impl SBKEY_KEY_CODE4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE4 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE4 {
        SBKEY_KEY_CODE4(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE4 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE5(pub u32);
impl SBKEY_KEY_CODE5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE5 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE5 {
        SBKEY_KEY_CODE5(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE5 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE6(pub u32);
impl SBKEY_KEY_CODE6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE6 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE6 {
        SBKEY_KEY_CODE6(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE6 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE7(pub u32);
impl SBKEY_KEY_CODE7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE7 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE7 {
        SBKEY_KEY_CODE7(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE7 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE8(pub u32);
impl SBKEY_KEY_CODE8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE8 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE8 {
        SBKEY_KEY_CODE8(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE8 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBKEY_KEY_CODE9(pub u32);
impl SBKEY_KEY_CODE9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SBKEY_KEY_CODE9 {
    #[inline(always)]
    fn default() -> SBKEY_KEY_CODE9 {
        SBKEY_KEY_CODE9(0)
    }
}
impl core::fmt::Debug for SBKEY_KEY_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBKEY_KEY_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBKEY_KEY_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBKEY_KEY_CODE9 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY0(pub u32);
impl UDS_BODY0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY0 {
    #[inline(always)]
    fn default() -> UDS_BODY0 {
        UDS_BODY0(0)
    }
}
impl core::fmt::Debug for UDS_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY1(pub u32);
impl UDS_BODY1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY1 {
    #[inline(always)]
    fn default() -> UDS_BODY1 {
        UDS_BODY1(0)
    }
}
impl core::fmt::Debug for UDS_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY10(pub u32);
impl UDS_BODY10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY10 {
    #[inline(always)]
    fn default() -> UDS_BODY10 {
        UDS_BODY10(0)
    }
}
impl core::fmt::Debug for UDS_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY10 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY11(pub u32);
impl UDS_BODY11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY11 {
    #[inline(always)]
    fn default() -> UDS_BODY11 {
        UDS_BODY11(0)
    }
}
impl core::fmt::Debug for UDS_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY11 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY2(pub u32);
impl UDS_BODY2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY2 {
    #[inline(always)]
    fn default() -> UDS_BODY2 {
        UDS_BODY2(0)
    }
}
impl core::fmt::Debug for UDS_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY3(pub u32);
impl UDS_BODY3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY3 {
    #[inline(always)]
    fn default() -> UDS_BODY3 {
        UDS_BODY3(0)
    }
}
impl core::fmt::Debug for UDS_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY4(pub u32);
impl UDS_BODY4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY4 {
    #[inline(always)]
    fn default() -> UDS_BODY4 {
        UDS_BODY4(0)
    }
}
impl core::fmt::Debug for UDS_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY4 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY5(pub u32);
impl UDS_BODY5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY5 {
    #[inline(always)]
    fn default() -> UDS_BODY5 {
        UDS_BODY5(0)
    }
}
impl core::fmt::Debug for UDS_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY5 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY6(pub u32);
impl UDS_BODY6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY6 {
    #[inline(always)]
    fn default() -> UDS_BODY6 {
        UDS_BODY6(0)
    }
}
impl core::fmt::Debug for UDS_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY6 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY7(pub u32);
impl UDS_BODY7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY7 {
    #[inline(always)]
    fn default() -> UDS_BODY7 {
        UDS_BODY7(0)
    }
}
impl core::fmt::Debug for UDS_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY7 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY8(pub u32);
impl UDS_BODY8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY8 {
    #[inline(always)]
    fn default() -> UDS_BODY8 {
        UDS_BODY8(0)
    }
}
impl core::fmt::Debug for UDS_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY8 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_BODY9(pub u32);
impl UDS_BODY9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_BODY9 {
    #[inline(always)]
    fn default() -> UDS_BODY9 {
        UDS_BODY9(0)
    }
}
impl core::fmt::Debug for UDS_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_BODY9 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_HEADER0(pub u32);
impl UDS_HEADER0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_HEADER0 {
    #[inline(always)]
    fn default() -> UDS_HEADER0 {
        UDS_HEADER0(0)
    }
}
impl core::fmt::Debug for UDS_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_HEADER0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_HEADER1(pub u32);
impl UDS_HEADER1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for UDS_HEADER1 {
    #[inline(always)]
    fn default() -> UDS_HEADER1 {
        UDS_HEADER1(0)
    }
}
impl core::fmt::Debug for UDS_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDS_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE0(pub u32);
impl UDS_KEY_CODE0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE0 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE0 {
        UDS_KEY_CODE0(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE1(pub u32);
impl UDS_KEY_CODE1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE1 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE1 {
        UDS_KEY_CODE1(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE10(pub u32);
impl UDS_KEY_CODE10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE10 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE10 {
        UDS_KEY_CODE10(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE10 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE11(pub u32);
impl UDS_KEY_CODE11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE11 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE11 {
        UDS_KEY_CODE11(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE11 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE12(pub u32);
impl UDS_KEY_CODE12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE12 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE12 {
        UDS_KEY_CODE12(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE12 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE13(pub u32);
impl UDS_KEY_CODE13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE13 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE13 {
        UDS_KEY_CODE13(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE13 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE2(pub u32);
impl UDS_KEY_CODE2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE2 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE2 {
        UDS_KEY_CODE2(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE3(pub u32);
impl UDS_KEY_CODE3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE3 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE3 {
        UDS_KEY_CODE3(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE4(pub u32);
impl UDS_KEY_CODE4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE4 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE4 {
        UDS_KEY_CODE4(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE4 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE5(pub u32);
impl UDS_KEY_CODE5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE5 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE5 {
        UDS_KEY_CODE5(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE5 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE6(pub u32);
impl UDS_KEY_CODE6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE6 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE6 {
        UDS_KEY_CODE6(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE6 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE7(pub u32);
impl UDS_KEY_CODE7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE7 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE7 {
        UDS_KEY_CODE7(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE7 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE8(pub u32);
impl UDS_KEY_CODE8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE8 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE8 {
        UDS_KEY_CODE8(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE8 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDS_KEY_CODE9(pub u32);
impl UDS_KEY_CODE9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDS_KEY_CODE9 {
    #[inline(always)]
    fn default() -> UDS_KEY_CODE9 {
        UDS_KEY_CODE9(0)
    }
}
impl core::fmt::Debug for UDS_KEY_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDS_KEY_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDS_KEY_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDS_KEY_CODE9 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY0(pub u32);
impl USER_KEK_BODY0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY0 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY0 {
        USER_KEK_BODY0(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY1(pub u32);
impl USER_KEK_BODY1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY1 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY1 {
        USER_KEK_BODY1(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY10(pub u32);
impl USER_KEK_BODY10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY10 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY10 {
        USER_KEK_BODY10(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY10 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY11(pub u32);
impl USER_KEK_BODY11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY11 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY11 {
        USER_KEK_BODY11(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY11 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY2(pub u32);
impl USER_KEK_BODY2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY2 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY2 {
        USER_KEK_BODY2(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY3(pub u32);
impl USER_KEK_BODY3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY3 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY3 {
        USER_KEK_BODY3(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY4(pub u32);
impl USER_KEK_BODY4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY4 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY4 {
        USER_KEK_BODY4(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY4 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY5(pub u32);
impl USER_KEK_BODY5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY5 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY5 {
        USER_KEK_BODY5(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY5 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY6(pub u32);
impl USER_KEK_BODY6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY6 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY6 {
        USER_KEK_BODY6(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY6 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY7(pub u32);
impl USER_KEK_BODY7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY7 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY7 {
        USER_KEK_BODY7(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY7 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY8(pub u32);
impl USER_KEK_BODY8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY8 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY8 {
        USER_KEK_BODY8(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY8 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_BODY9(pub u32);
impl USER_KEK_BODY9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_BODY9 {
    #[inline(always)]
    fn default() -> USER_KEK_BODY9 {
        USER_KEK_BODY9(0)
    }
}
impl core::fmt::Debug for USER_KEK_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_BODY9 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_HEADER0(pub u32);
impl USER_KEK_HEADER0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_HEADER0 {
    #[inline(always)]
    fn default() -> USER_KEK_HEADER0 {
        USER_KEK_HEADER0(0)
    }
}
impl core::fmt::Debug for USER_KEK_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_HEADER0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_HEADER1(pub u32);
impl USER_KEK_HEADER1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for USER_KEK_HEADER1 {
    #[inline(always)]
    fn default() -> USER_KEK_HEADER1 {
        USER_KEK_HEADER1(0)
    }
}
impl core::fmt::Debug for USER_KEK_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USER_KEK_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE0(pub u32);
impl USER_KEK_KEY_CODE0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE0 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE0 {
        USER_KEK_KEY_CODE0(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE1(pub u32);
impl USER_KEK_KEY_CODE1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE1 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE1 {
        USER_KEK_KEY_CODE1(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE10(pub u32);
impl USER_KEK_KEY_CODE10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE10 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE10 {
        USER_KEK_KEY_CODE10(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE10 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE11(pub u32);
impl USER_KEK_KEY_CODE11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE11 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE11 {
        USER_KEK_KEY_CODE11(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE11 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE12(pub u32);
impl USER_KEK_KEY_CODE12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE12 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE12 {
        USER_KEK_KEY_CODE12(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE12 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE13(pub u32);
impl USER_KEK_KEY_CODE13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE13 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE13 {
        USER_KEK_KEY_CODE13(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE13 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE2(pub u32);
impl USER_KEK_KEY_CODE2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE2 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE2 {
        USER_KEK_KEY_CODE2(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE3(pub u32);
impl USER_KEK_KEY_CODE3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE3 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE3 {
        USER_KEK_KEY_CODE3(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE4(pub u32);
impl USER_KEK_KEY_CODE4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE4 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE4 {
        USER_KEK_KEY_CODE4(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE4 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE5(pub u32);
impl USER_KEK_KEY_CODE5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE5 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE5 {
        USER_KEK_KEY_CODE5(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE5 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE6(pub u32);
impl USER_KEK_KEY_CODE6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE6 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE6 {
        USER_KEK_KEY_CODE6(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE6 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE7(pub u32);
impl USER_KEK_KEY_CODE7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE7 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE7 {
        USER_KEK_KEY_CODE7(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE7 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE8(pub u32);
impl USER_KEK_KEY_CODE8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE8 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE8 {
        USER_KEK_KEY_CODE8(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE8 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_KEK_KEY_CODE9(pub u32);
impl USER_KEK_KEY_CODE9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USER_KEK_KEY_CODE9 {
    #[inline(always)]
    fn default() -> USER_KEK_KEY_CODE9 {
        USER_KEK_KEY_CODE9(0)
    }
}
impl core::fmt::Debug for USER_KEK_KEY_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_KEK_KEY_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_KEK_KEY_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USER_KEK_KEY_CODE9 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "puf discharge time in ms."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct puf_discharge_time_in_ms(pub u32);
impl puf_discharge_time_in_ms {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for puf_discharge_time_in_ms {
    #[inline(always)]
    fn default() -> puf_discharge_time_in_ms {
        puf_discharge_time_in_ms(0)
    }
}
impl core::fmt::Debug for puf_discharge_time_in_ms {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("puf_discharge_time_in_ms")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for puf_discharge_time_in_ms {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "puf_discharge_time_in_ms {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
