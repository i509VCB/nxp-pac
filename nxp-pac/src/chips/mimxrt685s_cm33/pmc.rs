#[doc = "LPC-Next0 Power Management Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmc {
    ptr: *mut u8,
}
unsafe impl Send for Pmc {}
unsafe impl Sync for Pmc {}
impl Pmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PMC status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Wakeup, interrupt, and reset flags"]
    #[inline(always)]
    pub const fn flags(self) -> crate::common::Reg<regs::Flags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PMC control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "PMC controls used during run mode"]
    #[inline(always)]
    pub const fn runctrl(self) -> crate::common::Reg<regs::Runctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PMC controls used during deep sleep mode"]
    #[inline(always)]
    pub const fn sleepctrl(self) -> crate::common::Reg<regs::Sleepctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Active vddcore LVD monitor trip adjust"]
    #[inline(always)]
    pub const fn lvdcorectrl(self) -> crate::common::Reg<regs::Lvdcorectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Automatic wakeup from deepsleep / deep powerdown modes"]
    #[inline(always)]
    pub const fn autowkup(self) -> crate::common::Reg<regs::Autowkup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PMIC power mode select control configuration to let PMC know when vddcore or vdd1v8 will power off/on"]
    #[inline(always)]
    pub const fn pmiccfg(self) -> crate::common::Reg<regs::Pmiccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPIO vdde range selection control"]
    #[inline(always)]
    pub const fn padvrange(self) -> crate::common::Reg<regs::Padvrange, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Memory Sequencer Control Register"]
    #[inline(always)]
    pub const fn memseqctrl(self) -> crate::common::Reg<regs::Memseqctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs;
pub mod vals;
