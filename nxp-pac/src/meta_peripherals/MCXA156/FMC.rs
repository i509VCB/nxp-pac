#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "NPX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmc {
    ptr: *mut u8,
}
unsafe impl Send for Fmc {}
unsafe impl Sync for Fmc {}
impl Fmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data Remap."]
    #[inline(always)]
    pub const fn remap(self) -> crate::pac::common::Reg<Remap, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
#[doc = "Data Remap."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "Remap Lock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn remaplk(&self) -> Remaplk {
        let val = (self.0 >> 0usize) & 0x01;
        Remaplk::from_bits(val as u8)
    }
    #[doc = "Remap Lock Enable."]
    #[inline(always)]
    pub const fn set_remaplk(&mut self, val: Remaplk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LIM Remapping Address."]
    #[must_use]
    #[inline(always)]
    pub const fn lim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "LIM Remapping Address."]
    #[inline(always)]
    pub const fn set_lim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "LIMDP Remapping Address."]
    #[must_use]
    #[inline(always)]
    pub const fn limdp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "LIMDP Remapping Address."]
    #[inline(always)]
    pub const fn set_limdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Remap {
    #[inline(always)]
    fn default() -> Remap {
        Remap(0)
    }
}
impl core::fmt::Debug for Remap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remap")
            .field("remaplk", &self.remaplk())
            .field("lim", &self.lim())
            .field("limdp", &self.limdp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Remap {{ remaplk: {:?}, lim: {=u8:?}, limdp: {=u8:?} }}",
            self.remaplk(),
            self.lim(),
            self.limdp()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remaplk {
    #[doc = "Lock disabled: can write to REMAP."]
    LockDisabled = 0x0,
    #[doc = "Lock enabled: cannot write to REMAP."]
    LockEnabled = 0x01,
}
impl Remaplk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remaplk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remaplk {
    #[inline(always)]
    fn from(val: u8) -> Remaplk {
        Remaplk::from_bits(val)
    }
}
impl From<Remaplk> for u8 {
    #[inline(always)]
    fn from(val: Remaplk) -> u8 {
        Remaplk::to_bits(val)
    }
}
