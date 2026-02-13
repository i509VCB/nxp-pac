#[doc = "WAKE_TIMER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waketimer {
    ptr: *mut u8,
}
unsafe impl Send for Waketimer {}
unsafe impl Sync for Waketimer {}
impl Waketimer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Wake Timer Control"]
    #[inline(always)]
    pub const fn wake_timer_ctrl(
        self,
    ) -> crate::common::Reg<regs::WakeTimerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Wake Timer Counter"]
    #[inline(always)]
    pub const fn wake_timer_cnt(self) -> crate::common::Reg<regs::WakeTimerCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
