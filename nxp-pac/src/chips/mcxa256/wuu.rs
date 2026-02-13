#[doc = "Low-Leakage Wakeup Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wuu {
    ptr: *mut u8,
}
unsafe impl Send for Wuu {}
unsafe impl Sync for Wuu {}
impl Wuu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Pin Enable 1"]
    #[inline(always)]
    pub const fn pe1(self) -> crate::common::Reg<regs::Pe1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin Enable 2"]
    #[inline(always)]
    pub const fn pe2(self) -> crate::common::Reg<regs::Pe2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Module Interrupt Enable"]
    #[inline(always)]
    pub const fn me(self) -> crate::common::Reg<regs::Me, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Module DMA/Trigger Enable"]
    #[inline(always)]
    pub const fn de(self) -> crate::common::Reg<regs::De, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Pin Flag"]
    #[inline(always)]
    pub const fn pf(self) -> crate::common::Reg<regs::Pf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pin Filter"]
    #[inline(always)]
    pub const fn filt(self) -> crate::common::Reg<regs::Filt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Pin DMA/Trigger Configuration 1"]
    #[inline(always)]
    pub const fn pdc1(self) -> crate::common::Reg<regs::Pdc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Pin DMA/Trigger Configuration 2"]
    #[inline(always)]
    pub const fn pdc2(self) -> crate::common::Reg<regs::Pdc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Pin Filter DMA/Trigger Configuration"]
    #[inline(always)]
    pub const fn fdc(self) -> crate::common::Reg<regs::Fdc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Pin Mode Configuration"]
    #[inline(always)]
    pub const fn pmc(self) -> crate::common::Reg<regs::Pmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Pin Filter Mode Configuration"]
    #[inline(always)]
    pub const fn fmc(self) -> crate::common::Reg<regs::Fmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
}
pub mod regs;
pub mod vals;
