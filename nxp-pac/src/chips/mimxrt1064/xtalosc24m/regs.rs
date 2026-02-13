#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LowpwrCtrl(pub u32);
impl LowpwrCtrl {
    #[doc = "RC Osc. enable control."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> super::vals::LowpwrCtrlRcOscEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LowpwrCtrlRcOscEn::from_bits(val as u8)
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: super::vals::LowpwrCtrlRcOscEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_sel(&self) -> super::vals::LowpwrCtrlOscSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LowpwrCtrlOscSel::from_bits(val as u8)
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: super::vals::LowpwrCtrlOscSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> super::vals::LowpwrCtrlLpbgSel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LowpwrCtrlLpbgSel::from_bits(val as u8)
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: super::vals::LowpwrCtrlLpbgSel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[must_use]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(&mut self, val: super::vals::LowpwrCtrlXtaloscPwrupDelay) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> super::vals::LowpwrCtrlXtaloscPwrupStat {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LowpwrCtrlXtaloscPwrupStat::from_bits(val as u8)
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(&mut self, val: super::vals::LowpwrCtrlXtaloscPwrupStat) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrl {
    #[inline(always)]
    fn default() -> LowpwrCtrl {
        LowpwrCtrl(0)
    }
}
impl core::fmt::Debug for LowpwrCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LowpwrCtrl")
            .field("rc_osc_en", &self.rc_osc_en())
            .field("osc_sel", &self.osc_sel())
            .field("lpbg_sel", &self.lpbg_sel())
            .field("lpbg_test", &self.lpbg_test())
            .field("reftop_ibias_off", &self.reftop_ibias_off())
            .field("l1_pwrgate", &self.l1_pwrgate())
            .field("l2_pwrgate", &self.l2_pwrgate())
            .field("cpu_pwrgate", &self.cpu_pwrgate())
            .field("display_pwrgate", &self.display_pwrgate())
            .field("rcosc_cg_override", &self.rcosc_cg_override())
            .field("xtalosc_pwrup_delay", &self.xtalosc_pwrup_delay())
            .field("xtalosc_pwrup_stat", &self.xtalosc_pwrup_stat())
            .field("mix_pwrgate", &self.mix_pwrgate())
            .field("gpu_pwrgate", &self.gpu_pwrgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LowpwrCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LowpwrCtrl {{ rc_osc_en: {:?}, osc_sel: {:?}, lpbg_sel: {:?}, lpbg_test: {=bool:?}, reftop_ibias_off: {=bool:?}, l1_pwrgate: {=bool:?}, l2_pwrgate: {=bool:?}, cpu_pwrgate: {=bool:?}, display_pwrgate: {=bool:?}, rcosc_cg_override: {=bool:?}, xtalosc_pwrup_delay: {:?}, xtalosc_pwrup_stat: {:?}, mix_pwrgate: {=bool:?}, gpu_pwrgate: {=bool:?} }}",
            self.rc_osc_en(),
            self.osc_sel(),
            self.lpbg_sel(),
            self.lpbg_test(),
            self.reftop_ibias_off(),
            self.l1_pwrgate(),
            self.l2_pwrgate(),
            self.cpu_pwrgate(),
            self.display_pwrgate(),
            self.rcosc_cg_override(),
            self.xtalosc_pwrup_delay(),
            self.xtalosc_pwrup_stat(),
            self.mix_pwrgate(),
            self.gpu_pwrgate()
        )
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LowpwrCtrlClr(pub u32);
impl LowpwrCtrlClr {
    #[doc = "RC Osc. enable control."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> super::vals::LowpwrCtrlClrRcOscEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LowpwrCtrlClrRcOscEn::from_bits(val as u8)
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: super::vals::LowpwrCtrlClrRcOscEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_sel(&self) -> super::vals::LowpwrCtrlClrOscSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LowpwrCtrlClrOscSel::from_bits(val as u8)
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: super::vals::LowpwrCtrlClrOscSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> super::vals::LowpwrCtrlClrLpbgSel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LowpwrCtrlClrLpbgSel::from_bits(val as u8)
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: super::vals::LowpwrCtrlClrLpbgSel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[must_use]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlClrXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlClrXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(
        &mut self,
        val: super::vals::LowpwrCtrlClrXtaloscPwrupDelay,
    ) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> super::vals::LowpwrCtrlClrXtaloscPwrupStat {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LowpwrCtrlClrXtaloscPwrupStat::from_bits(val as u8)
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(
        &mut self,
        val: super::vals::LowpwrCtrlClrXtaloscPwrupStat,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrlClr {
    #[inline(always)]
    fn default() -> LowpwrCtrlClr {
        LowpwrCtrlClr(0)
    }
}
impl core::fmt::Debug for LowpwrCtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LowpwrCtrlClr")
            .field("rc_osc_en", &self.rc_osc_en())
            .field("osc_sel", &self.osc_sel())
            .field("lpbg_sel", &self.lpbg_sel())
            .field("lpbg_test", &self.lpbg_test())
            .field("reftop_ibias_off", &self.reftop_ibias_off())
            .field("l1_pwrgate", &self.l1_pwrgate())
            .field("l2_pwrgate", &self.l2_pwrgate())
            .field("cpu_pwrgate", &self.cpu_pwrgate())
            .field("display_pwrgate", &self.display_pwrgate())
            .field("rcosc_cg_override", &self.rcosc_cg_override())
            .field("xtalosc_pwrup_delay", &self.xtalosc_pwrup_delay())
            .field("xtalosc_pwrup_stat", &self.xtalosc_pwrup_stat())
            .field("mix_pwrgate", &self.mix_pwrgate())
            .field("gpu_pwrgate", &self.gpu_pwrgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LowpwrCtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LowpwrCtrlClr {{ rc_osc_en: {:?}, osc_sel: {:?}, lpbg_sel: {:?}, lpbg_test: {=bool:?}, reftop_ibias_off: {=bool:?}, l1_pwrgate: {=bool:?}, l2_pwrgate: {=bool:?}, cpu_pwrgate: {=bool:?}, display_pwrgate: {=bool:?}, rcosc_cg_override: {=bool:?}, xtalosc_pwrup_delay: {:?}, xtalosc_pwrup_stat: {:?}, mix_pwrgate: {=bool:?}, gpu_pwrgate: {=bool:?} }}",
            self.rc_osc_en(),
            self.osc_sel(),
            self.lpbg_sel(),
            self.lpbg_test(),
            self.reftop_ibias_off(),
            self.l1_pwrgate(),
            self.l2_pwrgate(),
            self.cpu_pwrgate(),
            self.display_pwrgate(),
            self.rcosc_cg_override(),
            self.xtalosc_pwrup_delay(),
            self.xtalosc_pwrup_stat(),
            self.mix_pwrgate(),
            self.gpu_pwrgate()
        )
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LowpwrCtrlSet(pub u32);
impl LowpwrCtrlSet {
    #[doc = "RC Osc. enable control."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> super::vals::LowpwrCtrlSetRcOscEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LowpwrCtrlSetRcOscEn::from_bits(val as u8)
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: super::vals::LowpwrCtrlSetRcOscEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_sel(&self) -> super::vals::LowpwrCtrlSetOscSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LowpwrCtrlSetOscSel::from_bits(val as u8)
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: super::vals::LowpwrCtrlSetOscSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> super::vals::LowpwrCtrlSetLpbgSel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LowpwrCtrlSetLpbgSel::from_bits(val as u8)
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: super::vals::LowpwrCtrlSetLpbgSel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[must_use]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlSetXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlSetXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(
        &mut self,
        val: super::vals::LowpwrCtrlSetXtaloscPwrupDelay,
    ) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> super::vals::LowpwrCtrlSetXtaloscPwrupStat {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LowpwrCtrlSetXtaloscPwrupStat::from_bits(val as u8)
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(
        &mut self,
        val: super::vals::LowpwrCtrlSetXtaloscPwrupStat,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrlSet {
    #[inline(always)]
    fn default() -> LowpwrCtrlSet {
        LowpwrCtrlSet(0)
    }
}
impl core::fmt::Debug for LowpwrCtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LowpwrCtrlSet")
            .field("rc_osc_en", &self.rc_osc_en())
            .field("osc_sel", &self.osc_sel())
            .field("lpbg_sel", &self.lpbg_sel())
            .field("lpbg_test", &self.lpbg_test())
            .field("reftop_ibias_off", &self.reftop_ibias_off())
            .field("l1_pwrgate", &self.l1_pwrgate())
            .field("l2_pwrgate", &self.l2_pwrgate())
            .field("cpu_pwrgate", &self.cpu_pwrgate())
            .field("display_pwrgate", &self.display_pwrgate())
            .field("rcosc_cg_override", &self.rcosc_cg_override())
            .field("xtalosc_pwrup_delay", &self.xtalosc_pwrup_delay())
            .field("xtalosc_pwrup_stat", &self.xtalosc_pwrup_stat())
            .field("mix_pwrgate", &self.mix_pwrgate())
            .field("gpu_pwrgate", &self.gpu_pwrgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LowpwrCtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LowpwrCtrlSet {{ rc_osc_en: {:?}, osc_sel: {:?}, lpbg_sel: {:?}, lpbg_test: {=bool:?}, reftop_ibias_off: {=bool:?}, l1_pwrgate: {=bool:?}, l2_pwrgate: {=bool:?}, cpu_pwrgate: {=bool:?}, display_pwrgate: {=bool:?}, rcosc_cg_override: {=bool:?}, xtalosc_pwrup_delay: {:?}, xtalosc_pwrup_stat: {:?}, mix_pwrgate: {=bool:?}, gpu_pwrgate: {=bool:?} }}",
            self.rc_osc_en(),
            self.osc_sel(),
            self.lpbg_sel(),
            self.lpbg_test(),
            self.reftop_ibias_off(),
            self.l1_pwrgate(),
            self.l2_pwrgate(),
            self.cpu_pwrgate(),
            self.display_pwrgate(),
            self.rcosc_cg_override(),
            self.xtalosc_pwrup_delay(),
            self.xtalosc_pwrup_stat(),
            self.mix_pwrgate(),
            self.gpu_pwrgate()
        )
    }
}
#[doc = "XTAL OSC (LP) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LowpwrCtrlTog(pub u32);
impl LowpwrCtrlTog {
    #[doc = "RC Osc. enable control."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_en(&self) -> super::vals::LowpwrCtrlTogRcOscEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LowpwrCtrlTogRcOscEn::from_bits(val as u8)
    }
    #[doc = "RC Osc. enable control."]
    #[inline(always)]
    pub const fn set_rc_osc_en(&mut self, val: super::vals::LowpwrCtrlTogRcOscEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_sel(&self) -> super::vals::LowpwrCtrlTogOscSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LowpwrCtrlTogOscSel::from_bits(val as u8)
    }
    #[doc = "Select the source for the 24MHz clock."]
    #[inline(always)]
    pub const fn set_osc_sel(&mut self, val: super::vals::LowpwrCtrlTogOscSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_sel(&self) -> super::vals::LowpwrCtrlTogLpbgSel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LowpwrCtrlTogLpbgSel::from_bits(val as u8)
    }
    #[doc = "Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_sel(&mut self, val: super::vals::LowpwrCtrlTogLpbgSel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_test(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_lpbg_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_ibias_off(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_reftop_ibias_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l1_pwrgate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l1_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn l2_pwrgate(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_l2_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub const fn set_cpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn display_pwrgate(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub const fn set_display_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "For debug purposes only"]
    #[must_use]
    #[inline(always)]
    pub const fn rcosc_cg_override(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "For debug purposes only"]
    #[inline(always)]
    pub const fn set_rcosc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_delay(&self) -> super::vals::LowpwrCtrlTogXtaloscPwrupDelay {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::LowpwrCtrlTogXtaloscPwrupDelay::from_bits(val as u8)
    }
    #[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_delay(
        &mut self,
        val: super::vals::LowpwrCtrlTogXtaloscPwrupDelay,
    ) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn xtalosc_pwrup_stat(&self) -> super::vals::LowpwrCtrlTogXtaloscPwrupStat {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LowpwrCtrlTogXtaloscPwrupStat::from_bits(val as u8)
    }
    #[doc = "Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub const fn set_xtalosc_pwrup_stat(
        &mut self,
        val: super::vals::LowpwrCtrlTogXtaloscPwrupStat,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn mix_pwrgate(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_mix_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[must_use]
    #[inline(always)]
    pub const fn gpu_pwrgate(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub const fn set_gpu_pwrgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LowpwrCtrlTog {
    #[inline(always)]
    fn default() -> LowpwrCtrlTog {
        LowpwrCtrlTog(0)
    }
}
impl core::fmt::Debug for LowpwrCtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LowpwrCtrlTog")
            .field("rc_osc_en", &self.rc_osc_en())
            .field("osc_sel", &self.osc_sel())
            .field("lpbg_sel", &self.lpbg_sel())
            .field("lpbg_test", &self.lpbg_test())
            .field("reftop_ibias_off", &self.reftop_ibias_off())
            .field("l1_pwrgate", &self.l1_pwrgate())
            .field("l2_pwrgate", &self.l2_pwrgate())
            .field("cpu_pwrgate", &self.cpu_pwrgate())
            .field("display_pwrgate", &self.display_pwrgate())
            .field("rcosc_cg_override", &self.rcosc_cg_override())
            .field("xtalosc_pwrup_delay", &self.xtalosc_pwrup_delay())
            .field("xtalosc_pwrup_stat", &self.xtalosc_pwrup_stat())
            .field("mix_pwrgate", &self.mix_pwrgate())
            .field("gpu_pwrgate", &self.gpu_pwrgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LowpwrCtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LowpwrCtrlTog {{ rc_osc_en: {:?}, osc_sel: {:?}, lpbg_sel: {:?}, lpbg_test: {=bool:?}, reftop_ibias_off: {=bool:?}, l1_pwrgate: {=bool:?}, l2_pwrgate: {=bool:?}, cpu_pwrgate: {=bool:?}, display_pwrgate: {=bool:?}, rcosc_cg_override: {=bool:?}, xtalosc_pwrup_delay: {:?}, xtalosc_pwrup_stat: {:?}, mix_pwrgate: {=bool:?}, gpu_pwrgate: {=bool:?} }}",
            self.rc_osc_en(),
            self.osc_sel(),
            self.lpbg_sel(),
            self.lpbg_test(),
            self.reftop_ibias_off(),
            self.l1_pwrgate(),
            self.l2_pwrgate(),
            self.cpu_pwrgate(),
            self.display_pwrgate(),
            self.rcosc_cg_override(),
            self.xtalosc_pwrup_delay(),
            self.xtalosc_pwrup_stat(),
            self.mix_pwrgate(),
            self.gpu_pwrgate()
        )
    }
}
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
    #[doc = "Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0ReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0ReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
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
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0StopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0StopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0VidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0VidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
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
            "Misc0 {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
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
    #[doc = "Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0ClrReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0ClrReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
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
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0ClrStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0ClrStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0ClrVidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0ClrVidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
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
            "Misc0Clr {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
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
    #[doc = "Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0SetReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0SetReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
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
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0SetStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0SetStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0SetVidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0SetVidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
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
            "Misc0Set {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
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
    #[doc = "Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn reftop_vbgadj(&self) -> super::vals::Misc0TogReftopVbgadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Misc0TogReftopVbgadj::from_bits(val as u8)
    }
    #[doc = "Not related to oscillator."]
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
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_mode_config(&self) -> super::vals::Misc0TogStopModeConfig {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Misc0TogStopModeConfig::from_bits(val as u8)
    }
    #[doc = "Configure the analog behavior in stop mode. Not related to oscillator."]
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
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_xtalok_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the detector that signals when the 24MHz crystal oscillator is stable."]
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
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn vid_pll_prediv(&self) -> super::vals::Misc0TogVidPllPrediv {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Misc0TogVidPllPrediv::from_bits(val as u8)
    }
    #[doc = "Predivider for the source clock of the PLL's. Not related to oscillator."]
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
            "Misc0Tog {{ reftop_pwd: {=bool:?}, reftop_selfbiasoff: {:?}, reftop_vbgadj: {:?}, reftop_vbgup: {=bool:?}, stop_mode_config: {:?}, discon_high_snvs: {:?}, osc_i: {:?}, osc_xtalok: {=bool:?}, osc_xtalok_en: {=bool:?}, clkgate_ctrl: {:?}, clkgate_delay: {:?}, rtc_xtal_source: {:?}, xtal_24m_pwd: {=bool:?}, vid_pll_prediv: {:?} }}",
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
            self.xtal_24m_pwd(),
            self.vid_pll_prediv()
        )
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig0(pub u32);
impl OscConfig0 {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0 {
    #[inline(always)]
    fn default() -> OscConfig0 {
        OscConfig0(0)
    }
}
impl core::fmt::Debug for OscConfig0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig0")
            .field("start", &self.start())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("invert", &self.invert())
            .field("rc_osc_prog", &self.rc_osc_prog())
            .field("hyst_plus", &self.hyst_plus())
            .field("hyst_minus", &self.hyst_minus())
            .field("rc_osc_prog_cur", &self.rc_osc_prog_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig0 {{ start: {=bool:?}, enable: {=bool:?}, bypass: {=bool:?}, invert: {=bool:?}, rc_osc_prog: {=u8:?}, hyst_plus: {=u8:?}, hyst_minus: {=u8:?}, rc_osc_prog_cur: {=u8:?} }}",
            self.start(),
            self.enable(),
            self.bypass(),
            self.invert(),
            self.rc_osc_prog(),
            self.hyst_plus(),
            self.hyst_minus(),
            self.rc_osc_prog_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig0Clr(pub u32);
impl OscConfig0Clr {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0Clr {
    #[inline(always)]
    fn default() -> OscConfig0Clr {
        OscConfig0Clr(0)
    }
}
impl core::fmt::Debug for OscConfig0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig0Clr")
            .field("start", &self.start())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("invert", &self.invert())
            .field("rc_osc_prog", &self.rc_osc_prog())
            .field("hyst_plus", &self.hyst_plus())
            .field("hyst_minus", &self.hyst_minus())
            .field("rc_osc_prog_cur", &self.rc_osc_prog_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig0Clr {{ start: {=bool:?}, enable: {=bool:?}, bypass: {=bool:?}, invert: {=bool:?}, rc_osc_prog: {=u8:?}, hyst_plus: {=u8:?}, hyst_minus: {=u8:?}, rc_osc_prog_cur: {=u8:?} }}",
            self.start(),
            self.enable(),
            self.bypass(),
            self.invert(),
            self.rc_osc_prog(),
            self.hyst_plus(),
            self.hyst_minus(),
            self.rc_osc_prog_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig0Set(pub u32);
impl OscConfig0Set {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0Set {
    #[inline(always)]
    fn default() -> OscConfig0Set {
        OscConfig0Set(0)
    }
}
impl core::fmt::Debug for OscConfig0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig0Set")
            .field("start", &self.start())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("invert", &self.invert())
            .field("rc_osc_prog", &self.rc_osc_prog())
            .field("hyst_plus", &self.hyst_plus())
            .field("hyst_minus", &self.hyst_minus())
            .field("rc_osc_prog_cur", &self.rc_osc_prog_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig0Set {{ start: {=bool:?}, enable: {=bool:?}, bypass: {=bool:?}, invert: {=bool:?}, rc_osc_prog: {=u8:?}, hyst_plus: {=u8:?}, hyst_minus: {=u8:?}, rc_osc_prog_cur: {=u8:?} }}",
            self.start(),
            self.enable(),
            self.bypass(),
            self.invert(),
            self.rc_osc_prog(),
            self.hyst_plus(),
            self.hyst_minus(),
            self.rc_osc_prog_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig0Tog(pub u32);
impl OscConfig0Tog {
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start/stop bit for the RC tuning calculation logic. If stopped the tuning logic is reset."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the tuning logic to calculate new RC tuning values"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bypasses any calculated RC tuning value and uses the programmed register value."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the stepping of the calculated RC tuning value."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RC osc. tuning values."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RC osc. tuning values."]
    #[inline(always)]
    pub const fn set_rc_osc_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Positive hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Negative hysteresis value"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value"]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_osc_prog_cur(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_rc_osc_prog_cur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OscConfig0Tog {
    #[inline(always)]
    fn default() -> OscConfig0Tog {
        OscConfig0Tog(0)
    }
}
impl core::fmt::Debug for OscConfig0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig0Tog")
            .field("start", &self.start())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("invert", &self.invert())
            .field("rc_osc_prog", &self.rc_osc_prog())
            .field("hyst_plus", &self.hyst_plus())
            .field("hyst_minus", &self.hyst_minus())
            .field("rc_osc_prog_cur", &self.rc_osc_prog_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig0Tog {{ start: {=bool:?}, enable: {=bool:?}, bypass: {=bool:?}, invert: {=bool:?}, rc_osc_prog: {=u8:?}, hyst_plus: {=u8:?}, hyst_minus: {=u8:?}, rc_osc_prog_cur: {=u8:?} }}",
            self.start(),
            self.enable(),
            self.bypass(),
            self.invert(),
            self.rc_osc_prog(),
            self.hyst_plus(),
            self.hyst_minus(),
            self.rc_osc_prog_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig1(pub u32);
impl OscConfig1 {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1 {
    #[inline(always)]
    fn default() -> OscConfig1 {
        OscConfig1(0)
    }
}
impl core::fmt::Debug for OscConfig1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig1")
            .field("count_rc_trg", &self.count_rc_trg())
            .field("count_rc_cur", &self.count_rc_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig1 {{ count_rc_trg: {=u16:?}, count_rc_cur: {=u16:?} }}",
            self.count_rc_trg(),
            self.count_rc_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig1Clr(pub u32);
impl OscConfig1Clr {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1Clr {
    #[inline(always)]
    fn default() -> OscConfig1Clr {
        OscConfig1Clr(0)
    }
}
impl core::fmt::Debug for OscConfig1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig1Clr")
            .field("count_rc_trg", &self.count_rc_trg())
            .field("count_rc_cur", &self.count_rc_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig1Clr {{ count_rc_trg: {=u16:?}, count_rc_cur: {=u16:?} }}",
            self.count_rc_trg(),
            self.count_rc_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig1Set(pub u32);
impl OscConfig1Set {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1Set {
    #[inline(always)]
    fn default() -> OscConfig1Set {
        OscConfig1Set(0)
    }
}
impl core::fmt::Debug for OscConfig1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig1Set")
            .field("count_rc_trg", &self.count_rc_trg())
            .field("count_rc_cur", &self.count_rc_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig1Set {{ count_rc_trg: {=u16:?}, count_rc_cur: {=u16:?} }}",
            self.count_rc_trg(),
            self.count_rc_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig1Tog(pub u32);
impl OscConfig1Tog {
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub const fn set_count_rc_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "The current tuning value in use."]
    #[must_use]
    #[inline(always)]
    pub const fn count_rc_cur(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current tuning value in use."]
    #[inline(always)]
    pub const fn set_count_rc_cur(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for OscConfig1Tog {
    #[inline(always)]
    fn default() -> OscConfig1Tog {
        OscConfig1Tog(0)
    }
}
impl core::fmt::Debug for OscConfig1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig1Tog")
            .field("count_rc_trg", &self.count_rc_trg())
            .field("count_rc_cur", &self.count_rc_cur())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig1Tog {{ count_rc_trg: {=u16:?}, count_rc_cur: {=u16:?} }}",
            self.count_rc_trg(),
            self.count_rc_cur()
        )
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig2(pub u32);
impl OscConfig2 {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2 {
    #[inline(always)]
    fn default() -> OscConfig2 {
        OscConfig2(0)
    }
}
impl core::fmt::Debug for OscConfig2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig2")
            .field("count_1m_trg", &self.count_1m_trg())
            .field("enable_1m", &self.enable_1m())
            .field("mux_1m", &self.mux_1m())
            .field("clk_1m_err_fl", &self.clk_1m_err_fl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig2 {{ count_1m_trg: {=u16:?}, enable_1m: {=bool:?}, mux_1m: {=bool:?}, clk_1m_err_fl: {=bool:?} }}",
            self.count_1m_trg(),
            self.enable_1m(),
            self.mux_1m(),
            self.clk_1m_err_fl()
        )
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig2Clr(pub u32);
impl OscConfig2Clr {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2Clr {
    #[inline(always)]
    fn default() -> OscConfig2Clr {
        OscConfig2Clr(0)
    }
}
impl core::fmt::Debug for OscConfig2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig2Clr")
            .field("count_1m_trg", &self.count_1m_trg())
            .field("enable_1m", &self.enable_1m())
            .field("mux_1m", &self.mux_1m())
            .field("clk_1m_err_fl", &self.clk_1m_err_fl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig2Clr {{ count_1m_trg: {=u16:?}, enable_1m: {=bool:?}, mux_1m: {=bool:?}, clk_1m_err_fl: {=bool:?} }}",
            self.count_1m_trg(),
            self.enable_1m(),
            self.mux_1m(),
            self.clk_1m_err_fl()
        )
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig2Set(pub u32);
impl OscConfig2Set {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2Set {
    #[inline(always)]
    fn default() -> OscConfig2Set {
        OscConfig2Set(0)
    }
}
impl core::fmt::Debug for OscConfig2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig2Set")
            .field("count_1m_trg", &self.count_1m_trg())
            .field("enable_1m", &self.enable_1m())
            .field("mux_1m", &self.mux_1m())
            .field("clk_1m_err_fl", &self.clk_1m_err_fl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig2Set {{ count_1m_trg: {=u16:?}, enable_1m: {=bool:?}, mux_1m: {=bool:?}, clk_1m_err_fl: {=bool:?} }}",
            self.count_1m_trg(),
            self.enable_1m(),
            self.mux_1m(),
            self.clk_1m_err_fl()
        )
    }
}
#[doc = "XTAL OSC Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscConfig2Tog(pub u32);
impl OscConfig2Tog {
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_trg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The target count used to tune the 1MHz clock frequency"]
    #[inline(always)]
    pub const fn set_count_1m_trg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_1m(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the 1MHz clock output. 0 - disabled; 1 - enabled."]
    #[inline(always)]
    pub const fn set_enable_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mux the corrected or uncorrected 1MHz clock to the output"]
    #[inline(always)]
    pub const fn set_mux_1m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_1m_err_fl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicates that the count_1m count wasn't reached within 1 32kHz period"]
    #[inline(always)]
    pub const fn set_clk_1m_err_fl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OscConfig2Tog {
    #[inline(always)]
    fn default() -> OscConfig2Tog {
        OscConfig2Tog(0)
    }
}
impl core::fmt::Debug for OscConfig2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OscConfig2Tog")
            .field("count_1m_trg", &self.count_1m_trg())
            .field("enable_1m", &self.enable_1m())
            .field("mux_1m", &self.mux_1m())
            .field("clk_1m_err_fl", &self.clk_1m_err_fl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OscConfig2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OscConfig2Tog {{ count_1m_trg: {=u16:?}, enable_1m: {=bool:?}, mux_1m: {=bool:?}, clk_1m_err_fl: {=bool:?} }}",
            self.count_1m_trg(),
            self.enable_1m(),
            self.mux_1m(),
            self.clk_1m_err_fl()
        )
    }
}
