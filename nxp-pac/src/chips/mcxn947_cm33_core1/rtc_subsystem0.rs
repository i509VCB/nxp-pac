#[doc = "RTC_SUBSYSTEM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcSubsystem0 {
    ptr: *mut u8,
}
unsafe impl Send for RtcSubsystem0 {}
unsafe impl Sync for RtcSubsystem0 {}
impl RtcSubsystem0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Subsecond Control"]
    #[inline(always)]
    pub const fn subsecond_ctrl(
        self,
    ) -> crate::common::Reg<regs::SubsecondCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Subsecond Counter"]
    #[inline(always)]
    pub const fn subsecond_cnt(self) -> crate::common::Reg<regs::SubsecondCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Wake Timer Control"]
    #[inline(always)]
    pub const fn wake_timer_ctrl(
        self,
    ) -> crate::common::Reg<regs::WakeTimerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Wake Timer Counter"]
    #[inline(always)]
    pub const fn wake_timer_cnt(self) -> crate::common::Reg<regs::WakeTimerCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c0cusize) as _) }
    }
}
pub mod regs;
