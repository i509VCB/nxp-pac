#[doc = "System Clock Generator"]
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
    #[doc = "Version ID Register"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Trim Lock register"]
    #[inline(always)]
    pub const fn trim_lock(self) -> crate::common::Reg<regs::TrimLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock Status Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Run Clock Control Register"]
    #[inline(always)]
    pub const fn rccr(self) -> crate::common::Reg<regs::Rccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SOSC Control Status Register"]
    #[inline(always)]
    pub const fn sosccsr(self) -> crate::common::Reg<regs::Sosccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SOSC Configuration Register"]
    #[inline(always)]
    pub const fn sosccfg(self) -> crate::common::Reg<regs::Sosccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SIRC Control Status Register"]
    #[inline(always)]
    pub const fn sirccsr(self) -> crate::common::Reg<regs::Sirccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SIRC Trim Configuration Register"]
    #[inline(always)]
    pub const fn sirctcfg(self) -> crate::common::Reg<regs::Sirctcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SIRC Trim Register"]
    #[inline(always)]
    pub const fn sirctrim(self) -> crate::common::Reg<regs::Sirctrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SIRC Auto-trimming Status Register"]
    #[inline(always)]
    pub const fn sircstat(self) -> crate::common::Reg<regs::Sircstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FIRC Control Status Register"]
    #[inline(always)]
    pub const fn firccsr(self) -> crate::common::Reg<regs::Firccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "FIRC Configuration Register"]
    #[inline(always)]
    pub const fn firccfg(self) -> crate::common::Reg<regs::Firccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "FIRC Trim Register"]
    #[inline(always)]
    pub const fn firctrim(self) -> crate::common::Reg<regs::Firctrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "ROSC Control Status Register"]
    #[inline(always)]
    pub const fn rosccsr(self) -> crate::common::Reg<regs::Rosccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SPLL Control Status Register"]
    #[inline(always)]
    pub const fn spllcsr(self) -> crate::common::Reg<regs::Spllcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "SPLL Control Register"]
    #[inline(always)]
    pub const fn spllctrl(self) -> crate::common::Reg<regs::Spllctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "SPLL Status Register"]
    #[inline(always)]
    pub const fn spllstat(self) -> crate::common::Reg<regs::Spllstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "SPLL N Divider Register"]
    #[inline(always)]
    pub const fn spllndiv(self) -> crate::common::Reg<regs::Spllndiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "SPLL M Divider Register"]
    #[inline(always)]
    pub const fn spllmdiv(self) -> crate::common::Reg<regs::Spllmdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "SPLL P Divider Register"]
    #[inline(always)]
    pub const fn spllpdiv(self) -> crate::common::Reg<regs::Spllpdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "SPLL LOCK Configuration Register"]
    #[inline(always)]
    pub const fn splllock_cnfg(self) -> crate::common::Reg<regs::SplllockCnfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "SPLL SSCG Status Register"]
    #[inline(always)]
    pub const fn spllsscgstat(self) -> crate::common::Reg<regs::Spllsscgstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "SPLL Spread Spectrum Control 0 Register"]
    #[inline(always)]
    pub const fn spllsscg0(self) -> crate::common::Reg<regs::Spllsscg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "SPLL Spread Spectrum Control 1 Register"]
    #[inline(always)]
    pub const fn spllsscg1(self) -> crate::common::Reg<regs::Spllsscg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "LDO Control and Status Register"]
    #[inline(always)]
    pub const fn ldocsr(self) -> crate::common::Reg<regs::Ldocsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
}
pub mod regs;
pub mod vals;
