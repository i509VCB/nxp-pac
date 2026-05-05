#[doc = "Synchronous OS/Event timer with Wakeup Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSTIMER {
    ptr: *mut u8,
}
unsafe impl Send for OSTIMER {}
unsafe impl Sync for OSTIMER {}
impl OSTIMER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EVTIMER Low Register."]
    #[inline(always)]
    pub const fn EVTIMERL(self) -> crate::common::Reg<regs::EVTIMERL, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "EVTIMER High Register."]
    #[inline(always)]
    pub const fn EVTIMERH(self) -> crate::common::Reg<regs::EVTIMERH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Capture Low Register."]
    #[inline(always)]
    pub const fn CAPTURE_L(self) -> crate::common::Reg<regs::CAPTURE_L, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Capture High Register."]
    #[inline(always)]
    pub const fn CAPTURE_H(self) -> crate::common::Reg<regs::CAPTURE_H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Match Low Register."]
    #[inline(always)]
    pub const fn MATCH_L(self) -> crate::common::Reg<regs::MATCH_L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Match High Register."]
    #[inline(always)]
    pub const fn MATCH_H(self) -> crate::common::Reg<regs::MATCH_H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "OS_EVENT TIMER Control Register."]
    #[inline(always)]
    pub const fn OSEVENT_CTRL(self) -> crate::common::Reg<regs::OSEVENT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
