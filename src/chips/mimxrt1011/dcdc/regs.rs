#[doc = "DCDC Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg0(pub u32);
impl Reg0 {
    #[doc = "Power Down Zero Cross Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_zcd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Zero Cross Detection"]
    #[inline(always)]
    pub const fn set_pwd_zcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable Auto Clock Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_auto_clk_switch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Auto Clock Switch"]
    #[inline(always)]
    pub const fn set_disable_auto_clk_switch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Select Clock"]
    #[inline(always)]
    pub const fn set_sel_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Power down internal osc"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_osc_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Power down internal osc"]
    #[inline(always)]
    pub const fn set_pwd_osc_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Power down signal of the current detector."]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_cur_sns_cmp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Power down signal of the current detector."]
    #[inline(always)]
    pub const fn set_pwd_cur_sns_cmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Current Sense (detector) Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn cur_sns_thrsh(&self) -> super::vals::CurSnsThrsh {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::CurSnsThrsh::from_bits(val as u8)
    }
    #[doc = "Current Sense (detector) Threshold"]
    #[inline(always)]
    pub const fn set_cur_sns_thrsh(&mut self, val: super::vals::CurSnsThrsh) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Power down overcurrent detection comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_overcur_det(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Power down overcurrent detection comparator"]
    #[inline(always)]
    pub const fn set_pwd_overcur_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Overcurrent Trigger Adjust"]
    #[must_use]
    #[inline(always)]
    pub const fn overcur_trig_adj(&self) -> super::vals::OvercurTrigAdj {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::OvercurTrigAdj::from_bits(val as u8)
    }
    #[doc = "Overcurrent Trigger Adjust"]
    #[inline(always)]
    pub const fn set_overcur_trig_adj(&mut self, val: super::vals::OvercurTrigAdj) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Power Down Battery Detection Comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_cmp_batt_det(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Battery Detection Comparator"]
    #[inline(always)]
    pub const fn set_pwd_cmp_batt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Low Power Overload Sense Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en_lp_overload_sns(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Overload Sense Enable"]
    #[inline(always)]
    pub const fn set_en_lp_overload_sns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Power Down High Voltage Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_high_volt_det(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down High Voltage Detection"]
    #[inline(always)]
    pub const fn set_pwd_high_volt_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Low Power Overload Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_overload_thrsh(&self) -> super::vals::LpOverloadThrsh {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::LpOverloadThrsh::from_bits(val as u8)
    }
    #[doc = "Low Power Overload Threshold"]
    #[inline(always)]
    pub const fn set_lp_overload_thrsh(&mut self, val: super::vals::LpOverloadThrsh) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Low Power Overload Frequency Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_overload_freq_sel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Overload Frequency Select"]
    #[inline(always)]
    pub const fn set_lp_overload_freq_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Low Power High Hysteric Value"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_high_hys(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power High Hysteric Value"]
    #[inline(always)]
    pub const fn set_lp_high_hys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Power down output range comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_cmp_offset(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Power down output range comparator"]
    #[inline(always)]
    pub const fn set_pwd_cmp_offset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Disable xtalok detection circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn xtalok_disable(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Disable xtalok detection circuit"]
    #[inline(always)]
    pub const fn set_xtalok_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Reset Current Alert Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn current_alert_reset(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Current Alert Signal"]
    #[inline(always)]
    pub const fn set_current_alert_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "24M XTAL OK"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_ok(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "24M XTAL OK"]
    #[inline(always)]
    pub const fn set_xtal_24m_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DCDC Output OK"]
    #[must_use]
    #[inline(always)]
    pub const fn sts_dc_ok(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Output OK"]
    #[inline(always)]
    pub const fn set_sts_dc_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Reg0 {
    #[inline(always)]
    fn default() -> Reg0 {
        Reg0(0)
    }
}
impl core::fmt::Debug for Reg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg0")
            .field("pwd_zcd", &self.pwd_zcd())
            .field("disable_auto_clk_switch", &self.disable_auto_clk_switch())
            .field("sel_clk", &self.sel_clk())
            .field("pwd_osc_int", &self.pwd_osc_int())
            .field("pwd_cur_sns_cmp", &self.pwd_cur_sns_cmp())
            .field("cur_sns_thrsh", &self.cur_sns_thrsh())
            .field("pwd_overcur_det", &self.pwd_overcur_det())
            .field("overcur_trig_adj", &self.overcur_trig_adj())
            .field("pwd_cmp_batt_det", &self.pwd_cmp_batt_det())
            .field("en_lp_overload_sns", &self.en_lp_overload_sns())
            .field("pwd_high_volt_det", &self.pwd_high_volt_det())
            .field("lp_overload_thrsh", &self.lp_overload_thrsh())
            .field("lp_overload_freq_sel", &self.lp_overload_freq_sel())
            .field("lp_high_hys", &self.lp_high_hys())
            .field("pwd_cmp_offset", &self.pwd_cmp_offset())
            .field("xtalok_disable", &self.xtalok_disable())
            .field("current_alert_reset", &self.current_alert_reset())
            .field("xtal_24m_ok", &self.xtal_24m_ok())
            .field("sts_dc_ok", &self.sts_dc_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg0 {{ pwd_zcd: {=bool:?}, disable_auto_clk_switch: {=bool:?}, sel_clk: {=bool:?}, pwd_osc_int: {=bool:?}, pwd_cur_sns_cmp: {=bool:?}, cur_sns_thrsh: {:?}, pwd_overcur_det: {=bool:?}, overcur_trig_adj: {:?}, pwd_cmp_batt_det: {=bool:?}, en_lp_overload_sns: {=bool:?}, pwd_high_volt_det: {=bool:?}, lp_overload_thrsh: {:?}, lp_overload_freq_sel: {=bool:?}, lp_high_hys: {=bool:?}, pwd_cmp_offset: {=bool:?}, xtalok_disable: {=bool:?}, current_alert_reset: {=bool:?}, xtal_24m_ok: {=bool:?}, sts_dc_ok: {=bool:?} }}",
            self.pwd_zcd(),
            self.disable_auto_clk_switch(),
            self.sel_clk(),
            self.pwd_osc_int(),
            self.pwd_cur_sns_cmp(),
            self.cur_sns_thrsh(),
            self.pwd_overcur_det(),
            self.overcur_trig_adj(),
            self.pwd_cmp_batt_det(),
            self.en_lp_overload_sns(),
            self.pwd_high_volt_det(),
            self.lp_overload_thrsh(),
            self.lp_overload_freq_sel(),
            self.lp_high_hys(),
            self.pwd_cmp_offset(),
            self.xtalok_disable(),
            self.current_alert_reset(),
            self.xtal_24m_ok(),
            self.sts_dc_ok()
        )
    }
}
#[doc = "DCDC Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1(pub u32);
impl Reg1 {
    #[doc = "Select the feedback point of the internal regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_fbk_sel(&self) -> super::vals::RegFbkSel {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::RegFbkSel::from_bits(val as u8)
    }
    #[doc = "Select the feedback point of the internal regulator"]
    #[inline(always)]
    pub const fn set_reg_fbk_sel(&mut self, val: super::vals::RegFbkSel) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "This controls the load resistor of the internal regulator of DCDC"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_rload_sw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This controls the load resistor of the internal regulator of DCDC"]
    #[inline(always)]
    pub const fn set_reg_rload_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Low Power Comparator Current Bias"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_cmp_isrc_sel(&self) -> super::vals::LpCmpIsrcSel {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::LpCmpIsrcSel::from_bits(val as u8)
    }
    #[doc = "Low Power Comparator Current Bias"]
    #[inline(always)]
    pub const fn set_lp_cmp_isrc_sel(&mut self, val: super::vals::LpCmpIsrcSel) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Increase Threshold Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_hst_thresh(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Increase Threshold Detection"]
    #[inline(always)]
    pub const fn set_loopctrl_hst_thresh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable Hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_en_hyst(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Hysteresis"]
    #[inline(always)]
    pub const fn set_loopctrl_en_hyst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Trim Bandgap Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn vbg_trim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Bandgap Voltage"]
    #[inline(always)]
    pub const fn set_vbg_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Reg1 {
    #[inline(always)]
    fn default() -> Reg1 {
        Reg1(0)
    }
}
impl core::fmt::Debug for Reg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1")
            .field("reg_fbk_sel", &self.reg_fbk_sel())
            .field("reg_rload_sw", &self.reg_rload_sw())
            .field("lp_cmp_isrc_sel", &self.lp_cmp_isrc_sel())
            .field("loopctrl_hst_thresh", &self.loopctrl_hst_thresh())
            .field("loopctrl_en_hyst", &self.loopctrl_en_hyst())
            .field("vbg_trim", &self.vbg_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg1 {{ reg_fbk_sel: {:?}, reg_rload_sw: {=bool:?}, lp_cmp_isrc_sel: {:?}, loopctrl_hst_thresh: {=bool:?}, loopctrl_en_hyst: {=bool:?}, vbg_trim: {=u8:?} }}",
            self.reg_fbk_sel(),
            self.reg_rload_sw(),
            self.lp_cmp_isrc_sel(),
            self.loopctrl_hst_thresh(),
            self.loopctrl_en_hyst(),
            self.vbg_trim()
        )
    }
}
#[doc = "DCDC Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg2(pub u32);
impl Reg2 {
    #[doc = "Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_dc_ff(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline(always)]
    pub const fn set_loopctrl_dc_ff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "Enable RC Scale"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_en_rcscale(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Enable RC Scale"]
    #[inline(always)]
    pub const fn set_loopctrl_en_rcscale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Increase the threshold detection for RC scale circuit."]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_rcscale_thrsh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub const fn set_loopctrl_rcscale_thrsh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_hyst_sign(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline(always)]
    pub const fn set_loopctrl_hyst_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Disable Pulse Skip"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_pulse_skip(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Pulse Skip"]
    #[inline(always)]
    pub const fn set_disable_pulse_skip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DCM Set Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dcm_set_ctrl(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DCM Set Control"]
    #[inline(always)]
    pub const fn set_dcm_set_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Reg2 {
    #[inline(always)]
    fn default() -> Reg2 {
        Reg2(0)
    }
}
impl core::fmt::Debug for Reg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg2")
            .field("loopctrl_dc_ff", &self.loopctrl_dc_ff())
            .field("loopctrl_en_rcscale", &self.loopctrl_en_rcscale())
            .field("loopctrl_rcscale_thrsh", &self.loopctrl_rcscale_thrsh())
            .field("loopctrl_hyst_sign", &self.loopctrl_hyst_sign())
            .field("disable_pulse_skip", &self.disable_pulse_skip())
            .field("dcm_set_ctrl", &self.dcm_set_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg2 {{ loopctrl_dc_ff: {=u8:?}, loopctrl_en_rcscale: {=u8:?}, loopctrl_rcscale_thrsh: {=bool:?}, loopctrl_hyst_sign: {=bool:?}, disable_pulse_skip: {=bool:?}, dcm_set_ctrl: {=bool:?} }}",
            self.loopctrl_dc_ff(),
            self.loopctrl_en_rcscale(),
            self.loopctrl_rcscale_thrsh(),
            self.loopctrl_hyst_sign(),
            self.disable_pulse_skip(),
            self.dcm_set_ctrl()
        )
    }
}
#[doc = "DCDC Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg3(pub u32);
impl Reg3 {
    #[doc = "Target value of VDD_SOC"]
    #[must_use]
    #[inline(always)]
    pub const fn trg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD_SOC"]
    #[inline(always)]
    pub const fn set_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Low Power Target Value"]
    #[must_use]
    #[inline(always)]
    pub const fn target_lp(&self) -> super::vals::TargetLp {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::TargetLp::from_bits(val as u8)
    }
    #[doc = "Low Power Target Value"]
    #[inline(always)]
    pub const fn set_target_lp(&mut self, val: super::vals::TargetLp) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Set DCDC clock to half frequency for continuous mode"]
    #[must_use]
    #[inline(always)]
    pub const fn minpwr_dc_halfclk(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set DCDC clock to half frequency for continuous mode"]
    #[inline(always)]
    pub const fn set_minpwr_dc_halfclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Disable Step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_step(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Step"]
    #[inline(always)]
    pub const fn set_disable_step(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Reg3 {
    #[inline(always)]
    fn default() -> Reg3 {
        Reg3(0)
    }
}
impl core::fmt::Debug for Reg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg3")
            .field("trg", &self.trg())
            .field("target_lp", &self.target_lp())
            .field("minpwr_dc_halfclk", &self.minpwr_dc_halfclk())
            .field("disable_step", &self.disable_step())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Reg3 {{ trg: {=u8:?}, target_lp: {:?}, minpwr_dc_halfclk: {=bool:?}, disable_step: {=bool:?} }}",
            self.trg(),
            self.target_lp(),
            self.minpwr_dc_halfclk(),
            self.disable_step()
        )
    }
}
