#[doc = "INTM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intm0 {
    ptr: *mut u8,
}
unsafe impl Send for Intm0 {}
unsafe impl Sync for Intm0 {}
impl Intm0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Monitor Mode"]
    #[inline(always)]
    pub const fn intm_mm(self) -> crate::common::Reg<regs::IntmMm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn intm_iack(self) -> crate::common::Reg<regs::IntmIack, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Monitoring"]
    #[inline(always)]
    pub const fn mon(self, n: usize) -> Mon {
        assert!(n < 4usize);
        unsafe { Mon::from_ptr(self.ptr.wrapping_add(0x08usize + n * 16usize) as _) }
    }
}
#[doc = "Monitoring"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mon {
    ptr: *mut u8,
}
unsafe impl Send for Mon {}
unsafe impl Sync for Mon {}
impl Mon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Request Select for Monitor mon_index"]
    #[inline(always)]
    pub const fn intm_irqsel(self) -> crate::common::Reg<regs::IntmIrqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt Latency for Monitor mon_index"]
    #[inline(always)]
    pub const fn intm_latency(self) -> crate::common::Reg<regs::IntmLatency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer for Monitor mon_index"]
    #[inline(always)]
    pub const fn intm_timer(self) -> crate::common::Reg<regs::IntmTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status for Monitor mon_index"]
    #[inline(always)]
    pub const fn intm_status(self) -> crate::common::Reg<regs::IntmStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
