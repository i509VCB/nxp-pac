#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCB {
    ptr: *mut u8,
}
unsafe impl Send for SCB {}
unsafe impl Sync for SCB {}
impl SCB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Application Interrupt and Reset Control Register."]
    #[inline(always)]
    pub const fn AIRCR(self) -> crate::common::Reg<regs::AIRCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "The SCR controls features of entry to and exit from low-power state."]
    #[inline(always)]
    pub const fn SCR(self) -> crate::common::Reg<regs::SCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "System Handler Control and State Register."]
    #[inline(always)]
    pub const fn SHCSR(self) -> crate::common::Reg<regs::SHCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Non-secure Access Control Register."]
    #[inline(always)]
    pub const fn NSACR(self) -> crate::common::Reg<regs::NSACR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
