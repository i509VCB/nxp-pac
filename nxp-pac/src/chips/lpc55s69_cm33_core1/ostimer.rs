#[doc = "Synchronous OS/Event timer with Wakeup Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostimer {
    ptr: *mut u8,
}
unsafe impl Send for Ostimer {}
unsafe impl Sync for Ostimer {}
impl Ostimer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EVTIMER Low Register"]
    #[inline(always)]
    pub const fn evtimerl(self) -> crate::common::Reg<regs::Evtimerl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "EVTIMER High Register"]
    #[inline(always)]
    pub const fn evtimerh(self) -> crate::common::Reg<regs::Evtimerh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Capture Low Register"]
    #[inline(always)]
    pub const fn capture_l(self) -> crate::common::Reg<regs::CaptureL, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Capture High Register"]
    #[inline(always)]
    pub const fn capture_h(self) -> crate::common::Reg<regs::CaptureH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Match Low Register"]
    #[inline(always)]
    pub const fn match_l(self) -> crate::common::Reg<regs::MatchL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Match High Register"]
    #[inline(always)]
    pub const fn match_h(self) -> crate::common::Reg<regs::MatchH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "OS_EVENT TIMER Control Register"]
    #[inline(always)]
    pub const fn osevent_ctrl(self) -> crate::common::Reg<regs::OseventCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
