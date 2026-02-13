#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoreg1(pub u32);
impl Aoreg1 {
    #[doc = "The last chip reset was caused by a Power On Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "The last chip reset was caused by a Pin Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn padreset(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub const fn set_padreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[must_use]
    #[inline(always)]
    pub const fn bodreset(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub const fn set_bodreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[must_use]
    #[inline(always)]
    pub const fn systemreset(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub const fn set_systemreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "The last chip reset was caused by the Watchdog Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtreset(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub const fn set_wdtreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "The last chip reset was caused by a Software event."]
    #[must_use]
    #[inline(always)]
    pub const fn swrreset(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub const fn set_swrreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdreset_wakeupio(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub const fn set_dpdreset_wakeupio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdreset_rtc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub const fn set_dpdreset_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdreset_ostimer(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub const fn set_dpdreset_ostimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ROM Boot Fatal Error Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn booterrorcounter(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub const fn set_booterrorcounter(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Aoreg1 {
    #[inline(always)]
    fn default() -> Aoreg1 {
        Aoreg1(0)
    }
}
impl core::fmt::Debug for Aoreg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aoreg1")
            .field("por", &self.por())
            .field("padreset", &self.padreset())
            .field("bodreset", &self.bodreset())
            .field("systemreset", &self.systemreset())
            .field("wdtreset", &self.wdtreset())
            .field("swrreset", &self.swrreset())
            .field("dpdreset_wakeupio", &self.dpdreset_wakeupio())
            .field("dpdreset_rtc", &self.dpdreset_rtc())
            .field("dpdreset_ostimer", &self.dpdreset_ostimer())
            .field("booterrorcounter", &self.booterrorcounter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aoreg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aoreg1 {{ por: {=bool:?}, padreset: {=bool:?}, bodreset: {=bool:?}, systemreset: {=bool:?}, wdtreset: {=bool:?}, swrreset: {=bool:?}, dpdreset_wakeupio: {=bool:?}, dpdreset_rtc: {=bool:?}, dpdreset_ostimer: {=bool:?}, booterrorcounter: {=u8:?} }}",
            self.por(),
            self.padreset(),
            self.bodreset(),
            self.systemreset(),
            self.wdtreset(),
            self.swrreset(),
            self.dpdreset_wakeupio(),
            self.dpdreset_rtc(),
            self.dpdreset_ostimer(),
            self.booterrorcounter()
        )
    }
}
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bodvbat(pub u32);
impl Bodvbat {
    #[doc = "BoD trigger level."]
    #[must_use]
    #[inline(always)]
    pub const fn triglvl(&self) -> super::vals::Triglvl {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Triglvl::from_bits(val as u8)
    }
    #[doc = "BoD trigger level."]
    #[inline(always)]
    pub const fn set_triglvl(&mut self, val: super::vals::Triglvl) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "BoD Hysteresis control."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> super::vals::BodvbatHyst {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::BodvbatHyst::from_bits(val as u8)
    }
    #[doc = "BoD Hysteresis control."]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: super::vals::BodvbatHyst) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
}
impl Default for Bodvbat {
    #[inline(always)]
    fn default() -> Bodvbat {
        Bodvbat(0)
    }
}
impl core::fmt::Debug for Bodvbat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bodvbat")
            .field("triglvl", &self.triglvl())
            .field("hyst", &self.hyst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bodvbat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bodvbat {{ triglvl: {:?}, hyst: {:?} }}",
            self.triglvl(),
            self.hyst()
        )
    }
}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp(pub u32);
impl Comp {
    #[doc = "Hysteris when hyst = '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hysteris when hyst = '1'."]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[must_use]
    #[inline(always)]
    pub const fn vrefinput(&self) -> super::vals::Vrefinput {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Vrefinput::from_bits(val as u8)
    }
    #[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub const fn set_vrefinput(&mut self, val: super::vals::Vrefinput) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Low power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lowpower(&self) -> super::vals::Lowpower {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lowpower::from_bits(val as u8)
    }
    #[doc = "Low power mode."]
    #[inline(always)]
    pub const fn set_lowpower(&mut self, val: super::vals::Lowpower) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Control word for P multiplexer:."]
    #[must_use]
    #[inline(always)]
    pub const fn pmux(&self) -> super::vals::Pmux {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Pmux::from_bits(val as u8)
    }
    #[doc = "Control word for P multiplexer:."]
    #[inline(always)]
    pub const fn set_pmux(&mut self, val: super::vals::Pmux) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Control word for N multiplexer:."]
    #[must_use]
    #[inline(always)]
    pub const fn nmux(&self) -> super::vals::Nmux {
        let val = (self.0 >> 7usize) & 0x07;
        super::vals::Nmux::from_bits(val as u8)
    }
    #[doc = "Control word for N multiplexer:."]
    #[inline(always)]
    pub const fn set_nmux(&mut self, val: super::vals::Nmux) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Control the filtering of the Analog Comparator output."]
    #[must_use]
    #[inline(always)]
    pub const fn filtercgf_samplemode(&self) -> super::vals::FiltercgfSamplemode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FiltercgfSamplemode::from_bits(val as u8)
    }
    #[doc = "Control the filtering of the Analog Comparator output."]
    #[inline(always)]
    pub const fn set_filtercgf_samplemode(&mut self, val: super::vals::FiltercgfSamplemode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Filter Clock divider."]
    #[must_use]
    #[inline(always)]
    pub const fn filtercgf_clkdiv(&self) -> super::vals::FiltercgfClkdiv {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::FiltercgfClkdiv::from_bits(val as u8)
    }
    #[doc = "Filter Clock divider."]
    #[inline(always)]
    pub const fn set_filtercgf_clkdiv(&mut self, val: super::vals::FiltercgfClkdiv) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
}
impl Default for Comp {
    #[inline(always)]
    fn default() -> Comp {
        Comp(0)
    }
}
impl core::fmt::Debug for Comp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp")
            .field("hyst", &self.hyst())
            .field("vrefinput", &self.vrefinput())
            .field("lowpower", &self.lowpower())
            .field("pmux", &self.pmux())
            .field("nmux", &self.nmux())
            .field("vref", &self.vref())
            .field("filtercgf_samplemode", &self.filtercgf_samplemode())
            .field("filtercgf_clkdiv", &self.filtercgf_clkdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp {{ hyst: {=bool:?}, vrefinput: {:?}, lowpower: {:?}, pmux: {:?}, nmux: {:?}, vref: {=u8:?}, filtercgf_samplemode: {:?}, filtercgf_clkdiv: {:?} }}",
            self.hyst(),
            self.vrefinput(),
            self.lowpower(),
            self.pmux(),
            self.nmux(),
            self.vref(),
            self.filtercgf_samplemode(),
            self.filtercgf_clkdiv()
        )
    }
}
#[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdc0(pub u32);
impl Dcdc0 {
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn rc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_rc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn icomp(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_icomp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn isel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_isel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn icenable(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_icenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn tmos(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_tmos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn disableisense(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_disableisense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> super::vals::Vout {
        let val = (self.0 >> 17usize) & 0x0f;
        super::vals::Vout::from_bits(val as u8)
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: super::vals::Vout) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn slicingenable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_slicingenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn inductorclampenable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_inductorclampenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_pwd(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_vout_pwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for Dcdc0 {
    #[inline(always)]
    fn default() -> Dcdc0 {
        Dcdc0(0)
    }
}
impl core::fmt::Debug for Dcdc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdc0")
            .field("rc", &self.rc())
            .field("icomp", &self.icomp())
            .field("isel", &self.isel())
            .field("icenable", &self.icenable())
            .field("tmos", &self.tmos())
            .field("disableisense", &self.disableisense())
            .field("vout", &self.vout())
            .field("slicingenable", &self.slicingenable())
            .field("inductorclampenable", &self.inductorclampenable())
            .field("vout_pwd", &self.vout_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdc0 {{ rc: {=u8:?}, icomp: {=u8:?}, isel: {=u8:?}, icenable: {=bool:?}, tmos: {=u8:?}, disableisense: {=bool:?}, vout: {:?}, slicingenable: {=bool:?}, inductorclampenable: {=bool:?}, vout_pwd: {=u8:?} }}",
            self.rc(),
            self.icomp(),
            self.isel(),
            self.icenable(),
            self.tmos(),
            self.disableisense(),
            self.vout(),
            self.slicingenable(),
            self.inductorclampenable(),
            self.vout_pwd()
        )
    }
}
#[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdc1(pub u32);
impl Dcdc1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn rtrimoffet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_rtrimoffet(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn rsensetrim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_rsensetrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestenable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_dtestenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setcurve(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setcurve(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn setdc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_setdc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn dtestsel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_dtestsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn iscaleenable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_iscaleenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn forcebypass(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_forcebypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn trimautocot(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_trimautocot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn forcefullcycle(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_forcefullcycle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn lcenable(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_lcenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn toff(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_toff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn toffenable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_toffenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dcdc1 {
    #[inline(always)]
    fn default() -> Dcdc1 {
        Dcdc1(0)
    }
}
impl core::fmt::Debug for Dcdc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcdc1")
            .field("rtrimoffet", &self.rtrimoffet())
            .field("rsensetrim", &self.rsensetrim())
            .field("dtestenable", &self.dtestenable())
            .field("setcurve", &self.setcurve())
            .field("setdc", &self.setdc())
            .field("dtestsel", &self.dtestsel())
            .field("iscaleenable", &self.iscaleenable())
            .field("forcebypass", &self.forcebypass())
            .field("trimautocot", &self.trimautocot())
            .field("forcefullcycle", &self.forcefullcycle())
            .field("lcenable", &self.lcenable())
            .field("toff", &self.toff())
            .field("toffenable", &self.toffenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcdc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dcdc1 {{ rtrimoffet: {=u8:?}, rsensetrim: {=u8:?}, dtestenable: {=bool:?}, setcurve: {=u8:?}, setdc: {=u8:?}, dtestsel: {=u8:?}, iscaleenable: {=bool:?}, forcebypass: {=bool:?}, trimautocot: {=u8:?}, forcefullcycle: {=bool:?}, lcenable: {=bool:?}, toff: {=u8:?}, toffenable: {=bool:?} }}",
            self.rtrimoffet(),
            self.rsensetrim(),
            self.dtestenable(),
            self.setcurve(),
            self.setdc(),
            self.dtestsel(),
            self.iscaleenable(),
            self.forcebypass(),
            self.trimautocot(),
            self.forcefullcycle(),
            self.lcenable(),
            self.toff(),
            self.toffenable()
        )
    }
}
#[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldopmu(pub u32);
impl Ldopmu {
    #[doc = "Sets the Always-On domain LDO output level."]
    #[must_use]
    #[inline(always)]
    pub const fn vadj(&self) -> super::vals::Vadj {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Vadj::from_bits(val as u8)
    }
    #[doc = "Sets the Always-On domain LDO output level."]
    #[inline(always)]
    pub const fn set_vadj(&mut self, val: super::vals::Vadj) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Sets the Always-On domain LDO output level in all power down modes."]
    #[must_use]
    #[inline(always)]
    pub const fn vadj_pwd(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the Always-On domain LDO output level in all power down modes."]
    #[inline(always)]
    pub const fn set_vadj_pwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Sets the Always-On domain LDO Boost output level."]
    #[must_use]
    #[inline(always)]
    pub const fn vadj_boost(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the Always-On domain LDO Boost output level."]
    #[inline(always)]
    pub const fn set_vadj_boost(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Sets the Always-On domain LDO Boost output level in all power down modes."]
    #[must_use]
    #[inline(always)]
    pub const fn vadj_boost_pwd(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the Always-On domain LDO Boost output level in all power down modes."]
    #[inline(always)]
    pub const fn set_vadj_boost_pwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "Control the LDO AO boost mode in ACTIVE mode."]
    #[must_use]
    #[inline(always)]
    pub const fn boost_ena(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Control the LDO AO boost mode in ACTIVE mode."]
    #[inline(always)]
    pub const fn set_boost_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
    #[must_use]
    #[inline(always)]
    pub const fn boost_ena_pwd(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Control the LDO AO boost mode in the different low power modes (DEEP SLEEP, POWERDOWN, and DEEP POWER DOWN)."]
    #[inline(always)]
    pub const fn set_boost_ena_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Ldopmu {
    #[inline(always)]
    fn default() -> Ldopmu {
        Ldopmu(0)
    }
}
impl core::fmt::Debug for Ldopmu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldopmu")
            .field("vadj", &self.vadj())
            .field("vadj_pwd", &self.vadj_pwd())
            .field("vadj_boost", &self.vadj_boost())
            .field("vadj_boost_pwd", &self.vadj_boost_pwd())
            .field("boost_ena", &self.boost_ena())
            .field("boost_ena_pwd", &self.boost_ena_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldopmu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldopmu {{ vadj: {:?}, vadj_pwd: {=u8:?}, vadj_boost: {=u8:?}, vadj_boost_pwd: {=u8:?}, boost_ena: {=bool:?}, boost_ena_pwd: {=bool:?} }}",
            self.vadj(),
            self.vadj_pwd(),
            self.vadj_boost(),
            self.vadj_boost_pwd(),
            self.boost_ena(),
            self.boost_ena_pwd()
        )
    }
}
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miscctrl(pub u32);
impl Miscctrl {
    #[doc = "Select LDO Deep Sleep reference source."]
    #[must_use]
    #[inline(always)]
    pub const fn ldodeepsleepref(&self) -> super::vals::Ldodeepsleepref {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ldodeepsleepref::from_bits(val as u8)
    }
    #[doc = "Select LDO Deep Sleep reference source."]
    #[inline(always)]
    pub const fn set_ldodeepsleepref(&mut self, val: super::vals::Ldodeepsleepref) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Control the activation of LDO MEM High Z mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ldomemhighzmode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub const fn set_ldomemhighzmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lowpwr_flash_buf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lowpwr_flash_buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn miscctrl_3_8(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_miscctrl_3_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "Configure wake up I/O 0 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 0 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Configure wake up I/O 1 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 1 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Configure wake up I/O 2 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 2 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Configure wake up I/O 3 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 3 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_bleed(&self) -> super::vals::DisableBleed {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DisableBleed::from_bits(val as u8)
    }
    #[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub const fn set_disable_bleed(&mut self, val: super::vals::DisableBleed) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn miscctrl_13_14(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_miscctrl_13_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "WAKEUP IO event detector reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn wakupio_rst(&self) -> super::vals::WakupioRst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::WakupioRst::from_bits(val as u8)
    }
    #[doc = "WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub const fn set_wakupio_rst(&mut self, val: super::vals::WakupioRst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Miscctrl {
    #[inline(always)]
    fn default() -> Miscctrl {
        Miscctrl(0)
    }
}
impl core::fmt::Debug for Miscctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Miscctrl")
            .field("ldodeepsleepref", &self.ldodeepsleepref())
            .field("ldomemhighzmode", &self.ldomemhighzmode())
            .field("lowpwr_flash_buf", &self.lowpwr_flash_buf())
            .field("miscctrl_3_8", &self.miscctrl_3_8())
            .field("modewakeup0", &self.modewakeup0())
            .field("modewakeup1", &self.modewakeup1())
            .field("modewakeup2", &self.modewakeup2())
            .field("modewakeup3", &self.modewakeup3())
            .field("disable_bleed", &self.disable_bleed())
            .field("miscctrl_13_14", &self.miscctrl_13_14())
            .field("wakupio_rst", &self.wakupio_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Miscctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Miscctrl {{ ldodeepsleepref: {:?}, ldomemhighzmode: {=bool:?}, lowpwr_flash_buf: {=bool:?}, miscctrl_3_8: {=u8:?}, modewakeup0: {=bool:?}, modewakeup1: {=bool:?}, modewakeup2: {=bool:?}, modewakeup3: {=bool:?}, disable_bleed: {:?}, miscctrl_13_14: {=u8:?}, wakupio_rst: {:?} }}",
            self.ldodeepsleepref(),
            self.ldomemhighzmode(),
            self.lowpwr_flash_buf(),
            self.miscctrl_3_8(),
            self.modewakeup0(),
            self.modewakeup1(),
            self.modewakeup2(),
            self.modewakeup3(),
            self.disable_bleed(),
            self.miscctrl_13_14(),
            self.wakupio_rst()
        )
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostimer(pub u32);
impl Ostimer {
    #[doc = "Active high reset."]
    #[must_use]
    #[inline(always)]
    pub const fn softreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Active high reset."]
    #[inline(always)]
    pub const fn set_softreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable OSTIMER 32 KHz clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clockenable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub const fn set_clockenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdwakeupenable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub const fn set_dpdwakeupenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[must_use]
    #[inline(always)]
    pub const fn osc32kpd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub const fn set_osc32kpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ostimer {
    #[inline(always)]
    fn default() -> Ostimer {
        Ostimer(0)
    }
}
impl core::fmt::Debug for Ostimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ostimer")
            .field("softreset", &self.softreset())
            .field("clockenable", &self.clockenable())
            .field("dpdwakeupenable", &self.dpdwakeupenable())
            .field("osc32kpd", &self.osc32kpd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ostimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ostimer {{ softreset: {=bool:?}, clockenable: {=bool:?}, dpdwakeupenable: {=bool:?}, osc32kpd: {=bool:?} }}",
            self.softreset(),
            self.clockenable(),
            self.dpdwakeupenable(),
            self.osc32kpd()
        )
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg0(pub u32);
impl Pdruncfg0 {
    #[doc = "Controls power to VBAT Brown Out Detector (BOD)."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_bodvbat(&self) -> super::vals::PdenBodvbat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PdenBodvbat::from_bits(val as u8)
    }
    #[doc = "Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub const fn set_pden_bodvbat(&mut self, val: super::vals::PdenBodvbat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_fro32k(&self) -> super::vals::PdenFro32k {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PdenFro32k::from_bits(val as u8)
    }
    #[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub const fn set_pden_fro32k(&mut self, val: super::vals::PdenFro32k) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Controls power to crystal 32 KHz."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_xtal32k(&self) -> super::vals::PdenXtal32k {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::PdenXtal32k::from_bits(val as u8)
    }
    #[doc = "Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub const fn set_pden_xtal32k(&mut self, val: super::vals::PdenXtal32k) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Controls power to high speed crystal."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_xtal32m(&self) -> super::vals::PdenXtal32m {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PdenXtal32m::from_bits(val as u8)
    }
    #[doc = "Controls power to high speed crystal."]
    #[inline(always)]
    pub const fn set_pden_xtal32m(&mut self, val: super::vals::PdenXtal32m) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls power to System PLL (also refered as PLL0)."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_pll0(&self) -> super::vals::PdenPll0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PdenPll0::from_bits(val as u8)
    }
    #[doc = "Controls power to System PLL (also refered as PLL0)."]
    #[inline(always)]
    pub const fn set_pden_pll0(&mut self, val: super::vals::PdenPll0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Controls power to USB PLL (also refered as PLL1)."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_pll1(&self) -> super::vals::PdenPll1 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PdenPll1::from_bits(val as u8)
    }
    #[doc = "Controls power to USB PLL (also refered as PLL1)."]
    #[inline(always)]
    pub const fn set_pden_pll1(&mut self, val: super::vals::PdenPll1) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Controls power to USB Full Speed phy."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_usbfsphy(&self) -> super::vals::PdenUsbfsphy {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PdenUsbfsphy::from_bits(val as u8)
    }
    #[doc = "Controls power to USB Full Speed phy."]
    #[inline(always)]
    pub const fn set_pden_usbfsphy(&mut self, val: super::vals::PdenUsbfsphy) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls power to USB High Speed Phy."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_usbhsphy(&self) -> super::vals::PdenUsbhsphy {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PdenUsbhsphy::from_bits(val as u8)
    }
    #[doc = "Controls power to USB High Speed Phy."]
    #[inline(always)]
    pub const fn set_pden_usbhsphy(&mut self, val: super::vals::PdenUsbhsphy) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Controls power to Analog Comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_comp(&self) -> super::vals::PdenComp {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PdenComp::from_bits(val as u8)
    }
    #[doc = "Controls power to Analog Comparator."]
    #[inline(always)]
    pub const fn set_pden_comp(&mut self, val: super::vals::PdenComp) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Controls power to USB high speed LDO."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_ldousbhs(&self) -> super::vals::PdenLdousbhs {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PdenLdousbhs::from_bits(val as u8)
    }
    #[doc = "Controls power to USB high speed LDO."]
    #[inline(always)]
    pub const fn set_pden_ldousbhs(&mut self, val: super::vals::PdenLdousbhs) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Controls power to auxiliary biasing (AUXBIAS)"]
    #[must_use]
    #[inline(always)]
    pub const fn pden_auxbias(&self) -> super::vals::PdenAuxbias {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PdenAuxbias::from_bits(val as u8)
    }
    #[doc = "Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub const fn set_pden_auxbias(&mut self, val: super::vals::PdenAuxbias) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Controls power to high speed crystal LDO."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_ldoxo32m(&self) -> super::vals::PdenLdoxo32m {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PdenLdoxo32m::from_bits(val as u8)
    }
    #[doc = "Controls power to high speed crystal LDO."]
    #[inline(always)]
    pub const fn set_pden_ldoxo32m(&mut self, val: super::vals::PdenLdoxo32m) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_rng(&self) -> super::vals::PdenRng {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::PdenRng::from_bits(val as u8)
    }
    #[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub const fn set_pden_rng(&mut self, val: super::vals::PdenRng) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[must_use]
    #[inline(always)]
    pub const fn pden_pll0_sscg(&self) -> super::vals::PdenPll0Sscg {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PdenPll0Sscg::from_bits(val as u8)
    }
    #[doc = "Controls power to System PLL (PLL0) Spread Spectrum module."]
    #[inline(always)]
    pub const fn set_pden_pll0_sscg(&mut self, val: super::vals::PdenPll0Sscg) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Pdruncfg0 {
    #[inline(always)]
    fn default() -> Pdruncfg0 {
        Pdruncfg0(0)
    }
}
impl core::fmt::Debug for Pdruncfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg0")
            .field("pden_bodvbat", &self.pden_bodvbat())
            .field("pden_fro32k", &self.pden_fro32k())
            .field("pden_xtal32k", &self.pden_xtal32k())
            .field("pden_xtal32m", &self.pden_xtal32m())
            .field("pden_pll0", &self.pden_pll0())
            .field("pden_pll1", &self.pden_pll1())
            .field("pden_usbfsphy", &self.pden_usbfsphy())
            .field("pden_usbhsphy", &self.pden_usbhsphy())
            .field("pden_comp", &self.pden_comp())
            .field("pden_ldousbhs", &self.pden_ldousbhs())
            .field("pden_auxbias", &self.pden_auxbias())
            .field("pden_ldoxo32m", &self.pden_ldoxo32m())
            .field("pden_rng", &self.pden_rng())
            .field("pden_pll0_sscg", &self.pden_pll0_sscg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg0 {{ pden_bodvbat: {:?}, pden_fro32k: {:?}, pden_xtal32k: {:?}, pden_xtal32m: {:?}, pden_pll0: {:?}, pden_pll1: {:?}, pden_usbfsphy: {:?}, pden_usbhsphy: {:?}, pden_comp: {:?}, pden_ldousbhs: {:?}, pden_auxbias: {:?}, pden_ldoxo32m: {:?}, pden_rng: {:?}, pden_pll0_sscg: {:?} }}",
            self.pden_bodvbat(),
            self.pden_fro32k(),
            self.pden_xtal32k(),
            self.pden_xtal32m(),
            self.pden_pll0(),
            self.pden_pll1(),
            self.pden_usbfsphy(),
            self.pden_usbhsphy(),
            self.pden_comp(),
            self.pden_ldousbhs(),
            self.pden_auxbias(),
            self.pden_ldoxo32m(),
            self.pden_rng(),
            self.pden_pll0_sscg()
        )
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfgclr0(pub u32);
impl Pdruncfgclr0 {
    #[doc = "Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn pdruncfgclr0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub const fn set_pdruncfgclr0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pdruncfgclr0 {
    #[inline(always)]
    fn default() -> Pdruncfgclr0 {
        Pdruncfgclr0(0)
    }
}
impl core::fmt::Debug for Pdruncfgclr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfgclr0")
            .field("pdruncfgclr0", &self.pdruncfgclr0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfgclr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfgclr0 {{ pdruncfgclr0: {=u32:?} }}",
            self.pdruncfgclr0()
        )
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfgset0(pub u32);
impl Pdruncfgset0 {
    #[doc = "Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn pdruncfgset0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub const fn set_pdruncfgset0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pdruncfgset0 {
    #[inline(always)]
    fn default() -> Pdruncfgset0 {
        Pdruncfgset0(0)
    }
}
impl core::fmt::Debug for Pdruncfgset0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfgset0")
            .field("pdruncfgset0", &self.pdruncfgset0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfgset0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfgset0 {{ pdruncfgset0: {=u32:?} }}",
            self.pdruncfgset0()
        )
    }
}
#[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reffastwkup(pub u32);
impl Reffastwkup {
    #[doc = "Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
    #[must_use]
    #[inline(always)]
    pub const fn lpwkup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
    #[inline(always)]
    pub const fn set_lpwkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Analog References fast wake-up in case of Hardware Pin reset: ."]
    #[must_use]
    #[inline(always)]
    pub const fn hwwkup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog References fast wake-up in case of Hardware Pin reset: ."]
    #[inline(always)]
    pub const fn set_hwwkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Reffastwkup {
    #[inline(always)]
    fn default() -> Reffastwkup {
        Reffastwkup(0)
    }
}
impl core::fmt::Debug for Reffastwkup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reffastwkup")
            .field("lpwkup", &self.lpwkup())
            .field("hwwkup", &self.hwwkup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reffastwkup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reffastwkup {{ lpwkup: {=bool:?}, hwwkup: {=bool:?} }}",
            self.lpwkup(),
            self.hwwkup()
        )
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resetctrl(pub u32);
impl Resetctrl {
    #[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdwakeupresetenable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub const fn set_dpdwakeupresetenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOD VBAT reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodvbatresetenable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT reset enable."]
    #[inline(always)]
    pub const fn set_bodvbatresetenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BOD CORE reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcoreresetenable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE reset enable."]
    #[inline(always)]
    pub const fn set_bodcoreresetenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Software reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swrresetenable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset enable."]
    #[inline(always)]
    pub const fn set_swrresetenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Resetctrl {
    #[inline(always)]
    fn default() -> Resetctrl {
        Resetctrl(0)
    }
}
impl core::fmt::Debug for Resetctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Resetctrl")
            .field("dpdwakeupresetenable", &self.dpdwakeupresetenable())
            .field("bodvbatresetenable", &self.bodvbatresetenable())
            .field("bodcoreresetenable", &self.bodcoreresetenable())
            .field("swrresetenable", &self.swrresetenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Resetctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Resetctrl {{ dpdwakeupresetenable: {=bool:?}, bodvbatresetenable: {=bool:?}, bodcoreresetenable: {=bool:?}, swrresetenable: {=bool:?} }}",
            self.dpdwakeupresetenable(),
            self.bodvbatresetenable(),
            self.bodcoreresetenable(),
            self.swrresetenable()
        )
    }
}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtcosc32k(pub u32);
impl Rtcosc32k {
    #[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sel::from_bits(val as u8)
    }
    #[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Sel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Actual division ratio is : 28 + CLK1KHZDIV."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1khzdiv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub const fn set_clk1khzdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "RTC 1KHz clock Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1khzdivupdatereq(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub const fn set_clk1khzdivupdatereq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Actual division ratio is : 31744 + CLK1HZDIV."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1hzdiv(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub const fn set_clk1hzdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1hzdivhalt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_clk1hzdivhalt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "RTC 1Hz Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1hzdivupdatereq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub const fn set_clk1hzdivupdatereq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rtcosc32k {
    #[inline(always)]
    fn default() -> Rtcosc32k {
        Rtcosc32k(0)
    }
}
impl core::fmt::Debug for Rtcosc32k {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rtcosc32k")
            .field("sel", &self.sel())
            .field("clk1khzdiv", &self.clk1khzdiv())
            .field("clk1khzdivupdatereq", &self.clk1khzdivupdatereq())
            .field("clk1hzdiv", &self.clk1hzdiv())
            .field("clk1hzdivhalt", &self.clk1hzdivhalt())
            .field("clk1hzdivupdatereq", &self.clk1hzdivupdatereq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rtcosc32k {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rtcosc32k {{ sel: {:?}, clk1khzdiv: {=u8:?}, clk1khzdivupdatereq: {=bool:?}, clk1hzdiv: {=u16:?}, clk1hzdivhalt: {=bool:?}, clk1hzdivupdatereq: {=bool:?} }}",
            self.sel(),
            self.clk1khzdiv(),
            self.clk1khzdivupdatereq(),
            self.clk1hzdiv(),
            self.clk1hzdivhalt(),
            self.clk1hzdivupdatereq()
        )
    }
}
#[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramctrl(pub u32);
impl Sramctrl {
    #[doc = "Source Biasing voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn smb(&self) -> super::vals::Smb {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Smb::from_bits(val as u8)
    }
    #[doc = "Source Biasing voltage."]
    #[inline(always)]
    pub const fn set_smb(&mut self, val: super::vals::Smb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Read Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn rm(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Read Margin control settings."]
    #[inline(always)]
    pub const fn set_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Write Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn wm(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Write Margin control settings."]
    #[inline(always)]
    pub const fn set_wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Write read margin enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wrme(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write read margin enable."]
    #[inline(always)]
    pub const fn set_wrme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Sramctrl {
    #[inline(always)]
    fn default() -> Sramctrl {
        Sramctrl(0)
    }
}
impl core::fmt::Debug for Sramctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramctrl")
            .field("smb", &self.smb())
            .field("rm", &self.rm())
            .field("wm", &self.wm())
            .field("wrme", &self.wrme())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramctrl {{ smb: {:?}, rm: {=u8:?}, wm: {=u8:?}, wrme: {=bool:?} }}",
            self.smb(),
            self.rm(),
            self.wm(),
            self.wrme()
        )
    }
}
#[doc = "Power Management Controller FSM (Finite State Machines) status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Latest IC Boot cause:."]
    #[must_use]
    #[inline(always)]
    pub const fn bootmode(&self) -> super::vals::Bootmode {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Bootmode::from_bits(val as u8)
    }
    #[doc = "Latest IC Boot cause:."]
    #[inline(always)]
    pub const fn set_bootmode(&mut self, val: super::vals::Bootmode) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
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
            .field("bootmode", &self.bootmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ bootmode: {:?} }}", self.bootmode())
    }
}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusclk(pub u32);
impl Statusclk {
    #[doc = "XTAL oscillator 32 K OK signal."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal32kok(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "XTAL oscillator 32 K OK signal."]
    #[inline(always)]
    pub const fn set_xtal32kok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal32koscfailure(&self) -> super::vals::Xtal32koscfailure {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Xtal32koscfailure::from_bits(val as u8)
    }
    #[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub const fn set_xtal32koscfailure(&mut self, val: super::vals::Xtal32koscfailure) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Statusclk {
    #[inline(always)]
    fn default() -> Statusclk {
        Statusclk(0)
    }
}
impl core::fmt::Debug for Statusclk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statusclk")
            .field("xtal32kok", &self.xtal32kok())
            .field("xtal32koscfailure", &self.xtal32koscfailure())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusclk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Statusclk {{ xtal32kok: {=bool:?}, xtal32koscfailure: {:?} }}",
            self.xtal32kok(),
            self.xtal32koscfailure()
        )
    }
}
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeiocause(pub u32);
impl Wakeiocause {
    #[doc = "Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_wakeup0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_wakeup1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_wakeup2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub const fn set_wakeup3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Wakeiocause {
    #[inline(always)]
    fn default() -> Wakeiocause {
        Wakeiocause(0)
    }
}
impl core::fmt::Debug for Wakeiocause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeiocause")
            .field("wakeup0", &self.wakeup0())
            .field("wakeup1", &self.wakeup1())
            .field("wakeup2", &self.wakeup2())
            .field("wakeup3", &self.wakeup3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeiocause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wakeiocause {{ wakeup0: {=bool:?}, wakeup1: {=bool:?}, wakeup2: {=bool:?}, wakeup3: {=bool:?} }}",
            self.wakeup0(),
            self.wakeup1(),
            self.wakeup2(),
            self.wakeup3()
        )
    }
}
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupioctrl(pub u32);
impl Wakeupioctrl {
    #[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn risingedgewakeup0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_risingedgewakeup0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn fallingedgewakeup0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_fallingedgewakeup0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn risingedgewakeup1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_risingedgewakeup1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn fallingedgewakeup1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_fallingedgewakeup1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn risingedgewakeup2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_risingedgewakeup2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn fallingedgewakeup2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_fallingedgewakeup2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn risingedgewakeup3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_risingedgewakeup3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[must_use]
    #[inline(always)]
    pub const fn fallingedgewakeup3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub const fn set_fallingedgewakeup3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure wake up I/O 0 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 0 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Configure wake up I/O 1 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 1 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Configure wake up I/O 2 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 2 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Configure wake up I/O 3 in Deep Power Down mode"]
    #[must_use]
    #[inline(always)]
    pub const fn modewakeup3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Configure wake up I/O 3 in Deep Power Down mode"]
    #[inline(always)]
    pub const fn set_modewakeup3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Wakeupioctrl {
    #[inline(always)]
    fn default() -> Wakeupioctrl {
        Wakeupioctrl(0)
    }
}
impl core::fmt::Debug for Wakeupioctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupioctrl")
            .field("risingedgewakeup0", &self.risingedgewakeup0())
            .field("fallingedgewakeup0", &self.fallingedgewakeup0())
            .field("risingedgewakeup1", &self.risingedgewakeup1())
            .field("fallingedgewakeup1", &self.fallingedgewakeup1())
            .field("risingedgewakeup2", &self.risingedgewakeup2())
            .field("fallingedgewakeup2", &self.fallingedgewakeup2())
            .field("risingedgewakeup3", &self.risingedgewakeup3())
            .field("fallingedgewakeup3", &self.fallingedgewakeup3())
            .field("modewakeup0", &self.modewakeup0())
            .field("modewakeup1", &self.modewakeup1())
            .field("modewakeup2", &self.modewakeup2())
            .field("modewakeup3", &self.modewakeup3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupioctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wakeupioctrl {{ risingedgewakeup0: {=bool:?}, fallingedgewakeup0: {=bool:?}, risingedgewakeup1: {=bool:?}, fallingedgewakeup1: {=bool:?}, risingedgewakeup2: {=bool:?}, fallingedgewakeup2: {=bool:?}, risingedgewakeup3: {=bool:?}, fallingedgewakeup3: {=bool:?}, modewakeup0: {=bool:?}, modewakeup1: {=bool:?}, modewakeup2: {=bool:?}, modewakeup3: {=bool:?} }}",
            self.risingedgewakeup0(),
            self.fallingedgewakeup0(),
            self.risingedgewakeup1(),
            self.fallingedgewakeup1(),
            self.risingedgewakeup2(),
            self.fallingedgewakeup2(),
            self.risingedgewakeup3(),
            self.fallingedgewakeup3(),
            self.modewakeup0(),
            self.modewakeup1(),
            self.modewakeup2(),
            self.modewakeup3()
        )
    }
}
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32k(pub u32);
impl Xtal32k {
    #[doc = "reference output current selection inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn iref(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "reference output current selection inputs."]
    #[inline(always)]
    pub const fn set_iref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Oscillator Test Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator Test Mode."]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bias current selection inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn ibias(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "bias current selection inputs."]
    #[inline(always)]
    pub const fn set_ibias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "oscillator amplitude selection inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn ampl(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "oscillator amplitude selection inputs."]
    #[inline(always)]
    pub const fn set_ampl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Capa bank setting input."]
    #[must_use]
    #[inline(always)]
    pub const fn capbankin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Capa bank setting input."]
    #[inline(always)]
    pub const fn set_capbankin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Capa bank setting output."]
    #[must_use]
    #[inline(always)]
    pub const fn capbankout(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x7f;
        val as u8
    }
    #[doc = "Capa bank setting output."]
    #[inline(always)]
    pub const fn set_capbankout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 15usize)) | (((val as u32) & 0x7f) << 15usize);
    }
    #[doc = "Source selection for xo32k_captest_start_ao_set."]
    #[must_use]
    #[inline(always)]
    pub const fn capteststartsrcsel(&self) -> super::vals::Capteststartsrcsel {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Capteststartsrcsel::from_bits(val as u8)
    }
    #[doc = "Source selection for xo32k_captest_start_ao_set."]
    #[inline(always)]
    pub const fn set_capteststartsrcsel(&mut self, val: super::vals::Capteststartsrcsel) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Start test."]
    #[must_use]
    #[inline(always)]
    pub const fn capteststart(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Start test."]
    #[inline(always)]
    pub const fn set_capteststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable signal for cap test."]
    #[must_use]
    #[inline(always)]
    pub const fn captestenable(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable signal for cap test."]
    #[inline(always)]
    pub const fn set_captestenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Select the input for test."]
    #[must_use]
    #[inline(always)]
    pub const fn captestoscinsel(&self) -> super::vals::Captestoscinsel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Captestoscinsel::from_bits(val as u8)
    }
    #[doc = "Select the input for test."]
    #[inline(always)]
    pub const fn set_captestoscinsel(&mut self, val: super::vals::Captestoscinsel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Xtal32k {
    #[inline(always)]
    fn default() -> Xtal32k {
        Xtal32k(0)
    }
}
impl core::fmt::Debug for Xtal32k {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xtal32k")
            .field("iref", &self.iref())
            .field("test", &self.test())
            .field("ibias", &self.ibias())
            .field("ampl", &self.ampl())
            .field("capbankin", &self.capbankin())
            .field("capbankout", &self.capbankout())
            .field("capteststartsrcsel", &self.capteststartsrcsel())
            .field("capteststart", &self.capteststart())
            .field("captestenable", &self.captestenable())
            .field("captestoscinsel", &self.captestoscinsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xtal32k {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xtal32k {{ iref: {=u8:?}, test: {=bool:?}, ibias: {=u8:?}, ampl: {=u8:?}, capbankin: {=u8:?}, capbankout: {=u8:?}, capteststartsrcsel: {:?}, capteststart: {=bool:?}, captestenable: {=bool:?}, captestoscinsel: {:?} }}",
            self.iref(),
            self.test(),
            self.ibias(),
            self.ampl(),
            self.capbankin(),
            self.capbankout(),
            self.capteststartsrcsel(),
            self.capteststart(),
            self.captestenable(),
            self.captestoscinsel()
        )
    }
}
