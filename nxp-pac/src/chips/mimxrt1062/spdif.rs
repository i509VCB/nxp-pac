#[doc = "SPDIF"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdif {
    ptr: *mut u8,
}
unsafe impl Send for Spdif {}
unsafe impl Sync for Spdif {}
impl Spdif {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPDIF Configuration Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CDText Control Register"]
    #[inline(always)]
    pub const fn srcd(self) -> crate::common::Reg<regs::Srcd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PhaseConfig Register"]
    #[inline(always)]
    pub const fn srpc(self) -> crate::common::Reg<regs::Srpc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "InterruptEn Register"]
    #[inline(always)]
    pub const fn sie(self) -> crate::common::Reg<regs::Sie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "InterruptClear Register"]
    #[inline(always)]
    pub const fn sic(self) -> crate::common::Reg<regs::Sic, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "InterruptStat Register"]
    #[inline(always)]
    pub const fn sis(self) -> crate::common::Reg<regs::Sis, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SPDIFRxLeft Register"]
    #[inline(always)]
    pub const fn srl(self) -> crate::common::Reg<regs::Srl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SPDIFRxRight Register"]
    #[inline(always)]
    pub const fn srr(self) -> crate::common::Reg<regs::Srr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SPDIFRxCChannel_h Register"]
    #[inline(always)]
    pub const fn srcsh(self) -> crate::common::Reg<regs::Srcsh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SPDIFRxCChannel_l Register"]
    #[inline(always)]
    pub const fn srcsl(self) -> crate::common::Reg<regs::Srcsl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "UchannelRx Register"]
    #[inline(always)]
    pub const fn sru(self) -> crate::common::Reg<regs::Sru, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "QchannelRx Register"]
    #[inline(always)]
    pub const fn srq(self) -> crate::common::Reg<regs::Srq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SPDIFTxLeft Register"]
    #[inline(always)]
    pub const fn stl(self) -> crate::common::Reg<regs::Stl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SPDIFTxRight Register"]
    #[inline(always)]
    pub const fn str(self) -> crate::common::Reg<regs::Str, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SPDIFTxCChannelCons_h Register"]
    #[inline(always)]
    pub const fn stcsch(self) -> crate::common::Reg<regs::Stcsch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SPDIFTxCChannelCons_l Register"]
    #[inline(always)]
    pub const fn stcscl(self) -> crate::common::Reg<regs::Stcscl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "FreqMeas Register"]
    #[inline(always)]
    pub const fn srfm(self) -> crate::common::Reg<regs::Srfm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SPDIFTxClk Register"]
    #[inline(always)]
    pub const fn stc(self) -> crate::common::Reg<regs::Stc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
}
pub mod regs;
pub mod vals;
