#[doc = "Memory Protection Unit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MPU {
    ptr: *mut u8,
}
unsafe impl Send for MPU {}
unsafe impl Sync for MPU {}
impl MPU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "The MPU Type Register indicates how many regions the MPU support. Software can use it to determine if the processor implements an MPU."]
    #[inline(always)]
    pub const fn TYPE(self) -> crate::common::Reg<regs::TYPE, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MPU Control Register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MPU Region Number Register."]
    #[inline(always)]
    pub const fn RNR(self) -> crate::common::Reg<regs::RNR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn RBAR(self) -> crate::common::Reg<regs::RBAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn RLAR(self) -> crate::common::Reg<regs::RLAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn RBAR_A1(self) -> crate::common::Reg<regs::RBAR_A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn RLAR_A1(self) -> crate::common::Reg<regs::RLAR_A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn RBAR_A2(self) -> crate::common::Reg<regs::RBAR_A2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn RLAR_A2(self) -> crate::common::Reg<regs::RLAR_A2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn RBAR_A3(self) -> crate::common::Reg<regs::RBAR_A3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn RLAR_A3(self) -> crate::common::Reg<regs::RLAR_A3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MPU Memory Attribute Indirection Registers 0."]
    #[inline(always)]
    pub const fn MAIR0(self) -> crate::common::Reg<regs::MAIR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "MPU Memory Attribute Indirection Registers 1."]
    #[inline(always)]
    pub const fn MAIR1(self) -> crate::common::Reg<regs::MAIR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
