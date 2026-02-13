#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmectrlRMeasureInProgress {
    #[doc = "Process complete. Measurement cycle is complete. The results are ready in the RESULT field."]
    CYCLE_DONE = 0x0,
    #[doc = "In Progress. Measurement cycle is in progress."]
    IN_PROGRESS = 0x01,
}
impl FreqmectrlRMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmectrlRMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmectrlRMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> FreqmectrlRMeasureInProgress {
        FreqmectrlRMeasureInProgress::from_bits(val)
    }
}
impl From<FreqmectrlRMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: FreqmectrlRMeasureInProgress) -> u8 {
        FreqmectrlRMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmectrlWMeasureInProgress {
    #[doc = "Force Terminate. Forces the termination of any measurement cycle currently in progress and resets RESULT or just resets RESULT if in idle."]
    FORCE_TERMINATE = 0x0,
    #[doc = "Initiates Measurement Cycle. Initiates frequency or pulse width measurement process. Hardware clears the MEASURE_IN_PROGRESS bit when the measurement cycle completes. A new measurement starts if there is an active measurement in progress."]
    INITIATE_A_FREQME_CYCLE = 0x01,
}
impl FreqmectrlWMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmectrlWMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmectrlWMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> FreqmectrlWMeasureInProgress {
        FreqmectrlWMeasureInProgress::from_bits(val)
    }
}
impl From<FreqmectrlWMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: FreqmectrlWMeasureInProgress) -> u8 {
        FreqmectrlWMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PulseMode {
    #[doc = "Frequency Measurement Mode. FREQMECTRL works in a Frequency Measurement mode. Once the measurement starts (real count start is aligned at rising edge arrival on reference clock), the target counter increments by the target clock until the reference counter running by the reference clock reaches the count end point selected by REF_SCALE."]
    FREQ_ME_MODE = 0x0,
    #[doc = "Pulse Width Measurement mode. FREQMECTRL works in a Pulse Width Measurement mode, measuring the high or low period of reference clock input selected by PULSE_POL. The target counter starts incrementing by the target clock once a corresponding trigger edge (rising edge for high period measurement and falling edge for low period) occurs."]
    PULSE_ME_MODE = 0x01,
}
impl PulseMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PulseMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PulseMode {
    #[inline(always)]
    fn from(val: u8) -> PulseMode {
        PulseMode::from_bits(val)
    }
}
impl From<PulseMode> for u8 {
    #[inline(always)]
    fn from(val: PulseMode) -> u8 {
        PulseMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PulsePol {
    #[doc = "High Period. High period of reference clock is measured in Pulse Width Measurement mode triggered by the rising edge on the reference clock input."]
    HIGH_PERIOD = 0x0,
    #[doc = "Low Period. Low period of reference clock is measured in Pulse Width Measurement mode triggered by the falling edge on the reference clock input."]
    LOW_PERIOD = 0x01,
}
impl PulsePol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PulsePol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PulsePol {
    #[inline(always)]
    fn from(val: u8) -> PulsePol {
        PulsePol::from_bits(val)
    }
}
impl From<PulsePol> for u8 {
    #[inline(always)]
    fn from(val: PulsePol) -> u8 {
        PulsePol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefScale {
    #[doc = "Count cycle = 2^0 = 1"]
    COUNTCYCLE_1 = 0x0,
    #[doc = "Count cycle = 2^1 = 2"]
    COUNTCYCLE_2 = 0x01,
    #[doc = "Count cycle = 2^4 = 4"]
    COUNTCYCLE_4 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Count cycle = 2^31 = 2,147,483,648"]
    COUNTCYCLE_31 = 0x1f,
}
impl RefScale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefScale {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefScale {
    #[inline(always)]
    fn from(val: u8) -> RefScale {
        RefScale::from_bits(val)
    }
}
impl From<RefScale> for u8 {
    #[inline(always)]
    fn from(val: RefScale) -> u8 {
        RefScale::to_bits(val)
    }
}
