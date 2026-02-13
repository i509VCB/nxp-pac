#[doc = "ERM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erm0 {
    ptr: *mut u8,
}
unsafe impl Send for Erm0 {}
unsafe impl Sync for Erm0 {}
impl Erm0 {
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
    #[doc = "ERM Configuration Register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ERM Status Register 0"]
    #[inline(always)]
    pub const fn sr0(self) -> crate::common::Reg<regs::Sr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ERM Status Register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
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
    #[doc = "ERM Memory 1 Error Address Register"]
    #[inline(always)]
    pub const fn ear1(self) -> crate::common::Reg<regs::Ear1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "ERM Memory 1 Syndrome Register"]
    #[inline(always)]
    pub const fn syn1(self) -> crate::common::Reg<regs::Syn1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "ERM Memory 1 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt1(self) -> crate::common::Reg<regs::CorrErrCnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "ERM Memory 2 Error Address Register"]
    #[inline(always)]
    pub const fn ear2(self) -> crate::common::Reg<regs::Ear2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "ERM Memory 2 Syndrome Register"]
    #[inline(always)]
    pub const fn syn2(self) -> crate::common::Reg<regs::Syn2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "ERM Memory 2 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt2(self) -> crate::common::Reg<regs::CorrErrCnt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "ERM Memory 3 Error Address Register"]
    #[inline(always)]
    pub const fn ear3(self) -> crate::common::Reg<regs::Ear3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "ERM Memory 3 Syndrome Register"]
    #[inline(always)]
    pub const fn syn3(self) -> crate::common::Reg<regs::Syn3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "ERM Memory 3 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt3(self) -> crate::common::Reg<regs::CorrErrCnt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ERM Memory 4 Error Address Register"]
    #[inline(always)]
    pub const fn ear4(self) -> crate::common::Reg<regs::Ear4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ERM Memory 4 Syndrome Register"]
    #[inline(always)]
    pub const fn syn4(self) -> crate::common::Reg<regs::Syn4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "ERM Memory 4 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt4(self) -> crate::common::Reg<regs::CorrErrCnt4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "ERM Memory 5 Error Address Register"]
    #[inline(always)]
    pub const fn ear5(self) -> crate::common::Reg<regs::Ear5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "ERM Memory 5 Syndrome Register"]
    #[inline(always)]
    pub const fn syn5(self) -> crate::common::Reg<regs::Syn5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "ERM Memory 5 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt5(self) -> crate::common::Reg<regs::CorrErrCnt5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "ERM Memory 6 Error Address Register"]
    #[inline(always)]
    pub const fn ear6(self) -> crate::common::Reg<regs::Ear6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "ERM Memory 6 Syndrome Register"]
    #[inline(always)]
    pub const fn syn6(self) -> crate::common::Reg<regs::Syn6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "ERM Memory 6 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt6(self) -> crate::common::Reg<regs::CorrErrCnt6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "ERM Memory 7 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt7(self) -> crate::common::Reg<regs::CorrErrCnt7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "ERM Memory 8 Syndrome Register"]
    #[inline(always)]
    pub const fn syn8(self) -> crate::common::Reg<regs::Syn8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "ERM Memory 8 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt8(self) -> crate::common::Reg<regs::CorrErrCnt8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "ERM Memory 9 Correctable Error Count Register"]
    #[inline(always)]
    pub const fn corr_err_cnt9(self) -> crate::common::Reg<regs::CorrErrCnt9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
}
pub mod regs;
pub mod vals;
