#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADCEN {
    #[doc = "ADC is disabled."]
    ADCEN_0 = 0x0,
    #[doc = "ADC is enabled."]
    ADCEN_1 = 0x01,
}
impl ADCEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADCEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADCEN {
    #[inline(always)]
    fn from(val: u8) -> ADCEN {
        ADCEN::from_bits(val)
    }
}
impl From<ADCEN> for u8 {
    #[inline(always)]
    fn from(val: ADCEN) -> u8 {
        ADCEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADC_ACTIVE {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    ADC_ACTIVE_0 = 0x0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    ADC_ACTIVE_1 = 0x01,
}
impl ADC_ACTIVE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADC_ACTIVE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADC_ACTIVE {
    #[inline(always)]
    fn from(val: u8) -> ADC_ACTIVE {
        ADC_ACTIVE::from_bits(val)
    }
}
impl From<ADC_ACTIVE> for u8 {
    #[inline(always)]
    fn from(val: ADC_ACTIVE) -> u8 {
        ADC_ACTIVE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CALOFS {
    #[doc = "Calibration function disabled."]
    CALOFS_0 = 0x0,
    #[doc = "Request for offset calibration function."]
    CALOFS_1 = 0x01,
}
impl CALOFS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CALOFS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CALOFS {
    #[inline(always)]
    fn from(val: u8) -> CALOFS {
        CALOFS::from_bits(val)
    }
}
impl From<CALOFS> for u8 {
    #[inline(always)]
    fn from(val: CALOFS) -> u8 {
        CALOFS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CALOFSI {
    #[doc = "Calibration Not Implemented."]
    CALOFSI_0 = 0x0,
    #[doc = "Calibration Implemented."]
    CALOFSI_1 = 0x01,
}
impl CALOFSI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CALOFSI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CALOFSI {
    #[inline(always)]
    fn from(val: u8) -> CALOFSI {
        CALOFSI::from_bits(val)
    }
}
impl From<CALOFSI> for u8 {
    #[inline(always)]
    fn from(val: CALOFSI) -> u8 {
        CALOFSI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAL_AVGS {
    #[doc = "Single conversion."]
    CAL_AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    CAL_AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    CAL_AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    CAL_AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    CAL_AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    CAL_AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    CAL_AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    CAL_AVGS_7 = 0x07,
}
impl CAL_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAL_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAL_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CAL_AVGS {
        CAL_AVGS::from_bits(val)
    }
}
impl From<CAL_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CAL_AVGS) -> u8 {
        CAL_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAL_RDY {
    #[doc = "Calibration is incomplete or hasn't been ran."]
    CAL_RDY_0 = 0x0,
    #[doc = "The ADC is calibrated."]
    CAL_RDY_1 = 0x01,
}
impl CAL_RDY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAL_RDY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAL_RDY {
    #[inline(always)]
    fn from(val: u8) -> CAL_RDY {
        CAL_RDY::from_bits(val)
    }
}
impl From<CAL_RDY> for u8 {
    #[inline(always)]
    fn from(val: CAL_RDY) -> u8 {
        CAL_RDY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAL_REQ {
    #[doc = "No request for auto-calibration has been made."]
    CAL_REQ_0 = 0x0,
    #[doc = "A request for auto-calibration has been made."]
    CAL_REQ_1 = 0x01,
}
impl CAL_REQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAL_REQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAL_REQ {
    #[inline(always)]
    fn from(val: u8) -> CAL_REQ {
        CAL_REQ::from_bits(val)
    }
}
impl From<CAL_REQ> for u8 {
    #[inline(always)]
    fn from(val: CAL_REQ) -> u8 {
        CAL_REQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDACT {
    #[doc = "No command is currently in progress."]
    CMDACT_0 = 0x0,
    #[doc = "Command 1 currently being executed."]
    CMDACT_1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    CMDACT_2 = 0x02,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_3 = 0x03,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_4 = 0x04,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_5 = 0x05,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_6 = 0x06,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_7 = 0x07,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_8 = 0x08,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CMDACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDACT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDACT {
    #[inline(always)]
    fn from(val: u8) -> CMDACT {
        CMDACT::from_bits(val)
    }
}
impl From<CMDACT> for u8 {
    #[inline(always)]
    fn from(val: CMDACT) -> u8 {
        CMDACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH10_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH10_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH10_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH10_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH10_AVGS {
        CMDH10_AVGS::from_bits(val)
    }
}
impl From<CMDH10_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH10_AVGS) -> u8 {
        CMDH10_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH10_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH10_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH10_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH10_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH10_LOOP {
        CMDH10_LOOP::from_bits(val)
    }
}
impl From<CMDH10_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH10_LOOP) -> u8 {
        CMDH10_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH10_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH10_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH10_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH10_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH10_LWI {
        CMDH10_LWI::from_bits(val)
    }
}
impl From<CMDH10_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH10_LWI) -> u8 {
        CMDH10_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH10_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH10_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH10_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH10_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH10_NEXT {
        CMDH10_NEXT::from_bits(val)
    }
}
impl From<CMDH10_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH10_NEXT) -> u8 {
        CMDH10_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH10_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH10_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH10_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH10_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH10_STS {
        CMDH10_STS::from_bits(val)
    }
}
impl From<CMDH10_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH10_STS) -> u8 {
        CMDH10_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH10_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH10_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH10_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH10_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH10_WAIT_TRIG {
        CMDH10_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH10_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH10_WAIT_TRIG) -> u8 {
        CMDH10_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH11_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH11_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH11_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH11_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH11_AVGS {
        CMDH11_AVGS::from_bits(val)
    }
}
impl From<CMDH11_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH11_AVGS) -> u8 {
        CMDH11_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH11_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH11_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH11_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH11_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH11_LOOP {
        CMDH11_LOOP::from_bits(val)
    }
}
impl From<CMDH11_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH11_LOOP) -> u8 {
        CMDH11_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH11_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH11_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH11_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH11_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH11_LWI {
        CMDH11_LWI::from_bits(val)
    }
}
impl From<CMDH11_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH11_LWI) -> u8 {
        CMDH11_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH11_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH11_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH11_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH11_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH11_NEXT {
        CMDH11_NEXT::from_bits(val)
    }
}
impl From<CMDH11_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH11_NEXT) -> u8 {
        CMDH11_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH11_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH11_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH11_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH11_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH11_STS {
        CMDH11_STS::from_bits(val)
    }
}
impl From<CMDH11_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH11_STS) -> u8 {
        CMDH11_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH11_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH11_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH11_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH11_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH11_WAIT_TRIG {
        CMDH11_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH11_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH11_WAIT_TRIG) -> u8 {
        CMDH11_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH12_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH12_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH12_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH12_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH12_AVGS {
        CMDH12_AVGS::from_bits(val)
    }
}
impl From<CMDH12_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH12_AVGS) -> u8 {
        CMDH12_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH12_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH12_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH12_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH12_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH12_LOOP {
        CMDH12_LOOP::from_bits(val)
    }
}
impl From<CMDH12_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH12_LOOP) -> u8 {
        CMDH12_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH12_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH12_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH12_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH12_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH12_LWI {
        CMDH12_LWI::from_bits(val)
    }
}
impl From<CMDH12_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH12_LWI) -> u8 {
        CMDH12_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH12_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH12_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH12_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH12_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH12_NEXT {
        CMDH12_NEXT::from_bits(val)
    }
}
impl From<CMDH12_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH12_NEXT) -> u8 {
        CMDH12_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH12_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH12_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH12_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH12_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH12_STS {
        CMDH12_STS::from_bits(val)
    }
}
impl From<CMDH12_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH12_STS) -> u8 {
        CMDH12_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH12_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH12_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH12_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH12_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH12_WAIT_TRIG {
        CMDH12_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH12_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH12_WAIT_TRIG) -> u8 {
        CMDH12_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH13_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH13_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH13_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH13_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH13_AVGS {
        CMDH13_AVGS::from_bits(val)
    }
}
impl From<CMDH13_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH13_AVGS) -> u8 {
        CMDH13_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH13_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH13_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH13_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH13_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH13_LOOP {
        CMDH13_LOOP::from_bits(val)
    }
}
impl From<CMDH13_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH13_LOOP) -> u8 {
        CMDH13_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH13_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH13_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH13_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH13_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH13_LWI {
        CMDH13_LWI::from_bits(val)
    }
}
impl From<CMDH13_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH13_LWI) -> u8 {
        CMDH13_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH13_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH13_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH13_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH13_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH13_NEXT {
        CMDH13_NEXT::from_bits(val)
    }
}
impl From<CMDH13_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH13_NEXT) -> u8 {
        CMDH13_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH13_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH13_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH13_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH13_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH13_STS {
        CMDH13_STS::from_bits(val)
    }
}
impl From<CMDH13_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH13_STS) -> u8 {
        CMDH13_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH13_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH13_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH13_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH13_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH13_WAIT_TRIG {
        CMDH13_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH13_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH13_WAIT_TRIG) -> u8 {
        CMDH13_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH14_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH14_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH14_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH14_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH14_AVGS {
        CMDH14_AVGS::from_bits(val)
    }
}
impl From<CMDH14_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH14_AVGS) -> u8 {
        CMDH14_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH14_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH14_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH14_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH14_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH14_LOOP {
        CMDH14_LOOP::from_bits(val)
    }
}
impl From<CMDH14_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH14_LOOP) -> u8 {
        CMDH14_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH14_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH14_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH14_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH14_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH14_LWI {
        CMDH14_LWI::from_bits(val)
    }
}
impl From<CMDH14_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH14_LWI) -> u8 {
        CMDH14_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH14_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH14_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH14_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH14_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH14_NEXT {
        CMDH14_NEXT::from_bits(val)
    }
}
impl From<CMDH14_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH14_NEXT) -> u8 {
        CMDH14_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH14_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH14_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH14_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH14_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH14_STS {
        CMDH14_STS::from_bits(val)
    }
}
impl From<CMDH14_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH14_STS) -> u8 {
        CMDH14_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH14_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH14_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH14_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH14_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH14_WAIT_TRIG {
        CMDH14_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH14_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH14_WAIT_TRIG) -> u8 {
        CMDH14_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH15_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH15_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH15_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH15_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH15_AVGS {
        CMDH15_AVGS::from_bits(val)
    }
}
impl From<CMDH15_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH15_AVGS) -> u8 {
        CMDH15_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH15_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH15_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH15_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH15_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH15_LOOP {
        CMDH15_LOOP::from_bits(val)
    }
}
impl From<CMDH15_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH15_LOOP) -> u8 {
        CMDH15_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH15_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH15_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH15_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH15_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH15_LWI {
        CMDH15_LWI::from_bits(val)
    }
}
impl From<CMDH15_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH15_LWI) -> u8 {
        CMDH15_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH15_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH15_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH15_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH15_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH15_NEXT {
        CMDH15_NEXT::from_bits(val)
    }
}
impl From<CMDH15_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH15_NEXT) -> u8 {
        CMDH15_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH15_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH15_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH15_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH15_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH15_STS {
        CMDH15_STS::from_bits(val)
    }
}
impl From<CMDH15_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH15_STS) -> u8 {
        CMDH15_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH15_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH15_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH15_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH15_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH15_WAIT_TRIG {
        CMDH15_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH15_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH15_WAIT_TRIG) -> u8 {
        CMDH15_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH1_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_AVGS {
        CMDH1_AVGS::from_bits(val)
    }
}
impl From<CMDH1_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_AVGS) -> u8 {
        CMDH1_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_CMPEN {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl CMDH1_CMPEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_CMPEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_CMPEN {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_CMPEN {
        CMDH1_CMPEN::from_bits(val)
    }
}
impl From<CMDH1_CMPEN> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_CMPEN) -> u8 {
        CMDH1_CMPEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH1_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_LOOP {
        CMDH1_LOOP::from_bits(val)
    }
}
impl From<CMDH1_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_LOOP) -> u8 {
        CMDH1_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH1_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_LWI {
        CMDH1_LWI::from_bits(val)
    }
}
impl From<CMDH1_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_LWI) -> u8 {
        CMDH1_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH1_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_NEXT {
        CMDH1_NEXT::from_bits(val)
    }
}
impl From<CMDH1_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_NEXT) -> u8 {
        CMDH1_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH1_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_STS {
        CMDH1_STS::from_bits(val)
    }
}
impl From<CMDH1_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_STS) -> u8 {
        CMDH1_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH1_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH1_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH1_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH1_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH1_WAIT_TRIG {
        CMDH1_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH1_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH1_WAIT_TRIG) -> u8 {
        CMDH1_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH2_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_AVGS {
        CMDH2_AVGS::from_bits(val)
    }
}
impl From<CMDH2_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_AVGS) -> u8 {
        CMDH2_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_CMPEN {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl CMDH2_CMPEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_CMPEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_CMPEN {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_CMPEN {
        CMDH2_CMPEN::from_bits(val)
    }
}
impl From<CMDH2_CMPEN> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_CMPEN) -> u8 {
        CMDH2_CMPEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH2_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_LOOP {
        CMDH2_LOOP::from_bits(val)
    }
}
impl From<CMDH2_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_LOOP) -> u8 {
        CMDH2_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH2_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_LWI {
        CMDH2_LWI::from_bits(val)
    }
}
impl From<CMDH2_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_LWI) -> u8 {
        CMDH2_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH2_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_NEXT {
        CMDH2_NEXT::from_bits(val)
    }
}
impl From<CMDH2_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_NEXT) -> u8 {
        CMDH2_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH2_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_STS {
        CMDH2_STS::from_bits(val)
    }
}
impl From<CMDH2_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_STS) -> u8 {
        CMDH2_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH2_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH2_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH2_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH2_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH2_WAIT_TRIG {
        CMDH2_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH2_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH2_WAIT_TRIG) -> u8 {
        CMDH2_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH3_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_AVGS {
        CMDH3_AVGS::from_bits(val)
    }
}
impl From<CMDH3_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_AVGS) -> u8 {
        CMDH3_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_CMPEN {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl CMDH3_CMPEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_CMPEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_CMPEN {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_CMPEN {
        CMDH3_CMPEN::from_bits(val)
    }
}
impl From<CMDH3_CMPEN> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_CMPEN) -> u8 {
        CMDH3_CMPEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH3_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_LOOP {
        CMDH3_LOOP::from_bits(val)
    }
}
impl From<CMDH3_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_LOOP) -> u8 {
        CMDH3_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH3_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_LWI {
        CMDH3_LWI::from_bits(val)
    }
}
impl From<CMDH3_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_LWI) -> u8 {
        CMDH3_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH3_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_NEXT {
        CMDH3_NEXT::from_bits(val)
    }
}
impl From<CMDH3_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_NEXT) -> u8 {
        CMDH3_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH3_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_STS {
        CMDH3_STS::from_bits(val)
    }
}
impl From<CMDH3_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_STS) -> u8 {
        CMDH3_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH3_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH3_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH3_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH3_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH3_WAIT_TRIG {
        CMDH3_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH3_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH3_WAIT_TRIG) -> u8 {
        CMDH3_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH4_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_AVGS {
        CMDH4_AVGS::from_bits(val)
    }
}
impl From<CMDH4_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_AVGS) -> u8 {
        CMDH4_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_CMPEN {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl CMDH4_CMPEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_CMPEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_CMPEN {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_CMPEN {
        CMDH4_CMPEN::from_bits(val)
    }
}
impl From<CMDH4_CMPEN> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_CMPEN) -> u8 {
        CMDH4_CMPEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH4_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_LOOP {
        CMDH4_LOOP::from_bits(val)
    }
}
impl From<CMDH4_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_LOOP) -> u8 {
        CMDH4_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH4_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_LWI {
        CMDH4_LWI::from_bits(val)
    }
}
impl From<CMDH4_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_LWI) -> u8 {
        CMDH4_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH4_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_NEXT {
        CMDH4_NEXT::from_bits(val)
    }
}
impl From<CMDH4_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_NEXT) -> u8 {
        CMDH4_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH4_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_STS {
        CMDH4_STS::from_bits(val)
    }
}
impl From<CMDH4_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_STS) -> u8 {
        CMDH4_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH4_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH4_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH4_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH4_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH4_WAIT_TRIG {
        CMDH4_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH4_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH4_WAIT_TRIG) -> u8 {
        CMDH4_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH5_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH5_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH5_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH5_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH5_AVGS {
        CMDH5_AVGS::from_bits(val)
    }
}
impl From<CMDH5_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH5_AVGS) -> u8 {
        CMDH5_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH5_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH5_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH5_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH5_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH5_LOOP {
        CMDH5_LOOP::from_bits(val)
    }
}
impl From<CMDH5_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH5_LOOP) -> u8 {
        CMDH5_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH5_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH5_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH5_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH5_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH5_LWI {
        CMDH5_LWI::from_bits(val)
    }
}
impl From<CMDH5_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH5_LWI) -> u8 {
        CMDH5_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH5_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH5_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH5_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH5_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH5_NEXT {
        CMDH5_NEXT::from_bits(val)
    }
}
impl From<CMDH5_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH5_NEXT) -> u8 {
        CMDH5_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH5_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH5_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH5_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH5_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH5_STS {
        CMDH5_STS::from_bits(val)
    }
}
impl From<CMDH5_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH5_STS) -> u8 {
        CMDH5_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH5_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH5_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH5_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH5_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH5_WAIT_TRIG {
        CMDH5_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH5_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH5_WAIT_TRIG) -> u8 {
        CMDH5_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH6_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH6_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH6_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH6_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH6_AVGS {
        CMDH6_AVGS::from_bits(val)
    }
}
impl From<CMDH6_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH6_AVGS) -> u8 {
        CMDH6_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH6_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH6_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH6_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH6_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH6_LOOP {
        CMDH6_LOOP::from_bits(val)
    }
}
impl From<CMDH6_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH6_LOOP) -> u8 {
        CMDH6_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH6_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH6_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH6_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH6_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH6_LWI {
        CMDH6_LWI::from_bits(val)
    }
}
impl From<CMDH6_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH6_LWI) -> u8 {
        CMDH6_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH6_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH6_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH6_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH6_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH6_NEXT {
        CMDH6_NEXT::from_bits(val)
    }
}
impl From<CMDH6_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH6_NEXT) -> u8 {
        CMDH6_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH6_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH6_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH6_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH6_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH6_STS {
        CMDH6_STS::from_bits(val)
    }
}
impl From<CMDH6_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH6_STS) -> u8 {
        CMDH6_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH6_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH6_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH6_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH6_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH6_WAIT_TRIG {
        CMDH6_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH6_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH6_WAIT_TRIG) -> u8 {
        CMDH6_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH7_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH7_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH7_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH7_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH7_AVGS {
        CMDH7_AVGS::from_bits(val)
    }
}
impl From<CMDH7_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH7_AVGS) -> u8 {
        CMDH7_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH7_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH7_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH7_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH7_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH7_LOOP {
        CMDH7_LOOP::from_bits(val)
    }
}
impl From<CMDH7_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH7_LOOP) -> u8 {
        CMDH7_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH7_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH7_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH7_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH7_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH7_LWI {
        CMDH7_LWI::from_bits(val)
    }
}
impl From<CMDH7_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH7_LWI) -> u8 {
        CMDH7_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH7_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH7_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH7_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH7_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH7_NEXT {
        CMDH7_NEXT::from_bits(val)
    }
}
impl From<CMDH7_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH7_NEXT) -> u8 {
        CMDH7_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH7_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH7_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH7_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH7_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH7_STS {
        CMDH7_STS::from_bits(val)
    }
}
impl From<CMDH7_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH7_STS) -> u8 {
        CMDH7_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH7_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH7_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH7_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH7_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH7_WAIT_TRIG {
        CMDH7_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH7_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH7_WAIT_TRIG) -> u8 {
        CMDH7_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH8_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH8_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH8_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH8_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH8_AVGS {
        CMDH8_AVGS::from_bits(val)
    }
}
impl From<CMDH8_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH8_AVGS) -> u8 {
        CMDH8_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH8_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH8_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH8_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH8_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH8_LOOP {
        CMDH8_LOOP::from_bits(val)
    }
}
impl From<CMDH8_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH8_LOOP) -> u8 {
        CMDH8_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH8_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH8_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH8_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH8_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH8_LWI {
        CMDH8_LWI::from_bits(val)
    }
}
impl From<CMDH8_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH8_LWI) -> u8 {
        CMDH8_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH8_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH8_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH8_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH8_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH8_NEXT {
        CMDH8_NEXT::from_bits(val)
    }
}
impl From<CMDH8_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH8_NEXT) -> u8 {
        CMDH8_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH8_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH8_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH8_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH8_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH8_STS {
        CMDH8_STS::from_bits(val)
    }
}
impl From<CMDH8_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH8_STS) -> u8 {
        CMDH8_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH8_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH8_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH8_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH8_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH8_WAIT_TRIG {
        CMDH8_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH8_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH8_WAIT_TRIG) -> u8 {
        CMDH8_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH9_AVGS {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CMDH9_AVGS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH9_AVGS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH9_AVGS {
    #[inline(always)]
    fn from(val: u8) -> CMDH9_AVGS {
        CMDH9_AVGS::from_bits(val)
    }
}
impl From<CMDH9_AVGS> for u8 {
    #[inline(always)]
    fn from(val: CMDH9_AVGS) -> u8 {
        CMDH9_AVGS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH9_LOOP {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CMDH9_LOOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH9_LOOP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH9_LOOP {
    #[inline(always)]
    fn from(val: u8) -> CMDH9_LOOP {
        CMDH9_LOOP::from_bits(val)
    }
}
impl From<CMDH9_LOOP> for u8 {
    #[inline(always)]
    fn from(val: CMDH9_LOOP) -> u8 {
        CMDH9_LOOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH9_LWI {
    #[doc = "Auto channel increment disabled."]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled."]
    LWI_1 = 0x01,
}
impl CMDH9_LWI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH9_LWI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH9_LWI {
    #[inline(always)]
    fn from(val: u8) -> CMDH9_LWI {
        CMDH9_LWI::from_bits(val)
    }
}
impl From<CMDH9_LWI> for u8 {
    #[inline(always)]
    fn from(val: CMDH9_LWI) -> u8 {
        CMDH9_LWI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH9_NEXT {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CMDH9_NEXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH9_NEXT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH9_NEXT {
    #[inline(always)]
    fn from(val: u8) -> CMDH9_NEXT {
        CMDH9_NEXT::from_bits(val)
    }
}
impl From<CMDH9_NEXT> for u8 {
    #[inline(always)]
    fn from(val: CMDH9_NEXT) -> u8 {
        CMDH9_NEXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH9_STS {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CMDH9_STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH9_STS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH9_STS {
    #[inline(always)]
    fn from(val: u8) -> CMDH9_STS {
        CMDH9_STS::from_bits(val)
    }
}
impl From<CMDH9_STS> for u8 {
    #[inline(always)]
    fn from(val: CMDH9_STS) -> u8 {
        CMDH9_STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDH9_WAIT_TRIG {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl CMDH9_WAIT_TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDH9_WAIT_TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDH9_WAIT_TRIG {
    #[inline(always)]
    fn from(val: u8) -> CMDH9_WAIT_TRIG {
        CMDH9_WAIT_TRIG::from_bits(val)
    }
}
impl From<CMDH9_WAIT_TRIG> for u8 {
    #[inline(always)]
    fn from(val: CMDH9_WAIT_TRIG) -> u8 {
        CMDH9_WAIT_TRIG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL10_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL10_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL10_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL10_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL10_ADCH {
        CMDL10_ADCH::from_bits(val)
    }
}
impl From<CMDL10_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL10_ADCH) -> u8 {
        CMDL10_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL10_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL10_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL10_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL10_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL10_CTYPE {
        CMDL10_CTYPE::from_bits(val)
    }
}
impl From<CMDL10_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL10_CTYPE) -> u8 {
        CMDL10_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL10_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL10_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL10_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL10_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL10_MODE {
        CMDL10_MODE::from_bits(val)
    }
}
impl From<CMDL10_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL10_MODE) -> u8 {
        CMDL10_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL11_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL11_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL11_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL11_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL11_ADCH {
        CMDL11_ADCH::from_bits(val)
    }
}
impl From<CMDL11_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL11_ADCH) -> u8 {
        CMDL11_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL11_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL11_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL11_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL11_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL11_CTYPE {
        CMDL11_CTYPE::from_bits(val)
    }
}
impl From<CMDL11_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL11_CTYPE) -> u8 {
        CMDL11_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL11_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL11_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL11_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL11_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL11_MODE {
        CMDL11_MODE::from_bits(val)
    }
}
impl From<CMDL11_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL11_MODE) -> u8 {
        CMDL11_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL12_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL12_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL12_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL12_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL12_ADCH {
        CMDL12_ADCH::from_bits(val)
    }
}
impl From<CMDL12_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL12_ADCH) -> u8 {
        CMDL12_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL12_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL12_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL12_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL12_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL12_CTYPE {
        CMDL12_CTYPE::from_bits(val)
    }
}
impl From<CMDL12_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL12_CTYPE) -> u8 {
        CMDL12_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL12_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL12_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL12_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL12_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL12_MODE {
        CMDL12_MODE::from_bits(val)
    }
}
impl From<CMDL12_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL12_MODE) -> u8 {
        CMDL12_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL13_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL13_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL13_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL13_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL13_ADCH {
        CMDL13_ADCH::from_bits(val)
    }
}
impl From<CMDL13_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL13_ADCH) -> u8 {
        CMDL13_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL13_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL13_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL13_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL13_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL13_CTYPE {
        CMDL13_CTYPE::from_bits(val)
    }
}
impl From<CMDL13_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL13_CTYPE) -> u8 {
        CMDL13_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL13_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL13_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL13_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL13_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL13_MODE {
        CMDL13_MODE::from_bits(val)
    }
}
impl From<CMDL13_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL13_MODE) -> u8 {
        CMDL13_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL14_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL14_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL14_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL14_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL14_ADCH {
        CMDL14_ADCH::from_bits(val)
    }
}
impl From<CMDL14_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL14_ADCH) -> u8 {
        CMDL14_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL14_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL14_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL14_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL14_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL14_CTYPE {
        CMDL14_CTYPE::from_bits(val)
    }
}
impl From<CMDL14_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL14_CTYPE) -> u8 {
        CMDL14_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL14_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL14_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL14_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL14_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL14_MODE {
        CMDL14_MODE::from_bits(val)
    }
}
impl From<CMDL14_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL14_MODE) -> u8 {
        CMDL14_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL15_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL15_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL15_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL15_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL15_ADCH {
        CMDL15_ADCH::from_bits(val)
    }
}
impl From<CMDL15_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL15_ADCH) -> u8 {
        CMDL15_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL15_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL15_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL15_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL15_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL15_CTYPE {
        CMDL15_CTYPE::from_bits(val)
    }
}
impl From<CMDL15_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL15_CTYPE) -> u8 {
        CMDL15_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL15_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL15_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL15_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL15_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL15_MODE {
        CMDL15_MODE::from_bits(val)
    }
}
impl From<CMDL15_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL15_MODE) -> u8 {
        CMDL15_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL1_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL1_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL1_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL1_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL1_ADCH {
        CMDL1_ADCH::from_bits(val)
    }
}
impl From<CMDL1_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL1_ADCH) -> u8 {
        CMDL1_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL1_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL1_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL1_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL1_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL1_CTYPE {
        CMDL1_CTYPE::from_bits(val)
    }
}
impl From<CMDL1_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL1_CTYPE) -> u8 {
        CMDL1_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL1_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL1_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL1_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL1_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL1_MODE {
        CMDL1_MODE::from_bits(val)
    }
}
impl From<CMDL1_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL1_MODE) -> u8 {
        CMDL1_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL2_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL2_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL2_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL2_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL2_ADCH {
        CMDL2_ADCH::from_bits(val)
    }
}
impl From<CMDL2_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL2_ADCH) -> u8 {
        CMDL2_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL2_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL2_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL2_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL2_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL2_CTYPE {
        CMDL2_CTYPE::from_bits(val)
    }
}
impl From<CMDL2_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL2_CTYPE) -> u8 {
        CMDL2_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL2_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL2_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL2_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL2_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL2_MODE {
        CMDL2_MODE::from_bits(val)
    }
}
impl From<CMDL2_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL2_MODE) -> u8 {
        CMDL2_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL3_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL3_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL3_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL3_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL3_ADCH {
        CMDL3_ADCH::from_bits(val)
    }
}
impl From<CMDL3_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL3_ADCH) -> u8 {
        CMDL3_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL3_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL3_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL3_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL3_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL3_CTYPE {
        CMDL3_CTYPE::from_bits(val)
    }
}
impl From<CMDL3_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL3_CTYPE) -> u8 {
        CMDL3_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL3_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL3_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL3_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL3_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL3_MODE {
        CMDL3_MODE::from_bits(val)
    }
}
impl From<CMDL3_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL3_MODE) -> u8 {
        CMDL3_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL4_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL4_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL4_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL4_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL4_ADCH {
        CMDL4_ADCH::from_bits(val)
    }
}
impl From<CMDL4_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL4_ADCH) -> u8 {
        CMDL4_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL4_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL4_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL4_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL4_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL4_CTYPE {
        CMDL4_CTYPE::from_bits(val)
    }
}
impl From<CMDL4_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL4_CTYPE) -> u8 {
        CMDL4_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL4_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL4_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL4_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL4_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL4_MODE {
        CMDL4_MODE::from_bits(val)
    }
}
impl From<CMDL4_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL4_MODE) -> u8 {
        CMDL4_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL5_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL5_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL5_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL5_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL5_ADCH {
        CMDL5_ADCH::from_bits(val)
    }
}
impl From<CMDL5_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL5_ADCH) -> u8 {
        CMDL5_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL5_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL5_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL5_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL5_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL5_CTYPE {
        CMDL5_CTYPE::from_bits(val)
    }
}
impl From<CMDL5_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL5_CTYPE) -> u8 {
        CMDL5_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL5_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL5_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL5_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL5_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL5_MODE {
        CMDL5_MODE::from_bits(val)
    }
}
impl From<CMDL5_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL5_MODE) -> u8 {
        CMDL5_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL6_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL6_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL6_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL6_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL6_ADCH {
        CMDL6_ADCH::from_bits(val)
    }
}
impl From<CMDL6_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL6_ADCH) -> u8 {
        CMDL6_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL6_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL6_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL6_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL6_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL6_CTYPE {
        CMDL6_CTYPE::from_bits(val)
    }
}
impl From<CMDL6_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL6_CTYPE) -> u8 {
        CMDL6_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL6_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL6_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL6_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL6_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL6_MODE {
        CMDL6_MODE::from_bits(val)
    }
}
impl From<CMDL6_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL6_MODE) -> u8 {
        CMDL6_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL7_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL7_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL7_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL7_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL7_ADCH {
        CMDL7_ADCH::from_bits(val)
    }
}
impl From<CMDL7_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL7_ADCH) -> u8 {
        CMDL7_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL7_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL7_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL7_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL7_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL7_CTYPE {
        CMDL7_CTYPE::from_bits(val)
    }
}
impl From<CMDL7_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL7_CTYPE) -> u8 {
        CMDL7_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL7_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL7_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL7_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL7_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL7_MODE {
        CMDL7_MODE::from_bits(val)
    }
}
impl From<CMDL7_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL7_MODE) -> u8 {
        CMDL7_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL8_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL8_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL8_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL8_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL8_ADCH {
        CMDL8_ADCH::from_bits(val)
    }
}
impl From<CMDL8_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL8_ADCH) -> u8 {
        CMDL8_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL8_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL8_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL8_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL8_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL8_CTYPE {
        CMDL8_CTYPE::from_bits(val)
    }
}
impl From<CMDL8_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL8_CTYPE) -> u8 {
        CMDL8_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL8_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL8_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL8_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL8_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL8_MODE {
        CMDL8_MODE::from_bits(val)
    }
}
impl From<CMDL8_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL8_MODE) -> u8 {
        CMDL8_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL9_ADCH {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CMDL9_ADCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL9_ADCH {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL9_ADCH {
    #[inline(always)]
    fn from(val: u8) -> CMDL9_ADCH {
        CMDL9_ADCH::from_bits(val)
    }
}
impl From<CMDL9_ADCH> for u8 {
    #[inline(always)]
    fn from(val: CMDL9_ADCH) -> u8 {
        CMDL9_ADCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL9_CTYPE {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl CMDL9_CTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL9_CTYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL9_CTYPE {
    #[inline(always)]
    fn from(val: u8) -> CMDL9_CTYPE {
        CMDL9_CTYPE::from_bits(val)
    }
}
impl From<CMDL9_CTYPE> for u8 {
    #[inline(always)]
    fn from(val: CMDL9_CTYPE) -> u8 {
        CMDL9_CTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDL9_MODE {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl CMDL9_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDL9_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDL9_MODE {
    #[inline(always)]
    fn from(val: u8) -> CMDL9_MODE {
        CMDL9_MODE::from_bits(val)
    }
}
impl From<CMDL9_MODE> for u8 {
    #[inline(always)]
    fn from(val: CMDL9_MODE) -> u8 {
        CMDL9_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMDSRC {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    CMDSRC_0 = 0x0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    CMDSRC_1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_6 = 0x06,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_7 = 0x07,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_8 = 0x08,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 buffer used as control settings for this conversion."]
    CMDSRC_15 = 0x0f,
}
impl CMDSRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMDSRC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMDSRC {
    #[inline(always)]
    fn from(val: u8) -> CMDSRC {
        CMDSRC::from_bits(val)
    }
}
impl From<CMDSRC> for u8 {
    #[inline(always)]
    fn from(val: CMDSRC) -> u8 {
        CMDSRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CSW {
    #[doc = "Channel scaling not supported."]
    CSW_0 = 0x0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    CSW_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    CSW_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CSW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CSW {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CSW {
    #[inline(always)]
    fn from(val: u8) -> CSW {
        CSW::from_bits(val)
    }
}
impl From<CSW> for u8 {
    #[inline(always)]
    fn from(val: CSW) -> u8 {
        CSW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DIFFEN {
    #[doc = "Differential operation not supported."]
    DIFFEN_0 = 0x0,
    #[doc = "Differential operation supported. CMDLa\\[CTYPE\\] controls fields implemented."]
    DIFFEN_1 = 0x01,
}
impl DIFFEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DIFFEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DIFFEN {
    #[inline(always)]
    fn from(val: u8) -> DIFFEN {
        DIFFEN::from_bits(val)
    }
}
impl From<DIFFEN> for u8 {
    #[inline(always)]
    fn from(val: DIFFEN) -> u8 {
        DIFFEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DOZEN {
    #[doc = "ADC is enabled in Doze mode."]
    DOZEN_0 = 0x0,
    #[doc = "ADC is disabled in Doze mode."]
    DOZEN_1 = 0x01,
}
impl DOZEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DOZEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DOZEN {
    #[inline(always)]
    fn from(val: u8) -> DOZEN {
        DOZEN::from_bits(val)
    }
}
impl From<DOZEN> for u8 {
    #[inline(always)]
    fn from(val: DOZEN) -> u8 {
        DOZEN::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FIFOSIZE(u8);
impl FIFOSIZE {
    #[doc = "Result FIFO depth = 1 dataword."]
    pub const FIFOSIZE_1: Self = Self(0x01);
    #[doc = "Result FIFO depth = 4 datawords."]
    pub const FIFOSIZE_4: Self = Self(0x04);
    #[doc = "Result FIFO depth = 8 datawords."]
    pub const FIFOSIZE_8: Self = Self(0x08);
    #[doc = "Result FIFO depth = 16 datawords."]
    pub const FIFOSIZE_16: Self = Self(0x10);
    #[doc = "Result FIFO depth = 32 datawords."]
    pub const FIFOSIZE_32: Self = Self(0x20);
    #[doc = "Result FIFO depth = 64 datawords."]
    pub const FIFOSIZE_64: Self = Self(0x40);
}
impl FIFOSIZE {
    pub const fn from_bits(val: u8) -> FIFOSIZE {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FIFOSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FIFOSIZE_1"),
            0x04 => f.write_str("FIFOSIZE_4"),
            0x08 => f.write_str("FIFOSIZE_8"),
            0x10 => f.write_str("FIFOSIZE_16"),
            0x20 => f.write_str("FIFOSIZE_32"),
            0x40 => f.write_str("FIFOSIZE_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FIFOSIZE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FIFOSIZE_1"),
            0x04 => defmt::write!(f, "FIFOSIZE_4"),
            0x08 => defmt::write!(f, "FIFOSIZE_8"),
            0x10 => defmt::write!(f, "FIFOSIZE_16"),
            0x20 => defmt::write!(f, "FIFOSIZE_32"),
            0x40 => defmt::write!(f, "FIFOSIZE_64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FIFOSIZE {
    #[inline(always)]
    fn from(val: u8) -> FIFOSIZE {
        FIFOSIZE::from_bits(val)
    }
}
impl From<FIFOSIZE> for u8 {
    #[inline(always)]
    fn from(val: FIFOSIZE) -> u8 {
        FIFOSIZE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FIFO_SEL_A {
    #[doc = "Result written to FIFO 0."]
    FIFO_SEL_A_0 = 0x0,
    #[doc = "Result written to FIFO 1."]
    FIFO_SEL_A_1 = 0x01,
}
impl FIFO_SEL_A {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FIFO_SEL_A {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FIFO_SEL_A {
    #[inline(always)]
    fn from(val: u8) -> FIFO_SEL_A {
        FIFO_SEL_A::from_bits(val)
    }
}
impl From<FIFO_SEL_A> for u8 {
    #[inline(always)]
    fn from(val: FIFO_SEL_A) -> u8 {
        FIFO_SEL_A::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FIFO_SEL_B {
    #[doc = "Result written to FIFO 0."]
    FIFO_SEL_B_0 = 0x0,
    #[doc = "Result written to FIFO 1."]
    FIFO_SEL_B_1 = 0x01,
}
impl FIFO_SEL_B {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FIFO_SEL_B {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FIFO_SEL_B {
    #[inline(always)]
    fn from(val: u8) -> FIFO_SEL_B {
        FIFO_SEL_B::from_bits(val)
    }
}
impl From<FIFO_SEL_B> for u8 {
    #[inline(always)]
    fn from(val: FIFO_SEL_B) -> u8 {
        FIFO_SEL_B::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FOF0 {
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_0 = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_1 = 0x01,
}
impl FOF0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FOF0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FOF0 {
    #[inline(always)]
    fn from(val: u8) -> FOF0 {
        FOF0::from_bits(val)
    }
}
impl From<FOF0> for u8 {
    #[inline(always)]
    fn from(val: FOF0) -> u8 {
        FOF0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FOF1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_0 = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_1 = 0x01,
}
impl FOF1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FOF1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FOF1 {
    #[inline(always)]
    fn from(val: u8) -> FOF1 {
        FOF1::from_bits(val)
    }
}
impl From<FOF1> for u8 {
    #[inline(always)]
    fn from(val: FOF1) -> u8 {
        FOF1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FOFIE0 {
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0 = 0x0,
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1 = 0x01,
}
impl FOFIE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FOFIE0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FOFIE0 {
    #[inline(always)]
    fn from(val: u8) -> FOFIE0 {
        FOFIE0::from_bits(val)
    }
}
impl From<FOFIE0> for u8 {
    #[inline(always)]
    fn from(val: FOFIE0) -> u8 {
        FOFIE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FOFIE1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0 = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1 = 0x01,
}
impl FOFIE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FOFIE1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FOFIE1 {
    #[inline(always)]
    fn from(val: u8) -> FOFIE1 {
        FOFIE1::from_bits(val)
    }
}
impl From<FOFIE1> for u8 {
    #[inline(always)]
    fn from(val: FOFIE1) -> u8 {
        FOFIE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FWMDE0 {
    #[doc = "DMA request disabled."]
    FWMDE0_0 = 0x0,
    #[doc = "DMA request enabled."]
    FWMDE0_1 = 0x01,
}
impl FWMDE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FWMDE0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FWMDE0 {
    #[inline(always)]
    fn from(val: u8) -> FWMDE0 {
        FWMDE0::from_bits(val)
    }
}
impl From<FWMDE0> for u8 {
    #[inline(always)]
    fn from(val: FWMDE0) -> u8 {
        FWMDE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FWMDE1 {
    #[doc = "DMA request disabled."]
    FWMDE1_0 = 0x0,
    #[doc = "DMA request enabled."]
    FWMDE1_1 = 0x01,
}
impl FWMDE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FWMDE1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FWMDE1 {
    #[inline(always)]
    fn from(val: u8) -> FWMDE1 {
        FWMDE1::from_bits(val)
    }
}
impl From<FWMDE1> for u8 {
    #[inline(always)]
    fn from(val: FWMDE1) -> u8 {
        FWMDE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FWMIE0 {
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0 = 0x0,
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1 = 0x01,
}
impl FWMIE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FWMIE0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FWMIE0 {
    #[inline(always)]
    fn from(val: u8) -> FWMIE0 {
        FWMIE0::from_bits(val)
    }
}
impl From<FWMIE0> for u8 {
    #[inline(always)]
    fn from(val: FWMIE0) -> u8 {
        FWMIE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FWMIE1 {
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0 = 0x0,
    #[doc = "FIFO1 watermark interrupts are enabled."]
    FWMIE1_1 = 0x01,
}
impl FWMIE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FWMIE1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FWMIE1 {
    #[inline(always)]
    fn from(val: u8) -> FWMIE1 {
        FWMIE1::from_bits(val)
    }
}
impl From<FWMIE1> for u8 {
    #[inline(always)]
    fn from(val: FWMIE1) -> u8 {
        FWMIE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GCC_RDY {
    #[doc = "The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    RDY_0 = 0x0,
    #[doc = "The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\] register field."]
    RDY_1 = 0x01,
}
impl GCC_RDY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GCC_RDY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GCC_RDY {
    #[inline(always)]
    fn from(val: u8) -> GCC_RDY {
        GCC_RDY::from_bits(val)
    }
}
impl From<GCC_RDY> for u8 {
    #[inline(always)]
    fn from(val: GCC_RDY) -> u8 {
        GCC_RDY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GCR_RDY {
    #[doc = "The gain offset calculation value is invalid."]
    RDY_0 = 0x0,
    #[doc = "The gain calibration value is valid."]
    RDY_1 = 0x01,
}
impl GCR_RDY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GCR_RDY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GCR_RDY {
    #[inline(always)]
    fn from(val: u8) -> GCR_RDY {
        GCR_RDY::from_bits(val)
    }
}
impl From<GCR_RDY> for u8 {
    #[inline(always)]
    fn from(val: GCR_RDY) -> u8 {
        GCR_RDY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HPT_EXDI {
    #[doc = "High priority trigger exceptions are enabled."]
    HPT_EXDI_0 = 0x0,
    #[doc = "High priority trigger exceptions are disabled."]
    HPT_EXDI_1 = 0x01,
}
impl HPT_EXDI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HPT_EXDI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HPT_EXDI {
    #[inline(always)]
    fn from(val: u8) -> HPT_EXDI {
        HPT_EXDI::from_bits(val)
    }
}
impl From<HPT_EXDI> for u8 {
    #[inline(always)]
    fn from(val: HPT_EXDI) -> u8 {
        HPT_EXDI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HTEN {
    #[doc = "Hardware trigger source disabled."]
    HTEN_0 = 0x0,
    #[doc = "Hardware trigger source enabled."]
    HTEN_1 = 0x01,
}
impl HTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HTEN {
    #[inline(always)]
    fn from(val: u8) -> HTEN {
        HTEN::from_bits(val)
    }
}
impl From<HTEN> for u8 {
    #[inline(always)]
    fn from(val: HTEN) -> u8 {
        HTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IADCKI {
    #[doc = "Internal clock source not implemented."]
    IADCKI_0 = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    IADCKI_1 = 0x01,
}
impl IADCKI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IADCKI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IADCKI {
    #[inline(always)]
    fn from(val: u8) -> IADCKI {
        IADCKI::from_bits(val)
    }
}
impl From<IADCKI> for u8 {
    #[inline(always)]
    fn from(val: IADCKI) -> u8 {
        IADCKI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOOPCNT {
    #[doc = "Result is from initial conversion in command."]
    LOOPCNT_0 = 0x0,
    #[doc = "Result is from second conversion in command."]
    LOOPCNT_1 = 0x01,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_2 = 0x02,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_3 = 0x03,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_4 = 0x04,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_5 = 0x05,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_6 = 0x06,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_7 = 0x07,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_8 = 0x08,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    LOOPCNT_15 = 0x0f,
}
impl LOOPCNT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOOPCNT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOOPCNT {
    #[inline(always)]
    fn from(val: u8) -> LOOPCNT {
        LOOPCNT::from_bits(val)
    }
}
impl From<LOOPCNT> for u8 {
    #[inline(always)]
    fn from(val: LOOPCNT) -> u8 {
        LOOPCNT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MVI {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MVI_0 = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MVI_1 = 0x01,
}
impl MVI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MVI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MVI {
    #[inline(always)]
    fn from(val: u8) -> MVI {
        MVI::from_bits(val)
    }
}
impl From<MVI> for u8 {
    #[inline(always)]
    fn from(val: MVI) -> u8 {
        MVI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NUM_FIFO {
    #[doc = "N/A."]
    NUM_FIFO_0 = 0x0,
    #[doc = "This design supports one result FIFO."]
    NUM_FIFO_1 = 0x01,
    #[doc = "This design supports two result FIFOs."]
    NUM_FIFO_2 = 0x02,
    #[doc = "This design supports three result FIFOs."]
    NUM_FIFO_3 = 0x03,
    #[doc = "This design supports four result FIFOs."]
    NUM_FIFO_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl NUM_FIFO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NUM_FIFO {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NUM_FIFO {
    #[inline(always)]
    fn from(val: u8) -> NUM_FIFO {
        NUM_FIFO::from_bits(val)
    }
}
impl From<NUM_FIFO> for u8 {
    #[inline(always)]
    fn from(val: NUM_FIFO) -> u8 {
        NUM_FIFO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NUM_SEC {
    #[doc = "This design supports one single ended conversion at a time."]
    NUM_SEC_0 = 0x0,
    #[doc = "This design supports two simultanious single ended conversions."]
    NUM_SEC_1 = 0x01,
}
impl NUM_SEC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NUM_SEC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NUM_SEC {
    #[inline(always)]
    fn from(val: u8) -> NUM_SEC {
        NUM_SEC::from_bits(val)
    }
}
impl From<NUM_SEC> for u8 {
    #[inline(always)]
    fn from(val: NUM_SEC) -> u8 {
        NUM_SEC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PAUSEEN {
    #[doc = "Pause operation disabled."]
    PAUSEEN_0 = 0x0,
    #[doc = "Pause operation enabled."]
    PAUSEEN_1 = 0x01,
}
impl PAUSEEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PAUSEEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PAUSEEN {
    #[inline(always)]
    fn from(val: u8) -> PAUSEEN {
        PAUSEEN::from_bits(val)
    }
}
impl From<PAUSEEN> for u8 {
    #[inline(always)]
    fn from(val: PAUSEEN) -> u8 {
        PAUSEEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWREN {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0 = 0x0,
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    PWREN_1 = 0x01,
}
impl PWREN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWREN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWREN {
    #[inline(always)]
    fn from(val: u8) -> PWREN {
        PWREN::from_bits(val)
    }
}
impl From<PWREN> for u8 {
    #[inline(always)]
    fn from(val: PWREN) -> u8 {
        PWREN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWRSEL {
    #[doc = "Lowest power setting."]
    PWRSEL_0 = 0x0,
    #[doc = "Higher power setting than 0b0."]
    PWRSEL_1 = 0x01,
    #[doc = "Higher power setting than 0b1."]
    PWRSEL_2 = 0x02,
    #[doc = "Highest power setting."]
    PWRSEL_3 = 0x03,
}
impl PWRSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWRSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWRSEL {
    #[inline(always)]
    fn from(val: u8) -> PWRSEL {
        PWRSEL::from_bits(val)
    }
}
impl From<PWRSEL> for u8 {
    #[inline(always)]
    fn from(val: PWRSEL) -> u8 {
        PWRSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RDY0 {
    #[doc = "Result FIFO 0 data level not above watermark level."]
    RDY0_0 = 0x0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    RDY0_1 = 0x01,
}
impl RDY0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RDY0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RDY0 {
    #[inline(always)]
    fn from(val: u8) -> RDY0 {
        RDY0::from_bits(val)
    }
}
impl From<RDY0> for u8 {
    #[inline(always)]
    fn from(val: RDY0) -> u8 {
        RDY0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RDY1 {
    #[doc = "Result FIFO1 data level not above watermark level."]
    RDY1_0 = 0x0,
    #[doc = "Result FIFO1 holding data above watermark level."]
    RDY1_1 = 0x01,
}
impl RDY1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RDY1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RDY1 {
    #[inline(always)]
    fn from(val: u8) -> RDY1 {
        RDY1::from_bits(val)
    }
}
impl From<RDY1> for u8 {
    #[inline(always)]
    fn from(val: RDY1) -> u8 {
        RDY1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REFSEL {
    #[doc = "(Default) Option 1 setting."]
    REFSEL_0 = 0x0,
    #[doc = "Option 2 setting."]
    REFSEL_1 = 0x01,
    #[doc = "Option 3 setting."]
    REFSEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl REFSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REFSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REFSEL {
    #[inline(always)]
    fn from(val: u8) -> REFSEL {
        REFSEL::from_bits(val)
    }
}
impl From<REFSEL> for u8 {
    #[inline(always)]
    fn from(val: REFSEL) -> u8 {
        REFSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RES {
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    RES_0 = 0x0,
    #[doc = "Up to 16-bit differential/16-bit single ended resolution supported."]
    RES_1 = 0x01,
}
impl RES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RES {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RES {
    #[inline(always)]
    fn from(val: u8) -> RES {
        RES::from_bits(val)
    }
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(val: RES) -> u8 {
        RES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RST {
    #[doc = "ADC logic is not reset."]
    RST_0 = 0x0,
    #[doc = "ADC logic is reset."]
    RST_1 = 0x01,
}
impl RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RST {
    #[inline(always)]
    fn from(val: u8) -> RST {
        RST::from_bits(val)
    }
}
impl From<RST> for u8 {
    #[inline(always)]
    fn from(val: RST) -> u8 {
        RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RSTFIFO0 {
    #[doc = "No effect."]
    RSTFIFO0_0 = 0x0,
    #[doc = "FIFO 0 is reset."]
    RSTFIFO0_1 = 0x01,
}
impl RSTFIFO0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RSTFIFO0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RSTFIFO0 {
    #[inline(always)]
    fn from(val: u8) -> RSTFIFO0 {
        RSTFIFO0::from_bits(val)
    }
}
impl From<RSTFIFO0> for u8 {
    #[inline(always)]
    fn from(val: RSTFIFO0) -> u8 {
        RSTFIFO0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RSTFIFO1 {
    #[doc = "No effect."]
    RSTFIFO1_0 = 0x0,
    #[doc = "FIFO 1 is reset."]
    RSTFIFO1_1 = 0x01,
}
impl RSTFIFO1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RSTFIFO1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RSTFIFO1 {
    #[inline(always)]
    fn from(val: u8) -> RSTFIFO1 {
        RSTFIFO1::from_bits(val)
    }
}
impl From<RSTFIFO1> for u8 {
    #[inline(always)]
    fn from(val: RSTFIFO1) -> u8 {
        RSTFIFO1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT0 {
    #[doc = "No trigger 0 event generated."]
    SWT0_0 = 0x0,
    #[doc = "Trigger 0 event generated."]
    SWT0_1 = 0x01,
}
impl SWT0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT0 {
    #[inline(always)]
    fn from(val: u8) -> SWT0 {
        SWT0::from_bits(val)
    }
}
impl From<SWT0> for u8 {
    #[inline(always)]
    fn from(val: SWT0) -> u8 {
        SWT0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT1 {
    #[doc = "No trigger 1 event generated."]
    SWT1_0 = 0x0,
    #[doc = "Trigger 1 event generated."]
    SWT1_1 = 0x01,
}
impl SWT1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT1 {
    #[inline(always)]
    fn from(val: u8) -> SWT1 {
        SWT1::from_bits(val)
    }
}
impl From<SWT1> for u8 {
    #[inline(always)]
    fn from(val: SWT1) -> u8 {
        SWT1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT10 {
    #[doc = "No trigger 10 event generated."]
    SWT10_0 = 0x0,
    #[doc = "Trigger 10 event generated."]
    SWT10_1 = 0x01,
}
impl SWT10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT10 {
    #[inline(always)]
    fn from(val: u8) -> SWT10 {
        SWT10::from_bits(val)
    }
}
impl From<SWT10> for u8 {
    #[inline(always)]
    fn from(val: SWT10) -> u8 {
        SWT10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT11 {
    #[doc = "No trigger 11 event generated."]
    SWT11_0 = 0x0,
    #[doc = "Trigger 11 event generated."]
    SWT11_1 = 0x01,
}
impl SWT11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT11 {
    #[inline(always)]
    fn from(val: u8) -> SWT11 {
        SWT11::from_bits(val)
    }
}
impl From<SWT11> for u8 {
    #[inline(always)]
    fn from(val: SWT11) -> u8 {
        SWT11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT12 {
    #[doc = "No trigger 12 event generated."]
    SWT12_0 = 0x0,
    #[doc = "Trigger 12 event generated."]
    SWT12_1 = 0x01,
}
impl SWT12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT12 {
    #[inline(always)]
    fn from(val: u8) -> SWT12 {
        SWT12::from_bits(val)
    }
}
impl From<SWT12> for u8 {
    #[inline(always)]
    fn from(val: SWT12) -> u8 {
        SWT12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT13 {
    #[doc = "No trigger 13 event generated."]
    SWT13_0 = 0x0,
    #[doc = "Trigger 13 event generated."]
    SWT13_1 = 0x01,
}
impl SWT13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT13 {
    #[inline(always)]
    fn from(val: u8) -> SWT13 {
        SWT13::from_bits(val)
    }
}
impl From<SWT13> for u8 {
    #[inline(always)]
    fn from(val: SWT13) -> u8 {
        SWT13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT14 {
    #[doc = "No trigger 14 event generated."]
    SWT14_0 = 0x0,
    #[doc = "Trigger 14 event generated."]
    SWT14_1 = 0x01,
}
impl SWT14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT14 {
    #[inline(always)]
    fn from(val: u8) -> SWT14 {
        SWT14::from_bits(val)
    }
}
impl From<SWT14> for u8 {
    #[inline(always)]
    fn from(val: SWT14) -> u8 {
        SWT14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT15 {
    #[doc = "No trigger 15 event generated."]
    SWT15_0 = 0x0,
    #[doc = "Trigger 15 event generated."]
    SWT15_1 = 0x01,
}
impl SWT15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT15 {
    #[inline(always)]
    fn from(val: u8) -> SWT15 {
        SWT15::from_bits(val)
    }
}
impl From<SWT15> for u8 {
    #[inline(always)]
    fn from(val: SWT15) -> u8 {
        SWT15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT2 {
    #[doc = "No trigger 2 event generated."]
    SWT2_0 = 0x0,
    #[doc = "Trigger 2 event generated."]
    SWT2_1 = 0x01,
}
impl SWT2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT2 {
    #[inline(always)]
    fn from(val: u8) -> SWT2 {
        SWT2::from_bits(val)
    }
}
impl From<SWT2> for u8 {
    #[inline(always)]
    fn from(val: SWT2) -> u8 {
        SWT2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT3 {
    #[doc = "No trigger 3 event generated."]
    SWT3_0 = 0x0,
    #[doc = "Trigger 3 event generated."]
    SWT3_1 = 0x01,
}
impl SWT3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT3 {
    #[inline(always)]
    fn from(val: u8) -> SWT3 {
        SWT3::from_bits(val)
    }
}
impl From<SWT3> for u8 {
    #[inline(always)]
    fn from(val: SWT3) -> u8 {
        SWT3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT4 {
    #[doc = "No trigger 4 event generated."]
    SWT4_0 = 0x0,
    #[doc = "Trigger 4 event generated."]
    SWT4_1 = 0x01,
}
impl SWT4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT4 {
    #[inline(always)]
    fn from(val: u8) -> SWT4 {
        SWT4::from_bits(val)
    }
}
impl From<SWT4> for u8 {
    #[inline(always)]
    fn from(val: SWT4) -> u8 {
        SWT4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT5 {
    #[doc = "No trigger 5 event generated."]
    SWT5_0 = 0x0,
    #[doc = "Trigger 5 event generated."]
    SWT5_1 = 0x01,
}
impl SWT5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT5 {
    #[inline(always)]
    fn from(val: u8) -> SWT5 {
        SWT5::from_bits(val)
    }
}
impl From<SWT5> for u8 {
    #[inline(always)]
    fn from(val: SWT5) -> u8 {
        SWT5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT6 {
    #[doc = "No trigger 6 event generated."]
    SWT6_0 = 0x0,
    #[doc = "Trigger 6 event generated."]
    SWT6_1 = 0x01,
}
impl SWT6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT6 {
    #[inline(always)]
    fn from(val: u8) -> SWT6 {
        SWT6::from_bits(val)
    }
}
impl From<SWT6> for u8 {
    #[inline(always)]
    fn from(val: SWT6) -> u8 {
        SWT6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT7 {
    #[doc = "No trigger 7 event generated."]
    SWT7_0 = 0x0,
    #[doc = "Trigger 7 event generated."]
    SWT7_1 = 0x01,
}
impl SWT7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT7 {
    #[inline(always)]
    fn from(val: u8) -> SWT7 {
        SWT7::from_bits(val)
    }
}
impl From<SWT7> for u8 {
    #[inline(always)]
    fn from(val: SWT7) -> u8 {
        SWT7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT8 {
    #[doc = "No trigger 8 event generated."]
    SWT8_0 = 0x0,
    #[doc = "Trigger 8 event generated."]
    SWT8_1 = 0x01,
}
impl SWT8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT8 {
    #[inline(always)]
    fn from(val: u8) -> SWT8 {
        SWT8::from_bits(val)
    }
}
impl From<SWT8> for u8 {
    #[inline(always)]
    fn from(val: SWT8) -> u8 {
        SWT8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWT9 {
    #[doc = "No trigger 9 event generated."]
    SWT9_0 = 0x0,
    #[doc = "Trigger 9 event generated."]
    SWT9_1 = 0x01,
}
impl SWT9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWT9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWT9 {
    #[inline(always)]
    fn from(val: u8) -> SWT9 {
        SWT9::from_bits(val)
    }
}
impl From<SWT9> for u8 {
    #[inline(always)]
    fn from(val: SWT9) -> u8 {
        SWT9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TCMD {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0 = 0x0,
    #[doc = "CMD1 is executed."]
    TCMD_1 = 0x01,
    #[doc = "Corresponding CMD is executed."]
    TCMD_2 = 0x02,
    #[doc = "Corresponding CMD is executed."]
    TCMD_3 = 0x03,
    #[doc = "Corresponding CMD is executed."]
    TCMD_4 = 0x04,
    #[doc = "Corresponding CMD is executed."]
    TCMD_5 = 0x05,
    #[doc = "Corresponding CMD is executed."]
    TCMD_6 = 0x06,
    #[doc = "Corresponding CMD is executed."]
    TCMD_7 = 0x07,
    #[doc = "Corresponding CMD is executed."]
    TCMD_8 = 0x08,
    #[doc = "Corresponding CMD is executed."]
    TCMD_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 is executed."]
    TCMD_15 = 0x0f,
}
impl TCMD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TCMD {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TCMD {
    #[inline(always)]
    fn from(val: u8) -> TCMD {
        TCMD::from_bits(val)
    }
}
impl From<TCMD> for u8 {
    #[inline(always)]
    fn from(val: TCMD) -> u8 {
        TCMD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TCMDRES {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    TCMDRES_0 = 0x0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    TCMDRES_1 = 0x01,
}
impl TCMDRES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TCMDRES {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TCMDRES {
    #[inline(always)]
    fn from(val: u8) -> TCMDRES {
        TCMDRES::from_bits(val)
    }
}
impl From<TCMDRES> for u8 {
    #[inline(always)]
    fn from(val: TCMDRES) -> u8 {
        TCMDRES::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TCOMP_FLAG(u16);
impl TCOMP_FLAG {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    pub const TCOMP_FLAG_0: Self = Self(0x0);
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    pub const TCOMP_FLAG_1: Self = Self(0x01);
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    pub const TCOMP_FLAG_2: Self = Self(0x02);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_3: Self = Self(0x03);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_4: Self = Self(0x04);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_5: Self = Self(0x05);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_6: Self = Self(0x06);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_7: Self = Self(0x07);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_8: Self = Self(0x08);
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    pub const TCOMP_FLAG_9: Self = Self(0x09);
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    pub const TCOMP_FLAG_65535: Self = Self(0xffff);
}
impl TCOMP_FLAG {
    pub const fn from_bits(val: u16) -> TCOMP_FLAG {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TCOMP_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TCOMP_FLAG_0"),
            0x01 => f.write_str("TCOMP_FLAG_1"),
            0x02 => f.write_str("TCOMP_FLAG_2"),
            0x03 => f.write_str("TCOMP_FLAG_3"),
            0x04 => f.write_str("TCOMP_FLAG_4"),
            0x05 => f.write_str("TCOMP_FLAG_5"),
            0x06 => f.write_str("TCOMP_FLAG_6"),
            0x07 => f.write_str("TCOMP_FLAG_7"),
            0x08 => f.write_str("TCOMP_FLAG_8"),
            0x09 => f.write_str("TCOMP_FLAG_9"),
            0xffff => f.write_str("TCOMP_FLAG_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TCOMP_FLAG {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TCOMP_FLAG_0"),
            0x01 => defmt::write!(f, "TCOMP_FLAG_1"),
            0x02 => defmt::write!(f, "TCOMP_FLAG_2"),
            0x03 => defmt::write!(f, "TCOMP_FLAG_3"),
            0x04 => defmt::write!(f, "TCOMP_FLAG_4"),
            0x05 => defmt::write!(f, "TCOMP_FLAG_5"),
            0x06 => defmt::write!(f, "TCOMP_FLAG_6"),
            0x07 => defmt::write!(f, "TCOMP_FLAG_7"),
            0x08 => defmt::write!(f, "TCOMP_FLAG_8"),
            0x09 => defmt::write!(f, "TCOMP_FLAG_9"),
            0xffff => defmt::write!(f, "TCOMP_FLAG_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TCOMP_FLAG {
    #[inline(always)]
    fn from(val: u16) -> TCOMP_FLAG {
        TCOMP_FLAG::from_bits(val)
    }
}
impl From<TCOMP_FLAG> for u16 {
    #[inline(always)]
    fn from(val: TCOMP_FLAG) -> u16 {
        TCOMP_FLAG::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TCOMP_IE(u16);
impl TCOMP_IE {
    #[doc = "Trigger completion interrupts are disabled."]
    pub const TCOMP_IE_0: Self = Self(0x0);
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    pub const TCOMP_IE_1: Self = Self(0x01);
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    pub const TCOMP_IE_2: Self = Self(0x02);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_3: Self = Self(0x03);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_4: Self = Self(0x04);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_5: Self = Self(0x05);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_6: Self = Self(0x06);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_7: Self = Self(0x07);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_8: Self = Self(0x08);
    #[doc = "Associated trigger completion interrupts are enabled."]
    pub const TCOMP_IE_9: Self = Self(0x09);
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    pub const TCOMP_IE_65535: Self = Self(0xffff);
}
impl TCOMP_IE {
    pub const fn from_bits(val: u16) -> TCOMP_IE {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TCOMP_IE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TCOMP_IE_0"),
            0x01 => f.write_str("TCOMP_IE_1"),
            0x02 => f.write_str("TCOMP_IE_2"),
            0x03 => f.write_str("TCOMP_IE_3"),
            0x04 => f.write_str("TCOMP_IE_4"),
            0x05 => f.write_str("TCOMP_IE_5"),
            0x06 => f.write_str("TCOMP_IE_6"),
            0x07 => f.write_str("TCOMP_IE_7"),
            0x08 => f.write_str("TCOMP_IE_8"),
            0x09 => f.write_str("TCOMP_IE_9"),
            0xffff => f.write_str("TCOMP_IE_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TCOMP_IE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TCOMP_IE_0"),
            0x01 => defmt::write!(f, "TCOMP_IE_1"),
            0x02 => defmt::write!(f, "TCOMP_IE_2"),
            0x03 => defmt::write!(f, "TCOMP_IE_3"),
            0x04 => defmt::write!(f, "TCOMP_IE_4"),
            0x05 => defmt::write!(f, "TCOMP_IE_5"),
            0x06 => defmt::write!(f, "TCOMP_IE_6"),
            0x07 => defmt::write!(f, "TCOMP_IE_7"),
            0x08 => defmt::write!(f, "TCOMP_IE_8"),
            0x09 => defmt::write!(f, "TCOMP_IE_9"),
            0xffff => defmt::write!(f, "TCOMP_IE_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TCOMP_IE {
    #[inline(always)]
    fn from(val: u16) -> TCOMP_IE {
        TCOMP_IE::from_bits(val)
    }
}
impl From<TCOMP_IE> for u16 {
    #[inline(always)]
    fn from(val: TCOMP_IE) -> u16 {
        TCOMP_IE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TCOMP_INT {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    TCOMP_INT_0 = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    TCOMP_INT_1 = 0x01,
}
impl TCOMP_INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TCOMP_INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TCOMP_INT {
    #[inline(always)]
    fn from(val: u8) -> TCOMP_INT {
        TCOMP_INT::from_bits(val)
    }
}
impl From<TCOMP_INT> for u8 {
    #[inline(always)]
    fn from(val: TCOMP_INT) -> u8 {
        TCOMP_INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TEXC_IE {
    #[doc = "Trigger exception interrupts are disabled."]
    TEXC_IE_0 = 0x0,
    #[doc = "Trigger exception interrupts are enabled."]
    TEXC_IE_1 = 0x01,
}
impl TEXC_IE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TEXC_IE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TEXC_IE {
    #[inline(always)]
    fn from(val: u8) -> TEXC_IE {
        TEXC_IE::from_bits(val)
    }
}
impl From<TEXC_IE> for u8 {
    #[inline(always)]
    fn from(val: TEXC_IE) -> u8 {
        TEXC_IE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TEXC_INT {
    #[doc = "No trigger exceptions have occurred."]
    TEXC_INT_0 = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    TEXC_INT_1 = 0x01,
}
impl TEXC_INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TEXC_INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TEXC_INT {
    #[inline(always)]
    fn from(val: u8) -> TEXC_INT {
        TEXC_INT::from_bits(val)
    }
}
impl From<TEXC_INT> for u8 {
    #[inline(always)]
    fn from(val: TEXC_INT) -> u8 {
        TEXC_INT::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TEXC_NUM(u16);
impl TEXC_NUM {
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    pub const TEXC_NUM_0: Self = Self(0x0);
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    pub const TEXC_NUM_1: Self = Self(0x01);
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    pub const TEXC_NUM_2: Self = Self(0x02);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_3: Self = Self(0x03);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_4: Self = Self(0x04);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_5: Self = Self(0x05);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_6: Self = Self(0x06);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_7: Self = Self(0x07);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_8: Self = Self(0x08);
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    pub const TEXC_NUM_9: Self = Self(0x09);
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    pub const TEXC_NUM_65535: Self = Self(0xffff);
}
impl TEXC_NUM {
    pub const fn from_bits(val: u16) -> TEXC_NUM {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TEXC_NUM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TEXC_NUM_0"),
            0x01 => f.write_str("TEXC_NUM_1"),
            0x02 => f.write_str("TEXC_NUM_2"),
            0x03 => f.write_str("TEXC_NUM_3"),
            0x04 => f.write_str("TEXC_NUM_4"),
            0x05 => f.write_str("TEXC_NUM_5"),
            0x06 => f.write_str("TEXC_NUM_6"),
            0x07 => f.write_str("TEXC_NUM_7"),
            0x08 => f.write_str("TEXC_NUM_8"),
            0x09 => f.write_str("TEXC_NUM_9"),
            0xffff => f.write_str("TEXC_NUM_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEXC_NUM {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TEXC_NUM_0"),
            0x01 => defmt::write!(f, "TEXC_NUM_1"),
            0x02 => defmt::write!(f, "TEXC_NUM_2"),
            0x03 => defmt::write!(f, "TEXC_NUM_3"),
            0x04 => defmt::write!(f, "TEXC_NUM_4"),
            0x05 => defmt::write!(f, "TEXC_NUM_5"),
            0x06 => defmt::write!(f, "TEXC_NUM_6"),
            0x07 => defmt::write!(f, "TEXC_NUM_7"),
            0x08 => defmt::write!(f, "TEXC_NUM_8"),
            0x09 => defmt::write!(f, "TEXC_NUM_9"),
            0xffff => defmt::write!(f, "TEXC_NUM_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TEXC_NUM {
    #[inline(always)]
    fn from(val: u16) -> TEXC_NUM {
        TEXC_NUM::from_bits(val)
    }
}
impl From<TEXC_NUM> for u16 {
    #[inline(always)]
    fn from(val: TEXC_NUM) -> u16 {
        TEXC_NUM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TPRI {
    #[doc = "Set to highest priority, Level 1."]
    TPRI_0 = 0x0,
    #[doc = "Set to corresponding priority level."]
    TPRI_1 = 0x01,
    #[doc = "Set to corresponding priority level."]
    TPRI_2 = 0x02,
    #[doc = "Set to corresponding priority level."]
    TPRI_3 = 0x03,
    #[doc = "Set to corresponding priority level."]
    TPRI_4 = 0x04,
    #[doc = "Set to corresponding priority level."]
    TPRI_5 = 0x05,
    #[doc = "Set to corresponding priority level."]
    TPRI_6 = 0x06,
    #[doc = "Set to corresponding priority level."]
    TPRI_7 = 0x07,
    #[doc = "Set to corresponding priority level."]
    TPRI_8 = 0x08,
    #[doc = "Set to corresponding priority level."]
    TPRI_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Set to lowest priority, Level 16."]
    TPRI_15 = 0x0f,
}
impl TPRI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TPRI {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TPRI {
    #[inline(always)]
    fn from(val: u8) -> TPRI {
        TPRI::from_bits(val)
    }
}
impl From<TPRI> for u8 {
    #[inline(always)]
    fn from(val: TPRI) -> u8 {
        TPRI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TPRICTRL {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0 = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    TPRICTRL_1 = 0x01,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    TPRICTRL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl TPRICTRL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TPRICTRL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TPRICTRL {
    #[inline(always)]
    fn from(val: u8) -> TPRICTRL {
        TPRICTRL::from_bits(val)
    }
}
impl From<TPRICTRL> for u8 {
    #[inline(always)]
    fn from(val: TPRICTRL) -> u8 {
        TPRICTRL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRES {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    TRES_0 = 0x0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    TRES_1 = 0x01,
}
impl TRES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRES {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRES {
    #[inline(always)]
    fn from(val: u8) -> TRES {
        TRES::from_bits(val)
    }
}
impl From<TRES> for u8 {
    #[inline(always)]
    fn from(val: TRES) -> u8 {
        TRES::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRGACT {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    TRGACT_0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRGACT_1 = 0x01,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    TRGACT_2 = 0x02,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_3 = 0x03,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_4 = 0x04,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_5 = 0x05,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_6 = 0x06,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_7 = 0x07,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_8 = 0x08,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl TRGACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRGACT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRGACT {
    #[inline(always)]
    fn from(val: u8) -> TRGACT {
        TRGACT::from_bits(val)
    }
}
impl From<TRGACT> for u8 {
    #[inline(always)]
    fn from(val: TRGACT) -> u8 {
        TRGACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TSRC {
    #[doc = "Trigger source 0 initiated this conversion."]
    TSRC_0 = 0x0,
    #[doc = "Trigger source 1 initiated this conversion."]
    TSRC_1 = 0x01,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_2 = 0x02,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_3 = 0x03,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_4 = 0x04,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_5 = 0x05,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_6 = 0x06,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_7 = 0x07,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_8 = 0x08,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Trigger source 15 initiated this conversion."]
    TSRC_15 = 0x0f,
}
impl TSRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TSRC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TSRC {
    #[inline(always)]
    fn from(val: u8) -> TSRC {
        TSRC::from_bits(val)
    }
}
impl From<TSRC> for u8 {
    #[inline(always)]
    fn from(val: TSRC) -> u8 {
        TSRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VALID {
    #[doc = "FIFO is empty. Discard any read from RESFIFO."]
    VALID_0 = 0x0,
    #[doc = "FIFO record read from RESFIFO is valid."]
    VALID_1 = 0x01,
}
impl VALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VALID {
    #[inline(always)]
    fn from(val: u8) -> VALID {
        VALID::from_bits(val)
    }
}
impl From<VALID> for u8 {
    #[inline(always)]
    fn from(val: VALID) -> u8 {
        VALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VR1RNGI {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    VR1RNGI_0 = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    VR1RNGI_1 = 0x01,
}
impl VR1RNGI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VR1RNGI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VR1RNGI {
    #[inline(always)]
    fn from(val: u8) -> VR1RNGI {
        VR1RNGI::from_bits(val)
    }
}
impl From<VR1RNGI> for u8 {
    #[inline(always)]
    fn from(val: VR1RNGI) -> u8 {
        VR1RNGI::to_bits(val)
    }
}
