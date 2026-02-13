#[doc = "FREQME"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqme {
    ptr: *mut u8,
}
unsafe impl Send for Freqme {}
unsafe impl Sync for Freqme {}
impl Freqme {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control (in Read mode)"]
    #[inline(always)]
    pub const fn ctrl_r(self) -> crate::common::Reg<regs::CtrlR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control (in Write mode)"]
    #[inline(always)]
    pub const fn ctrl_w(self) -> crate::common::Reg<regs::CtrlW, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Status"]
    #[inline(always)]
    pub const fn ctrlstat(self) -> crate::common::Reg<regs::Ctrlstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Minimum"]
    #[inline(always)]
    pub const fn min(self) -> crate::common::Reg<regs::Min, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Maximum"]
    #[inline(always)]
    pub const fn max(self) -> crate::common::Reg<regs::Max, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
