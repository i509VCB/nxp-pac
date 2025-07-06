#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlRMeasureInProgress {
    #[doc = "Complete"]
    CYCLE_DONE = 0x0,
    #[doc = "In progress"]
    IN_PROGRESS = 0x01,
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
pub enum CtrlWContinuousModeEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CtrlWContinuousModeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWContinuousModeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWContinuousModeEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlWContinuousModeEn {
        CtrlWContinuousModeEn::from_bits(val)
    }
}
impl From<CtrlWContinuousModeEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlWContinuousModeEn) -> u8 {
        CtrlWContinuousModeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWGtMaxIntEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CtrlWGtMaxIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWGtMaxIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWGtMaxIntEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlWGtMaxIntEn {
        CtrlWGtMaxIntEn::from_bits(val)
    }
}
impl From<CtrlWGtMaxIntEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlWGtMaxIntEn) -> u8 {
        CtrlWGtMaxIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWLtMinIntEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CtrlWLtMinIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWLtMinIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWLtMinIntEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlWLtMinIntEn {
        CtrlWLtMinIntEn::from_bits(val)
    }
}
impl From<CtrlWLtMinIntEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlWLtMinIntEn) -> u8 {
        CtrlWLtMinIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWMeasureInProgress {
    #[doc = "Terminates measurement"]
    FORCE_TERMINATE = 0x0,
    #[doc = "Initiates measurement"]
    INITIATE_A_FREQME_CYCLE = 0x01,
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
    #[doc = "Frequency Measurement mode"]
    FREQ_ME_MODE = 0x0,
    #[doc = "Pulse Width Measurement mode"]
    PULSE_ME_MODE = 0x01,
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
    #[doc = "High period"]
    HIGH_PERIOD = 0x0,
    #[doc = "Low period"]
    LOW_PERIOD = 0x01,
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
pub enum CtrlWResultReadyIntEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CtrlWResultReadyIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWResultReadyIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWResultReadyIntEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlWResultReadyIntEn {
        CtrlWResultReadyIntEn::from_bits(val)
    }
}
impl From<CtrlWResultReadyIntEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlWResultReadyIntEn) -> u8 {
        CtrlWResultReadyIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatContinuousModeEn {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl CtrlstatContinuousModeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatContinuousModeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatContinuousModeEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatContinuousModeEn {
        CtrlstatContinuousModeEn::from_bits(val)
    }
}
impl From<CtrlstatContinuousModeEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatContinuousModeEn) -> u8 {
        CtrlstatContinuousModeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatGtMaxIntEn {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl CtrlstatGtMaxIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatGtMaxIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatGtMaxIntEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatGtMaxIntEn {
        CtrlstatGtMaxIntEn::from_bits(val)
    }
}
impl From<CtrlstatGtMaxIntEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatGtMaxIntEn) -> u8 {
        CtrlstatGtMaxIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatLtMinIntEn {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl CtrlstatLtMinIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatLtMinIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatLtMinIntEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatLtMinIntEn {
        CtrlstatLtMinIntEn::from_bits(val)
    }
}
impl From<CtrlstatLtMinIntEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatLtMinIntEn) -> u8 {
        CtrlstatLtMinIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlstatMeasureInProgress {
    #[doc = "Not in progress"]
    IDLE = 0x0,
    #[doc = "In progress"]
    ONGOING = 0x01,
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
    #[doc = "Frequency Measurement mode"]
    FREQ = 0x0,
    #[doc = "Pulse Width Measurement mode"]
    PULSE = 0x01,
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
    #[doc = "High period"]
    HIGH = 0x0,
    #[doc = "Low period"]
    LOW = 0x01,
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
pub enum CtrlstatResultReadyIntEn {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl CtrlstatResultReadyIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlstatResultReadyIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlstatResultReadyIntEn {
    #[inline(always)]
    fn from(val: u8) -> CtrlstatResultReadyIntEn {
        CtrlstatResultReadyIntEn::from_bits(val)
    }
}
impl From<CtrlstatResultReadyIntEn> for u8 {
    #[inline(always)]
    fn from(val: CtrlstatResultReadyIntEn) -> u8 {
        CtrlstatResultReadyIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GtMaxStat {
    #[doc = "Less than MAX\\[MAX_VALUE\\]"]
    IN_RANGE = 0x0,
    #[doc = "Greater than MAX\\[MAX_VALUE\\]"]
    GT_MAX = 0x01,
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
    #[doc = "Greater than MIN\\[MIN_VALUE\\]"]
    IN_RANGE = 0x0,
    #[doc = "Less than MIN\\[MIN_VALUE\\]"]
    LT_MIN = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResultReadyStat {
    #[doc = "Not complete"]
    NOT_COMPLETE = 0x0,
    #[doc = "Complete"]
    COMPLETE = 0x01,
}
impl ResultReadyStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResultReadyStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResultReadyStat {
    #[inline(always)]
    fn from(val: u8) -> ResultReadyStat {
        ResultReadyStat::from_bits(val)
    }
}
impl From<ResultReadyStat> for u8 {
    #[inline(always)]
    fn from(val: ResultReadyStat) -> u8 {
        ResultReadyStat::to_bits(val)
    }
}
