#[doc = "reset ccontroller 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstctl1 {
    ptr: *mut u8,
}
unsafe impl Send for Rstctl1 {}
unsafe impl Sync for Rstctl1 {}
impl Rstctl1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "system reset status register"]
    #[inline(always)]
    pub const fn sysrststat(self) -> crate::common::Reg<regs::Sysrststat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "peripheral reset control register 0"]
    #[inline(always)]
    pub const fn prstctl0(self) -> crate::common::Reg<regs::Prstctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "peripheral reset control register 1"]
    #[inline(always)]
    pub const fn prstctl1(self) -> crate::common::Reg<regs::Prstctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "peripheral reset control register 2"]
    #[inline(always)]
    pub const fn prstctl2(self) -> crate::common::Reg<regs::Prstctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "peripheral reset set register 0"]
    #[inline(always)]
    pub const fn prstctl0_set(self) -> crate::common::Reg<regs::Prstctl0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "peripheral reset set register 1"]
    #[inline(always)]
    pub const fn prstctl1_set(self) -> crate::common::Reg<regs::Prstctl1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "peripheral reset set register 2"]
    #[inline(always)]
    pub const fn prstctl2_set(self) -> crate::common::Reg<regs::Prstctl2Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "peripheral reset clear register 0"]
    #[inline(always)]
    pub const fn prstctl0_clr(self) -> crate::common::Reg<regs::Prstctl0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "peripheral reset clear register 1"]
    #[inline(always)]
    pub const fn prstctl1_clr(self) -> crate::common::Reg<regs::Prstctl1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "peripheral reset clear register 2"]
    #[inline(always)]
    pub const fn prstctl2_clr(self) -> crate::common::Reg<regs::Prstctl2Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
}
pub mod regs;
pub mod vals;
