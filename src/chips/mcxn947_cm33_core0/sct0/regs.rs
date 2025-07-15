#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap0(pub u32);
impl Cap0 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap1(pub u32);
impl Cap1 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap10(pub u32);
impl Cap10 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap11(pub u32);
impl Cap11 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap12(pub u32);
impl Cap12 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap13(pub u32);
impl Cap13 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap14(pub u32);
impl Cap14 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap15(pub u32);
impl Cap15 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap2(pub u32);
impl Cap2 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap3(pub u32);
impl Cap3 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap4(pub u32);
impl Cap4 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap5(pub u32);
impl Cap5 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap6(pub u32);
impl Cap6 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap7(pub u32);
impl Cap7 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap8(pub u32);
impl Cap8 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap9(pub u32);
impl Cap9 {
    #[doc = "Capture Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low"]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High"]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl0(pub u32);
impl Capctrl0 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl1(pub u32);
impl Capctrl1 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl10(pub u32);
impl Capctrl10 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl11(pub u32);
impl Capctrl11 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl12(pub u32);
impl Capctrl12 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl13(pub u32);
impl Capctrl13 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl14(pub u32);
impl Capctrl14 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl15(pub u32);
impl Capctrl15 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl2(pub u32);
impl Capctrl2 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl3(pub u32);
impl Capctrl3 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl4(pub u32);
impl Capctrl4 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl5(pub u32);
impl Capctrl5 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl6(pub u32);
impl Capctrl6 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl7(pub u32);
impl Capctrl7 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl8(pub u32);
impl Capctrl8 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Capture Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl9(pub u32);
impl Capctrl9 {
    #[doc = "Capture Control Low"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low"]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High"]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High"]
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
#[doc = "Conflict Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conen(pub u32);
impl Conen {
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ncen9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ncen9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
        f.debug_struct("Conen")
            .field("ncen0", &self.ncen0())
            .field("ncen1", &self.ncen1())
            .field("ncen2", &self.ncen2())
            .field("ncen3", &self.ncen3())
            .field("ncen4", &self.ncen4())
            .field("ncen5", &self.ncen5())
            .field("ncen6", &self.ncen6())
            .field("ncen7", &self.ncen7())
            .field("ncen8", &self.ncen8())
            .field("ncen9", &self.ncen9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Conen {{ ncen0: {=bool:?}, ncen1: {=bool:?}, ncen2: {=bool:?}, ncen3: {=bool:?}, ncen4: {=bool:?}, ncen5: {=bool:?}, ncen6: {=bool:?}, ncen7: {=bool:?}, ncen8: {=bool:?}, ncen9: {=bool:?} }}",
            self.ncen0(),
            self.ncen1(),
            self.ncen2(),
            self.ncen3(),
            self.ncen4(),
            self.ncen5(),
            self.ncen6(),
            self.ncen7(),
            self.ncen8(),
            self.ncen9()
        )
    }
}
#[doc = "SCT Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "SCT Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn unify(&self) -> super::vals::Unify {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Unify::from_bits(val as u8)
    }
    #[doc = "SCT Operation"]
    #[inline(always)]
    pub const fn set_unify(&mut self, val: super::vals::Unify) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SCT Clock Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn clkmode(&self) -> super::vals::Clkmode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Clkmode::from_bits(val as u8)
    }
    #[doc = "SCT Clock Mode"]
    #[inline(always)]
    pub const fn set_clkmode(&mut self, val: super::vals::Clkmode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SCT Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cksel(&self) -> super::vals::Cksel {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Cksel::from_bits(val as u8)
    }
    #[doc = "SCT Clock Select"]
    #[inline(always)]
    pub const fn set_cksel(&mut self, val: super::vals::Cksel) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "No Reload Lower Match"]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_l(&self) -> super::vals::NoreloadL {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::NoreloadL::from_bits(val as u8)
    }
    #[doc = "No Reload Lower Match"]
    #[inline(always)]
    pub const fn set_noreload_l(&mut self, val: super::vals::NoreloadL) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "No Reload Higher Match"]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_h(&self) -> super::vals::NoreloadH {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::NoreloadH::from_bits(val as u8)
    }
    #[doc = "No Reload Higher Match"]
    #[inline(always)]
    pub const fn set_noreload_h(&mut self, val: super::vals::NoreloadH) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Input Synchronization"]
    #[must_use]
    #[inline(always)]
    pub const fn insync(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "Input Synchronization"]
    #[inline(always)]
    pub const fn set_insync(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "Auto Limit Lower"]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_l(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Limit Lower"]
    #[inline(always)]
    pub const fn set_autolimit_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Auto Limit Higher"]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Limit Higher"]
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
            "Config {{ unify: {:?}, clkmode: {:?}, cksel: {:?}, noreload_l: {:?}, noreload_h: {:?}, insync: {=u8:?}, autolimit_l: {=bool:?}, autolimit_h: {=bool:?} }}",
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
#[doc = "Conflict Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conflag(pub u32);
impl Conflag {
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "No Change Conflict Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag"]
    #[inline(always)]
    pub const fn set_ncflag9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Error Low or Unified"]
    #[must_use]
    #[inline(always)]
    pub const fn buserrl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Low or Unified"]
    #[inline(always)]
    pub const fn set_buserrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Bus Error High"]
    #[must_use]
    #[inline(always)]
    pub const fn buserrh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error High"]
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
            .field("ncflag0", &self.ncflag0())
            .field("ncflag1", &self.ncflag1())
            .field("ncflag2", &self.ncflag2())
            .field("ncflag3", &self.ncflag3())
            .field("ncflag4", &self.ncflag4())
            .field("ncflag5", &self.ncflag5())
            .field("ncflag6", &self.ncflag6())
            .field("ncflag7", &self.ncflag7())
            .field("ncflag8", &self.ncflag8())
            .field("ncflag9", &self.ncflag9())
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
            "Conflag {{ ncflag0: {=bool:?}, ncflag1: {=bool:?}, ncflag2: {=bool:?}, ncflag3: {=bool:?}, ncflag4: {=bool:?}, ncflag5: {=bool:?}, ncflag6: {=bool:?}, ncflag7: {=bool:?}, ncflag8: {=bool:?}, ncflag9: {=bool:?}, buserrl: {=bool:?}, buserrh: {=bool:?} }}",
            self.ncflag0(),
            self.ncflag1(),
            self.ncflag2(),
            self.ncflag3(),
            self.ncflag4(),
            self.ncflag5(),
            self.ncflag6(),
            self.ncflag7(),
            self.ncflag8(),
            self.ncflag9(),
            self.buserrl(),
            self.buserrh()
        )
    }
}
#[doc = "Counter Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "Counter Low"]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Low"]
    #[inline(always)]
    pub const fn set_ctr_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter High"]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter High"]
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
#[doc = "SCT Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Down Counter Low"]
    #[must_use]
    #[inline(always)]
    pub const fn down_l(&self) -> super::vals::DownL {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DownL::from_bits(val as u8)
    }
    #[doc = "Down Counter Low"]
    #[inline(always)]
    pub const fn set_down_l(&mut self, val: super::vals::DownL) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop Counter Low"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Counter Low"]
    #[inline(always)]
    pub const fn set_stop_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Halt Counter Low"]
    #[must_use]
    #[inline(always)]
    pub const fn halt_l(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Halt Counter Low"]
    #[inline(always)]
    pub const fn set_halt_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Counter Low"]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_l(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Counter Low"]
    #[inline(always)]
    pub const fn set_clrctr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bidirectional Select Low"]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_l(&self) -> super::vals::BidirL {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BidirL::from_bits(val as u8)
    }
    #[doc = "Bidirectional Select Low"]
    #[inline(always)]
    pub const fn set_bidir_l(&mut self, val: super::vals::BidirL) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Prescaler for Low Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_l(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler for Low Counter"]
    #[inline(always)]
    pub const fn set_pre_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "Down Counter High"]
    #[must_use]
    #[inline(always)]
    pub const fn down_h(&self) -> super::vals::DownH {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DownH::from_bits(val as u8)
    }
    #[doc = "Down Counter High"]
    #[inline(always)]
    pub const fn set_down_h(&mut self, val: super::vals::DownH) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Stop Counter High"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_h(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Counter High"]
    #[inline(always)]
    pub const fn set_stop_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Halt Counter High"]
    #[must_use]
    #[inline(always)]
    pub const fn halt_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Halt Counter High"]
    #[inline(always)]
    pub const fn set_halt_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Clear Counter High"]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_h(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Counter High"]
    #[inline(always)]
    pub const fn set_clrctr_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bidirectional Select High"]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_h(&self) -> super::vals::BidirH {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::BidirH::from_bits(val as u8)
    }
    #[doc = "Bidirectional Select High"]
    #[inline(always)]
    pub const fn set_bidir_h(&mut self, val: super::vals::BidirH) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Prescaler for High Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_h(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler for High Counter"]
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
            "Ctrl {{ down_l: {:?}, stop_l: {=bool:?}, halt_l: {=bool:?}, clrctr_l: {=bool:?}, bidir_l: {:?}, pre_l: {=u8:?}, down_h: {:?}, stop_h: {=bool:?}, halt_h: {=bool:?}, clrctr_h: {=bool:?}, bidir_h: {:?}, pre_h: {=u8:?} }}",
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
#[doc = "Dither Condition"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dither(pub u32);
impl Dither {
    #[doc = "Dither Low"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Dither Low"]
    #[inline(always)]
    pub const fn set_dither_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Dither High"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Dither High"]
    #[inline(always)]
    pub const fn set_dither_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dither {
    #[inline(always)]
    fn default() -> Dither {
        Dither(0)
    }
}
impl core::fmt::Debug for Dither {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dither")
            .field("dither_l", &self.dither_l())
            .field("dither_h", &self.dither_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dither {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dither {{ dither_l: {=u16:?}, dither_h: {=u16:?} }}",
            self.dither_l(),
            self.dither_h()
        )
    }
}
#[doc = "DMA Request 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq0(pub u32);
impl Dmareq0 {
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA Request Low 0"]
    #[must_use]
    #[inline(always)]
    pub const fn drl0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Low 0"]
    #[inline(always)]
    pub const fn set_drl0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA Request 0 State"]
    #[must_use]
    #[inline(always)]
    pub const fn drq0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request 0 State"]
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
            .field("dev_1", &self.dev_1())
            .field("dev_2", &self.dev_2())
            .field("dev_3", &self.dev_3())
            .field("dev_4", &self.dev_4())
            .field("dev_5", &self.dev_5())
            .field("dev_6", &self.dev_6())
            .field("dev_7", &self.dev_7())
            .field("dev_8", &self.dev_8())
            .field("dev_9", &self.dev_9())
            .field("dev_10", &self.dev_10())
            .field("dev_11", &self.dev_11())
            .field("dev_12", &self.dev_12())
            .field("dev_13", &self.dev_13())
            .field("dev_14", &self.dev_14())
            .field("dev_15", &self.dev_15())
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
            "Dmareq0 {{ dev_0: {=bool:?}, dev_1: {=bool:?}, dev_2: {=bool:?}, dev_3: {=bool:?}, dev_4: {=bool:?}, dev_5: {=bool:?}, dev_6: {=bool:?}, dev_7: {=bool:?}, dev_8: {=bool:?}, dev_9: {=bool:?}, dev_10: {=bool:?}, dev_11: {=bool:?}, dev_12: {=bool:?}, dev_13: {=bool:?}, dev_14: {=bool:?}, dev_15: {=bool:?}, drl0: {=bool:?}, drq0: {=bool:?} }}",
            self.dev_0(),
            self.dev_1(),
            self.dev_2(),
            self.dev_3(),
            self.dev_4(),
            self.dev_5(),
            self.dev_6(),
            self.dev_7(),
            self.dev_8(),
            self.dev_9(),
            self.dev_10(),
            self.dev_11(),
            self.dev_12(),
            self.dev_13(),
            self.dev_14(),
            self.dev_15(),
            self.drl0(),
            self.drq0()
        )
    }
}
#[doc = "DMA Request 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq1(pub u32);
impl Dmareq1 {
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA Request Event"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event"]
    #[inline(always)]
    pub const fn set_dev_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA Request Low 1"]
    #[must_use]
    #[inline(always)]
    pub const fn drl1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Low 1"]
    #[inline(always)]
    pub const fn set_drl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA Request 1 State"]
    #[must_use]
    #[inline(always)]
    pub const fn drq1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request 1 State"]
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
            .field("dev_0", &self.dev_0())
            .field("dev_1", &self.dev_1())
            .field("dev_2", &self.dev_2())
            .field("dev_3", &self.dev_3())
            .field("dev_4", &self.dev_4())
            .field("dev_5", &self.dev_5())
            .field("dev_6", &self.dev_6())
            .field("dev_7", &self.dev_7())
            .field("dev_8", &self.dev_8())
            .field("dev_9", &self.dev_9())
            .field("dev_10", &self.dev_10())
            .field("dev_11", &self.dev_11())
            .field("dev_12", &self.dev_12())
            .field("dev_13", &self.dev_13())
            .field("dev_14", &self.dev_14())
            .field("dev_15", &self.dev_15())
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
            "Dmareq1 {{ dev_0: {=bool:?}, dev_1: {=bool:?}, dev_2: {=bool:?}, dev_3: {=bool:?}, dev_4: {=bool:?}, dev_5: {=bool:?}, dev_6: {=bool:?}, dev_7: {=bool:?}, dev_8: {=bool:?}, dev_9: {=bool:?}, dev_10: {=bool:?}, dev_11: {=bool:?}, dev_12: {=bool:?}, dev_13: {=bool:?}, dev_14: {=bool:?}, dev_15: {=bool:?}, drl1: {=bool:?}, drq1: {=bool:?} }}",
            self.dev_0(),
            self.dev_1(),
            self.dev_2(),
            self.dev_3(),
            self.dev_4(),
            self.dev_5(),
            self.dev_6(),
            self.dev_7(),
            self.dev_8(),
            self.dev_9(),
            self.dev_10(),
            self.dev_11(),
            self.dev_12(),
            self.dev_13(),
            self.dev_14(),
            self.dev_15(),
            self.drl1(),
            self.drq1()
        )
    }
}
#[doc = "Event n Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvCtrl(pub u32);
impl EvCtrl {
    #[doc = "Match Select"]
    #[must_use]
    #[inline(always)]
    pub const fn matchsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Match Select"]
    #[inline(always)]
    pub const fn set_matchsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "High Event"]
    #[must_use]
    #[inline(always)]
    pub const fn hevent(&self) -> super::vals::Hevent {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hevent::from_bits(val as u8)
    }
    #[doc = "High Event"]
    #[inline(always)]
    pub const fn set_hevent(&mut self, val: super::vals::Hevent) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Input and Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn outsel(&self) -> super::vals::Outsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Outsel::from_bits(val as u8)
    }
    #[doc = "Input and Output Select"]
    #[inline(always)]
    pub const fn set_outsel(&mut self, val: super::vals::Outsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Input or Output Signal Select"]
    #[must_use]
    #[inline(always)]
    pub const fn iosel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Input or Output Signal Select"]
    #[inline(always)]
    pub const fn set_iosel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "Input or Output Condition"]
    #[must_use]
    #[inline(always)]
    pub const fn iocond(&self) -> super::vals::Iocond {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Iocond::from_bits(val as u8)
    }
    #[doc = "Input or Output Condition"]
    #[inline(always)]
    pub const fn set_iocond(&mut self, val: super::vals::Iocond) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Combination Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn combmode(&self) -> super::vals::Combmode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Combmode::from_bits(val as u8)
    }
    #[doc = "Combination Mode"]
    #[inline(always)]
    pub const fn set_combmode(&mut self, val: super::vals::Combmode) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "State Load"]
    #[must_use]
    #[inline(always)]
    pub const fn stateld(&self) -> super::vals::Stateld {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Stateld::from_bits(val as u8)
    }
    #[doc = "State Load"]
    #[inline(always)]
    pub const fn set_stateld(&mut self, val: super::vals::Stateld) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "State Value"]
    #[must_use]
    #[inline(always)]
    pub const fn statev(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "State Value"]
    #[inline(always)]
    pub const fn set_statev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "Match Mem"]
    #[must_use]
    #[inline(always)]
    pub const fn matchmem(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Match Mem"]
    #[inline(always)]
    pub const fn set_matchmem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Direction {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Direction::from_bits(val as u8)
    }
    #[doc = "Direction"]
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
#[doc = "Event n State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvState(pub u32);
impl EvState {
    #[doc = "Event State Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn statemskn(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Event State Mask"]
    #[inline(always)]
    pub const fn set_statemskn(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "EvState {{ statemskn: {=u32:?} }}", self.statemskn())
    }
}
#[doc = "Event Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Even(pub u32);
impl Even {
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ien15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ien15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        f.debug_struct("Even")
            .field("ien0", &self.ien0())
            .field("ien1", &self.ien1())
            .field("ien2", &self.ien2())
            .field("ien3", &self.ien3())
            .field("ien4", &self.ien4())
            .field("ien5", &self.ien5())
            .field("ien6", &self.ien6())
            .field("ien7", &self.ien7())
            .field("ien8", &self.ien8())
            .field("ien9", &self.ien9())
            .field("ien10", &self.ien10())
            .field("ien11", &self.ien11())
            .field("ien12", &self.ien12())
            .field("ien13", &self.ien13())
            .field("ien14", &self.ien14())
            .field("ien15", &self.ien15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Even {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Even {{ ien0: {=bool:?}, ien1: {=bool:?}, ien2: {=bool:?}, ien3: {=bool:?}, ien4: {=bool:?}, ien5: {=bool:?}, ien6: {=bool:?}, ien7: {=bool:?}, ien8: {=bool:?}, ien9: {=bool:?}, ien10: {=bool:?}, ien11: {=bool:?}, ien12: {=bool:?}, ien13: {=bool:?}, ien14: {=bool:?}, ien15: {=bool:?} }}",
            self.ien0(),
            self.ien1(),
            self.ien2(),
            self.ien3(),
            self.ien4(),
            self.ien5(),
            self.ien6(),
            self.ien7(),
            self.ien8(),
            self.ien9(),
            self.ien10(),
            self.ien11(),
            self.ien12(),
            self.ien13(),
            self.ien14(),
            self.ien15()
        )
    }
}
#[doc = "Event Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evflag(pub u32);
impl Evflag {
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Event Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flag15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn set_flag15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            .field("flag0", &self.flag0())
            .field("flag1", &self.flag1())
            .field("flag2", &self.flag2())
            .field("flag3", &self.flag3())
            .field("flag4", &self.flag4())
            .field("flag5", &self.flag5())
            .field("flag6", &self.flag6())
            .field("flag7", &self.flag7())
            .field("flag8", &self.flag8())
            .field("flag9", &self.flag9())
            .field("flag10", &self.flag10())
            .field("flag11", &self.flag11())
            .field("flag12", &self.flag12())
            .field("flag13", &self.flag13())
            .field("flag14", &self.flag14())
            .field("flag15", &self.flag15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Evflag {{ flag0: {=bool:?}, flag1: {=bool:?}, flag2: {=bool:?}, flag3: {=bool:?}, flag4: {=bool:?}, flag5: {=bool:?}, flag6: {=bool:?}, flag7: {=bool:?}, flag8: {=bool:?}, flag9: {=bool:?}, flag10: {=bool:?}, flag11: {=bool:?}, flag12: {=bool:?}, flag13: {=bool:?}, flag14: {=bool:?}, flag15: {=bool:?} }}",
            self.flag0(),
            self.flag1(),
            self.flag2(),
            self.flag3(),
            self.flag4(),
            self.flag5(),
            self.flag6(),
            self.flag7(),
            self.flag8(),
            self.flag9(),
            self.flag10(),
            self.flag11(),
            self.flag12(),
            self.flag13(),
            self.flag14(),
            self.flag15()
        )
    }
}
#[doc = "Fractional Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fracmat(pub u32);
impl Fracmat {
    #[doc = "Fractional Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn fracmat_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Fractional Match Low"]
    #[inline(always)]
    pub const fn set_fracmat_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Fractional Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn fracmat_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Fractional Match High"]
    #[inline(always)]
    pub const fn set_fracmat_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fracmat {
    #[inline(always)]
    fn default() -> Fracmat {
        Fracmat(0)
    }
}
impl core::fmt::Debug for Fracmat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fracmat")
            .field("fracmat_l", &self.fracmat_l())
            .field("fracmat_h", &self.fracmat_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fracmat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fracmat {{ fracmat_l: {=u8:?}, fracmat_h: {=u8:?} }}",
            self.fracmat_l(),
            self.fracmat_h()
        )
    }
}
#[doc = "Fractional Match Reload"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fracmatrel(pub u32);
impl Fracmatrel {
    #[doc = "Reload Fractional Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn relfrac_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Reload Fractional Match Low"]
    #[inline(always)]
    pub const fn set_relfrac_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reload Fractional Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn relfrac_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Reload Fractional Match High"]
    #[inline(always)]
    pub const fn set_relfrac_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fracmatrel {
    #[inline(always)]
    fn default() -> Fracmatrel {
        Fracmatrel(0)
    }
}
impl core::fmt::Debug for Fracmatrel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fracmatrel")
            .field("relfrac_l", &self.relfrac_l())
            .field("relfrac_h", &self.relfrac_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fracmatrel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fracmatrel {{ relfrac_l: {=u8:?}, relfrac_h: {=u8:?} }}",
            self.relfrac_l(),
            self.relfrac_h()
        )
    }
}
#[doc = "Halt Event Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halt(pub u32);
impl Halt {
    #[doc = "Halt Event Low"]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Halt Event Low"]
    #[inline(always)]
    pub const fn set_haltmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Halt Event High"]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Halt Event High"]
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
#[doc = "Input State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "Input 0 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state"]
    #[inline(always)]
    pub const fn set_ain0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Input 1 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Input 1 state"]
    #[inline(always)]
    pub const fn set_ain1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Input 2 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Input 2 state"]
    #[inline(always)]
    pub const fn set_ain2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Input 3 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Input 3 state"]
    #[inline(always)]
    pub const fn set_ain3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Input 4 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Input 4 state"]
    #[inline(always)]
    pub const fn set_ain4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Input 5 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Input 5 state"]
    #[inline(always)]
    pub const fn set_ain5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Input 6 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input 6 state"]
    #[inline(always)]
    pub const fn set_ain6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Input 7 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input 7 state"]
    #[inline(always)]
    pub const fn set_ain7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Input 8 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Input 8 state"]
    #[inline(always)]
    pub const fn set_ain8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Input 9 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Input 9 state"]
    #[inline(always)]
    pub const fn set_ain9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Input 10 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Input 10 state"]
    #[inline(always)]
    pub const fn set_ain10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Input 11 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Input 11 state"]
    #[inline(always)]
    pub const fn set_ain11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Input 12 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Input 12 state"]
    #[inline(always)]
    pub const fn set_ain12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Input 13 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Input 13 state"]
    #[inline(always)]
    pub const fn set_ain13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Input 14 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Input 14 state"]
    #[inline(always)]
    pub const fn set_ain14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Input 15 state"]
    #[must_use]
    #[inline(always)]
    pub const fn ain15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Input 15 state"]
    #[inline(always)]
    pub const fn set_ain15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Input 0 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state"]
    #[inline(always)]
    pub const fn set_sin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Input 1 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Input 1 state"]
    #[inline(always)]
    pub const fn set_sin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Input 2 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Input 2 state"]
    #[inline(always)]
    pub const fn set_sin2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Input 3 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Input 3 state"]
    #[inline(always)]
    pub const fn set_sin3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Input 4 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Input 4 state"]
    #[inline(always)]
    pub const fn set_sin4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Input 5 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Input 5 state"]
    #[inline(always)]
    pub const fn set_sin5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Input 6 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Input 6 state"]
    #[inline(always)]
    pub const fn set_sin6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Input 7 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Input 7 state"]
    #[inline(always)]
    pub const fn set_sin7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Input 8 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Input 8 state"]
    #[inline(always)]
    pub const fn set_sin8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Input 9 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Input 9 state"]
    #[inline(always)]
    pub const fn set_sin9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Input 10 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Input 10 state"]
    #[inline(always)]
    pub const fn set_sin10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Input 11 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Input 11 state"]
    #[inline(always)]
    pub const fn set_sin11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Input 12 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Input 12 state"]
    #[inline(always)]
    pub const fn set_sin12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Input 13 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Input 13 state"]
    #[inline(always)]
    pub const fn set_sin13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Input 14 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Input 14 state"]
    #[inline(always)]
    pub const fn set_sin14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Input 15 state"]
    #[must_use]
    #[inline(always)]
    pub const fn sin15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Input 15 state"]
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
#[doc = "SCT Limit Event Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Limit(pub u32);
impl Limit {
    #[doc = "Limit Event Counter Low"]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Limit Event Counter Low"]
    #[inline(always)]
    pub const fn set_limmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Limit Event Counter High"]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Limit Event Counter High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match0(pub u32);
impl Match0 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match1(pub u32);
impl Match1 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match10(pub u32);
impl Match10 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match11(pub u32);
impl Match11 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match12(pub u32);
impl Match12 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match13(pub u32);
impl Match13 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match14(pub u32);
impl Match14 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match15(pub u32);
impl Match15 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match2(pub u32);
impl Match2 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match3(pub u32);
impl Match3 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match4(pub u32);
impl Match4 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match5(pub u32);
impl Match5 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match6(pub u32);
impl Match6 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match7(pub u32);
impl Match7 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match8(pub u32);
impl Match8 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match9(pub u32);
impl Match9 {
    #[doc = "Match Low"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low"]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High"]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel0(pub u32);
impl Matchrel0 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel1(pub u32);
impl Matchrel1 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel10(pub u32);
impl Matchrel10 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel11(pub u32);
impl Matchrel11 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel12(pub u32);
impl Matchrel12 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel13(pub u32);
impl Matchrel13 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel14(pub u32);
impl Matchrel14 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel15(pub u32);
impl Matchrel15 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel2(pub u32);
impl Matchrel2 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel3(pub u32);
impl Matchrel3 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel4(pub u32);
impl Matchrel4 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel5(pub u32);
impl Matchrel5 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel6(pub u32);
impl Matchrel6 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel7(pub u32);
impl Matchrel7 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel8(pub u32);
impl Matchrel8 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Match Reload Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel9(pub u32);
impl Matchrel9 {
    #[doc = "Reload Low"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low"]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High"]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High"]
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
#[doc = "Output n Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClr(pub u32);
impl OutClr {
    #[doc = "Clear Output"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Clear Output"]
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
#[doc = "Output n Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet(pub u32);
impl OutSet {
    #[doc = "Set Output"]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set Output"]
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
#[doc = "Output State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Output(pub u32);
impl Output {
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out0(&self) -> super::vals::Out0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Out0::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out0(&mut self, val: super::vals::Out0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out1(&self) -> super::vals::Out1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Out1::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out1(&mut self, val: super::vals::Out1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out2(&self) -> super::vals::Out2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Out2::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out2(&mut self, val: super::vals::Out2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out3(&self) -> super::vals::Out3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Out3::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out3(&mut self, val: super::vals::Out3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out4(&self) -> super::vals::Out4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Out4::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out4(&mut self, val: super::vals::Out4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out5(&self) -> super::vals::Out5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Out5::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out5(&mut self, val: super::vals::Out5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out6(&self) -> super::vals::Out6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Out6::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out6(&mut self, val: super::vals::Out6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out7(&self) -> super::vals::Out7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Out7::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out7(&mut self, val: super::vals::Out7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out8(&self) -> super::vals::Out8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Out8::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out8(&mut self, val: super::vals::Out8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Output Low and High"]
    #[must_use]
    #[inline(always)]
    pub const fn out9(&self) -> super::vals::Out9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Out9::from_bits(val as u8)
    }
    #[doc = "Output Low and High"]
    #[inline(always)]
    pub const fn set_out9(&mut self, val: super::vals::Out9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
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
        f.debug_struct("Output")
            .field("out0", &self.out0())
            .field("out1", &self.out1())
            .field("out2", &self.out2())
            .field("out3", &self.out3())
            .field("out4", &self.out4())
            .field("out5", &self.out5())
            .field("out6", &self.out6())
            .field("out7", &self.out7())
            .field("out8", &self.out8())
            .field("out9", &self.out9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Output {{ out0: {:?}, out1: {:?}, out2: {:?}, out3: {:?}, out4: {:?}, out5: {:?}, out6: {:?}, out7: {:?}, out8: {:?}, out9: {:?} }}",
            self.out0(),
            self.out1(),
            self.out2(),
            self.out3(),
            self.out4(),
            self.out5(),
            self.out6(),
            self.out7(),
            self.out8(),
            self.out9()
        )
    }
}
#[doc = "Output Counter Direction Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputdirctrl(pub u32);
impl Outputdirctrl {
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr0(&self) -> super::vals::Setclr0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Setclr0::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr0(&mut self, val: super::vals::Setclr0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr1(&self) -> super::vals::Setclr1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Setclr1::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr1(&mut self, val: super::vals::Setclr1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr2(&self) -> super::vals::Setclr2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Setclr2::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr2(&mut self, val: super::vals::Setclr2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr3(&self) -> super::vals::Setclr3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Setclr3::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr3(&mut self, val: super::vals::Setclr3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr4(&self) -> super::vals::Setclr4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Setclr4::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr4(&mut self, val: super::vals::Setclr4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr5(&self) -> super::vals::Setclr5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Setclr5::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr5(&mut self, val: super::vals::Setclr5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr6(&self) -> super::vals::Setclr6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Setclr6::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr6(&mut self, val: super::vals::Setclr6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr7(&self) -> super::vals::Setclr7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Setclr7::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr7(&mut self, val: super::vals::Setclr7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr8(&self) -> super::vals::Setclr8 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Setclr8::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr8(&mut self, val: super::vals::Setclr8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Set and Clear Operation on Output"]
    #[must_use]
    #[inline(always)]
    pub const fn setclr9(&self) -> super::vals::Setclr9 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Setclr9::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output"]
    #[inline(always)]
    pub const fn set_setclr9(&mut self, val: super::vals::Setclr9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outputdirctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outputdirctrl {{ setclr0: {:?}, setclr1: {:?}, setclr2: {:?}, setclr3: {:?}, setclr4: {:?}, setclr5: {:?}, setclr6: {:?}, setclr7: {:?}, setclr8: {:?}, setclr9: {:?} }}",
            self.setclr0(),
            self.setclr1(),
            self.setclr2(),
            self.setclr3(),
            self.setclr4(),
            self.setclr5(),
            self.setclr6(),
            self.setclr7(),
            self.setclr8(),
            self.setclr9()
        )
    }
}
#[doc = "Match and Capture Register Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regmode(pub u32);
impl Regmode {
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l0(&self) -> super::vals::RegmodL0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RegmodL0::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l0(&mut self, val: super::vals::RegmodL0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l1(&self) -> super::vals::RegmodL1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RegmodL1::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l1(&mut self, val: super::vals::RegmodL1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l2(&self) -> super::vals::RegmodL2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RegmodL2::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l2(&mut self, val: super::vals::RegmodL2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l3(&self) -> super::vals::RegmodL3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RegmodL3::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l3(&mut self, val: super::vals::RegmodL3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l4(&self) -> super::vals::RegmodL4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RegmodL4::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l4(&mut self, val: super::vals::RegmodL4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l5(&self) -> super::vals::RegmodL5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RegmodL5::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l5(&mut self, val: super::vals::RegmodL5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l6(&self) -> super::vals::RegmodL6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RegmodL6::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l6(&mut self, val: super::vals::RegmodL6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l7(&self) -> super::vals::RegmodL7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RegmodL7::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l7(&mut self, val: super::vals::RegmodL7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l8(&self) -> super::vals::RegmodL8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RegmodL8::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l8(&mut self, val: super::vals::RegmodL8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l9(&self) -> super::vals::RegmodL9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RegmodL9::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l9(&mut self, val: super::vals::RegmodL9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l10(&self) -> super::vals::RegmodL10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::RegmodL10::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l10(&mut self, val: super::vals::RegmodL10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l11(&self) -> super::vals::RegmodL11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::RegmodL11::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l11(&mut self, val: super::vals::RegmodL11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l12(&self) -> super::vals::RegmodL12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::RegmodL12::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l12(&mut self, val: super::vals::RegmodL12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l13(&self) -> super::vals::RegmodL13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::RegmodL13::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l13(&mut self, val: super::vals::RegmodL13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l14(&self) -> super::vals::RegmodL14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::RegmodL14::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l14(&mut self, val: super::vals::RegmodL14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Register Mode Low"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l15(&self) -> super::vals::RegmodL15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::RegmodL15::from_bits(val as u8)
    }
    #[doc = "Register Mode Low"]
    #[inline(always)]
    pub const fn set_regmod_l15(&mut self, val: super::vals::RegmodL15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h0(&self) -> super::vals::RegmodH0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::RegmodH0::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h0(&mut self, val: super::vals::RegmodH0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h1(&self) -> super::vals::RegmodH1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::RegmodH1::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h1(&mut self, val: super::vals::RegmodH1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h2(&self) -> super::vals::RegmodH2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::RegmodH2::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h2(&mut self, val: super::vals::RegmodH2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h3(&self) -> super::vals::RegmodH3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::RegmodH3::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h3(&mut self, val: super::vals::RegmodH3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h4(&self) -> super::vals::RegmodH4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::RegmodH4::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h4(&mut self, val: super::vals::RegmodH4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h5(&self) -> super::vals::RegmodH5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::RegmodH5::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h5(&mut self, val: super::vals::RegmodH5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h6(&self) -> super::vals::RegmodH6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RegmodH6::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h6(&mut self, val: super::vals::RegmodH6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h7(&self) -> super::vals::RegmodH7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RegmodH7::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h7(&mut self, val: super::vals::RegmodH7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h8(&self) -> super::vals::RegmodH8 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::RegmodH8::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h8(&mut self, val: super::vals::RegmodH8) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h9(&self) -> super::vals::RegmodH9 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::RegmodH9::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h9(&mut self, val: super::vals::RegmodH9) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h10(&self) -> super::vals::RegmodH10 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::RegmodH10::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h10(&mut self, val: super::vals::RegmodH10) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h11(&self) -> super::vals::RegmodH11 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::RegmodH11::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h11(&mut self, val: super::vals::RegmodH11) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h12(&self) -> super::vals::RegmodH12 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::RegmodH12::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h12(&mut self, val: super::vals::RegmodH12) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h13(&self) -> super::vals::RegmodH13 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::RegmodH13::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h13(&mut self, val: super::vals::RegmodH13) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h14(&self) -> super::vals::RegmodH14 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::RegmodH14::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h14(&mut self, val: super::vals::RegmodH14) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Register Mode High"]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h15(&self) -> super::vals::RegmodH15 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::RegmodH15::from_bits(val as u8)
    }
    #[doc = "Register Mode High"]
    #[inline(always)]
    pub const fn set_regmod_h15(&mut self, val: super::vals::RegmodH15) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regmod_l0", &self.regmod_l0())
            .field("regmod_l1", &self.regmod_l1())
            .field("regmod_l2", &self.regmod_l2())
            .field("regmod_l3", &self.regmod_l3())
            .field("regmod_l4", &self.regmod_l4())
            .field("regmod_l5", &self.regmod_l5())
            .field("regmod_l6", &self.regmod_l6())
            .field("regmod_l7", &self.regmod_l7())
            .field("regmod_l8", &self.regmod_l8())
            .field("regmod_l9", &self.regmod_l9())
            .field("regmod_l10", &self.regmod_l10())
            .field("regmod_l11", &self.regmod_l11())
            .field("regmod_l12", &self.regmod_l12())
            .field("regmod_l13", &self.regmod_l13())
            .field("regmod_l14", &self.regmod_l14())
            .field("regmod_l15", &self.regmod_l15())
            .field("regmod_h0", &self.regmod_h0())
            .field("regmod_h1", &self.regmod_h1())
            .field("regmod_h2", &self.regmod_h2())
            .field("regmod_h3", &self.regmod_h3())
            .field("regmod_h4", &self.regmod_h4())
            .field("regmod_h5", &self.regmod_h5())
            .field("regmod_h6", &self.regmod_h6())
            .field("regmod_h7", &self.regmod_h7())
            .field("regmod_h8", &self.regmod_h8())
            .field("regmod_h9", &self.regmod_h9())
            .field("regmod_h10", &self.regmod_h10())
            .field("regmod_h11", &self.regmod_h11())
            .field("regmod_h12", &self.regmod_h12())
            .field("regmod_h13", &self.regmod_h13())
            .field("regmod_h14", &self.regmod_h14())
            .field("regmod_h15", &self.regmod_h15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Regmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Regmode {{ regmod_l0: {:?}, regmod_l1: {:?}, regmod_l2: {:?}, regmod_l3: {:?}, regmod_l4: {:?}, regmod_l5: {:?}, regmod_l6: {:?}, regmod_l7: {:?}, regmod_l8: {:?}, regmod_l9: {:?}, regmod_l10: {:?}, regmod_l11: {:?}, regmod_l12: {:?}, regmod_l13: {:?}, regmod_l14: {:?}, regmod_l15: {:?}, regmod_h0: {:?}, regmod_h1: {:?}, regmod_h2: {:?}, regmod_h3: {:?}, regmod_h4: {:?}, regmod_h5: {:?}, regmod_h6: {:?}, regmod_h7: {:?}, regmod_h8: {:?}, regmod_h9: {:?}, regmod_h10: {:?}, regmod_h11: {:?}, regmod_h12: {:?}, regmod_h13: {:?}, regmod_h14: {:?}, regmod_h15: {:?} }}",
            self.regmod_l0(),
            self.regmod_l1(),
            self.regmod_l2(),
            self.regmod_l3(),
            self.regmod_l4(),
            self.regmod_l5(),
            self.regmod_l6(),
            self.regmod_l7(),
            self.regmod_l8(),
            self.regmod_l9(),
            self.regmod_l10(),
            self.regmod_l11(),
            self.regmod_l12(),
            self.regmod_l13(),
            self.regmod_l14(),
            self.regmod_l15(),
            self.regmod_h0(),
            self.regmod_h1(),
            self.regmod_h2(),
            self.regmod_h3(),
            self.regmod_h4(),
            self.regmod_h5(),
            self.regmod_h6(),
            self.regmod_h7(),
            self.regmod_h8(),
            self.regmod_h9(),
            self.regmod_h10(),
            self.regmod_h11(),
            self.regmod_h12(),
            self.regmod_h13(),
            self.regmod_h14(),
            self.regmod_h15()
        )
    }
}
#[doc = "Output Conflict Resolution"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res(pub u32);
impl Res {
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o0res(&self) -> super::vals::O0res {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::O0res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o0res(&mut self, val: super::vals::O0res) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o1res(&self) -> super::vals::O1res {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::O1res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o1res(&mut self, val: super::vals::O1res) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o2res(&self) -> super::vals::O2res {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::O2res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o2res(&mut self, val: super::vals::O2res) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o3res(&self) -> super::vals::O3res {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::O3res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o3res(&mut self, val: super::vals::O3res) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o4res(&self) -> super::vals::O4res {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::O4res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o4res(&mut self, val: super::vals::O4res) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o5res(&self) -> super::vals::O5res {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::O5res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o5res(&mut self, val: super::vals::O5res) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o6res(&self) -> super::vals::O6res {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::O6res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o6res(&mut self, val: super::vals::O6res) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o7res(&self) -> super::vals::O7res {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::O7res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o7res(&mut self, val: super::vals::O7res) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o8res(&self) -> super::vals::O8res {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::O8res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o8res(&mut self, val: super::vals::O8res) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Output Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn o9res(&self) -> super::vals::O9res {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::O9res::from_bits(val as u8)
    }
    #[doc = "Output Resolution"]
    #[inline(always)]
    pub const fn set_o9res(&mut self, val: super::vals::O9res) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Res {{ o0res: {:?}, o1res: {:?}, o2res: {:?}, o3res: {:?}, o4res: {:?}, o5res: {:?}, o6res: {:?}, o7res: {:?}, o8res: {:?}, o9res: {:?} }}",
            self.o0res(),
            self.o1res(),
            self.o2res(),
            self.o3res(),
            self.o4res(),
            self.o5res(),
            self.o6res(),
            self.o7res(),
            self.o8res(),
            self.o9res()
        )
    }
}
#[doc = "Start Event Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "Start Event Low"]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Start Event Low"]
    #[inline(always)]
    pub const fn set_startmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Start Event High"]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Start Event High"]
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
#[doc = "State Variable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "State Variable Low"]
    #[must_use]
    #[inline(always)]
    pub const fn state_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "State Variable Low"]
    #[inline(always)]
    pub const fn set_state_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "State Variable High"]
    #[must_use]
    #[inline(always)]
    pub const fn state_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "State Variable High"]
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
#[doc = "Stop Event Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "Stop Event Low"]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop Event Low"]
    #[inline(always)]
    pub const fn set_stopmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Stop Event High"]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop Event High"]
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
