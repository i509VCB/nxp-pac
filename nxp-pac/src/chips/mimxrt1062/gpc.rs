#[doc = "GPC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpc {
    ptr: *mut u8,
}
unsafe impl Send for Gpc {}
unsafe impl Sync for Gpc {}
impl Gpc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPC Interface control register"]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "IRQ masking register 1"]
    #[inline(always)]
    pub const fn imr1(self) -> crate::common::Reg<regs::Imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "IRQ masking register 2"]
    #[inline(always)]
    pub const fn imr2(self) -> crate::common::Reg<regs::Imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "IRQ masking register 3"]
    #[inline(always)]
    pub const fn imr3(self) -> crate::common::Reg<regs::Imr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "IRQ masking register 4"]
    #[inline(always)]
    pub const fn imr4(self) -> crate::common::Reg<regs::Imr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "IRQ status resister 1"]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "IRQ status resister 2"]
    #[inline(always)]
    pub const fn isr2(self) -> crate::common::Reg<regs::Isr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "IRQ status resister 3"]
    #[inline(always)]
    pub const fn isr3(self) -> crate::common::Reg<regs::Isr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "IRQ status resister 4"]
    #[inline(always)]
    pub const fn isr4(self) -> crate::common::Reg<regs::Isr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "IRQ masking register 5"]
    #[inline(always)]
    pub const fn imr5(self) -> crate::common::Reg<regs::Imr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "IRQ status resister 5"]
    #[inline(always)]
    pub const fn isr5(self) -> crate::common::Reg<regs::Isr5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs;
pub mod vals;
