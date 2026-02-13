#[doc = "SmartDMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smartdma0 {
    ptr: *mut u8,
}
unsafe impl Send for Smartdma0 {}
unsafe impl Sync for Smartdma0 {}
impl Smartdma0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Boot Address"]
    #[inline(always)]
    pub const fn bootadr(self) -> crate::common::Reg<regs::Bootadr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Program Counter"]
    #[inline(always)]
    pub const fn pc(self) -> crate::common::Reg<regs::Pc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Stack Pointer"]
    #[inline(always)]
    pub const fn sp(self) -> crate::common::Reg<regs::Sp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Breakpoint Address"]
    #[inline(always)]
    pub const fn break_addr(self) -> crate::common::Reg<regs::BreakAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Breakpoint Vector"]
    #[inline(always)]
    pub const fn break_vect(self) -> crate::common::Reg<regs::BreakVect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Emergency Vector"]
    #[inline(always)]
    pub const fn emer_vect(self) -> crate::common::Reg<regs::EmerVect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Emergency Select"]
    #[inline(always)]
    pub const fn emer_sel(self) -> crate::common::Reg<regs::EmerSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "ARM to EZH Interrupt Control"]
    #[inline(always)]
    pub const fn arm2ezh(self) -> crate::common::Reg<regs::Arm2ezh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "EZH to ARM Trigger"]
    #[inline(always)]
    pub const fn ezh2arm(self) -> crate::common::Reg<regs::Ezh2arm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Pending Trap Control"]
    #[inline(always)]
    pub const fn pendtrap(self) -> crate::common::Reg<regs::Pendtrap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs;
