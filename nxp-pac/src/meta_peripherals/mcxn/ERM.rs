#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "ERM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erm {
    ptr: *mut u8,
}
unsafe impl Send for Erm {}
unsafe impl Sync for Erm {}
impl Erm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ERM Configuration Register 0."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::pac::common::Reg<Cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ERM Configuration Register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::pac::common::Reg<Cr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ERM Status Register 0."]
    #[inline(always)]
    pub const fn sr0(self) -> crate::pac::common::Reg<Sr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ERM Status Register 1."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::pac::common::Reg<Sr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ERM Memory Error Address Register."]
    #[inline(always)]
    pub const fn ear(self, n: usize) -> crate::pac::common::Reg<Ear, crate::pac::common::R> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 16usize) as _)
        }
    }
    #[doc = "ERM Memory Syndrome Register."]
    #[inline(always)]
    pub const fn syn(self, n: usize) -> crate::pac::common::Reg<Syn, crate::pac::common::R> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize + n * 16usize) as _)
        }
    }
    #[doc = "ERM Memory Correctable Error Count Register."]
    #[inline(always)]
    pub const fn corr_err_cnt(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<CorrErrCnt, crate::pac::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize + n * 16usize) as _)
        }
    }
    #[doc = "ERM Memory 8 Syndrome Register."]
    #[inline(always)]
    pub const fn syn8(self) -> crate::pac::common::Reg<Syn8, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
}
#[doc = "ERM Memory Correctable Error Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt(pub u32);
impl CorrErrCnt {
    #[doc = "Memory n Correctable Error Count."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt {
    #[inline(always)]
    fn default() -> CorrErrCnt {
        CorrErrCnt(0)
    }
}
impl core::fmt::Debug for CorrErrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CorrErrCnt {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "ERM Configuration Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0(pub u32);
impl Cr0 {
    #[doc = "ENCIE7."]
    #[must_use]
    #[inline(always)]
    pub const fn encie7(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE7."]
    #[inline(always)]
    pub const fn set_encie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ESCIE7."]
    #[must_use]
    #[inline(always)]
    pub const fn escie7(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE7."]
    #[inline(always)]
    pub const fn set_escie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ENCIE6."]
    #[must_use]
    #[inline(always)]
    pub const fn encie6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE6."]
    #[inline(always)]
    pub const fn set_encie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "ESCIE6."]
    #[must_use]
    #[inline(always)]
    pub const fn escie6(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE6."]
    #[inline(always)]
    pub const fn set_escie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ENCIE5."]
    #[must_use]
    #[inline(always)]
    pub const fn encie5(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE5."]
    #[inline(always)]
    pub const fn set_encie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "ESCIE5."]
    #[must_use]
    #[inline(always)]
    pub const fn escie5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE5."]
    #[inline(always)]
    pub const fn set_escie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ENCIE4."]
    #[must_use]
    #[inline(always)]
    pub const fn encie4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE4."]
    #[inline(always)]
    pub const fn set_encie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ESCIE4."]
    #[must_use]
    #[inline(always)]
    pub const fn escie4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE4."]
    #[inline(always)]
    pub const fn set_escie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ENCIE3."]
    #[must_use]
    #[inline(always)]
    pub const fn encie3(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE3."]
    #[inline(always)]
    pub const fn set_encie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ESCIE3."]
    #[must_use]
    #[inline(always)]
    pub const fn escie3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE3."]
    #[inline(always)]
    pub const fn set_escie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ENCIE2."]
    #[must_use]
    #[inline(always)]
    pub const fn encie2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE2."]
    #[inline(always)]
    pub const fn set_encie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ESCIE2."]
    #[must_use]
    #[inline(always)]
    pub const fn escie2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE2."]
    #[inline(always)]
    pub const fn set_escie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ENCIE1."]
    #[must_use]
    #[inline(always)]
    pub const fn encie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE1."]
    #[inline(always)]
    pub const fn set_encie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ESCIE1."]
    #[must_use]
    #[inline(always)]
    pub const fn escie1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE1."]
    #[inline(always)]
    pub const fn set_escie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ENCIE0."]
    #[must_use]
    #[inline(always)]
    pub const fn encie0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE0."]
    #[inline(always)]
    pub const fn set_encie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ESCIE0."]
    #[must_use]
    #[inline(always)]
    pub const fn escie0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE0."]
    #[inline(always)]
    pub const fn set_escie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        Cr0(0)
    }
}
impl core::fmt::Debug for Cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr0")
            .field("encie7", &self.encie7())
            .field("escie7", &self.escie7())
            .field("encie6", &self.encie6())
            .field("escie6", &self.escie6())
            .field("encie5", &self.encie5())
            .field("escie5", &self.escie5())
            .field("encie4", &self.encie4())
            .field("escie4", &self.escie4())
            .field("encie3", &self.encie3())
            .field("escie3", &self.escie3())
            .field("encie2", &self.encie2())
            .field("escie2", &self.escie2())
            .field("encie1", &self.encie1())
            .field("escie1", &self.escie1())
            .field("encie0", &self.encie0())
            .field("escie0", &self.escie0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr0 {{ encie7: {=bool:?}, escie7: {=bool:?}, encie6: {=bool:?}, escie6: {=bool:?}, encie5: {=bool:?}, escie5: {=bool:?}, encie4: {=bool:?}, escie4: {=bool:?}, encie3: {=bool:?}, escie3: {=bool:?}, encie2: {=bool:?}, escie2: {=bool:?}, encie1: {=bool:?}, escie1: {=bool:?}, encie0: {=bool:?}, escie0: {=bool:?} }}",
            self.encie7(),
            self.escie7(),
            self.encie6(),
            self.escie6(),
            self.encie5(),
            self.escie5(),
            self.encie4(),
            self.escie4(),
            self.encie3(),
            self.escie3(),
            self.encie2(),
            self.escie2(),
            self.encie1(),
            self.escie1(),
            self.encie0(),
            self.escie0()
        )
    }
}
#[doc = "ERM Configuration Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "ENCIE9."]
    #[must_use]
    #[inline(always)]
    pub const fn encie9(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE9."]
    #[inline(always)]
    pub const fn set_encie9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ESCIE9."]
    #[must_use]
    #[inline(always)]
    pub const fn escie9(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE9."]
    #[inline(always)]
    pub const fn set_escie9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ENCIE8."]
    #[must_use]
    #[inline(always)]
    pub const fn encie8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE8."]
    #[inline(always)]
    pub const fn set_encie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ESCIE8."]
    #[must_use]
    #[inline(always)]
    pub const fn escie8(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE8."]
    #[inline(always)]
    pub const fn set_escie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("encie9", &self.encie9())
            .field("escie9", &self.escie9())
            .field("encie8", &self.encie8())
            .field("escie8", &self.escie8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ encie9: {=bool:?}, escie9: {=bool:?}, encie8: {=bool:?}, escie8: {=bool:?} }}",
            self.encie9(),
            self.escie9(),
            self.encie8(),
            self.escie8()
        )
    }
}
#[doc = "ERM Memory Error Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ear(pub u32);
impl Ear {
    #[doc = "EAR."]
    #[must_use]
    #[inline(always)]
    pub const fn ear(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "EAR."]
    #[inline(always)]
    pub const fn set_ear(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ear {
    #[inline(always)]
    fn default() -> Ear {
        Ear(0)
    }
}
impl core::fmt::Debug for Ear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ear").field("ear", &self.ear()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ear {{ ear: {=u32:?} }}", self.ear())
    }
}
#[doc = "ERM Status Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr0(pub u32);
impl Sr0 {
    #[doc = "NCE7."]
    #[must_use]
    #[inline(always)]
    pub const fn nce7(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NCE7."]
    #[inline(always)]
    pub const fn set_nce7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SBC7."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc7(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SBC7."]
    #[inline(always)]
    pub const fn set_sbc7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NCE6."]
    #[must_use]
    #[inline(always)]
    pub const fn nce6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NCE6."]
    #[inline(always)]
    pub const fn set_nce6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SBC6."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc6(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SBC6."]
    #[inline(always)]
    pub const fn set_sbc6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "NCE5."]
    #[must_use]
    #[inline(always)]
    pub const fn nce5(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NCE5."]
    #[inline(always)]
    pub const fn set_nce5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SBC5."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc5(&self) -> Sbc5 {
        let val = (self.0 >> 11usize) & 0x01;
        Sbc5::from_bits(val as u8)
    }
    #[doc = "SBC5."]
    #[inline(always)]
    pub const fn set_sbc5(&mut self, val: Sbc5) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "NCE4."]
    #[must_use]
    #[inline(always)]
    pub const fn nce4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "NCE4."]
    #[inline(always)]
    pub const fn set_nce4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SBC4."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SBC4."]
    #[inline(always)]
    pub const fn set_sbc4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NCE3."]
    #[must_use]
    #[inline(always)]
    pub const fn nce3(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "NCE3."]
    #[inline(always)]
    pub const fn set_nce3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SBC3."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SBC3."]
    #[inline(always)]
    pub const fn set_sbc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "NCE2."]
    #[must_use]
    #[inline(always)]
    pub const fn nce2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "NCE2."]
    #[inline(always)]
    pub const fn set_nce2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SBC2."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SBC2."]
    #[inline(always)]
    pub const fn set_sbc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "NCE1."]
    #[must_use]
    #[inline(always)]
    pub const fn nce1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "NCE1."]
    #[inline(always)]
    pub const fn set_nce1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SBC1."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "SBC1."]
    #[inline(always)]
    pub const fn set_sbc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "NCE0."]
    #[must_use]
    #[inline(always)]
    pub const fn nce0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "NCE0."]
    #[inline(always)]
    pub const fn set_nce0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SBC0."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SBC0."]
    #[inline(always)]
    pub const fn set_sbc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sr0 {
    #[inline(always)]
    fn default() -> Sr0 {
        Sr0(0)
    }
}
impl core::fmt::Debug for Sr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr0")
            .field("nce7", &self.nce7())
            .field("sbc7", &self.sbc7())
            .field("nce6", &self.nce6())
            .field("sbc6", &self.sbc6())
            .field("nce5", &self.nce5())
            .field("sbc5", &self.sbc5())
            .field("nce4", &self.nce4())
            .field("sbc4", &self.sbc4())
            .field("nce3", &self.nce3())
            .field("sbc3", &self.sbc3())
            .field("nce2", &self.nce2())
            .field("sbc2", &self.sbc2())
            .field("nce1", &self.nce1())
            .field("sbc1", &self.sbc1())
            .field("nce0", &self.nce0())
            .field("sbc0", &self.sbc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr0 {{ nce7: {=bool:?}, sbc7: {=bool:?}, nce6: {=bool:?}, sbc6: {=bool:?}, nce5: {=bool:?}, sbc5: {:?}, nce4: {=bool:?}, sbc4: {=bool:?}, nce3: {=bool:?}, sbc3: {=bool:?}, nce2: {=bool:?}, sbc2: {=bool:?}, nce1: {=bool:?}, sbc1: {=bool:?}, nce0: {=bool:?}, sbc0: {=bool:?} }}",
            self.nce7(),
            self.sbc7(),
            self.nce6(),
            self.sbc6(),
            self.nce5(),
            self.sbc5(),
            self.nce4(),
            self.sbc4(),
            self.nce3(),
            self.sbc3(),
            self.nce2(),
            self.sbc2(),
            self.nce1(),
            self.sbc1(),
            self.nce0(),
            self.sbc0()
        )
    }
}
#[doc = "ERM Status Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr1(pub u32);
impl Sr1 {
    #[doc = "NCE9."]
    #[must_use]
    #[inline(always)]
    pub const fn nce9(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "NCE9."]
    #[inline(always)]
    pub const fn set_nce9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SBC9."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc9(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "SBC9."]
    #[inline(always)]
    pub const fn set_sbc9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "NCE8."]
    #[must_use]
    #[inline(always)]
    pub const fn nce8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "NCE8."]
    #[inline(always)]
    pub const fn set_nce8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SBC8."]
    #[must_use]
    #[inline(always)]
    pub const fn sbc8(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SBC8."]
    #[inline(always)]
    pub const fn set_sbc8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sr1 {
    #[inline(always)]
    fn default() -> Sr1 {
        Sr1(0)
    }
}
impl core::fmt::Debug for Sr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr1")
            .field("nce9", &self.nce9())
            .field("sbc9", &self.sbc9())
            .field("nce8", &self.nce8())
            .field("sbc8", &self.sbc8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr1 {{ nce9: {=bool:?}, sbc9: {=bool:?}, nce8: {=bool:?}, sbc8: {=bool:?} }}",
            self.nce9(),
            self.sbc9(),
            self.nce8(),
            self.sbc8()
        )
    }
}
#[doc = "ERM Memory Syndrome Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syn(pub u32);
impl Syn {
    #[doc = "SYNDROME."]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SYNDROME."]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Syn {
    #[inline(always)]
    fn default() -> Syn {
        Syn(0)
    }
}
impl core::fmt::Debug for Syn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syn")
            .field("syndrome", &self.syndrome())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syn {{ syndrome: {=u8:?} }}", self.syndrome())
    }
}
#[doc = "ERM Memory 8 Syndrome Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syn8(pub u32);
impl Syn8 {
    #[doc = "SYNDROME."]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SYNDROME."]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Syn8 {
    #[inline(always)]
    fn default() -> Syn8 {
        Syn8(0)
    }
}
impl core::fmt::Debug for Syn8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syn8")
            .field("syndrome", &self.syndrome())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syn8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syn8 {{ syndrome: {=u8:?} }}", self.syndrome())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc5 {
    #[doc = "No single-bit correction event on Memory 5 detected."]
    NoEvent = 0x0,
    #[doc = "Single-bit correction event on Memory 5 detected."]
    CorrEvent = 0x01,
}
impl Sbc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc5 {
    #[inline(always)]
    fn from(val: u8) -> Sbc5 {
        Sbc5::from_bits(val)
    }
}
impl From<Sbc5> for u8 {
    #[inline(always)]
    fn from(val: Sbc5) -> u8 {
        Sbc5::to_bits(val)
    }
}
