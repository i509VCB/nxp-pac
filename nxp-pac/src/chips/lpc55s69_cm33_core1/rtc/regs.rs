#[doc = "RTC counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "A read reflects the current value of the main, 1 Hz RTC timer. A write loads a new initial value into the timer. The RTC counter will count up continuously at a 1 Hz rate once the RTC Software Reset is removed (by clearing bit 0 of the CTRL register). Only write to this register when the RTC_EN bit in the RTC CTRL Register is 0. The counter increments one second after the RTC_EN bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A read reflects the current value of the main, 1 Hz RTC timer. A write loads a new initial value into the timer. The RTC counter will count up continuously at a 1 Hz rate once the RTC Software Reset is removed (by clearing bit 0 of the CTRL register). Only write to this register when the RTC_EN bit in the RTC CTRL Register is 0. The counter increments one second after the RTC_EN bit is set."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
impl core::fmt::Debug for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Count").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Count {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "RTC control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Software reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn swreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset control"]
    #[inline(always)]
    pub const fn set_swreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RTC 1 Hz timer alarm flag status."]
    #[must_use]
    #[inline(always)]
    pub const fn alarm1hz(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub const fn set_alarm1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RTC 1 kHz timer wake-up flag status."]
    #[must_use]
    #[inline(always)]
    pub const fn wake1khz(&self) -> super::vals::Wake1khz {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wake1khz::from_bits(val as u8)
    }
    #[doc = "RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub const fn set_wake1khz(&mut self, val: super::vals::Wake1khz) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RTC 1 Hz timer alarm enable for Deep power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn alarmdpd_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub const fn set_alarmdpd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn wakedpd_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub const fn set_wakedpd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc1khz_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub const fn set_rtc1khz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RTC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RTC enable."]
    #[inline(always)]
    pub const fn set_rtc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RTC oscillator power-down control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_osc_pd(&self) -> super::vals::RtcOscPd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RtcOscPd::from_bits(val as u8)
    }
    #[doc = "RTC oscillator power-down control."]
    #[inline(always)]
    pub const fn set_rtc_osc_pd(&mut self, val: super::vals::RtcOscPd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RTC oscillator bypass control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_osc_bypass(&self) -> super::vals::RtcOscBypass {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RtcOscBypass::from_bits(val as u8)
    }
    #[doc = "RTC oscillator bypass control."]
    #[inline(always)]
    pub const fn set_rtc_osc_bypass(&mut self, val: super::vals::RtcOscBypass) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "RTC Sub-second counter control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_subsec_ena(&self) -> super::vals::RtcSubsecEna {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::RtcSubsecEna::from_bits(val as u8)
    }
    #[doc = "RTC Sub-second counter control."]
    #[inline(always)]
    pub const fn set_rtc_subsec_ena(&mut self, val: super::vals::RtcSubsecEna) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
            .field("swreset", &self.swreset())
            .field("alarm1hz", &self.alarm1hz())
            .field("wake1khz", &self.wake1khz())
            .field("alarmdpd_en", &self.alarmdpd_en())
            .field("wakedpd_en", &self.wakedpd_en())
            .field("rtc1khz_en", &self.rtc1khz_en())
            .field("rtc_en", &self.rtc_en())
            .field("rtc_osc_pd", &self.rtc_osc_pd())
            .field("rtc_osc_bypass", &self.rtc_osc_bypass())
            .field("rtc_subsec_ena", &self.rtc_subsec_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ swreset: {=bool:?}, alarm1hz: {=bool:?}, wake1khz: {:?}, alarmdpd_en: {=bool:?}, wakedpd_en: {=bool:?}, rtc1khz_en: {=bool:?}, rtc_en: {=bool:?}, rtc_osc_pd: {:?}, rtc_osc_bypass: {:?}, rtc_subsec_ena: {:?} }}",
            self.swreset(),
            self.alarm1hz(),
            self.wake1khz(),
            self.alarmdpd_en(),
            self.wakedpd_en(),
            self.rtc1khz_en(),
            self.rtc_en(),
            self.rtc_osc_pd(),
            self.rtc_osc_bypass(),
            self.rtc_subsec_ena()
        )
    }
}
#[doc = "General Purpose register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg(pub u32);
impl Gpreg {
    #[doc = "Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[must_use]
    #[inline(always)]
    pub const fn gpdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[inline(always)]
    pub const fn set_gpdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpreg {
    #[inline(always)]
    fn default() -> Gpreg {
        Gpreg(0)
    }
}
impl core::fmt::Debug for Gpreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpreg")
            .field("gpdata", &self.gpdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpreg {{ gpdata: {=u32:?} }}", self.gpdata())
    }
}
#[doc = "RTC match register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn matval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub const fn set_matval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
impl core::fmt::Debug for Match {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match")
            .field("matval", &self.matval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Match {{ matval: {=u32:?} }}", self.matval())
    }
}
#[doc = "Sub-second counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Subsec(pub u32);
impl Subsec {
    #[doc = "A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[must_use]
    #[inline(always)]
    pub const fn subsec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub const fn set_subsec(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Subsec {
    #[inline(always)]
    fn default() -> Subsec {
        Subsec(0)
    }
}
impl core::fmt::Debug for Subsec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Subsec")
            .field("subsec", &self.subsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Subsec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Subsec {{ subsec: {=u16:?} }}", self.subsec())
    }
}
#[doc = "High-resolution/wake-up timer control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wake(pub u32);
impl Wake {
    #[doc = "A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Wake {
    #[inline(always)]
    fn default() -> Wake {
        Wake(0)
    }
}
impl core::fmt::Debug for Wake {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wake").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wake {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wake {{ val: {=u16:?} }}", self.val())
    }
}
