#[doc = "Pin interrupt and pattern match (PINT)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pint {
    ptr: *mut u8,
}
unsafe impl Send for Pint {}
unsafe impl Sync for Pint {}
impl Pint {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pin Interrupt Mode register"]
    #[inline(always)]
    pub const fn isel(self) -> crate::common::Reg<regs::Isel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Pin interrupt level or rising edge interrupt enable register"]
    #[inline(always)]
    pub const fn ienr(self) -> crate::common::Reg<regs::Ienr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Pin interrupt level or rising edge interrupt set register"]
    #[inline(always)]
    pub const fn sienr(self) -> crate::common::Reg<regs::Sienr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin interrupt level (rising edge interrupt) clear register"]
    #[inline(always)]
    pub const fn cienr(self) -> crate::common::Reg<regs::Cienr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pin interrupt active level or falling edge interrupt enable register"]
    #[inline(always)]
    pub const fn ienf(self) -> crate::common::Reg<regs::Ienf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Pin interrupt active level or falling edge interrupt set register"]
    #[inline(always)]
    pub const fn sienf(self) -> crate::common::Reg<regs::Sienf, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pin interrupt active level or falling edge interrupt clear register"]
    #[inline(always)]
    pub const fn cienf(self) -> crate::common::Reg<regs::Cienf, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Pin interrupt rising edge register"]
    #[inline(always)]
    pub const fn rise(self) -> crate::common::Reg<regs::Rise, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Pin interrupt falling edge register"]
    #[inline(always)]
    pub const fn fall(self) -> crate::common::Reg<regs::Fall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pin interrupt status register"]
    #[inline(always)]
    pub const fn ist(self) -> crate::common::Reg<regs::Ist, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Pattern match interrupt control register"]
    #[inline(always)]
    pub const fn pmctrl(self) -> crate::common::Reg<regs::Pmctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Pattern match interrupt bit-slice source register"]
    #[inline(always)]
    pub const fn pmsrc(self) -> crate::common::Reg<regs::Pmsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Pattern match interrupt bit slice configuration register"]
    #[inline(always)]
    pub const fn pmcfg(self) -> crate::common::Reg<regs::Pmcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs;
pub mod vals;
