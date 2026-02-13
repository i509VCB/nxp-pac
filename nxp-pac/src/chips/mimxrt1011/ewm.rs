#[doc = "EWM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ewm {
    ptr: *mut u8,
}
unsafe impl Send for Ewm {}
unsafe impl Sync for Ewm {}
impl Ewm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Service Register"]
    #[inline(always)]
    pub const fn serv(self) -> crate::common::Reg<regs::Serv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Compare Low Register"]
    #[inline(always)]
    pub const fn cmpl(self) -> crate::common::Reg<regs::Cmpl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Compare High Register"]
    #[inline(always)]
    pub const fn cmph(self) -> crate::common::Reg<regs::Cmph, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Clock Control Register"]
    #[inline(always)]
    pub const fn clkctrl(self) -> crate::common::Reg<regs::Clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock Prescaler Register"]
    #[inline(always)]
    pub const fn clkprescaler(self) -> crate::common::Reg<regs::Clkprescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
}
pub mod regs;
pub mod vals;
