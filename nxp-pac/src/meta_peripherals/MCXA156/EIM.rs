#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Error Injection Module."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eim {
    ptr: *mut u8,
}
unsafe impl Send for Eim {}
unsafe impl Sync for Eim {}
impl Eim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Error Injection Module Configuration Register."]
    #[inline(always)]
    pub const fn eimcr(self) -> crate::pac::common::Reg<Eimcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Error Injection Channel Enable register."]
    #[inline(always)]
    pub const fn eichen(self) -> crate::pac::common::Reg<Eichen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word0."]
    #[inline(always)]
    pub const fn eichd0_word0(
        self,
    ) -> crate::pac::common::Reg<Eichd0Word0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word1."]
    #[inline(always)]
    pub const fn eichd0_word1(
        self,
    ) -> crate::pac::common::Reg<Eichd0Word1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd0Word0(pub u32);
impl Eichd0Word0 {
    #[doc = "Checkbit Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Checkbit Mask."]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Eichd0Word0 {
    #[inline(always)]
    fn default() -> Eichd0Word0 {
        Eichd0Word0(0)
    }
}
impl core::fmt::Debug for Eichd0Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd0Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd0Word0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd0Word0 {{ chkbit_mask: {=u8:?} }}",
            self.chkbit_mask()
        )
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd0Word1(pub u32);
impl Eichd0Word1 {
    #[doc = "Data Mask Bytes 0-3."]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Mask Bytes 0-3."]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Eichd0Word1 {
    #[inline(always)]
    fn default() -> Eichd0Word1 {
        Eichd0Word1(0)
    }
}
impl core::fmt::Debug for Eichd0Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd0Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd0Word1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eichd0Word1 {{ b0_3data_mask: {=u32:?} }}",
            self.b0_3data_mask()
        )
    }
}
#[doc = "Error Injection Channel Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichen(pub u32);
impl Eichen {
    #[doc = "Error Injection Channel 0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eich0en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 0 Enable."]
    #[inline(always)]
    pub const fn set_eich0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eichen {
    #[inline(always)]
    fn default() -> Eichen {
        Eichen(0)
    }
}
impl core::fmt::Debug for Eichen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichen")
            .field("eich0en", &self.eich0en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eichen {{ eich0en: {=bool:?} }}", self.eich0en())
    }
}
#[doc = "Error Injection Module Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eimcr(pub u32);
impl Eimcr {
    #[doc = "Global Error Injection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn geien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Global Error Injection Enable."]
    #[inline(always)]
    pub const fn set_geien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Eimcr {
    #[inline(always)]
    fn default() -> Eimcr {
        Eimcr(0)
    }
}
impl core::fmt::Debug for Eimcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eimcr")
            .field("geien", &self.geien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eimcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eimcr {{ geien: {=bool:?} }}", self.geien())
    }
}
