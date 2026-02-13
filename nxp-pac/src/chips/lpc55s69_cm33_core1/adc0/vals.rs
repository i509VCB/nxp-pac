#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcActive {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    ADC_ACTIVE_0 = 0x0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    ADC_ACTIVE_1 = 0x01,
}
impl AdcActive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcActive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcActive {
    #[inline(always)]
    fn from(val: u8) -> AdcActive {
        AdcActive::from_bits(val)
    }
}
impl From<AdcActive> for u8 {
    #[inline(always)]
    fn from(val: AdcActive) -> u8 {
        AdcActive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adcen {
    #[doc = "ADC is disabled."]
    ADCEN_0 = 0x0,
    #[doc = "ADC is enabled."]
    ADCEN_1 = 0x01,
}
impl Adcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcen {
    #[inline(always)]
    fn from(val: u8) -> Adcen {
        Adcen::from_bits(val)
    }
}
impl From<Adcen> for u8 {
    #[inline(always)]
    fn from(val: Adcen) -> u8 {
        Adcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalAvgs {
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
impl CalAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalAvgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalAvgs {
    #[inline(always)]
    fn from(val: u8) -> CalAvgs {
        CalAvgs::from_bits(val)
    }
}
impl From<CalAvgs> for u8 {
    #[inline(always)]
    fn from(val: CalAvgs) -> u8 {
        CalAvgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalRdy {
    #[doc = "Calibration is incomplete or hasn't been ran."]
    CAL_RDY_0 = 0x0,
    #[doc = "The ADC is calibrated."]
    CAL_RDY_1 = 0x01,
}
impl CalRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalRdy {
    #[inline(always)]
    fn from(val: u8) -> CalRdy {
        CalRdy::from_bits(val)
    }
}
impl From<CalRdy> for u8 {
    #[inline(always)]
    fn from(val: CalRdy) -> u8 {
        CalRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalReq {
    #[doc = "No request for auto-calibration has been made."]
    CAL_REQ_0 = 0x0,
    #[doc = "A request for auto-calibration has been made"]
    CAL_REQ_1 = 0x01,
}
impl CalReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalReq {
    #[inline(always)]
    fn from(val: u8) -> CalReq {
        CalReq::from_bits(val)
    }
}
impl From<CalReq> for u8 {
    #[inline(always)]
    fn from(val: CalReq) -> u8 {
        CalReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofs {
    #[doc = "Calibration function disabled"]
    CALOFS_0 = 0x0,
    #[doc = "Request for offset calibration function"]
    CALOFS_1 = 0x01,
}
impl Calofs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofs {
    #[inline(always)]
    fn from(val: u8) -> Calofs {
        Calofs::from_bits(val)
    }
}
impl From<Calofs> for u8 {
    #[inline(always)]
    fn from(val: Calofs) -> u8 {
        Calofs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofsi {
    #[doc = "Calibration Not Implemented."]
    CALOFSI_0 = 0x0,
    #[doc = "Calibration Implemented."]
    CALOFSI_1 = 0x01,
}
impl Calofsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsi {
    #[inline(always)]
    fn from(val: u8) -> Calofsi {
        Calofsi::from_bits(val)
    }
}
impl From<Calofsi> for u8 {
    #[inline(always)]
    fn from(val: Calofsi) -> u8 {
        Calofsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdact {
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
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdact {
    #[inline(always)]
    fn from(val: u8) -> Cmdact {
        Cmdact::from_bits(val)
    }
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(val: Cmdact) -> u8 {
        Cmdact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Avgs {
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
impl Cmdh10Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Avgs {
        Cmdh10Avgs::from_bits(val)
    }
}
impl From<Cmdh10Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Avgs) -> u8 {
        Cmdh10Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Loop {
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
impl Cmdh10Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Loop {
        Cmdh10Loop::from_bits(val)
    }
}
impl From<Cmdh10Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Loop) -> u8 {
        Cmdh10Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh10Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Lwi {
        Cmdh10Lwi::from_bits(val)
    }
}
impl From<Cmdh10Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Lwi) -> u8 {
        Cmdh10Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh10Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Next {
        Cmdh10Next::from_bits(val)
    }
}
impl From<Cmdh10Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Next) -> u8 {
        Cmdh10Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10Sts {
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
impl Cmdh10Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Sts {
        Cmdh10Sts::from_bits(val)
    }
}
impl From<Cmdh10Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Sts) -> u8 {
        Cmdh10Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh10WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh10WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10WaitTrig {
        Cmdh10WaitTrig::from_bits(val)
    }
}
impl From<Cmdh10WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10WaitTrig) -> u8 {
        Cmdh10WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Avgs {
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
impl Cmdh11Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Avgs {
        Cmdh11Avgs::from_bits(val)
    }
}
impl From<Cmdh11Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Avgs) -> u8 {
        Cmdh11Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Loop {
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
impl Cmdh11Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Loop {
        Cmdh11Loop::from_bits(val)
    }
}
impl From<Cmdh11Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Loop) -> u8 {
        Cmdh11Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh11Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Lwi {
        Cmdh11Lwi::from_bits(val)
    }
}
impl From<Cmdh11Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Lwi) -> u8 {
        Cmdh11Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh11Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Next {
        Cmdh11Next::from_bits(val)
    }
}
impl From<Cmdh11Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Next) -> u8 {
        Cmdh11Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11Sts {
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
impl Cmdh11Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Sts {
        Cmdh11Sts::from_bits(val)
    }
}
impl From<Cmdh11Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Sts) -> u8 {
        Cmdh11Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh11WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh11WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11WaitTrig {
        Cmdh11WaitTrig::from_bits(val)
    }
}
impl From<Cmdh11WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11WaitTrig) -> u8 {
        Cmdh11WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Avgs {
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
impl Cmdh12Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Avgs {
        Cmdh12Avgs::from_bits(val)
    }
}
impl From<Cmdh12Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Avgs) -> u8 {
        Cmdh12Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Loop {
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
impl Cmdh12Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Loop {
        Cmdh12Loop::from_bits(val)
    }
}
impl From<Cmdh12Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Loop) -> u8 {
        Cmdh12Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh12Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Lwi {
        Cmdh12Lwi::from_bits(val)
    }
}
impl From<Cmdh12Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Lwi) -> u8 {
        Cmdh12Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh12Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Next {
        Cmdh12Next::from_bits(val)
    }
}
impl From<Cmdh12Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Next) -> u8 {
        Cmdh12Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12Sts {
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
impl Cmdh12Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Sts {
        Cmdh12Sts::from_bits(val)
    }
}
impl From<Cmdh12Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Sts) -> u8 {
        Cmdh12Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh12WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh12WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12WaitTrig {
        Cmdh12WaitTrig::from_bits(val)
    }
}
impl From<Cmdh12WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12WaitTrig) -> u8 {
        Cmdh12WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Avgs {
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
impl Cmdh13Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Avgs {
        Cmdh13Avgs::from_bits(val)
    }
}
impl From<Cmdh13Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Avgs) -> u8 {
        Cmdh13Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Loop {
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
impl Cmdh13Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Loop {
        Cmdh13Loop::from_bits(val)
    }
}
impl From<Cmdh13Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Loop) -> u8 {
        Cmdh13Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh13Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Lwi {
        Cmdh13Lwi::from_bits(val)
    }
}
impl From<Cmdh13Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Lwi) -> u8 {
        Cmdh13Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh13Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Next {
        Cmdh13Next::from_bits(val)
    }
}
impl From<Cmdh13Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Next) -> u8 {
        Cmdh13Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13Sts {
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
impl Cmdh13Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Sts {
        Cmdh13Sts::from_bits(val)
    }
}
impl From<Cmdh13Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Sts) -> u8 {
        Cmdh13Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh13WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh13WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13WaitTrig {
        Cmdh13WaitTrig::from_bits(val)
    }
}
impl From<Cmdh13WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13WaitTrig) -> u8 {
        Cmdh13WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Avgs {
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
impl Cmdh14Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Avgs {
        Cmdh14Avgs::from_bits(val)
    }
}
impl From<Cmdh14Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Avgs) -> u8 {
        Cmdh14Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Loop {
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
impl Cmdh14Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Loop {
        Cmdh14Loop::from_bits(val)
    }
}
impl From<Cmdh14Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Loop) -> u8 {
        Cmdh14Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh14Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Lwi {
        Cmdh14Lwi::from_bits(val)
    }
}
impl From<Cmdh14Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Lwi) -> u8 {
        Cmdh14Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh14Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Next {
        Cmdh14Next::from_bits(val)
    }
}
impl From<Cmdh14Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Next) -> u8 {
        Cmdh14Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14Sts {
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
impl Cmdh14Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Sts {
        Cmdh14Sts::from_bits(val)
    }
}
impl From<Cmdh14Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Sts) -> u8 {
        Cmdh14Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh14WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh14WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14WaitTrig {
        Cmdh14WaitTrig::from_bits(val)
    }
}
impl From<Cmdh14WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14WaitTrig) -> u8 {
        Cmdh14WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Avgs {
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
impl Cmdh15Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Avgs {
        Cmdh15Avgs::from_bits(val)
    }
}
impl From<Cmdh15Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Avgs) -> u8 {
        Cmdh15Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Loop {
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
impl Cmdh15Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Loop {
        Cmdh15Loop::from_bits(val)
    }
}
impl From<Cmdh15Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Loop) -> u8 {
        Cmdh15Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh15Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Lwi {
        Cmdh15Lwi::from_bits(val)
    }
}
impl From<Cmdh15Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Lwi) -> u8 {
        Cmdh15Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh15Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Next {
        Cmdh15Next::from_bits(val)
    }
}
impl From<Cmdh15Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Next) -> u8 {
        Cmdh15Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15Sts {
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
impl Cmdh15Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Sts {
        Cmdh15Sts::from_bits(val)
    }
}
impl From<Cmdh15Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Sts) -> u8 {
        Cmdh15Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh15WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh15WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15WaitTrig {
        Cmdh15WaitTrig::from_bits(val)
    }
}
impl From<Cmdh15WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15WaitTrig) -> u8 {
        Cmdh15WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Avgs {
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
impl Cmdh1Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Avgs {
        Cmdh1Avgs::from_bits(val)
    }
}
impl From<Cmdh1Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Avgs) -> u8 {
        Cmdh1Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh1Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Cmpen {
        Cmdh1Cmpen::from_bits(val)
    }
}
impl From<Cmdh1Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Cmpen) -> u8 {
        Cmdh1Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Loop {
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
impl Cmdh1Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Loop {
        Cmdh1Loop::from_bits(val)
    }
}
impl From<Cmdh1Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Loop) -> u8 {
        Cmdh1Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh1Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Lwi {
        Cmdh1Lwi::from_bits(val)
    }
}
impl From<Cmdh1Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Lwi) -> u8 {
        Cmdh1Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh1Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Next {
        Cmdh1Next::from_bits(val)
    }
}
impl From<Cmdh1Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Next) -> u8 {
        Cmdh1Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1Sts {
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
impl Cmdh1Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Sts {
        Cmdh1Sts::from_bits(val)
    }
}
impl From<Cmdh1Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Sts) -> u8 {
        Cmdh1Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh1WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh1WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1WaitTrig {
        Cmdh1WaitTrig::from_bits(val)
    }
}
impl From<Cmdh1WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1WaitTrig) -> u8 {
        Cmdh1WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Avgs {
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
impl Cmdh2Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Avgs {
        Cmdh2Avgs::from_bits(val)
    }
}
impl From<Cmdh2Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Avgs) -> u8 {
        Cmdh2Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh2Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Cmpen {
        Cmdh2Cmpen::from_bits(val)
    }
}
impl From<Cmdh2Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Cmpen) -> u8 {
        Cmdh2Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Loop {
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
impl Cmdh2Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Loop {
        Cmdh2Loop::from_bits(val)
    }
}
impl From<Cmdh2Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Loop) -> u8 {
        Cmdh2Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh2Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Lwi {
        Cmdh2Lwi::from_bits(val)
    }
}
impl From<Cmdh2Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Lwi) -> u8 {
        Cmdh2Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh2Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Next {
        Cmdh2Next::from_bits(val)
    }
}
impl From<Cmdh2Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Next) -> u8 {
        Cmdh2Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2Sts {
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
impl Cmdh2Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Sts {
        Cmdh2Sts::from_bits(val)
    }
}
impl From<Cmdh2Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Sts) -> u8 {
        Cmdh2Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh2WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh2WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2WaitTrig {
        Cmdh2WaitTrig::from_bits(val)
    }
}
impl From<Cmdh2WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2WaitTrig) -> u8 {
        Cmdh2WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Avgs {
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
impl Cmdh3Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Avgs {
        Cmdh3Avgs::from_bits(val)
    }
}
impl From<Cmdh3Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Avgs) -> u8 {
        Cmdh3Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh3Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Cmpen {
        Cmdh3Cmpen::from_bits(val)
    }
}
impl From<Cmdh3Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Cmpen) -> u8 {
        Cmdh3Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Loop {
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
impl Cmdh3Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Loop {
        Cmdh3Loop::from_bits(val)
    }
}
impl From<Cmdh3Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Loop) -> u8 {
        Cmdh3Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh3Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Lwi {
        Cmdh3Lwi::from_bits(val)
    }
}
impl From<Cmdh3Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Lwi) -> u8 {
        Cmdh3Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh3Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Next {
        Cmdh3Next::from_bits(val)
    }
}
impl From<Cmdh3Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Next) -> u8 {
        Cmdh3Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3Sts {
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
impl Cmdh3Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Sts {
        Cmdh3Sts::from_bits(val)
    }
}
impl From<Cmdh3Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Sts) -> u8 {
        Cmdh3Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh3WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh3WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3WaitTrig {
        Cmdh3WaitTrig::from_bits(val)
    }
}
impl From<Cmdh3WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3WaitTrig) -> u8 {
        Cmdh3WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Avgs {
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
impl Cmdh4Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Avgs {
        Cmdh4Avgs::from_bits(val)
    }
}
impl From<Cmdh4Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Avgs) -> u8 {
        Cmdh4Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh4Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Cmpen {
        Cmdh4Cmpen::from_bits(val)
    }
}
impl From<Cmdh4Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Cmpen) -> u8 {
        Cmdh4Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Loop {
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
impl Cmdh4Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Loop {
        Cmdh4Loop::from_bits(val)
    }
}
impl From<Cmdh4Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Loop) -> u8 {
        Cmdh4Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh4Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Lwi {
        Cmdh4Lwi::from_bits(val)
    }
}
impl From<Cmdh4Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Lwi) -> u8 {
        Cmdh4Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh4Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Next {
        Cmdh4Next::from_bits(val)
    }
}
impl From<Cmdh4Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Next) -> u8 {
        Cmdh4Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4Sts {
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
impl Cmdh4Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Sts {
        Cmdh4Sts::from_bits(val)
    }
}
impl From<Cmdh4Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Sts) -> u8 {
        Cmdh4Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh4WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh4WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4WaitTrig {
        Cmdh4WaitTrig::from_bits(val)
    }
}
impl From<Cmdh4WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4WaitTrig) -> u8 {
        Cmdh4WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Avgs {
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
impl Cmdh5Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Avgs {
        Cmdh5Avgs::from_bits(val)
    }
}
impl From<Cmdh5Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Avgs) -> u8 {
        Cmdh5Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Loop {
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
impl Cmdh5Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Loop {
        Cmdh5Loop::from_bits(val)
    }
}
impl From<Cmdh5Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Loop) -> u8 {
        Cmdh5Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh5Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Lwi {
        Cmdh5Lwi::from_bits(val)
    }
}
impl From<Cmdh5Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Lwi) -> u8 {
        Cmdh5Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh5Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Next {
        Cmdh5Next::from_bits(val)
    }
}
impl From<Cmdh5Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Next) -> u8 {
        Cmdh5Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Sts {
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
impl Cmdh5Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Sts {
        Cmdh5Sts::from_bits(val)
    }
}
impl From<Cmdh5Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Sts) -> u8 {
        Cmdh5Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh5WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5WaitTrig {
        Cmdh5WaitTrig::from_bits(val)
    }
}
impl From<Cmdh5WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5WaitTrig) -> u8 {
        Cmdh5WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Avgs {
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
impl Cmdh6Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Avgs {
        Cmdh6Avgs::from_bits(val)
    }
}
impl From<Cmdh6Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Avgs) -> u8 {
        Cmdh6Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Loop {
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
impl Cmdh6Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Loop {
        Cmdh6Loop::from_bits(val)
    }
}
impl From<Cmdh6Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Loop) -> u8 {
        Cmdh6Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh6Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Lwi {
        Cmdh6Lwi::from_bits(val)
    }
}
impl From<Cmdh6Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Lwi) -> u8 {
        Cmdh6Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh6Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Next {
        Cmdh6Next::from_bits(val)
    }
}
impl From<Cmdh6Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Next) -> u8 {
        Cmdh6Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Sts {
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
impl Cmdh6Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Sts {
        Cmdh6Sts::from_bits(val)
    }
}
impl From<Cmdh6Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Sts) -> u8 {
        Cmdh6Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh6WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6WaitTrig {
        Cmdh6WaitTrig::from_bits(val)
    }
}
impl From<Cmdh6WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6WaitTrig) -> u8 {
        Cmdh6WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Avgs {
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
impl Cmdh7Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Avgs {
        Cmdh7Avgs::from_bits(val)
    }
}
impl From<Cmdh7Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Avgs) -> u8 {
        Cmdh7Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Loop {
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
impl Cmdh7Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Loop {
        Cmdh7Loop::from_bits(val)
    }
}
impl From<Cmdh7Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Loop) -> u8 {
        Cmdh7Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh7Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Lwi {
        Cmdh7Lwi::from_bits(val)
    }
}
impl From<Cmdh7Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Lwi) -> u8 {
        Cmdh7Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh7Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Next {
        Cmdh7Next::from_bits(val)
    }
}
impl From<Cmdh7Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Next) -> u8 {
        Cmdh7Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Sts {
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
impl Cmdh7Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Sts {
        Cmdh7Sts::from_bits(val)
    }
}
impl From<Cmdh7Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Sts) -> u8 {
        Cmdh7Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh7WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7WaitTrig {
        Cmdh7WaitTrig::from_bits(val)
    }
}
impl From<Cmdh7WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7WaitTrig) -> u8 {
        Cmdh7WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Avgs {
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
impl Cmdh8Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Avgs {
        Cmdh8Avgs::from_bits(val)
    }
}
impl From<Cmdh8Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Avgs) -> u8 {
        Cmdh8Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Loop {
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
impl Cmdh8Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Loop {
        Cmdh8Loop::from_bits(val)
    }
}
impl From<Cmdh8Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Loop) -> u8 {
        Cmdh8Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh8Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Lwi {
        Cmdh8Lwi::from_bits(val)
    }
}
impl From<Cmdh8Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Lwi) -> u8 {
        Cmdh8Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh8Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Next {
        Cmdh8Next::from_bits(val)
    }
}
impl From<Cmdh8Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Next) -> u8 {
        Cmdh8Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8Sts {
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
impl Cmdh8Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Sts {
        Cmdh8Sts::from_bits(val)
    }
}
impl From<Cmdh8Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Sts) -> u8 {
        Cmdh8Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh8WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh8WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8WaitTrig {
        Cmdh8WaitTrig::from_bits(val)
    }
}
impl From<Cmdh8WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8WaitTrig) -> u8 {
        Cmdh8WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Avgs {
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
impl Cmdh9Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Avgs {
        Cmdh9Avgs::from_bits(val)
    }
}
impl From<Cmdh9Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Avgs) -> u8 {
        Cmdh9Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Loop {
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
impl Cmdh9Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Loop {
        Cmdh9Loop::from_bits(val)
    }
}
impl From<Cmdh9Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Loop) -> u8 {
        Cmdh9Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh9Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Lwi {
        Cmdh9Lwi::from_bits(val)
    }
}
impl From<Cmdh9Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Lwi) -> u8 {
        Cmdh9Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh9Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Next {
        Cmdh9Next::from_bits(val)
    }
}
impl From<Cmdh9Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Next) -> u8 {
        Cmdh9Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9Sts {
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
impl Cmdh9Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Sts {
        Cmdh9Sts::from_bits(val)
    }
}
impl From<Cmdh9Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Sts) -> u8 {
        Cmdh9Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh9WaitTrig {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0 = 0x0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 0x01,
}
impl Cmdh9WaitTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9WaitTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9WaitTrig {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9WaitTrig {
        Cmdh9WaitTrig::from_bits(val)
    }
}
impl From<Cmdh9WaitTrig> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9WaitTrig) -> u8 {
        Cmdh9WaitTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Adch {
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
impl Cmdl10Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Adch {
        Cmdl10Adch::from_bits(val)
    }
}
impl From<Cmdl10Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Adch) -> u8 {
        Cmdl10Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl10Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Ctype {
        Cmdl10Ctype::from_bits(val)
    }
}
impl From<Cmdl10Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Ctype) -> u8 {
        Cmdl10Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl10Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl10Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Mode {
        Cmdl10Mode::from_bits(val)
    }
}
impl From<Cmdl10Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Mode) -> u8 {
        Cmdl10Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Adch {
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
impl Cmdl11Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Adch {
        Cmdl11Adch::from_bits(val)
    }
}
impl From<Cmdl11Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Adch) -> u8 {
        Cmdl11Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl11Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Ctype {
        Cmdl11Ctype::from_bits(val)
    }
}
impl From<Cmdl11Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Ctype) -> u8 {
        Cmdl11Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl11Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl11Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Mode {
        Cmdl11Mode::from_bits(val)
    }
}
impl From<Cmdl11Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Mode) -> u8 {
        Cmdl11Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Adch {
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
impl Cmdl12Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Adch {
        Cmdl12Adch::from_bits(val)
    }
}
impl From<Cmdl12Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Adch) -> u8 {
        Cmdl12Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl12Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Ctype {
        Cmdl12Ctype::from_bits(val)
    }
}
impl From<Cmdl12Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Ctype) -> u8 {
        Cmdl12Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl12Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl12Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Mode {
        Cmdl12Mode::from_bits(val)
    }
}
impl From<Cmdl12Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Mode) -> u8 {
        Cmdl12Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Adch {
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
impl Cmdl13Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Adch {
        Cmdl13Adch::from_bits(val)
    }
}
impl From<Cmdl13Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Adch) -> u8 {
        Cmdl13Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl13Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Ctype {
        Cmdl13Ctype::from_bits(val)
    }
}
impl From<Cmdl13Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Ctype) -> u8 {
        Cmdl13Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl13Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl13Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Mode {
        Cmdl13Mode::from_bits(val)
    }
}
impl From<Cmdl13Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Mode) -> u8 {
        Cmdl13Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Adch {
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
impl Cmdl14Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Adch {
        Cmdl14Adch::from_bits(val)
    }
}
impl From<Cmdl14Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Adch) -> u8 {
        Cmdl14Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl14Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Ctype {
        Cmdl14Ctype::from_bits(val)
    }
}
impl From<Cmdl14Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Ctype) -> u8 {
        Cmdl14Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl14Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl14Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Mode {
        Cmdl14Mode::from_bits(val)
    }
}
impl From<Cmdl14Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Mode) -> u8 {
        Cmdl14Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Adch {
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
impl Cmdl15Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Adch {
        Cmdl15Adch::from_bits(val)
    }
}
impl From<Cmdl15Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Adch) -> u8 {
        Cmdl15Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl15Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Ctype {
        Cmdl15Ctype::from_bits(val)
    }
}
impl From<Cmdl15Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Ctype) -> u8 {
        Cmdl15Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl15Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl15Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Mode {
        Cmdl15Mode::from_bits(val)
    }
}
impl From<Cmdl15Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Mode) -> u8 {
        Cmdl15Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Adch {
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
impl Cmdl1Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Adch {
        Cmdl1Adch::from_bits(val)
    }
}
impl From<Cmdl1Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Adch) -> u8 {
        Cmdl1Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl1Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Ctype {
        Cmdl1Ctype::from_bits(val)
    }
}
impl From<Cmdl1Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Ctype) -> u8 {
        Cmdl1Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl1Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl1Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Mode {
        Cmdl1Mode::from_bits(val)
    }
}
impl From<Cmdl1Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Mode) -> u8 {
        Cmdl1Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Adch {
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
impl Cmdl2Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Adch {
        Cmdl2Adch::from_bits(val)
    }
}
impl From<Cmdl2Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Adch) -> u8 {
        Cmdl2Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl2Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Ctype {
        Cmdl2Ctype::from_bits(val)
    }
}
impl From<Cmdl2Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Ctype) -> u8 {
        Cmdl2Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl2Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl2Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Mode {
        Cmdl2Mode::from_bits(val)
    }
}
impl From<Cmdl2Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Mode) -> u8 {
        Cmdl2Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Adch {
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
impl Cmdl3Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Adch {
        Cmdl3Adch::from_bits(val)
    }
}
impl From<Cmdl3Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Adch) -> u8 {
        Cmdl3Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl3Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Ctype {
        Cmdl3Ctype::from_bits(val)
    }
}
impl From<Cmdl3Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Ctype) -> u8 {
        Cmdl3Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl3Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl3Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Mode {
        Cmdl3Mode::from_bits(val)
    }
}
impl From<Cmdl3Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Mode) -> u8 {
        Cmdl3Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Adch {
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
impl Cmdl4Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Adch {
        Cmdl4Adch::from_bits(val)
    }
}
impl From<Cmdl4Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Adch) -> u8 {
        Cmdl4Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl4Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Ctype {
        Cmdl4Ctype::from_bits(val)
    }
}
impl From<Cmdl4Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Ctype) -> u8 {
        Cmdl4Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl4Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl4Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Mode {
        Cmdl4Mode::from_bits(val)
    }
}
impl From<Cmdl4Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Mode) -> u8 {
        Cmdl4Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Adch {
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
impl Cmdl5Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Adch {
        Cmdl5Adch::from_bits(val)
    }
}
impl From<Cmdl5Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Adch) -> u8 {
        Cmdl5Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl5Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Ctype {
        Cmdl5Ctype::from_bits(val)
    }
}
impl From<Cmdl5Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Ctype) -> u8 {
        Cmdl5Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl5Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl5Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Mode {
        Cmdl5Mode::from_bits(val)
    }
}
impl From<Cmdl5Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Mode) -> u8 {
        Cmdl5Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Adch {
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
impl Cmdl6Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Adch {
        Cmdl6Adch::from_bits(val)
    }
}
impl From<Cmdl6Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Adch) -> u8 {
        Cmdl6Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl6Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Ctype {
        Cmdl6Ctype::from_bits(val)
    }
}
impl From<Cmdl6Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Ctype) -> u8 {
        Cmdl6Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl6Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl6Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Mode {
        Cmdl6Mode::from_bits(val)
    }
}
impl From<Cmdl6Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Mode) -> u8 {
        Cmdl6Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Adch {
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
impl Cmdl7Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Adch {
        Cmdl7Adch::from_bits(val)
    }
}
impl From<Cmdl7Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Adch) -> u8 {
        Cmdl7Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl7Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Ctype {
        Cmdl7Ctype::from_bits(val)
    }
}
impl From<Cmdl7Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Ctype) -> u8 {
        Cmdl7Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl7Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl7Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Mode {
        Cmdl7Mode::from_bits(val)
    }
}
impl From<Cmdl7Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Mode) -> u8 {
        Cmdl7Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Adch {
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
impl Cmdl8Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Adch {
        Cmdl8Adch::from_bits(val)
    }
}
impl From<Cmdl8Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Adch) -> u8 {
        Cmdl8Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl8Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Ctype {
        Cmdl8Ctype::from_bits(val)
    }
}
impl From<Cmdl8Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Ctype) -> u8 {
        Cmdl8Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl8Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl8Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Mode {
        Cmdl8Mode::from_bits(val)
    }
}
impl From<Cmdl8Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Mode) -> u8 {
        Cmdl8Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Adch {
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
impl Cmdl9Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Adch {
        Cmdl9Adch::from_bits(val)
    }
}
impl From<Cmdl9Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Adch) -> u8 {
        Cmdl9Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0x0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 0x01,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2 = 0x02,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 0x03,
}
impl Cmdl9Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Ctype {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Ctype {
        Cmdl9Ctype::from_bits(val)
    }
}
impl From<Cmdl9Ctype> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Ctype) -> u8 {
        Cmdl9Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdl9Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 0x01,
}
impl Cmdl9Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Mode {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Mode {
        Cmdl9Mode::from_bits(val)
    }
}
impl From<Cmdl9Mode> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Mode) -> u8 {
        Cmdl9Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdsrc {
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
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Cmdsrc {
        Cmdsrc::from_bits(val)
    }
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Cmdsrc) -> u8 {
        Cmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CstLong {
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    CST_LONG_0 = 0x0,
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    CST_LONG_1 = 0x01,
}
impl CstLong {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CstLong {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CstLong {
    #[inline(always)]
    fn from(val: u8) -> CstLong {
        CstLong::from_bits(val)
    }
}
impl From<CstLong> for u8 {
    #[inline(always)]
    fn from(val: CstLong) -> u8 {
        CstLong::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csw {
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
impl Csw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csw {
    #[inline(always)]
    fn from(val: u8) -> Csw {
        Csw::from_bits(val)
    }
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(val: Csw) -> u8 {
        Csw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diffen {
    #[doc = "Differential operation not supported."]
    DIFFEN_0 = 0x0,
    #[doc = "Differential operation supported. CMDLa\\[CTYPE\\] controls fields implemented."]
    DIFFEN_1 = 0x01,
}
impl Diffen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diffen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diffen {
    #[inline(always)]
    fn from(val: u8) -> Diffen {
        Diffen::from_bits(val)
    }
}
impl From<Diffen> for u8 {
    #[inline(always)]
    fn from(val: Diffen) -> u8 {
        Diffen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "ADC is enabled in Doze mode."]
    DOZEN_0 = 0x0,
    #[doc = "ADC is disabled in Doze mode."]
    DOZEN_1 = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelA {
    #[doc = "Result written to FIFO 0"]
    FIFO_SEL_A_0 = 0x0,
    #[doc = "Result written to FIFO 1"]
    FIFO_SEL_A_1 = 0x01,
}
impl FifoSelA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelA {
    #[inline(always)]
    fn from(val: u8) -> FifoSelA {
        FifoSelA::from_bits(val)
    }
}
impl From<FifoSelA> for u8 {
    #[inline(always)]
    fn from(val: FifoSelA) -> u8 {
        FifoSelA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelB {
    #[doc = "Result written to FIFO 0"]
    FIFO_SEL_B_0 = 0x0,
    #[doc = "Result written to FIFO 1"]
    FIFO_SEL_B_1 = 0x01,
}
impl FifoSelB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelB {
    #[inline(always)]
    fn from(val: u8) -> FifoSelB {
        FifoSelB::from_bits(val)
    }
}
impl From<FifoSelB> for u8 {
    #[inline(always)]
    fn from(val: FifoSelB) -> u8 {
        FifoSelB::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fifosize(u8);
impl Fifosize {
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
impl Fifosize {
    pub const fn from_bits(val: u8) -> Fifosize {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fifosize {
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
impl defmt::Format for Fifosize {
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
impl From<u8> for Fifosize {
    #[inline(always)]
    fn from(val: u8) -> Fifosize {
        Fifosize::from_bits(val)
    }
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(val: Fifosize) -> u8 {
        Fifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof0 {
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_0 = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_1 = 0x01,
}
impl Fof0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof0 {
    #[inline(always)]
    fn from(val: u8) -> Fof0 {
        Fof0::from_bits(val)
    }
}
impl From<Fof0> for u8 {
    #[inline(always)]
    fn from(val: Fof0) -> u8 {
        Fof0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_0 = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_1 = 0x01,
}
impl Fof1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof1 {
    #[inline(always)]
    fn from(val: u8) -> Fof1 {
        Fof1::from_bits(val)
    }
}
impl From<Fof1> for u8 {
    #[inline(always)]
    fn from(val: Fof1) -> u8 {
        Fof1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Foffm {
    #[doc = "Normal operation. No forced offset."]
    FOFFM_0 = 0x0,
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    FOFFM_1 = 0x01,
}
impl Foffm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Foffm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Foffm {
    #[inline(always)]
    fn from(val: u8) -> Foffm {
        Foffm::from_bits(val)
    }
}
impl From<Foffm> for u8 {
    #[inline(always)]
    fn from(val: Foffm) -> u8 {
        Foffm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Foffm2 {
    #[doc = "Normal operation. No forced offset."]
    FOFFM2_0 = 0x0,
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    FOFFM2_1 = 0x01,
}
impl Foffm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Foffm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Foffm2 {
    #[inline(always)]
    fn from(val: u8) -> Foffm2 {
        Foffm2::from_bits(val)
    }
}
impl From<Foffm2> for u8 {
    #[inline(always)]
    fn from(val: Foffm2) -> u8 {
        Foffm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Foffp {
    #[doc = "Normal operation. No forced offset."]
    FOFFP_0 = 0x0,
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    FOFFP_1 = 0x01,
}
impl Foffp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Foffp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Foffp {
    #[inline(always)]
    fn from(val: u8) -> Foffp {
        Foffp::from_bits(val)
    }
}
impl From<Foffp> for u8 {
    #[inline(always)]
    fn from(val: Foffp) -> u8 {
        Foffp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Foffp2 {
    #[doc = "Normal operation. No forced offset."]
    FOFFP2_0 = 0x0,
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    FOFFP2_1 = 0x01,
}
impl Foffp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Foffp2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Foffp2 {
    #[inline(always)]
    fn from(val: u8) -> Foffp2 {
        Foffp2::from_bits(val)
    }
}
impl From<Foffp2> for u8 {
    #[inline(always)]
    fn from(val: Foffp2) -> u8 {
        Foffp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fofie0 {
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0 = 0x0,
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1 = 0x01,
}
impl Fofie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fofie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fofie0 {
    #[inline(always)]
    fn from(val: u8) -> Fofie0 {
        Fofie0::from_bits(val)
    }
}
impl From<Fofie0> for u8 {
    #[inline(always)]
    fn from(val: Fofie0) -> u8 {
        Fofie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fofie1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0 = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1 = 0x01,
}
impl Fofie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fofie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fofie1 {
    #[inline(always)]
    fn from(val: u8) -> Fofie1 {
        Fofie1::from_bits(val)
    }
}
impl From<Fofie1> for u8 {
    #[inline(always)]
    fn from(val: Fofie1) -> u8 {
        Fofie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fwmde0 {
    #[doc = "DMA request disabled."]
    FWMDE0_0 = 0x0,
    #[doc = "DMA request enabled."]
    FWMDE0_1 = 0x01,
}
impl Fwmde0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmde0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmde0 {
    #[inline(always)]
    fn from(val: u8) -> Fwmde0 {
        Fwmde0::from_bits(val)
    }
}
impl From<Fwmde0> for u8 {
    #[inline(always)]
    fn from(val: Fwmde0) -> u8 {
        Fwmde0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fwmde1 {
    #[doc = "DMA request disabled."]
    FWMDE1_0 = 0x0,
    #[doc = "DMA request enabled."]
    FWMDE1_1 = 0x01,
}
impl Fwmde1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmde1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmde1 {
    #[inline(always)]
    fn from(val: u8) -> Fwmde1 {
        Fwmde1::from_bits(val)
    }
}
impl From<Fwmde1> for u8 {
    #[inline(always)]
    fn from(val: Fwmde1) -> u8 {
        Fwmde1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fwmie0 {
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0 = 0x0,
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1 = 0x01,
}
impl Fwmie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmie0 {
    #[inline(always)]
    fn from(val: u8) -> Fwmie0 {
        Fwmie0::from_bits(val)
    }
}
impl From<Fwmie0> for u8 {
    #[inline(always)]
    fn from(val: Fwmie0) -> u8 {
        Fwmie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fwmie1 {
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0 = 0x0,
    #[doc = "FIFO1 watermark interrupts are enabled."]
    FWMIE1_1 = 0x01,
}
impl Fwmie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmie1 {
    #[inline(always)]
    fn from(val: u8) -> Fwmie1 {
        Fwmie1::from_bits(val)
    }
}
impl From<Fwmie1> for u8 {
    #[inline(always)]
    fn from(val: Fwmie1) -> u8 {
        Fwmie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GccRdy {
    #[doc = "The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    RDY_0 = 0x0,
    #[doc = "The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\] register field."]
    RDY_1 = 0x01,
}
impl GccRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GccRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GccRdy {
    #[inline(always)]
    fn from(val: u8) -> GccRdy {
        GccRdy::from_bits(val)
    }
}
impl From<GccRdy> for u8 {
    #[inline(always)]
    fn from(val: GccRdy) -> u8 {
        GccRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GcrRdy {
    #[doc = "The gain offset calculation value is invalid."]
    RDY_0 = 0x0,
    #[doc = "The gain calibration value is valid."]
    RDY_1 = 0x01,
}
impl GcrRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GcrRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GcrRdy {
    #[inline(always)]
    fn from(val: u8) -> GcrRdy {
        GcrRdy::from_bits(val)
    }
}
impl From<GcrRdy> for u8 {
    #[inline(always)]
    fn from(val: GcrRdy) -> u8 {
        GcrRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HptExdi {
    #[doc = "High priority trigger exceptions are enabled."]
    HPT_EXDI_0 = 0x0,
    #[doc = "High priority trigger exceptions are disabled."]
    HPT_EXDI_1 = 0x01,
}
impl HptExdi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HptExdi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HptExdi {
    #[inline(always)]
    fn from(val: u8) -> HptExdi {
        HptExdi::from_bits(val)
    }
}
impl From<HptExdi> for u8 {
    #[inline(always)]
    fn from(val: HptExdi) -> u8 {
        HptExdi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hten {
    #[doc = "Hardware trigger source disabled"]
    HTEN_0 = 0x0,
    #[doc = "Hardware trigger source enabled"]
    HTEN_1 = 0x01,
}
impl Hten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hten {
    #[inline(always)]
    fn from(val: u8) -> Hten {
        Hten::from_bits(val)
    }
}
impl From<Hten> for u8 {
    #[inline(always)]
    fn from(val: Hten) -> u8 {
        Hten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iadcki {
    #[doc = "Internal clock source not implemented."]
    IADCKI_0 = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    IADCKI_1 = 0x01,
}
impl Iadcki {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iadcki {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iadcki {
    #[inline(always)]
    fn from(val: u8) -> Iadcki {
        Iadcki::from_bits(val)
    }
}
impl From<Iadcki> for u8 {
    #[inline(always)]
    fn from(val: Iadcki) -> u8 {
        Iadcki::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopcnt {
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
impl Loopcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopcnt {
    #[inline(always)]
    fn from(val: u8) -> Loopcnt {
        Loopcnt::from_bits(val)
    }
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(val: Loopcnt) -> u8 {
        Loopcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mvi {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MVI_0 = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MVI_1 = 0x01,
}
impl Mvi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mvi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mvi {
    #[inline(always)]
    fn from(val: u8) -> Mvi {
        Mvi::from_bits(val)
    }
}
impl From<Mvi> for u8 {
    #[inline(always)]
    fn from(val: Mvi) -> u8 {
        Mvi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumFifo {
    #[doc = "N/A"]
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
impl NumFifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumFifo {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumFifo {
    #[inline(always)]
    fn from(val: u8) -> NumFifo {
        NumFifo::from_bits(val)
    }
}
impl From<NumFifo> for u8 {
    #[inline(always)]
    fn from(val: NumFifo) -> u8 {
        NumFifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumSec {
    #[doc = "This design supports one single ended conversion at a time."]
    NUM_SEC_0 = 0x0,
    #[doc = "This design supports two simultanious single ended conversions."]
    NUM_SEC_1 = 0x01,
}
impl NumSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumSec {
    #[inline(always)]
    fn from(val: u8) -> NumSec {
        NumSec::from_bits(val)
    }
}
impl From<NumSec> for u8 {
    #[inline(always)]
    fn from(val: NumSec) -> u8 {
        NumSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pauseen {
    #[doc = "Pause operation disabled"]
    PAUSEEN_0 = 0x0,
    #[doc = "Pause operation enabled"]
    PAUSEEN_1 = 0x01,
}
impl Pauseen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pauseen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pauseen {
    #[inline(always)]
    fn from(val: u8) -> Pauseen {
        Pauseen::from_bits(val)
    }
}
impl From<Pauseen> for u8 {
    #[inline(always)]
    fn from(val: Pauseen) -> u8 {
        Pauseen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwren {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0 = 0x0,
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    PWREN_1 = 0x01,
}
impl Pwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwren {
    #[inline(always)]
    fn from(val: u8) -> Pwren {
        Pwren::from_bits(val)
    }
}
impl From<Pwren> for u8 {
    #[inline(always)]
    fn from(val: Pwren) -> u8 {
        Pwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrsel {
    #[doc = "Lowest power setting."]
    PWRSEL_0 = 0x0,
    #[doc = "Higher power setting than 0b0."]
    PWRSEL_1 = 0x01,
    #[doc = "Higher power setting than 0b1."]
    PWRSEL_2 = 0x02,
    #[doc = "Highest power setting."]
    PWRSEL_3 = 0x03,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsel {
    #[inline(always)]
    fn from(val: u8) -> Pwrsel {
        Pwrsel::from_bits(val)
    }
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(val: Pwrsel) -> u8 {
        Pwrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy0 {
    #[doc = "Result FIFO 0 data level not above watermark level."]
    RDY0_0 = 0x0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    RDY0_1 = 0x01,
}
impl Rdy0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy0 {
    #[inline(always)]
    fn from(val: u8) -> Rdy0 {
        Rdy0::from_bits(val)
    }
}
impl From<Rdy0> for u8 {
    #[inline(always)]
    fn from(val: Rdy0) -> u8 {
        Rdy0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy1 {
    #[doc = "Result FIFO1 data level not above watermark level."]
    RDY1_0 = 0x0,
    #[doc = "Result FIFO1 holding data above watermark level."]
    RDY1_1 = 0x01,
}
impl Rdy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy1 {
    #[inline(always)]
    fn from(val: u8) -> Rdy1 {
        Rdy1::from_bits(val)
    }
}
impl From<Rdy1> for u8 {
    #[inline(always)]
    fn from(val: Rdy1) -> u8 {
        Rdy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "(Default) Option 1 setting."]
    REFSEL_0 = 0x0,
    #[doc = "Option 2 setting."]
    REFSEL_1 = 0x01,
    #[doc = "Option 3 setting."]
    REFSEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res {
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    RES_0 = 0x0,
    #[doc = "Up to 16-bit differential/16-bit single ended resolution supported."]
    RES_1 = 0x01,
}
impl Res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res {
    #[inline(always)]
    fn from(val: u8) -> Res {
        Res::from_bits(val)
    }
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(val: Res) -> u8 {
        Res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "ADC logic is not reset."]
    RST_0 = 0x0,
    #[doc = "ADC logic is reset."]
    RST_1 = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo0 {
    #[doc = "No effect."]
    RSTFIFO0_0 = 0x0,
    #[doc = "FIFO 0 is reset."]
    RSTFIFO0_1 = 0x01,
}
impl Rstfifo0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo0 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo0 {
        Rstfifo0::from_bits(val)
    }
}
impl From<Rstfifo0> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo0) -> u8 {
        Rstfifo0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo1 {
    #[doc = "No effect."]
    RSTFIFO1_0 = 0x0,
    #[doc = "FIFO 1 is reset."]
    RSTFIFO1_1 = 0x01,
}
impl Rstfifo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo1 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo1 {
        Rstfifo1::from_bits(val)
    }
}
impl From<Rstfifo1> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo1) -> u8 {
        Rstfifo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt0 {
    #[doc = "No trigger 0 event generated."]
    SWT0_0 = 0x0,
    #[doc = "Trigger 0 event generated."]
    SWT0_1 = 0x01,
}
impl Swt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt0 {
    #[inline(always)]
    fn from(val: u8) -> Swt0 {
        Swt0::from_bits(val)
    }
}
impl From<Swt0> for u8 {
    #[inline(always)]
    fn from(val: Swt0) -> u8 {
        Swt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt1 {
    #[doc = "No trigger 1 event generated."]
    SWT1_0 = 0x0,
    #[doc = "Trigger 1 event generated."]
    SWT1_1 = 0x01,
}
impl Swt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt1 {
    #[inline(always)]
    fn from(val: u8) -> Swt1 {
        Swt1::from_bits(val)
    }
}
impl From<Swt1> for u8 {
    #[inline(always)]
    fn from(val: Swt1) -> u8 {
        Swt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt10 {
    #[doc = "No trigger 10 event generated."]
    SWT10_0 = 0x0,
    #[doc = "Trigger 10 event generated."]
    SWT10_1 = 0x01,
}
impl Swt10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt10 {
    #[inline(always)]
    fn from(val: u8) -> Swt10 {
        Swt10::from_bits(val)
    }
}
impl From<Swt10> for u8 {
    #[inline(always)]
    fn from(val: Swt10) -> u8 {
        Swt10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt11 {
    #[doc = "No trigger 11 event generated."]
    SWT11_0 = 0x0,
    #[doc = "Trigger 11 event generated."]
    SWT11_1 = 0x01,
}
impl Swt11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt11 {
    #[inline(always)]
    fn from(val: u8) -> Swt11 {
        Swt11::from_bits(val)
    }
}
impl From<Swt11> for u8 {
    #[inline(always)]
    fn from(val: Swt11) -> u8 {
        Swt11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt12 {
    #[doc = "No trigger 12 event generated."]
    SWT12_0 = 0x0,
    #[doc = "Trigger 12 event generated."]
    SWT12_1 = 0x01,
}
impl Swt12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt12 {
    #[inline(always)]
    fn from(val: u8) -> Swt12 {
        Swt12::from_bits(val)
    }
}
impl From<Swt12> for u8 {
    #[inline(always)]
    fn from(val: Swt12) -> u8 {
        Swt12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt13 {
    #[doc = "No trigger 13 event generated."]
    SWT13_0 = 0x0,
    #[doc = "Trigger 13 event generated."]
    SWT13_1 = 0x01,
}
impl Swt13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt13 {
    #[inline(always)]
    fn from(val: u8) -> Swt13 {
        Swt13::from_bits(val)
    }
}
impl From<Swt13> for u8 {
    #[inline(always)]
    fn from(val: Swt13) -> u8 {
        Swt13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt14 {
    #[doc = "No trigger 14 event generated."]
    SWT14_0 = 0x0,
    #[doc = "Trigger 14 event generated."]
    SWT14_1 = 0x01,
}
impl Swt14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt14 {
    #[inline(always)]
    fn from(val: u8) -> Swt14 {
        Swt14::from_bits(val)
    }
}
impl From<Swt14> for u8 {
    #[inline(always)]
    fn from(val: Swt14) -> u8 {
        Swt14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt15 {
    #[doc = "No trigger 15 event generated."]
    SWT15_0 = 0x0,
    #[doc = "Trigger 15 event generated."]
    SWT15_1 = 0x01,
}
impl Swt15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt15 {
    #[inline(always)]
    fn from(val: u8) -> Swt15 {
        Swt15::from_bits(val)
    }
}
impl From<Swt15> for u8 {
    #[inline(always)]
    fn from(val: Swt15) -> u8 {
        Swt15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt2 {
    #[doc = "No trigger 2 event generated."]
    SWT2_0 = 0x0,
    #[doc = "Trigger 2 event generated."]
    SWT2_1 = 0x01,
}
impl Swt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt2 {
    #[inline(always)]
    fn from(val: u8) -> Swt2 {
        Swt2::from_bits(val)
    }
}
impl From<Swt2> for u8 {
    #[inline(always)]
    fn from(val: Swt2) -> u8 {
        Swt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt3 {
    #[doc = "No trigger 3 event generated."]
    SWT3_0 = 0x0,
    #[doc = "Trigger 3 event generated."]
    SWT3_1 = 0x01,
}
impl Swt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt3 {
    #[inline(always)]
    fn from(val: u8) -> Swt3 {
        Swt3::from_bits(val)
    }
}
impl From<Swt3> for u8 {
    #[inline(always)]
    fn from(val: Swt3) -> u8 {
        Swt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt4 {
    #[doc = "No trigger 4 event generated."]
    SWT4_0 = 0x0,
    #[doc = "Trigger 4 event generated."]
    SWT4_1 = 0x01,
}
impl Swt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt4 {
    #[inline(always)]
    fn from(val: u8) -> Swt4 {
        Swt4::from_bits(val)
    }
}
impl From<Swt4> for u8 {
    #[inline(always)]
    fn from(val: Swt4) -> u8 {
        Swt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt5 {
    #[doc = "No trigger 5 event generated."]
    SWT5_0 = 0x0,
    #[doc = "Trigger 5 event generated."]
    SWT5_1 = 0x01,
}
impl Swt5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt5 {
    #[inline(always)]
    fn from(val: u8) -> Swt5 {
        Swt5::from_bits(val)
    }
}
impl From<Swt5> for u8 {
    #[inline(always)]
    fn from(val: Swt5) -> u8 {
        Swt5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt6 {
    #[doc = "No trigger 6 event generated."]
    SWT6_0 = 0x0,
    #[doc = "Trigger 6 event generated."]
    SWT6_1 = 0x01,
}
impl Swt6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt6 {
    #[inline(always)]
    fn from(val: u8) -> Swt6 {
        Swt6::from_bits(val)
    }
}
impl From<Swt6> for u8 {
    #[inline(always)]
    fn from(val: Swt6) -> u8 {
        Swt6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt7 {
    #[doc = "No trigger 7 event generated."]
    SWT7_0 = 0x0,
    #[doc = "Trigger 7 event generated."]
    SWT7_1 = 0x01,
}
impl Swt7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt7 {
    #[inline(always)]
    fn from(val: u8) -> Swt7 {
        Swt7::from_bits(val)
    }
}
impl From<Swt7> for u8 {
    #[inline(always)]
    fn from(val: Swt7) -> u8 {
        Swt7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt8 {
    #[doc = "No trigger 8 event generated."]
    SWT8_0 = 0x0,
    #[doc = "Trigger 8 event generated."]
    SWT8_1 = 0x01,
}
impl Swt8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt8 {
    #[inline(always)]
    fn from(val: u8) -> Swt8 {
        Swt8::from_bits(val)
    }
}
impl From<Swt8> for u8 {
    #[inline(always)]
    fn from(val: Swt8) -> u8 {
        Swt8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt9 {
    #[doc = "No trigger 9 event generated."]
    SWT9_0 = 0x0,
    #[doc = "Trigger 9 event generated."]
    SWT9_1 = 0x01,
}
impl Swt9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt9 {
    #[inline(always)]
    fn from(val: u8) -> Swt9 {
        Swt9::from_bits(val)
    }
}
impl From<Swt9> for u8 {
    #[inline(always)]
    fn from(val: Swt9) -> u8 {
        Swt9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0 = 0x0,
    #[doc = "CMD1 is executed"]
    TCMD_1 = 0x01,
    #[doc = "Corresponding CMD is executed"]
    TCMD_2 = 0x02,
    #[doc = "Corresponding CMD is executed"]
    TCMD_3 = 0x03,
    #[doc = "Corresponding CMD is executed"]
    TCMD_4 = 0x04,
    #[doc = "Corresponding CMD is executed"]
    TCMD_5 = 0x05,
    #[doc = "Corresponding CMD is executed"]
    TCMD_6 = 0x06,
    #[doc = "Corresponding CMD is executed"]
    TCMD_7 = 0x07,
    #[doc = "Corresponding CMD is executed"]
    TCMD_8 = 0x08,
    #[doc = "Corresponding CMD is executed"]
    TCMD_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 is executed"]
    TCMD_15 = 0x0f,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmd {
    #[inline(always)]
    fn from(val: u8) -> Tcmd {
        Tcmd::from_bits(val)
    }
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(val: Tcmd) -> u8 {
        Tcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmdres {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    TCMDRES_0 = 0x0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    TCMDRES_1 = 0x01,
}
impl Tcmdres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmdres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmdres {
    #[inline(always)]
    fn from(val: u8) -> Tcmdres {
        Tcmdres::from_bits(val)
    }
}
impl From<Tcmdres> for u8 {
    #[inline(always)]
    fn from(val: Tcmdres) -> u8 {
        Tcmdres::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TcompFlag(u16);
impl TcompFlag {
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
impl TcompFlag {
    pub const fn from_bits(val: u16) -> TcompFlag {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TcompFlag {
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
impl defmt::Format for TcompFlag {
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
impl From<u16> for TcompFlag {
    #[inline(always)]
    fn from(val: u16) -> TcompFlag {
        TcompFlag::from_bits(val)
    }
}
impl From<TcompFlag> for u16 {
    #[inline(always)]
    fn from(val: TcompFlag) -> u16 {
        TcompFlag::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TcompIe(u16);
impl TcompIe {
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
impl TcompIe {
    pub const fn from_bits(val: u16) -> TcompIe {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TcompIe {
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
impl defmt::Format for TcompIe {
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
impl From<u16> for TcompIe {
    #[inline(always)]
    fn from(val: u16) -> TcompIe {
        TcompIe::from_bits(val)
    }
}
impl From<TcompIe> for u16 {
    #[inline(always)]
    fn from(val: TcompIe) -> u16 {
        TcompIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompInt {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    TCOMP_INT_0 = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    TCOMP_INT_1 = 0x01,
}
impl TcompInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompInt {
    #[inline(always)]
    fn from(val: u8) -> TcompInt {
        TcompInt::from_bits(val)
    }
}
impl From<TcompInt> for u8 {
    #[inline(always)]
    fn from(val: TcompInt) -> u8 {
        TcompInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Testen {
    #[doc = "Normal operation. Test configuration not enabled."]
    TESTEN_0 = 0x0,
    #[doc = "Hardware BIST Test in progress."]
    TESTEN_1 = 0x01,
}
impl Testen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Testen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Testen {
    #[inline(always)]
    fn from(val: u8) -> Testen {
        Testen::from_bits(val)
    }
}
impl From<Testen> for u8 {
    #[inline(always)]
    fn from(val: Testen) -> u8 {
        Testen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcIe {
    #[doc = "Trigger exception interrupts are disabled."]
    TEXC_IE_0 = 0x0,
    #[doc = "Trigger exception interrupts are enabled."]
    TEXC_IE_1 = 0x01,
}
impl TexcIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcIe {
    #[inline(always)]
    fn from(val: u8) -> TexcIe {
        TexcIe::from_bits(val)
    }
}
impl From<TexcIe> for u8 {
    #[inline(always)]
    fn from(val: TexcIe) -> u8 {
        TexcIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcInt {
    #[doc = "No trigger exceptions have occurred."]
    TEXC_INT_0 = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    TEXC_INT_1 = 0x01,
}
impl TexcInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcInt {
    #[inline(always)]
    fn from(val: u8) -> TexcInt {
        TexcInt::from_bits(val)
    }
}
impl From<TexcInt> for u8 {
    #[inline(always)]
    fn from(val: TexcInt) -> u8 {
        TexcInt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TexcNum(u16);
impl TexcNum {
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
impl TexcNum {
    pub const fn from_bits(val: u16) -> TexcNum {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TexcNum {
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
impl defmt::Format for TexcNum {
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
impl From<u16> for TexcNum {
    #[inline(always)]
    fn from(val: u16) -> TexcNum {
        TexcNum::from_bits(val)
    }
}
impl From<TexcNum> for u16 {
    #[inline(always)]
    fn from(val: TexcNum) -> u16 {
        TexcNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpri {
    #[doc = "Set to highest priority, Level 1"]
    TPRI_0 = 0x0,
    #[doc = "Set to corresponding priority level"]
    TPRI_1 = 0x01,
    #[doc = "Set to corresponding priority level"]
    TPRI_2 = 0x02,
    #[doc = "Set to corresponding priority level"]
    TPRI_3 = 0x03,
    #[doc = "Set to corresponding priority level"]
    TPRI_4 = 0x04,
    #[doc = "Set to corresponding priority level"]
    TPRI_5 = 0x05,
    #[doc = "Set to corresponding priority level"]
    TPRI_6 = 0x06,
    #[doc = "Set to corresponding priority level"]
    TPRI_7 = 0x07,
    #[doc = "Set to corresponding priority level"]
    TPRI_8 = 0x08,
    #[doc = "Set to corresponding priority level"]
    TPRI_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Set to lowest priority, Level 16"]
    TPRI_15 = 0x0f,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpri {
    #[inline(always)]
    fn from(val: u8) -> Tpri {
        Tpri::from_bits(val)
    }
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(val: Tpri) -> u8 {
        Tpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tprictrl {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0 = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    TPRICTRL_1 = 0x01,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    TPRICTRL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tprictrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tprictrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tprictrl {
    #[inline(always)]
    fn from(val: u8) -> Tprictrl {
        Tprictrl::from_bits(val)
    }
}
impl From<Tprictrl> for u8 {
    #[inline(always)]
    fn from(val: Tprictrl) -> u8 {
        Tprictrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tres {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    TRES_0 = 0x0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    TRES_1 = 0x01,
}
impl Tres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tres {
    #[inline(always)]
    fn from(val: u8) -> Tres {
        Tres::from_bits(val)
    }
}
impl From<Tres> for u8 {
    #[inline(always)]
    fn from(val: Tres) -> u8 {
        Tres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgact {
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
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgact {
    #[inline(always)]
    fn from(val: u8) -> Trgact {
        Trgact::from_bits(val)
    }
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(val: Trgact) -> u8 {
        Trgact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrc {
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
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Valid {
    #[doc = "FIFO is empty. Discard any read from RESFIFO."]
    VALID_0 = 0x0,
    #[doc = "FIFO record read from RESFIFO is valid."]
    VALID_1 = 0x01,
}
impl Valid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Valid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Valid {
    #[inline(always)]
    fn from(val: u8) -> Valid {
        Valid::from_bits(val)
    }
}
impl From<Valid> for u8 {
    #[inline(always)]
    fn from(val: Valid) -> u8 {
        Valid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vr1rngi {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    VR1RNGI_0 = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    VR1RNGI_1 = 0x01,
}
impl Vr1rngi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vr1rngi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vr1rngi {
    #[inline(always)]
    fn from(val: u8) -> Vr1rngi {
        Vr1rngi::from_bits(val)
    }
}
impl From<Vr1rngi> for u8 {
    #[inline(always)]
    fn from(val: Vr1rngi) -> u8 {
        Vr1rngi::to_bits(val)
    }
}
