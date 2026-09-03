#[doc = "SCG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scg {
    ptr: *mut u8,
}
unsafe impl Send for Scg {}
unsafe impl Sync for Scg {}
impl Scg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register."]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register."]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Trim Lock register."]
    #[inline(always)]
    pub const fn trim_lock(self) -> crate::common::Reg<regs::TrimLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock Status Register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Run Clock Control Register."]
    #[inline(always)]
    pub const fn rccr(self) -> crate::common::Reg<regs::Rccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SOSC Control Status Register."]
    #[inline(always)]
    pub const fn sosccsr(self) -> crate::common::Reg<regs::Sosccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SOSC Configuration Register."]
    #[inline(always)]
    pub const fn sosccfg(self) -> crate::common::Reg<regs::Sosccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SIRC Control Status Register."]
    #[inline(always)]
    pub const fn sirccsr(self) -> crate::common::Reg<regs::Sirccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SIRC Trim Configuration Register."]
    #[inline(always)]
    pub const fn sirctcfg(self) -> crate::common::Reg<regs::Sirctcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SIRC Trim Register."]
    #[inline(always)]
    pub const fn sirctrim(self) -> crate::common::Reg<regs::Sirctrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SIRC Auto-trimming Status Register."]
    #[inline(always)]
    pub const fn sircstat(self) -> crate::common::Reg<regs::Sircstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FIRC Control Status Register."]
    #[inline(always)]
    pub const fn firccsr(self) -> crate::common::Reg<regs::Firccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "FIRC Configuration Register."]
    #[inline(always)]
    pub const fn firccfg(self) -> crate::common::Reg<regs::Firccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "FIRC Trim Configuration Register."]
    #[inline(always)]
    pub const fn firctcfg(self) -> crate::common::Reg<regs::Firctcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "FIRC Trim Register."]
    #[inline(always)]
    pub const fn firctrim(self) -> crate::common::Reg<regs::Firctrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Status Register."]
    #[inline(always)]
    pub const fn fircstat(self) -> crate::common::Reg<regs::Fircstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 1."]
    #[inline(always)]
    pub const fn fircatc1(self) -> crate::common::Reg<regs::Fircatc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 2."]
    #[inline(always)]
    pub const fn fircatc2(self) -> crate::common::Reg<regs::Fircatc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "FIRC Auto-trimming Counter 2."]
    #[inline(always)]
    pub const fn fircatc3(self) -> crate::common::Reg<regs::Fircatc3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "ROSC Control Status Register."]
    #[inline(always)]
    pub const fn rosccsr(self) -> crate::common::Reg<regs::Rosccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
}
pub mod regs;
pub mod vals;
