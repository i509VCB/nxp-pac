#[doc = "Real-Time Clock (RTC)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC match register"]
    #[inline(always)]
    pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC counter register"]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "High-resolution/wake-up timer control register"]
    #[inline(always)]
    pub const fn wake(self) -> crate::common::Reg<regs::Wake, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Sub-second counter register"]
    #[inline(always)]
    pub const fn subsec(self) -> crate::common::Reg<regs::Subsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "General Purpose register"]
    #[inline(always)]
    pub const fn gpreg(self, n: usize) -> crate::common::Reg<regs::Gpreg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
