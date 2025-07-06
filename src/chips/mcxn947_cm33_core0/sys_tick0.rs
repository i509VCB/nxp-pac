#[doc = "M33 Systick module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysTick0 {
    ptr: *mut u8,
}
unsafe impl Send for SysTick0 {}
unsafe impl Sync for SysTick0 {}
impl SysTick0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SysTick Control and Status Register"]
    #[inline(always)]
    pub const fn syst_csr(self) -> crate::common::Reg<regs::SystCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SysTick Reload Value Register"]
    #[inline(always)]
    pub const fn syst_rvr(self) -> crate::common::Reg<regs::SystRvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SysTick Current Value Register"]
    #[inline(always)]
    pub const fn syst_cvr(self) -> crate::common::Reg<regs::SystCvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SysTick Calibration Value Register"]
    #[inline(always)]
    pub const fn syst_calib(self) -> crate::common::Reg<regs::SystCalib, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
