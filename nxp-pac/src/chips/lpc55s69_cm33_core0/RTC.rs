#[doc = "Real-Time Clock (RTC)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTC {
    ptr: *mut u8,
}
unsafe impl Send for RTC {}
unsafe impl Sync for RTC {}
impl RTC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC control register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC match register."]
    #[inline(always)]
    pub const fn MATCH(self) -> crate::common::Reg<regs::MATCH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC counter register."]
    #[inline(always)]
    pub const fn COUNT(self) -> crate::common::Reg<regs::COUNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "High-resolution/wake-up timer control register."]
    #[inline(always)]
    pub const fn WAKE(self) -> crate::common::Reg<regs::WAKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Sub-second counter register."]
    #[inline(always)]
    pub const fn SUBSEC(self) -> crate::common::Reg<regs::SUBSEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "General Purpose register."]
    #[inline(always)]
    pub const fn GPREG(self, n: usize) -> crate::common::Reg<regs::GPREG, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
