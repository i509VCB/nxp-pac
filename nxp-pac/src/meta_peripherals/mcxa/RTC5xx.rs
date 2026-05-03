#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Real Time Clock."]
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
    #[doc = "Year and Month Counters."]
    #[inline(always)]
    pub const fn yearmon(self) -> crate::pac::common::Reg<Yearmon, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Days and Day-of-Week Counters."]
    #[inline(always)]
    pub const fn days(self) -> crate::pac::common::Reg<Days, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Hours and Minutes Counters."]
    #[inline(always)]
    pub const fn hourmin(self) -> crate::pac::common::Reg<Hourmin, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Seconds Counters."]
    #[inline(always)]
    pub const fn seconds(self) -> crate::pac::common::Reg<Seconds, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Year and Months Alarm."]
    #[inline(always)]
    pub const fn alm_yearmon(self) -> crate::pac::common::Reg<AlmYearmon, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Days Alarm."]
    #[inline(always)]
    pub const fn alm_days(self) -> crate::pac::common::Reg<AlmDays, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Hours and Minutes Alarm."]
    #[inline(always)]
    pub const fn alm_hourmin(self) -> crate::pac::common::Reg<AlmHourmin, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Seconds Alarm."]
    #[inline(always)]
    pub const fn alm_seconds(self) -> crate::pac::common::Reg<AlmSeconds, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::pac::common::Reg<Status, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Byte Access to Status"]
    #[inline(always)]
    pub const fn status8(self) -> crate::pac::common::Reg<u8, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn isr(self) -> crate::pac::common::Reg<Isr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Minutes Count Down Timer."]
    #[inline(always)]
    pub const fn cntdwn_timer(
        self,
    ) -> crate::pac::common::Reg<CntdwnTimer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Daylight Saving Hour."]
    #[inline(always)]
    pub const fn dst_hour(self) -> crate::pac::common::Reg<DstHour, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Daylight Saving Month."]
    #[inline(always)]
    pub const fn dst_month(self) -> crate::pac::common::Reg<DstMonth, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Daylight Saving Day."]
    #[inline(always)]
    pub const fn dst_day(self) -> crate::pac::common::Reg<DstDay, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Compensation."]
    #[inline(always)]
    pub const fn compen(self) -> crate::pac::common::Reg<Compen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Tamper Queue Status and Control."]
    #[inline(always)]
    pub const fn tamper_qscr(self) -> crate::pac::common::Reg<TamperQscr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Tamper Time Stamp Year."]
    #[inline(always)]
    pub const fn ttsr_year(self) -> crate::pac::common::Reg<TtsrYear, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Tamper Status and Control."]
    #[inline(always)]
    pub const fn tamper_scr(self) -> crate::pac::common::Reg<TamperScr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Tamper 01 Filter Configuration."]
    #[inline(always)]
    pub const fn filter01_cfg(
        self,
    ) -> crate::pac::common::Reg<Filter01Cfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Tamper 23 Filter Configuration."]
    #[inline(always)]
    pub const fn filter23_cfg(
        self,
    ) -> crate::pac::common::Reg<Filter23Cfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Tamper 05 Filter Configuration."]
    #[inline(always)]
    pub const fn filter45_cfg(
        self,
    ) -> crate::pac::common::Reg<Filter45Cfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Tamper 67 Filter Configuration."]
    #[inline(always)]
    pub const fn filter67_cfg(
        self,
    ) -> crate::pac::common::Reg<Filter67Cfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "Tamper Queue."]
    #[inline(always)]
    pub const fn tamper_queue(self) -> crate::pac::common::Reg<TamperQueue, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Control 2."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::pac::common::Reg<Ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
}
#[doc = "Days Alarm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmDays(pub u16);
impl AlmDays {
    #[doc = "Days Value for Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Days Value for Alarm."]
    #[inline(always)]
    pub const fn set_alm_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
}
impl Default for AlmDays {
    #[inline(always)]
    fn default() -> AlmDays {
        AlmDays(0)
    }
}
impl core::fmt::Debug for AlmDays {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlmDays")
            .field("alm_day", &self.alm_day())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlmDays {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlmDays {{ alm_day: {=u8:?} }}", self.alm_day())
    }
}
#[doc = "Hours and Minutes Alarm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmHourmin(pub u16);
impl AlmHourmin {
    #[doc = "Minutes Value for Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_min(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes Value for Alarm."]
    #[inline(always)]
    pub const fn set_alm_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Hours Value for Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_hour(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours Value for Alarm."]
    #[inline(always)]
    pub const fn set_alm_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
}
impl Default for AlmHourmin {
    #[inline(always)]
    fn default() -> AlmHourmin {
        AlmHourmin(0)
    }
}
impl core::fmt::Debug for AlmHourmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlmHourmin")
            .field("alm_min", &self.alm_min())
            .field("alm_hour", &self.alm_hour())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlmHourmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AlmHourmin {{ alm_min: {=u8:?}, alm_hour: {=u8:?} }}",
            self.alm_min(),
            self.alm_hour()
        )
    }
}
#[doc = "Seconds Alarm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmSeconds(pub u16);
impl AlmSeconds {
    #[doc = "Seconds Alarm Value."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds Alarm Value."]
    #[inline(always)]
    pub const fn set_alm_sec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Decrement Seconds Counter by 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dec_sec(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Decrement Seconds Counter by 1."]
    #[inline(always)]
    pub const fn set_dec_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Increment Seconds Counter by 1."]
    #[must_use]
    #[inline(always)]
    pub const fn inc_sec(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Increment Seconds Counter by 1."]
    #[inline(always)]
    pub const fn set_inc_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for AlmSeconds {
    #[inline(always)]
    fn default() -> AlmSeconds {
        AlmSeconds(0)
    }
}
impl core::fmt::Debug for AlmSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlmSeconds")
            .field("alm_sec", &self.alm_sec())
            .field("dec_sec", &self.dec_sec())
            .field("inc_sec", &self.inc_sec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlmSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AlmSeconds {{ alm_sec: {=u8:?}, dec_sec: {=bool:?}, inc_sec: {=bool:?} }}",
            self.alm_sec(),
            self.dec_sec(),
            self.inc_sec()
        )
    }
}
#[doc = "Year and Months Alarm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmYearmon(pub u16);
impl AlmYearmon {
    #[doc = "Months Value for Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_mon(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Months Value for Alarm."]
    #[inline(always)]
    pub const fn set_alm_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Year Value for Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_year(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Year Value for Alarm."]
    #[inline(always)]
    pub const fn set_alm_year(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for AlmYearmon {
    #[inline(always)]
    fn default() -> AlmYearmon {
        AlmYearmon(0)
    }
}
impl core::fmt::Debug for AlmYearmon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlmYearmon")
            .field("alm_mon", &self.alm_mon())
            .field("alm_year", &self.alm_year())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlmYearmon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AlmYearmon {{ alm_mon: {=u8:?}, alm_year: {=u8:?} }}",
            self.alm_mon(),
            self.alm_year()
        )
    }
}
#[doc = "Minutes Count Down Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CntdwnTimer(pub u16);
impl CntdwnTimer {
    #[doc = "Count Down Timer Value."]
    #[must_use]
    #[inline(always)]
    pub const fn count_down_timer(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count Down Timer Value."]
    #[inline(always)]
    pub const fn set_count_down_timer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for CntdwnTimer {
    #[inline(always)]
    fn default() -> CntdwnTimer {
        CntdwnTimer(0)
    }
}
impl core::fmt::Debug for CntdwnTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CntdwnTimer")
            .field("count_down_timer", &self.count_down_timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CntdwnTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CntdwnTimer {{ count_down_timer: {=u8:?} }}",
            self.count_down_timer()
        )
    }
}
#[doc = "Compensation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compen(pub u16);
impl Compen {
    #[doc = "Compensation Value."]
    #[must_use]
    #[inline(always)]
    pub const fn compen_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compensation Value."]
    #[inline(always)]
    pub const fn set_compen_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Compen {
    #[inline(always)]
    fn default() -> Compen {
        Compen(0)
    }
}
impl core::fmt::Debug for Compen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compen")
            .field("compen_val", &self.compen_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Compen {{ compen_val: {=u16:?} }}", self.compen_val())
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u16);
impl Ctrl {
    #[doc = "Fine Compensation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fineen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fine Compensation Enable."]
    #[inline(always)]
    pub const fn set_fineen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Compensation Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn comp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation Enable."]
    #[inline(always)]
    pub const fn set_comp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Alarm Match."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_match(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Alarm Match."]
    #[inline(always)]
    pub const fn set_alm_match(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
    }
    #[doc = "Daylight Saving Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Daylight Saving Enable."]
    #[inline(always)]
    pub const fn set_dst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "BCD Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bcd_en(&self) -> BcdEn {
        let val = (self.0 >> 7usize) & 0x01;
        BcdEn::from_bits(val as u8)
    }
    #[doc = "BCD Mode Enable."]
    #[inline(always)]
    pub const fn set_bcd_en(&mut self, val: BcdEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> Swr {
        let val = (self.0 >> 8usize) & 0x01;
        Swr::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: Swr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "RTC Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Clock Output Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn clko_dis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Output Disable."]
    #[inline(always)]
    pub const fn set_clko_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "RTC Clock Output Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn clkout(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "RTC Clock Output Selection."]
    #[inline(always)]
    pub const fn set_clkout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u16) & 0x03) << 13usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("fineen", &self.fineen())
            .field("comp_en", &self.comp_en())
            .field("alm_match", &self.alm_match())
            .field("dst_en", &self.dst_en())
            .field("bcd_en", &self.bcd_en())
            .field("swr", &self.swr())
            .field("clk_sel", &self.clk_sel())
            .field("clko_dis", &self.clko_dis())
            .field("clkout", &self.clkout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ fineen: {=bool:?}, comp_en: {=bool:?}, alm_match: {=u8:?}, dst_en: {=bool:?}, bcd_en: {:?}, swr: {:?}, clk_sel: {=bool:?}, clko_dis: {=bool:?}, clkout: {=u8:?} }}",
            self.fineen(),
            self.comp_en(),
            self.alm_match(),
            self.dst_en(),
            self.bcd_en(),
            self.swr(),
            self.clk_sel(),
            self.clko_dis(),
            self.clkout()
        )
    }
}
#[doc = "Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u16);
impl Ctrl2 {
    #[doc = "Tamper Configuration Over."]
    #[must_use]
    #[inline(always)]
    pub const fn tamp_cfg_over(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Configuration Over."]
    #[inline(always)]
    pub const fn set_tamp_cfg_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Status."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_status(&self) -> WakeupStatus {
        let val = (self.0 >> 5usize) & 0x03;
        WakeupStatus::from_bits(val as u8)
    }
    #[doc = "Wakeup Status."]
    #[inline(always)]
    pub const fn set_wakeup_status(&mut self, val: WakeupStatus) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u16) & 0x03) << 5usize);
    }
    #[doc = "Wakeup Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_mode(&self) -> WakeupMode {
        let val = (self.0 >> 7usize) & 0x01;
        WakeupMode::from_bits(val as u8)
    }
    #[doc = "Wakeup Mode."]
    #[inline(always)]
    pub const fn set_wakeup_mode(&mut self, val: WakeupMode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("tamp_cfg_over", &self.tamp_cfg_over())
            .field("wakeup_status", &self.wakeup_status())
            .field("wakeup_mode", &self.wakeup_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ tamp_cfg_over: {=bool:?}, wakeup_status: {:?}, wakeup_mode: {:?} }}",
            self.tamp_cfg_over(),
            self.wakeup_status(),
            self.wakeup_mode()
        )
    }
}
#[doc = "Days and Day-of-Week Counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Days(pub u16);
impl Days {
    #[doc = "Days Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn day_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Days Counter Value."]
    #[inline(always)]
    pub const fn set_day_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Day of Week Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dow(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Day of Week Counter Value."]
    #[inline(always)]
    pub const fn set_dow(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Days {
    #[inline(always)]
    fn default() -> Days {
        Days(0)
    }
}
impl core::fmt::Debug for Days {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Days")
            .field("day_cnt", &self.day_cnt())
            .field("dow", &self.dow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Days {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Days {{ day_cnt: {=u8:?}, dow: {=u8:?} }}",
            self.day_cnt(),
            self.dow()
        )
    }
}
#[doc = "Daylight Saving Day."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstDay(pub u16);
impl DstDay {
    #[doc = "Daylight Saving Time (DST) Day End Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_end_day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Day End Value."]
    #[inline(always)]
    pub const fn set_dst_end_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Daylight Saving Time (DST) Day Start Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_start_day(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Day Start Value."]
    #[inline(always)]
    pub const fn set_dst_start_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
}
impl Default for DstDay {
    #[inline(always)]
    fn default() -> DstDay {
        DstDay(0)
    }
}
impl core::fmt::Debug for DstDay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DstDay")
            .field("dst_end_day", &self.dst_end_day())
            .field("dst_start_day", &self.dst_start_day())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DstDay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DstDay {{ dst_end_day: {=u8:?}, dst_start_day: {=u8:?} }}",
            self.dst_end_day(),
            self.dst_start_day()
        )
    }
}
#[doc = "Daylight Saving Hour."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstHour(pub u16);
impl DstHour {
    #[doc = "Daylight Saving Time (DST) Hours End Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_end_hour(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Hours End Value."]
    #[inline(always)]
    pub const fn set_dst_end_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Daylight Saving Time (DST) Hours Start Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_start_hour(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Hours Start Value."]
    #[inline(always)]
    pub const fn set_dst_start_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
}
impl Default for DstHour {
    #[inline(always)]
    fn default() -> DstHour {
        DstHour(0)
    }
}
impl core::fmt::Debug for DstHour {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DstHour")
            .field("dst_end_hour", &self.dst_end_hour())
            .field("dst_start_hour", &self.dst_start_hour())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DstHour {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DstHour {{ dst_end_hour: {=u8:?}, dst_start_hour: {=u8:?} }}",
            self.dst_end_hour(),
            self.dst_start_hour()
        )
    }
}
#[doc = "Daylight Saving Month."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstMonth(pub u16);
impl DstMonth {
    #[doc = "Daylight Saving Time (DST) Month End Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_end_month(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Month End Value."]
    #[inline(always)]
    pub const fn set_dst_end_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Daylight Saving Time (DST) Month Start Value."]
    #[must_use]
    #[inline(always)]
    pub const fn dst_start_month(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Month Start Value."]
    #[inline(always)]
    pub const fn set_dst_start_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for DstMonth {
    #[inline(always)]
    fn default() -> DstMonth {
        DstMonth(0)
    }
}
impl core::fmt::Debug for DstMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DstMonth")
            .field("dst_end_month", &self.dst_end_month())
            .field("dst_start_month", &self.dst_start_month())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DstMonth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DstMonth {{ dst_end_month: {=u8:?}, dst_start_month: {=u8:?} }}",
            self.dst_end_month(),
            self.dst_start_month()
        )
    }
}
#[doc = "Tamper 01 Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filter01Cfg(pub u16);
impl Filter01Cfg {
    #[doc = "Tamper Detect Bit 1 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur1(&self) -> FilDur1 {
        let val = (self.0 >> 0usize) & 0x0f;
        FilDur1::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 1 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur1(&mut self, val: FilDur1) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Tamper Filter 1 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel1(&self) -> ClkSel1 {
        let val = (self.0 >> 4usize) & 0x07;
        ClkSel1::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 1 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel1(&mut self, val: ClkSel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Tamper Detect Input Bit 1 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol1(&self) -> Pol1 {
        let val = (self.0 >> 7usize) & 0x01;
        Pol1::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 1 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol1(&mut self, val: Pol1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Tamper Detect Bit 0 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur0(&self) -> FilDur0 {
        let val = (self.0 >> 8usize) & 0x0f;
        FilDur0::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 0 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur0(&mut self, val: FilDur0) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Tamper Filter 0 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel0(&self) -> ClkSel0 {
        let val = (self.0 >> 12usize) & 0x07;
        ClkSel0::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 0 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel0(&mut self, val: ClkSel0) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u16) & 0x07) << 12usize);
    }
    #[doc = "Tamper Detect Input Bit 0 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol0(&self) -> Pol0 {
        let val = (self.0 >> 15usize) & 0x01;
        Pol0::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 0 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol0(&mut self, val: Pol0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Filter01Cfg {
    #[inline(always)]
    fn default() -> Filter01Cfg {
        Filter01Cfg(0)
    }
}
impl core::fmt::Debug for Filter01Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filter01Cfg")
            .field("fil_dur1", &self.fil_dur1())
            .field("clk_sel1", &self.clk_sel1())
            .field("pol1", &self.pol1())
            .field("fil_dur0", &self.fil_dur0())
            .field("clk_sel0", &self.clk_sel0())
            .field("pol0", &self.pol0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filter01Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filter01Cfg {{ fil_dur1: {:?}, clk_sel1: {:?}, pol1: {:?}, fil_dur0: {:?}, clk_sel0: {:?}, pol0: {:?} }}",
            self.fil_dur1(),
            self.clk_sel1(),
            self.pol1(),
            self.fil_dur0(),
            self.clk_sel0(),
            self.pol0()
        )
    }
}
#[doc = "Tamper 23 Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filter23Cfg(pub u16);
impl Filter23Cfg {
    #[doc = "Tamper Detect Bit 3 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur3(&self) -> FilDur3 {
        let val = (self.0 >> 0usize) & 0x0f;
        FilDur3::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 3 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur3(&mut self, val: FilDur3) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Tamper Filter 3 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel3(&self) -> ClkSel3 {
        let val = (self.0 >> 4usize) & 0x07;
        ClkSel3::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 3 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel3(&mut self, val: ClkSel3) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Tamper Detect Input Bit 3 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol3(&self) -> Pol3 {
        let val = (self.0 >> 7usize) & 0x01;
        Pol3::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 3 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol3(&mut self, val: Pol3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Tamper Detect Bit 2 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur2(&self) -> FilDur2 {
        let val = (self.0 >> 8usize) & 0x0f;
        FilDur2::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 2 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur2(&mut self, val: FilDur2) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Tamper Filter 2 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel2(&self) -> ClkSel2 {
        let val = (self.0 >> 12usize) & 0x07;
        ClkSel2::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 2 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel2(&mut self, val: ClkSel2) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u16) & 0x07) << 12usize);
    }
    #[doc = "Tamper Detect Input Bit 2 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol2(&self) -> Pol2 {
        let val = (self.0 >> 15usize) & 0x01;
        Pol2::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 2 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol2(&mut self, val: Pol2) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Filter23Cfg {
    #[inline(always)]
    fn default() -> Filter23Cfg {
        Filter23Cfg(0)
    }
}
impl core::fmt::Debug for Filter23Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filter23Cfg")
            .field("fil_dur3", &self.fil_dur3())
            .field("clk_sel3", &self.clk_sel3())
            .field("pol3", &self.pol3())
            .field("fil_dur2", &self.fil_dur2())
            .field("clk_sel2", &self.clk_sel2())
            .field("pol2", &self.pol2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filter23Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filter23Cfg {{ fil_dur3: {:?}, clk_sel3: {:?}, pol3: {:?}, fil_dur2: {:?}, clk_sel2: {:?}, pol2: {:?} }}",
            self.fil_dur3(),
            self.clk_sel3(),
            self.pol3(),
            self.fil_dur2(),
            self.clk_sel2(),
            self.pol2()
        )
    }
}
#[doc = "Tamper 05 Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filter45Cfg(pub u16);
impl Filter45Cfg {
    #[doc = "Tamper Detect Bit 5 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur5(&self) -> FilDur5 {
        let val = (self.0 >> 0usize) & 0x0f;
        FilDur5::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 5 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur5(&mut self, val: FilDur5) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Tamper Filter 5 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel5(&self) -> ClkSel5 {
        let val = (self.0 >> 4usize) & 0x07;
        ClkSel5::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 5 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel5(&mut self, val: ClkSel5) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Tamper Detect Input Bit 5 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol5(&self) -> Pol5 {
        let val = (self.0 >> 7usize) & 0x01;
        Pol5::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 5 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol5(&mut self, val: Pol5) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Tamper Detect Bit 4 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur4(&self) -> FilDur4 {
        let val = (self.0 >> 8usize) & 0x0f;
        FilDur4::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 4 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur4(&mut self, val: FilDur4) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Tamper Filter 4 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel4(&self) -> ClkSel4 {
        let val = (self.0 >> 12usize) & 0x07;
        ClkSel4::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 4 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel4(&mut self, val: ClkSel4) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u16) & 0x07) << 12usize);
    }
    #[doc = "Tamper Detect Input Bit 4 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol4(&self) -> Pol4 {
        let val = (self.0 >> 15usize) & 0x01;
        Pol4::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 4 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol4(&mut self, val: Pol4) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Filter45Cfg {
    #[inline(always)]
    fn default() -> Filter45Cfg {
        Filter45Cfg(0)
    }
}
impl core::fmt::Debug for Filter45Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filter45Cfg")
            .field("fil_dur5", &self.fil_dur5())
            .field("clk_sel5", &self.clk_sel5())
            .field("pol5", &self.pol5())
            .field("fil_dur4", &self.fil_dur4())
            .field("clk_sel4", &self.clk_sel4())
            .field("pol4", &self.pol4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filter45Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filter45Cfg {{ fil_dur5: {:?}, clk_sel5: {:?}, pol5: {:?}, fil_dur4: {:?}, clk_sel4: {:?}, pol4: {:?} }}",
            self.fil_dur5(),
            self.clk_sel5(),
            self.pol5(),
            self.fil_dur4(),
            self.clk_sel4(),
            self.pol4()
        )
    }
}
#[doc = "Tamper 67 Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filter67Cfg(pub u16);
impl Filter67Cfg {
    #[doc = "Tamper Detect Bit 7 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur7(&self) -> FilDur7 {
        let val = (self.0 >> 0usize) & 0x0f;
        FilDur7::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 7 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur7(&mut self, val: FilDur7) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Tamper Filter 7 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel7(&self) -> ClkSel7 {
        let val = (self.0 >> 4usize) & 0x07;
        ClkSel7::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 7 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel7(&mut self, val: ClkSel7) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Tamper Detect Input Bit 7 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol7(&self) -> Pol7 {
        let val = (self.0 >> 7usize) & 0x01;
        Pol7::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 7 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol7(&mut self, val: Pol7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Tamper Detect Bit 6 Filter Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_dur6(&self) -> FilDur6 {
        let val = (self.0 >> 8usize) & 0x0f;
        FilDur6::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Bit 6 Filter Duration."]
    #[inline(always)]
    pub const fn set_fil_dur6(&mut self, val: FilDur6) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Tamper Filter 6 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel6(&self) -> ClkSel6 {
        let val = (self.0 >> 12usize) & 0x07;
        ClkSel6::from_bits(val as u8)
    }
    #[doc = "Tamper Filter 6 Clock Select."]
    #[inline(always)]
    pub const fn set_clk_sel6(&mut self, val: ClkSel6) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u16) & 0x07) << 12usize);
    }
    #[doc = "Tamper Detect Input Bit 6 Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pol6(&self) -> Pol6 {
        let val = (self.0 >> 15usize) & 0x01;
        Pol6::from_bits(val as u8)
    }
    #[doc = "Tamper Detect Input Bit 6 Polarity Control."]
    #[inline(always)]
    pub const fn set_pol6(&mut self, val: Pol6) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Filter67Cfg {
    #[inline(always)]
    fn default() -> Filter67Cfg {
        Filter67Cfg(0)
    }
}
impl core::fmt::Debug for Filter67Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filter67Cfg")
            .field("fil_dur7", &self.fil_dur7())
            .field("clk_sel7", &self.clk_sel7())
            .field("pol7", &self.pol7())
            .field("fil_dur6", &self.fil_dur6())
            .field("clk_sel6", &self.clk_sel6())
            .field("pol6", &self.pol6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filter67Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filter67Cfg {{ fil_dur7: {:?}, clk_sel7: {:?}, pol7: {:?}, fil_dur6: {:?}, clk_sel6: {:?}, pol6: {:?} }}",
            self.fil_dur7(),
            self.clk_sel7(),
            self.pol7(),
            self.fil_dur6(),
            self.clk_sel6(),
            self.pol6()
        )
    }
}
#[doc = "Hours and Minutes Counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hourmin(pub u16);
impl Hourmin {
    #[doc = "Minutes Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn min_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes Counter Value."]
    #[inline(always)]
    pub const fn set_min_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Hours Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn hour_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours Counter Value."]
    #[inline(always)]
    pub const fn set_hour_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
}
impl Default for Hourmin {
    #[inline(always)]
    fn default() -> Hourmin {
        Hourmin(0)
    }
}
impl core::fmt::Debug for Hourmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hourmin")
            .field("min_cnt", &self.min_cnt())
            .field("hour_cnt", &self.hour_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hourmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hourmin {{ min_cnt: {=u8:?}, hour_cnt: {=u8:?} }}",
            self.min_cnt(),
            self.hour_cnt()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u16);
