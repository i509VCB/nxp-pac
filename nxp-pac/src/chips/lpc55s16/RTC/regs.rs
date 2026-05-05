#[doc = "RTC counter register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COUNT(pub u32);
impl COUNT {
    #[doc = "A read reflects the current value of the main, 1 Hz RTC timer. A write loads a new initial value into the timer. The RTC counter will count up continuously at a 1 Hz rate once the RTC Software Reset is removed (by clearing bit 0 of the CTRL register). Only write to this register when the RTC_EN bit in the RTC CTRL Register is 0. The counter increments one second after the RTC_EN bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A read reflects the current value of the main, 1 Hz RTC timer. A write loads a new initial value into the timer. The RTC counter will count up continuously at a 1 Hz rate once the RTC Software Reset is removed (by clearing bit 0 of the CTRL register). Only write to this register when the RTC_EN bit in the RTC CTRL Register is 0. The counter increments one second after the RTC_EN bit is set."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for COUNT {
    #[inline(always)]
    fn default() -> COUNT {
        COUNT(0)
    }
}
impl core::fmt::Debug for COUNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT").field("VAL", &self.VAL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COUNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "COUNT {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "RTC control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Software reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn SWRESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset control."]
    #[inline(always)]
    pub const fn set_SWRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RTC 1 Hz timer alarm flag status."]
    #[must_use]
    #[inline(always)]
    pub const fn ALARM1HZ(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub const fn set_ALARM1HZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RTC 1 kHz timer wake-up flag status."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKE1KHZ(&self) -> super::vals::WAKE1KHZ {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::WAKE1KHZ::from_bits(val as u8)
    }
    #[doc = "RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub const fn set_WAKE1KHZ(&mut self, val: super::vals::WAKE1KHZ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RTC 1 Hz timer alarm enable for Deep power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn ALARMDPD_EN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub const fn set_ALARMDPD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEDPD_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub const fn set_WAKEDPD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC1KHZ_EN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub const fn set_RTC1KHZ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RTC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RTC enable."]
    #[inline(always)]
    pub const fn set_RTC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RTC oscillator power-down control."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_OSC_PD(&self) -> super::vals::RTC_OSC_PD {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RTC_OSC_PD::from_bits(val as u8)
    }
    #[doc = "RTC oscillator power-down control."]
    #[inline(always)]
    pub const fn set_RTC_OSC_PD(&mut self, val: super::vals::RTC_OSC_PD) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RTC oscillator bypass control."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_OSC_BYPASS(&self) -> super::vals::RTC_OSC_BYPASS {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RTC_OSC_BYPASS::from_bits(val as u8)
    }
    #[doc = "RTC oscillator bypass control."]
    #[inline(always)]
    pub const fn set_RTC_OSC_BYPASS(&mut self, val: super::vals::RTC_OSC_BYPASS) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "RTC Sub-second counter control."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_SUBSEC_ENA(&self) -> super::vals::RTC_SUBSEC_ENA {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::RTC_SUBSEC_ENA::from_bits(val as u8)
    }
    #[doc = "RTC Sub-second counter control."]
    #[inline(always)]
    pub const fn set_RTC_SUBSEC_ENA(&mut self, val: super::vals::RTC_SUBSEC_ENA) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("SWRESET", &self.SWRESET())
            .field("ALARM1HZ", &self.ALARM1HZ())
            .field("WAKE1KHZ", &self.WAKE1KHZ())
            .field("ALARMDPD_EN", &self.ALARMDPD_EN())
            .field("WAKEDPD_EN", &self.WAKEDPD_EN())
            .field("RTC1KHZ_EN", &self.RTC1KHZ_EN())
            .field("RTC_EN", &self.RTC_EN())
            .field("RTC_OSC_PD", &self.RTC_OSC_PD())
            .field("RTC_OSC_BYPASS", &self.RTC_OSC_BYPASS())
            .field("RTC_SUBSEC_ENA", &self.RTC_SUBSEC_ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ SWRESET: {=bool:?}, ALARM1HZ: {=bool:?}, WAKE1KHZ: {:?}, ALARMDPD_EN: {=bool:?}, WAKEDPD_EN: {=bool:?}, RTC1KHZ_EN: {=bool:?}, RTC_EN: {=bool:?}, RTC_OSC_PD: {:?}, RTC_OSC_BYPASS: {:?}, RTC_SUBSEC_ENA: {:?} }}",
            self.SWRESET(),
            self.ALARM1HZ(),
            self.WAKE1KHZ(),
            self.ALARMDPD_EN(),
            self.WAKEDPD_EN(),
            self.RTC1KHZ_EN(),
            self.RTC_EN(),
            self.RTC_OSC_PD(),
            self.RTC_OSC_BYPASS(),
            self.RTC_SUBSEC_ENA()
        )
    }
}
#[doc = "General Purpose register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPREG(pub u32);
impl GPREG {
    #[doc = "Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[must_use]
    #[inline(always)]
    pub const fn GPDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[inline(always)]
    pub const fn set_GPDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPREG {
    #[inline(always)]
    fn default() -> GPREG {
        GPREG(0)
    }
}
impl core::fmt::Debug for GPREG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPREG")
            .field("GPDATA", &self.GPDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPREG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPREG {{ GPDATA: {=u32:?} }}", self.GPDATA())
    }
}
#[doc = "RTC match register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MATCH(pub u32);
impl MATCH {
    #[doc = "Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn MATVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub const fn set_MATVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MATCH {
    #[inline(always)]
    fn default() -> MATCH {
        MATCH(0)
    }
}
impl core::fmt::Debug for MATCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MATCH")
            .field("MATVAL", &self.MATVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MATCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MATCH {{ MATVAL: {=u32:?} }}", self.MATVAL())
    }
}
#[doc = "Sub-second counter register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUBSEC(pub u32);
impl SUBSEC {
    #[doc = "A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[must_use]
    #[inline(always)]
    pub const fn SUBSEC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "A read reflects the current value of the 32KHz sub-second counter. This counter is cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep power-down mode or after the main RTC module is disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub const fn set_SUBSEC(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for SUBSEC {
    #[inline(always)]
    fn default() -> SUBSEC {
        SUBSEC(0)
    }
}
impl core::fmt::Debug for SUBSEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUBSEC")
            .field("SUBSEC", &self.SUBSEC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUBSEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUBSEC {{ SUBSEC: {=u16:?} }}", self.SUBSEC())
    }
}
#[doc = "High-resolution/wake-up timer control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKE(pub u32);
impl WAKE {
    #[doc = "A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for WAKE {
    #[inline(always)]
    fn default() -> WAKE {
        WAKE(0)
    }
}
impl core::fmt::Debug for WAKE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKE").field("VAL", &self.VAL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAKE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WAKE {{ VAL: {=u16:?} }}", self.VAL())
    }
}
