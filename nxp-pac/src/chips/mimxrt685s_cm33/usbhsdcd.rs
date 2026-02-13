#[doc = "USBDCD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhsdcd {
    ptr: *mut u8,
}
unsafe impl Send for Usbhsdcd {}
unsafe impl Sync for Usbhsdcd {}
impl Usbhsdcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Clock register"]
    #[inline(always)]
    pub const fn clock(self) -> crate::common::Reg<regs::Clock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Signal Override Register"]
    #[inline(always)]
    pub const fn signal_override(
        self,
    ) -> crate::common::Reg<regs::SignalOverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TIMER0 register"]
    #[inline(always)]
    pub const fn timer0(self) -> crate::common::Reg<regs::Timer0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TIMER1 register"]
    #[inline(always)]
    pub const fn timer1(self) -> crate::common::Reg<regs::Timer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TIMER2_BC11 register"]
    #[inline(always)]
    pub const fn timer2_bc11(self) -> crate::common::Reg<regs::Timer2Bc11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TIMER2_BC12 register"]
    #[inline(always)]
    pub const fn timer2_bc12(self) -> crate::common::Reg<regs::Timer2Bc12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
pub mod vals;
