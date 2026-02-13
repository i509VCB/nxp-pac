#[doc = "Days Alarm"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmDays(pub u16);
impl AlmDays {
    #[doc = "Days Value for Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Days Value for Alarm"]
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
#[doc = "Hours and Minutes Alarm"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmHourmin(pub u16);
impl AlmHourmin {
    #[doc = "Minutes Value for Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_min(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes Value for Alarm"]
    #[inline(always)]
    pub const fn set_alm_min(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Hours Value for Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_hour(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours Value for Alarm"]
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
#[doc = "Seconds Alarm"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmSeconds(pub u16);
impl AlmSeconds {
    #[doc = "Seconds Alarm Value"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_sec(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds Alarm Value"]
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
#[doc = "Year and Months Alarm"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlmYearmon(pub u16);
impl AlmYearmon {
    #[doc = "Months Value for Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_mon(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Months Value for Alarm"]
    #[inline(always)]
    pub const fn set_alm_mon(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Year Value for Alarm"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_year(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Year Value for Alarm"]
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
#[doc = "Compensation"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compen(pub u16);
impl Compen {
    #[doc = "Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn compen_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compensation Value"]
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
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u16);
impl Ctrl {
    #[doc = "Fine Compensation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fineen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fine Compensation Enable"]
    #[inline(always)]
    pub const fn set_fineen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Compensation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn comp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation Enable"]
    #[inline(always)]
    pub const fn set_comp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Alarm Match"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_match(&self) -> super::vals::AlmMatch {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::AlmMatch::from_bits(val as u8)
    }
    #[doc = "Alarm Match"]
    #[inline(always)]
    pub const fn set_alm_match(&mut self, val: super::vals::AlmMatch) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Daylight Saving Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Daylight Saving Enable"]
    #[inline(always)]
    pub const fn set_dst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> super::vals::Swr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Swr::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: super::vals::Swr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "RTC Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::ClkSel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::ClkSel::from_bits(val as u8)
    }
    #[doc = "RTC Clock Select"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::ClkSel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Clock Output Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn clko_dis(&self) -> super::vals::ClkoDis {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ClkoDis::from_bits(val as u8)
    }
    #[doc = "Clock Output Disable"]
    #[inline(always)]
    pub const fn set_clko_dis(&mut self, val: super::vals::ClkoDis) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "RTC Clock Output Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn clkout(&self) -> super::vals::Clkout {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Clkout::from_bits(val as u8)
    }
    #[doc = "RTC Clock Output Selection"]
    #[inline(always)]
    pub const fn set_clkout(&mut self, val: super::vals::Clkout) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u16) & 0x03) << 13usize);
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
            "Ctrl {{ fineen: {=bool:?}, comp_en: {=bool:?}, alm_match: {:?}, dst_en: {=bool:?}, swr: {:?}, clk_sel: {:?}, clko_dis: {:?}, clkout: {:?} }}",
            self.fineen(),
            self.comp_en(),
            self.alm_match(),
            self.dst_en(),
            self.swr(),
            self.clk_sel(),
            self.clko_dis(),
            self.clkout()
        )
    }
}
#[doc = "Days and Day-of-Week Counters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Days(pub u16);
impl Days {
    #[doc = "Days Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn day_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Days Counter Value"]
    #[inline(always)]
    pub const fn set_day_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Day of Week Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dow(&self) -> super::vals::Dow {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Dow::from_bits(val as u8)
    }
    #[doc = "Day of Week Counter Value"]
    #[inline(always)]
    pub const fn set_dow(&mut self, val: super::vals::Dow) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
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
            "Days {{ day_cnt: {=u8:?}, dow: {:?} }}",
            self.day_cnt(),
            self.dow()
        )
    }
}
#[doc = "Daylight Saving Day"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstDay(pub u16);
impl DstDay {
    #[doc = "Daylight Saving Time (DST) Day End Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_end_day(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Day End Value"]
    #[inline(always)]
    pub const fn set_dst_end_day(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Daylight Saving Time (DST) Day Start Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_start_day(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Day Start Value"]
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
#[doc = "Daylight Saving Hour"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstHour(pub u16);
impl DstHour {
    #[doc = "Daylight Saving Time (DST) Hours End Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_end_hour(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Hours End Value"]
    #[inline(always)]
    pub const fn set_dst_end_hour(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
    }
    #[doc = "Daylight Saving Time (DST) Hours Start Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_start_hour(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Hours Start Value"]
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
#[doc = "Daylight Saving Month"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstMonth(pub u16);
impl DstMonth {
    #[doc = "Daylight Saving Time (DST) Month End Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_end_month(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Month End Value"]
    #[inline(always)]
    pub const fn set_dst_end_month(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Daylight Saving Time (DST) Month Start Value"]
    #[must_use]
    #[inline(always)]
    pub const fn dst_start_month(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Daylight Saving Time (DST) Month Start Value"]
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
#[doc = "Hours and Minutes Counters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hourmin(pub u16);
impl Hourmin {
    #[doc = "Minutes Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn min_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Minutes Counter Value"]
    #[inline(always)]
    pub const fn set_min_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "Hours Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn hour_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Hours Counter Value"]
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
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u16);
impl Ier {
    #[doc = "Alarm Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Interrupt Enable"]
    #[inline(always)]
    pub const fn set_alm_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Days Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn day_ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Days Interrupt Enable"]
    #[inline(always)]
    pub const fn set_day_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Hours Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hour_ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hours Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hour_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Minutes Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn min_ie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Minutes Interrupt Enable"]
    #[inline(always)]
    pub const fn set_min_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "1 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_1hz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "2 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_2hz(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "2 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_2hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "4 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_4hz(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "4 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_4hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "8 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_8hz(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "8 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_8hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "16 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_16hz(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "16 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_16hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "32 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_32hz(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "32 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_32hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "64 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_64hz(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "64 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_64hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "128 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_128hz(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "128 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_128hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "256 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_256hz(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "256 Hz Interval Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie_256hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "512 Hz Interval Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie_512hz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "512 Hz Interval Interrupt Enable"]
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
            "Ier {{ alm_ie: {=bool:?}, day_ie: {=bool:?}, hour_ie: {=bool:?}, min_ie: {=bool:?}, ie_1hz: {=bool:?}, ie_2hz: {=bool:?}, ie_4hz: {=bool:?}, ie_8hz: {=bool:?}, ie_16hz: {=bool:?}, ie_32hz: {=bool:?}, ie_64hz: {=bool:?}, ie_128hz: {=bool:?}, ie_256hz: {=bool:?}, ie_512hz: {=bool:?} }}",
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
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u16);
impl Isr {
    #[doc = "Alarm Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn alm_is(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm Interrupt Status"]
    #[inline(always)]
    pub const fn set_alm_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Days Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn day_is(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Days Interrupt Status"]
    #[inline(always)]
    pub const fn set_day_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Hours Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn hour_is(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hours Interrupt Status"]
    #[inline(always)]
    pub const fn set_hour_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Minutes Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn min_is(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Minutes Interrupt Status"]
    #[inline(always)]
    pub const fn set_min_is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "1 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_1hz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "2 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_2hz(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "2 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_2hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "4 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_4hz(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "4 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_4hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "8 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_8hz(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "8 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_8hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "16 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_16hz(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "16 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_16hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "32 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_32hz(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "32 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_32hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "64 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_64hz(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "64 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_64hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "128 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_128hz(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "128 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_128hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "256 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_256hz(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "256 Hz Interval Interrupt Status"]
    #[inline(always)]
    pub const fn set_is_256hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "512 Hz Interval Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn is_512hz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "512 Hz Interval Interrupt Status"]
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
            "Isr {{ alm_is: {=bool:?}, day_is: {=bool:?}, hour_is: {=bool:?}, min_is: {=bool:?}, is_1hz: {=bool:?}, is_2hz: {=bool:?}, is_4hz: {=bool:?}, is_8hz: {=bool:?}, is_16hz: {=bool:?}, is_32hz: {=bool:?}, is_64hz: {=bool:?}, is_128hz: {=bool:?}, is_256hz: {=bool:?}, is_512hz: {=bool:?} }}",
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
#[doc = "Sub Second Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTest2(pub u16);
impl RtcTest2 {
    #[doc = "Sub Second Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn sub_second_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sub Second Counter Value"]
    #[inline(always)]
    pub const fn set_sub_second_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcTest2 {
    #[inline(always)]
    fn default() -> RtcTest2 {
        RtcTest2(0)
    }
}
impl core::fmt::Debug for RtcTest2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcTest2")
            .field("sub_second_count", &self.sub_second_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcTest2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RtcTest2 {{ sub_second_count: {=u16:?} }}",
            self.sub_second_count()
        )
    }
}
#[doc = "Seconds Counters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seconds(pub u16);
impl Seconds {
    #[doc = "Seconds Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Seconds Counter Value"]
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
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u16);
impl Status {
    #[doc = "Invalidate CPU Read/Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn inval_bit(&self) -> super::vals::InvalBit {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::InvalBit::from_bits(val as u8)
    }
    #[doc = "Invalidate CPU Read/Write Access"]
    #[inline(always)]
    pub const fn set_inval_bit(&mut self, val: super::vals::InvalBit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Write Protect Enable Status"]
    #[must_use]
    #[inline(always)]
    pub const fn write_prot_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write Protect Enable Status"]
    #[inline(always)]
    pub const fn set_write_prot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Compensation Interval"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation Interval"]
    #[inline(always)]
    pub const fn set_cmp_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> super::vals::We {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::We::from_bits(val as u8)
    }
    #[doc = "Write Enable"]
    #[inline(always)]
    pub const fn set_we(&mut self, val: super::vals::We) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn bus_err(&self) -> super::vals::BusErr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::BusErr::from_bits(val as u8)
    }
    #[doc = "Bus Error"]
    #[inline(always)]
    pub const fn set_bus_err(&mut self, val: super::vals::BusErr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Compensation Done"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_done(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation Done"]
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
            "Status {{ inval_bit: {:?}, write_prot_en: {=bool:?}, cmp_int: {=bool:?}, we: {:?}, bus_err: {:?}, cmp_done: {=bool:?} }}",
            self.inval_bit(),
            self.write_prot_en(),
            self.cmp_int(),
            self.we(),
            self.bus_err(),
            self.cmp_done()
        )
    }
}
#[doc = "Year and Month Counters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Yearmon(pub u16);
impl Yearmon {
    #[doc = "Month Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_cnt(&self) -> super::vals::MonCnt {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::MonCnt::from_bits(val as u8)
    }
    #[doc = "Month Counter"]
    #[inline(always)]
    pub const fn set_mon_cnt(&mut self, val: super::vals::MonCnt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Year Offset Count Value"]
    #[must_use]
    #[inline(always)]
    pub const fn yrofst(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Year Offset Count Value"]
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
            "Yearmon {{ mon_cnt: {:?}, yrofst: {=u8:?} }}",
            self.mon_cnt(),
            self.yrofst()
        )
    }
}
