#[doc = "System Control Block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scb {
    ptr: *mut u8,
}
unsafe impl Send for Scb {}
unsafe impl Sync for Scb {}
impl Scb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Application Interrupt and Reset Control Register"]
    #[inline(always)]
    pub const fn aircr(self) -> crate::common::Reg<regs::Aircr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "The SCR controls features of entry to and exit from low-power state."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "System Handler Control and State Register"]
    #[inline(always)]
    pub const fn shcsr(self) -> crate::common::Reg<regs::Shcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Coprocessor Access Control Register"]
    #[inline(always)]
    pub const fn cpacr(self) -> crate::common::Reg<regs::Cpacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Non-secure Access Control Register"]
    #[inline(always)]
    pub const fn nsacr(self) -> crate::common::Reg<regs::Nsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
