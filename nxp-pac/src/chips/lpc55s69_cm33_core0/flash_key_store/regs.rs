#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActivationCode(pub u32);
impl ActivationCode {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActivationCode {
    #[inline(always)]
    fn default() -> ActivationCode {
        ActivationCode(0)
    }
}
impl core::fmt::Debug for ActivationCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActivationCode")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActivationCode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ActivationCode {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Valid Key Sore Header : 0x95959595"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Header(pub u32);
impl Header {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Header {
    #[inline(always)]
    fn default() -> Header {
        Header(0)
    }
}
impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Header {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Header {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body0(pub u32);
impl PrinceRegion0Body0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body0 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body0 {
        PrinceRegion0Body0(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body1(pub u32);
impl PrinceRegion0Body1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body1 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body1 {
        PrinceRegion0Body1(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body10(pub u32);
impl PrinceRegion0Body10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body10 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body10 {
        PrinceRegion0Body10(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body11(pub u32);
impl PrinceRegion0Body11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body11 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body11 {
        PrinceRegion0Body11(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body2(pub u32);
impl PrinceRegion0Body2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body2 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body2 {
        PrinceRegion0Body2(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body3(pub u32);
impl PrinceRegion0Body3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body3 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body3 {
        PrinceRegion0Body3(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body4(pub u32);
impl PrinceRegion0Body4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body4 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body4 {
        PrinceRegion0Body4(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body5(pub u32);
impl PrinceRegion0Body5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body5 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body5 {
        PrinceRegion0Body5(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body6(pub u32);
impl PrinceRegion0Body6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body6 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body6 {
        PrinceRegion0Body6(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body7(pub u32);
impl PrinceRegion0Body7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body7 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body7 {
        PrinceRegion0Body7(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body8(pub u32);
impl PrinceRegion0Body8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body8 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body8 {
        PrinceRegion0Body8(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Body9(pub u32);
impl PrinceRegion0Body9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Body9 {
    #[inline(always)]
    fn default() -> PrinceRegion0Body9 {
        PrinceRegion0Body9(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Body9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Body9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Body9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion0Body9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Header0(pub u32);
impl PrinceRegion0Header0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0Header0 {
    #[inline(always)]
    fn default() -> PrinceRegion0Header0 {
        PrinceRegion0Header0(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Header0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Header0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Header0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0Header0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0Header1(pub u32);
impl PrinceRegion0Header1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PrinceRegion0Header1 {
    #[inline(always)]
    fn default() -> PrinceRegion0Header1 {
        PrinceRegion0Header1(0)
    }
}
impl core::fmt::Debug for PrinceRegion0Header1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0Header1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0Header1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0Header1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode0(pub u32);
impl PrinceRegion0KeyCode0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode0 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode0 {
        PrinceRegion0KeyCode0(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode1(pub u32);
impl PrinceRegion0KeyCode1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode1 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode1 {
        PrinceRegion0KeyCode1(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode10(pub u32);
impl PrinceRegion0KeyCode10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode10 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode10 {
        PrinceRegion0KeyCode10(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode11(pub u32);
impl PrinceRegion0KeyCode11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode11 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode11 {
        PrinceRegion0KeyCode11(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode12(pub u32);
impl PrinceRegion0KeyCode12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode12 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode12 {
        PrinceRegion0KeyCode12(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode12 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode13(pub u32);
impl PrinceRegion0KeyCode13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode13 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode13 {
        PrinceRegion0KeyCode13(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode13 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode2(pub u32);
impl PrinceRegion0KeyCode2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode2 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode2 {
        PrinceRegion0KeyCode2(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode3(pub u32);
impl PrinceRegion0KeyCode3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode3 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode3 {
        PrinceRegion0KeyCode3(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode4(pub u32);
impl PrinceRegion0KeyCode4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode4 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode4 {
        PrinceRegion0KeyCode4(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode5(pub u32);
impl PrinceRegion0KeyCode5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode5 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode5 {
        PrinceRegion0KeyCode5(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode6(pub u32);
impl PrinceRegion0KeyCode6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode6 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode6 {
        PrinceRegion0KeyCode6(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode7(pub u32);
impl PrinceRegion0KeyCode7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode7 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode7 {
        PrinceRegion0KeyCode7(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode8(pub u32);
impl PrinceRegion0KeyCode8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode8 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode8 {
        PrinceRegion0KeyCode8(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0KeyCode9(pub u32);
impl PrinceRegion0KeyCode9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0KeyCode9 {
    #[inline(always)]
    fn default() -> PrinceRegion0KeyCode9 {
        PrinceRegion0KeyCode9(0)
    }
}
impl core::fmt::Debug for PrinceRegion0KeyCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0KeyCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0KeyCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0KeyCode9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body0(pub u32);
impl PrinceRegion1Body0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body0 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body0 {
        PrinceRegion1Body0(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body1(pub u32);
impl PrinceRegion1Body1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body1 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body1 {
        PrinceRegion1Body1(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body10(pub u32);
impl PrinceRegion1Body10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body10 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body10 {
        PrinceRegion1Body10(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body11(pub u32);
impl PrinceRegion1Body11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body11 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body11 {
        PrinceRegion1Body11(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body2(pub u32);
impl PrinceRegion1Body2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body2 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body2 {
        PrinceRegion1Body2(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body3(pub u32);
impl PrinceRegion1Body3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body3 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body3 {
        PrinceRegion1Body3(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body4(pub u32);
impl PrinceRegion1Body4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body4 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body4 {
        PrinceRegion1Body4(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body5(pub u32);
impl PrinceRegion1Body5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body5 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body5 {
        PrinceRegion1Body5(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body6(pub u32);
impl PrinceRegion1Body6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body6 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body6 {
        PrinceRegion1Body6(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body7(pub u32);
impl PrinceRegion1Body7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body7 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body7 {
        PrinceRegion1Body7(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body8(pub u32);
impl PrinceRegion1Body8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body8 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body8 {
        PrinceRegion1Body8(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Body9(pub u32);
impl PrinceRegion1Body9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Body9 {
    #[inline(always)]
    fn default() -> PrinceRegion1Body9 {
        PrinceRegion1Body9(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Body9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Body9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Body9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion1Body9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Header0(pub u32);
impl PrinceRegion1Header0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1Header0 {
    #[inline(always)]
    fn default() -> PrinceRegion1Header0 {
        PrinceRegion1Header0(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Header0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Header0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Header0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1Header0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1Header1(pub u32);
impl PrinceRegion1Header1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PrinceRegion1Header1 {
    #[inline(always)]
    fn default() -> PrinceRegion1Header1 {
        PrinceRegion1Header1(0)
    }
}
impl core::fmt::Debug for PrinceRegion1Header1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1Header1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1Header1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1Header1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode0(pub u32);
impl PrinceRegion1KeyCode0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode0 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode0 {
        PrinceRegion1KeyCode0(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode1(pub u32);
impl PrinceRegion1KeyCode1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode1 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode1 {
        PrinceRegion1KeyCode1(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode10(pub u32);
impl PrinceRegion1KeyCode10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode10 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode10 {
        PrinceRegion1KeyCode10(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode11(pub u32);
impl PrinceRegion1KeyCode11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode11 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode11 {
        PrinceRegion1KeyCode11(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode12(pub u32);
impl PrinceRegion1KeyCode12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode12 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode12 {
        PrinceRegion1KeyCode12(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode12 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode13(pub u32);
impl PrinceRegion1KeyCode13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode13 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode13 {
        PrinceRegion1KeyCode13(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode13 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode2(pub u32);
impl PrinceRegion1KeyCode2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode2 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode2 {
        PrinceRegion1KeyCode2(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode3(pub u32);
impl PrinceRegion1KeyCode3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode3 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode3 {
        PrinceRegion1KeyCode3(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode4(pub u32);
impl PrinceRegion1KeyCode4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode4 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode4 {
        PrinceRegion1KeyCode4(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode5(pub u32);
impl PrinceRegion1KeyCode5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode5 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode5 {
        PrinceRegion1KeyCode5(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode6(pub u32);
impl PrinceRegion1KeyCode6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode6 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode6 {
        PrinceRegion1KeyCode6(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode7(pub u32);
impl PrinceRegion1KeyCode7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode7 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode7 {
        PrinceRegion1KeyCode7(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode8(pub u32);
impl PrinceRegion1KeyCode8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode8 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode8 {
        PrinceRegion1KeyCode8(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1KeyCode9(pub u32);
impl PrinceRegion1KeyCode9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1KeyCode9 {
    #[inline(always)]
    fn default() -> PrinceRegion1KeyCode9 {
        PrinceRegion1KeyCode9(0)
    }
}
impl core::fmt::Debug for PrinceRegion1KeyCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1KeyCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1KeyCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1KeyCode9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body0(pub u32);
impl PrinceRegion2Body0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body0 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body0 {
        PrinceRegion2Body0(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body1(pub u32);
impl PrinceRegion2Body1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body1 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body1 {
        PrinceRegion2Body1(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body10(pub u32);
impl PrinceRegion2Body10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body10 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body10 {
        PrinceRegion2Body10(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body11(pub u32);
impl PrinceRegion2Body11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body11 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body11 {
        PrinceRegion2Body11(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body2(pub u32);
impl PrinceRegion2Body2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body2 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body2 {
        PrinceRegion2Body2(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body3(pub u32);
impl PrinceRegion2Body3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body3 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body3 {
        PrinceRegion2Body3(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body4(pub u32);
impl PrinceRegion2Body4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body4 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body4 {
        PrinceRegion2Body4(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body5(pub u32);
impl PrinceRegion2Body5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body5 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body5 {
        PrinceRegion2Body5(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body6(pub u32);
impl PrinceRegion2Body6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body6 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body6 {
        PrinceRegion2Body6(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body7(pub u32);
impl PrinceRegion2Body7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body7 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body7 {
        PrinceRegion2Body7(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body8(pub u32);
impl PrinceRegion2Body8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body8 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body8 {
        PrinceRegion2Body8(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Body9(pub u32);
impl PrinceRegion2Body9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Body9 {
    #[inline(always)]
    fn default() -> PrinceRegion2Body9 {
        PrinceRegion2Body9(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Body9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Body9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Body9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceRegion2Body9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Header0(pub u32);
impl PrinceRegion2Header0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2Header0 {
    #[inline(always)]
    fn default() -> PrinceRegion2Header0 {
        PrinceRegion2Header0(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Header0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Header0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Header0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2Header0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2Header1(pub u32);
impl PrinceRegion2Header1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PrinceRegion2Header1 {
    #[inline(always)]
    fn default() -> PrinceRegion2Header1 {
        PrinceRegion2Header1(0)
    }
}
impl core::fmt::Debug for PrinceRegion2Header1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2Header1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2Header1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2Header1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode0(pub u32);
impl PrinceRegion2KeyCode0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode0 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode0 {
        PrinceRegion2KeyCode0(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode1(pub u32);
impl PrinceRegion2KeyCode1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode1 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode1 {
        PrinceRegion2KeyCode1(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode10(pub u32);
impl PrinceRegion2KeyCode10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode10 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode10 {
        PrinceRegion2KeyCode10(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode11(pub u32);
impl PrinceRegion2KeyCode11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode11 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode11 {
        PrinceRegion2KeyCode11(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode12(pub u32);
impl PrinceRegion2KeyCode12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode12 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode12 {
        PrinceRegion2KeyCode12(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode12 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode13(pub u32);
impl PrinceRegion2KeyCode13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode13 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode13 {
        PrinceRegion2KeyCode13(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode13 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode2(pub u32);
impl PrinceRegion2KeyCode2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode2 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode2 {
        PrinceRegion2KeyCode2(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode3(pub u32);
impl PrinceRegion2KeyCode3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode3 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode3 {
        PrinceRegion2KeyCode3(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode4(pub u32);
impl PrinceRegion2KeyCode4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode4 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode4 {
        PrinceRegion2KeyCode4(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode5(pub u32);
impl PrinceRegion2KeyCode5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode5 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode5 {
        PrinceRegion2KeyCode5(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode6(pub u32);
impl PrinceRegion2KeyCode6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode6 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode6 {
        PrinceRegion2KeyCode6(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode7(pub u32);
impl PrinceRegion2KeyCode7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode7 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode7 {
        PrinceRegion2KeyCode7(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode8(pub u32);
impl PrinceRegion2KeyCode8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode8 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode8 {
        PrinceRegion2KeyCode8(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2KeyCode9(pub u32);
impl PrinceRegion2KeyCode9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2KeyCode9 {
    #[inline(always)]
    fn default() -> PrinceRegion2KeyCode9 {
        PrinceRegion2KeyCode9(0)
    }
}
impl core::fmt::Debug for PrinceRegion2KeyCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2KeyCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2KeyCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2KeyCode9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "puf discharge time in ms."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PufDischargeTimeInMs(pub u32);
impl PufDischargeTimeInMs {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PufDischargeTimeInMs {
    #[inline(always)]
    fn default() -> PufDischargeTimeInMs {
        PufDischargeTimeInMs(0)
    }
}
impl core::fmt::Debug for PufDischargeTimeInMs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PufDischargeTimeInMs")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PufDischargeTimeInMs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PufDischargeTimeInMs {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody0(pub u32);
impl SbkeyBody0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody0 {
    #[inline(always)]
    fn default() -> SbkeyBody0 {
        SbkeyBody0(0)
    }
}
impl core::fmt::Debug for SbkeyBody0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody1(pub u32);
impl SbkeyBody1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody1 {
    #[inline(always)]
    fn default() -> SbkeyBody1 {
        SbkeyBody1(0)
    }
}
impl core::fmt::Debug for SbkeyBody1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody10(pub u32);
impl SbkeyBody10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody10 {
    #[inline(always)]
    fn default() -> SbkeyBody10 {
        SbkeyBody10(0)
    }
}
impl core::fmt::Debug for SbkeyBody10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody11(pub u32);
impl SbkeyBody11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody11 {
    #[inline(always)]
    fn default() -> SbkeyBody11 {
        SbkeyBody11(0)
    }
}
impl core::fmt::Debug for SbkeyBody11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody2(pub u32);
impl SbkeyBody2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody2 {
    #[inline(always)]
    fn default() -> SbkeyBody2 {
        SbkeyBody2(0)
    }
}
impl core::fmt::Debug for SbkeyBody2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody3(pub u32);
impl SbkeyBody3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody3 {
    #[inline(always)]
    fn default() -> SbkeyBody3 {
        SbkeyBody3(0)
    }
}
impl core::fmt::Debug for SbkeyBody3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody4(pub u32);
impl SbkeyBody4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody4 {
    #[inline(always)]
    fn default() -> SbkeyBody4 {
        SbkeyBody4(0)
    }
}
impl core::fmt::Debug for SbkeyBody4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody5(pub u32);
impl SbkeyBody5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody5 {
    #[inline(always)]
    fn default() -> SbkeyBody5 {
        SbkeyBody5(0)
    }
}
impl core::fmt::Debug for SbkeyBody5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody6(pub u32);
impl SbkeyBody6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody6 {
    #[inline(always)]
    fn default() -> SbkeyBody6 {
        SbkeyBody6(0)
    }
}
impl core::fmt::Debug for SbkeyBody6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody7(pub u32);
impl SbkeyBody7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody7 {
    #[inline(always)]
    fn default() -> SbkeyBody7 {
        SbkeyBody7(0)
    }
}
impl core::fmt::Debug for SbkeyBody7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody8(pub u32);
impl SbkeyBody8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody8 {
    #[inline(always)]
    fn default() -> SbkeyBody8 {
        SbkeyBody8(0)
    }
}
impl core::fmt::Debug for SbkeyBody8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyBody9(pub u32);
impl SbkeyBody9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyBody9 {
    #[inline(always)]
    fn default() -> SbkeyBody9 {
        SbkeyBody9(0)
    }
}
impl core::fmt::Debug for SbkeyBody9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyBody9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyBody9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyBody9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyHeader0(pub u32);
impl SbkeyHeader0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyHeader0 {
    #[inline(always)]
    fn default() -> SbkeyHeader0 {
        SbkeyHeader0(0)
    }
}
impl core::fmt::Debug for SbkeyHeader0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyHeader0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyHeader0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyHeader0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyHeader1(pub u32);
impl SbkeyHeader1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for SbkeyHeader1 {
    #[inline(always)]
    fn default() -> SbkeyHeader1 {
        SbkeyHeader1(0)
    }
}
impl core::fmt::Debug for SbkeyHeader1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyHeader1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyHeader1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SbkeyHeader1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode0(pub u32);
impl SbkeyKeyCode0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode0 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode0 {
        SbkeyKeyCode0(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode1(pub u32);
impl SbkeyKeyCode1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode1 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode1 {
        SbkeyKeyCode1(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode10(pub u32);
impl SbkeyKeyCode10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode10 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode10 {
        SbkeyKeyCode10(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode11(pub u32);
impl SbkeyKeyCode11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode11 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode11 {
        SbkeyKeyCode11(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode12(pub u32);
impl SbkeyKeyCode12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode12 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode12 {
        SbkeyKeyCode12(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode12 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode13(pub u32);
impl SbkeyKeyCode13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode13 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode13 {
        SbkeyKeyCode13(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode13 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode2(pub u32);
impl SbkeyKeyCode2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode2 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode2 {
        SbkeyKeyCode2(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode3(pub u32);
impl SbkeyKeyCode3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode3 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode3 {
        SbkeyKeyCode3(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode4(pub u32);
impl SbkeyKeyCode4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode4 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode4 {
        SbkeyKeyCode4(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode5(pub u32);
impl SbkeyKeyCode5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode5 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode5 {
        SbkeyKeyCode5(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode6(pub u32);
impl SbkeyKeyCode6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode6 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode6 {
        SbkeyKeyCode6(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode7(pub u32);
impl SbkeyKeyCode7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode7 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode7 {
        SbkeyKeyCode7(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode8(pub u32);
impl SbkeyKeyCode8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode8 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode8 {
        SbkeyKeyCode8(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbkeyKeyCode9(pub u32);
impl SbkeyKeyCode9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SbkeyKeyCode9 {
    #[inline(always)]
    fn default() -> SbkeyKeyCode9 {
        SbkeyKeyCode9(0)
    }
}
impl core::fmt::Debug for SbkeyKeyCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbkeyKeyCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbkeyKeyCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SbkeyKeyCode9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody0(pub u32);
impl UdsBody0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody0 {
    #[inline(always)]
    fn default() -> UdsBody0 {
        UdsBody0(0)
    }
}
impl core::fmt::Debug for UdsBody0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody1(pub u32);
impl UdsBody1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody1 {
    #[inline(always)]
    fn default() -> UdsBody1 {
        UdsBody1(0)
    }
}
impl core::fmt::Debug for UdsBody1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody10(pub u32);
impl UdsBody10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody10 {
    #[inline(always)]
    fn default() -> UdsBody10 {
        UdsBody10(0)
    }
}
impl core::fmt::Debug for UdsBody10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody11(pub u32);
impl UdsBody11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody11 {
    #[inline(always)]
    fn default() -> UdsBody11 {
        UdsBody11(0)
    }
}
impl core::fmt::Debug for UdsBody11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody2(pub u32);
impl UdsBody2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody2 {
    #[inline(always)]
    fn default() -> UdsBody2 {
        UdsBody2(0)
    }
}
impl core::fmt::Debug for UdsBody2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody3(pub u32);
impl UdsBody3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody3 {
    #[inline(always)]
    fn default() -> UdsBody3 {
        UdsBody3(0)
    }
}
impl core::fmt::Debug for UdsBody3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody4(pub u32);
impl UdsBody4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody4 {
    #[inline(always)]
    fn default() -> UdsBody4 {
        UdsBody4(0)
    }
}
impl core::fmt::Debug for UdsBody4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody5(pub u32);
impl UdsBody5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody5 {
    #[inline(always)]
    fn default() -> UdsBody5 {
        UdsBody5(0)
    }
}
impl core::fmt::Debug for UdsBody5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody6(pub u32);
impl UdsBody6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody6 {
    #[inline(always)]
    fn default() -> UdsBody6 {
        UdsBody6(0)
    }
}
impl core::fmt::Debug for UdsBody6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody7(pub u32);
impl UdsBody7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody7 {
    #[inline(always)]
    fn default() -> UdsBody7 {
        UdsBody7(0)
    }
}
impl core::fmt::Debug for UdsBody7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody8(pub u32);
impl UdsBody8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody8 {
    #[inline(always)]
    fn default() -> UdsBody8 {
        UdsBody8(0)
    }
}
impl core::fmt::Debug for UdsBody8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsBody9(pub u32);
impl UdsBody9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsBody9 {
    #[inline(always)]
    fn default() -> UdsBody9 {
        UdsBody9(0)
    }
}
impl core::fmt::Debug for UdsBody9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsBody9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsBody9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsBody9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsHeader0(pub u32);
impl UdsHeader0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsHeader0 {
    #[inline(always)]
    fn default() -> UdsHeader0 {
        UdsHeader0(0)
    }
}
impl core::fmt::Debug for UdsHeader0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsHeader0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsHeader0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsHeader0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsHeader1(pub u32);
impl UdsHeader1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for UdsHeader1 {
    #[inline(always)]
    fn default() -> UdsHeader1 {
        UdsHeader1(0)
    }
}
impl core::fmt::Debug for UdsHeader1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsHeader1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsHeader1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UdsHeader1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode0(pub u32);
impl UdsKeyCode0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode0 {
    #[inline(always)]
    fn default() -> UdsKeyCode0 {
        UdsKeyCode0(0)
    }
}
impl core::fmt::Debug for UdsKeyCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode1(pub u32);
impl UdsKeyCode1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode1 {
    #[inline(always)]
    fn default() -> UdsKeyCode1 {
        UdsKeyCode1(0)
    }
}
impl core::fmt::Debug for UdsKeyCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode10(pub u32);
impl UdsKeyCode10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode10 {
    #[inline(always)]
    fn default() -> UdsKeyCode10 {
        UdsKeyCode10(0)
    }
}
impl core::fmt::Debug for UdsKeyCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode11(pub u32);
impl UdsKeyCode11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode11 {
    #[inline(always)]
    fn default() -> UdsKeyCode11 {
        UdsKeyCode11(0)
    }
}
impl core::fmt::Debug for UdsKeyCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode12(pub u32);
impl UdsKeyCode12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode12 {
    #[inline(always)]
    fn default() -> UdsKeyCode12 {
        UdsKeyCode12(0)
    }
}
impl core::fmt::Debug for UdsKeyCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode12 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode13(pub u32);
impl UdsKeyCode13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode13 {
    #[inline(always)]
    fn default() -> UdsKeyCode13 {
        UdsKeyCode13(0)
    }
}
impl core::fmt::Debug for UdsKeyCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode13 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode2(pub u32);
impl UdsKeyCode2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode2 {
    #[inline(always)]
    fn default() -> UdsKeyCode2 {
        UdsKeyCode2(0)
    }
}
impl core::fmt::Debug for UdsKeyCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode3(pub u32);
impl UdsKeyCode3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode3 {
    #[inline(always)]
    fn default() -> UdsKeyCode3 {
        UdsKeyCode3(0)
    }
}
impl core::fmt::Debug for UdsKeyCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode4(pub u32);
impl UdsKeyCode4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode4 {
    #[inline(always)]
    fn default() -> UdsKeyCode4 {
        UdsKeyCode4(0)
    }
}
impl core::fmt::Debug for UdsKeyCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode5(pub u32);
impl UdsKeyCode5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode5 {
    #[inline(always)]
    fn default() -> UdsKeyCode5 {
        UdsKeyCode5(0)
    }
}
impl core::fmt::Debug for UdsKeyCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode6(pub u32);
impl UdsKeyCode6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode6 {
    #[inline(always)]
    fn default() -> UdsKeyCode6 {
        UdsKeyCode6(0)
    }
}
impl core::fmt::Debug for UdsKeyCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode7(pub u32);
impl UdsKeyCode7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode7 {
    #[inline(always)]
    fn default() -> UdsKeyCode7 {
        UdsKeyCode7(0)
    }
}
impl core::fmt::Debug for UdsKeyCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode8(pub u32);
impl UdsKeyCode8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode8 {
    #[inline(always)]
    fn default() -> UdsKeyCode8 {
        UdsKeyCode8(0)
    }
}
impl core::fmt::Debug for UdsKeyCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdsKeyCode9(pub u32);
impl UdsKeyCode9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdsKeyCode9 {
    #[inline(always)]
    fn default() -> UdsKeyCode9 {
        UdsKeyCode9(0)
    }
}
impl core::fmt::Debug for UdsKeyCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdsKeyCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdsKeyCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdsKeyCode9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody0(pub u32);
impl UserKekBody0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody0 {
    #[inline(always)]
    fn default() -> UserKekBody0 {
        UserKekBody0(0)
    }
}
impl core::fmt::Debug for UserKekBody0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody1(pub u32);
impl UserKekBody1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody1 {
    #[inline(always)]
    fn default() -> UserKekBody1 {
        UserKekBody1(0)
    }
}
impl core::fmt::Debug for UserKekBody1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody10(pub u32);
impl UserKekBody10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody10 {
    #[inline(always)]
    fn default() -> UserKekBody10 {
        UserKekBody10(0)
    }
}
impl core::fmt::Debug for UserKekBody10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody11(pub u32);
impl UserKekBody11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody11 {
    #[inline(always)]
    fn default() -> UserKekBody11 {
        UserKekBody11(0)
    }
}
impl core::fmt::Debug for UserKekBody11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody2(pub u32);
impl UserKekBody2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody2 {
    #[inline(always)]
    fn default() -> UserKekBody2 {
        UserKekBody2(0)
    }
}
impl core::fmt::Debug for UserKekBody2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody3(pub u32);
impl UserKekBody3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody3 {
    #[inline(always)]
    fn default() -> UserKekBody3 {
        UserKekBody3(0)
    }
}
impl core::fmt::Debug for UserKekBody3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody4(pub u32);
impl UserKekBody4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody4 {
    #[inline(always)]
    fn default() -> UserKekBody4 {
        UserKekBody4(0)
    }
}
impl core::fmt::Debug for UserKekBody4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody5(pub u32);
impl UserKekBody5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody5 {
    #[inline(always)]
    fn default() -> UserKekBody5 {
        UserKekBody5(0)
    }
}
impl core::fmt::Debug for UserKekBody5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody6(pub u32);
impl UserKekBody6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody6 {
    #[inline(always)]
    fn default() -> UserKekBody6 {
        UserKekBody6(0)
    }
}
impl core::fmt::Debug for UserKekBody6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody7(pub u32);
impl UserKekBody7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody7 {
    #[inline(always)]
    fn default() -> UserKekBody7 {
        UserKekBody7(0)
    }
}
impl core::fmt::Debug for UserKekBody7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody8(pub u32);
impl UserKekBody8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody8 {
    #[inline(always)]
    fn default() -> UserKekBody8 {
        UserKekBody8(0)
    }
}
impl core::fmt::Debug for UserKekBody8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekBody9(pub u32);
impl UserKekBody9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekBody9 {
    #[inline(always)]
    fn default() -> UserKekBody9 {
        UserKekBody9(0)
    }
}
impl core::fmt::Debug for UserKekBody9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekBody9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekBody9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekBody9 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekHeader0(pub u32);
impl UserKekHeader0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekHeader0 {
    #[inline(always)]
    fn default() -> UserKekHeader0 {
        UserKekHeader0(0)
    }
}
impl core::fmt::Debug for UserKekHeader0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekHeader0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekHeader0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekHeader0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekHeader1(pub u32);
impl UserKekHeader1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for UserKekHeader1 {
    #[inline(always)]
    fn default() -> UserKekHeader1 {
        UserKekHeader1(0)
    }
}
impl core::fmt::Debug for UserKekHeader1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekHeader1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekHeader1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UserKekHeader1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode0(pub u32);
impl UserKekKeyCode0 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode0 {
    #[inline(always)]
    fn default() -> UserKekKeyCode0 {
        UserKekKeyCode0(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode1(pub u32);
impl UserKekKeyCode1 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode1 {
    #[inline(always)]
    fn default() -> UserKekKeyCode1 {
        UserKekKeyCode1(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode10(pub u32);
impl UserKekKeyCode10 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode10 {
    #[inline(always)]
    fn default() -> UserKekKeyCode10 {
        UserKekKeyCode10(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode10 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode11(pub u32);
impl UserKekKeyCode11 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode11 {
    #[inline(always)]
    fn default() -> UserKekKeyCode11 {
        UserKekKeyCode11(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode11 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode12(pub u32);
impl UserKekKeyCode12 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode12 {
    #[inline(always)]
    fn default() -> UserKekKeyCode12 {
        UserKekKeyCode12(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode12 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode13(pub u32);
impl UserKekKeyCode13 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode13 {
    #[inline(always)]
    fn default() -> UserKekKeyCode13 {
        UserKekKeyCode13(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode13 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode2(pub u32);
impl UserKekKeyCode2 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode2 {
    #[inline(always)]
    fn default() -> UserKekKeyCode2 {
        UserKekKeyCode2(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode3(pub u32);
impl UserKekKeyCode3 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode3 {
    #[inline(always)]
    fn default() -> UserKekKeyCode3 {
        UserKekKeyCode3(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode3 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode4(pub u32);
impl UserKekKeyCode4 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode4 {
    #[inline(always)]
    fn default() -> UserKekKeyCode4 {
        UserKekKeyCode4(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode4 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode5(pub u32);
impl UserKekKeyCode5 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode5 {
    #[inline(always)]
    fn default() -> UserKekKeyCode5 {
        UserKekKeyCode5(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode5 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode6(pub u32);
impl UserKekKeyCode6 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode6 {
    #[inline(always)]
    fn default() -> UserKekKeyCode6 {
        UserKekKeyCode6(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode6 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode7(pub u32);
impl UserKekKeyCode7 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode7 {
    #[inline(always)]
    fn default() -> UserKekKeyCode7 {
        UserKekKeyCode7(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode7 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode8(pub u32);
impl UserKekKeyCode8 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode8 {
    #[inline(always)]
    fn default() -> UserKekKeyCode8 {
        UserKekKeyCode8(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode8 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UserKekKeyCode9(pub u32);
impl UserKekKeyCode9 {
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UserKekKeyCode9 {
    #[inline(always)]
    fn default() -> UserKekKeyCode9 {
        UserKekKeyCode9(0)
    }
}
impl core::fmt::Debug for UserKekKeyCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UserKekKeyCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UserKekKeyCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UserKekKeyCode9 {{ field: {=u32:?} }}", self.field())
    }
}
