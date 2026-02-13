#[doc = "OSTIMER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostimer0 {
    ptr: *mut u8,
}
unsafe impl Send for Ostimer0 {}
unsafe impl Sync for Ostimer0 {}
impl Ostimer0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EVTIMER Low"]
    #[inline(always)]
    pub const fn evtimerl(self) -> crate::common::Reg<regs::Evtimerl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "EVTIMER High"]
    #[inline(always)]
    pub const fn evtimerh(self) -> crate::common::Reg<regs::Evtimerh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Local Capture Low for CPU"]
    #[inline(always)]
    pub const fn capture_l(self) -> crate::common::Reg<regs::CaptureL, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Local Capture High for CPU"]
    #[inline(always)]
    pub const fn capture_h(self) -> crate::common::Reg<regs::CaptureH, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Local Match Low for CPU"]
    #[inline(always)]
    pub const fn match_l(self) -> crate::common::Reg<regs::MatchL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Local Match High for CPU"]
    #[inline(always)]
    pub const fn match_h(self) -> crate::common::Reg<regs::MatchH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "OSTIMER Control for CPU"]
    #[inline(always)]
    pub const fn osevent_ctrl(self) -> crate::common::Reg<regs::OseventCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
