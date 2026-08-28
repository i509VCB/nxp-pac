#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "AOI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoi {
    ptr: *mut u8,
}
unsafe impl Send for Aoi {}
unsafe impl Sync for Aoi {}
impl Aoi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0."]
    #[inline(always)]
    pub const fn bfcrt010(self) -> crate::pac::common::Reg<Bfcrt010, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0."]
    #[inline(always)]
    pub const fn bfcrt230(self) -> crate::pac::common::Reg<Bfcrt230, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT1."]
    #[inline(always)]
    pub const fn bfcrt011(self) -> crate::pac::common::Reg<Bfcrt011, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT1."]
    #[inline(always)]
    pub const fn bfcrt231(self) -> crate::pac::common::Reg<Bfcrt231, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT2."]
    #[inline(always)]
    pub const fn bfcrt012(self) -> crate::pac::common::Reg<Bfcrt012, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT2."]
    #[inline(always)]
    pub const fn bfcrt232(self) -> crate::pac::common::Reg<Bfcrt232, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT3."]
    #[inline(always)]
    pub const fn bfcrt013(self) -> crate::pac::common::Reg<Bfcrt013, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT3."]
    #[inline(always)]
    pub const fn bfcrt233(self) -> crate::pac::common::Reg<Bfcrt233, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt010(pub u16);