impl Ier {
    #[doc = "Tamper Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tamper_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tamper_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Count Down Timer Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_dn_timeout_ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Count Down Timer Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cnt_dn_timeout_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Alarm Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Interrupt Enable."]
    #[inline(always)]
    pub const fn set_alm_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Days Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn day_ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Days Interrupt Enable."]
    #[inline(always)]
    pub const fn set_day_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Hours Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hour_ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hours Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hour_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Minutes Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn min_ie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Minutes Interrupt Enable."]
    #[inline(always)]
    pub const fn set_min_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "1 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_1hz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "2 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_2hz(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "2 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_2hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "4 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_4hz(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "4 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_4hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "8 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_8hz(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "8 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_8hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "16 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_16hz(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "16 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_16hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "32 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_32hz(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "32 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_32hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "64 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_64hz(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "64 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_64hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "128 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_128hz(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "128 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_128hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "256 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_256hz(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "256 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_256hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "512 Hz Interval Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie_512hz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "512 Hz Interval Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie_512hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("tamper_ie", &self.tamper_ie())
            .field("cnt_dn_timeout_ie", &self.cnt_dn_timeout_ie())
            .field("alm_ie", &self.alm_ie())
            .field("day_ie", &self.day_ie())
            .field("hour_ie", &self.hour_ie())
            .field("min_ie", &self.min_ie())
            .field("ie_1hz", &self.ie_1hz())
            .field("ie_2hz", &self.ie_2hz())
            .field("ie_4hz", &self.ie_4hz())
            .field("ie_8hz", &self.ie_8hz())
            .field("ie_16hz", &self.ie_16hz())
            .field("ie_32hz", &self.ie_32hz())
            .field("ie_64hz", &self.ie_64hz())
            .field("ie_128hz", &self.ie_128hz())
            .field("ie_256hz", &self.ie_256hz())
            .field("ie_512hz", &self.ie_512hz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ tamper_ie: {=bool:?}, cnt_dn_timeout_ie: {=bool:?}, alm_ie: {=bool:?}, day_ie: {=bool:?}, hour_ie: {=bool:?}, min_ie: {=bool:?}, ie_1hz: {=bool:?}, ie_2hz: {=bool:?}, ie_4hz: {=bool:?}, ie_8hz: {=bool:?}, ie_16hz: {=bool:?}, ie_32hz: {=bool:?}, ie_64hz: {=bool:?}, ie_128hz: {=bool:?}, ie_256hz: {=bool:?}, ie_512hz: {=bool:?} }}",
            self.tamper_ie(),
            self.cnt_dn_timeout_ie(),
            self.alm_ie(),
            self.day_ie(),
            self.hour_ie(),
            self.min_ie(),
            self.ie_1hz(),
            self.ie_2hz(),
            self.ie_4hz(),
            self.ie_8hz(),
            self.ie_16hz(),
            self.ie_32hz(),
            self.ie_64hz(),
            self.ie_128hz(),
            self.ie_256hz(),
            self.ie_512hz()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u16);
impl Isr {
    #[doc = "Tamper Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tamper_is(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Interrupt Status."]
    #[inline(always)]
    pub const fn set_tamper_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Count Down Timer Timeout Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_dn_timeout_is(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Count Down Timer Timeout Interrupt Status."]
    #[inline(always)]
    pub const fn set_cnt_dn_timeout_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Alarm Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn alm_is(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Interrupt Status."]
    #[inline(always)]
    pub const fn set_alm_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Days Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn day_is(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Days Interrupt Status."]
    #[inline(always)]
    pub const fn set_day_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Hours Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn hour_is(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hours Interrupt Status."]
    #[inline(always)]
    pub const fn set_hour_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Minutes Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn min_is(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Minutes Interrupt Status."]
    #[inline(always)]
    pub const fn set_min_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "1 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_1hz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "2 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_2hz(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "2 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_2hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "4 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_4hz(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "4 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_4hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "8 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_8hz(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "8 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_8hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "16 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_16hz(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "16 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_16hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "32 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_32hz(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "32 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_32hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "64 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_64hz(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "64 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_64hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "128 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_128hz(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "128 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_128hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "256 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_256hz(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "256 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_256hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "512 Hz Interval Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn is_512hz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "512 Hz Interval Interrupt Status."]
    #[inline(always)]
    pub const fn set_is_512hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("tamper_is", &self.tamper_is())
            .field("cnt_dn_timeout_is", &self.cnt_dn_timeout_is())
            .field("alm_is", &self.alm_is())
            .field("day_is", &self.day_is())
            .field("hour_is", &self.hour_is())
            .field("min_is", &self.min_is())
            .field("is_1hz", &self.is_1hz())
            .field("is_2hz", &self.is_2hz())
            .field("is_4hz", &self.is_4hz())
            .field("is_8hz", &self.is_8hz())
            .field("is_16hz", &self.is_16hz())
            .field("is_32hz", &self.is_32hz())
            .field("is_64hz", &self.is_64hz())
            .field("is_128hz", &self.is_128hz())
            .field("is_256hz", &self.is_256hz())
            .field("is_512hz", &self.is_512hz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ tamper_is: {=bool:?}, cnt_dn_timeout_is: {=bool:?}, alm_is: {=bool:?}, day_is: {=bool:?}, hour_is: {=bool:?}, min_is: {=bool:?}, is_1hz: {=bool:?}, is_2hz: {=bool:?}, is_4hz: {=bool:?}, is_8hz: {=bool:?}, is_16hz: {=bool:?}, is_32hz: {=bool:?}, is_64hz: {=bool:?}, is_128hz: {=bool:?}, is_256hz: {=bool:?}, is_512hz: {=bool:?} }}",
            self.tamper_is(),
            self.cnt_dn_timeout_is(),
            self.alm_is(),
            self.day_is(),
            self.hour_is(),
            self.min_is(),
            self.is_1hz(),
            self.is_2hz(),
            self.is_4hz(),
            self.is_8hz(),
            self.is_16hz(),
            self.is_32hz(),
            self.is_64hz(),
            self.is_128hz(),
            self.is_256hz(),
            self.is_512hz()
        )
    }
}
#[doc = "Seconds Counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seconds(pub u16);
impl Seconds {
    #[doc = "Seconds Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds Counter Value."]
    #[inline(always)]
    pub const fn set_sec_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
}
impl Default for Seconds {
    #[inline(always)]
    fn default() -> Seconds {
        Seconds(0)
    }
}
impl core::fmt::Debug for Seconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seconds")
            .field("sec_cnt", &self.sec_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Seconds {{ sec_cnt: {=u8:?} }}", self.sec_cnt())
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u16);
impl Status {
    #[doc = "Invalidate CPU Read/Write Access."]
    #[must_use]
    #[inline(always)]
    pub const fn inval_bit(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Invalidate CPU Read/Write Access."]
    #[inline(always)]
    pub const fn set_inval_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Write Protect Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn write_prot_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write Protect Enable Status."]
    #[inline(always)]
    pub const fn set_write_prot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Down Counter Invalidate Read/Write Access."]
    #[must_use]
    #[inline(always)]
    pub const fn dwn_cntr_inval_bit(&self) -> DwnCntrInvalBit {
        let val = (self.0 >> 4usize) & 0x01;
        DwnCntrInvalBit::from_bits(val as u8)
    }
    #[doc = "Down Counter Invalidate Read/Write Access."]
    #[inline(always)]
    pub const fn set_dwn_cntr_inval_bit(&mut self, val: DwnCntrInvalBit) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Compensation Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation Interval."]
    #[inline(always)]
    pub const fn set_cmp_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Write Enable."]
    #[inline(always)]
    pub const fn set_we(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
    }
    #[doc = "Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_err(&self) -> BusErr {
        let val = (self.0 >> 8usize) & 0x01;
        BusErr::from_bits(val as u8)
    }
    #[doc = "Bus Error."]
    #[inline(always)]
    pub const fn set_bus_err(&mut self, val: BusErr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Compensation Done."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_done(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation Done."]
    #[inline(always)]
    pub const fn set_cmp_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("inval_bit", &self.inval_bit())
            .field("write_prot_en", &self.write_prot_en())
            .field("dwn_cntr_inval_bit", &self.dwn_cntr_inval_bit())
            .field("cmp_int", &self.cmp_int())
            .field("we", &self.we())
            .field("bus_err", &self.bus_err())
            .field("cmp_done", &self.cmp_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ inval_bit: {=bool:?}, write_prot_en: {=bool:?}, dwn_cntr_inval_bit: {:?}, cmp_int: {=bool:?}, we: {=u8:?}, bus_err: {:?}, cmp_done: {=bool:?} }}",
            self.inval_bit(),
            self.write_prot_en(),
            self.dwn_cntr_inval_bit(),
            self.cmp_int(),
            self.we(),
            self.bus_err(),
            self.cmp_done()
        )
    }
}
#[doc = "Tamper Queue Status and Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TamperQscr(pub u16);
impl TamperQscr {
    #[doc = "Q_FULL."]
    #[must_use]
    #[inline(always)]
    pub const fn q_full(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Q_FULL."]
    #[inline(always)]
    pub const fn set_q_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Q_FULL_INT_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn q_full_int_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Q_FULL_INT_EN."]
    #[inline(always)]
    pub const fn set_q_full_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Q_CLEAR."]
    #[must_use]
    #[inline(always)]
    pub const fn q_clear(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Q_CLEAR."]
    #[inline(always)]
    pub const fn set_q_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "LFSR_CLK_SEL."]
    #[must_use]
    #[inline(always)]
    pub const fn lfsr_clk_sel(&self) -> LfsrClkSel {
        let val = (self.0 >> 8usize) & 0x07;
        LfsrClkSel::from_bits(val as u8)
    }
    #[doc = "LFSR_CLK_SEL."]
    #[inline(always)]
    pub const fn set_lfsr_clk_sel(&mut self, val: LfsrClkSel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "LFSR_DURATION."]
    #[must_use]
    #[inline(always)]
    pub const fn lfsr_duration(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "LFSR_DURATION."]
    #[inline(always)]
    pub const fn set_lfsr_duration(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for TamperQscr {
    #[inline(always)]
    fn default() -> TamperQscr {
        TamperQscr(0)
    }
}
impl core::fmt::Debug for TamperQscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TamperQscr")
            .field("q_full", &self.q_full())
            .field("q_full_int_en", &self.q_full_int_en())
            .field("q_clear", &self.q_clear())
            .field("lfsr_clk_sel", &self.lfsr_clk_sel())
            .field("lfsr_duration", &self.lfsr_duration())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TamperQscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TamperQscr {{ q_full: {=bool:?}, q_full_int_en: {=bool:?}, q_clear: {=bool:?}, lfsr_clk_sel: {:?}, lfsr_duration: {=u8:?} }}",
            self.q_full(),
            self.q_full_int_en(),
            self.q_clear(),
            self.lfsr_clk_sel(),
            self.lfsr_duration()
        )
    }
}
#[doc = "Tamper Queue."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TamperQueue(pub u16);
impl TamperQueue {
    #[doc = "Tamper type stamp and pin number information register."]
    #[must_use]
    #[inline(always)]
    pub const fn tamper_data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Tamper type stamp and pin number information register."]
    #[inline(always)]
    pub const fn set_tamper_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TamperQueue {
    #[inline(always)]
    fn default() -> TamperQueue {
        TamperQueue(0)
    }
}
impl core::fmt::Debug for TamperQueue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TamperQueue")
            .field("tamper_data", &self.tamper_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TamperQueue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TamperQueue {{ tamper_data: {=u16:?} }}",
            self.tamper_data()
        )
    }
}
#[doc = "Tamper Status and Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TamperScr(pub u16);
impl TamperScr {
    #[doc = "Tamper Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmpr_en(&self) -> TmprEn {
        let val = (self.0 >> 0usize) & 0x0f;
        TmprEn::from_bits(val as u8)
    }
    #[doc = "Tamper Control."]
    #[inline(always)]
    pub const fn set_tmpr_en(&mut self, val: TmprEn) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Tamper Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tmpr_sts(&self) -> TmprSts {
        let val = (self.0 >> 8usize) & 0x0f;
        TmprSts::from_bits(val as u8)
    }
    #[doc = "Tamper Status."]
    #[inline(always)]
    pub const fn set_tmpr_sts(&mut self, val: TmprSts) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
}
impl Default for TamperScr {
    #[inline(always)]
    fn default() -> TamperScr {
        TamperScr(0)
    }
}
impl core::fmt::Debug for TamperScr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TamperScr")
            .field("tmpr_en", &self.tmpr_en())
            .field("tmpr_sts", &self.tmpr_sts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TamperScr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TamperScr {{ tmpr_en: {:?}, tmpr_sts: {:?} }}",
            self.tmpr_en(),
            self.tmpr_sts()
        )
    }
}
#[doc = "Tamper Time Stamp Year."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TtsrYear(pub u16);
impl TtsrYear {
    #[doc = "Year Value for Tamper Time Stamp."]
    #[must_use]
    #[inline(always)]
    pub const fn ts_yrofst(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Year Value for Tamper Time Stamp."]
    #[inline(always)]
    pub const fn set_ts_yrofst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for TtsrYear {
    #[inline(always)]
    fn default() -> TtsrYear {
        TtsrYear(0)
    }
}
impl core::fmt::Debug for TtsrYear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TtsrYear")
            .field("ts_yrofst", &self.ts_yrofst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TtsrYear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TtsrYear {{ ts_yrofst: {=u8:?} }}", self.ts_yrofst())
    }
}
#[doc = "Year and Month Counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Yearmon(pub u16);
impl Yearmon {
    #[doc = "Month Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn mon_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Month Counter."]
    #[inline(always)]
    pub const fn set_mon_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Year Offset Count Value."]
    #[must_use]
    #[inline(always)]
    pub const fn yrofst(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Year Offset Count Value."]
    #[inline(always)]
    pub const fn set_yrofst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Yearmon {
    #[inline(always)]
    fn default() -> Yearmon {
        Yearmon(0)
    }
}
impl core::fmt::Debug for Yearmon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Yearmon")
            .field("mon_cnt", &self.mon_cnt())
            .field("yrofst", &self.yrofst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Yearmon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Yearmon {{ mon_cnt: {=u8:?}, yrofst: {=u8:?} }}",
            self.mon_cnt(),
            self.yrofst()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BcdEn {
    #[doc = "Binary mode."]
    BinaryMode = 0x0,
    #[doc = "BCD mode."]
    BcdMode = 0x01,
}
impl BcdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BcdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BcdEn {
    #[inline(always)]
    fn from(val: u8) -> BcdEn {
        BcdEn::from_bits(val)
    }
}
impl From<BcdEn> for u8 {
    #[inline(always)]
    fn from(val: BcdEn) -> u8 {
        BcdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusErr {
    #[doc = "Read and write accesses are normal."]
    Normal = 0x0,
    #[doc = "Read or write accesses occurred when STATUS\\[INVAL_BIT\\] was asserted."]
    Asserted = 0x01,
}
impl BusErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusErr {
    #[inline(always)]
    fn from(val: u8) -> BusErr {
        BusErr::from_bits(val)
    }
}
impl From<BusErr> for u8 {
    #[inline(always)]
    fn from(val: BusErr) -> u8 {
        BusErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel0 {
    #[doc = "32/16 kHz clock."]
    Clk32khz = 0x0,
    #[doc = "512 Hz clock."]
    Clk512hz = 0x01,
    #[doc = "128 Hz clock."]
    Clk128hz = 0x02,
    #[doc = "64 Hz clock."]
    Clk64hz = 0x03,
    #[doc = "16 Hz clock."]
    Clk16hz = 0x04,
    #[doc = "8 Hz clock."]
    Clk8hz = 0x05,
    #[doc = "4 Hz clock."]
    Clk4hz = 0x06,
    #[doc = "2 Hz clock."]
    Clk2hz = 0x07,
}
impl ClkSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel0 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel0 {
        ClkSel0::from_bits(val)
    }
}
impl From<ClkSel0> for u8 {
    #[inline(always)]
    fn from(val: ClkSel0) -> u8 {
        ClkSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel1 {
    #[doc = "32/16 kHz clock."]
    Clk32khz = 0x0,
    #[doc = "512 Hz clock."]
    Clk512hz = 0x01,
    #[doc = "128 Hz clock."]
    Clk128hz = 0x02,
    #[doc = "64 Hz clock."]
    Clk64hz = 0x03,
    #[doc = "16 Hz clock."]
    Clk16hz = 0x04,
    #[doc = "8 Hz clock."]
    Clk8hz = 0x05,
    #[doc = "4 Hz clock."]
    Clk4hz = 0x06,
    #[doc = "2 Hz clock."]
    Clk2hz = 0x07,
}
impl ClkSel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel1 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel1 {
        ClkSel1::from_bits(val)
    }
}
impl From<ClkSel1> for u8 {
    #[inline(always)]
    fn from(val: ClkSel1) -> u8 {
        ClkSel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel2 {
    #[doc = "32/16 kHz clock."]
    Clk32khz = 0x0,
    #[doc = "512 Hz clock."]
    Clk512hz = 0x01,
    #[doc = "128 Hz clock."]
    Clk128hz = 0x02,
    #[doc = "64 Hz clock."]
    Clk64hz = 0x03,
    #[doc = "16 Hz clock."]
    Clk16hz = 0x04,
    #[doc = "8 Hz clock."]
    Clk8hz = 0x05,
    #[doc = "4 Hz clock."]
    Clk4hz = 0x06,
    #[doc = "2 Hz clock."]
    Clk2hz = 0x07,
}
impl ClkSel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel2 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel2 {
        ClkSel2::from_bits(val)
    }
}
impl From<ClkSel2> for u8 {
    #[inline(always)]
    fn from(val: ClkSel2) -> u8 {
        ClkSel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel3 {
    #[doc = "32/16 kHz clock."]
    Clk32khz = 0x0,
    #[doc = "512 Hz clock."]
    Clk512hz = 0x01,
    #[doc = "128 Hz clock."]
    Clk128hz = 0x02,
    #[doc = "64 Hz clock."]
    Clk64hz = 0x03,
    #[doc = "16 Hz clock."]
    Clk16hz = 0x04,
    #[doc = "8 Hz clock."]
    Clk8hz = 0x05,
    #[doc = "4 Hz clock."]
    Clk4hz = 0x06,
    #[doc = "2 Hz clock."]
    Clk2hz = 0x07,
}
impl ClkSel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel3 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel3 {
        ClkSel3::from_bits(val)
    }
}
impl From<ClkSel3> for u8 {
    #[inline(always)]
    fn from(val: ClkSel3) -> u8 {
        ClkSel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel4 {
    #[doc = "Clock to tamper filter 4 is 32.768/16.384 kHz (Oscillator clock) Tamper filter duration is 45.5us (i.e. 1.5 clock) to 1.95ms (64 clocks) in increments of 30.5us."]
    Clk32768khz = 0x0,
    #[doc = "Clock to tamper filter 4 is 512 Hz Tamper filter duration is 2.85ms (i.e. 1.5 clock) to 125ms (64 clocks) in increments of 1.95ms."]
    Clk512hz = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkSel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel4 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel4 {
        ClkSel4::from_bits(val)
    }
}
impl From<ClkSel4> for u8 {
    #[inline(always)]
    fn from(val: ClkSel4) -> u8 {
        ClkSel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel5 {
    #[doc = "Clock to tamper filter 5 is 32.768/16.384 kHz (Oscillator clock) Tamper filter duration is 45.5us (i.e. 1.5 clock) to 1.95ms (64 clocks) in increments of 30.5us."]
    Clk32768khz = 0x0,
    #[doc = "Clock to tamper filter 5 is 512 Hz Tamper filter duration is 2.85ms (i.e. 1.5 clock) to 125ms (64 clocks) in increments of 1.95ms."]
    Clk512hz = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkSel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel5 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel5 {
        ClkSel5::from_bits(val)
    }
}
impl From<ClkSel5> for u8 {
    #[inline(always)]
    fn from(val: ClkSel5) -> u8 {
        ClkSel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel6 {
    #[doc = "Clock to tamper filter 6 is 32.768/16.384 kHz (Oscillator clock) Tamper filter duration is 45.5us (i.e. 1.5 clock) to 1.95ms (64 clocks) in increments of 30.5us."]
    Clk32768khz = 0x0,
    #[doc = "Clock to tamper filter 6 is 512 Hz Tamper filter duration is 2.85ms (i.e. 1.5 clock) to 125ms (64 clocks) in increments of 1.95ms."]
    Clk512hz = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkSel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel6 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel6 {
        ClkSel6::from_bits(val)
    }
}
impl From<ClkSel6> for u8 {
    #[inline(always)]
    fn from(val: ClkSel6) -> u8 {
        ClkSel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel7 {
    #[doc = "Clock to tamper filter 7 is 32.768/16.384 kHz (Oscillator clock) Tamper filter duration is 45.5us (i.e. 1.5 clock) to 1.95ms (64 clocks) in increments of 30.5us."]
    Clk32768khz = 0x0,
    #[doc = "Clock to tamper filter 7 is 512 Hz Tamper filter duration is 2.85ms (i.e. 1.5 clock) to 125ms (64 clocks) in increments of 1.95ms."]
    Clk512hz = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkSel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel7 {
    #[inline(always)]
    fn from(val: u8) -> ClkSel7 {
        ClkSel7::from_bits(val)
    }
}
impl From<ClkSel7> for u8 {
    #[inline(always)]
    fn from(val: ClkSel7) -> u8 {
        ClkSel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DwnCntrInvalBit {
    #[doc = "Count down timer can be changed or read."]
    Valid = 0x0,
    #[doc = "Count down timer (if running) is changing value and cannot be read or written."]
    Invalid = 0x01,
}
impl DwnCntrInvalBit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DwnCntrInvalBit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DwnCntrInvalBit {
    #[inline(always)]
    fn from(val: u8) -> DwnCntrInvalBit {
        DwnCntrInvalBit::from_bits(val)
    }
}
impl From<DwnCntrInvalBit> for u8 {
    #[inline(always)]
    fn from(val: DwnCntrInvalBit) -> u8 {
        DwnCntrInvalBit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur0 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur0 {
    #[inline(always)]
    fn from(val: u8) -> FilDur0 {
        FilDur0::from_bits(val)
    }
}
impl From<FilDur0> for u8 {
    #[inline(always)]
    fn from(val: FilDur0) -> u8 {
        FilDur0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur1 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    Enabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur1 {
    #[inline(always)]
    fn from(val: u8) -> FilDur1 {
        FilDur1::from_bits(val)
    }
}
impl From<FilDur1> for u8 {
    #[inline(always)]
    fn from(val: FilDur1) -> u8 {
        FilDur1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur2 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur2 {
    #[inline(always)]
    fn from(val: u8) -> FilDur2 {
        FilDur2::from_bits(val)
    }
}
impl From<FilDur2> for u8 {
    #[inline(always)]
    fn from(val: FilDur2) -> u8 {
        FilDur2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur3 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur3 {
    #[inline(always)]
    fn from(val: u8) -> FilDur3 {
        FilDur3::from_bits(val)
    }
}
impl From<FilDur3> for u8 {
    #[inline(always)]
    fn from(val: FilDur3) -> u8 {
        FilDur3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur4 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur4 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur4 {
    #[inline(always)]
    fn from(val: u8) -> FilDur4 {
        FilDur4::from_bits(val)
    }
}
impl From<FilDur4> for u8 {
    #[inline(always)]
    fn from(val: FilDur4) -> u8 {
        FilDur4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur5 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur5 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur5 {
    #[inline(always)]
    fn from(val: u8) -> FilDur5 {
        FilDur5::from_bits(val)
    }
}
impl From<FilDur5> for u8 {
    #[inline(always)]
    fn from(val: FilDur5) -> u8 {
        FilDur5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur6 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur6 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur6 {
    #[inline(always)]
    fn from(val: u8) -> FilDur6 {
        FilDur6::from_bits(val)
    }
}
impl From<FilDur6> for u8 {
    #[inline(always)]
    fn from(val: FilDur6) -> u8 {
        FilDur6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilDur7 {
    #[doc = "Filtering operation disabled."]
    Disabled = 0x0,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled1 = 0x01,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled2 = 0x02,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled3 = 0x03,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled4 = 0x04,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled5 = 0x05,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled6 = 0x06,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled7 = 0x07,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled8 = 0x08,
    #[doc = "Number of tamper filter clock cycles to be counted when tamper is asserted."]
    NumberOfClkCyclesEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FilDur7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilDur7 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilDur7 {
    #[inline(always)]
    fn from(val: u8) -> FilDur7 {
        FilDur7::from_bits(val)
    }
}
impl From<FilDur7> for u8 {
    #[inline(always)]
    fn from(val: FilDur7) -> u8 {
        FilDur7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LfsrClkSel {
    #[doc = "32/16 KHz."]
    Clk3216khz = 0x0,
    #[doc = "512 Hz."]
    Clk512hz = 0x01,
    #[doc = "128 Hz."]
    Clk128hz = 0x02,
    #[doc = "64 Hz."]
    Clk64hz = 0x03,
    #[doc = "16 Hz."]
    Clk16hz = 0x04,
    #[doc = "8 Hz."]
    Clk8hz = 0x05,
    #[doc = "4 Hz."]
    Clk4hz = 0x06,
    #[doc = "2 Hz."]
    Clk2hz = 0x07,
}
impl LfsrClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfsrClkSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfsrClkSel {
    #[inline(always)]
    fn from(val: u8) -> LfsrClkSel {
        LfsrClkSel::from_bits(val)
    }
}
impl From<LfsrClkSel> for u8 {
    #[inline(always)]
    fn from(val: LfsrClkSel) -> u8 {
        LfsrClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol0 {
    #[doc = "Tamper detect input bit 0 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 0 is active low."]
    ActiveLow = 0x01,
}
impl Pol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol0 {
    #[inline(always)]
    fn from(val: u8) -> Pol0 {
        Pol0::from_bits(val)
    }
}
impl From<Pol0> for u8 {
    #[inline(always)]
    fn from(val: Pol0) -> u8 {
        Pol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol1 {
    #[doc = "Tamper detect input bit 1 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 1 is active low."]
    ActiveLow = 0x01,
}
impl Pol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol1 {
    #[inline(always)]
    fn from(val: u8) -> Pol1 {
        Pol1::from_bits(val)
    }
}
impl From<Pol1> for u8 {
    #[inline(always)]
    fn from(val: Pol1) -> u8 {
        Pol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol2 {
    #[doc = "Tamper detect input bit 2 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 2 is active low."]
    ActiveLow = 0x01,
}
impl Pol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol2 {
    #[inline(always)]
    fn from(val: u8) -> Pol2 {
        Pol2::from_bits(val)
    }
}
impl From<Pol2> for u8 {
    #[inline(always)]
    fn from(val: Pol2) -> u8 {
        Pol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol3 {
    #[doc = "Tamper detect input bit 3 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 3 is active low."]
    ActiveLow = 0x01,
}
impl Pol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol3 {
    #[inline(always)]
    fn from(val: u8) -> Pol3 {
        Pol3::from_bits(val)
    }
}
impl From<Pol3> for u8 {
    #[inline(always)]
    fn from(val: Pol3) -> u8 {
        Pol3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol4 {
    #[doc = "Tamper detect input bit 4is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 4 is active low."]
    ActiveLow = 0x01,
}
impl Pol4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol4 {
    #[inline(always)]
    fn from(val: u8) -> Pol4 {
        Pol4::from_bits(val)
    }
}
impl From<Pol4> for u8 {
    #[inline(always)]
    fn from(val: Pol4) -> u8 {
        Pol4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol5 {
    #[doc = "Tamper detect input bit 5 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 5 is active low."]
    ActiveLow = 0x01,
}
impl Pol5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol5 {
    #[inline(always)]
    fn from(val: u8) -> Pol5 {
        Pol5::from_bits(val)
    }
}
impl From<Pol5> for u8 {
    #[inline(always)]
    fn from(val: Pol5) -> u8 {
        Pol5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol6 {
    #[doc = "Tamper detect input bit 6 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 6 is active low."]
    ActiveLow = 0x01,
}
impl Pol6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol6 {
    #[inline(always)]
    fn from(val: u8) -> Pol6 {
        Pol6::from_bits(val)
    }
}
impl From<Pol6> for u8 {
    #[inline(always)]
    fn from(val: Pol6) -> u8 {
        Pol6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pol7 {
    #[doc = "Tamper detect input bit 7 is active high."]
    ActiveHigh = 0x0,
    #[doc = "Tamper detect input bit 7 is active low."]
    ActiveLow = 0x01,
}
impl Pol7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pol7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pol7 {
    #[inline(always)]
    fn from(val: u8) -> Pol7 {
        Pol7::from_bits(val)
    }
}
impl From<Pol7> for u8 {
    #[inline(always)]
    fn from(val: Pol7) -> u8 {
        Pol7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "Software Reset cleared."]
    Cleared = 0x0,
    #[doc = "Software Reset asserted."]
    Asserted = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmprEn {
    #[doc = "Tamper Status reporting disabled."]
    Disabled = 0x0,
    #[doc = "Tamper Status reporting enabled."]
    Enabled = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl TmprEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmprEn {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmprEn {
    #[inline(always)]
    fn from(val: u8) -> TmprEn {
        TmprEn::from_bits(val)
    }
}
impl From<TmprEn> for u8 {
    #[inline(always)]
    fn from(val: TmprEn) -> u8 {
        TmprEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmprSts {
    #[doc = "No Tamper Detected."]
    NoTamper = 0x0,
    #[doc = "Tamper Event Detected."]
    TamperDetected = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl TmprSts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmprSts {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmprSts {
    #[inline(always)]
    fn from(val: u8) -> TmprSts {
        TmprSts::from_bits(val)
    }
}
impl From<TmprSts> for u8 {
    #[inline(always)]
    fn from(val: TmprSts) -> u8 {
        TmprSts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeupMode {
    #[doc = "Tamper pin 0 is used as the tamper pin."]
    TamperPin = 0x0,
    #[doc = "Tamper pin 0 is used as a wakeup and hibernation pin."]
    WakeupPin = 0x01,
}
impl WakeupMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeupMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeupMode {
    #[inline(always)]
    fn from(val: u8) -> WakeupMode {
        WakeupMode::from_bits(val)
    }
}
impl From<WakeupMode> for u8 {
    #[inline(always)]
    fn from(val: WakeupMode) -> u8 {
        WakeupMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeupStatus {
    #[doc = "The wakeup and hibernation pin is in HiZ mode."]
    HiZMode = 0x0,
    #[doc = "The wakeup and hibernation pin is at logic 0. MCU is in sleep mode."]
    Logic0 = 0x01,
    #[doc = "The wakeup and hibernation pin is at logic 1. MCU is in sleep mode."]
    Logic1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl WakeupStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeupStatus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeupStatus {
    #[inline(always)]
    fn from(val: u8) -> WakeupStatus {
        WakeupStatus::from_bits(val)
    }
}
impl From<WakeupStatus> for u8 {
    #[inline(always)]
    fn from(val: WakeupStatus) -> u8 {
        WakeupStatus::to_bits(val)
    }
}
