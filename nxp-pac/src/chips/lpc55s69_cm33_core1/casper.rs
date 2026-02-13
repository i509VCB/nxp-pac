#[doc = "CASPER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Casper {
    ptr: *mut u8,
}
unsafe impl Send for Casper {}
unsafe impl Sync for Casper {}
impl Casper {
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
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
    #[inline(always)]
    pub const fn loader(self) -> crate::common::Reg<regs::Loader, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Indicates operational status and would contain the carry bit if used."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Sets interrupts"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Intenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Clears interrupts"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Intenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "A register"]
    #[inline(always)]
    pub const fn areg(self) -> crate::common::Reg<regs::Areg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "B register"]
    #[inline(always)]
    pub const fn breg(self) -> crate::common::Reg<regs::Breg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "C register"]
    #[inline(always)]
    pub const fn creg(self) -> crate::common::Reg<regs::Creg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "D register"]
    #[inline(always)]
    pub const fn dreg(self) -> crate::common::Reg<regs::Dreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Result register 0"]
    #[inline(always)]
    pub const fn res0(self) -> crate::common::Reg<regs::Res0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Result register 1"]
    #[inline(always)]
    pub const fn res1(self) -> crate::common::Reg<regs::Res1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Result register 2"]
    #[inline(always)]
    pub const fn res2(self) -> crate::common::Reg<regs::Res2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Result register 3"]
    #[inline(always)]
    pub const fn res3(self) -> crate::common::Reg<regs::Res3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Optional mask register"]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Optional re-mask register"]
    #[inline(always)]
    pub const fn remask(self) -> crate::common::Reg<regs::Remask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Security lock register"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs;
pub mod vals;
