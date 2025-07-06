#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc0(pub u32);
impl Misc0 {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> super::vals::Misc0ReftopSelfbiasoff {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Misc0ReftopSelfbiasoff::from_bits(val as u8)
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: super::vals::Misc0ReftopSelfbiasoff) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0ReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0ReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0ReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0StopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0StopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0StopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[must_use]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> super::vals::Misc0DisconHighSnvs {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Misc0DisconHighSnvs::from_bits(val as u8)
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: super::vals::Misc0DisconHighSnvs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0OscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0OscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0OscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> super::vals::Misc0ClkgateCtrl {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Misc0ClkgateCtrl::from_bits(val as u8)
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: super::vals::Misc0ClkgateCtrl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0ClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0ClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0ClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0RtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0RtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0RtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0 {
    #[inline(always)]
    fn default() -> Misc0 {
        Misc0(0)
    }
}
impl core::fmt::Debug for Misc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc0")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .field("reftop_vbgadj", &self.reftop_vbgadj())
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("stop_mode_config", &self.stop_mode_config())
            .field("discon_high_snvs", &self.discon_high_snvs())
            .field("osc_i", &self.osc_i())
            .field("osc_xtalok", &self.osc_xtalok())
            .field("osc_xtalok_en", &self.osc_xtalok_en())
            .field("clkgate_ctrl", &self.clkgate_ctrl())
            .field("clkgate_delay", &self.clkgate_delay())
            .field("rtc_xtal_source", &self.rtc_xtal_source())
            .field("xtal_24m_pwd", &self.xtal_24m_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0 {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?} }}",
            self.reftop_pwd(),
            self.reftop_selfbiasoff(),
            self.reftop_vbgadj(),
            self.reftop_vbgup(),
            self.stop_mode_config(),
            self.discon_high_snvs(),
            self.osc_i(),
            self.osc_xtalok(),
            self.osc_xtalok_en(),
            self.clkgate_ctrl(),
            self.clkgate_delay(),
            self.rtc_xtal_source(),
            self.xtal_24m_pwd()
        )
    }
}
#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc0Clr(pub u32);
impl Misc0Clr {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> super::vals::Misc0ClrReftopSelfbiasoff {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Misc0ClrReftopSelfbiasoff::from_bits(val as u8)
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: super::vals::Misc0ClrReftopSelfbiasoff) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0ClrReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0ClrReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0ClrReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0ClrStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0ClrStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0ClrStopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[must_use]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> super::vals::Misc0ClrDisconHighSnvs {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Misc0ClrDisconHighSnvs::from_bits(val as u8)
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: super::vals::Misc0ClrDisconHighSnvs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0ClrOscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0ClrOscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0ClrOscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> super::vals::Misc0ClrClkgateCtrl {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Misc0ClrClkgateCtrl::from_bits(val as u8)
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: super::vals::Misc0ClrClkgateCtrl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0ClrClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0ClrClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0ClrClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0ClrRtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0ClrRtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0ClrRtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0Clr {
    #[inline(always)]
    fn default() -> Misc0Clr {
        Misc0Clr(0)
    }
}
impl core::fmt::Debug for Misc0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc0Clr")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .field("reftop_vbgadj", &self.reftop_vbgadj())
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("stop_mode_config", &self.stop_mode_config())
            .field("discon_high_snvs", &self.discon_high_snvs())
            .field("osc_i", &self.osc_i())
            .field("osc_xtalok", &self.osc_xtalok())
            .field("osc_xtalok_en", &self.osc_xtalok_en())
            .field("clkgate_ctrl", &self.clkgate_ctrl())
            .field("clkgate_delay", &self.clkgate_delay())
            .field("rtc_xtal_source", &self.rtc_xtal_source())
            .field("xtal_24m_pwd", &self.xtal_24m_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0Clr {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?} }}",
            self.reftop_pwd(),
            self.reftop_selfbiasoff(),
            self.reftop_vbgadj(),
            self.reftop_vbgup(),
            self.stop_mode_config(),
            self.discon_high_snvs(),
            self.osc_i(),
            self.osc_xtalok(),
            self.osc_xtalok_en(),
            self.clkgate_ctrl(),
            self.clkgate_delay(),
            self.rtc_xtal_source(),
            self.xtal_24m_pwd()
        )
    }
}
#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc0Set(pub u32);
impl Misc0Set {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> super::vals::Misc0SetReftopSelfbiasoff {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Misc0SetReftopSelfbiasoff::from_bits(val as u8)
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: super::vals::Misc0SetReftopSelfbiasoff) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0SetReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0SetReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0SetReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0SetStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0SetStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0SetStopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[must_use]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> super::vals::Misc0SetDisconHighSnvs {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Misc0SetDisconHighSnvs::from_bits(val as u8)
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: super::vals::Misc0SetDisconHighSnvs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0SetOscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0SetOscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0SetOscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> super::vals::Misc0SetClkgateCtrl {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Misc0SetClkgateCtrl::from_bits(val as u8)
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: super::vals::Misc0SetClkgateCtrl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0SetClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0SetClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0SetClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0SetRtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0SetRtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0SetRtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0Set {
    #[inline(always)]
    fn default() -> Misc0Set {
        Misc0Set(0)
    }
}
impl core::fmt::Debug for Misc0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc0Set")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .field("reftop_vbgadj", &self.reftop_vbgadj())
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("stop_mode_config", &self.stop_mode_config())
            .field("discon_high_snvs", &self.discon_high_snvs())
            .field("osc_i", &self.osc_i())
            .field("osc_xtalok", &self.osc_xtalok())
            .field("osc_xtalok_en", &self.osc_xtalok_en())
            .field("clkgate_ctrl", &self.clkgate_ctrl())
            .field("clkgate_delay", &self.clkgate_delay())
            .field("rtc_xtal_source", &self.rtc_xtal_source())
            .field("xtal_24m_pwd", &self.xtal_24m_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0Set {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?} }}",
            self.reftop_pwd(),
            self.reftop_selfbiasoff(),
            self.reftop_vbgadj(),
            self.reftop_vbgup(),
            self.stop_mode_config(),
            self.discon_high_snvs(),
            self.osc_i(),
            self.osc_xtalok(),
            self.osc_xtalok_en(),
            self.clkgate_ctrl(),
            self.clkgate_delay(),
            self.rtc_xtal_source(),
            self.xtal_24m_pwd()
        )
    }
}
#[doc = "Miscellaneous Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc0Tog(pub u32);
impl Misc0Tog {
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub const fn set_reftop_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_selfbiasoff(&self) -> super::vals::Misc0TogReftopSelfbiasoff {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Misc0TogReftopSelfbiasoff::from_bits(val as u8)
    }
    #[doc = "Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub const fn set_reftop_selfbiasoff(&mut self, val: super::vals::Misc0TogReftopSelfbiasoff) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0TogReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0TogReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0TogReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable"]
    #[inline(always)]
    pub const fn set_reftop_vbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0TogStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0TogStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub const fn set_stop_mode_config(&mut self, val: super::vals::Misc0TogStopModeConfig) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[must_use]
    #[inline(always)]
    pub const fn discon_high_snvs(&self) -> super::vals::Misc0TogDisconHighSnvs {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Misc0TogDisconHighSnvs::from_bits(val as u8)
    }
    #[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub const fn set_discon_high_snvs(&mut self, val: super::vals::Misc0TogDisconHighSnvs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_i(&self) -> super::vals::Misc0TogOscI {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Misc0TogOscI::from_bits(val as u8)
    }
    #[doc = "This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub const fn set_osc_i(&mut self, val: super::vals::Misc0TogOscI) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub const fn set_osc_xtalok_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_ctrl(&self) -> super::vals::Misc0TogClkgateCtrl {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Misc0TogClkgateCtrl::from_bits(val as u8)
    }
    #[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_ctrl(&mut self, val: super::vals::Misc0TogClkgateCtrl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate_delay(&self) -> super::vals::Misc0TogClkgateDelay {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Misc0TogClkgateDelay::from_bits(val as u8)
    }
    #[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub const fn set_clkgate_delay(&mut self, val: super::vals::Misc0TogClkgateDelay) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0TogRtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0TogRtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock"]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0TogRtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true"]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Misc0Tog {
    #[inline(always)]
    fn default() -> Misc0Tog {
        Misc0Tog(0)
    }
}
impl core::fmt::Debug for Misc0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc0Tog")
            .field("reftop_pwd", &self.reftop_pwd())
            .field("reftop_selfbiasoff", &self.reftop_selfbiasoff())
            .field("reftop_vbgadj", &self.reftop_vbgadj())
            .field("reftop_vbgup", &self.reftop_vbgup())
            .field("stop_mode_config", &self.stop_mode_config())
            .field("discon_high_snvs", &self.discon_high_snvs())
            .field("osc_i", &self.osc_i())
            .field("osc_xtalok", &self.osc_xtalok())
            .field("osc_xtalok_en", &self.osc_xtalok_en())
            .field("clkgate_ctrl", &self.clkgate_ctrl())
            .field("clkgate_delay", &self.clkgate_delay())
            .field("rtc_xtal_source", &self.rtc_xtal_source())
            .field("xtal_24m_pwd", &self.xtal_24m_pwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0Tog {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?} }}",
            self.reftop_pwd(),
            self.reftop_selfbiasoff(),
            self.reftop_vbgadj(),
            self.reftop_vbgup(),
            self.stop_mode_config(),
            self.discon_high_snvs(),
            self.osc_i(),
            self.osc_xtalok(),
            self.osc_xtalok_en(),
            self.clkgate_ctrl(),
            self.clkgate_delay(),
            self.rtc_xtal_source(),
            self.xtal_24m_pwd()
        )
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc1(pub u32);
impl Misc1 {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1Lvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1Lvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1Lvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_oben(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_iben(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1 {
    #[inline(always)]
    fn default() -> Misc1 {
        Misc1(0)
    }
}
impl core::fmt::Debug for Misc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc1")
            .field("lvds1_clk_sel", &self.lvds1_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("pfd_480_autogate_en", &self.pfd_480_autogate_en())
            .field("pfd_528_autogate_en", &self.pfd_528_autogate_en())
            .field("irq_temppanic", &self.irq_temppanic())
            .field("irq_templow", &self.irq_templow())
            .field("irq_temphigh", &self.irq_temphigh())
            .field("irq_ana_bo", &self.irq_ana_bo())
            .field("irq_dig_bo", &self.irq_dig_bo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc1 {{ lvds1_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk1_iben(),
            self.pfd_480_autogate_en(),
            self.pfd_528_autogate_en(),
            self.irq_temppanic(),
            self.irq_templow(),
            self.irq_temphigh(),
            self.irq_ana_bo(),
            self.irq_dig_bo()
        )
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc1Clr(pub u32);
impl Misc1Clr {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1ClrLvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1ClrLvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1ClrLvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_oben(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_iben(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1Clr {
    #[inline(always)]
    fn default() -> Misc1Clr {
        Misc1Clr(0)
    }
}
impl core::fmt::Debug for Misc1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc1Clr")
            .field("lvds1_clk_sel", &self.lvds1_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("pfd_480_autogate_en", &self.pfd_480_autogate_en())
            .field("pfd_528_autogate_en", &self.pfd_528_autogate_en())
            .field("irq_temppanic", &self.irq_temppanic())
            .field("irq_templow", &self.irq_templow())
            .field("irq_temphigh", &self.irq_temphigh())
            .field("irq_ana_bo", &self.irq_ana_bo())
            .field("irq_dig_bo", &self.irq_dig_bo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc1Clr {{ lvds1_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk1_iben(),
            self.pfd_480_autogate_en(),
            self.pfd_528_autogate_en(),
            self.irq_temppanic(),
            self.irq_templow(),
            self.irq_temphigh(),
            self.irq_ana_bo(),
            self.irq_dig_bo()
        )
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc1Set(pub u32);
impl Misc1Set {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1SetLvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1SetLvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1SetLvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_oben(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_iben(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1Set {
    #[inline(always)]
    fn default() -> Misc1Set {
        Misc1Set(0)
    }
}
impl core::fmt::Debug for Misc1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc1Set")
            .field("lvds1_clk_sel", &self.lvds1_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("pfd_480_autogate_en", &self.pfd_480_autogate_en())
            .field("pfd_528_autogate_en", &self.pfd_528_autogate_en())
            .field("irq_temppanic", &self.irq_temppanic())
            .field("irq_templow", &self.irq_templow())
            .field("irq_temphigh", &self.irq_temphigh())
            .field("irq_ana_bo", &self.irq_ana_bo())
            .field("irq_dig_bo", &self.irq_dig_bo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc1Set {{ lvds1_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk1_iben(),
            self.pfd_480_autogate_en(),
            self.pfd_528_autogate_en(),
            self.irq_temppanic(),
            self.irq_templow(),
            self.irq_temphigh(),
            self.irq_ana_bo(),
            self.irq_dig_bo()
        )
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc1Tog(pub u32);
impl Misc1Tog {
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1TogLvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1TogLvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1TogLvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_oben(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk1_iben(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk1/1b"]
    #[inline(always)]
    pub const fn set_lvdsclk1_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_480_autogate_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_480_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_528_autogate_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub const fn set_pfd_528_autogate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temppanic(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub const fn set_irq_temppanic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_templow(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub const fn set_irq_templow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_temphigh(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub const fn set_irq_temphigh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_ana_bo(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_ana_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_dig_bo(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub const fn set_irq_dig_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc1Tog {
    #[inline(always)]
    fn default() -> Misc1Tog {
        Misc1Tog(0)
    }
}
impl core::fmt::Debug for Misc1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc1Tog")
            .field("lvds1_clk_sel", &self.lvds1_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("pfd_480_autogate_en", &self.pfd_480_autogate_en())
            .field("pfd_528_autogate_en", &self.pfd_528_autogate_en())
            .field("irq_temppanic", &self.irq_temppanic())
            .field("irq_templow", &self.irq_templow())
            .field("irq_temphigh", &self.irq_temphigh())
            .field("irq_ana_bo", &self.irq_ana_bo())
            .field("irq_dig_bo", &self.irq_dig_bo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc1Tog {{ lvds1_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk1_iben(),
            self.pfd_480_autogate_en(),
            self.pfd_528_autogate_en(),
            self.irq_temppanic(),
            self.irq_templow(),
            self.irq_temphigh(),
            self.irq_ana_bo(),
            self.irq_dig_bo()
        )
    }
}
#[doc = "Miscellaneous Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc2(pub u32);
impl Misc2 {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2Reg0BoOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2Reg0BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2Reg0BoOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_ok(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> super::vals::Misc2Pll3Disable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Misc2Pll3Disable::from_bits(val as u8)
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: super::vals::Misc2Pll3Disable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2Reg1BoOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2Reg1BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2Reg1BoOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_ok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> super::vals::Misc2AudioDivLsb {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Misc2AudioDivLsb::from_bits(val as u8)
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: super::vals::Misc2AudioDivLsb) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2Reg2BoOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2Reg2BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2Reg2BoOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> super::vals::Misc2AudioDivMsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Misc2AudioDivMsb::from_bits(val as u8)
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: super::vals::Misc2AudioDivMsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2Reg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2Reg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2Reg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2Reg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2Reg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2Reg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2Reg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2Reg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2Reg2StepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Post-divider for video"]
    #[must_use]
    #[inline(always)]
    pub const fn video_div(&self) -> super::vals::Misc2VideoDiv {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Misc2VideoDiv::from_bits(val as u8)
    }
    #[doc = "Post-divider for video"]
    #[inline(always)]
    pub const fn set_video_div(&mut self, val: super::vals::Misc2VideoDiv) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Misc2 {
    #[inline(always)]
    fn default() -> Misc2 {
        Misc2(0)
    }
}
impl core::fmt::Debug for Misc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc2")
            .field("reg0_bo_offset", &self.reg0_bo_offset())
            .field("reg0_bo_status", &self.reg0_bo_status())
            .field("reg0_enable_bo", &self.reg0_enable_bo())
            .field("reg0_ok", &self.reg0_ok())
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
            .field("reg1_ok", &self.reg1_ok())
            .field("audio_div_lsb", &self.audio_div_lsb())
            .field("reg2_bo_offset", &self.reg2_bo_offset())
            .field("reg2_bo_status", &self.reg2_bo_status())
            .field("reg2_enable_bo", &self.reg2_enable_bo())
            .field("reg2_ok", &self.reg2_ok())
            .field("audio_div_msb", &self.audio_div_msb())
            .field("reg0_step_time", &self.reg0_step_time())
            .field("reg1_step_time", &self.reg1_step_time())
            .field("reg2_step_time", &self.reg2_step_time())
            .field("video_div", &self.video_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc2 {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, reg0_ok: {=bool:?}, pll3_disable: {:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, reg1_ok: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.reg0_ok(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
            self.reg1_ok(),
            self.audio_div_lsb(),
            self.reg2_bo_offset(),
            self.reg2_bo_status(),
            self.reg2_enable_bo(),
            self.reg2_ok(),
            self.audio_div_msb(),
            self.reg0_step_time(),
            self.reg1_step_time(),
            self.reg2_step_time(),
            self.video_div()
        )
    }
}
#[doc = "Miscellaneous Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc2Clr(pub u32);
impl Misc2Clr {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2ClrReg0BoOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2ClrReg0BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2ClrReg0BoOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_ok(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> super::vals::Misc2ClrPll3Disable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Misc2ClrPll3Disable::from_bits(val as u8)
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: super::vals::Misc2ClrPll3Disable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2ClrReg1BoOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2ClrReg1BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2ClrReg1BoOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_ok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> super::vals::Misc2ClrAudioDivLsb {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Misc2ClrAudioDivLsb::from_bits(val as u8)
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: super::vals::Misc2ClrAudioDivLsb) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2ClrReg2BoOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2ClrReg2BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2ClrReg2BoOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> super::vals::Misc2ClrAudioDivMsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Misc2ClrAudioDivMsb::from_bits(val as u8)
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: super::vals::Misc2ClrAudioDivMsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2ClrReg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2ClrReg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2ClrReg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2ClrReg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2ClrReg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2ClrReg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2ClrReg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2ClrReg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2ClrReg2StepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Post-divider for video"]
    #[must_use]
    #[inline(always)]
    pub const fn video_div(&self) -> super::vals::Misc2ClrVideoDiv {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Misc2ClrVideoDiv::from_bits(val as u8)
    }
    #[doc = "Post-divider for video"]
    #[inline(always)]
    pub const fn set_video_div(&mut self, val: super::vals::Misc2ClrVideoDiv) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Misc2Clr {
    #[inline(always)]
    fn default() -> Misc2Clr {
        Misc2Clr(0)
    }
}
impl core::fmt::Debug for Misc2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc2Clr")
            .field("reg0_bo_offset", &self.reg0_bo_offset())
            .field("reg0_bo_status", &self.reg0_bo_status())
            .field("reg0_enable_bo", &self.reg0_enable_bo())
            .field("reg0_ok", &self.reg0_ok())
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
            .field("reg1_ok", &self.reg1_ok())
            .field("audio_div_lsb", &self.audio_div_lsb())
            .field("reg2_bo_offset", &self.reg2_bo_offset())
            .field("reg2_bo_status", &self.reg2_bo_status())
            .field("reg2_enable_bo", &self.reg2_enable_bo())
            .field("reg2_ok", &self.reg2_ok())
            .field("audio_div_msb", &self.audio_div_msb())
            .field("reg0_step_time", &self.reg0_step_time())
            .field("reg1_step_time", &self.reg1_step_time())
            .field("reg2_step_time", &self.reg2_step_time())
            .field("video_div", &self.video_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc2Clr {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, reg0_ok: {=bool:?}, pll3_disable: {:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, reg1_ok: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.reg0_ok(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
            self.reg1_ok(),
            self.audio_div_lsb(),
            self.reg2_bo_offset(),
            self.reg2_bo_status(),
            self.reg2_enable_bo(),
            self.reg2_ok(),
            self.audio_div_msb(),
            self.reg0_step_time(),
            self.reg1_step_time(),
            self.reg2_step_time(),
            self.video_div()
        )
    }
}
#[doc = "Miscellaneous Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc2Set(pub u32);
impl Misc2Set {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2SetReg0BoOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2SetReg0BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2SetReg0BoOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_ok(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> super::vals::Misc2SetPll3Disable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Misc2SetPll3Disable::from_bits(val as u8)
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: super::vals::Misc2SetPll3Disable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2SetReg1BoOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2SetReg1BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2SetReg1BoOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_ok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> super::vals::Misc2SetAudioDivLsb {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Misc2SetAudioDivLsb::from_bits(val as u8)
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: super::vals::Misc2SetAudioDivLsb) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2SetReg2BoOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2SetReg2BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2SetReg2BoOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> super::vals::Misc2SetAudioDivMsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Misc2SetAudioDivMsb::from_bits(val as u8)
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: super::vals::Misc2SetAudioDivMsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2SetReg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2SetReg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2SetReg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2SetReg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2SetReg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2SetReg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2SetReg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2SetReg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2SetReg2StepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Post-divider for video"]
    #[must_use]
    #[inline(always)]
    pub const fn video_div(&self) -> super::vals::Misc2SetVideoDiv {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Misc2SetVideoDiv::from_bits(val as u8)
    }
    #[doc = "Post-divider for video"]
    #[inline(always)]
    pub const fn set_video_div(&mut self, val: super::vals::Misc2SetVideoDiv) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Misc2Set {
    #[inline(always)]
    fn default() -> Misc2Set {
        Misc2Set(0)
    }
}
impl core::fmt::Debug for Misc2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc2Set")
            .field("reg0_bo_offset", &self.reg0_bo_offset())
            .field("reg0_bo_status", &self.reg0_bo_status())
            .field("reg0_enable_bo", &self.reg0_enable_bo())
            .field("reg0_ok", &self.reg0_ok())
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
            .field("reg1_ok", &self.reg1_ok())
            .field("audio_div_lsb", &self.audio_div_lsb())
            .field("reg2_bo_offset", &self.reg2_bo_offset())
            .field("reg2_bo_status", &self.reg2_bo_status())
            .field("reg2_enable_bo", &self.reg2_enable_bo())
            .field("reg2_ok", &self.reg2_ok())
            .field("audio_div_msb", &self.audio_div_msb())
            .field("reg0_step_time", &self.reg0_step_time())
            .field("reg1_step_time", &self.reg1_step_time())
            .field("reg2_step_time", &self.reg2_step_time())
            .field("video_div", &self.video_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc2Set {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, reg0_ok: {=bool:?}, pll3_disable: {:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, reg1_ok: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.reg0_ok(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
            self.reg1_ok(),
            self.audio_div_lsb(),
            self.reg2_bo_offset(),
            self.reg2_bo_status(),
            self.reg2_enable_bo(),
            self.reg2_ok(),
            self.audio_div_msb(),
            self.reg0_step_time(),
            self.reg1_step_time(),
            self.reg2_step_time(),
            self.video_div()
        )
    }
}
#[doc = "Miscellaneous Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc2Tog(pub u32);
impl Misc2Tog {
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_offset(&self) -> super::vals::Misc2TogReg0BoOffset {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Misc2TogReg0BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub const fn set_reg0_bo_offset(&mut self, val: super::vals::Misc2TogReg0BoOffset) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_ok(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arm supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> super::vals::Misc2TogPll3Disable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Misc2TogPll3Disable::from_bits(val as u8)
    }
    #[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: super::vals::Misc2TogPll3Disable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_offset(&self) -> super::vals::Misc2TogReg1BoOffset {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Misc2TogReg1BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg1_bo_offset(&mut self, val: super::vals::Misc2TogReg1BoOffset) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_ok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_lsb(&self) -> super::vals::Misc2TogAudioDivLsb {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Misc2TogAudioDivLsb::from_bits(val as u8)
    }
    #[doc = "LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_lsb(&mut self, val: super::vals::Misc2TogAudioDivLsb) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_offset(&self) -> super::vals::Misc2TogReg2BoOffset {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Misc2TogReg2BoOffset::from_bits(val as u8)
    }
    #[doc = "This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub const fn set_reg2_bo_offset(&mut self, val: super::vals::Misc2TogReg2BoOffset) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_ok(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub const fn set_reg2_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn audio_div_msb(&self) -> super::vals::Misc2TogAudioDivMsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Misc2TogAudioDivMsb::from_bits(val as u8)
    }
    #[doc = "MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub const fn set_audio_div_msb(&mut self, val: super::vals::Misc2TogAudioDivMsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2TogReg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2TogReg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2TogReg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2TogReg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2TogReg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2TogReg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2TogReg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2TogReg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub const fn set_reg2_step_time(&mut self, val: super::vals::Misc2TogReg2StepTime) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Post-divider for video"]
    #[must_use]
    #[inline(always)]
    pub const fn video_div(&self) -> super::vals::Misc2TogVideoDiv {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Misc2TogVideoDiv::from_bits(val as u8)
    }
    #[doc = "Post-divider for video"]
    #[inline(always)]
    pub const fn set_video_div(&mut self, val: super::vals::Misc2TogVideoDiv) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Misc2Tog {
    #[inline(always)]
    fn default() -> Misc2Tog {
        Misc2Tog(0)
    }
}
impl core::fmt::Debug for Misc2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc2Tog")
            .field("reg0_bo_offset", &self.reg0_bo_offset())
            .field("reg0_bo_status", &self.reg0_bo_status())
            .field("reg0_enable_bo", &self.reg0_enable_bo())
            .field("reg0_ok", &self.reg0_ok())
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
            .field("reg1_ok", &self.reg1_ok())
            .field("audio_div_lsb", &self.audio_div_lsb())
            .field("reg2_bo_offset", &self.reg2_bo_offset())
            .field("reg2_bo_status", &self.reg2_bo_status())
            .field("reg2_enable_bo", &self.reg2_enable_bo())
            .field("reg2_ok", &self.reg2_ok())
            .field("audio_div_msb", &self.audio_div_msb())
            .field("reg0_step_time", &self.reg0_step_time())
            .field("reg1_step_time", &self.reg1_step_time())
            .field("reg2_step_time", &self.reg2_step_time())
            .field("video_div", &self.video_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc2Tog {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, reg0_ok: {=bool:?}, pll3_disable: {:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, reg1_ok: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.reg0_ok(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
            self.reg1_ok(),
            self.audio_div_lsb(),
            self.reg2_bo_offset(),
            self.reg2_bo_status(),
            self.reg2_enable_bo(),
            self.reg2_ok(),
            self.audio_div_msb(),
            self.reg0_step_time(),
            self.reg1_step_time(),
            self.reg2_step_time(),
            self.video_div()
        )
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd480(pub u32);
impl Pfd480 {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd480 {
    #[inline(always)]
    fn default() -> Pfd480 {
        Pfd480(0)
    }
}
impl core::fmt::Debug for Pfd480 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd480")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd480 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd480 {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd480Clr(pub u32);
impl Pfd480Clr {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd480Clr {
    #[inline(always)]
    fn default() -> Pfd480Clr {
        Pfd480Clr(0)
    }
}
impl core::fmt::Debug for Pfd480Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd480Clr")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd480Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd480Clr {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd480Set(pub u32);
impl Pfd480Set {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd480Set {
    #[inline(always)]
    fn default() -> Pfd480Set {
        Pfd480Set(0)
    }
}
impl core::fmt::Debug for Pfd480Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd480Set")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd480Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd480Set {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd480Tog(pub u32);
impl Pfd480Tog {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd480Tog {
    #[inline(always)]
    fn default() -> Pfd480Tog {
        Pfd480Tog(0)
    }
}
impl core::fmt::Debug for Pfd480Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd480Tog")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd480Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd480Tog {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd528(pub u32);
impl Pfd528 {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd528 {
    #[inline(always)]
    fn default() -> Pfd528 {
        Pfd528(0)
    }
}
impl core::fmt::Debug for Pfd528 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd528")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd528 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd528 {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd528Clr(pub u32);
impl Pfd528Clr {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd528Clr {
    #[inline(always)]
    fn default() -> Pfd528Clr {
        Pfd528Clr(0)
    }
}
impl core::fmt::Debug for Pfd528Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd528Clr")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd528Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd528Clr {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd528Set(pub u32);
impl Pfd528Set {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd528Set {
    #[inline(always)]
    fn default() -> Pfd528Set {
        Pfd528Set(0)
    }
}
impl core::fmt::Debug for Pfd528Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd528Set")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd528Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd528Set {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfd528Tog(pub u32);
impl Pfd528Tog {
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd1_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_stable(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd1_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_frac(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd2_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_stable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd2_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This field controls the fractional divide value"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_frac(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "This field controls the fractional divide value"]
    #[inline(always)]
    pub const fn set_pfd3_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_stable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code"]
    #[inline(always)]
    pub const fn set_pfd3_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "IO Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IO Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfd528Tog {
    #[inline(always)]
    fn default() -> Pfd528Tog {
        Pfd528Tog(0)
    }
}
impl core::fmt::Debug for Pfd528Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfd528Tog")
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1_frac", &self.pfd1_frac())
            .field("pfd1_stable", &self.pfd1_stable())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2_frac", &self.pfd2_frac())
            .field("pfd2_stable", &self.pfd2_stable())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3_frac", &self.pfd3_frac())
            .field("pfd3_stable", &self.pfd3_stable())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfd528Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfd528Tog {{ pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1_frac: {=u8:?}, pfd1_stable: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2_frac: {=u8:?}, pfd2_stable: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3_frac: {=u8:?}, pfd3_stable: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0_frac(),
            self.pfd0_stable(),
            self.pfd0_clkgate(),
            self.pfd1_frac(),
            self.pfd1_stable(),
            self.pfd1_clkgate(),
            self.pfd2_frac(),
            self.pfd2_stable(),
            self.pfd2_clkgate(),
            self.pfd3_frac(),
            self.pfd3_stable(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "Analog ARM PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllArm(pub u32);
impl PllArm {
    #[doc = "This field controls the PLL loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllArmBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllArmBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source"]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllArmBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_pll_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllArm {
    #[inline(always)]
    fn default() -> PllArm {
        PllArm(0)
    }
}
impl core::fmt::Debug for PllArm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllArm")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("pll_sel", &self.pll_sel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllArm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllArm {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, pll_sel: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.pll_sel(),
            self.lock()
        )
    }
}
#[doc = "Analog ARM PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllArmClr(pub u32);
impl PllArmClr {
    #[doc = "This field controls the PLL loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllArmClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllArmClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source"]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllArmClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_pll_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllArmClr {
    #[inline(always)]
    fn default() -> PllArmClr {
        PllArmClr(0)
    }
}
impl core::fmt::Debug for PllArmClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllArmClr")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("pll_sel", &self.pll_sel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllArmClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllArmClr {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, pll_sel: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.pll_sel(),
            self.lock()
        )
    }
}
#[doc = "Analog ARM PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllArmSet(pub u32);
impl PllArmSet {
    #[doc = "This field controls the PLL loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllArmSetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllArmSetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source"]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllArmSetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_pll_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllArmSet {
    #[inline(always)]
    fn default() -> PllArmSet {
        PllArmSet(0)
    }
}
impl core::fmt::Debug for PllArmSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllArmSet")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("pll_sel", &self.pll_sel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllArmSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllArmSet {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, pll_sel: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.pll_sel(),
            self.lock()
        )
    }
}
#[doc = "Analog ARM PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllArmTog(pub u32);
impl PllArmTog {
    #[doc = "This field controls the PLL loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllArmTogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllArmTogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source"]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllArmTogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_pll_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllArmTog {
    #[inline(always)]
    fn default() -> PllArmTog {
        PllArmTog(0)
    }
}
impl core::fmt::Debug for PllArmTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllArmTog")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("pll_sel", &self.pll_sel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllArmTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllArmTog {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, pll_sel: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.pll_sel(),
            self.lock()
        )
    }
}
#[doc = "Analog Audio PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudio(pub u32);
impl PllAudio {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllAudioBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllAudioBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllAudioBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllAudioPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllAudioPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllAudioPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllAudio {
    #[inline(always)]
    fn default() -> PllAudio {
        PllAudio(0)
    }
}
impl core::fmt::Debug for PllAudio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudio")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllAudio {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Analog Audio PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudioClr(pub u32);
impl PllAudioClr {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllAudioClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllAudioClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllAudioClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllAudioClrPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllAudioClrPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllAudioClrPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllAudioClr {
    #[inline(always)]
    fn default() -> PllAudioClr {
        PllAudioClr(0)
    }
}
impl core::fmt::Debug for PllAudioClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudioClr")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudioClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllAudioClr {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudioDenom(pub u32);
impl PllAudioDenom {
    #[doc = "30 bit denominator of fractional loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "30 bit denominator of fractional loop divider."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for PllAudioDenom {
    #[inline(always)]
    fn default() -> PllAudioDenom {
        PllAudioDenom(0)
    }
}
impl core::fmt::Debug for PllAudioDenom {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudioDenom")
            .field("b", &self.b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudioDenom {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PllAudioDenom {{ b: {=u32:?} }}", self.b())
    }
}
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudioNum(pub u32);
impl PllAudioNum {
    #[doc = "30 bit numerator of fractional loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "30 bit numerator of fractional loop divider."]
    #[inline(always)]
    pub const fn set_a(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for PllAudioNum {
    #[inline(always)]
    fn default() -> PllAudioNum {
        PllAudioNum(0)
    }
}
impl core::fmt::Debug for PllAudioNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudioNum").field("a", &self.a()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudioNum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PllAudioNum {{ a: {=u32:?} }}", self.a())
    }
}
#[doc = "Analog Audio PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudioSet(pub u32);
impl PllAudioSet {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllAudioSetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllAudioSetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllAudioSetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllAudioSetPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllAudioSetPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllAudioSetPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllAudioSet {
    #[inline(always)]
    fn default() -> PllAudioSet {
        PllAudioSet(0)
    }
}
impl core::fmt::Debug for PllAudioSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudioSet")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudioSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllAudioSet {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Analog Audio PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllAudioTog(pub u32);
impl PllAudioTog {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllAudioTogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllAudioTogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllAudioTogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllAudioTogPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllAudioTogPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllAudioTogPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllAudioTog {
    #[inline(always)]
    fn default() -> PllAudioTog {
        PllAudioTog(0)
    }
}
impl core::fmt::Debug for PllAudioTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllAudioTog")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllAudioTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllAudioTog {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Analog ENET PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllEnet(pub u32);
impl PllEnet {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_div_select(&self) -> super::vals::PllEnetEnet2DivSelect {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PllEnetEnet2DivSelect::from_bits(val as u8)
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[inline(always)]
    pub const fn set_enet2_div_select(&mut self, val: super::vals::PllEnetEnet2DivSelect) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllEnetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllEnetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllEnetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_ref_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[inline(always)]
    pub const fn set_enet2_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_25m_ref_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[inline(always)]
    pub const fn set_enet_25m_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllEnet {
    #[inline(always)]
    fn default() -> PllEnet {
        PllEnet(0)
    }
}
impl core::fmt::Debug for PllEnet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllEnet")
            .field("div_select", &self.div_select())
            .field("enet2_div_select", &self.enet2_div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("enet2_ref_en", &self.enet2_ref_en())
            .field("enet_25m_ref_en", &self.enet_25m_ref_en())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllEnet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllEnet {{ div_select: {=u8:?}, enet2_div_select: {:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, enet2_ref_en: {=bool:?}, enet_25m_ref_en: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.enet2_div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.enet2_ref_en(),
            self.enet_25m_ref_en(),
            self.lock()
        )
    }
}
#[doc = "Analog ENET PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllEnetClr(pub u32);
impl PllEnetClr {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_div_select(&self) -> super::vals::PllEnetClrEnet2DivSelect {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PllEnetClrEnet2DivSelect::from_bits(val as u8)
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[inline(always)]
    pub const fn set_enet2_div_select(&mut self, val: super::vals::PllEnetClrEnet2DivSelect) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllEnetClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllEnetClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllEnetClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_ref_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[inline(always)]
    pub const fn set_enet2_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_25m_ref_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[inline(always)]
    pub const fn set_enet_25m_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllEnetClr {
    #[inline(always)]
    fn default() -> PllEnetClr {
        PllEnetClr(0)
    }
}
impl core::fmt::Debug for PllEnetClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllEnetClr")
            .field("div_select", &self.div_select())
            .field("enet2_div_select", &self.enet2_div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("enet2_ref_en", &self.enet2_ref_en())
            .field("enet_25m_ref_en", &self.enet_25m_ref_en())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllEnetClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllEnetClr {{ div_select: {=u8:?}, enet2_div_select: {:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, enet2_ref_en: {=bool:?}, enet_25m_ref_en: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.enet2_div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.enet2_ref_en(),
            self.enet_25m_ref_en(),
            self.lock()
        )
    }
}
#[doc = "Analog ENET PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllEnetSet(pub u32);
impl PllEnetSet {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_div_select(&self) -> super::vals::PllEnetSetEnet2DivSelect {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PllEnetSetEnet2DivSelect::from_bits(val as u8)
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[inline(always)]
    pub const fn set_enet2_div_select(&mut self, val: super::vals::PllEnetSetEnet2DivSelect) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllEnetSetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllEnetSetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllEnetSetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_ref_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[inline(always)]
    pub const fn set_enet2_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_25m_ref_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[inline(always)]
    pub const fn set_enet_25m_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllEnetSet {
    #[inline(always)]
    fn default() -> PllEnetSet {
        PllEnetSet(0)
    }
}
impl core::fmt::Debug for PllEnetSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllEnetSet")
            .field("div_select", &self.div_select())
            .field("enet2_div_select", &self.enet2_div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("enet2_ref_en", &self.enet2_ref_en())
            .field("enet_25m_ref_en", &self.enet_25m_ref_en())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllEnetSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllEnetSet {{ div_select: {=u8:?}, enet2_div_select: {:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, enet2_ref_en: {=bool:?}, enet_25m_ref_en: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.enet2_div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.enet2_ref_en(),
            self.enet_25m_ref_en(),
            self.lock()
        )
    }
}
#[doc = "Analog ENET PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllEnetTog(pub u32);
impl PllEnetTog {
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Controls the frequency of the ethernet reference clock"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_div_select(&self) -> super::vals::PllEnetTogEnet2DivSelect {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PllEnetTogEnet2DivSelect::from_bits(val as u8)
    }
    #[doc = "Controls the frequency of the ENET2 reference clock."]
    #[inline(always)]
    pub const fn set_enet2_div_select(&mut self, val: super::vals::PllEnetTogEnet2DivSelect) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET reference clock."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllEnetTogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllEnetTogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllEnetTogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_ref_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing the ENET2 reference clock"]
    #[inline(always)]
    pub const fn set_enet2_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_25m_ref_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL providing ENET 25 MHz reference clock"]
    #[inline(always)]
    pub const fn set_enet_25m_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllEnetTog {
    #[inline(always)]
    fn default() -> PllEnetTog {
        PllEnetTog(0)
    }
}
impl core::fmt::Debug for PllEnetTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllEnetTog")
            .field("div_select", &self.div_select())
            .field("enet2_div_select", &self.enet2_div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("enet2_ref_en", &self.enet2_ref_en())
            .field("enet_25m_ref_en", &self.enet_25m_ref_en())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllEnetTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllEnetTog {{ div_select: {=u8:?}, enet2_div_select: {:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, enet2_ref_en: {=bool:?}, enet_25m_ref_en: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.enet2_div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.enet2_ref_en(),
            self.enet_25m_ref_en(),
            self.lock()
        )
    }
}
#[doc = "Analog System PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSys(pub u32);
impl PllSys {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllSysBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllSysBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllSysBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSys {
    #[inline(always)]
    fn default() -> PllSys {
        PllSys(0)
    }
}
impl core::fmt::Debug for PllSys {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSys")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSys {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSys {{ div_select: {=bool:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog System PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysClr(pub u32);
impl PllSysClr {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllSysClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllSysClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllSysClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSysClr {
    #[inline(always)]
    fn default() -> PllSysClr {
        PllSysClr(0)
    }
}
impl core::fmt::Debug for PllSysClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSysClr")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSysClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSysClr {{ div_select: {=bool:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysDenom(pub u32);
impl PllSysDenom {
    #[doc = "30 bit denominator (B) of fractional loop divider (unsigned integer)."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "30 bit denominator (B) of fractional loop divider (unsigned integer)."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for PllSysDenom {
    #[inline(always)]
    fn default() -> PllSysDenom {
        PllSysDenom(0)
    }
}
impl core::fmt::Debug for PllSysDenom {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSysDenom").field("b", &self.b()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSysDenom {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PllSysDenom {{ b: {=u32:?} }}", self.b())
    }
}
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysNum(pub u32);
impl PllSysNum {
    #[doc = "30 bit numerator (A) of fractional loop divider (signed integer)."]
    #[must_use]
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "30 bit numerator (A) of fractional loop divider (signed integer)."]
    #[inline(always)]
    pub const fn set_a(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for PllSysNum {
    #[inline(always)]
    fn default() -> PllSysNum {
        PllSysNum(0)
    }
}
impl core::fmt::Debug for PllSysNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSysNum").field("a", &self.a()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSysNum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PllSysNum {{ a: {=u32:?} }}", self.a())
    }
}
#[doc = "Analog System PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysSet(pub u32);
impl PllSysSet {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllSysSetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllSysSetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllSysSetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSysSet {
    #[inline(always)]
    fn default() -> PllSysSet {
        PllSysSet(0)
    }
}
impl core::fmt::Debug for PllSysSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSysSet")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSysSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSysSet {{ div_select: {=bool:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "528MHz System PLL Spread Spectrum Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysSs(pub u32);
impl PllSysSs {
    #[doc = "Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable bit"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PllSysSs {
    #[inline(always)]
    fn default() -> PllSysSs {
        PllSysSs(0)
    }
}
impl core::fmt::Debug for PllSysSs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSysSs")
            .field("step", &self.step())
            .field("enable", &self.enable())
            .field("stop", &self.stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSysSs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSysSs {{ step: {=u16:?}, enable: {:?}, stop: {=u16:?} }}",
            self.step(),
            self.enable(),
            self.stop()
        )
    }
}
#[doc = "Analog System PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysTog(pub u32);
impl PllSysTog {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllSysTogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllSysTogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllSysTogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSysTog {
    #[inline(always)]
    fn default() -> PllSysTog {
        PllSysTog(0)
    }
}
impl core::fmt::Debug for PllSysTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSysTog")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSysTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSysTog {{ div_select: {=bool:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb1(pub u32);
impl PllUsb1 {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> super::vals::PllUsb1EnUsbClks {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PllUsb1EnUsbClks::from_bits(val as u8)
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: super::vals::PllUsb1EnUsbClks) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb1BypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb1BypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb1BypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb1 {
    #[inline(always)]
    fn default() -> PllUsb1 {
        PllUsb1(0)
    }
}
impl core::fmt::Debug for PllUsb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb1")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb1 {{ div_select: {=bool:?}, en_usb_clks: {:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb1Clr(pub u32);
impl PllUsb1Clr {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> super::vals::PllUsb1ClrEnUsbClks {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PllUsb1ClrEnUsbClks::from_bits(val as u8)
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: super::vals::PllUsb1ClrEnUsbClks) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb1ClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb1ClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb1ClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb1Clr {
    #[inline(always)]
    fn default() -> PllUsb1Clr {
        PllUsb1Clr(0)
    }
}
impl core::fmt::Debug for PllUsb1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb1Clr")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb1Clr {{ div_select: {=bool:?}, en_usb_clks: {:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb1Set(pub u32);
impl PllUsb1Set {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> super::vals::PllUsb1SetEnUsbClks {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PllUsb1SetEnUsbClks::from_bits(val as u8)
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: super::vals::PllUsb1SetEnUsbClks) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb1SetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb1SetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb1SetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb1Set {
    #[inline(always)]
    fn default() -> PllUsb1Set {
        PllUsb1Set(0)
    }
}
impl core::fmt::Debug for PllUsb1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb1Set")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb1Set {{ div_select: {=bool:?}, en_usb_clks: {:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb1Tog(pub u32);
impl PllUsb1Tog {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> super::vals::PllUsb1TogEnUsbClks {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PllUsb1TogEnUsbClks::from_bits(val as u8)
    }
    #[doc = "Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: super::vals::PllUsb1TogEnUsbClks) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb1TogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb1TogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb1TogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb1Tog {
    #[inline(always)]
    fn default() -> PllUsb1Tog {
        PllUsb1Tog(0)
    }
}
impl core::fmt::Debug for PllUsb1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb1Tog")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb1Tog {{ div_select: {=bool:?}, en_usb_clks: {:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb2(pub u32);
impl PllUsb2 {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb2BypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb2BypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb2BypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb2 {
    #[inline(always)]
    fn default() -> PllUsb2 {
        PllUsb2(0)
    }
}
impl core::fmt::Debug for PllUsb2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb2")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb2 {{ div_select: {=bool:?}, en_usb_clks: {=bool:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb2Clr(pub u32);
impl PllUsb2Clr {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb2ClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb2ClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb2ClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb2Clr {
    #[inline(always)]
    fn default() -> PllUsb2Clr {
        PllUsb2Clr(0)
    }
}
impl core::fmt::Debug for PllUsb2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb2Clr")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb2Clr {{ div_select: {=bool:?}, en_usb_clks: {=bool:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb2Set(pub u32);
impl PllUsb2Set {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb2SetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb2SetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb2SetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb2Set {
    #[inline(always)]
    fn default() -> PllUsb2Set {
        PllUsb2Set(0)
    }
}
impl core::fmt::Debug for PllUsb2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb2Set")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb2Set {{ div_select: {=bool:?}, en_usb_clks: {=bool:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb2Tog(pub u32);
impl PllUsb2Tog {
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[must_use]
    #[inline(always)]
    pub const fn en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "0: 8-phase PLL outputs for USBPHY1 are powered down"]
    #[inline(always)]
    pub const fn set_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[must_use]
    #[inline(always)]
    pub const fn power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers up the PLL. This bit will be set automatically when USBPHY1 remote wakeup event happens."]
    #[inline(always)]
    pub const fn set_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable the PLL clock output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PLL clock output."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllUsb2TogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllUsb2TogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllUsb2TogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllUsb2Tog {
    #[inline(always)]
    fn default() -> PllUsb2Tog {
        PllUsb2Tog(0)
    }
}
impl core::fmt::Debug for PllUsb2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllUsb2Tog")
            .field("div_select", &self.div_select())
            .field("en_usb_clks", &self.en_usb_clks())
            .field("power", &self.power())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllUsb2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllUsb2Tog {{ div_select: {=bool:?}, en_usb_clks: {=bool:?}, power: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.en_usb_clks(),
            self.power(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.lock()
        )
    }
}
#[doc = "Analog Video PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllVideo(pub u32);
impl PllVideo {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllVideoBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllVideoBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllVideoBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllVideoPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllVideoPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllVideoPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllVideo {
    #[inline(always)]
    fn default() -> PllVideo {
        PllVideo(0)
    }
}
impl core::fmt::Debug for PllVideo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllVideo")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllVideo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllVideo {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Analog Video PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllVideoClr(pub u32);
impl PllVideoClr {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllVideoClrBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllVideoClrBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllVideoClrBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllVideoClrPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllVideoClrPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllVideoClrPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllVideoClr {
    #[inline(always)]
    fn default() -> PllVideoClr {
        PllVideoClr(0)
    }
}
impl core::fmt::Debug for PllVideoClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllVideoClr")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllVideoClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllVideoClr {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllVideoDenom(pub u32);
impl PllVideoDenom {
    #[doc = "30 bit denominator of fractional loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "30 bit denominator of fractional loop divider."]
    #[inline(always)]
    pub const fn set_b(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for PllVideoDenom {
    #[inline(always)]
    fn default() -> PllVideoDenom {
        PllVideoDenom(0)
    }
}
impl core::fmt::Debug for PllVideoDenom {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllVideoDenom")
            .field("b", &self.b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllVideoDenom {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PllVideoDenom {{ b: {=u32:?} }}", self.b())
    }
}
#[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllVideoNum(pub u32);
impl PllVideoNum {
    #[doc = "30 bit numerator of fractional loop divider (signed number)"]
    #[must_use]
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "30 bit numerator of fractional loop divider (signed number)"]
    #[inline(always)]
    pub const fn set_a(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for PllVideoNum {
    #[inline(always)]
    fn default() -> PllVideoNum {
        PllVideoNum(0)
    }
}
impl core::fmt::Debug for PllVideoNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllVideoNum").field("a", &self.a()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllVideoNum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PllVideoNum {{ a: {=u32:?} }}", self.a())
    }
}
#[doc = "Analog Video PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllVideoSet(pub u32);
impl PllVideoSet {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllVideoSetBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllVideoSetBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllVideoSetBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllVideoSetPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllVideoSetPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllVideoSetPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllVideoSet {
    #[inline(always)]
    fn default() -> PllVideoSet {
        PllVideoSet(0)
    }
}
impl core::fmt::Debug for PllVideoSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllVideoSet")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllVideoSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllVideoSet {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
#[doc = "Analog Video PLL control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllVideoTog(pub u32);
impl PllVideoTog {
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Powers down the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn powerdown(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Powers down the PLL."]
    #[inline(always)]
    pub const fn set_powerdown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable PLL output"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL output"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Determines the bypass source."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_clk_src(&self) -> super::vals::PllVideoTogBypassClkSrc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PllVideoTogBypassClkSrc::from_bits(val as u8)
    }
    #[doc = "Determines the bypass source."]
    #[inline(always)]
    pub const fn set_bypass_clk_src(&mut self, val: super::vals::PllVideoTogBypassClkSrc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Bypass the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the PLL."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_select(&self) -> super::vals::PllVideoTogPostDivSelect {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PllVideoTogPostDivSelect::from_bits(val as u8)
    }
    #[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub const fn set_post_div_select(&mut self, val: super::vals::PllVideoTogPostDivSelect) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 - PLL is currently locked; 0 - PLL is not currently locked."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllVideoTog {
    #[inline(always)]
    fn default() -> PllVideoTog {
        PllVideoTog(0)
    }
}
impl core::fmt::Debug for PllVideoTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllVideoTog")
            .field("div_select", &self.div_select())
            .field("powerdown", &self.powerdown())
            .field("enable", &self.enable())
            .field("bypass_clk_src", &self.bypass_clk_src())
            .field("bypass", &self.bypass())
            .field("post_div_select", &self.post_div_select())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllVideoTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllVideoTog {{ div_select: {=u8:?}, powerdown: {=bool:?}, enable: {=bool:?}, bypass_clk_src: {:?}, bypass: {=bool:?}, post_div_select: {:?}, lock: {=bool:?} }}",
            self.div_select(),
            self.powerdown(),
            self.enable(),
            self.bypass_clk_src(),
            self.bypass(),
            self.post_div_select(),
            self.lock()
        )
    }
}
