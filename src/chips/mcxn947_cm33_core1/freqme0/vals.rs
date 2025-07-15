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
