#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "CMX Performance Monitor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmxPerfmon {
    ptr: *mut u8,
}
unsafe impl Send for CmxPerfmon {}
unsafe impl Sync for CmxPerfmon {}
impl CmxPerfmon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Performance Monitor Control."]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::pac::common::Reg<Pmcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Performance Monitor Event Counter."]
    #[inline(always)]
    pub const fn pmectr_hi(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<PmectrHi, crate::pac::common::R> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 8usize) as _)
        }
    }
    #[doc = "Performance Monitor Event Counter."]
    #[inline(always)]
    pub const fn pmectr_lo(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<PmectrLo, crate::pac::common::R> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize + n * 8usize) as _)
        }
    }
}
#[doc = "Performance Monitor Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcr(pub u32);
impl Pmcr {
    #[doc = "Module Is Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn menb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Is Enabled."]
    #[inline(always)]
    pub const fn set_menb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start and Stop Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ssc(&self) -> Ssc {
        let val = (self.0 >> 1usize) & 0x07;
        Ssc::from_bits(val as u8)
    }
    #[doc = "Start and Stop Control."]
    #[inline(always)]
    pub const fn set_ssc(&mut self, val: Ssc) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Count Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cmode(&self) -> Cmode {
        let val = (self.0 >> 4usize) & 0x03;
        Cmode::from_bits(val as u8)
    }
    #[doc = "Count Mode."]
    #[inline(always)]
    pub const fn set_cmode(&mut self, val: Cmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Reset Event Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rectr(&self, n: usize) -> Rectr {
        assert!(n < 3usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Rectr::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter."]
    #[inline(always)]
    pub const fn set_rectr(&mut self, n: usize, val: Rectr) {
        assert!(n < 3usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Select Event."]
    #[must_use]
    #[inline(always)]
    pub const fn selevt(&self, n: usize) -> u8 {
        assert!(n < 3usize);
        let offs = 11usize + n * 7usize;
        let val = (self.0 >> offs) & 0x7f;
        val as u8
    }
    #[doc = "Select Event."]
    #[inline(always)]
    pub const fn set_selevt(&mut self, n: usize, val: u8) {
        assert!(n < 3usize);
        let offs = 11usize + n * 7usize;
        self.0 = (self.0 & !(0x7f << offs)) | (((val as u32) & 0x7f) << offs);
    }
}
impl Default for Pmcr {
    #[inline(always)]
    fn default() -> Pmcr {
        Pmcr(0)
    }
}
impl core::fmt::Debug for Pmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcr")
            .field("menb", &self.menb())
            .field("ssc", &self.ssc())
            .field("cmode", &self.cmode())
            .field("rectr[0]", &self.rectr(0usize))
            .field("rectr[1]", &self.rectr(1usize))
            .field("rectr[2]", &self.rectr(2usize))
            .field("selevt[0]", &self.selevt(0usize))
            .field("selevt[1]", &self.selevt(1usize))
            .field("selevt[2]", &self.selevt(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmcr {{ menb: {=bool:?}, ssc: {:?}, cmode: {:?}, rectr[0]: {:?}, rectr[1]: {:?}, rectr[2]: {:?}, selevt[0]: {=u8:?}, selevt[1]: {=u8:?}, selevt[2]: {=u8:?} }}",
            self.menb(),
            self.ssc(),
            self.cmode(),
            self.rectr(0usize),
            self.rectr(1usize),
            self.rectr(2usize),
            self.selevt(0usize),
            self.selevt(1usize),
            self.selevt(2usize)
        )
    }
}
#[doc = "Performance Monitor Event Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmectrHi(pub u8);
impl PmectrHi {
    #[doc = "Event Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Event Counter."]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for PmectrHi {
    #[inline(always)]
    fn default() -> PmectrHi {
        PmectrHi(0)
    }
}
impl core::fmt::Debug for PmectrHi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmectrHi")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmectrHi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PmectrHi {{ ectr: {=u8:?} }}", self.ectr())
    }
}
#[doc = "Performance Monitor Event Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmectrLo(pub u32);
impl PmectrLo {
    #[doc = "Event Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ectr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Event Counter."]
    #[inline(always)]
    pub const fn set_ectr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PmectrLo {
    #[inline(always)]
    fn default() -> PmectrLo {
        PmectrLo(0)
    }
}
impl core::fmt::Debug for PmectrLo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmectrLo")
            .field("ectr", &self.ectr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmectrLo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PmectrLo {{ ectr: {=u32:?} }}", self.ectr())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmode {
    #[doc = "Counted in both User and Privileged modes."]
    UserAndPriv = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Counted only in User mode."]
    UserOnly = 0x02,
    #[doc = "Counted only in Privileged mode."]
    PrivOnly = 0x03,
}
impl Cmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmode {
    #[inline(always)]
    fn from(val: u8) -> Cmode {
        Cmode::from_bits(val)
    }
}
impl From<Cmode> for u8 {
    #[inline(always)]
    fn from(val: Cmode) -> u8 {
        Cmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rectr {
    #[doc = "Run normally."]
    Run = 0x0,
    #[doc = "Reset."]
    Reset = 0x01,
}
impl Rectr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rectr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rectr {
    #[inline(always)]
    fn from(val: u8) -> Rectr {
        Rectr::from_bits(val)
    }
}
impl From<Rectr> for u8 {
    #[inline(always)]
    fn from(val: Rectr) -> u8 {
        Rectr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssc {
    #[doc = "Idle or no-op."]
    Idle = 0x0,
    #[doc = "Local stop."]
    Lstop = 0x01,
    #[doc = "Local start."]
    Lstart2 = 0x02,
    #[doc = "Local start."]
    Lstart3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ssc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssc {
    #[inline(always)]
    fn from(val: u8) -> Ssc {
        Ssc::from_bits(val)
    }
}
impl From<Ssc> for u8 {
    #[inline(always)]
    fn from(val: Ssc) -> u8 {
        Ssc::to_bits(val)
    }
}
