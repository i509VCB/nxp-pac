#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AOREG1(pub u32);
impl AOREG1 {
    #[doc = "The last chip reset was caused by a Power On Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn POR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub const fn set_POR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The last chip reset was caused by a Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn PADRESET(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub const fn set_PADRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[must_use]
    #[inline(always)]
    pub const fn BODRESET(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub const fn set_BODRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSTEMRESET(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub const fn set_SYSTEMRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "The last chip reset was caused by the Watchdog Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTRESET(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub const fn set_WDTRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "The last chip reset was caused by a Software event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWRRESET(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub const fn set_SWRRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDRESET_WAKEUPIO(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub const fn set_DPDRESET_WAKEUPIO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDRESET_RTC(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub const fn set_DPDRESET_RTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDRESET_OSTIMER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub const fn set_DPDRESET_OSTIMER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "The last chip reset was caused by the code Watchdog."]
    #[must_use]
    #[inline(always)]
    pub const fn CDOGRESET(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by the code Watchdog."]
    #[inline(always)]
    pub const fn set_CDOGRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ROM Boot Fatal Error Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOTERRORCOUNTER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub const fn set_BOOTERRORCOUNTER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for AOREG1 {
    #[inline(always)]
    fn default() -> AOREG1 {
        AOREG1(0)
    }
}
impl core::fmt::Debug for AOREG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AOREG1")
            .field("POR", &self.POR())
            .field("PADRESET", &self.PADRESET())
            .field("BODRESET", &self.BODRESET())
            .field("SYSTEMRESET", &self.SYSTEMRESET())
            .field("WDTRESET", &self.WDTRESET())
            .field("SWRRESET", &self.SWRRESET())
            .field("DPDRESET_WAKEUPIO", &self.DPDRESET_WAKEUPIO())
            .field("DPDRESET_RTC", &self.DPDRESET_RTC())
            .field("DPDRESET_OSTIMER", &self.DPDRESET_OSTIMER())
            .field("CDOGRESET", &self.CDOGRESET())
            .field("BOOTERRORCOUNTER", &self.BOOTERRORCOUNTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AOREG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AOREG1 {{ POR: {=bool:?}, PADRESET: {=bool:?}, BODRESET: {=bool:?}, SYSTEMRESET: {=bool:?}, WDTRESET: {=bool:?}, SWRRESET: {=bool:?}, DPDRESET_WAKEUPIO: {=bool:?}, DPDRESET_RTC: {=bool:?}, DPDRESET_OSTIMER: {=bool:?}, CDOGRESET: {=bool:?}, BOOTERRORCOUNTER: {=u8:?} }}",
            self.POR(),
            self.PADRESET(),
            self.BODRESET(),
            self.SYSTEMRESET(),
            self.WDTRESET(),
            self.SWRRESET(),
            self.DPDRESET_WAKEUPIO(),
            self.DPDRESET_RTC(),
            self.DPDRESET_OSTIMER(),
            self.CDOGRESET(),
            self.BOOTERRORCOUNTER()
        )
    }
}
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BODVBAT(pub u32);
impl BODVBAT {
    #[doc = "BoD trigger level."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGLVL(&self) -> super::vals::TRIGLVL {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TRIGLVL::from_bits(val as u8)
    }
    #[doc = "BoD trigger level."]
    #[inline(always)]
    pub const fn set_TRIGLVL(&mut self, val: super::vals::TRIGLVL) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "BoD Hysteresis control."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST(&self) -> super::vals::BODVBAT_HYST {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::BODVBAT_HYST::from_bits(val as u8)
    }
    #[doc = "BoD Hysteresis control."]
    #[inline(always)]
    pub const fn set_HYST(&mut self, val: super::vals::BODVBAT_HYST) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
}
impl Default for BODVBAT {
    #[inline(always)]
    fn default() -> BODVBAT {
        BODVBAT(0)
    }
}
impl core::fmt::Debug for BODVBAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BODVBAT")
            .field("TRIGLVL", &self.TRIGLVL())
            .field("HYST", &self.HYST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BODVBAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BODVBAT {{ TRIGLVL: {:?}, HYST: {:?} }}",
            self.TRIGLVL(),
            self.HYST()
        )
    }
}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP(pub u32);
impl COMP {
    #[doc = "Hysteris when hyst = '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hysteris when hyst = '1'."]
    #[inline(always)]
    pub const fn set_HYST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREFINPUT(&self) -> super::vals::VREFINPUT {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::VREFINPUT::from_bits(val as u8)
    }
    #[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub const fn set_VREFINPUT(&mut self, val: super::vals::VREFINPUT) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Low power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn LOWPOWER(&self) -> super::vals::LOWPOWER {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LOWPOWER::from_bits(val as u8)
    }
    #[doc = "Low power mode."]
    #[inline(always)]
    pub const fn set_LOWPOWER(&mut self, val: super::vals::LOWPOWER) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Control word for P multiplexer:."]
    #[must_use]
    #[inline(always)]
    pub const fn PMUX(&self) -> super::vals::PMUX {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::PMUX::from_bits(val as u8)
    }
    #[doc = "Control word for P multiplexer:."]
    #[inline(always)]
    pub const fn set_PMUX(&mut self, val: super::vals::PMUX) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Control word for N multiplexer:."]
    #[must_use]
    #[inline(always)]
    pub const fn NMUX(&self) -> super::vals::NMUX {
        let val = (self.0 >> 7usize) & 0x07;
        super::vals::NMUX::from_bits(val as u8)
    }
    #[doc = "Control word for N multiplexer:."]
    #[inline(always)]
    pub const fn set_NMUX(&mut self, val: super::vals::NMUX) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub const fn set_VREF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Control the filtering of the Analog Comparator output."]
    #[must_use]
    #[inline(always)]
    pub const fn FILTERCGF_SAMPLEMODE(&self) -> super::vals::FILTERCGF_SAMPLEMODE {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FILTERCGF_SAMPLEMODE::from_bits(val as u8)
    }
    #[doc = "Control the filtering of the Analog Comparator output."]
    #[inline(always)]
    pub const fn set_FILTERCGF_SAMPLEMODE(&mut self, val: super::vals::FILTERCGF_SAMPLEMODE) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Filter Clock divider."]
    #[must_use]
    #[inline(always)]
    pub const fn FILTERCGF_CLKDIV(&self) -> super::vals::FILTERCGF_CLKDIV {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::FILTERCGF_CLKDIV::from_bits(val as u8)
    }
    #[doc = "Filter Clock divider."]
    #[inline(always)]
    pub const fn set_FILTERCGF_CLKDIV(&mut self, val: super::vals::FILTERCGF_CLKDIV) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
}
impl Default for COMP {
    #[inline(always)]
    fn default() -> COMP {
        COMP(0)
    }
}
impl core::fmt::Debug for COMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP")
            .field("HYST", &self.HYST())
            .field("VREFINPUT", &self.VREFINPUT())
            .field("LOWPOWER", &self.LOWPOWER())
            .field("PMUX", &self.PMUX())
            .field("NMUX", &self.NMUX())
            .field("VREF", &self.VREF())
            .field("FILTERCGF_SAMPLEMODE", &self.FILTERCGF_SAMPLEMODE())
            .field("FILTERCGF_CLKDIV", &self.FILTERCGF_CLKDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP {{ HYST: {=bool:?}, VREFINPUT: {:?}, LOWPOWER: {:?}, PMUX: {:?}, NMUX: {:?}, VREF: {=u8:?}, FILTERCGF_SAMPLEMODE: {:?}, FILTERCGF_CLKDIV: {:?} }}",
            self.HYST(),
            self.VREFINPUT(),
            self.LOWPOWER(),
            self.PMUX(),
            self.NMUX(),
            self.VREF(),
            self.FILTERCGF_SAMPLEMODE(),
            self.FILTERCGF_CLKDIV()
        )
    }
}
#[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC0(pub u32);
impl DCDC0 {
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn RC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_RC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn ICOMP(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_ICOMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn ISEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_ISEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn ICENABLE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_ICENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn TMOS(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_TMOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLEISENSE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_DISABLEISENSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT(&self) -> super::vals::VOUT {
        let val = (self.0 >> 17usize) & 0x0f;
        super::vals::VOUT::from_bits(val as u8)
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_VOUT(&mut self, val: super::vals::VOUT) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn SLICINGENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_SLICINGENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn INDUCTORCLAMPENABLE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_INDUCTORCLAMPENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT_PWD(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_VOUT_PWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for DCDC0 {
    #[inline(always)]
    fn default() -> DCDC0 {
        DCDC0(0)
    }
}
impl core::fmt::Debug for DCDC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC0")
            .field("RC", &self.RC())
            .field("ICOMP", &self.ICOMP())
            .field("ISEL", &self.ISEL())
            .field("ICENABLE", &self.ICENABLE())
            .field("TMOS", &self.TMOS())
            .field("DISABLEISENSE", &self.DISABLEISENSE())
            .field("VOUT", &self.VOUT())
            .field("SLICINGENABLE", &self.SLICINGENABLE())
            .field("INDUCTORCLAMPENABLE", &self.INDUCTORCLAMPENABLE())
            .field("VOUT_PWD", &self.VOUT_PWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC0 {{ RC: {=u8:?}, ICOMP: {=u8:?}, ISEL: {=u8:?}, ICENABLE: {=bool:?}, TMOS: {=u8:?}, DISABLEISENSE: {=bool:?}, VOUT: {:?}, SLICINGENABLE: {=bool:?}, INDUCTORCLAMPENABLE: {=bool:?}, VOUT_PWD: {=u8:?} }}",
            self.RC(),
            self.ICOMP(),
            self.ISEL(),
            self.ICENABLE(),
            self.TMOS(),
            self.DISABLEISENSE(),
            self.VOUT(),
            self.SLICINGENABLE(),
            self.INDUCTORCLAMPENABLE(),
            self.VOUT_PWD()
        )
    }
}
#[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC1(pub u32);
impl DCDC1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn RTRIMOFFET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_RTRIMOFFET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn RSENSETRIM(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_RSENSETRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTENABLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_DTESTENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCURVE(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETCURVE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETDC(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETDC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTSEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_DTESTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn ISCALEENABLE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_ISCALEENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEBYPASS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_FORCEBYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMAUTOCOT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_TRIMAUTOCOT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEFULLCYCLE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_FORCEFULLCYCLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn LCENABLE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_LCENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFF(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_TOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFFENABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_TOFFENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DCDC1 {
    #[inline(always)]
    fn default() -> DCDC1 {
        DCDC1(0)
    }
}
impl core::fmt::Debug for DCDC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC1")
            .field("RTRIMOFFET", &self.RTRIMOFFET())
            .field("RSENSETRIM", &self.RSENSETRIM())
            .field("DTESTENABLE", &self.DTESTENABLE())
            .field("SETCURVE", &self.SETCURVE())
            .field("SETDC", &self.SETDC())
            .field("DTESTSEL", &self.DTESTSEL())
            .field("ISCALEENABLE", &self.ISCALEENABLE())
            .field("FORCEBYPASS", &self.FORCEBYPASS())
            .field("TRIMAUTOCOT", &self.TRIMAUTOCOT())
            .field("FORCEFULLCYCLE", &self.FORCEFULLCYCLE())
            .field("LCENABLE", &self.LCENABLE())
            .field("TOFF", &self.TOFF())
            .field("TOFFENABLE", &self.TOFFENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC1 {{ RTRIMOFFET: {=u8:?}, RSENSETRIM: {=u8:?}, DTESTENABLE: {=bool:?}, SETCURVE: {=u8:?}, SETDC: {=u8:?}, DTESTSEL: {=u8:?}, ISCALEENABLE: {=bool:?}, FORCEBYPASS: {=bool:?}, TRIMAUTOCOT: {=u8:?}, FORCEFULLCYCLE: {=bool:?}, LCENABLE: {=bool:?}, TOFF: {=u8:?}, TOFFENABLE: {=bool:?} }}",
            self.RTRIMOFFET(),
            self.RSENSETRIM(),
            self.DTESTENABLE(),
            self.SETCURVE(),
            self.SETDC(),
            self.DTESTSEL(),
            self.ISCALEENABLE(),
            self.FORCEBYPASS(),
            self.TRIMAUTOCOT(),
            self.FORCEFULLCYCLE(),
            self.LCENABLE(),
            self.TOFF(),
            self.TOFFENABLE()
        )
    }
}
#[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LDOPMU(pub u32);
impl LDOPMU {
    #[doc = "Sets the Always-On domain LDO output level."]
    #[must_use]
    #[inline(always)]
    pub const fn VADJ(&self) -> super::vals::VADJ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::VADJ::from_bits(val as u8)
    }
    #[doc = "Sets the Always-On domain LDO output level."]
    #[inline(always)]
    pub const fn set_VADJ(&mut self, val: super::vals::VADJ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Sets the Always-On domain LDO output level in all power down modes."]
    #[must_use]
    #[inline(always)]
    pub const fn VADJ_PWD(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the Always-On domain LDO output level in all power down modes."]
    #[inline(always)]
    pub const fn set_VADJ_PWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Sets the Always-On domain LDO Boost output level."]
    #[must_use]
    #[inline(always)]
    pub const fn VADJ_BOOST(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the Always-On domain LDO Boost output level."]
    #[inline(always)]
    pub const fn set_VADJ_BOOST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Sets the Always-On domain LDO Boost output level in all power down modes."]
    #[must_use]
    #[inline(always)]
    pub const fn VADJ_BOOST_PWD(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the Always-On domain LDO Boost output level in all power down modes."]
    #[inline(always)]
    pub const fn set_VADJ_BOOST_PWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "Controls LDOMEM bleed current."]
    #[must_use]
    #[inline(always)]
    pub const fn BLEED(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Controls LDOMEM bleed current."]
    #[inline(always)]
    pub const fn set_BLEED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Control the LDO AO boost mode in ACTIVE mode."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOST_ENA(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Control the LDO AO boost mode in ACTIVE mode."]
    #[inline(always)]
    pub const fn set_BOOST_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOST_ENA_PWD(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
    #[inline(always)]
    pub const fn set_BOOST_ENA_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for LDOPMU {
    #[inline(always)]
    fn default() -> LDOPMU {
        LDOPMU(0)
    }
}
impl core::fmt::Debug for LDOPMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDOPMU")
            .field("VADJ", &self.VADJ())
            .field("VADJ_PWD", &self.VADJ_PWD())
            .field("VADJ_BOOST", &self.VADJ_BOOST())
            .field("VADJ_BOOST_PWD", &self.VADJ_BOOST_PWD())
            .field("BLEED", &self.BLEED())
            .field("BOOST_ENA", &self.BOOST_ENA())
            .field("BOOST_ENA_PWD", &self.BOOST_ENA_PWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LDOPMU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LDOPMU {{ VADJ: {:?}, VADJ_PWD: {=u8:?}, VADJ_BOOST: {=u8:?}, VADJ_BOOST_PWD: {=u8:?}, BLEED: {=bool:?}, BOOST_ENA: {=bool:?}, BOOST_ENA_PWD: {=bool:?} }}",
            self.VADJ(),
            self.VADJ_PWD(),
            self.VADJ_BOOST(),
            self.VADJ_BOOST_PWD(),
            self.BLEED(),
            self.BOOST_ENA(),
            self.BOOST_ENA_PWD()
        )
    }
}
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISCCTRL(pub u32);
impl MISCCTRL {
    #[doc = "Select LDO Deep Sleep reference source."]
    #[must_use]
    #[inline(always)]
    pub const fn LDODEEPSLEEPREF(&self) -> super::vals::LDODEEPSLEEPREF {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LDODEEPSLEEPREF::from_bits(val as u8)
    }
    #[doc = "Select LDO Deep Sleep reference source."]
    #[inline(always)]
    pub const fn set_LDODEEPSLEEPREF(&mut self, val: super::vals::LDODEEPSLEEPREF) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Control the activation of LDO MEM High Z mode."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOMEMHIGHZMODE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub const fn set_LDOMEMHIGHZMODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn LOWPWR_FLASH_BUF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_LOWPWR_FLASH_BUF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn MISCCTRL_3_11(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_MISCCTRL_3_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 3usize)) | (((val as u32) & 0x01ff) << 3usize);
    }
    #[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_BLEED(&self) -> super::vals::DISABLE_BLEED {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DISABLE_BLEED::from_bits(val as u8)
    }
    #[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub const fn set_DISABLE_BLEED(&mut self, val: super::vals::DISABLE_BLEED) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn MISCCTRL_13_15(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_MISCCTRL_13_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
}
impl Default for MISCCTRL {
    #[inline(always)]
    fn default() -> MISCCTRL {
        MISCCTRL(0)
    }
}
impl core::fmt::Debug for MISCCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISCCTRL")
            .field("LDODEEPSLEEPREF", &self.LDODEEPSLEEPREF())
            .field("LDOMEMHIGHZMODE", &self.LDOMEMHIGHZMODE())
            .field("LOWPWR_FLASH_BUF", &self.LOWPWR_FLASH_BUF())
            .field("MISCCTRL_3_11", &self.MISCCTRL_3_11())
            .field("DISABLE_BLEED", &self.DISABLE_BLEED())
            .field("MISCCTRL_13_15", &self.MISCCTRL_13_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISCCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISCCTRL {{ LDODEEPSLEEPREF: {:?}, LDOMEMHIGHZMODE: {=bool:?}, LOWPWR_FLASH_BUF: {=bool:?}, MISCCTRL_3_11: {=u16:?}, DISABLE_BLEED: {:?}, MISCCTRL_13_15: {=u8:?} }}",
            self.LDODEEPSLEEPREF(),
            self.LDOMEMHIGHZMODE(),
            self.LOWPWR_FLASH_BUF(),
            self.MISCCTRL_3_11(),
            self.DISABLE_BLEED(),
            self.MISCCTRL_13_15()
        )
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSTIMER(pub u32);
impl OSTIMER {
    #[doc = "Active high reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SOFTRESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Active high reset."]
    #[inline(always)]
    pub const fn set_SOFTRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable OS event timer clock."]
    #[must_use]
    #[inline(always)]
    pub const fn CLOCKENABLE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OS event timer clock."]
    #[inline(always)]
    pub const fn set_CLOCKENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDWAKEUPENABLE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub const fn set_DPDWAKEUPENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[must_use]
    #[inline(always)]
    pub const fn OSC32KPD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub const fn set_OSC32KPD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "OS event timer clock select."]
    #[must_use]
    #[inline(always)]
    pub const fn OSTIMERCLKSEL(&self) -> super::vals::OSTIMERCLKSEL {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::OSTIMERCLKSEL::from_bits(val as u8)
    }
    #[doc = "OS event timer clock select."]
    #[inline(always)]
    pub const fn set_OSTIMERCLKSEL(&mut self, val: super::vals::OSTIMERCLKSEL) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for OSTIMER {
    #[inline(always)]
    fn default() -> OSTIMER {
        OSTIMER(0)
    }
}
impl core::fmt::Debug for OSTIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSTIMER")
            .field("SOFTRESET", &self.SOFTRESET())
            .field("CLOCKENABLE", &self.CLOCKENABLE())
            .field("DPDWAKEUPENABLE", &self.DPDWAKEUPENABLE())
            .field("OSC32KPD", &self.OSC32KPD())
            .field("OSTIMERCLKSEL", &self.OSTIMERCLKSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OSTIMER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OSTIMER {{ SOFTRESET: {=bool:?}, CLOCKENABLE: {=bool:?}, DPDWAKEUPENABLE: {=bool:?}, OSC32KPD: {=bool:?}, OSTIMERCLKSEL: {:?} }}",
            self.SOFTRESET(),
            self.CLOCKENABLE(),
            self.DPDWAKEUPENABLE(),
            self.OSC32KPD(),
            self.OSTIMERCLKSEL()
        )
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDRUNCFG0(pub u32);
impl PDRUNCFG0 {
    #[doc = "Controls power to VBAT Brown Out Detector (BOD)."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_BODVBAT(&self) -> super::vals::PDEN_BODVBAT {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PDEN_BODVBAT::from_bits(val as u8)
    }
    #[doc = "Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub const fn set_PDEN_BODVBAT(&mut self, val: super::vals::PDEN_BODVBAT) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_FRO32K(&self) -> super::vals::PDEN_FRO32K {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PDEN_FRO32K::from_bits(val as u8)
    }
    #[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub const fn set_PDEN_FRO32K(&mut self, val: super::vals::PDEN_FRO32K) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Controls power to crystal 32 KHz."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_XTAL32K(&self) -> super::vals::PDEN_XTAL32K {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PDEN_XTAL32K::from_bits(val as u8)
    }
    #[doc = "Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub const fn set_PDEN_XTAL32K(&mut self, val: super::vals::PDEN_XTAL32K) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Controls power to high speed crystal."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_XTAL32M(&self) -> super::vals::PDEN_XTAL32M {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PDEN_XTAL32M::from_bits(val as u8)
    }
    #[doc = "Controls power to high speed crystal."]
    #[inline(always)]
    pub const fn set_PDEN_XTAL32M(&mut self, val: super::vals::PDEN_XTAL32M) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls power to System PLL (also refered as PLL0)."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_PLL0(&self) -> super::vals::PDEN_PLL0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PDEN_PLL0::from_bits(val as u8)
    }
    #[doc = "Controls power to System PLL (also refered as PLL0)."]
    #[inline(always)]
    pub const fn set_PDEN_PLL0(&mut self, val: super::vals::PDEN_PLL0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Controls power to USB PLL (also refered as PLL1)."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_PLL1(&self) -> super::vals::PDEN_PLL1 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PDEN_PLL1::from_bits(val as u8)
    }
    #[doc = "Controls power to USB PLL (also refered as PLL1)."]
    #[inline(always)]
    pub const fn set_PDEN_PLL1(&mut self, val: super::vals::PDEN_PLL1) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Controls power to USB Full Speed phy."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_USBFSPHY(&self) -> super::vals::PDEN_USBFSPHY {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PDEN_USBFSPHY::from_bits(val as u8)
    }
    #[doc = "Controls power to USB Full Speed phy."]
    #[inline(always)]
    pub const fn set_PDEN_USBFSPHY(&mut self, val: super::vals::PDEN_USBFSPHY) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls power to USB High Speed Phy."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_USBHSPHY(&self) -> super::vals::PDEN_USBHSPHY {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PDEN_USBHSPHY::from_bits(val as u8)
    }
    #[doc = "Controls power to USB High Speed Phy."]
    #[inline(always)]
    pub const fn set_PDEN_USBHSPHY(&mut self, val: super::vals::PDEN_USBHSPHY) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Controls power to Analog Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_COMP(&self) -> super::vals::PDEN_COMP {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PDEN_COMP::from_bits(val as u8)
    }
    #[doc = "Controls power to Analog Comparator."]
    #[inline(always)]
    pub const fn set_PDEN_COMP(&mut self, val: super::vals::PDEN_COMP) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Controls power to USB high speed LDO."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_LDOUSBHS(&self) -> super::vals::PDEN_LDOUSBHS {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PDEN_LDOUSBHS::from_bits(val as u8)
    }
    #[doc = "Controls power to USB high speed LDO."]
    #[inline(always)]
    pub const fn set_PDEN_LDOUSBHS(&mut self, val: super::vals::PDEN_LDOUSBHS) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Controls power to auxiliary biasing (AUXBIAS)."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_AUXBIAS(&self) -> super::vals::PDEN_AUXBIAS {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PDEN_AUXBIAS::from_bits(val as u8)
    }
    #[doc = "Controls power to auxiliary biasing (AUXBIAS)."]
    #[inline(always)]
    pub const fn set_PDEN_AUXBIAS(&mut self, val: super::vals::PDEN_AUXBIAS) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Controls power to high speed crystal LDO."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_LDOXO32M(&self) -> super::vals::PDEN_LDOXO32M {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PDEN_LDOXO32M::from_bits(val as u8)
    }
    #[doc = "Controls power to high speed crystal LDO."]
    #[inline(always)]
    pub const fn set_PDEN_LDOXO32M(&mut self, val: super::vals::PDEN_LDOXO32M) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_RNG(&self) -> super::vals::PDEN_RNG {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PDEN_RNG::from_bits(val as u8)
    }
    #[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub const fn set_PDEN_RNG(&mut self, val: super::vals::PDEN_RNG) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[must_use]
    #[inline(always)]
    pub const fn PDEN_PLL0_SSCG(&self) -> super::vals::PDEN_PLL0_SSCG {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PDEN_PLL0_SSCG::from_bits(val as u8)
    }
    #[doc = "Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline(always)]
    pub const fn set_PDEN_PLL0_SSCG(&mut self, val: super::vals::PDEN_PLL0_SSCG) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for PDRUNCFG0 {
    #[inline(always)]
    fn default() -> PDRUNCFG0 {
        PDRUNCFG0(0)
    }
}
impl core::fmt::Debug for PDRUNCFG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDRUNCFG0")
            .field("PDEN_BODVBAT", &self.PDEN_BODVBAT())
            .field("PDEN_FRO32K", &self.PDEN_FRO32K())
            .field("PDEN_XTAL32K", &self.PDEN_XTAL32K())
            .field("PDEN_XTAL32M", &self.PDEN_XTAL32M())
            .field("PDEN_PLL0", &self.PDEN_PLL0())
            .field("PDEN_PLL1", &self.PDEN_PLL1())
            .field("PDEN_USBFSPHY", &self.PDEN_USBFSPHY())
            .field("PDEN_USBHSPHY", &self.PDEN_USBHSPHY())
            .field("PDEN_COMP", &self.PDEN_COMP())
            .field("PDEN_LDOUSBHS", &self.PDEN_LDOUSBHS())
            .field("PDEN_AUXBIAS", &self.PDEN_AUXBIAS())
            .field("PDEN_LDOXO32M", &self.PDEN_LDOXO32M())
            .field("PDEN_RNG", &self.PDEN_RNG())
            .field("PDEN_PLL0_SSCG", &self.PDEN_PLL0_SSCG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDRUNCFG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDRUNCFG0 {{ PDEN_BODVBAT: {:?}, PDEN_FRO32K: {:?}, PDEN_XTAL32K: {:?}, PDEN_XTAL32M: {:?}, PDEN_PLL0: {:?}, PDEN_PLL1: {:?}, PDEN_USBFSPHY: {:?}, PDEN_USBHSPHY: {:?}, PDEN_COMP: {:?}, PDEN_LDOUSBHS: {:?}, PDEN_AUXBIAS: {:?}, PDEN_LDOXO32M: {:?}, PDEN_RNG: {:?}, PDEN_PLL0_SSCG: {:?} }}",
            self.PDEN_BODVBAT(),
            self.PDEN_FRO32K(),
            self.PDEN_XTAL32K(),
            self.PDEN_XTAL32M(),
            self.PDEN_PLL0(),
            self.PDEN_PLL1(),
            self.PDEN_USBFSPHY(),
            self.PDEN_USBHSPHY(),
            self.PDEN_COMP(),
            self.PDEN_LDOUSBHS(),
            self.PDEN_AUXBIAS(),
            self.PDEN_LDOXO32M(),
            self.PDEN_RNG(),
            self.PDEN_PLL0_SSCG()
        )
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDRUNCFGCLR0(pub u32);
impl PDRUNCFGCLR0 {
    #[doc = "Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn PDRUNCFGCLR0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub const fn set_PDRUNCFGCLR0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PDRUNCFGCLR0 {
    #[inline(always)]
    fn default() -> PDRUNCFGCLR0 {
        PDRUNCFGCLR0(0)
    }
}
impl core::fmt::Debug for PDRUNCFGCLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDRUNCFGCLR0")
            .field("PDRUNCFGCLR0", &self.PDRUNCFGCLR0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDRUNCFGCLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDRUNCFGCLR0 {{ PDRUNCFGCLR0: {=u32:?} }}",
            self.PDRUNCFGCLR0()
        )
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDRUNCFGSET0(pub u32);
impl PDRUNCFGSET0 {
    #[doc = "Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn PDRUNCFGSET0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub const fn set_PDRUNCFGSET0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PDRUNCFGSET0 {
    #[inline(always)]
    fn default() -> PDRUNCFGSET0 {
        PDRUNCFGSET0(0)
    }
}
impl core::fmt::Debug for PDRUNCFGSET0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDRUNCFGSET0")
            .field("PDRUNCFGSET0", &self.PDRUNCFGSET0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDRUNCFGSET0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDRUNCFGSET0 {{ PDRUNCFGSET0: {=u32:?} }}",
            self.PDRUNCFGSET0()
        )
    }
}
#[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REFFASTWKUP(pub u32);
impl REFFASTWKUP {
    #[doc = "Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN):."]
    #[must_use]
    #[inline(always)]
    pub const fn LPWKUP(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN):."]
    #[inline(always)]
    pub const fn set_LPWKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog References fast wake-up in case of Hardware Pin reset:."]
    #[must_use]
    #[inline(always)]
    pub const fn HWWKUP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog References fast wake-up in case of Hardware Pin reset:."]
    #[inline(always)]
    pub const fn set_HWWKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for REFFASTWKUP {
    #[inline(always)]
    fn default() -> REFFASTWKUP {
        REFFASTWKUP(0)
    }
}
impl core::fmt::Debug for REFFASTWKUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REFFASTWKUP")
            .field("LPWKUP", &self.LPWKUP())
            .field("HWWKUP", &self.HWWKUP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REFFASTWKUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REFFASTWKUP {{ LPWKUP: {=bool:?}, HWWKUP: {=bool:?} }}",
            self.LPWKUP(),
            self.HWWKUP()
        )
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESETCTRL(pub u32);
impl RESETCTRL {
    #[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDWAKEUPRESETENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub const fn set_DPDWAKEUPRESETENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SWRRESETENABLE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset enable."]
    #[inline(always)]
    pub const fn set_SWRRESETENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "BOD VBAT reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBATRESETENA_SECURE(&self) -> super::vals::BODVBATRESETENA_SECURE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::BODVBATRESETENA_SECURE::from_bits(val as u8)
    }
    #[doc = "BOD VBAT reset enable."]
    #[inline(always)]
    pub const fn set_BODVBATRESETENA_SECURE(&mut self, val: super::vals::BODVBATRESETENA_SECURE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "BOD Core reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORERESETENA_SECURE(&self) -> super::vals::BODCORERESETENA_SECURE {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::BODCORERESETENA_SECURE::from_bits(val as u8)
    }
    #[doc = "BOD Core reset enable."]
    #[inline(always)]
    pub const fn set_BODCORERESETENA_SECURE(&mut self, val: super::vals::BODCORERESETENA_SECURE) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "BOD VBAT reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBATRESETENA_SECURE_DP(&self) -> super::vals::BODVBATRESETENA_SECURE_DP {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::BODVBATRESETENA_SECURE_DP::from_bits(val as u8)
    }
    #[doc = "BOD VBAT reset enable."]
    #[inline(always)]
    pub const fn set_BODVBATRESETENA_SECURE_DP(
        &mut self,
        val: super::vals::BODVBATRESETENA_SECURE_DP,
    ) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "BOD Core reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORERESETENA_SECURE_DP(&self) -> super::vals::BODCORERESETENA_SECURE_DP {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::BODCORERESETENA_SECURE_DP::from_bits(val as u8)
    }
    #[doc = "BOD Core reset enable."]
    #[inline(always)]
    pub const fn set_BODCORERESETENA_SECURE_DP(
        &mut self,
        val: super::vals::BODCORERESETENA_SECURE_DP,
    ) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for RESETCTRL {
    #[inline(always)]
    fn default() -> RESETCTRL {
        RESETCTRL(0)
    }
}
impl core::fmt::Debug for RESETCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESETCTRL")
            .field("DPDWAKEUPRESETENABLE", &self.DPDWAKEUPRESETENABLE())
            .field("SWRRESETENABLE", &self.SWRRESETENABLE())
            .field("BODVBATRESETENA_SECURE", &self.BODVBATRESETENA_SECURE())
            .field("BODCORERESETENA_SECURE", &self.BODCORERESETENA_SECURE())
            .field(
                "BODVBATRESETENA_SECURE_DP",
                &self.BODVBATRESETENA_SECURE_DP(),
            )
            .field(
                "BODCORERESETENA_SECURE_DP",
                &self.BODCORERESETENA_SECURE_DP(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESETCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RESETCTRL {{ DPDWAKEUPRESETENABLE: {=bool:?}, SWRRESETENABLE: {=bool:?}, BODVBATRESETENA_SECURE: {:?}, BODCORERESETENA_SECURE: {:?}, BODVBATRESETENA_SECURE_DP: {:?}, BODCORERESETENA_SECURE_DP: {:?} }}",
            self.DPDWAKEUPRESETENABLE(),
            self.SWRRESETENABLE(),
            self.BODVBATRESETENA_SECURE(),
            self.BODCORERESETENA_SECURE(),
            self.BODVBATRESETENA_SECURE_DP(),
            self.BODCORERESETENA_SECURE_DP()
        )
    }
}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTCOSC32K(pub u32);
impl RTCOSC32K {
    #[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz)."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::SEL {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SEL::from_bits(val as u8)
    }
    #[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz)."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::SEL) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Actual division ratio is : 28 + CLK1KHZDIV."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK1KHZDIV(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub const fn set_CLK1KHZDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "RTC 1KHz clock Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK1KHZDIVUPDATEREQ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub const fn set_CLK1KHZDIVUPDATEREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Actual division ratio is : 31744 + CLK1HZDIV."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK1HZDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub const fn set_CLK1HZDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK1HZDIVHALT(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_CLK1HZDIVHALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "RTC 1Hz Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK1HZDIVUPDATEREQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub const fn set_CLK1HZDIVUPDATEREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RTCOSC32K {
    #[inline(always)]
    fn default() -> RTCOSC32K {
        RTCOSC32K(0)
    }
}
impl core::fmt::Debug for RTCOSC32K {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCOSC32K")
            .field("SEL", &self.SEL())
            .field("CLK1KHZDIV", &self.CLK1KHZDIV())
            .field("CLK1KHZDIVUPDATEREQ", &self.CLK1KHZDIVUPDATEREQ())
            .field("CLK1HZDIV", &self.CLK1HZDIV())
            .field("CLK1HZDIVHALT", &self.CLK1HZDIVHALT())
            .field("CLK1HZDIVUPDATEREQ", &self.CLK1HZDIVUPDATEREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTCOSC32K {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTCOSC32K {{ SEL: {:?}, CLK1KHZDIV: {=u8:?}, CLK1KHZDIVUPDATEREQ: {=bool:?}, CLK1HZDIV: {=u16:?}, CLK1HZDIVHALT: {=bool:?}, CLK1HZDIVUPDATEREQ: {=bool:?} }}",
            self.SEL(),
            self.CLK1KHZDIV(),
            self.CLK1KHZDIVUPDATEREQ(),
            self.CLK1HZDIV(),
            self.CLK1HZDIVHALT(),
            self.CLK1HZDIVUPDATEREQ()
        )
    }
}
#[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SRAMCTRL(pub u32);
impl SRAMCTRL {
    #[doc = "Source Biasing voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn SMB(&self) -> super::vals::SMB {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SMB::from_bits(val as u8)
    }
    #[doc = "Source Biasing voltage."]
    #[inline(always)]
    pub const fn set_SMB(&mut self, val: super::vals::SMB) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Read Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn RM(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Read Margin control settings."]
    #[inline(always)]
    pub const fn set_RM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Write Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn WM(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Write Margin control settings."]
    #[inline(always)]
    pub const fn set_WM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Write read margin enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WRME(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write read margin enable."]
    #[inline(always)]
    pub const fn set_WRME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for SRAMCTRL {
    #[inline(always)]
    fn default() -> SRAMCTRL {
        SRAMCTRL(0)
    }
}
impl core::fmt::Debug for SRAMCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMCTRL")
            .field("SMB", &self.SMB())
            .field("RM", &self.RM())
            .field("WM", &self.WM())
            .field("WRME", &self.WRME())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SRAMCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SRAMCTRL {{ SMB: {:?}, RM: {=u8:?}, WM: {=u8:?}, WRME: {=bool:?} }}",
            self.SMB(),
            self.RM(),
            self.WM(),
            self.WRME()
        )
    }
}
#[doc = "Power Management Controller FSM (Finite State Machines) status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "Power Management Controller Main Finite State Machine (FSM) status."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMMAIN(&self) -> super::vals::FSMMAIN {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FSMMAIN::from_bits(val as u8)
    }
    #[doc = "Power Management Controller Main Finite State Machine (FSM) status."]
    #[inline(always)]
    pub const fn set_FSMMAIN(&mut self, val: super::vals::FSMMAIN) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "POWER UP Finite State Machine (FSM) status."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMPWUP(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "POWER UP Finite State Machine (FSM) status."]
    #[inline(always)]
    pub const fn set_FSMPWUP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "DEEP SLEEP Finite State Machine (FSM) status."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMDSLP(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "DEEP SLEEP Finite State Machine (FSM) status."]
    #[inline(always)]
    pub const fn set_FSMDSLP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
    }
    #[doc = "POWER DOWN Finite State Machine (FSM) status."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMPWDN(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "POWER DOWN Finite State Machine (FSM) status."]
    #[inline(always)]
    pub const fn set_FSMPWDN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "DEEP POWER DOWN Finite State Machine (FSM) status."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMDPWD(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "DEEP POWER DOWN Finite State Machine (FSM) status."]
    #[inline(always)]
    pub const fn set_FSMDPWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Latest IC Boot cause:."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOTMODE(&self) -> super::vals::BOOTMODE {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::BOOTMODE::from_bits(val as u8)
    }
    #[doc = "Latest IC Boot cause:."]
    #[inline(always)]
    pub const fn set_BOOTMODE(&mut self, val: super::vals::BOOTMODE) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Indicates cuurent status of wafer test level."]
    #[must_use]
    #[inline(always)]
    pub const fn WAFERTESTDONEVECT(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates cuurent status of wafer test level."]
    #[inline(always)]
    pub const fn set_WAFERTESTDONEVECT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("FSMMAIN", &self.FSMMAIN())
            .field("FSMPWUP", &self.FSMPWUP())
            .field("FSMDSLP", &self.FSMDSLP())
            .field("FSMPWDN", &self.FSMPWDN())
            .field("FSMDPWD", &self.FSMDPWD())
            .field("BOOTMODE", &self.BOOTMODE())
            .field("WAFERTESTDONEVECT", &self.WAFERTESTDONEVECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ FSMMAIN: {:?}, FSMPWUP: {=u8:?}, FSMDSLP: {=u8:?}, FSMPWDN: {=u8:?}, FSMDPWD: {=u8:?}, BOOTMODE: {:?}, WAFERTESTDONEVECT: {=u8:?} }}",
            self.FSMMAIN(),
            self.FSMPWUP(),
            self.FSMDSLP(),
            self.FSMPWDN(),
            self.FSMDPWD(),
            self.BOOTMODE(),
            self.WAFERTESTDONEVECT()
        )
    }
}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUSCLK(pub u32);
impl STATUSCLK {
    #[doc = "XTAL oscillator 32 K OK signal."]
    #[must_use]
    #[inline(always)]
    pub const fn XTAL32KOK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "XTAL oscillator 32 K OK signal."]
    #[inline(always)]
    pub const fn set_XTAL32KOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn XTAL32KOSCFAILURE(&self) -> super::vals::XTAL32KOSCFAILURE {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::XTAL32KOSCFAILURE::from_bits(val as u8)
    }
    #[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub const fn set_XTAL32KOSCFAILURE(&mut self, val: super::vals::XTAL32KOSCFAILURE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for STATUSCLK {
    #[inline(always)]
    fn default() -> STATUSCLK {
        STATUSCLK(0)
    }
}
impl core::fmt::Debug for STATUSCLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUSCLK")
            .field("XTAL32KOK", &self.XTAL32KOK())
            .field("XTAL32KOSCFAILURE", &self.XTAL32KOSCFAILURE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUSCLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUSCLK {{ XTAL32KOK: {=bool:?}, XTAL32KOSCFAILURE: {:?} }}",
            self.XTAL32KOK(),
            self.XTAL32KOSCFAILURE()
        )
    }
}
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKEIOCAUSE(pub u32);
impl WAKEIOCAUSE {
    #[doc = "Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_WAKEUP0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_WAKEUP1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_WAKEUP2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_WAKEUP3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for WAKEIOCAUSE {
    #[inline(always)]
    fn default() -> WAKEIOCAUSE {
        WAKEIOCAUSE(0)
    }
}
impl core::fmt::Debug for WAKEIOCAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEIOCAUSE")
            .field("WAKEUP0", &self.WAKEUP0())
            .field("WAKEUP1", &self.WAKEUP1())
            .field("WAKEUP2", &self.WAKEUP2())
            .field("WAKEUP3", &self.WAKEUP3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAKEIOCAUSE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAKEIOCAUSE {{ WAKEUP0: {=bool:?}, WAKEUP1: {=bool:?}, WAKEUP2: {=bool:?}, WAKEUP3: {=bool:?} }}",
            self.WAKEUP0(),
            self.WAKEUP1(),
            self.WAKEUP2(),
            self.WAKEUP3()
        )
    }
}
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKEUPIOCTRL(pub u32);
impl WAKEUPIOCTRL {
    #[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn RISINGEDGEWAKEUP0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_RISINGEDGEWAKEUP0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn FALLINGEDGEWAKEUP0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_FALLINGEDGEWAKEUP0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn RISINGEDGEWAKEUP1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_RISINGEDGEWAKEUP1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn FALLINGEDGEWAKEUP1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_FALLINGEDGEWAKEUP1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn RISINGEDGEWAKEUP2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_RISINGEDGEWAKEUP2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn FALLINGEDGEWAKEUP2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_FALLINGEDGEWAKEUP2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn RISINGEDGEWAKEUP3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_RISINGEDGEWAKEUP3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn FALLINGEDGEWAKEUP3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_FALLINGEDGEWAKEUP3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODEWAKEUPIOPAD0(&self) -> super::vals::MODEWAKEUPIOPAD0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::MODEWAKEUPIOPAD0::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODEWAKEUPIOPAD0(&mut self, val: super::vals::MODEWAKEUPIOPAD0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODEWAKEUPIOPAD1(&self) -> super::vals::MODEWAKEUPIOPAD1 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MODEWAKEUPIOPAD1::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODEWAKEUPIOPAD1(&mut self, val: super::vals::MODEWAKEUPIOPAD1) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODEWAKEUPIOPAD2(&self) -> super::vals::MODEWAKEUPIOPAD2 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MODEWAKEUPIOPAD2::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODEWAKEUPIOPAD2(&mut self, val: super::vals::MODEWAKEUPIOPAD2) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODEWAKEUPIOPAD3(&self) -> super::vals::MODEWAKEUPIOPAD3 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::MODEWAKEUPIOPAD3::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODEWAKEUPIOPAD3(&mut self, val: super::vals::MODEWAKEUPIOPAD3) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUPIO_ENABLE_CTRL(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub const fn set_WAKEUPIO_ENABLE_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "WAKEUP IO event detector reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUPIO_RSTN(&self) -> super::vals::WAKEUPIO_RSTN {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::WAKEUPIO_RSTN::from_bits(val as u8)
    }
    #[doc = "WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub const fn set_WAKEUPIO_RSTN(&mut self, val: super::vals::WAKEUPIO_RSTN) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for WAKEUPIOCTRL {
    #[inline(always)]
    fn default() -> WAKEUPIOCTRL {
        WAKEUPIOCTRL(0)
    }
}
impl core::fmt::Debug for WAKEUPIOCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUPIOCTRL")
            .field("RISINGEDGEWAKEUP0", &self.RISINGEDGEWAKEUP0())
            .field("FALLINGEDGEWAKEUP0", &self.FALLINGEDGEWAKEUP0())
            .field("RISINGEDGEWAKEUP1", &self.RISINGEDGEWAKEUP1())
            .field("FALLINGEDGEWAKEUP1", &self.FALLINGEDGEWAKEUP1())
            .field("RISINGEDGEWAKEUP2", &self.RISINGEDGEWAKEUP2())
            .field("FALLINGEDGEWAKEUP2", &self.FALLINGEDGEWAKEUP2())
            .field("RISINGEDGEWAKEUP3", &self.RISINGEDGEWAKEUP3())
            .field("FALLINGEDGEWAKEUP3", &self.FALLINGEDGEWAKEUP3())
            .field("MODEWAKEUPIOPAD0", &self.MODEWAKEUPIOPAD0())
            .field("MODEWAKEUPIOPAD1", &self.MODEWAKEUPIOPAD1())
            .field("MODEWAKEUPIOPAD2", &self.MODEWAKEUPIOPAD2())
            .field("MODEWAKEUPIOPAD3", &self.MODEWAKEUPIOPAD3())
            .field("WAKEUPIO_ENABLE_CTRL", &self.WAKEUPIO_ENABLE_CTRL())
            .field("WAKEUPIO_RSTN", &self.WAKEUPIO_RSTN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAKEUPIOCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAKEUPIOCTRL {{ RISINGEDGEWAKEUP0: {=bool:?}, FALLINGEDGEWAKEUP0: {=bool:?}, RISINGEDGEWAKEUP1: {=bool:?}, FALLINGEDGEWAKEUP1: {=bool:?}, RISINGEDGEWAKEUP2: {=bool:?}, FALLINGEDGEWAKEUP2: {=bool:?}, RISINGEDGEWAKEUP3: {=bool:?}, FALLINGEDGEWAKEUP3: {=bool:?}, MODEWAKEUPIOPAD0: {:?}, MODEWAKEUPIOPAD1: {:?}, MODEWAKEUPIOPAD2: {:?}, MODEWAKEUPIOPAD3: {:?}, WAKEUPIO_ENABLE_CTRL: {=bool:?}, WAKEUPIO_RSTN: {:?} }}",
            self.RISINGEDGEWAKEUP0(),
            self.FALLINGEDGEWAKEUP0(),
            self.RISINGEDGEWAKEUP1(),
            self.FALLINGEDGEWAKEUP1(),
            self.RISINGEDGEWAKEUP2(),
            self.FALLINGEDGEWAKEUP2(),
            self.RISINGEDGEWAKEUP3(),
            self.FALLINGEDGEWAKEUP3(),
            self.MODEWAKEUPIOPAD0(),
            self.MODEWAKEUPIOPAD1(),
            self.MODEWAKEUPIOPAD2(),
            self.MODEWAKEUPIOPAD3(),
            self.WAKEUPIO_ENABLE_CTRL(),
            self.WAKEUPIO_RSTN()
        )
    }
}
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XTAL32K(pub u32);
impl XTAL32K {
    #[doc = "reference output current selection inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn IREF(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "reference output current selection inputs."]
    #[inline(always)]
    pub const fn set_IREF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Oscillator Test Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Test Mode."]
    #[inline(always)]
    pub const fn set_TEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bias current selection inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn IBIAS(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "bias current selection inputs."]
    #[inline(always)]
    pub const fn set_IBIAS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "oscillator amplitude selection inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn AMPL(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "oscillator amplitude selection inputs."]
    #[inline(always)]
    pub const fn set_AMPL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Capa bank setting input."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPBANKIN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Capa bank setting input."]
    #[inline(always)]
    pub const fn set_CAPBANKIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Capa bank setting output."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPBANKOUT(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x7f;
        val as u8
    }
    #[doc = "Capa bank setting output."]
    #[inline(always)]
    pub const fn set_CAPBANKOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 15usize)) | (((val as u32) & 0x7f) << 15usize);
    }
    #[doc = "Source selection for xo32k_captest_start_ao_set."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTESTSTARTSRCSEL(&self) -> super::vals::CAPTESTSTARTSRCSEL {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::CAPTESTSTARTSRCSEL::from_bits(val as u8)
    }
    #[doc = "Source selection for xo32k_captest_start_ao_set."]
    #[inline(always)]
    pub const fn set_CAPTESTSTARTSRCSEL(&mut self, val: super::vals::CAPTESTSTARTSRCSEL) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Start test."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTESTSTART(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Start test."]
    #[inline(always)]
    pub const fn set_CAPTESTSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable signal for cap test."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTESTENABLE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable signal for cap test."]
    #[inline(always)]
    pub const fn set_CAPTESTENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Select the input for test."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTESTOSCINSEL(&self) -> super::vals::CAPTESTOSCINSEL {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::CAPTESTOSCINSEL::from_bits(val as u8)
    }
    #[doc = "Select the input for test."]
    #[inline(always)]
    pub const fn set_CAPTESTOSCINSEL(&mut self, val: super::vals::CAPTESTOSCINSEL) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for XTAL32K {
    #[inline(always)]
    fn default() -> XTAL32K {
        XTAL32K(0)
    }
}
impl core::fmt::Debug for XTAL32K {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL32K")
            .field("IREF", &self.IREF())
            .field("TEST", &self.TEST())
            .field("IBIAS", &self.IBIAS())
            .field("AMPL", &self.AMPL())
            .field("CAPBANKIN", &self.CAPBANKIN())
            .field("CAPBANKOUT", &self.CAPBANKOUT())
            .field("CAPTESTSTARTSRCSEL", &self.CAPTESTSTARTSRCSEL())
            .field("CAPTESTSTART", &self.CAPTESTSTART())
            .field("CAPTESTENABLE", &self.CAPTESTENABLE())
            .field("CAPTESTOSCINSEL", &self.CAPTESTOSCINSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XTAL32K {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XTAL32K {{ IREF: {=u8:?}, TEST: {=bool:?}, IBIAS: {=u8:?}, AMPL: {=u8:?}, CAPBANKIN: {=u8:?}, CAPBANKOUT: {=u8:?}, CAPTESTSTARTSRCSEL: {:?}, CAPTESTSTART: {=bool:?}, CAPTESTENABLE: {=bool:?}, CAPTESTOSCINSEL: {:?} }}",
            self.IREF(),
            self.TEST(),
            self.IBIAS(),
            self.AMPL(),
            self.CAPBANKIN(),
            self.CAPBANKOUT(),
            self.CAPTESTSTARTSRCSEL(),
            self.CAPTESTSTART(),
            self.CAPTESTENABLE(),
            self.CAPTESTOSCINSEL()
        )
    }
}
