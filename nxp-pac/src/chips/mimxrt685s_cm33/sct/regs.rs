#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap0(pub u32);
impl Cap0 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap0 {
    #[inline(always)]
    fn default() -> Cap0 {
        Cap0(0)
    }
}
impl core::fmt::Debug for Cap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap0")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap0 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap1(pub u32);
impl Cap1 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap1 {
    #[inline(always)]
    fn default() -> Cap1 {
        Cap1(0)
    }
}
impl core::fmt::Debug for Cap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap1")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap1 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap10(pub u32);
impl Cap10 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap10 {
    #[inline(always)]
    fn default() -> Cap10 {
        Cap10(0)
    }
}
impl core::fmt::Debug for Cap10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap10")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap10 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap11(pub u32);
impl Cap11 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap11 {
    #[inline(always)]
    fn default() -> Cap11 {
        Cap11(0)
    }
}
impl core::fmt::Debug for Cap11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap11")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap11 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap12(pub u32);
impl Cap12 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap12 {
    #[inline(always)]
    fn default() -> Cap12 {
        Cap12(0)
    }
}
impl core::fmt::Debug for Cap12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap12")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap12 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap13(pub u32);
impl Cap13 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap13 {
    #[inline(always)]
    fn default() -> Cap13 {
        Cap13(0)
    }
}
impl core::fmt::Debug for Cap13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap13")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap13 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap14(pub u32);
impl Cap14 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap14 {
    #[inline(always)]
    fn default() -> Cap14 {
        Cap14(0)
    }
}
impl core::fmt::Debug for Cap14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap14")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap14 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap15(pub u32);
impl Cap15 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap15 {
    #[inline(always)]
    fn default() -> Cap15 {
        Cap15(0)
    }
}
impl core::fmt::Debug for Cap15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap15")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap15 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap2(pub u32);
impl Cap2 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap2 {
    #[inline(always)]
    fn default() -> Cap2 {
        Cap2(0)
    }
}
impl core::fmt::Debug for Cap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap2")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap2 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap3(pub u32);
impl Cap3 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap3 {
    #[inline(always)]
    fn default() -> Cap3 {
        Cap3(0)
    }
}
impl core::fmt::Debug for Cap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap3")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap3 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap4(pub u32);
impl Cap4 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap4 {
    #[inline(always)]
    fn default() -> Cap4 {
        Cap4(0)
    }
}
impl core::fmt::Debug for Cap4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap4")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap4 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap5(pub u32);
impl Cap5 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap5 {
    #[inline(always)]
    fn default() -> Cap5 {
        Cap5(0)
    }
}
impl core::fmt::Debug for Cap5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap5")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap5 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap6(pub u32);
impl Cap6 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap6 {
    #[inline(always)]
    fn default() -> Cap6 {
        Cap6(0)
    }
}
impl core::fmt::Debug for Cap6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap6")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap6 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap7(pub u32);
impl Cap7 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap7 {
    #[inline(always)]
    fn default() -> Cap7 {
        Cap7(0)
    }
}
impl core::fmt::Debug for Cap7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap7")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap7 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap8(pub u32);
impl Cap8 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap8 {
    #[inline(always)]
    fn default() -> Cap8 {
        Cap8(0)
    }
}
impl core::fmt::Debug for Cap8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap8")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap8 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap9(pub u32);
impl Cap9 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap9 {
    #[inline(always)]
    fn default() -> Cap9 {
        Cap9(0)
    }
}
impl core::fmt::Debug for Cap9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap9")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap9 {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl0(pub u32);
impl Capctrl0 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl0 {
    #[inline(always)]
    fn default() -> Capctrl0 {
        Capctrl0(0)
    }
}
impl core::fmt::Debug for Capctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl0")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl0 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl1(pub u32);
impl Capctrl1 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl1 {
    #[inline(always)]
    fn default() -> Capctrl1 {
        Capctrl1(0)
    }
}
impl core::fmt::Debug for Capctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl1")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl1 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl10(pub u32);
impl Capctrl10 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl10 {
    #[inline(always)]
    fn default() -> Capctrl10 {
        Capctrl10(0)
    }
}
impl core::fmt::Debug for Capctrl10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl10")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl10 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl11(pub u32);
impl Capctrl11 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl11 {
    #[inline(always)]
    fn default() -> Capctrl11 {
        Capctrl11(0)
    }
}
impl core::fmt::Debug for Capctrl11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl11")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl11 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl12(pub u32);
impl Capctrl12 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl12 {
    #[inline(always)]
    fn default() -> Capctrl12 {
        Capctrl12(0)
    }
}
impl core::fmt::Debug for Capctrl12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl12")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl12 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl13(pub u32);
impl Capctrl13 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl13 {
    #[inline(always)]
    fn default() -> Capctrl13 {
        Capctrl13(0)
    }
}
impl core::fmt::Debug for Capctrl13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl13")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl13 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl14(pub u32);
impl Capctrl14 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl14 {
    #[inline(always)]
    fn default() -> Capctrl14 {
        Capctrl14(0)
    }
}
impl core::fmt::Debug for Capctrl14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl14")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl14 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl15(pub u32);
impl Capctrl15 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl15 {
    #[inline(always)]
    fn default() -> Capctrl15 {
        Capctrl15(0)
    }
}
impl core::fmt::Debug for Capctrl15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl15")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl15 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl2(pub u32);
impl Capctrl2 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl2 {
    #[inline(always)]
    fn default() -> Capctrl2 {
        Capctrl2(0)
    }
}
impl core::fmt::Debug for Capctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl2")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl2 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl3(pub u32);
impl Capctrl3 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl3 {
    #[inline(always)]
    fn default() -> Capctrl3 {
        Capctrl3(0)
    }
}
impl core::fmt::Debug for Capctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl3")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl3 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl4(pub u32);
impl Capctrl4 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl4 {
    #[inline(always)]
    fn default() -> Capctrl4 {
        Capctrl4(0)
    }
}
impl core::fmt::Debug for Capctrl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl4")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl4 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl5(pub u32);
impl Capctrl5 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl5 {
    #[inline(always)]
    fn default() -> Capctrl5 {
        Capctrl5(0)
    }
}
impl core::fmt::Debug for Capctrl5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl5")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl5 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl6(pub u32);
impl Capctrl6 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl6 {
    #[inline(always)]
    fn default() -> Capctrl6 {
        Capctrl6(0)
    }
}
impl core::fmt::Debug for Capctrl6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl6")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl6 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl7(pub u32);
impl Capctrl7 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl7 {
    #[inline(always)]
    fn default() -> Capctrl7 {
        Capctrl7(0)
    }
}
impl core::fmt::Debug for Capctrl7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl7")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl7 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl8(pub u32);
impl Capctrl8 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl8 {
    #[inline(always)]
    fn default() -> Capctrl8 {
        Capctrl8(0)
    }
}
impl core::fmt::Debug for Capctrl8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl8")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl8 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl9(pub u32);
impl Capctrl9 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl9 {
    #[inline(always)]
    fn default() -> Capctrl9 {
        Capctrl9(0)
    }
}
impl core::fmt::Debug for Capctrl9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl9")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl9 {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT conflict interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conen(pub u32);
impl Conen {
    #[doc = "The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn ncen(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_ncen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Conen {
    #[inline(always)]
    fn default() -> Conen {
        Conen(0)
    }
}
impl core::fmt::Debug for Conen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conen").field("ncen", &self.ncen()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Conen {{ ncen: {=u16:?} }}", self.ncen())
    }
}
#[doc = "SCT configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "SCT operation"]
    #[must_use]
    #[inline(always)]
    pub const fn unify(&self) -> super::vals::Unify {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Unify::from_bits(val as u8)
    }
    #[doc = "SCT operation"]
    #[inline(always)]
    pub const fn set_unify(&mut self, val: super::vals::Unify) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SCT clock mode"]
    #[must_use]
    #[inline(always)]
    pub const fn clkmode(&self) -> super::vals::Clkmode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Clkmode::from_bits(val as u8)
    }
    #[doc = "SCT clock mode"]
    #[inline(always)]
    pub const fn set_clkmode(&mut self, val: super::vals::Clkmode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[must_use]
    #[inline(always)]
    pub const fn cksel(&self) -> super::vals::Cksel {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Cksel::from_bits(val as u8)
    }
    #[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub const fn set_cksel(&mut self, val: super::vals::Cksel) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_l(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_noreload_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_h(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_noreload_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[must_use]
    #[inline(always)]
    pub const fn insync(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub const fn set_insync(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_l(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_autolimit_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_autolimit_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("unify", &self.unify())
            .field("clkmode", &self.clkmode())
            .field("cksel", &self.cksel())
            .field("noreload_l", &self.noreload_l())
            .field("noreload_h", &self.noreload_h())
            .field("insync", &self.insync())
            .field("autolimit_l", &self.autolimit_l())
            .field("autolimit_h", &self.autolimit_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ unify: {:?}, clkmode: {:?}, cksel: {:?}, noreload_l: {=bool:?}, noreload_h: {=bool:?}, insync: {=u8:?}, autolimit_l: {=bool:?}, autolimit_h: {=bool:?} }}",
            self.unify(),
            self.clkmode(),
            self.cksel(),
            self.noreload_l(),
            self.noreload_h(),
            self.insync(),
            self.autolimit_l(),
            self.autolimit_h()
        )
    }
}
#[doc = "SCT conflict flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conflag(pub u32);
impl Conflag {
    #[doc = "Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_ncflag(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[must_use]
    #[inline(always)]
    pub const fn buserrl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub const fn set_buserrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[must_use]
    #[inline(always)]
    pub const fn buserrh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub const fn set_buserrh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Conflag {
    #[inline(always)]
    fn default() -> Conflag {
        Conflag(0)
    }
}
impl core::fmt::Debug for Conflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conflag")
            .field("ncflag", &self.ncflag())
            .field("buserrl", &self.buserrl())
            .field("buserrh", &self.buserrh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Conflag {{ ncflag: {=u16:?}, buserrl: {=bool:?}, buserrh: {=bool:?} }}",
            self.ncflag(),
            self.buserrl(),
            self.buserrh()
        )
    }
}
#[doc = "SCT counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub const fn set_ctr_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub const fn set_ctr_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
impl core::fmt::Debug for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Count")
            .field("ctr_l", &self.ctr_l())
            .field("ctr_h", &self.ctr_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Count {{ ctr_l: {=u16:?}, ctr_h: {=u16:?} }}",
            self.ctr_l(),
            self.ctr_h()
        )
    }
}
#[doc = "SCT control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn down_l(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_down_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub const fn set_stop_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn halt_l(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub const fn set_halt_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_l(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub const fn set_clrctr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "L or unified counter direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_l(&self) -> super::vals::BidirL {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BidirL::from_bits(val as u8)
    }
    #[doc = "L or unified counter direction select"]
    #[inline(always)]
    pub const fn set_bidir_l(&mut self, val: super::vals::BidirL) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_l(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub const fn set_pre_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn down_h(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_down_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_h(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub const fn set_stop_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn halt_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub const fn set_halt_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_h(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub const fn set_clrctr_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_h(&self) -> super::vals::BidirH {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::BidirH::from_bits(val as u8)
    }
    #[doc = "Direction select"]
    #[inline(always)]
    pub const fn set_bidir_h(&mut self, val: super::vals::BidirH) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_h(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub const fn set_pre_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
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
            .field("down_l", &self.down_l())
            .field("stop_l", &self.stop_l())
            .field("halt_l", &self.halt_l())
            .field("clrctr_l", &self.clrctr_l())
            .field("bidir_l", &self.bidir_l())
            .field("pre_l", &self.pre_l())
            .field("down_h", &self.down_h())
            .field("stop_h", &self.stop_h())
            .field("halt_h", &self.halt_h())
            .field("clrctr_h", &self.clrctr_h())
            .field("bidir_h", &self.bidir_h())
            .field("pre_h", &self.pre_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ down_l: {=bool:?}, stop_l: {=bool:?}, halt_l: {=bool:?}, clrctr_l: {=bool:?}, bidir_l: {:?}, pre_l: {=u8:?}, down_h: {=bool:?}, stop_h: {=bool:?}, halt_h: {=bool:?}, clrctr_h: {=bool:?}, bidir_h: {:?}, pre_h: {=u8:?} }}",
            self.down_l(),
            self.stop_l(),
            self.halt_l(),
            self.clrctr_l(),
            self.bidir_l(),
            self.pre_l(),
            self.down_h(),
            self.stop_h(),
            self.halt_h(),
            self.clrctr_h(),
            self.bidir_h(),
            self.pre_h()
        )
    }
}
#[doc = "SCT DMA request 0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq0(pub u32);
impl Dmareq0 {
    #[doc = "If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_dev_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[must_use]
    #[inline(always)]
    pub const fn drl0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub const fn set_drl0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[must_use]
    #[inline(always)]
    pub const fn drq0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub const fn set_drq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmareq0 {
    #[inline(always)]
    fn default() -> Dmareq0 {
        Dmareq0(0)
    }
}
impl core::fmt::Debug for Dmareq0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq0")
            .field("dev_0", &self.dev_0())
            .field("drl0", &self.drl0())
            .field("drq0", &self.drq0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmareq0 {{ dev_0: {=u16:?}, drl0: {=bool:?}, drq0: {=bool:?} }}",
            self.dev_0(),
            self.drl0(),
            self.drq0()
        )
    }
}
#[doc = "SCT DMA request 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq1(pub u32);
impl Dmareq1 {
    #[doc = "If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_dev_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[must_use]
    #[inline(always)]
    pub const fn drl1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub const fn set_drl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[must_use]
    #[inline(always)]
    pub const fn drq1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub const fn set_drq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmareq1 {
    #[inline(always)]
    fn default() -> Dmareq1 {
        Dmareq1(0)
    }
}
impl core::fmt::Debug for Dmareq1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq1")
            .field("dev_1", &self.dev_1())
            .field("drl1", &self.drl1())
            .field("drq1", &self.drq1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmareq1 {{ dev_1: {=u16:?}, drl1: {=bool:?}, drq1: {=bool:?} }}",
            self.dev_1(),
            self.drl1(),
            self.drq1()
        )
    }
}
#[doc = "SCT event control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvCtrl(pub u32);
impl EvCtrl {
    #[doc = "Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[must_use]
    #[inline(always)]
    pub const fn matchsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub const fn set_matchsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn hevent(&self) -> super::vals::Hevent {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hevent::from_bits(val as u8)
    }
    #[doc = "Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub const fn set_hevent(&mut self, val: super::vals::Hevent) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Input/output select"]
    #[must_use]
    #[inline(always)]
    pub const fn outsel(&self) -> super::vals::Outsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Outsel::from_bits(val as u8)
    }
    #[doc = "Input/output select"]
    #[inline(always)]
    pub const fn set_outsel(&mut self, val: super::vals::Outsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[must_use]
    #[inline(always)]
    pub const fn iosel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub const fn set_iosel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[must_use]
    #[inline(always)]
    pub const fn iocond(&self) -> super::vals::Iocond {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Iocond::from_bits(val as u8)
    }
    #[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub const fn set_iocond(&mut self, val: super::vals::Iocond) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects how the specified match and I/O condition are used and combined."]
    #[must_use]
    #[inline(always)]
    pub const fn combmode(&self) -> super::vals::Combmode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Combmode::from_bits(val as u8)
    }
    #[doc = "Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub const fn set_combmode(&mut self, val: super::vals::Combmode) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[must_use]
    #[inline(always)]
    pub const fn stateld(&self) -> super::vals::Stateld {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Stateld::from_bits(val as u8)
    }
    #[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub const fn set_stateld(&mut self, val: super::vals::Stateld) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[must_use]
    #[inline(always)]
    pub const fn statev(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub const fn set_statev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[must_use]
    #[inline(always)]
    pub const fn matchmem(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub const fn set_matchmem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Direction {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Direction::from_bits(val as u8)
    }
    #[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::Direction) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for EvCtrl {
    #[inline(always)]
    fn default() -> EvCtrl {
        EvCtrl(0)
    }
}
impl core::fmt::Debug for EvCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvCtrl")
            .field("matchsel", &self.matchsel())
            .field("hevent", &self.hevent())
            .field("outsel", &self.outsel())
            .field("iosel", &self.iosel())
            .field("iocond", &self.iocond())
            .field("combmode", &self.combmode())
            .field("stateld", &self.stateld())
            .field("statev", &self.statev())
            .field("matchmem", &self.matchmem())
            .field("direction", &self.direction())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvCtrl {{ matchsel: {=u8:?}, hevent: {:?}, outsel: {:?}, iosel: {=u8:?}, iocond: {:?}, combmode: {:?}, stateld: {:?}, statev: {=u8:?}, matchmem: {=bool:?}, direction: {:?} }}",
            self.matchsel(),
            self.hevent(),
            self.outsel(),
            self.iosel(),
            self.iocond(),
            self.combmode(),
            self.stateld(),
            self.statev(),
            self.matchmem(),
            self.direction()
        )
    }
}
#[doc = "SCT event state register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvState(pub u32);
impl EvState {
    #[doc = "If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn statemskn(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub const fn set_statemskn(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EvState {
    #[inline(always)]
    fn default() -> EvState {
        EvState(0)
    }
}
impl core::fmt::Debug for EvState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvState")
            .field("statemskn", &self.statemskn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EvState {{ statemskn: {=u16:?} }}", self.statemskn())
    }
}
#[doc = "SCT event interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Even(pub u32);
impl Even {
    #[doc = "The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn ien(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_ien(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Even {
    #[inline(always)]
    fn default() -> Even {
        Even(0)
    }
}
impl core::fmt::Debug for Even {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Even").field("ien", &self.ien()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Even {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Even {{ ien: {=u16:?} }}", self.ien())
    }
}
#[doc = "SCT event flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evflag(pub u32);
impl Evflag {
    #[doc = "Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn flag(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_flag(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Evflag {
    #[inline(always)]
    fn default() -> Evflag {
        Evflag(0)
    }
}
impl core::fmt::Debug for Evflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Evflag")
            .field("flag", &self.flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Evflag {{ flag: {=u16:?} }}", self.flag())
    }
}
#[doc = "SCT halt event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halt(pub u32);
impl Halt {
    #[doc = "If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_haltmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_haltmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Halt {
    #[inline(always)]
    fn default() -> Halt {
        Halt(0)
    }
}
impl core::fmt::Debug for Halt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Halt")
            .field("haltmsk_l", &self.haltmsk_l())
            .field("haltmsk_h", &self.haltmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Halt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Halt {{ haltmsk_l: {=u16:?}, haltmsk_h: {=u16:?} }}",
            self.haltmsk_l(),
            self.haltmsk_h()
        )
    }
}
#[doc = "SCT input register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "Input 0 state. Input 0 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Input 1 state. Input 1 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Input 1 state. Input 1 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input 2 state. Input 2 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input 2 state. Input 2 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input 3 state. Input 3 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input 3 state. Input 3 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input 4 state. Input 4 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input 4 state. Input 4 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Input 5 state. Input 5 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Input 5 state. Input 5 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input 6 state. Input 6 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input 6 state. Input 6 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input 7 state. Input 7 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input 7 state. Input 7 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Input 8 state. Input 8 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Input 8 state. Input 8 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Input 9 state. Input 9 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input 9 state. Input 9 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Input 10 state. Input 10 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input 10 state. Input 10 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 11 state. Input 11 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input 11 state. Input 11 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Input 12 state. Input 12 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 12 state. Input 12 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Input 13 state. Input 13 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Input 13 state. Input 13 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Input 14 state. Input 14 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Input 14 state. Input 14 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Input 15 state. Input 15 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Input 15 state. Input 15 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Input {
    #[inline(always)]
    fn default() -> Input {
        Input(0)
    }
}
impl core::fmt::Debug for Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Input")
            .field("ain0", &self.ain0())
            .field("ain1", &self.ain1())
            .field("ain2", &self.ain2())
            .field("ain3", &self.ain3())
            .field("ain4", &self.ain4())
            .field("ain5", &self.ain5())
            .field("ain6", &self.ain6())
            .field("ain7", &self.ain7())
            .field("ain8", &self.ain8())
            .field("ain9", &self.ain9())
            .field("ain10", &self.ain10())
            .field("ain11", &self.ain11())
            .field("ain12", &self.ain12())
            .field("ain13", &self.ain13())
            .field("ain14", &self.ain14())
            .field("ain15", &self.ain15())
            .field("sin0", &self.sin0())
            .field("sin1", &self.sin1())
            .field("sin2", &self.sin2())
            .field("sin3", &self.sin3())
            .field("sin4", &self.sin4())
            .field("sin5", &self.sin5())
            .field("sin6", &self.sin6())
            .field("sin7", &self.sin7())
            .field("sin8", &self.sin8())
            .field("sin9", &self.sin9())
            .field("sin10", &self.sin10())
            .field("sin11", &self.sin11())
            .field("sin12", &self.sin12())
            .field("sin13", &self.sin13())
            .field("sin14", &self.sin14())
            .field("sin15", &self.sin15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Input {{ ain0: {=bool:?}, ain1: {=bool:?}, ain2: {=bool:?}, ain3: {=bool:?}, ain4: {=bool:?}, ain5: {=bool:?}, ain6: {=bool:?}, ain7: {=bool:?}, ain8: {=bool:?}, ain9: {=bool:?}, ain10: {=bool:?}, ain11: {=bool:?}, ain12: {=bool:?}, ain13: {=bool:?}, ain14: {=bool:?}, ain15: {=bool:?}, sin0: {=bool:?}, sin1: {=bool:?}, sin2: {=bool:?}, sin3: {=bool:?}, sin4: {=bool:?}, sin5: {=bool:?}, sin6: {=bool:?}, sin7: {=bool:?}, sin8: {=bool:?}, sin9: {=bool:?}, sin10: {=bool:?}, sin11: {=bool:?}, sin12: {=bool:?}, sin13: {=bool:?}, sin14: {=bool:?}, sin15: {=bool:?} }}",
            self.ain0(),
            self.ain1(),
            self.ain2(),
            self.ain3(),
            self.ain4(),
            self.ain5(),
            self.ain6(),
            self.ain7(),
            self.ain8(),
            self.ain9(),
            self.ain10(),
            self.ain11(),
            self.ain12(),
            self.ain13(),
            self.ain14(),
            self.ain15(),
            self.sin0(),
            self.sin1(),
            self.sin2(),
            self.sin3(),
            self.sin4(),
            self.sin5(),
            self.sin6(),
            self.sin7(),
            self.sin8(),
            self.sin9(),
            self.sin10(),
            self.sin11(),
            self.sin12(),
            self.sin13(),
            self.sin14(),
            self.sin15()
        )
    }
}
#[doc = "SCT limit event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Limit(pub u32);
impl Limit {
    #[doc = "If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_limmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_limmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Limit {
    #[inline(always)]
    fn default() -> Limit {
        Limit(0)
    }
}
impl core::fmt::Debug for Limit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Limit")
            .field("limmsk_l", &self.limmsk_l())
            .field("limmsk_h", &self.limmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Limit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Limit {{ limmsk_l: {=u16:?}, limmsk_h: {=u16:?} }}",
            self.limmsk_l(),
            self.limmsk_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match0(pub u32);
impl Match0 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match0 {
    #[inline(always)]
    fn default() -> Match0 {
        Match0(0)
    }
}
impl core::fmt::Debug for Match0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match0")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match0 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match1(pub u32);
impl Match1 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match1 {
    #[inline(always)]
    fn default() -> Match1 {
        Match1(0)
    }
}
impl core::fmt::Debug for Match1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match1")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match1 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match10(pub u32);
impl Match10 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match10 {
    #[inline(always)]
    fn default() -> Match10 {
        Match10(0)
    }
}
impl core::fmt::Debug for Match10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match10")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match10 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match11(pub u32);
impl Match11 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match11 {
    #[inline(always)]
    fn default() -> Match11 {
        Match11(0)
    }
}
impl core::fmt::Debug for Match11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match11")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match11 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match12(pub u32);
impl Match12 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match12 {
    #[inline(always)]
    fn default() -> Match12 {
        Match12(0)
    }
}
impl core::fmt::Debug for Match12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match12")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match12 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match13(pub u32);
impl Match13 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match13 {
    #[inline(always)]
    fn default() -> Match13 {
        Match13(0)
    }
}
impl core::fmt::Debug for Match13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match13")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match13 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match14(pub u32);
impl Match14 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match14 {
    #[inline(always)]
    fn default() -> Match14 {
        Match14(0)
    }
}
impl core::fmt::Debug for Match14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match14")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match14 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match15(pub u32);
impl Match15 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match15 {
    #[inline(always)]
    fn default() -> Match15 {
        Match15(0)
    }
}
impl core::fmt::Debug for Match15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match15")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match15 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match2(pub u32);
impl Match2 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match2 {
    #[inline(always)]
    fn default() -> Match2 {
        Match2(0)
    }
}
impl core::fmt::Debug for Match2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match2")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match2 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match3(pub u32);
impl Match3 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match3 {
    #[inline(always)]
    fn default() -> Match3 {
        Match3(0)
    }
}
impl core::fmt::Debug for Match3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match3")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match3 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match4(pub u32);
impl Match4 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match4 {
    #[inline(always)]
    fn default() -> Match4 {
        Match4(0)
    }
}
impl core::fmt::Debug for Match4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match4")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match4 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match5(pub u32);
impl Match5 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match5 {
    #[inline(always)]
    fn default() -> Match5 {
        Match5(0)
    }
}
impl core::fmt::Debug for Match5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match5")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match5 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match6(pub u32);
impl Match6 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match6 {
    #[inline(always)]
    fn default() -> Match6 {
        Match6(0)
    }
}
impl core::fmt::Debug for Match6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match6")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match6 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match7(pub u32);
impl Match7 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match7 {
    #[inline(always)]
    fn default() -> Match7 {
        Match7(0)
    }
}
impl core::fmt::Debug for Match7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match7")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match7 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match8(pub u32);
impl Match8 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match8 {
    #[inline(always)]
    fn default() -> Match8 {
        Match8(0)
    }
}
impl core::fmt::Debug for Match8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match8")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match8 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match9(pub u32);
impl Match9 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match9 {
    #[inline(always)]
    fn default() -> Match9 {
        Match9(0)
    }
}
impl core::fmt::Debug for Match9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match9")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match9 {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel0(pub u32);
impl Matchrel0 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel0 {
    #[inline(always)]
    fn default() -> Matchrel0 {
        Matchrel0(0)
    }
}
impl core::fmt::Debug for Matchrel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel0")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel0 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel1(pub u32);
impl Matchrel1 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel1 {
    #[inline(always)]
    fn default() -> Matchrel1 {
        Matchrel1(0)
    }
}
impl core::fmt::Debug for Matchrel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel1")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel1 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel10(pub u32);
impl Matchrel10 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel10 {
    #[inline(always)]
    fn default() -> Matchrel10 {
        Matchrel10(0)
    }
}
impl core::fmt::Debug for Matchrel10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel10")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel10 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel11(pub u32);
impl Matchrel11 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel11 {
    #[inline(always)]
    fn default() -> Matchrel11 {
        Matchrel11(0)
    }
}
impl core::fmt::Debug for Matchrel11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel11")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel11 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel12(pub u32);
impl Matchrel12 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel12 {
    #[inline(always)]
    fn default() -> Matchrel12 {
        Matchrel12(0)
    }
}
impl core::fmt::Debug for Matchrel12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel12")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel12 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel13(pub u32);
impl Matchrel13 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel13 {
    #[inline(always)]
    fn default() -> Matchrel13 {
        Matchrel13(0)
    }
}
impl core::fmt::Debug for Matchrel13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel13")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel13 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel14(pub u32);
impl Matchrel14 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel14 {
    #[inline(always)]
    fn default() -> Matchrel14 {
        Matchrel14(0)
    }
}
impl core::fmt::Debug for Matchrel14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel14")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel14 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel15(pub u32);
impl Matchrel15 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel15 {
    #[inline(always)]
    fn default() -> Matchrel15 {
        Matchrel15(0)
    }
}
impl core::fmt::Debug for Matchrel15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel15")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel15 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel2(pub u32);
impl Matchrel2 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel2 {
    #[inline(always)]
    fn default() -> Matchrel2 {
        Matchrel2(0)
    }
}
impl core::fmt::Debug for Matchrel2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel2")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel2 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel3(pub u32);
impl Matchrel3 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel3 {
    #[inline(always)]
    fn default() -> Matchrel3 {
        Matchrel3(0)
    }
}
impl core::fmt::Debug for Matchrel3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel3")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel3 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel4(pub u32);
impl Matchrel4 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel4 {
    #[inline(always)]
    fn default() -> Matchrel4 {
        Matchrel4(0)
    }
}
impl core::fmt::Debug for Matchrel4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel4")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel4 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel5(pub u32);
impl Matchrel5 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel5 {
    #[inline(always)]
    fn default() -> Matchrel5 {
        Matchrel5(0)
    }
}
impl core::fmt::Debug for Matchrel5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel5")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel5 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel6(pub u32);
impl Matchrel6 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel6 {
    #[inline(always)]
    fn default() -> Matchrel6 {
        Matchrel6(0)
    }
}
impl core::fmt::Debug for Matchrel6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel6")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel6 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel7(pub u32);
impl Matchrel7 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel7 {
    #[inline(always)]
    fn default() -> Matchrel7 {
        Matchrel7(0)
    }
}
impl core::fmt::Debug for Matchrel7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel7")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel7 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel8(pub u32);
impl Matchrel8 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel8 {
    #[inline(always)]
    fn default() -> Matchrel8 {
        Matchrel8(0)
    }
}
impl core::fmt::Debug for Matchrel8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel8")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel8 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel9(pub u32);
impl Matchrel9 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel9 {
    #[inline(always)]
    fn default() -> Matchrel9 {
        Matchrel9(0)
    }
}
impl core::fmt::Debug for Matchrel9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel9")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel9 {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT output 0 clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClr(pub u32);
impl OutClr {
    #[doc = "A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OutClr {
    #[inline(always)]
    fn default() -> OutClr {
        OutClr(0)
    }
}
impl core::fmt::Debug for OutClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutClr").field("clr", &self.clr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutClr {{ clr: {=u16:?} }}", self.clr())
    }
}
#[doc = "SCT output 0 set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet(pub u32);
impl OutSet {
    #[doc = "A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        OutSet(0)
    }
}
impl core::fmt::Debug for OutSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSet").field("set", &self.set()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutSet {{ set: {=u16:?} }}", self.set())
    }
}
#[doc = "SCT output register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Output(pub u32);
impl Output {
    #[doc = "Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_out(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Output {
    #[inline(always)]
    fn default() -> Output {
        Output(0)
    }
}
impl core::fmt::Debug for Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Output").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Output {{ out: {=u16:?} }}", self.out())
    }
}
#[doc = "SCT output counter direction control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputdirctrl(pub u32);
impl Outputdirctrl {
    #[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr0(&self) -> super::vals::Setclr0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Setclr0::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr0(&mut self, val: super::vals::Setclr0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr1(&self) -> super::vals::Setclr1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Setclr1::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr1(&mut self, val: super::vals::Setclr1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr2(&self) -> super::vals::Setclr2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Setclr2::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr2(&mut self, val: super::vals::Setclr2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr3(&self) -> super::vals::Setclr3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Setclr3::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr3(&mut self, val: super::vals::Setclr3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr4(&self) -> super::vals::Setclr4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Setclr4::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr4(&mut self, val: super::vals::Setclr4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr5(&self) -> super::vals::Setclr5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Setclr5::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr5(&mut self, val: super::vals::Setclr5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr6(&self) -> super::vals::Setclr6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Setclr6::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr6(&mut self, val: super::vals::Setclr6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr7(&self) -> super::vals::Setclr7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Setclr7::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr7(&mut self, val: super::vals::Setclr7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr8(&self) -> super::vals::Setclr8 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Setclr8::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr8(&mut self, val: super::vals::Setclr8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr9(&self) -> super::vals::Setclr9 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Setclr9::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr9(&mut self, val: super::vals::Setclr9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr10(&self) -> super::vals::Setclr10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Setclr10::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr10(&mut self, val: super::vals::Setclr10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr11(&self) -> super::vals::Setclr11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Setclr11::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr11(&mut self, val: super::vals::Setclr11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr12(&self) -> super::vals::Setclr12 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Setclr12::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr12(&mut self, val: super::vals::Setclr12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr13(&self) -> super::vals::Setclr13 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Setclr13::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr13(&mut self, val: super::vals::Setclr13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr14(&self) -> super::vals::Setclr14 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Setclr14::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr14(&mut self, val: super::vals::Setclr14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr15(&self) -> super::vals::Setclr15 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Setclr15::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr15(&mut self, val: super::vals::Setclr15) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Outputdirctrl {
    #[inline(always)]
    fn default() -> Outputdirctrl {
        Outputdirctrl(0)
    }
}
impl core::fmt::Debug for Outputdirctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outputdirctrl")
            .field("setclr0", &self.setclr0())
            .field("setclr1", &self.setclr1())
            .field("setclr2", &self.setclr2())
            .field("setclr3", &self.setclr3())
            .field("setclr4", &self.setclr4())
            .field("setclr5", &self.setclr5())
            .field("setclr6", &self.setclr6())
            .field("setclr7", &self.setclr7())
            .field("setclr8", &self.setclr8())
            .field("setclr9", &self.setclr9())
            .field("setclr10", &self.setclr10())
            .field("setclr11", &self.setclr11())
            .field("setclr12", &self.setclr12())
            .field("setclr13", &self.setclr13())
            .field("setclr14", &self.setclr14())
            .field("setclr15", &self.setclr15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outputdirctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outputdirctrl {{ setclr0: {:?}, setclr1: {:?}, setclr2: {:?}, setclr3: {:?}, setclr4: {:?}, setclr5: {:?}, setclr6: {:?}, setclr7: {:?}, setclr8: {:?}, setclr9: {:?}, setclr10: {:?}, setclr11: {:?}, setclr12: {:?}, setclr13: {:?}, setclr14: {:?}, setclr15: {:?} }}",
            self.setclr0(),
            self.setclr1(),
            self.setclr2(),
            self.setclr3(),
            self.setclr4(),
            self.setclr5(),
            self.setclr6(),
            self.setclr7(),
            self.setclr8(),
            self.setclr9(),
            self.setclr10(),
            self.setclr11(),
            self.setclr12(),
            self.setclr13(),
            self.setclr14(),
            self.setclr15()
        )
    }
}
#[doc = "SCT match/capture mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regmode(pub u32);
impl Regmode {
    #[doc = "Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub const fn set_regmod_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub const fn set_regmod_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Regmode {
    #[inline(always)]
    fn default() -> Regmode {
        Regmode(0)
    }
}
impl core::fmt::Debug for Regmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Regmode")
            .field("regmod_l", &self.regmod_l())
            .field("regmod_h", &self.regmod_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Regmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Regmode {{ regmod_l: {=u16:?}, regmod_h: {=u16:?} }}",
            self.regmod_l(),
            self.regmod_h()
        )
    }
}
#[doc = "SCT conflict resolution register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res(pub u32);
impl Res {
    #[doc = "Effect of simultaneous set and clear on output 0."]
    #[must_use]
    #[inline(always)]
    pub const fn o0res(&self) -> super::vals::O0res {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::O0res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub const fn set_o0res(&mut self, val: super::vals::O0res) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 1."]
    #[must_use]
    #[inline(always)]
    pub const fn o1res(&self) -> super::vals::O1res {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::O1res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub const fn set_o1res(&mut self, val: super::vals::O1res) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 2."]
    #[must_use]
    #[inline(always)]
    pub const fn o2res(&self) -> super::vals::O2res {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::O2res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub const fn set_o2res(&mut self, val: super::vals::O2res) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 3."]
    #[must_use]
    #[inline(always)]
    pub const fn o3res(&self) -> super::vals::O3res {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::O3res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub const fn set_o3res(&mut self, val: super::vals::O3res) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 4."]
    #[must_use]
    #[inline(always)]
    pub const fn o4res(&self) -> super::vals::O4res {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::O4res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub const fn set_o4res(&mut self, val: super::vals::O4res) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 5."]
    #[must_use]
    #[inline(always)]
    pub const fn o5res(&self) -> super::vals::O5res {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::O5res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub const fn set_o5res(&mut self, val: super::vals::O5res) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 6."]
    #[must_use]
    #[inline(always)]
    pub const fn o6res(&self) -> super::vals::O6res {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::O6res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub const fn set_o6res(&mut self, val: super::vals::O6res) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 7."]
    #[must_use]
    #[inline(always)]
    pub const fn o7res(&self) -> super::vals::O7res {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::O7res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub const fn set_o7res(&mut self, val: super::vals::O7res) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 8."]
    #[must_use]
    #[inline(always)]
    pub const fn o8res(&self) -> super::vals::O8res {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::O8res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub const fn set_o8res(&mut self, val: super::vals::O8res) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 9."]
    #[must_use]
    #[inline(always)]
    pub const fn o9res(&self) -> super::vals::O9res {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::O9res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub const fn set_o9res(&mut self, val: super::vals::O9res) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 10."]
    #[must_use]
    #[inline(always)]
    pub const fn o10res(&self) -> super::vals::O10res {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::O10res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub const fn set_o10res(&mut self, val: super::vals::O10res) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 11."]
    #[must_use]
    #[inline(always)]
    pub const fn o11res(&self) -> super::vals::O11res {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::O11res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub const fn set_o11res(&mut self, val: super::vals::O11res) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 12."]
    #[must_use]
    #[inline(always)]
    pub const fn o12res(&self) -> super::vals::O12res {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::O12res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub const fn set_o12res(&mut self, val: super::vals::O12res) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 13."]
    #[must_use]
    #[inline(always)]
    pub const fn o13res(&self) -> super::vals::O13res {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::O13res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub const fn set_o13res(&mut self, val: super::vals::O13res) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 14."]
    #[must_use]
    #[inline(always)]
    pub const fn o14res(&self) -> super::vals::O14res {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::O14res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub const fn set_o14res(&mut self, val: super::vals::O14res) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 15."]
    #[must_use]
    #[inline(always)]
    pub const fn o15res(&self) -> super::vals::O15res {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::O15res::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub const fn set_o15res(&mut self, val: super::vals::O15res) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Res {
    #[inline(always)]
    fn default() -> Res {
        Res(0)
    }
}
impl core::fmt::Debug for Res {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res")
            .field("o0res", &self.o0res())
            .field("o1res", &self.o1res())
            .field("o2res", &self.o2res())
            .field("o3res", &self.o3res())
            .field("o4res", &self.o4res())
            .field("o5res", &self.o5res())
            .field("o6res", &self.o6res())
            .field("o7res", &self.o7res())
            .field("o8res", &self.o8res())
            .field("o9res", &self.o9res())
            .field("o10res", &self.o10res())
            .field("o11res", &self.o11res())
            .field("o12res", &self.o12res())
            .field("o13res", &self.o13res())
            .field("o14res", &self.o14res())
            .field("o15res", &self.o15res())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Res {{ o0res: {:?}, o1res: {:?}, o2res: {:?}, o3res: {:?}, o4res: {:?}, o5res: {:?}, o6res: {:?}, o7res: {:?}, o8res: {:?}, o9res: {:?}, o10res: {:?}, o11res: {:?}, o12res: {:?}, o13res: {:?}, o14res: {:?}, o15res: {:?} }}",
            self.o0res(),
            self.o1res(),
            self.o2res(),
            self.o3res(),
            self.o4res(),
            self.o5res(),
            self.o6res(),
            self.o7res(),
            self.o8res(),
            self.o9res(),
            self.o10res(),
            self.o11res(),
            self.o12res(),
            self.o13res(),
            self.o14res(),
            self.o15res()
        )
    }
}
#[doc = "SCT start event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_startmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_startmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Start {
    #[inline(always)]
    fn default() -> Start {
        Start(0)
    }
}
impl core::fmt::Debug for Start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Start")
            .field("startmsk_l", &self.startmsk_l())
            .field("startmsk_h", &self.startmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Start {{ startmsk_l: {=u16:?}, startmsk_h: {=u16:?} }}",
            self.startmsk_l(),
            self.startmsk_h()
        )
    }
}
#[doc = "SCT state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "State variable."]
    #[must_use]
    #[inline(always)]
    pub const fn state_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "State variable."]
    #[inline(always)]
    pub const fn set_state_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "State variable."]
    #[must_use]
    #[inline(always)]
    pub const fn state_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "State variable."]
    #[inline(always)]
    pub const fn set_state_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("State")
            .field("state_l", &self.state_l())
            .field("state_h", &self.state_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for State {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "State {{ state_l: {=u8:?}, state_h: {=u8:?} }}",
            self.state_l(),
            self.state_h()
        )
    }
}
#[doc = "SCT stop event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_stopmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_stopmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Stop {
    #[inline(always)]
    fn default() -> Stop {
        Stop(0)
    }
}
impl core::fmt::Debug for Stop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stop")
            .field("stopmsk_l", &self.stopmsk_l())
            .field("stopmsk_h", &self.stopmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stop {{ stopmsk_l: {=u16:?}, stopmsk_h: {=u16:?} }}",
            self.stopmsk_l(),
            self.stopmsk_h()
        )
    }
}
