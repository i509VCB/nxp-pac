#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP0(pub u32);
impl CAP0 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP0 {
    #[inline(always)]
    fn default() -> CAP0 {
        CAP0(0)
    }
}
impl core::fmt::Debug for CAP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP0")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP0 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP1(pub u32);
impl CAP1 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP1 {
    #[inline(always)]
    fn default() -> CAP1 {
        CAP1(0)
    }
}
impl core::fmt::Debug for CAP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP1")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP1 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP10(pub u32);
impl CAP10 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP10 {
    #[inline(always)]
    fn default() -> CAP10 {
        CAP10(0)
    }
}
impl core::fmt::Debug for CAP10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP10")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP10 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP11(pub u32);
impl CAP11 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP11 {
    #[inline(always)]
    fn default() -> CAP11 {
        CAP11(0)
    }
}
impl core::fmt::Debug for CAP11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP11")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP11 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP12(pub u32);
impl CAP12 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP12 {
    #[inline(always)]
    fn default() -> CAP12 {
        CAP12(0)
    }
}
impl core::fmt::Debug for CAP12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP12")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP12 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP13(pub u32);
impl CAP13 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP13 {
    #[inline(always)]
    fn default() -> CAP13 {
        CAP13(0)
    }
}
impl core::fmt::Debug for CAP13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP13")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP13 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP14(pub u32);
impl CAP14 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP14 {
    #[inline(always)]
    fn default() -> CAP14 {
        CAP14(0)
    }
}
impl core::fmt::Debug for CAP14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP14")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP14 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP15(pub u32);
impl CAP15 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP15 {
    #[inline(always)]
    fn default() -> CAP15 {
        CAP15(0)
    }
}
impl core::fmt::Debug for CAP15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP15")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP15 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP2(pub u32);
impl CAP2 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP2 {
    #[inline(always)]
    fn default() -> CAP2 {
        CAP2(0)
    }
}
impl core::fmt::Debug for CAP2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP2")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP2 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP3(pub u32);
impl CAP3 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP3 {
    #[inline(always)]
    fn default() -> CAP3 {
        CAP3(0)
    }
}
impl core::fmt::Debug for CAP3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP3")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP3 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP4(pub u32);
impl CAP4 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP4 {
    #[inline(always)]
    fn default() -> CAP4 {
        CAP4(0)
    }
}
impl core::fmt::Debug for CAP4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP4")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP4 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP5(pub u32);
impl CAP5 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP5 {
    #[inline(always)]
    fn default() -> CAP5 {
        CAP5(0)
    }
}
impl core::fmt::Debug for CAP5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP5")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP5 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP6(pub u32);
impl CAP6 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP6 {
    #[inline(always)]
    fn default() -> CAP6 {
        CAP6(0)
    }
}
impl core::fmt::Debug for CAP6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP6")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP6 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP7(pub u32);
impl CAP7 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP7 {
    #[inline(always)]
    fn default() -> CAP7 {
        CAP7(0)
    }
}
impl core::fmt::Debug for CAP7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP7")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP7 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP8(pub u32);
impl CAP8 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP8 {
    #[inline(always)]
    fn default() -> CAP8 {
        CAP8(0)
    }
}
impl core::fmt::Debug for CAP8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP8")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP8 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture register of capture channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP9(pub u32);
impl CAP9 {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_CAPn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP9 {
    #[inline(always)]
    fn default() -> CAP9 {
        CAP9(0)
    }
}
impl core::fmt::Debug for CAP9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP9")
            .field("CAPn_L", &self.CAPn_L())
            .field("CAPn_H", &self.CAPn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP9 {{ CAPn_L: {=u16:?}, CAPn_H: {=u16:?} }}",
            self.CAPn_L(),
            self.CAPn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL0(pub u32);
impl CAPCTRL0 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL0 {
    #[inline(always)]
    fn default() -> CAPCTRL0 {
        CAPCTRL0(0)
    }
}
impl core::fmt::Debug for CAPCTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL0")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL0 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL1(pub u32);
impl CAPCTRL1 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL1 {
    #[inline(always)]
    fn default() -> CAPCTRL1 {
        CAPCTRL1(0)
    }
}
impl core::fmt::Debug for CAPCTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL1")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL1 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL10(pub u32);
impl CAPCTRL10 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL10 {
    #[inline(always)]
    fn default() -> CAPCTRL10 {
        CAPCTRL10(0)
    }
}
impl core::fmt::Debug for CAPCTRL10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL10")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL10 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL11(pub u32);
impl CAPCTRL11 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL11 {
    #[inline(always)]
    fn default() -> CAPCTRL11 {
        CAPCTRL11(0)
    }
}
impl core::fmt::Debug for CAPCTRL11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL11")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL11 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL12(pub u32);
impl CAPCTRL12 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL12 {
    #[inline(always)]
    fn default() -> CAPCTRL12 {
        CAPCTRL12(0)
    }
}
impl core::fmt::Debug for CAPCTRL12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL12")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL12 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL13(pub u32);
impl CAPCTRL13 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL13 {
    #[inline(always)]
    fn default() -> CAPCTRL13 {
        CAPCTRL13(0)
    }
}
impl core::fmt::Debug for CAPCTRL13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL13")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL13 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL14(pub u32);
impl CAPCTRL14 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL14 {
    #[inline(always)]
    fn default() -> CAPCTRL14 {
        CAPCTRL14(0)
    }
}
impl core::fmt::Debug for CAPCTRL14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL14")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL14 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL15(pub u32);
impl CAPCTRL15 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL15 {
    #[inline(always)]
    fn default() -> CAPCTRL15 {
        CAPCTRL15(0)
    }
}
impl core::fmt::Debug for CAPCTRL15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL15")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL15 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL2(pub u32);
impl CAPCTRL2 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL2 {
    #[inline(always)]
    fn default() -> CAPCTRL2 {
        CAPCTRL2(0)
    }
}
impl core::fmt::Debug for CAPCTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL2")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL2 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL3(pub u32);
impl CAPCTRL3 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL3 {
    #[inline(always)]
    fn default() -> CAPCTRL3 {
        CAPCTRL3(0)
    }
}
impl core::fmt::Debug for CAPCTRL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL3")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL3 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL4(pub u32);
impl CAPCTRL4 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL4 {
    #[inline(always)]
    fn default() -> CAPCTRL4 {
        CAPCTRL4(0)
    }
}
impl core::fmt::Debug for CAPCTRL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL4")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL4 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL5(pub u32);
impl CAPCTRL5 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL5 {
    #[inline(always)]
    fn default() -> CAPCTRL5 {
        CAPCTRL5(0)
    }
}
impl core::fmt::Debug for CAPCTRL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL5")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL5 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL6(pub u32);
impl CAPCTRL6 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL6 {
    #[inline(always)]
    fn default() -> CAPCTRL6 {
        CAPCTRL6(0)
    }
}
impl core::fmt::Debug for CAPCTRL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL6")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL6 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL7(pub u32);
impl CAPCTRL7 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL7 {
    #[inline(always)]
    fn default() -> CAPCTRL7 {
        CAPCTRL7(0)
    }
}
impl core::fmt::Debug for CAPCTRL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL7")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL7 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL8(pub u32);
impl CAPCTRL8 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL8 {
    #[inline(always)]
    fn default() -> CAPCTRL8 {
        CAPCTRL8(0)
    }
}
impl core::fmt::Debug for CAPCTRL8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL8")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL8 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT capture control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPCTRL9(pub u32);
impl CAPCTRL9 {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPCONn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_CAPCONn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAPCTRL9 {
    #[inline(always)]
    fn default() -> CAPCTRL9 {
        CAPCTRL9(0)
    }
}
impl core::fmt::Debug for CAPCTRL9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCTRL9")
            .field("CAPCONn_L", &self.CAPCONn_L())
            .field("CAPCONn_H", &self.CAPCONn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPCTRL9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAPCTRL9 {{ CAPCONn_L: {=u16:?}, CAPCONn_H: {=u16:?} }}",
            self.CAPCONn_L(),
            self.CAPCONn_H()
        )
    }
}
#[doc = "SCT conflict interrupt enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONEN(pub u32);
impl CONEN {
    #[doc = "The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn NCEN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_NCEN(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CONEN {
    #[inline(always)]
    fn default() -> CONEN {
        CONEN(0)
    }
}
impl core::fmt::Debug for CONEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONEN").field("NCEN", &self.NCEN()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CONEN {{ NCEN: {=u16:?} }}", self.NCEN())
    }
}
#[doc = "SCT configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG(pub u32);
impl CONFIG {
    #[doc = "SCT operation."]
    #[must_use]
    #[inline(always)]
    pub const fn UNIFY(&self) -> super::vals::UNIFY {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UNIFY::from_bits(val as u8)
    }
    #[doc = "SCT operation."]
    #[inline(always)]
    pub const fn set_UNIFY(&mut self, val: super::vals::UNIFY) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SCT clock mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKMODE(&self) -> super::vals::CLKMODE {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::CLKMODE::from_bits(val as u8)
    }
    #[doc = "SCT clock mode."]
    #[inline(always)]
    pub const fn set_CLKMODE(&mut self, val: super::vals::CLKMODE) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[must_use]
    #[inline(always)]
    pub const fn CKSEL(&self) -> super::vals::CKSEL {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::CKSEL::from_bits(val as u8)
    }
    #[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub const fn set_CKSEL(&mut self, val: super::vals::CKSEL) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn NORELOAD_L(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_NORELOAD_L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn NORELOAD_H(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_NORELOAD_H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[must_use]
    #[inline(always)]
    pub const fn INSYNC(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub const fn set_INSYNC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOLIMIT_L(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_AUTOLIMIT_L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOLIMIT_H(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_AUTOLIMIT_H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for CONFIG {
    #[inline(always)]
    fn default() -> CONFIG {
        CONFIG(0)
    }
}
impl core::fmt::Debug for CONFIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("UNIFY", &self.UNIFY())
            .field("CLKMODE", &self.CLKMODE())
            .field("CKSEL", &self.CKSEL())
            .field("NORELOAD_L", &self.NORELOAD_L())
            .field("NORELOAD_H", &self.NORELOAD_H())
            .field("INSYNC", &self.INSYNC())
            .field("AUTOLIMIT_L", &self.AUTOLIMIT_L())
            .field("AUTOLIMIT_H", &self.AUTOLIMIT_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG {{ UNIFY: {:?}, CLKMODE: {:?}, CKSEL: {:?}, NORELOAD_L: {=bool:?}, NORELOAD_H: {=bool:?}, INSYNC: {=u8:?}, AUTOLIMIT_L: {=bool:?}, AUTOLIMIT_H: {=bool:?} }}",
            self.UNIFY(),
            self.CLKMODE(),
            self.CKSEL(),
            self.NORELOAD_L(),
            self.NORELOAD_H(),
            self.INSYNC(),
            self.AUTOLIMIT_L(),
            self.AUTOLIMIT_H()
        )
    }
}
#[doc = "SCT conflict flag register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFLAG(pub u32);
impl CONFLAG {
    #[doc = "Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn NCFLAG(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_NCFLAG(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSERRL(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub const fn set_BUSERRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSERRH(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub const fn set_BUSERRH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CONFLAG {
    #[inline(always)]
    fn default() -> CONFLAG {
        CONFLAG(0)
    }
}
impl core::fmt::Debug for CONFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFLAG")
            .field("NCFLAG", &self.NCFLAG())
            .field("BUSERRL", &self.BUSERRL())
            .field("BUSERRH", &self.BUSERRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFLAG {{ NCFLAG: {=u16:?}, BUSERRL: {=bool:?}, BUSERRH: {=bool:?} }}",
            self.NCFLAG(),
            self.BUSERRL(),
            self.BUSERRH()
        )
    }
}
#[doc = "SCT counter register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COUNT(pub u32);
impl COUNT {
    #[doc = "When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn CTR_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub const fn set_CTR_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn CTR_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub const fn set_CTR_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for COUNT {
    #[inline(always)]
    fn default() -> COUNT {
        COUNT(0)
    }
}
impl core::fmt::Debug for COUNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT")
            .field("CTR_L", &self.CTR_L())
            .field("CTR_H", &self.CTR_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COUNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COUNT {{ CTR_L: {=u16:?}, CTR_H: {=u16:?} }}",
            self.CTR_L(),
            self.CTR_H()
        )
    }
}
#[doc = "SCT control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DOWN_L(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_DOWN_L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP_L(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub const fn set_STOP_L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT_L(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub const fn set_HALT_L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRCTR_L(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub const fn set_CLRCTR_L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "L or unified counter direction select."]
    #[must_use]
    #[inline(always)]
    pub const fn BIDIR_L(&self) -> super::vals::BIDIR_L {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BIDIR_L::from_bits(val as u8)
    }
    #[doc = "L or unified counter direction select."]
    #[inline(always)]
    pub const fn set_BIDIR_L(&mut self, val: super::vals::BIDIR_L) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[must_use]
    #[inline(always)]
    pub const fn PRE_L(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub const fn set_PRE_L(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DOWN_H(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_DOWN_H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP_H(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub const fn set_STOP_H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn HALT_H(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub const fn set_HALT_H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRCTR_H(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub const fn set_CLRCTR_H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Direction select."]
    #[must_use]
    #[inline(always)]
    pub const fn BIDIR_H(&self) -> super::vals::BIDIR_H {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::BIDIR_H::from_bits(val as u8)
    }
    #[doc = "Direction select."]
    #[inline(always)]
    pub const fn set_BIDIR_H(&mut self, val: super::vals::BIDIR_H) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[must_use]
    #[inline(always)]
    pub const fn PRE_H(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub const fn set_PRE_H(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("DOWN_L", &self.DOWN_L())
            .field("STOP_L", &self.STOP_L())
            .field("HALT_L", &self.HALT_L())
            .field("CLRCTR_L", &self.CLRCTR_L())
            .field("BIDIR_L", &self.BIDIR_L())
            .field("PRE_L", &self.PRE_L())
            .field("DOWN_H", &self.DOWN_H())
            .field("STOP_H", &self.STOP_H())
            .field("HALT_H", &self.HALT_H())
            .field("CLRCTR_H", &self.CLRCTR_H())
            .field("BIDIR_H", &self.BIDIR_H())
            .field("PRE_H", &self.PRE_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ DOWN_L: {=bool:?}, STOP_L: {=bool:?}, HALT_L: {=bool:?}, CLRCTR_L: {=bool:?}, BIDIR_L: {:?}, PRE_L: {=u8:?}, DOWN_H: {=bool:?}, STOP_H: {=bool:?}, HALT_H: {=bool:?}, CLRCTR_H: {=bool:?}, BIDIR_H: {:?}, PRE_H: {=u8:?} }}",
            self.DOWN_L(),
            self.STOP_L(),
            self.HALT_L(),
            self.CLRCTR_L(),
            self.BIDIR_L(),
            self.PRE_L(),
            self.DOWN_H(),
            self.STOP_H(),
            self.HALT_H(),
            self.CLRCTR_H(),
            self.BIDIR_H(),
            self.PRE_H()
        )
    }
}
#[doc = "SCT DMA request 0 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMAREQ0(pub u32);
impl DMAREQ0 {
    #[doc = "If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_DEV_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[must_use]
    #[inline(always)]
    pub const fn DRL0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub const fn set_DRL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[must_use]
    #[inline(always)]
    pub const fn DRQ0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub const fn set_DRQ0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMAREQ0 {
    #[inline(always)]
    fn default() -> DMAREQ0 {
        DMAREQ0(0)
    }
}
impl core::fmt::Debug for DMAREQ0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAREQ0")
            .field("DEV_0", &self.DEV_0())
            .field("DRL0", &self.DRL0())
            .field("DRQ0", &self.DRQ0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMAREQ0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMAREQ0 {{ DEV_0: {=u16:?}, DRL0: {=bool:?}, DRQ0: {=bool:?} }}",
            self.DEV_0(),
            self.DRL0(),
            self.DRQ0()
        )
    }
}
#[doc = "SCT DMA request 1 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMAREQ1(pub u32);
impl DMAREQ1 {
    #[doc = "If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_DEV_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[must_use]
    #[inline(always)]
    pub const fn DRL1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub const fn set_DRL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[must_use]
    #[inline(always)]
    pub const fn DRQ1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub const fn set_DRQ1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMAREQ1 {
    #[inline(always)]
    fn default() -> DMAREQ1 {
        DMAREQ1(0)
    }
}
impl core::fmt::Debug for DMAREQ1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAREQ1")
            .field("DEV_1", &self.DEV_1())
            .field("DRL1", &self.DRL1())
            .field("DRQ1", &self.DRQ1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMAREQ1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMAREQ1 {{ DEV_1: {=u16:?}, DRL1: {=bool:?}, DRQ1: {=bool:?} }}",
            self.DEV_1(),
            self.DRL1(),
            self.DRQ1()
        )
    }
}
#[doc = "SCT event interrupt enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVEN(pub u32);
impl EVEN {
    #[doc = "The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn IEN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_IEN(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EVEN {
    #[inline(always)]
    fn default() -> EVEN {
        EVEN(0)
    }
}
impl core::fmt::Debug for EVEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVEN").field("IEN", &self.IEN()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EVEN {{ IEN: {=u16:?} }}", self.IEN())
    }
}
#[doc = "SCT event flag register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVFLAG(pub u32);
impl EVFLAG {
    #[doc = "Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn FLAG(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_FLAG(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EVFLAG {
    #[inline(always)]
    fn default() -> EVFLAG {
        EVFLAG(0)
    }
}
impl core::fmt::Debug for EVFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVFLAG")
            .field("FLAG", &self.FLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVFLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EVFLAG {{ FLAG: {=u16:?} }}", self.FLAG())
    }
}
#[doc = "SCT event control register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EV_CTRL(pub u32);
impl EV_CTRL {
    #[doc = "Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub const fn set_MATCHSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn HEVENT(&self) -> super::vals::HEVENT {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HEVENT::from_bits(val as u8)
    }
    #[doc = "Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub const fn set_HEVENT(&mut self, val: super::vals::HEVENT) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Input/output select."]
    #[must_use]
    #[inline(always)]
    pub const fn OUTSEL(&self) -> super::vals::OUTSEL {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OUTSEL::from_bits(val as u8)
    }
    #[doc = "Input/output select."]
    #[inline(always)]
    pub const fn set_OUTSEL(&mut self, val: super::vals::OUTSEL) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSEL(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub const fn set_IOSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCOND(&self) -> super::vals::IOCOND {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCOND::from_bits(val as u8)
    }
    #[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period."]
    #[inline(always)]
    pub const fn set_IOCOND(&mut self, val: super::vals::IOCOND) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects how the specified match and I/O condition are used and combined."]
    #[must_use]
    #[inline(always)]
    pub const fn COMBMODE(&self) -> super::vals::COMBMODE {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::COMBMODE::from_bits(val as u8)
    }
    #[doc = "Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub const fn set_COMBMODE(&mut self, val: super::vals::COMBMODE) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[must_use]
    #[inline(always)]
    pub const fn STATELD(&self) -> super::vals::STATELD {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::STATELD::from_bits(val as u8)
    }
    #[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub const fn set_STATELD(&mut self, val: super::vals::STATELD) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[must_use]
    #[inline(always)]
    pub const fn STATEV(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub const fn set_STATEV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHMEM(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub const fn set_MATCHMEM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn DIRECTION(&self) -> super::vals::DIRECTION {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::DIRECTION::from_bits(val as u8)
    }
    #[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub const fn set_DIRECTION(&mut self, val: super::vals::DIRECTION) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for EV_CTRL {
    #[inline(always)]
    fn default() -> EV_CTRL {
        EV_CTRL(0)
    }
}
impl core::fmt::Debug for EV_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EV_CTRL")
            .field("MATCHSEL", &self.MATCHSEL())
            .field("HEVENT", &self.HEVENT())
            .field("OUTSEL", &self.OUTSEL())
            .field("IOSEL", &self.IOSEL())
            .field("IOCOND", &self.IOCOND())
            .field("COMBMODE", &self.COMBMODE())
            .field("STATELD", &self.STATELD())
            .field("STATEV", &self.STATEV())
            .field("MATCHMEM", &self.MATCHMEM())
            .field("DIRECTION", &self.DIRECTION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EV_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EV_CTRL {{ MATCHSEL: {=u8:?}, HEVENT: {:?}, OUTSEL: {:?}, IOSEL: {=u8:?}, IOCOND: {:?}, COMBMODE: {:?}, STATELD: {:?}, STATEV: {=u8:?}, MATCHMEM: {=bool:?}, DIRECTION: {:?} }}",
            self.MATCHSEL(),
            self.HEVENT(),
            self.OUTSEL(),
            self.IOSEL(),
            self.IOCOND(),
            self.COMBMODE(),
            self.STATELD(),
            self.STATEV(),
            self.MATCHMEM(),
            self.DIRECTION()
        )
    }
}
#[doc = "SCT event state register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EV_STATE(pub u32);
impl EV_STATE {
    #[doc = "If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn STATEMSKn(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub const fn set_STATEMSKn(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EV_STATE {
    #[inline(always)]
    fn default() -> EV_STATE {
        EV_STATE(0)
    }
}
impl core::fmt::Debug for EV_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EV_STATE")
            .field("STATEMSKn", &self.STATEMSKn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EV_STATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EV_STATE {{ STATEMSKn: {=u16:?} }}", self.STATEMSKn())
    }
}
#[doc = "SCT halt event select register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HALT(pub u32);
impl HALT {
    #[doc = "If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn HALTMSK_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_HALTMSK_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn HALTMSK_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_HALTMSK_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HALT {
    #[inline(always)]
    fn default() -> HALT {
        HALT(0)
    }
}
impl core::fmt::Debug for HALT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HALT")
            .field("HALTMSK_L", &self.HALTMSK_L())
            .field("HALTMSK_H", &self.HALTMSK_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HALT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HALT {{ HALTMSK_L: {=u16:?}, HALTMSK_H: {=u16:?} }}",
            self.HALTMSK_L(),
            self.HALTMSK_H()
        )
    }
}
#[doc = "SCT input register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INPUT(pub u32);
impl INPUT {
    #[doc = "Input 0 state. Input 0 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Input 1 state. Input 1 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Input 1 state. Input 1 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input 2 state. Input 2 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input 2 state. Input 2 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input 3 state. Input 3 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input 3 state. Input 3 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input 4 state. Input 4 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input 4 state. Input 4 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Input 5 state. Input 5 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Input 5 state. Input 5 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input 6 state. Input 6 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input 6 state. Input 6 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input 7 state. Input 7 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input 7 state. Input 7 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Input 8 state. Input 8 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Input 8 state. Input 8 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Input 9 state. Input 9 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input 9 state. Input 9 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Input 10 state. Input 10 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input 10 state. Input 10 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 11 state. Input 11 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input 11 state. Input 11 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Input 12 state. Input 12 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 12 state. Input 12 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Input 13 state. Input 13 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Input 13 state. Input 13 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Input 14 state. Input 14 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Input 14 state. Input 14 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Input 15 state. Input 15 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn AIN15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Input 15 state. Input 15 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_AIN15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn SIN15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_SIN15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for INPUT {
    #[inline(always)]
    fn default() -> INPUT {
        INPUT(0)
    }
}
impl core::fmt::Debug for INPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUT")
            .field("AIN0", &self.AIN0())
            .field("AIN1", &self.AIN1())
            .field("AIN2", &self.AIN2())
            .field("AIN3", &self.AIN3())
            .field("AIN4", &self.AIN4())
            .field("AIN5", &self.AIN5())
            .field("AIN6", &self.AIN6())
            .field("AIN7", &self.AIN7())
            .field("AIN8", &self.AIN8())
            .field("AIN9", &self.AIN9())
            .field("AIN10", &self.AIN10())
            .field("AIN11", &self.AIN11())
            .field("AIN12", &self.AIN12())
            .field("AIN13", &self.AIN13())
            .field("AIN14", &self.AIN14())
            .field("AIN15", &self.AIN15())
            .field("SIN0", &self.SIN0())
            .field("SIN1", &self.SIN1())
            .field("SIN2", &self.SIN2())
            .field("SIN3", &self.SIN3())
            .field("SIN4", &self.SIN4())
            .field("SIN5", &self.SIN5())
            .field("SIN6", &self.SIN6())
            .field("SIN7", &self.SIN7())
            .field("SIN8", &self.SIN8())
            .field("SIN9", &self.SIN9())
            .field("SIN10", &self.SIN10())
            .field("SIN11", &self.SIN11())
            .field("SIN12", &self.SIN12())
            .field("SIN13", &self.SIN13())
            .field("SIN14", &self.SIN14())
            .field("SIN15", &self.SIN15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INPUT {{ AIN0: {=bool:?}, AIN1: {=bool:?}, AIN2: {=bool:?}, AIN3: {=bool:?}, AIN4: {=bool:?}, AIN5: {=bool:?}, AIN6: {=bool:?}, AIN7: {=bool:?}, AIN8: {=bool:?}, AIN9: {=bool:?}, AIN10: {=bool:?}, AIN11: {=bool:?}, AIN12: {=bool:?}, AIN13: {=bool:?}, AIN14: {=bool:?}, AIN15: {=bool:?}, SIN0: {=bool:?}, SIN1: {=bool:?}, SIN2: {=bool:?}, SIN3: {=bool:?}, SIN4: {=bool:?}, SIN5: {=bool:?}, SIN6: {=bool:?}, SIN7: {=bool:?}, SIN8: {=bool:?}, SIN9: {=bool:?}, SIN10: {=bool:?}, SIN11: {=bool:?}, SIN12: {=bool:?}, SIN13: {=bool:?}, SIN14: {=bool:?}, SIN15: {=bool:?} }}",
            self.AIN0(),
            self.AIN1(),
            self.AIN2(),
            self.AIN3(),
            self.AIN4(),
            self.AIN5(),
            self.AIN6(),
            self.AIN7(),
            self.AIN8(),
            self.AIN9(),
            self.AIN10(),
            self.AIN11(),
            self.AIN12(),
            self.AIN13(),
            self.AIN14(),
            self.AIN15(),
            self.SIN0(),
            self.SIN1(),
            self.SIN2(),
            self.SIN3(),
            self.SIN4(),
            self.SIN5(),
            self.SIN6(),
            self.SIN7(),
            self.SIN8(),
            self.SIN9(),
            self.SIN10(),
            self.SIN11(),
            self.SIN12(),
            self.SIN13(),
            self.SIN14(),
            self.SIN15()
        )
    }
}
#[doc = "SCT limit event select register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LIMIT(pub u32);
impl LIMIT {
    #[doc = "If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMMSK_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_LIMMSK_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMMSK_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_LIMMSK_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for LIMIT {
    #[inline(always)]
    fn default() -> LIMIT {
        LIMIT(0)
    }
}
impl core::fmt::Debug for LIMIT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIMIT")
            .field("LIMMSK_L", &self.LIMMSK_L())
            .field("LIMMSK_H", &self.LIMMSK_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LIMIT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LIMIT {{ LIMMSK_L: {=u16:?}, LIMMSK_H: {=u16:?} }}",
            self.LIMMSK_L(),
            self.LIMMSK_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH0(pub u32);
impl MATCH0 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH0 {
    #[inline(always)]
    fn default() -> MATCH0 {
        MATCH0(0)
    }
}
impl core::fmt::Debug for MATCH0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH0")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH0 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH1(pub u32);
impl MATCH1 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH1 {
    #[inline(always)]
    fn default() -> MATCH1 {
        MATCH1(0)
    }
}
impl core::fmt::Debug for MATCH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH1")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH1 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH10(pub u32);
impl MATCH10 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH10 {
    #[inline(always)]
    fn default() -> MATCH10 {
        MATCH10(0)
    }
}
impl core::fmt::Debug for MATCH10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH10")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH10 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH11(pub u32);
impl MATCH11 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH11 {
    #[inline(always)]
    fn default() -> MATCH11 {
        MATCH11(0)
    }
}
impl core::fmt::Debug for MATCH11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH11")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH11 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH12(pub u32);
impl MATCH12 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH12 {
    #[inline(always)]
    fn default() -> MATCH12 {
        MATCH12(0)
    }
}
impl core::fmt::Debug for MATCH12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH12")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH12 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH13(pub u32);
impl MATCH13 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH13 {
    #[inline(always)]
    fn default() -> MATCH13 {
        MATCH13(0)
    }
}
impl core::fmt::Debug for MATCH13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH13")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH13 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH14(pub u32);
impl MATCH14 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH14 {
    #[inline(always)]
    fn default() -> MATCH14 {
        MATCH14(0)
    }
}
impl core::fmt::Debug for MATCH14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH14")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH14 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH15(pub u32);
impl MATCH15 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH15 {
    #[inline(always)]
    fn default() -> MATCH15 {
        MATCH15(0)
    }
}
impl core::fmt::Debug for MATCH15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH15")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH15 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH2(pub u32);
impl MATCH2 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH2 {
    #[inline(always)]
    fn default() -> MATCH2 {
        MATCH2(0)
    }
}
impl core::fmt::Debug for MATCH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH2")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH2 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH3(pub u32);
impl MATCH3 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH3 {
    #[inline(always)]
    fn default() -> MATCH3 {
        MATCH3(0)
    }
}
impl core::fmt::Debug for MATCH3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH3")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH3 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH4(pub u32);
impl MATCH4 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH4 {
    #[inline(always)]
    fn default() -> MATCH4 {
        MATCH4(0)
    }
}
impl core::fmt::Debug for MATCH4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH4")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH4 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH5(pub u32);
impl MATCH5 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH5 {
    #[inline(always)]
    fn default() -> MATCH5 {
        MATCH5(0)
    }
}
impl core::fmt::Debug for MATCH5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH5")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH5 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH6(pub u32);
impl MATCH6 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH6 {
    #[inline(always)]
    fn default() -> MATCH6 {
        MATCH6(0)
    }
}
impl core::fmt::Debug for MATCH6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH6")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH6 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH7(pub u32);
impl MATCH7 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH7 {
    #[inline(always)]
    fn default() -> MATCH7 {
        MATCH7(0)
    }
}
impl core::fmt::Debug for MATCH7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH7")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH7 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH8(pub u32);
impl MATCH8 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH8 {
    #[inline(always)]
    fn default() -> MATCH8 {
        MATCH8(0)
    }
}
impl core::fmt::Debug for MATCH8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH8")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH8 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match value register of match channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH9(pub u32);
impl MATCH9 {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_MATCHn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCH9 {
    #[inline(always)]
    fn default() -> MATCH9 {
        MATCH9(0)
    }
}
impl core::fmt::Debug for MATCH9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH9")
            .field("MATCHn_L", &self.MATCHn_L())
            .field("MATCHn_H", &self.MATCHn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCH9 {{ MATCHn_L: {=u16:?}, MATCHn_H: {=u16:?} }}",
            self.MATCHn_L(),
            self.MATCHn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL0(pub u32);
impl MATCHREL0 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL0 {
    #[inline(always)]
    fn default() -> MATCHREL0 {
        MATCHREL0(0)
    }
}
impl core::fmt::Debug for MATCHREL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL0")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL0 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL1(pub u32);
impl MATCHREL1 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL1 {
    #[inline(always)]
    fn default() -> MATCHREL1 {
        MATCHREL1(0)
    }
}
impl core::fmt::Debug for MATCHREL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL1")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL1 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL10(pub u32);
impl MATCHREL10 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL10 {
    #[inline(always)]
    fn default() -> MATCHREL10 {
        MATCHREL10(0)
    }
}
impl core::fmt::Debug for MATCHREL10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL10")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL10 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL11(pub u32);
impl MATCHREL11 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL11 {
    #[inline(always)]
    fn default() -> MATCHREL11 {
        MATCHREL11(0)
    }
}
impl core::fmt::Debug for MATCHREL11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL11")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL11 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL12(pub u32);
impl MATCHREL12 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL12 {
    #[inline(always)]
    fn default() -> MATCHREL12 {
        MATCHREL12(0)
    }
}
impl core::fmt::Debug for MATCHREL12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL12")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL12 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL13(pub u32);
impl MATCHREL13 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL13 {
    #[inline(always)]
    fn default() -> MATCHREL13 {
        MATCHREL13(0)
    }
}
impl core::fmt::Debug for MATCHREL13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL13")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL13 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL14(pub u32);
impl MATCHREL14 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL14 {
    #[inline(always)]
    fn default() -> MATCHREL14 {
        MATCHREL14(0)
    }
}
impl core::fmt::Debug for MATCHREL14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL14")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL14 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL15(pub u32);
impl MATCHREL15 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL15 {
    #[inline(always)]
    fn default() -> MATCHREL15 {
        MATCHREL15(0)
    }
}
impl core::fmt::Debug for MATCHREL15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL15")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL15 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL2(pub u32);
impl MATCHREL2 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL2 {
    #[inline(always)]
    fn default() -> MATCHREL2 {
        MATCHREL2(0)
    }
}
impl core::fmt::Debug for MATCHREL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL2")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL2 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL3(pub u32);
impl MATCHREL3 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL3 {
    #[inline(always)]
    fn default() -> MATCHREL3 {
        MATCHREL3(0)
    }
}
impl core::fmt::Debug for MATCHREL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL3")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL3 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL4(pub u32);
impl MATCHREL4 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL4 {
    #[inline(always)]
    fn default() -> MATCHREL4 {
        MATCHREL4(0)
    }
}
impl core::fmt::Debug for MATCHREL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL4")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL4 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL5(pub u32);
impl MATCHREL5 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL5 {
    #[inline(always)]
    fn default() -> MATCHREL5 {
        MATCHREL5(0)
    }
}
impl core::fmt::Debug for MATCHREL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL5")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL5 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL6(pub u32);
impl MATCHREL6 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL6 {
    #[inline(always)]
    fn default() -> MATCHREL6 {
        MATCHREL6(0)
    }
}
impl core::fmt::Debug for MATCHREL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL6")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL6 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL7(pub u32);
impl MATCHREL7 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL7 {
    #[inline(always)]
    fn default() -> MATCHREL7 {
        MATCHREL7(0)
    }
}
impl core::fmt::Debug for MATCHREL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL7")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL7 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL8(pub u32);
impl MATCHREL8 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL8 {
    #[inline(always)]
    fn default() -> MATCHREL8 {
        MATCHREL8(0)
    }
}
impl core::fmt::Debug for MATCHREL8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL8")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL8 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT match reload value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCHREL9(pub u32);
impl MATCHREL9 {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOADn_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_RELOADn_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MATCHREL9 {
    #[inline(always)]
    fn default() -> MATCHREL9 {
        MATCHREL9(0)
    }
}
impl core::fmt::Debug for MATCHREL9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCHREL9")
            .field("RELOADn_L", &self.RELOADn_L())
            .field("RELOADn_H", &self.RELOADn_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCHREL9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MATCHREL9 {{ RELOADn_L: {=u16:?}, RELOADn_H: {=u16:?} }}",
            self.RELOADn_L(),
            self.RELOADn_H()
        )
    }
}
#[doc = "SCT output register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUTPUT(pub u32);
impl OUTPUT {
    #[doc = "Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn OUT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_OUT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OUTPUT {
    #[inline(always)]
    fn default() -> OUTPUT {
        OUTPUT(0)
    }
}
impl core::fmt::Debug for OUTPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUT").field("OUT", &self.OUT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUTPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUTPUT {{ OUT: {=u16:?} }}", self.OUT())
    }
}
#[doc = "SCT output counter direction control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUTPUTDIRCTRL(pub u32);
impl OUTPUTDIRCTRL {
    #[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR0(&self) -> super::vals::SETCLR0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SETCLR0::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR0(&mut self, val: super::vals::SETCLR0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR1(&self) -> super::vals::SETCLR1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SETCLR1::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR1(&mut self, val: super::vals::SETCLR1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR2(&self) -> super::vals::SETCLR2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SETCLR2::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR2(&mut self, val: super::vals::SETCLR2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR3(&self) -> super::vals::SETCLR3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SETCLR3::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR3(&mut self, val: super::vals::SETCLR3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR4(&self) -> super::vals::SETCLR4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SETCLR4::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR4(&mut self, val: super::vals::SETCLR4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR5(&self) -> super::vals::SETCLR5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::SETCLR5::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR5(&mut self, val: super::vals::SETCLR5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR6(&self) -> super::vals::SETCLR6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SETCLR6::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR6(&mut self, val: super::vals::SETCLR6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR7(&self) -> super::vals::SETCLR7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SETCLR7::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR7(&mut self, val: super::vals::SETCLR7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR8(&self) -> super::vals::SETCLR8 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SETCLR8::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR8(&mut self, val: super::vals::SETCLR8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR9(&self) -> super::vals::SETCLR9 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::SETCLR9::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR9(&mut self, val: super::vals::SETCLR9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR10(&self) -> super::vals::SETCLR10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::SETCLR10::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR10(&mut self, val: super::vals::SETCLR10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR11(&self) -> super::vals::SETCLR11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::SETCLR11::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR11(&mut self, val: super::vals::SETCLR11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR12(&self) -> super::vals::SETCLR12 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SETCLR12::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR12(&mut self, val: super::vals::SETCLR12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR13(&self) -> super::vals::SETCLR13 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::SETCLR13::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR13(&mut self, val: super::vals::SETCLR13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR14(&self) -> super::vals::SETCLR14 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::SETCLR14::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR14(&mut self, val: super::vals::SETCLR14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCLR15(&self) -> super::vals::SETCLR15 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::SETCLR15::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_SETCLR15(&mut self, val: super::vals::SETCLR15) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for OUTPUTDIRCTRL {
    #[inline(always)]
    fn default() -> OUTPUTDIRCTRL {
        OUTPUTDIRCTRL(0)
    }
}
impl core::fmt::Debug for OUTPUTDIRCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTPUTDIRCTRL")
            .field("SETCLR0", &self.SETCLR0())
            .field("SETCLR1", &self.SETCLR1())
            .field("SETCLR2", &self.SETCLR2())
            .field("SETCLR3", &self.SETCLR3())
            .field("SETCLR4", &self.SETCLR4())
            .field("SETCLR5", &self.SETCLR5())
            .field("SETCLR6", &self.SETCLR6())
            .field("SETCLR7", &self.SETCLR7())
            .field("SETCLR8", &self.SETCLR8())
            .field("SETCLR9", &self.SETCLR9())
            .field("SETCLR10", &self.SETCLR10())
            .field("SETCLR11", &self.SETCLR11())
            .field("SETCLR12", &self.SETCLR12())
            .field("SETCLR13", &self.SETCLR13())
            .field("SETCLR14", &self.SETCLR14())
            .field("SETCLR15", &self.SETCLR15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUTPUTDIRCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OUTPUTDIRCTRL {{ SETCLR0: {:?}, SETCLR1: {:?}, SETCLR2: {:?}, SETCLR3: {:?}, SETCLR4: {:?}, SETCLR5: {:?}, SETCLR6: {:?}, SETCLR7: {:?}, SETCLR8: {:?}, SETCLR9: {:?}, SETCLR10: {:?}, SETCLR11: {:?}, SETCLR12: {:?}, SETCLR13: {:?}, SETCLR14: {:?}, SETCLR15: {:?} }}",
            self.SETCLR0(),
            self.SETCLR1(),
            self.SETCLR2(),
            self.SETCLR3(),
            self.SETCLR4(),
            self.SETCLR5(),
            self.SETCLR6(),
            self.SETCLR7(),
            self.SETCLR8(),
            self.SETCLR9(),
            self.SETCLR10(),
            self.SETCLR11(),
            self.SETCLR12(),
            self.SETCLR13(),
            self.SETCLR14(),
            self.SETCLR15()
        )
    }
}
#[doc = "SCT output 0 clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUT_CLR(pub u32);
impl OUT_CLR {
    #[doc = "A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x2) event 0 = bit 0, event 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OUT_CLR {
    #[inline(always)]
    fn default() -> OUT_CLR {
        OUT_CLR(0)
    }
}
impl core::fmt::Debug for OUT_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CLR").field("CLR", &self.CLR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUT_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUT_CLR {{ CLR: {=u16:?} }}", self.CLR())
    }
}
#[doc = "SCT output 0 set register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUT_SET(pub u32);
impl OUT_SET {
    #[doc = "A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[must_use]
    #[inline(always)]
    pub const fn SET(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub const fn set_SET(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OUT_SET {
    #[inline(always)]
    fn default() -> OUT_SET {
        OUT_SET(0)
    }
}
impl core::fmt::Debug for OUT_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_SET").field("SET", &self.SET()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUT_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUT_SET {{ SET: {=u16:?} }}", self.SET())
    }
}
#[doc = "SCT match/capture mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REGMODE(pub u32);
impl REGMODE {
    #[doc = "Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn REGMOD_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub const fn set_REGMOD_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[must_use]
    #[inline(always)]
    pub const fn REGMOD_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub const fn set_REGMOD_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for REGMODE {
    #[inline(always)]
    fn default() -> REGMODE {
        REGMODE(0)
    }
}
impl core::fmt::Debug for REGMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGMODE")
            .field("REGMOD_L", &self.REGMOD_L())
            .field("REGMOD_H", &self.REGMOD_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REGMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REGMODE {{ REGMOD_L: {=u16:?}, REGMOD_H: {=u16:?} }}",
            self.REGMOD_L(),
            self.REGMOD_H()
        )
    }
}
#[doc = "SCT conflict resolution register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RES(pub u32);
impl RES {
    #[doc = "Effect of simultaneous set and clear on output 0."]
    #[must_use]
    #[inline(always)]
    pub const fn O0RES(&self) -> super::vals::O0RES {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::O0RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub const fn set_O0RES(&mut self, val: super::vals::O0RES) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 1."]
    #[must_use]
    #[inline(always)]
    pub const fn O1RES(&self) -> super::vals::O1RES {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::O1RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub const fn set_O1RES(&mut self, val: super::vals::O1RES) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 2."]
    #[must_use]
    #[inline(always)]
    pub const fn O2RES(&self) -> super::vals::O2RES {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::O2RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub const fn set_O2RES(&mut self, val: super::vals::O2RES) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 3."]
    #[must_use]
    #[inline(always)]
    pub const fn O3RES(&self) -> super::vals::O3RES {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::O3RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub const fn set_O3RES(&mut self, val: super::vals::O3RES) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 4."]
    #[must_use]
    #[inline(always)]
    pub const fn O4RES(&self) -> super::vals::O4RES {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::O4RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub const fn set_O4RES(&mut self, val: super::vals::O4RES) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 5."]
    #[must_use]
    #[inline(always)]
    pub const fn O5RES(&self) -> super::vals::O5RES {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::O5RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub const fn set_O5RES(&mut self, val: super::vals::O5RES) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 6."]
    #[must_use]
    #[inline(always)]
    pub const fn O6RES(&self) -> super::vals::O6RES {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::O6RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub const fn set_O6RES(&mut self, val: super::vals::O6RES) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 7."]
    #[must_use]
    #[inline(always)]
    pub const fn O7RES(&self) -> super::vals::O7RES {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::O7RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub const fn set_O7RES(&mut self, val: super::vals::O7RES) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 8."]
    #[must_use]
    #[inline(always)]
    pub const fn O8RES(&self) -> super::vals::O8RES {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::O8RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub const fn set_O8RES(&mut self, val: super::vals::O8RES) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 9."]
    #[must_use]
    #[inline(always)]
    pub const fn O9RES(&self) -> super::vals::O9RES {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::O9RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub const fn set_O9RES(&mut self, val: super::vals::O9RES) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 10."]
    #[must_use]
    #[inline(always)]
    pub const fn O10RES(&self) -> super::vals::O10RES {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::O10RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub const fn set_O10RES(&mut self, val: super::vals::O10RES) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 11."]
    #[must_use]
    #[inline(always)]
    pub const fn O11RES(&self) -> super::vals::O11RES {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::O11RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub const fn set_O11RES(&mut self, val: super::vals::O11RES) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 12."]
    #[must_use]
    #[inline(always)]
    pub const fn O12RES(&self) -> super::vals::O12RES {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::O12RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub const fn set_O12RES(&mut self, val: super::vals::O12RES) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 13."]
    #[must_use]
    #[inline(always)]
    pub const fn O13RES(&self) -> super::vals::O13RES {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::O13RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub const fn set_O13RES(&mut self, val: super::vals::O13RES) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 14."]
    #[must_use]
    #[inline(always)]
    pub const fn O14RES(&self) -> super::vals::O14RES {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::O14RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub const fn set_O14RES(&mut self, val: super::vals::O14RES) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Effect of simultaneous set and clear on output 15."]
    #[must_use]
    #[inline(always)]
    pub const fn O15RES(&self) -> super::vals::O15RES {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::O15RES::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub const fn set_O15RES(&mut self, val: super::vals::O15RES) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for RES {
    #[inline(always)]
    fn default() -> RES {
        RES(0)
    }
}
impl core::fmt::Debug for RES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES")
            .field("O0RES", &self.O0RES())
            .field("O1RES", &self.O1RES())
            .field("O2RES", &self.O2RES())
            .field("O3RES", &self.O3RES())
            .field("O4RES", &self.O4RES())
            .field("O5RES", &self.O5RES())
            .field("O6RES", &self.O6RES())
            .field("O7RES", &self.O7RES())
            .field("O8RES", &self.O8RES())
            .field("O9RES", &self.O9RES())
            .field("O10RES", &self.O10RES())
            .field("O11RES", &self.O11RES())
            .field("O12RES", &self.O12RES())
            .field("O13RES", &self.O13RES())
            .field("O14RES", &self.O14RES())
            .field("O15RES", &self.O15RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RES {{ O0RES: {:?}, O1RES: {:?}, O2RES: {:?}, O3RES: {:?}, O4RES: {:?}, O5RES: {:?}, O6RES: {:?}, O7RES: {:?}, O8RES: {:?}, O9RES: {:?}, O10RES: {:?}, O11RES: {:?}, O12RES: {:?}, O13RES: {:?}, O14RES: {:?}, O15RES: {:?} }}",
            self.O0RES(),
            self.O1RES(),
            self.O2RES(),
            self.O3RES(),
            self.O4RES(),
            self.O5RES(),
            self.O6RES(),
            self.O7RES(),
            self.O8RES(),
            self.O9RES(),
            self.O10RES(),
            self.O11RES(),
            self.O12RES(),
            self.O13RES(),
            self.O14RES(),
            self.O15RES()
        )
    }
}
#[doc = "SCT start event select register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct START(pub u32);
impl START {
    #[doc = "If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTMSK_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_STARTMSK_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTMSK_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_STARTMSK_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for START {
    #[inline(always)]
    fn default() -> START {
        START(0)
    }
}
impl core::fmt::Debug for START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START")
            .field("STARTMSK_L", &self.STARTMSK_L())
            .field("STARTMSK_H", &self.STARTMSK_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "START {{ STARTMSK_L: {=u16:?}, STARTMSK_H: {=u16:?} }}",
            self.STARTMSK_L(),
            self.STARTMSK_H()
        )
    }
}
#[doc = "SCT state register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATE(pub u32);
impl STATE {
    #[doc = "State variable."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE_L(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "State variable."]
    #[inline(always)]
    pub const fn set_STATE_L(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "State variable."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE_H(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "State variable."]
    #[inline(always)]
    pub const fn set_STATE_H(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for STATE {
    #[inline(always)]
    fn default() -> STATE {
        STATE(0)
    }
}
impl core::fmt::Debug for STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("STATE_L", &self.STATE_L())
            .field("STATE_H", &self.STATE_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATE {{ STATE_L: {=u8:?}, STATE_H: {=u8:?} }}",
            self.STATE_L(),
            self.STATE_H()
        )
    }
}
#[doc = "SCT stop event select register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STOP(pub u32);
impl STOP {
    #[doc = "If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPMSK_L(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_STOPMSK_L(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPMSK_H(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_STOPMSK_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for STOP {
    #[inline(always)]
    fn default() -> STOP {
        STOP(0)
    }
}
impl core::fmt::Debug for STOP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STOP")
            .field("STOPMSK_L", &self.STOPMSK_L())
            .field("STOPMSK_H", &self.STOPMSK_H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STOP {{ STOPMSK_L: {=u16:?}, STOPMSK_H: {=u16:?} }}",
            self.STOPMSK_L(),
            self.STOPMSK_H()
        )
    }
}
