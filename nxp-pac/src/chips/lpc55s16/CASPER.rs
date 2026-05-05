#[doc = "CASPER."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CASPER {
    ptr: *mut u8,
}
unsafe impl Send for CASPER {}
unsafe impl Sync for CASPER {}
impl CASPER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Contains the offsets of AB and CD in the RAM."]
    #[inline(always)]
    pub const fn CTRL0(self) -> crate::common::Reg<regs::CTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
    #[inline(always)]
    pub const fn CTRL1(self) -> crate::common::Reg<regs::CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
    #[inline(always)]
    pub const fn LOADER(self) -> crate::common::Reg<regs::LOADER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Indicates operational status and would contain the carry bit if used."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Sets interrupts."]
    #[inline(always)]
    pub const fn INTENSET(self) -> crate::common::Reg<regs::INTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Clears interrupts."]
    #[inline(always)]
    pub const fn INTENCLR(self) -> crate::common::Reg<regs::INTENCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt status bits (mask of INTENSET and STATUS)."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "A register."]
    #[inline(always)]
    pub const fn AREG(self) -> crate::common::Reg<regs::AREG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "B register."]
    #[inline(always)]
    pub const fn BREG(self) -> crate::common::Reg<regs::BREG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "C register."]
    #[inline(always)]
    pub const fn CREG(self) -> crate::common::Reg<regs::CREG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "D register."]
    #[inline(always)]
    pub const fn DREG(self) -> crate::common::Reg<regs::DREG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Result register 0."]
    #[inline(always)]
    pub const fn RES0(self) -> crate::common::Reg<regs::RES0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Result register 1."]
    #[inline(always)]
    pub const fn RES1(self) -> crate::common::Reg<regs::RES1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Result register 2."]
    #[inline(always)]
    pub const fn RES2(self) -> crate::common::Reg<regs::RES2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Result register 3."]
    #[inline(always)]
    pub const fn RES3(self) -> crate::common::Reg<regs::RES3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Optional mask register."]
    #[inline(always)]
    pub const fn MASK(self) -> crate::common::Reg<regs::MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Optional re-mask register."]
    #[inline(always)]
    pub const fn REMASK(self) -> crate::common::Reg<regs::REMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Security lock register."]
    #[inline(always)]
    pub const fn LOCK(self) -> crate::common::Reg<regs::LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs;
pub mod vals;
