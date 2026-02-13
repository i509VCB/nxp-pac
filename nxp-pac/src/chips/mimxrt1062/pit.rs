#[doc = "PIT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pit {
    ptr: *mut u8,
}
unsafe impl Send for Pit {}
unsafe impl Sync for Pit {}
impl Pit {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PIT Module Control Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PIT Upper Lifetime Timer Register"]
    #[inline(always)]
    pub const fn ltmr64h(self) -> crate::common::Reg<regs::Ltmr64h, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "PIT Lower Lifetime Timer Register"]
    #[inline(always)]
    pub const fn ltmr64l(self) -> crate::common::Reg<regs::Ltmr64l, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Timer array."]
    #[inline(always)]
    pub const fn timer(self, n: usize) -> Timer {
        assert!(n < 4usize);
        unsafe { Timer::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 16usize) as _) }
    }
}
#[doc = "Timer array."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer {
    ptr: *mut u8,
}
unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}
impl Timer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timer Load Value Register"]
    #[inline(always)]
    pub const fn ldval(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Current Timer Value Register"]
    #[inline(always)]
    pub const fn cval(self) -> crate::common::Reg<regs::Cval, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer Control Register"]
    #[inline(always)]
    pub const fn tctrl(self) -> crate::common::Reg<regs::Tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Timer Flag Register"]
    #[inline(always)]
    pub const fn tflg(self) -> crate::common::Reg<regs::Tflg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
