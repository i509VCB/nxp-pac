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
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0ReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0ReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0ReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0RtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0RtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0RtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0VidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0VidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: super::vals::Misc0VidPllPrediv) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
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
            .field("vid_pll_prediv", &self.vid_pll_prediv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0 {{ reftop_pwd: {=bool:?}, reftop_pwdvbgup: {=bool:?}, reftop_lowpower: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
            self.reftop_pwd(),
            self.reftop_pwdvbgup(),
            self.reftop_lowpower(),
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
            self.xtal_24m_pwd(),
            self.vid_pll_prediv()
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
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0ClrReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0ClrReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0ClrReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0ClrRtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0ClrRtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0ClrRtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0ClrVidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0ClrVidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: super::vals::Misc0ClrVidPllPrediv) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
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
            .field("vid_pll_prediv", &self.vid_pll_prediv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0Clr {{ reftop_pwd: {=bool:?}, reftop_pwdvbgup: {=bool:?}, reftop_lowpower: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
            self.reftop_pwd(),
            self.reftop_pwdvbgup(),
            self.reftop_lowpower(),
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
            self.xtal_24m_pwd(),
            self.vid_pll_prediv()
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
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0SetReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0SetReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0SetReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0SetRtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0SetRtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0SetRtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0SetVidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0SetVidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: super::vals::Misc0SetVidPllPrediv) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
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
            .field("vid_pll_prediv", &self.vid_pll_prediv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0Set {{ reftop_pwd: {=bool:?}, reftop_pwdvbgup: {=bool:?}, reftop_lowpower: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
            self.reftop_pwd(),
            self.reftop_pwdvbgup(),
            self.reftop_lowpower(),
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
            self.xtal_24m_pwd(),
            self.vid_pll_prediv()
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
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_pwdvbgup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_pwdvbgup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_lowpower(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub const fn set_reftop_lowpower(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0TogReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0TogReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_reftop_vbgadj(&mut self, val: super::vals::Misc0TogReftopVbgadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgup(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
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
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_xtal_source(&self) -> super::vals::Misc0TogRtcXtalSource {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Misc0TogRtcXtalSource::from_bits(val as u8)
    }
    #[doc = "This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub const fn set_rtc_xtal_source(&mut self, val: super::vals::Misc0TogRtcXtalSource) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_pwd(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub const fn set_xtal_24m_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0TogVidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0TogVidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's."]
    #[inline(always)]
    pub const fn set_vid_pll_prediv(&mut self, val: super::vals::Misc0TogVidPllPrediv) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("reftop_pwdvbgup", &self.reftop_pwdvbgup())
            .field("reftop_lowpower", &self.reftop_lowpower())
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
            .field("vid_pll_prediv", &self.vid_pll_prediv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc0Tog {{ reftop_pwd: {=bool:?}, reftop_pwdvbgup: {=bool:?}, reftop_lowpower: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
            self.reftop_pwd(),
            self.reftop_pwdvbgup(),
            self.reftop_lowpower(),
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
            self.xtal_24m_pwd(),
            self.vid_pll_prediv()
        )
    }
}
#[doc = "Miscellaneous Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc1(pub u32);
impl Misc1 {
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1Lvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1Lvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1Lvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds2_clk_sel(&self) -> super::vals::Misc1Lvds2ClkSel {
        let val = (self.0 >> 5usize) & 0x1f;
        super::vals::Misc1Lvds2ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds2_clk_sel(&mut self, val: super::vals::Misc1Lvds2ClkSel) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val.to_bits() as u32) & 0x1f) << 5usize);
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_oben(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_iben(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("lvds2_clk_sel", &self.lvds2_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk2_oben", &self.lvdsclk2_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("lvdsclk2_iben", &self.lvdsclk2_iben())
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
            "Misc1 {{ lvds1_clk_sel: {:?}, lvds2_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk2_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, lvdsclk2_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvds2_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk2_oben(),
            self.lvdsclk1_iben(),
            self.lvdsclk2_iben(),
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
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1ClrLvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1ClrLvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1ClrLvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds2_clk_sel(&self) -> super::vals::Misc1ClrLvds2ClkSel {
        let val = (self.0 >> 5usize) & 0x1f;
        super::vals::Misc1ClrLvds2ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds2_clk_sel(&mut self, val: super::vals::Misc1ClrLvds2ClkSel) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val.to_bits() as u32) & 0x1f) << 5usize);
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_oben(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_iben(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("lvds2_clk_sel", &self.lvds2_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk2_oben", &self.lvdsclk2_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("lvdsclk2_iben", &self.lvdsclk2_iben())
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
            "Misc1Clr {{ lvds1_clk_sel: {:?}, lvds2_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk2_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, lvdsclk2_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvds2_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk2_oben(),
            self.lvdsclk1_iben(),
            self.lvdsclk2_iben(),
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
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1SetLvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1SetLvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1SetLvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds2_clk_sel(&self) -> super::vals::Misc1SetLvds2ClkSel {
        let val = (self.0 >> 5usize) & 0x1f;
        super::vals::Misc1SetLvds2ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds2_clk_sel(&mut self, val: super::vals::Misc1SetLvds2ClkSel) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val.to_bits() as u32) & 0x1f) << 5usize);
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_oben(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_iben(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("lvds2_clk_sel", &self.lvds2_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk2_oben", &self.lvdsclk2_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("lvdsclk2_iben", &self.lvdsclk2_iben())
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
            "Misc1Set {{ lvds1_clk_sel: {:?}, lvds2_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk2_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, lvdsclk2_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvds2_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk2_oben(),
            self.lvdsclk1_iben(),
            self.lvdsclk2_iben(),
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
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds1_clk_sel(&self) -> super::vals::Misc1TogLvds1ClkSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Misc1TogLvds1ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds1_clk_sel(&mut self, val: super::vals::Misc1TogLvds1ClkSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[must_use]
    #[inline(always)]
    pub const fn lvds2_clk_sel(&self) -> super::vals::Misc1TogLvds2ClkSel {
        let val = (self.0 >> 5usize) & 0x1f;
        super::vals::Misc1TogLvds2ClkSel::from_bits(val as u8)
    }
    #[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[inline(always)]
    pub const fn set_lvds2_clk_sel(&mut self, val: super::vals::Misc1TogLvds2ClkSel) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val.to_bits() as u32) & 0x1f) << 5usize);
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
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_oben(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS output buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_oben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdsclk2_iben(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This enables the LVDS input buffer for anaclk2/2b"]
    #[inline(always)]
    pub const fn set_lvdsclk2_iben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("lvds2_clk_sel", &self.lvds2_clk_sel())
            .field("lvdsclk1_oben", &self.lvdsclk1_oben())
            .field("lvdsclk2_oben", &self.lvdsclk2_oben())
            .field("lvdsclk1_iben", &self.lvdsclk1_iben())
            .field("lvdsclk2_iben", &self.lvdsclk2_iben())
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
            "Misc1Tog {{ lvds1_clk_sel: {:?}, lvds2_clk_sel: {:?}, lvdsclk1_oben: {=bool:?}, lvdsclk2_oben: {=bool:?}, lvdsclk1_iben: {=bool:?}, lvdsclk2_iben: {=bool:?}, pfd_480_autogate_en: {=bool:?}, pfd_528_autogate_en: {=bool:?}, irq_temppanic: {=bool:?}, irq_templow: {=bool:?}, irq_temphigh: {=bool:?}, irq_ana_bo: {=bool:?}, irq_dig_bo: {=bool:?} }}",
            self.lvds1_clk_sel(),
            self.lvds2_clk_sel(),
            self.lvdsclk1_oben(),
            self.lvdsclk2_oben(),
            self.lvdsclk1_iben(),
            self.lvdsclk2_iben(),
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Reg1 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "Reg2 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2Reg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2Reg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2Reg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2Reg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2Reg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2Reg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2Reg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2Reg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
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
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
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
            "Misc2 {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, pll3_disable: {=bool:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Reg1 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "Reg2 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2ClrReg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2ClrReg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2ClrReg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2ClrReg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2ClrReg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2ClrReg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2ClrReg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2ClrReg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
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
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
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
            "Misc2Clr {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, pll3_disable: {=bool:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Reg1 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "Reg2 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2SetReg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2SetReg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2SetReg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2SetReg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2SetReg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2SetReg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2SetReg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2SetReg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
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
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
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
            "Misc2Set {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, pll3_disable: {=bool:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
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
#[doc = "Miscellaneous Control Register"]
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
    #[doc = "Reg0 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_bo_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reg0 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg0_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_enable_bo(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg0_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Default value of \"0\""]
    #[must_use]
    #[inline(always)]
    pub const fn pll3_disable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Default value of \"0\""]
    #[inline(always)]
    pub const fn set_pll3_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Reg1 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_bo_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reg1 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg1_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_enable_bo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
    #[inline(always)]
    pub const fn set_reg1_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
    #[doc = "Reg2 brownout status bit."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_bo_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reg2 brownout status bit."]
    #[inline(always)]
    pub const fn set_reg2_bo_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the brownout detection."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_enable_bo(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the brownout detection."]
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
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_step_time(&self) -> super::vals::Misc2TogReg0StepTime {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Misc2TogReg0StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg0_step_time(&mut self, val: super::vals::Misc2TogReg0StepTime) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_step_time(&self) -> super::vals::Misc2TogReg1StepTime {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Misc2TogReg1StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub const fn set_reg1_step_time(&mut self, val: super::vals::Misc2TogReg1StepTime) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Number of clock periods (24MHz clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_step_time(&self) -> super::vals::Misc2TogReg2StepTime {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Misc2TogReg2StepTime::from_bits(val as u8)
    }
    #[doc = "Number of clock periods (24MHz clock)."]
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
            .field("pll3_disable", &self.pll3_disable())
            .field("reg1_bo_offset", &self.reg1_bo_offset())
            .field("reg1_bo_status", &self.reg1_bo_status())
            .field("reg1_enable_bo", &self.reg1_enable_bo())
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
            "Misc2Tog {{ reg0_bo_offset: {:?}, reg0_bo_status: {=bool:?}, reg0_enable_bo: {=bool:?}, pll3_disable: {=bool:?}, reg1_bo_offset: {:?}, reg1_bo_status: {=bool:?}, reg1_enable_bo: {=bool:?}, audio_div_lsb: {:?}, reg2_bo_offset: {:?}, reg2_bo_status: {=bool:?}, reg2_enable_bo: {=bool:?}, reg2_ok: {=bool:?}, audio_div_msb: {:?}, reg0_step_time: {:?}, reg1_step_time: {:?}, reg2_step_time: {:?}, video_div: {:?} }}",
            self.reg0_bo_offset(),
            self.reg0_bo_status(),
            self.reg0_enable_bo(),
            self.pll3_disable(),
            self.reg1_bo_offset(),
            self.reg1_bo_status(),
            self.reg1_enable_bo(),
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
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1p1(pub u32);
impl Reg1p1 {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1OutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1OutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1OutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> super::vals::Reg1p1SelrefWeakLinreg {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Reg1p1SelrefWeakLinreg::from_bits(val as u8)
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: super::vals::Reg1p1SelrefWeakLinreg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1 {
    #[inline(always)]
    fn default() -> Reg1p1 {
        Reg1p1(0)
    }
}
impl core::fmt::Debug for Reg1p1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1p1")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd1p1", &self.bo_vdd1p1())
            .field("ok_vdd1p1", &self.ok_vdd1p1())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .field("selref_weak_linreg", &self.selref_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1p1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg1p1 {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd1p1: {=bool:?}, ok_vdd1p1: {=bool:?}, enable_weak_linreg: {=bool:?}, selref_weak_linreg: {:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd1p1(),
            self.ok_vdd1p1(),
            self.enable_weak_linreg(),
            self.selref_weak_linreg()
        )
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1p1Clr(pub u32);
impl Reg1p1Clr {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1ClrOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1ClrOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1ClrOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> super::vals::Reg1p1ClrSelrefWeakLinreg {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Reg1p1ClrSelrefWeakLinreg::from_bits(val as u8)
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: super::vals::Reg1p1ClrSelrefWeakLinreg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1Clr {
    #[inline(always)]
    fn default() -> Reg1p1Clr {
        Reg1p1Clr(0)
    }
}
impl core::fmt::Debug for Reg1p1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1p1Clr")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd1p1", &self.bo_vdd1p1())
            .field("ok_vdd1p1", &self.ok_vdd1p1())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .field("selref_weak_linreg", &self.selref_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1p1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg1p1Clr {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd1p1: {=bool:?}, ok_vdd1p1: {=bool:?}, enable_weak_linreg: {=bool:?}, selref_weak_linreg: {:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd1p1(),
            self.ok_vdd1p1(),
            self.enable_weak_linreg(),
            self.selref_weak_linreg()
        )
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1p1Set(pub u32);
impl Reg1p1Set {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1SetOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1SetOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1SetOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> super::vals::Reg1p1SetSelrefWeakLinreg {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Reg1p1SetSelrefWeakLinreg::from_bits(val as u8)
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: super::vals::Reg1p1SetSelrefWeakLinreg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1Set {
    #[inline(always)]
    fn default() -> Reg1p1Set {
        Reg1p1Set(0)
    }
}
impl core::fmt::Debug for Reg1p1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1p1Set")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd1p1", &self.bo_vdd1p1())
            .field("ok_vdd1p1", &self.ok_vdd1p1())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .field("selref_weak_linreg", &self.selref_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1p1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg1p1Set {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd1p1: {=bool:?}, ok_vdd1p1: {=bool:?}, enable_weak_linreg: {=bool:?}, selref_weak_linreg: {:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd1p1(),
            self.ok_vdd1p1(),
            self.enable_weak_linreg(),
            self.selref_weak_linreg()
        )
    }
}
#[doc = "Regulator 1P1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1p1Tog(pub u32);
impl Reg1p1Tog {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg1p1TogOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg1p1TogOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg1p1TogOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd1p1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd1p1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd1p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn selref_weak_linreg(&self) -> super::vals::Reg1p1TogSelrefWeakLinreg {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Reg1p1TogSelrefWeakLinreg::from_bits(val as u8)
    }
    #[doc = "Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub const fn set_selref_weak_linreg(&mut self, val: super::vals::Reg1p1TogSelrefWeakLinreg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Reg1p1Tog {
    #[inline(always)]
    fn default() -> Reg1p1Tog {
        Reg1p1Tog(0)
    }
}
impl core::fmt::Debug for Reg1p1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1p1Tog")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd1p1", &self.bo_vdd1p1())
            .field("ok_vdd1p1", &self.ok_vdd1p1())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .field("selref_weak_linreg", &self.selref_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1p1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg1p1Tog {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd1p1: {=bool:?}, ok_vdd1p1: {=bool:?}, enable_weak_linreg: {=bool:?}, selref_weak_linreg: {:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd1p1(),
            self.ok_vdd1p1(),
            self.enable_weak_linreg(),
            self.selref_weak_linreg()
        )
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg2p5(pub u32);
impl Reg2p5 {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5OutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5OutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5OutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5 {
    #[inline(always)]
    fn default() -> Reg2p5 {
        Reg2p5(0)
    }
}
impl core::fmt::Debug for Reg2p5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg2p5")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd2p5", &self.bo_vdd2p5())
            .field("ok_vdd2p5", &self.ok_vdd2p5())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg2p5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg2p5 {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd2p5: {=bool:?}, ok_vdd2p5: {=bool:?}, enable_weak_linreg: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd2p5(),
            self.ok_vdd2p5(),
            self.enable_weak_linreg()
        )
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg2p5Clr(pub u32);
impl Reg2p5Clr {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5ClrOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5ClrOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5ClrOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5Clr {
    #[inline(always)]
    fn default() -> Reg2p5Clr {
        Reg2p5Clr(0)
    }
}
impl core::fmt::Debug for Reg2p5Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg2p5Clr")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd2p5", &self.bo_vdd2p5())
            .field("ok_vdd2p5", &self.ok_vdd2p5())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg2p5Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg2p5Clr {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd2p5: {=bool:?}, ok_vdd2p5: {=bool:?}, enable_weak_linreg: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd2p5(),
            self.ok_vdd2p5(),
            self.enable_weak_linreg()
        )
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg2p5Set(pub u32);
impl Reg2p5Set {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5SetOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5SetOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5SetOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5Set {
    #[inline(always)]
    fn default() -> Reg2p5Set {
        Reg2p5Set(0)
    }
}
impl core::fmt::Debug for Reg2p5Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg2p5Set")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd2p5", &self.bo_vdd2p5())
            .field("ok_vdd2p5", &self.ok_vdd2p5())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg2p5Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg2p5Set {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd2p5: {=bool:?}, ok_vdd2p5: {=bool:?}, enable_weak_linreg: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd2p5(),
            self.ok_vdd2p5(),
            self.enable_weak_linreg()
        )
    }
}
#[doc = "Regulator 2P5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg2p5Tog(pub u32);
impl Reg2p5Tog {
    #[doc = "Control bit to enable the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output."]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pulldown(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub const fn set_enable_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg2p5TogOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg2p5TogOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg2p5TogOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd2p5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd2p5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd2p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_weak_linreg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the weak 2p5 regulator"]
    #[inline(always)]
    pub const fn set_enable_weak_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Reg2p5Tog {
    #[inline(always)]
    fn default() -> Reg2p5Tog {
        Reg2p5Tog(0)
    }
}
impl core::fmt::Debug for Reg2p5Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg2p5Tog")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("enable_pulldown", &self.enable_pulldown())
            .field("bo_offset", &self.bo_offset())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd2p5", &self.bo_vdd2p5())
            .field("ok_vdd2p5", &self.ok_vdd2p5())
            .field("enable_weak_linreg", &self.enable_weak_linreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg2p5Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg2p5Tog {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, enable_pulldown: {=bool:?}, bo_offset: {=u8:?}, output_trg: {:?}, bo_vdd2p5: {=bool:?}, ok_vdd2p5: {=bool:?}, enable_weak_linreg: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.enable_pulldown(),
            self.bo_offset(),
            self.output_trg(),
            self.bo_vdd2p5(),
            self.ok_vdd2p5(),
            self.enable_weak_linreg()
        )
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg3p0(pub u32);
impl Reg3p0 {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> super::vals::Reg3p0VbusSel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Reg3p0VbusSel::from_bits(val as u8)
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: super::vals::Reg3p0VbusSel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0OutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0OutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0OutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0 {
    #[inline(always)]
    fn default() -> Reg3p0 {
        Reg3p0(0)
    }
}
impl core::fmt::Debug for Reg3p0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg3p0")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("bo_offset", &self.bo_offset())
            .field("vbus_sel", &self.vbus_sel())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd3p0", &self.bo_vdd3p0())
            .field("ok_vdd3p0", &self.ok_vdd3p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg3p0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg3p0 {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, bo_offset: {=u8:?}, vbus_sel: {:?}, output_trg: {:?}, bo_vdd3p0: {=bool:?}, ok_vdd3p0: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.bo_offset(),
            self.vbus_sel(),
            self.output_trg(),
            self.bo_vdd3p0(),
            self.ok_vdd3p0()
        )
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg3p0Clr(pub u32);
impl Reg3p0Clr {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> super::vals::Reg3p0ClrVbusSel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Reg3p0ClrVbusSel::from_bits(val as u8)
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: super::vals::Reg3p0ClrVbusSel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0ClrOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0ClrOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0ClrOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0Clr {
    #[inline(always)]
    fn default() -> Reg3p0Clr {
        Reg3p0Clr(0)
    }
}
impl core::fmt::Debug for Reg3p0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg3p0Clr")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("bo_offset", &self.bo_offset())
            .field("vbus_sel", &self.vbus_sel())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd3p0", &self.bo_vdd3p0())
            .field("ok_vdd3p0", &self.ok_vdd3p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg3p0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg3p0Clr {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, bo_offset: {=u8:?}, vbus_sel: {:?}, output_trg: {:?}, bo_vdd3p0: {=bool:?}, ok_vdd3p0: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.bo_offset(),
            self.vbus_sel(),
            self.output_trg(),
            self.bo_vdd3p0(),
            self.ok_vdd3p0()
        )
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg3p0Set(pub u32);
impl Reg3p0Set {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> super::vals::Reg3p0SetVbusSel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Reg3p0SetVbusSel::from_bits(val as u8)
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: super::vals::Reg3p0SetVbusSel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0SetOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0SetOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0SetOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0Set {
    #[inline(always)]
    fn default() -> Reg3p0Set {
        Reg3p0Set(0)
    }
}
impl core::fmt::Debug for Reg3p0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg3p0Set")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("bo_offset", &self.bo_offset())
            .field("vbus_sel", &self.vbus_sel())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd3p0", &self.bo_vdd3p0())
            .field("ok_vdd3p0", &self.ok_vdd3p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg3p0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg3p0Set {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, bo_offset: {=u8:?}, vbus_sel: {:?}, output_trg: {:?}, bo_vdd3p0: {=bool:?}, ok_vdd3p0: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.bo_offset(),
            self.vbus_sel(),
            self.output_trg(),
            self.bo_vdd3p0(),
            self.ok_vdd3p0()
        )
    }
}
#[doc = "Regulator 3P0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg3p0Tog(pub u32);
impl Reg3p0Tog {
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_linreg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub const fn set_enable_linreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_bo(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_bo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ilimit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub const fn set_enable_ilimit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[must_use]
    #[inline(always)]
    pub const fn bo_offset(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub const fn set_bo_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_sel(&self) -> super::vals::Reg3p0TogVbusSel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Reg3p0TogVbusSel::from_bits(val as u8)
    }
    #[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub const fn set_vbus_sel(&mut self, val: super::vals::Reg3p0TogVbusSel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn output_trg(&self) -> super::vals::Reg3p0TogOutputTrg {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::Reg3p0TogOutputTrg::from_bits(val as u8)
    }
    #[doc = "Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub const fn set_output_trg(&mut self, val: super::vals::Reg3p0TogOutputTrg) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[must_use]
    #[inline(always)]
    pub const fn bo_vdd3p0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub const fn set_bo_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_vdd3p0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub const fn set_ok_vdd3p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Reg3p0Tog {
    #[inline(always)]
    fn default() -> Reg3p0Tog {
        Reg3p0Tog(0)
    }
}
impl core::fmt::Debug for Reg3p0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg3p0Tog")
            .field("enable_linreg", &self.enable_linreg())
            .field("enable_bo", &self.enable_bo())
            .field("enable_ilimit", &self.enable_ilimit())
            .field("bo_offset", &self.bo_offset())
            .field("vbus_sel", &self.vbus_sel())
            .field("output_trg", &self.output_trg())
            .field("bo_vdd3p0", &self.bo_vdd3p0())
            .field("ok_vdd3p0", &self.ok_vdd3p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg3p0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg3p0Tog {{ enable_linreg: {=bool:?}, enable_bo: {=bool:?}, enable_ilimit: {=bool:?}, bo_offset: {=u8:?}, vbus_sel: {:?}, output_trg: {:?}, bo_vdd3p0: {=bool:?}, ok_vdd3p0: {=bool:?} }}",
            self.enable_linreg(),
            self.enable_bo(),
            self.enable_ilimit(),
            self.bo_offset(),
            self.vbus_sel(),
            self.output_trg(),
            self.bo_vdd3p0(),
            self.ok_vdd3p0()
        )
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegCore(pub u32);
impl RegCore {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreReg0Targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreReg0Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreReg0Targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreReg0Adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreReg0Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreReg0Adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreReg1Targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreReg1Targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreReg1Targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreReg1Adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreReg1Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreReg1Adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreReg2Targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreReg2Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreReg2Targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreReg2Adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreReg2Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreReg2Adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[must_use]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCore {
    #[inline(always)]
    fn default() -> RegCore {
        RegCore(0)
    }
}
impl core::fmt::Debug for RegCore {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegCore")
            .field("reg0_targ", &self.reg0_targ())
            .field("reg0_adj", &self.reg0_adj())
            .field("reg1_targ", &self.reg1_targ())
            .field("reg1_adj", &self.reg1_adj())
            .field("reg2_targ", &self.reg2_targ())
            .field("reg2_adj", &self.reg2_adj())
            .field("ramp_rate", &self.ramp_rate())
            .field("fet_odrive", &self.fet_odrive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegCore {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RegCore {{ reg0_targ: {:?}, reg0_adj: {:?}, reg1_targ: {:?}, reg1_adj: {:?}, reg2_targ: {:?}, reg2_adj: {:?}, ramp_rate: {:?}, fet_odrive: {=bool:?} }}",
            self.reg0_targ(),
            self.reg0_adj(),
            self.reg1_targ(),
            self.reg1_adj(),
            self.reg2_targ(),
            self.reg2_adj(),
            self.ramp_rate(),
            self.fet_odrive()
        )
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegCoreClr(pub u32);
impl RegCoreClr {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreClrReg0Targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreClrReg0Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreClrReg0Targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreClrReg0Adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreClrReg0Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreClrReg0Adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreClrReg1Targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreClrReg1Targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreClrReg1Targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreClrReg1Adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreClrReg1Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreClrReg1Adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreClrReg2Targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreClrReg2Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreClrReg2Targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreClrReg2Adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreClrReg2Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreClrReg2Adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreClrRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreClrRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreClrRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[must_use]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCoreClr {
    #[inline(always)]
    fn default() -> RegCoreClr {
        RegCoreClr(0)
    }
}
impl core::fmt::Debug for RegCoreClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegCoreClr")
            .field("reg0_targ", &self.reg0_targ())
            .field("reg0_adj", &self.reg0_adj())
            .field("reg1_targ", &self.reg1_targ())
            .field("reg1_adj", &self.reg1_adj())
            .field("reg2_targ", &self.reg2_targ())
            .field("reg2_adj", &self.reg2_adj())
            .field("ramp_rate", &self.ramp_rate())
            .field("fet_odrive", &self.fet_odrive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegCoreClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RegCoreClr {{ reg0_targ: {:?}, reg0_adj: {:?}, reg1_targ: {:?}, reg1_adj: {:?}, reg2_targ: {:?}, reg2_adj: {:?}, ramp_rate: {:?}, fet_odrive: {=bool:?} }}",
            self.reg0_targ(),
            self.reg0_adj(),
            self.reg1_targ(),
            self.reg1_adj(),
            self.reg2_targ(),
            self.reg2_adj(),
            self.ramp_rate(),
            self.fet_odrive()
        )
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegCoreSet(pub u32);
impl RegCoreSet {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreSetReg0Targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreSetReg0Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreSetReg0Targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreSetReg0Adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreSetReg0Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreSetReg0Adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreSetReg1Targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreSetReg1Targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreSetReg1Targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreSetReg1Adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreSetReg1Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreSetReg1Adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreSetReg2Targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreSetReg2Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreSetReg2Targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreSetReg2Adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreSetReg2Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreSetReg2Adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreSetRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreSetRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreSetRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[must_use]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCoreSet {
    #[inline(always)]
    fn default() -> RegCoreSet {
        RegCoreSet(0)
    }
}
impl core::fmt::Debug for RegCoreSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegCoreSet")
            .field("reg0_targ", &self.reg0_targ())
            .field("reg0_adj", &self.reg0_adj())
            .field("reg1_targ", &self.reg1_targ())
            .field("reg1_adj", &self.reg1_adj())
            .field("reg2_targ", &self.reg2_targ())
            .field("reg2_adj", &self.reg2_adj())
            .field("ramp_rate", &self.ramp_rate())
            .field("fet_odrive", &self.fet_odrive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegCoreSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RegCoreSet {{ reg0_targ: {:?}, reg0_adj: {:?}, reg1_targ: {:?}, reg1_adj: {:?}, reg2_targ: {:?}, reg2_adj: {:?}, ramp_rate: {:?}, fet_odrive: {=bool:?} }}",
            self.reg0_targ(),
            self.reg0_adj(),
            self.reg1_targ(),
            self.reg1_adj(),
            self.reg2_targ(),
            self.reg2_adj(),
            self.ramp_rate(),
            self.fet_odrive()
        )
    }
}
#[doc = "Digital Regulator Core Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegCoreTog(pub u32);
impl RegCoreTog {
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_targ(&self) -> super::vals::RegCoreTogReg0Targ {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RegCoreTogReg0Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub const fn set_reg0_targ(&mut self, val: super::vals::RegCoreTogReg0Targ) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_adj(&self) -> super::vals::RegCoreTogReg0Adj {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::RegCoreTogReg0Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg0_adj(&mut self, val: super::vals::RegCoreTogReg0Adj) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_targ(&self) -> super::vals::RegCoreTogReg1Targ {
        let val = (self.0 >> 9usize) & 0x1f;
        super::vals::RegCoreTogReg1Targ::from_bits(val as u8)
    }
    #[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub const fn set_reg1_targ(&mut self, val: super::vals::RegCoreTogReg1Targ) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_adj(&self) -> super::vals::RegCoreTogReg1Adj {
        let val = (self.0 >> 14usize) & 0x0f;
        super::vals::RegCoreTogReg1Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg1_adj(&mut self, val: super::vals::RegCoreTogReg1Adj) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val.to_bits() as u32) & 0x0f) << 14usize);
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_targ(&self) -> super::vals::RegCoreTogReg2Targ {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::RegCoreTogReg2Targ::from_bits(val as u8)
    }
    #[doc = "This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub const fn set_reg2_targ(&mut self, val: super::vals::RegCoreTogReg2Targ) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_adj(&self) -> super::vals::RegCoreTogReg2Adj {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::RegCoreTogReg2Adj::from_bits(val as u8)
    }
    #[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub const fn set_reg2_adj(&mut self, val: super::vals::RegCoreTogReg2Adj) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Regulator voltage ramp rate."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_rate(&self) -> super::vals::RegCoreTogRampRate {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::RegCoreTogRampRate::from_bits(val as u8)
    }
    #[doc = "Regulator voltage ramp rate."]
    #[inline(always)]
    pub const fn set_ramp_rate(&mut self, val: super::vals::RegCoreTogRampRate) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[must_use]
    #[inline(always)]
    pub const fn fet_odrive(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub const fn set_fet_odrive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for RegCoreTog {
    #[inline(always)]
    fn default() -> RegCoreTog {
        RegCoreTog(0)
    }
}
impl core::fmt::Debug for RegCoreTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RegCoreTog")
            .field("reg0_targ", &self.reg0_targ())
            .field("reg0_adj", &self.reg0_adj())
            .field("reg1_targ", &self.reg1_targ())
            .field("reg1_adj", &self.reg1_adj())
            .field("reg2_targ", &self.reg2_targ())
            .field("reg2_adj", &self.reg2_adj())
            .field("ramp_rate", &self.ramp_rate())
            .field("fet_odrive", &self.fet_odrive())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RegCoreTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RegCoreTog {{ reg0_targ: {:?}, reg0_adj: {:?}, reg1_targ: {:?}, reg1_adj: {:?}, reg2_targ: {:?}, reg2_adj: {:?}, ramp_rate: {:?}, fet_odrive: {=bool:?} }}",
            self.reg0_targ(),
            self.reg0_adj(),
            self.reg1_targ(),
            self.reg1_adj(),
            self.reg2_targ(),
            self.reg2_adj(),
            self.ramp_rate(),
            self.fet_odrive()
        )
    }
}
