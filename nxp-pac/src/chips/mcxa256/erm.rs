#[doc = "Error Reporting Module"]
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
    #[doc = "ERM Configuration Register 0"]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ERM Status Register 0"]
    #[inline(always)]
    pub const fn sr0(self) -> crate::common::Reg<regs::Sr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ERM Memory 0 Error Address Register"]
    #[inline(always)]
    pub const fn ear0(self) -> crate::common::Reg<regs::Ear0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "ERM Memory 0 Syndrome Register"]
    #[inline(always)]
    pub const fn syn0(self) -> crate::common::Reg<regs::Syn0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "ERM Memory 0 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt0(self) -> crate::common::Reg<regs::CorrErrCnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "ERM Memory 1 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt1(self) -> crate::common::Reg<regs::CorrErrCnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
}
pub mod regs;
