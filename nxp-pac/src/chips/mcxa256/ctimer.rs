#[doc = "Standard Counter or Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer {
    ptr: *mut u8,
}
unsafe impl Send for Ctimer {}
unsafe impl Sync for Ctimer {}
impl Ctimer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer Control"]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer Counter"]
    #[inline(always)]
    pub const fn tc(self) -> crate::common::Reg<regs::Tc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Prescale"]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<regs::Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Prescale Counter"]
    #[inline(always)]
    pub const fn pc(self) -> crate::common::Reg<regs::Pc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Match Control"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Match"]
    #[inline(always)]
    pub const fn mr(self, n: usize) -> crate::common::Reg<regs::Mr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Capture"]
    #[inline(always)]
    pub const fn cr(self, n: usize) -> crate::common::Reg<regs::Cr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize + n * 4usize) as _) }
    }
    #[doc = "External Match"]
    #[inline(always)]
    pub const fn emr(self) -> crate::common::Reg<regs::Emr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Count Control"]
    #[inline(always)]
    pub const fn ctcr(self) -> crate::common::Reg<regs::Ctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "PWM Control"]
    #[inline(always)]
    pub const fn pwmc(self) -> crate::common::Reg<regs::Pwmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Match Shadow"]
    #[inline(always)]
    pub const fn msr(self, n: usize) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
