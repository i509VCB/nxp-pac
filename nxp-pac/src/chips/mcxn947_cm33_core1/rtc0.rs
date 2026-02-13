#[doc = "RTC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc0 {
    ptr: *mut u8,
}
unsafe impl Send for Rtc0 {}
unsafe impl Sync for Rtc0 {}
impl Rtc0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Year and Month Counters"]
    #[inline(always)]
    pub const fn yearmon(self) -> crate::common::Reg<regs::Yearmon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Days and Day-of-Week Counters"]
    #[inline(always)]
    pub const fn days(self) -> crate::common::Reg<regs::Days, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Hours and Minutes Counters"]
    #[inline(always)]
    pub const fn hourmin(self) -> crate::common::Reg<regs::Hourmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Seconds Counters"]
    #[inline(always)]
    pub const fn seconds(self) -> crate::common::Reg<regs::Seconds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Year and Months Alarm"]
    #[inline(always)]
    pub const fn alm_yearmon(self) -> crate::common::Reg<regs::AlmYearmon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Days Alarm"]
    #[inline(always)]
    pub const fn alm_days(self) -> crate::common::Reg<regs::AlmDays, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Hours and Minutes Alarm"]
    #[inline(always)]
    pub const fn alm_hourmin(self) -> crate::common::Reg<regs::AlmHourmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Seconds Alarm"]
    #[inline(always)]
    pub const fn alm_seconds(self) -> crate::common::Reg<regs::AlmSeconds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Sub Second Counter"]
    #[inline(always)]
    pub const fn rtc_test2(self) -> crate::common::Reg<regs::RtcTest2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Daylight Saving Hour"]
    #[inline(always)]
    pub const fn dst_hour(self) -> crate::common::Reg<regs::DstHour, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Daylight Saving Month"]
    #[inline(always)]
    pub const fn dst_month(self) -> crate::common::Reg<regs::DstMonth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Daylight Saving Day"]
    #[inline(always)]
    pub const fn dst_day(self) -> crate::common::Reg<regs::DstDay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Compensation"]
    #[inline(always)]
    pub const fn compen(self) -> crate::common::Reg<regs::Compen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs;
pub mod vals;
