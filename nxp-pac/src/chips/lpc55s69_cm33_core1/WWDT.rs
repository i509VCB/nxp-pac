#[doc = "Windowed Watchdog Timer (WWDT)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WWDT {
    ptr: *mut u8,
}
unsafe impl Send for WWDT {}
unsafe impl Sync for WWDT {}
impl WWDT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
    #[inline(always)]
    pub const fn MOD(self) -> crate::common::Reg<regs::MOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
    #[inline(always)]
    pub const fn TC(self) -> crate::common::Reg<regs::TC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
    #[inline(always)]
    pub const fn FEED(self) -> crate::common::Reg<regs::FEED, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
    #[inline(always)]
    pub const fn TV(self) -> crate::common::Reg<regs::TV, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Watchdog Warning Interrupt compare value."]
    #[inline(always)]
    pub const fn WARNINT(self) -> crate::common::Reg<regs::WARNINT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Watchdog Window compare value."]
    #[inline(always)]
    pub const fn WINDOW(self) -> crate::common::Reg<regs::WINDOW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
pub mod vals;
