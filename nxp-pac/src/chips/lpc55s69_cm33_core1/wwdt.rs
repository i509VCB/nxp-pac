#[doc = "Windowed Watchdog Timer (WWDT)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdt {
    ptr: *mut u8,
}
unsafe impl Send for Wwdt {}
unsafe impl Sync for Wwdt {}
impl Wwdt {
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
    pub const fn mod_(self) -> crate::common::Reg<regs::Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
    #[inline(always)]
    pub const fn tc(self) -> crate::common::Reg<regs::Tc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
    #[inline(always)]
    pub const fn feed(self) -> crate::common::Reg<regs::Feed, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
    #[inline(always)]
    pub const fn tv(self) -> crate::common::Reg<regs::Tv, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Watchdog Warning Interrupt compare value."]
    #[inline(always)]
    pub const fn warnint(self) -> crate::common::Reg<regs::Warnint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Watchdog Window compare value."]
    #[inline(always)]
    pub const fn window(self) -> crate::common::Reg<regs::Window, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
pub mod vals;
