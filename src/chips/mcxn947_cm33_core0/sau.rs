#[doc = "Security Attribution Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sau {
    ptr: *mut u8,
}
unsafe impl Send for Sau {}
unsafe impl Sync for Sau {}
impl Sau {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Security Attribution Unit Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Security Attribution Unit Type Register"]
    #[inline(always)]
    pub const fn type_(self) -> crate::common::Reg<regs::Type, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Security Attribution Unit Region Number Register"]
    #[inline(always)]
    pub const fn rnr(self) -> crate::common::Reg<regs::Rnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Security Attribution Unit Region Base Address Register"]
    #[inline(always)]
    pub const fn rbar(self) -> crate::common::Reg<regs::Rbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Security Attribution Unit Region Limit Address Register"]
    #[inline(always)]
    pub const fn rlar(self) -> crate::common::Reg<regs::Rlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Secure Fault Status Register"]
    #[inline(always)]
    pub const fn sfsr(self) -> crate::common::Reg<regs::Sfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "Secure Fault Address Register"]
    #[inline(always)]
    pub const fn sfar(self) -> crate::common::Reg<regs::Sfar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
