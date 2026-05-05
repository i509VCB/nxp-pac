#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEED(pub u32);
impl FEED {
    #[doc = "Feed value should be 0xAA followed by 0x55."]
    #[must_use]
    #[inline(always)]
    pub const fn FEED(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Feed value should be 0xAA followed by 0x55."]
    #[inline(always)]
    pub const fn set_FEED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FEED {
    #[inline(always)]
    fn default() -> FEED {
        FEED(0)
    }
}
impl core::fmt::Debug for FEED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEED").field("FEED", &self.FEED()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEED {{ FEED: {=u8:?} }}", self.FEED())
    }
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MOD(pub u32);
impl MOD {
    #[doc = "Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[must_use]
    #[inline(always)]
    pub const fn WDEN(&self) -> super::vals::WDEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::WDEN::from_bits(val as u8)
    }
    #[doc = "Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub const fn set_WDEN(&mut self, val: super::vals::WDEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[must_use]
    #[inline(always)]
    pub const fn WDRESET(&self) -> super::vals::WDRESET {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::WDRESET::from_bits(val as u8)
    }
    #[doc = "Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub const fn set_WDRESET(&mut self, val: super::vals::WDRESET) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTOF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub const fn set_WDTOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn WDINT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub const fn set_WDINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn WDPROTECT(&self) -> super::vals::WDPROTECT {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::WDPROTECT::from_bits(val as u8)
    }
    #[doc = "Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub const fn set_WDPROTECT(&mut self, val: super::vals::WDPROTECT) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for MOD {
    #[inline(always)]
    fn default() -> MOD {
        MOD(0)
    }
}
impl core::fmt::Debug for MOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOD")
            .field("WDEN", &self.WDEN())
            .field("WDRESET", &self.WDRESET())
            .field("WDTOF", &self.WDTOF())
            .field("WDINT", &self.WDINT())
            .field("WDPROTECT", &self.WDPROTECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MOD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MOD {{ WDEN: {:?}, WDRESET: {:?}, WDTOF: {=bool:?}, WDINT: {=bool:?}, WDPROTECT: {:?} }}",
            self.WDEN(),
            self.WDRESET(),
            self.WDTOF(),
            self.WDINT(),
            self.WDPROTECT()
        )
    }
}
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TC(pub u32);
impl TC {
    #[doc = "Watchdog time-out value."]
    #[must_use]
    #[inline(always)]
    pub const fn COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Watchdog time-out value."]
    #[inline(always)]
    pub const fn set_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for TC {
    #[inline(always)]
    fn default() -> TC {
        TC(0)
    }
}
impl core::fmt::Debug for TC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC").field("COUNT", &self.COUNT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TC {{ COUNT: {=u32:?} }}", self.COUNT())
    }
}
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TV(pub u32);
impl TV {
    #[doc = "Counter timer value."]
    #[must_use]
    #[inline(always)]
    pub const fn COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter timer value."]
    #[inline(always)]
    pub const fn set_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for TV {
    #[inline(always)]
    fn default() -> TV {
        TV(0)
    }
}
impl core::fmt::Debug for TV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TV").field("COUNT", &self.COUNT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TV {{ COUNT: {=u32:?} }}", self.COUNT())
    }
}
#[doc = "Watchdog Warning Interrupt compare value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WARNINT(pub u32);
impl WARNINT {
    #[doc = "Watchdog warning interrupt compare value."]
    #[must_use]
    #[inline(always)]
    pub const fn WARNINT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Watchdog warning interrupt compare value."]
    #[inline(always)]
    pub const fn set_WARNINT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for WARNINT {
    #[inline(always)]
    fn default() -> WARNINT {
        WARNINT(0)
    }
}
impl core::fmt::Debug for WARNINT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WARNINT")
            .field("WARNINT", &self.WARNINT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WARNINT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WARNINT {{ WARNINT: {=u16:?} }}", self.WARNINT())
    }
}
#[doc = "Watchdog Window compare value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WINDOW(pub u32);
impl WINDOW {
    #[doc = "Watchdog window value."]
    #[must_use]
    #[inline(always)]
    pub const fn WINDOW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Watchdog window value."]
    #[inline(always)]
    pub const fn set_WINDOW(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for WINDOW {
    #[inline(always)]
    fn default() -> WINDOW {
        WINDOW(0)
    }
}
impl core::fmt::Debug for WINDOW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WINDOW")
            .field("WINDOW", &self.WINDOW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WINDOW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WINDOW {{ WINDOW: {=u32:?} }}", self.WINDOW())
    }
}
