#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "FREQME."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqme {
    ptr: *mut u8,
}
unsafe impl Send for Freqme {}
unsafe impl Sync for Freqme {}
impl Freqme {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control (in Read mode)."]
    #[inline(always)]
    pub const fn ctrl_r(self) -> crate::pac::common::Reg<CtrlR, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control (in Write mode)."]
    #[inline(always)]
    pub const fn ctrl_w(self) -> crate::pac::common::Reg<CtrlW, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Status."]
    #[inline(always)]
    pub const fn ctrlstat(self) -> crate::pac::common::Reg<Ctrlstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Minimum."]
    #[inline(always)]
    pub const fn min(self) -> crate::pac::common::Reg<Min, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Maximum."]
    #[inline(always)]
    pub const fn max(self) -> crate::pac::common::Reg<Max, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Control (in Read mode)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlR(pub u32);
impl CtrlR {
    #[doc = "Indicates the measurement result-either the target clock counter value (for Frequency Measurement mode) or pulse width measurement (for Pulse Width Measurement mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn result(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Indicates the measurement result-either the target clock counter value (for Frequency Measurement mode) or pulse width measurement (for Pulse Width Measurement mode)."]
    #[inline(always)]
    pub const fn set_result(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Measurement In Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> CtrlRMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        CtrlRMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measurement In Progress."]
    #[inline(always)]
    pub const fn set_measure_in_progress(&mut self, val: CtrlRMeasureInProgress) {
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
#[doc = "Control (in Write mode)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlW(pub u32);
impl CtrlW {
    #[doc = "Reference Clock Scaling Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_scale(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reference Clock Scaling Factor."]
    #[inline(always)]
    pub const fn set_ref_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Pulse Width Measurement Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_mode(&self) -> CtrlWPulseMode {
        let val = (self.0 >> 8usize) & 0x01;
        CtrlWPulseMode::from_bits(val as u8)
    }
    #[doc = "Pulse Width Measurement Mode Select."]
    #[inline(always)]
    pub const fn set_pulse_mode(&mut self, val: CtrlWPulseMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pulse Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_pol(&self) -> CtrlWPulsePol {
        let val = (self.0 >> 9usize) & 0x01;
        CtrlWPulsePol::from_bits(val as u8)
    }
    #[doc = "Pulse Polarity."]
    #[inline(always)]
    pub const fn set_pulse_pol(&mut self, val: CtrlWPulsePol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Less Than Minimum Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lt_min_int_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Less Than Minimum Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lt_min_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Greater Than Maximum Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gt_max_int_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Greater Than Maximum Interrupt Enable."]
    #[inline(always)]
    pub const fn set_gt_max_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Result Ready Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn result_ready_int_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Result Ready Interrupt Enable."]
    #[inline(always)]
    pub const fn set_result_ready_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Continuous Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn continuous_mode_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Mode Enable."]
    #[inline(always)]
    pub const fn set_continuous_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Measurement In Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> CtrlWMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        CtrlWMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measurement In Progress."]
    #[inline(always)]
    pub const fn set_measure_in_progress(&mut self, val: CtrlWMeasureInProgress) {
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
#[doc = "Control Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrlstat(pub u32);
impl Ctrlstat {
    #[doc = "Reference Scale."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_scale(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reference Scale."]
    #[inline(always)]
    pub const fn set_ref_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Pulse Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_mode(&self) -> CtrlstatPulseMode {
        let val = (self.0 >> 8usize) & 0x01;
        CtrlstatPulseMode::from_bits(val as u8)
    }
    #[doc = "Pulse Mode."]
    #[inline(always)]
    pub const fn set_pulse_mode(&mut self, val: CtrlstatPulseMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pulse Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_pol(&self) -> CtrlstatPulsePol {
        let val = (self.0 >> 9usize) & 0x01;
        CtrlstatPulsePol::from_bits(val as u8)
    }
    #[doc = "Pulse Polarity."]
    #[inline(always)]
    pub const fn set_pulse_pol(&mut self, val: CtrlstatPulsePol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Less Than Minimum Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lt_min_int_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Less Than Minimum Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lt_min_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Greater Than Maximum Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gt_max_int_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Greater Than Maximum Interrupt Enable."]
    #[inline(always)]
    pub const fn set_gt_max_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Result Ready Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn result_ready_int_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Result Ready Interrupt Enable."]
    #[inline(always)]
    pub const fn set_result_ready_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Less Than Minimum Results Status."]
    #[must_use]
    #[inline(always)]
    pub const fn lt_min_stat(&self) -> LtMinStat {
        let val = (self.0 >> 24usize) & 0x01;
        LtMinStat::from_bits(val as u8)
    }
    #[doc = "Less Than Minimum Results Status."]
    #[inline(always)]
    pub const fn set_lt_min_stat(&mut self, val: LtMinStat) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Greater Than Maximum Result Status."]
    #[must_use]
    #[inline(always)]
    pub const fn gt_max_stat(&self) -> GtMaxStat {
        let val = (self.0 >> 25usize) & 0x01;
        GtMaxStat::from_bits(val as u8)
    }
    #[doc = "Greater Than Maximum Result Status."]
    #[inline(always)]
    pub const fn set_gt_max_stat(&mut self, val: GtMaxStat) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Result Ready Status."]
    #[must_use]
    #[inline(always)]
    pub const fn result_ready_stat(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Result Ready Status."]
    #[inline(always)]
    pub const fn set_result_ready_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Continuous Mode Enable Status."]
    #[must_use]
    #[inline(always)]
    pub const fn continuous_mode_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Mode Enable Status."]
    #[inline(always)]
    pub const fn set_continuous_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Measurement in Progress Status."]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> CtrlstatMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        CtrlstatMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measurement in Progress Status."]
    #[inline(always)]
    pub const fn set_measure_in_progress(&mut self, val: CtrlstatMeasureInProgress) {
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
#[doc = "Maximum."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Max(pub u32);
impl Max {
    #[doc = "Maximum Value."]
    #[must_use]
    #[inline(always)]
    pub const fn max_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Maximum Value."]
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
#[doc = "Minimum."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Min(pub u32);
impl Min {
    #[doc = "Minimum Value."]
    #[must_use]
    #[inline(always)]
    pub const fn min_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Minimum Value."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlRMeasureInProgress {
    #[doc = "Complete."]
    CycleDone = 0x0,
    #[doc = "In progress."]
    InProgress = 0x01,
}
impl CtrlRMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlRMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlRMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> CtrlRMeasureInProgress {
        CtrlRMeasureInProgress::from_bits(val)
    }
}
impl From<CtrlRMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: CtrlRMeasureInProgress) -> u8 {
        CtrlRMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWMeasureInProgress {
    #[doc = "Terminates measurement."]
    ForceTerminate = 0x0,
    #[doc = "Initiates measurement."]
    InitiateAFreqmeCycle = 0x01,
}
impl CtrlWMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> CtrlWMeasureInProgress {
        CtrlWMeasureInProgress::from_bits(val)
    }
}
impl From<CtrlWMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: CtrlWMeasureInProgress) -> u8 {
        CtrlWMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWPulseMode {
    #[doc = "Frequency Measurement mode."]
    FreqMeMode = 0x0,
    #[doc = "Pulse Width Measurement mode."]
    PulseMeMode = 0x01,
}
impl CtrlWPulseMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWPulseMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWPulseMode {
    #[inline(always)]
    fn from(val: u8) -> CtrlWPulseMode {
        CtrlWPulseMode::from_bits(val)
    }
}
impl From<CtrlWPulseMode> for u8 {
    #[inline(always)]
    fn from(val: CtrlWPulseMode) -> u8 {
        CtrlWPulseMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWPulsePol {
    #[doc = "High period."]
    HighPeriod = 0x0,
    #[doc = "Low period."]
    LowPeriod = 0x01,
}
impl CtrlWPulsePol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWPulsePol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWPulsePol {
    #[inline(always)]
    fn from(val: u8) -> CtrlWPulsePol {
        CtrlWPulsePol::from_bits(val)
    }
}
impl From<CtrlWPulsePol> for u8 {
    #[inline(always)]
    fn from(val: CtrlWPulsePol) -> u8 {
        CtrlWPulsePol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatMeasureInProgress {
    #[doc = "Not in progress."]
    Idle = 0x0,
    #[doc = "In progress."]
    Ongoing = 0x01,
}
impl CtrlstatMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatMeasureInProgress {
        CtrlstatMeasureInProgress::from_bits(val)
    }
}
impl From<CtrlstatMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatMeasureInProgress) -> u8 {
        CtrlstatMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatPulseMode {
    #[doc = "Frequency Measurement mode."]
    Freq = 0x0,
    #[doc = "Pulse Width Measurement mode."]
    Pulse = 0x01,
}
impl CtrlstatPulseMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatPulseMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatPulseMode {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatPulseMode {
        CtrlstatPulseMode::from_bits(val)
    }
}
impl From<CtrlstatPulseMode> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatPulseMode) -> u8 {
        CtrlstatPulseMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatPulsePol {
    #[doc = "High period."]
    High = 0x0,
    #[doc = "Low period."]
    Low = 0x01,
}
impl CtrlstatPulsePol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatPulsePol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatPulsePol {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatPulsePol {
        CtrlstatPulsePol::from_bits(val)
    }
}
impl From<CtrlstatPulsePol> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatPulsePol) -> u8 {
        CtrlstatPulsePol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GtMaxStat {
    #[doc = "Less than MAX\\[MAX_VALUE\\]."]
    InRange = 0x0,
    #[doc = "Greater than MAX\\[MAX_VALUE\\]."]
    GtMax = 0x01,
}
impl GtMaxStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GtMaxStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GtMaxStat {
    #[inline(always)]
    fn from(val: u8) -> GtMaxStat {
        GtMaxStat::from_bits(val)
    }
}
impl From<GtMaxStat> for u8 {
    #[inline(always)]
    fn from(val: GtMaxStat) -> u8 {
        GtMaxStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LtMinStat {
    #[doc = "Greater than MIN\\[MIN_VALUE\\]."]
    InRange = 0x0,
    #[doc = "Less than MIN\\[MIN_VALUE\\]."]
    LtMin = 0x01,
}
impl LtMinStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LtMinStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LtMinStat {
    #[inline(always)]
    fn from(val: u8) -> LtMinStat {
        LtMinStat::from_bits(val)
    }
}
impl From<LtMinStat> for u8 {
    #[inline(always)]
    fn from(val: LtMinStat) -> u8 {
        LtMinStat::to_bits(val)
    }
}