impl Bfcrt010 {
    #[doc = "Product Term 1, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> Bfcrt010Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt010Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: Bfcrt010Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> Bfcrt010Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt010Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: Bfcrt010Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> Bfcrt010Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt010Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: Bfcrt010Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> Bfcrt010Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt010Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: Bfcrt010Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> Bfcrt010Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt010Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: Bfcrt010Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> Bfcrt010Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt010Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: Bfcrt010Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> Bfcrt010Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt010Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: Bfcrt010Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> Bfcrt010Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt010Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: Bfcrt010Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt010 {
    #[inline(always)]
    fn default() -> Bfcrt010 {
        Bfcrt010(0)
    }
}
impl core::fmt::Debug for Bfcrt010 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt010")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt010 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt010 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt011(pub u16);
impl Bfcrt011 {
    #[doc = "Product Term 1, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> Bfcrt011Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt011Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: Bfcrt011Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> Bfcrt011Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt011Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: Bfcrt011Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> Bfcrt011Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt011Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: Bfcrt011Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> Bfcrt011Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt011Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: Bfcrt011Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> Bfcrt011Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt011Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: Bfcrt011Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> Bfcrt011Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt011Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: Bfcrt011Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> Bfcrt011Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt011Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: Bfcrt011Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> Bfcrt011Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt011Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: Bfcrt011Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt011 {
    #[inline(always)]
    fn default() -> Bfcrt011 {
        Bfcrt011(0)
    }
}
impl core::fmt::Debug for Bfcrt011 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt011")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt011 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt011 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt012(pub u16);
impl Bfcrt012 {
    #[doc = "Product Term 1, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> Bfcrt012Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt012Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: Bfcrt012Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> Bfcrt012Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt012Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: Bfcrt012Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> Bfcrt012Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt012Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: Bfcrt012Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> Bfcrt012Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt012Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: Bfcrt012Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> Bfcrt012Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt012Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: Bfcrt012Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> Bfcrt012Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt012Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: Bfcrt012Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> Bfcrt012Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt012Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: Bfcrt012Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> Bfcrt012Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt012Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: Bfcrt012Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt012 {
    #[inline(always)]
    fn default() -> Bfcrt012 {
        Bfcrt012(0)
    }
}
impl core::fmt::Debug for Bfcrt012 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt012")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt012 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt012 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt013(pub u16);
impl Bfcrt013 {
    #[doc = "Product Term 1, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> Bfcrt013Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt013Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: Bfcrt013Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> Bfcrt013Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt013Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: Bfcrt013Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> Bfcrt013Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt013Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: Bfcrt013Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> Bfcrt013Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt013Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: Bfcrt013Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> Bfcrt013Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt013Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: Bfcrt013Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> Bfcrt013Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt013Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: Bfcrt013Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> Bfcrt013Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt013Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: Bfcrt013Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> Bfcrt013Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt013Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: Bfcrt013Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt013 {
    #[inline(always)]
    fn default() -> Bfcrt013 {
        Bfcrt013(0)
    }
}
impl core::fmt::Debug for Bfcrt013 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt013")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt013 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt013 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt230(pub u16);
impl Bfcrt230 {
    #[doc = "Product Term 3, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> Bfcrt230Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt230Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: Bfcrt230Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> Bfcrt230Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt230Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: Bfcrt230Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> Bfcrt230Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt230Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: Bfcrt230Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> Bfcrt230Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt230Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: Bfcrt230Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> Bfcrt230Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt230Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: Bfcrt230Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> Bfcrt230Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt230Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: Bfcrt230Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> Bfcrt230Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt230Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: Bfcrt230Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> Bfcrt230Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt230Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: Bfcrt230Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt230 {
    #[inline(always)]
    fn default() -> Bfcrt230 {
        Bfcrt230(0)
    }
}
impl core::fmt::Debug for Bfcrt230 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt230")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt230 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt230 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt231(pub u16);
impl Bfcrt231 {
    #[doc = "Product Term 3, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> Bfcrt231Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt231Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: Bfcrt231Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> Bfcrt231Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt231Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: Bfcrt231Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> Bfcrt231Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt231Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: Bfcrt231Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> Bfcrt231Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt231Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: Bfcrt231Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> Bfcrt231Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt231Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: Bfcrt231Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> Bfcrt231Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt231Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: Bfcrt231Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> Bfcrt231Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt231Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: Bfcrt231Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> Bfcrt231Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt231Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: Bfcrt231Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt231 {
    #[inline(always)]
    fn default() -> Bfcrt231 {
        Bfcrt231(0)
    }
}
impl core::fmt::Debug for Bfcrt231 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt231")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt231 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt231 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt232(pub u16);
impl Bfcrt232 {
    #[doc = "Product Term 3, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> Bfcrt232Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt232Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: Bfcrt232Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> Bfcrt232Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt232Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: Bfcrt232Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> Bfcrt232Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt232Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: Bfcrt232Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> Bfcrt232Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt232Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: Bfcrt232Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> Bfcrt232Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt232Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: Bfcrt232Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> Bfcrt232Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt232Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: Bfcrt232Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> Bfcrt232Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt232Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: Bfcrt232Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> Bfcrt232Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt232Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: Bfcrt232Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt232 {
    #[inline(always)]
    fn default() -> Bfcrt232 {
        Bfcrt232(0)
    }
}
impl core::fmt::Debug for Bfcrt232 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt232")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt232 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt232 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bfcrt233(pub u16);
impl Bfcrt233 {
    #[doc = "Product Term 3, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> Bfcrt233Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        Bfcrt233Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: Bfcrt233Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> Bfcrt233Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        Bfcrt233Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: Bfcrt233Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> Bfcrt233Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        Bfcrt233Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: Bfcrt233Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> Bfcrt233Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        Bfcrt233Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: Bfcrt233Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> Bfcrt233Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        Bfcrt233Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input D Configuration."]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: Bfcrt233Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> Bfcrt233Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        Bfcrt233Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input C Configuration."]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: Bfcrt233Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> Bfcrt233Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        Bfcrt233Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input B Configuration."]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: Bfcrt233Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> Bfcrt233Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        Bfcrt233Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, Input A Configuration."]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: Bfcrt233Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Bfcrt233 {
    #[inline(always)]
    fn default() -> Bfcrt233 {
        Bfcrt233(0)
    }
}
impl core::fmt::Debug for Bfcrt233 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bfcrt233")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bfcrt233 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bfcrt233 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Ac {
        Bfcrt010Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Ac) -> u8 {
        Bfcrt010Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Bc {
        Bfcrt010Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Bc) -> u8 {
        Bfcrt010Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Cc {
        Bfcrt010Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Cc) -> u8 {
        Bfcrt010Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Dc {
        Bfcrt010Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Dc) -> u8 {
        Bfcrt010Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Ac {
        Bfcrt010Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Ac) -> u8 {
        Bfcrt010Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Bc {
        Bfcrt010Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Bc) -> u8 {
        Bfcrt010Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Cc {
        Bfcrt010Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Cc) -> u8 {
        Bfcrt010Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt010Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Dc {
        Bfcrt010Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Dc) -> u8 {
        Bfcrt010Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Ac {
        Bfcrt011Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Ac) -> u8 {
        Bfcrt011Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Bc {
        Bfcrt011Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Bc) -> u8 {
        Bfcrt011Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Cc {
        Bfcrt011Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Cc) -> u8 {
        Bfcrt011Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Dc {
        Bfcrt011Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Dc) -> u8 {
        Bfcrt011Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Ac {
        Bfcrt011Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Ac) -> u8 {
        Bfcrt011Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Bc {
        Bfcrt011Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Bc) -> u8 {
        Bfcrt011Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Cc {
        Bfcrt011Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Cc) -> u8 {
        Bfcrt011Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt011Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Dc {
        Bfcrt011Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Dc) -> u8 {
        Bfcrt011Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Ac {
        Bfcrt012Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Ac) -> u8 {
        Bfcrt012Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Bc {
        Bfcrt012Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Bc) -> u8 {
        Bfcrt012Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Cc {
        Bfcrt012Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Cc) -> u8 {
        Bfcrt012Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Dc {
        Bfcrt012Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Dc) -> u8 {
        Bfcrt012Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Ac {
        Bfcrt012Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Ac) -> u8 {
        Bfcrt012Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Bc {
        Bfcrt012Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Bc) -> u8 {
        Bfcrt012Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Cc {
        Bfcrt012Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Cc) -> u8 {
        Bfcrt012Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt012Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Dc {
        Bfcrt012Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Dc) -> u8 {
        Bfcrt012Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Ac {
        Bfcrt013Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Ac) -> u8 {
        Bfcrt013Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Bc {
        Bfcrt013Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Bc) -> u8 {
        Bfcrt013Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Cc {
        Bfcrt013Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Cc) -> u8 {
        Bfcrt013Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Dc {
        Bfcrt013Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Dc) -> u8 {
        Bfcrt013Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Ac {
        Bfcrt013Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Ac) -> u8 {
        Bfcrt013Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Bc {
        Bfcrt013Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Bc) -> u8 {
        Bfcrt013Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Cc {
        Bfcrt013Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Cc) -> u8 {
        Bfcrt013Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt013Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Dc {
        Bfcrt013Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Dc) -> u8 {
        Bfcrt013Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Ac {
        Bfcrt230Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Ac) -> u8 {
        Bfcrt230Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Bc {
        Bfcrt230Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Bc) -> u8 {
        Bfcrt230Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Cc {
        Bfcrt230Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Cc) -> u8 {
        Bfcrt230Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Dc {
        Bfcrt230Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Dc) -> u8 {
        Bfcrt230Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Ac {
        Bfcrt230Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Ac) -> u8 {
        Bfcrt230Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Bc {
        Bfcrt230Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Bc) -> u8 {
        Bfcrt230Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Cc {
        Bfcrt230Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Cc) -> u8 {
        Bfcrt230Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt230Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Dc {
        Bfcrt230Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Dc) -> u8 {
        Bfcrt230Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Ac {
        Bfcrt231Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Ac) -> u8 {
        Bfcrt231Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Bc {
        Bfcrt231Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Bc) -> u8 {
        Bfcrt231Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Cc {
        Bfcrt231Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Cc) -> u8 {
        Bfcrt231Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Dc {
        Bfcrt231Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Dc) -> u8 {
        Bfcrt231Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Ac {
        Bfcrt231Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Ac) -> u8 {
        Bfcrt231Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Bc {
        Bfcrt231Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Bc) -> u8 {
        Bfcrt231Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Cc {
        Bfcrt231Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Cc) -> u8 {
        Bfcrt231Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt231Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Dc {
        Bfcrt231Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Dc) -> u8 {
        Bfcrt231Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Ac {
        Bfcrt232Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Ac) -> u8 {
        Bfcrt232Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Bc {
        Bfcrt232Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Bc) -> u8 {
        Bfcrt232Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Cc {
        Bfcrt232Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Cc) -> u8 {
        Bfcrt232Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Dc {
        Bfcrt232Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Dc) -> u8 {
        Bfcrt232Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Ac {
        Bfcrt232Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Ac) -> u8 {
        Bfcrt232Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Bc {
        Bfcrt232Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Bc) -> u8 {
        Bfcrt232Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Cc {
        Bfcrt232Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Cc) -> u8 {
        Bfcrt232Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt232Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Dc {
        Bfcrt232Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Dc) -> u8 {
        Bfcrt232Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input A to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Ac {
        Bfcrt233Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Ac) -> u8 {
        Bfcrt233Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Bc {
        Bfcrt233Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Bc) -> u8 {
        Bfcrt233Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Cc {
        Bfcrt233Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Cc) -> u8 {
        Bfcrt233Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Dc {
        Bfcrt233Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Dc) -> u8 {
        Bfcrt233Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Ac {
    #[doc = "Force input A to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input A."]
    Pass = 0x01,
    #[doc = "Complement input A."]
    Complement = 0x02,
    #[doc = "Force input to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Ac {
        Bfcrt233Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Ac) -> u8 {
        Bfcrt233Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Bc {
    #[doc = "Force input B to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input B."]
    Pass = 0x01,
    #[doc = "Complement input B."]
    Complement = 0x02,
    #[doc = "Force input B to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Bc {
        Bfcrt233Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Bc) -> u8 {
        Bfcrt233Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Cc {
    #[doc = "Force input C to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input C."]
    Pass = 0x01,
    #[doc = "Complement input C."]
    Complement = 0x02,
    #[doc = "Force input C to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Cc {
        Bfcrt233Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Cc) -> u8 {
        Bfcrt233Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Dc {
    #[doc = "Force input D to become 0."]
    Force0 = 0x0,
    #[doc = "Pass input D."]
    Pass = 0x01,
    #[doc = "Complement input D."]
    Complement = 0x02,
    #[doc = "Force input D to become 1."]
    Force1 = 0x03,
}
impl Bfcrt233Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Dc {
        Bfcrt233Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Dc) -> u8 {
        Bfcrt233Pt3Dc::to_bits(val)
    }
}
