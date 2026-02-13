#[doc = "Control (in Read mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlR(pub u32);
impl CtrlR {
    #[doc = "Indicates the measurement result-either the target clock counter value (for Frequency Measurement mode) or pulse width measurement (for Pulse Width Measurement mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn result(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Indicates the measurement result-either the target clock counter value (for Frequency Measurement mode) or pulse width measurement (for Pulse Width Measurement mode)"]
    #[inline(always)]
    pub const fn set_result(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Measurement In Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::CtrlRMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlRMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measurement In Progress"]
    #[inline(always)]
    pub const fn set_measure_in_progress(&mut self, val: super::vals::CtrlRMeasureInProgress) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlR {
    #[inline(always)]
    fn default() -> CtrlR {
        CtrlR(0)
    }
}
impl core::fmt::Debug for CtrlR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlR")
            .field("result", &self.result())
            .field("measure_in_progress", &self.measure_in_progress())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlR {{ result: {=u32:?}, measure_in_progress: {:?} }}",
            self.result(),
            self.measure_in_progress()
        )
    }
}
#[doc = "Control (in Write mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlW(pub u32);
impl CtrlW {
    #[doc = "Reference Clock Scaling Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_scale(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reference Clock Scaling Factor"]
    #[inline(always)]
    pub const fn set_ref_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Pulse Width Measurement Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_mode(&self) -> super::vals::CtrlWPulseMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CtrlWPulseMode::from_bits(val as u8)
    }
    #[doc = "Pulse Width Measurement Mode Select"]
    #[inline(always)]
    pub const fn set_pulse_mode(&mut self, val: super::vals::CtrlWPulseMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pulse Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_pol(&self) -> super::vals::CtrlWPulsePol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CtrlWPulsePol::from_bits(val as u8)
    }
    #[doc = "Pulse Polarity"]
    #[inline(always)]
    pub const fn set_pulse_pol(&mut self, val: super::vals::CtrlWPulsePol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Less Than Minimum Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lt_min_int_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Less Than Minimum Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lt_min_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Greater Than Maximum Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gt_max_int_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Greater Than Maximum Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gt_max_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Result Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn result_ready_int_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Result Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_result_ready_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Continuous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn continuous_mode_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Mode Enable"]
    #[inline(always)]
    pub const fn set_continuous_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Measurement In Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::CtrlWMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlWMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measurement In Progress"]
    #[inline(always)]
    pub const fn set_measure_in_progress(&mut self, val: super::vals::CtrlWMeasureInProgress) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlW {
    #[inline(always)]
    fn default() -> CtrlW {
        CtrlW(0)
    }
}
impl core::fmt::Debug for CtrlW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlW")
            .field("ref_scale", &self.ref_scale())
            .field("pulse_mode", &self.pulse_mode())
            .field("pulse_pol", &self.pulse_pol())
            .field("lt_min_int_en", &self.lt_min_int_en())
            .field("gt_max_int_en", &self.gt_max_int_en())
            .field("result_ready_int_en", &self.result_ready_int_en())
            .field("continuous_mode_en", &self.continuous_mode_en())
            .field("measure_in_progress", &self.measure_in_progress())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlW {{ ref_scale: {=u8:?}, pulse_mode: {:?}, pulse_pol: {:?}, lt_min_int_en: {=bool:?}, gt_max_int_en: {=bool:?}, result_ready_int_en: {=bool:?}, continuous_mode_en: {=bool:?}, measure_in_progress: {:?} }}",
            self.ref_scale(),
            self.pulse_mode(),
            self.pulse_pol(),
            self.lt_min_int_en(),
            self.gt_max_int_en(),
            self.result_ready_int_en(),
            self.continuous_mode_en(),
            self.measure_in_progress()
        )
    }
}
#[doc = "Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrlstat(pub u32);
impl Ctrlstat {
    #[doc = "Reference Scale"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_scale(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reference Scale"]
    #[inline(always)]
    pub const fn set_ref_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Pulse Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_mode(&self) -> super::vals::CtrlstatPulseMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CtrlstatPulseMode::from_bits(val as u8)
    }
    #[doc = "Pulse Mode"]
    #[inline(always)]
    pub const fn set_pulse_mode(&mut self, val: super::vals::CtrlstatPulseMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pulse Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_pol(&self) -> super::vals::CtrlstatPulsePol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CtrlstatPulsePol::from_bits(val as u8)
    }
    #[doc = "Pulse Polarity"]
    #[inline(always)]
    pub const fn set_pulse_pol(&mut self, val: super::vals::CtrlstatPulsePol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Less Than Minimum Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lt_min_int_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Less Than Minimum Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lt_min_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Greater Than Maximum Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gt_max_int_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Greater Than Maximum Interrupt Enable"]
    #[inline(always)]
    pub const fn set_gt_max_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Result Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn result_ready_int_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Result Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_result_ready_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Less Than Minimum Results Status"]
    #[must_use]
    #[inline(always)]
    pub const fn lt_min_stat(&self) -> super::vals::LtMinStat {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::LtMinStat::from_bits(val as u8)
    }
    #[doc = "Less Than Minimum Results Status"]
    #[inline(always)]
    pub const fn set_lt_min_stat(&mut self, val: super::vals::LtMinStat) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Greater Than Maximum Result Status"]
    #[must_use]
    #[inline(always)]
    pub const fn gt_max_stat(&self) -> super::vals::GtMaxStat {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::GtMaxStat::from_bits(val as u8)
    }
    #[doc = "Greater Than Maximum Result Status"]
    #[inline(always)]
    pub const fn set_gt_max_stat(&mut self, val: super::vals::GtMaxStat) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Result Ready Status"]
    #[must_use]
    #[inline(always)]
    pub const fn result_ready_stat(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Result Ready Status"]
    #[inline(always)]
    pub const fn set_result_ready_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Continuous Mode Enable Status"]
    #[must_use]
    #[inline(always)]
    pub const fn continuous_mode_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Mode Enable Status"]
    #[inline(always)]
    pub const fn set_continuous_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Measurement in Progress Status"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::CtrlstatMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlstatMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measurement in Progress Status"]
    #[inline(always)]
    pub const fn set_measure_in_progress(&mut self, val: super::vals::CtrlstatMeasureInProgress) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrlstat {
    #[inline(always)]
    fn default() -> Ctrlstat {
        Ctrlstat(0)
    }
}
impl core::fmt::Debug for Ctrlstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrlstat")
            .field("ref_scale", &self.ref_scale())
            .field("pulse_mode", &self.pulse_mode())
            .field("pulse_pol", &self.pulse_pol())
            .field("lt_min_int_en", &self.lt_min_int_en())
            .field("gt_max_int_en", &self.gt_max_int_en())
            .field("result_ready_int_en", &self.result_ready_int_en())
            .field("lt_min_stat", &self.lt_min_stat())
            .field("gt_max_stat", &self.gt_max_stat())
            .field("result_ready_stat", &self.result_ready_stat())
            .field("continuous_mode_en", &self.continuous_mode_en())
            .field("measure_in_progress", &self.measure_in_progress())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrlstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrlstat {{ ref_scale: {=u8:?}, pulse_mode: {:?}, pulse_pol: {:?}, lt_min_int_en: {=bool:?}, gt_max_int_en: {=bool:?}, result_ready_int_en: {=bool:?}, lt_min_stat: {:?}, gt_max_stat: {:?}, result_ready_stat: {=bool:?}, continuous_mode_en: {=bool:?}, measure_in_progress: {:?} }}",
            self.ref_scale(),
            self.pulse_mode(),
            self.pulse_pol(),
            self.lt_min_int_en(),
            self.gt_max_int_en(),
            self.result_ready_int_en(),
            self.lt_min_stat(),
            self.gt_max_stat(),
            self.result_ready_stat(),
            self.continuous_mode_en(),
            self.measure_in_progress()
        )
    }
}
#[doc = "Maximum"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Max(pub u32);
impl Max {
    #[doc = "Maximum Value"]
    #[must_use]
    #[inline(always)]
    pub const fn max_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Maximum Value"]
    #[inline(always)]
    pub const fn set_max_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for Max {
    #[inline(always)]
    fn default() -> Max {
        Max(0)
    }
}
impl core::fmt::Debug for Max {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Max")
            .field("max_value", &self.max_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Max {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Max {{ max_value: {=u32:?} }}", self.max_value())
    }
}
#[doc = "Minimum"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Min(pub u32);
impl Min {
    #[doc = "Minimum Value"]
    #[must_use]
    #[inline(always)]
    pub const fn min_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub const fn set_min_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for Min {
    #[inline(always)]
    fn default() -> Min {
        Min(0)
    }
}
impl core::fmt::Debug for Min {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Min")
            .field("min_value", &self.min_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Min {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Min {{ min_value: {=u32:?} }}", self.min_value())
    }
}
