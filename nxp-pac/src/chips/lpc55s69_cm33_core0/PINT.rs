#[doc = "Pin interrupt and pattern match (PINT)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PINT {
    ptr: *mut u8,
}
unsafe impl Send for PINT {}
unsafe impl Sync for PINT {}
impl PINT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pin Interrupt Mode register."]
    #[inline(always)]
    pub const fn ISEL(self) -> crate::common::Reg<regs::ISEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Pin interrupt level or rising edge interrupt enable register."]
    #[inline(always)]
    pub const fn IENR(self) -> crate::common::Reg<regs::IENR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Pin interrupt level or rising edge interrupt set register."]
    #[inline(always)]
    pub const fn SIENR(self) -> crate::common::Reg<regs::SIENR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin interrupt level (rising edge interrupt) clear register."]
    #[inline(always)]
    pub const fn CIENR(self) -> crate::common::Reg<regs::CIENR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pin interrupt active level or falling edge interrupt enable register."]
    #[inline(always)]
    pub const fn IENF(self) -> crate::common::Reg<regs::IENF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Pin interrupt active level or falling edge interrupt set register."]
    #[inline(always)]
    pub const fn SIENF(self) -> crate::common::Reg<regs::SIENF, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pin interrupt active level or falling edge interrupt clear register."]
    #[inline(always)]
    pub const fn CIENF(self) -> crate::common::Reg<regs::CIENF, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Pin interrupt rising edge register."]
    #[inline(always)]
    pub const fn RISE(self) -> crate::common::Reg<regs::RISE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Pin interrupt falling edge register."]
    #[inline(always)]
    pub const fn FALL(self) -> crate::common::Reg<regs::FALL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pin interrupt status register."]
    #[inline(always)]
    pub const fn IST(self) -> crate::common::Reg<regs::IST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Pattern match interrupt control register."]
    #[inline(always)]
    pub const fn PMCTRL(self) -> crate::common::Reg<regs::PMCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Pattern match interrupt bit-slice source register."]
    #[inline(always)]
    pub const fn PMSRC(self) -> crate::common::Reg<regs::PMSRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Pattern match interrupt bit slice configuration register."]
    #[inline(always)]
    pub const fn PMCFG(self) -> crate::common::Reg<regs::PMCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs;
pub mod vals;
