#[doc = "Memory Protection Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpu {
    ptr: *mut u8,
}
unsafe impl Send for Mpu {}
unsafe impl Sync for Mpu {}
impl Mpu {
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
    pub const fn type_(self) -> crate::common::Reg<regs::Type, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "MPU Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "MPU Region Number Register."]
    #[inline(always)]
    pub const fn rnr(self) -> crate::common::Reg<regs::Rnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn rbar(self) -> crate::common::Reg<regs::Rbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn rlar(self) -> crate::common::Reg<regs::Rlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn rbar_a1(self) -> crate::common::Reg<regs::RbarA1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn rlar_a1(self) -> crate::common::Reg<regs::RlarA1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn rbar_a2(self) -> crate::common::Reg<regs::RbarA2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn rlar_a2(self) -> crate::common::Reg<regs::RlarA2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "MPU Region Base Address Register."]
    #[inline(always)]
    pub const fn rbar_a3(self) -> crate::common::Reg<regs::RbarA3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "MPU Region Limit Address Register."]
    #[inline(always)]
    pub const fn rlar_a3(self) -> crate::common::Reg<regs::RlarA3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "MPU Memory Attribute Indirection Registers 0."]
    #[inline(always)]
    pub const fn mair0(self) -> crate::common::Reg<regs::Mair0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "MPU Memory Attribute Indirection Registers 1."]
    #[inline(always)]
    pub const fn mair1(self) -> crate::common::Reg<regs::Mair1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
