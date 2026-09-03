#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcActive {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    NotActive = 0x0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    Busy = 0x01,
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
pub enum CalAvgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CalAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalAvgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
    NotSet = 0x0,
    #[doc = "The ADC is calibrated."]
    HardwareCalStepCompleted = 0x01,
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
    #[doc = "No request for hardware calibration has been made."]
    NoCalibrationRequest = 0x0,
    #[doc = "A request for hardware calibration has been made."]
    CalibrationRequestPending = 0x01,
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
pub enum Calhs {
    #[doc = "No request for high speed mode trim has been made."]
    NoActiveHsTrimRequest = 0x0,
    #[doc = "Request for high speed mode trim has been made."]
    HsTrimRequestPending = 0x01,
}
impl Calhs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calhs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calhs {
    #[inline(always)]
    fn from(val: u8) -> Calhs {
        Calhs::from_bits(val)
    }
}
impl From<Calhs> for u8 {
    #[inline(always)]
    fn from(val: Calhs) -> u8 {
        Calhs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofs {
    #[doc = "No request for offset calibration has been made."]
    NoActiveOffsetCalibrationRequest = 0x0,
    #[doc = "Request for offset calibration function."]
    OffsetCalibrationRequestPending = 0x01,
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
    CalFunctionNotAvailable = 0x0,
    #[doc = "Calibration Implemented."]
    CalFunctionAvailable = 0x01,
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
    NoCommandActive = 0x0,
    #[doc = "Command 1 currently being executed."]
    Command1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    Command2 = 0x02,
    #[doc = "Associated command number is currently being executed."]
    CommandX3 = 0x03,
    #[doc = "Associated command number is currently being executed."]
    CommandX4 = 0x04,
    #[doc = "Associated command number is currently being executed."]
    CommandX5 = 0x05,
    #[doc = "Associated command number is currently being executed."]
    CommandX6 = 0x06,
    #[doc = "Associated command number is currently being executed."]
    CommandX7 = 0x07,
}
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Cmdh1Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh1Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
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
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh1Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh1Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdh2Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh2Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
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
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh2Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh2Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdh3Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh3Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
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
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh3Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh3Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdh4Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh4Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
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
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh4Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh4Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdh5Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh5Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Cmdh5Cmpen {
    #[doc = "Compare disabled."]
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
}
impl Cmdh5Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Cmpen {
        Cmdh5Cmpen::from_bits(val)
    }
}
impl From<Cmdh5Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Cmpen) -> u8 {
        Cmdh5Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh5Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh5Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh5Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdh6Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh6Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Cmdh6Cmpen {
    #[doc = "Compare disabled."]
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
}
impl Cmdh6Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Cmpen {
        Cmdh6Cmpen::from_bits(val)
    }
}
impl From<Cmdh6Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Cmpen) -> u8 {
        Cmdh6Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh6Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh6Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh6Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdh7Avgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdh7Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
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
pub enum Cmdh7Cmpen {
    #[doc = "Compare disabled."]
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
}
impl Cmdh7Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Cmpen {
        Cmdh7Cmpen::from_bits(val)
    }
}
impl From<Cmdh7Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Cmpen) -> u8 {
        Cmdh7Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdh7Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CmdExec1x = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CmdExec2x = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
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
pub enum Cmdh7Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DoCmd7Next = 0x07,
}
impl Cmdh7Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Next {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    Sample5p5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    Sample7p5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    Sample11p5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    Sample19p5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    Sample35p5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    Sample67p5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    Sample131p5 = 0x07,
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
pub enum Cmdl1Adch {
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
    #[doc = "Select CH0A."]
    SelectCh0 = 0x0,
    #[doc = "Select CH1A."]
    SelectCh1 = 0x01,
    #[doc = "Select CH2A."]
    SelectCh2 = 0x02,
    #[doc = "Select CH3A."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA."]
    SelectCorrespondingChannel9 = 0x09,
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
    #[doc = "Select CH30A."]
    SelectCh30 = 0x1e,
    #[doc = "Select CH31A."]
    SelectCh31 = 0x1f,
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
    SingleEndedASideChannel = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    Data16Bits = 0x01,
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
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    NotValid = 0x0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    Cmd1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd6 = 0x06,
    #[doc = "CMD7 buffer used as control settings for this conversion."]
    Cmd7 = 0x07,
}
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Csw {
    #[doc = "Channel scaling not supported."]
    CscaleNotSupported = 0x0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    BitWidth1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    BitWidth6 = 0x06,
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
    DifferentialNotSupported = 0x0,
    #[doc = "Differential operation supported."]
    DifferentialSupported = 0x01,
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
    #[doc = "ADC is enabled in low power mode."]
    Enabled = 0x0,
    #[doc = "ADC is disabled in low power mode."]
    Disabled = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fifosize(u8);
impl Fifosize {
    #[doc = "Result FIFO depth = 2 dataword."]
    pub const Entries2: Self = Self(0x01);
    #[doc = "Result FIFO depth = 4 datawords."]
    pub const Entries4: Self = Self(0x04);
    #[doc = "Result FIFO depth = 8 datawords."]
    pub const Entries8: Self = Self(0x08);
    #[doc = "Result FIFO depth = 16 datawords."]
    pub const Entries16: Self = Self(0x10);
    #[doc = "Result FIFO depth = 32 datawords."]
    pub const Entries32: Self = Self(0x20);
    #[doc = "Result FIFO depth = 64 datawords."]
    pub const Entries64: Self = Self(0x40);
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
            0x01 => f.write_str("Entries2"),
            0x04 => f.write_str("Entries4"),
            0x08 => f.write_str("Entries8"),
            0x10 => f.write_str("Entries16"),
            0x20 => f.write_str("Entries32"),
            0x40 => f.write_str("Entries64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifosize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Entries2"),
            0x04 => defmt::write!(f, "Entries4"),
            0x08 => defmt::write!(f, "Entries8"),
            0x10 => defmt::write!(f, "Entries16"),
            0x20 => defmt::write!(f, "Entries32"),
            0x40 => defmt::write!(f, "Entries64"),
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
    NoOverflow = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    OverflowDetected = 0x01,
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
pub enum Gcc0Rdy {
    #[doc = "The GAIN_CAL value is invalid. Run the hardware calibration routine for this value to be set."]
    GainCalNotValid = 0x0,
    #[doc = "The GAIN_CAL value is valid. GAIN_CAL should be used by software to derive GCRa\\[GCALR\\]."]
    HardwareCalRoutineCompleted = 0x01,
}
impl Gcc0Rdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gcc0Rdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gcc0Rdy {
    #[inline(always)]
    fn from(val: u8) -> Gcc0Rdy {
        Gcc0Rdy::from_bits(val)
    }
}
impl From<Gcc0Rdy> for u8 {
    #[inline(always)]
    fn from(val: Gcc0Rdy) -> u8 {
        Gcc0Rdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HptExdi {
    #[doc = "High priority trigger exceptions are enabled."]
    Enabled = 0x0,
    #[doc = "High priority trigger exceptions are disabled."]
    Disabled = 0x01,
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
pub enum Hsextra {
    #[doc = "No extra cycle added."]
    Hsextra0 = 0x0,
    #[doc = "Extra cycle added."]
    Hsextra1 = 0x01,
}
impl Hsextra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsextra {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsextra {
    #[inline(always)]
    fn from(val: u8) -> Hsextra {
        Hsextra::from_bits(val)
    }
}
impl From<Hsextra> for u8 {
    #[inline(always)]
    fn from(val: Hsextra) -> u8 {
        Hsextra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iadcki {
    #[doc = "Internal clock source not implemented."]
    InternalClkNotAvailable = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    InternalClkAvailable = 0x01,
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
    Result1 = 0x0,
    #[doc = "Result is from second conversion in command."]
    Result2 = 0x01,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult2 = 0x02,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult3 = 0x03,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult4 = 0x04,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult5 = 0x05,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult6 = 0x06,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult7 = 0x07,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult8 = 0x08,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CorrespondingResult9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    Result16 = 0x0f,
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
    MultipleRefNotSupported = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MultipleRefSupported = 0x01,
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
    #[doc = "N/A."]
    NoFifoImplemented = 0x0,
    #[doc = "This design supports one result FIFO."]
    Cnt1 = 0x01,
    #[doc = "This design supports two result FIFOs."]
    Cnt2 = 0x02,
    #[doc = "This design supports three result FIFOs."]
    Cnt3 = 0x03,
    #[doc = "This design supports four result FIFOs."]
    Cnt4 = 0x04,
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
    SingleConvertor = 0x0,
    #[doc = "This design supports two simultaneous single ended conversions."]
    DualConvertor = 0x01,
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
pub enum Pwrsel {
    #[doc = "Low power."]
    Lowest = 0x0,
    #[doc = "High power."]
    Highest = 0x01,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x01) }
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
    BelowThreshold = 0x0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    AboveThreshold = 0x01,
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
pub enum Refsel {
    #[doc = "(Default) Option 1 setting."]
    Option1 = 0x0,
    #[doc = "Option 2 setting."]
    Option2 = 0x01,
    #[doc = "Option 3 setting."]
    Option3 = 0x02,
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
    #[doc = "Up to 12-bit single ended resolution supported (and 13-bit differential resolution if VERID\\[DIFFEN\\] = 1b)."]
    Max13Bit = 0x0,
    #[doc = "Up to 16-bit single ended resolution supported (and 16-bit differential resolution if VERID\\[DIFFEN\\] = 1b)."]
    Max16Bit = 0x01,
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
    ReleasedFromReset = 0x0,
    #[doc = "ADC logic is reset."]
    HeldInReset = 0x01,
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
    NoAction = 0x0,
    #[doc = "FIFO 0 is reset."]
    TriggerReset = 0x01,
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
pub enum Swt0 {
    #[doc = "No trigger 0 event generated."]
    NoTrigger = 0x0,
    #[doc = "Trigger 0 event generated."]
    InitiateTrigger0 = 0x01,
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
    NoTrigger = 0x0,
    #[doc = "Trigger 1 event generated."]
    InitiateTrigger1 = 0x01,
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
pub enum Swt2 {
    #[doc = "No trigger 2 event generated."]
    NoTrigger = 0x0,
    #[doc = "Trigger 2 event generated."]
    InitiateTrigger2 = 0x01,
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
    NoTrigger = 0x0,
    #[doc = "Trigger 3 event generated."]
    InitiateTrigger3 = 0x01,
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
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    NotValid = 0x0,
    #[doc = "CMD1 is executed."]
    ExecuteCmd1 = 0x01,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd2 = 0x02,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd3 = 0x03,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd4 = 0x04,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd5 = 0x05,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd6 = 0x06,
    #[doc = "CMD7 is executed."]
    ExecuteCmd7 = 0x07,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum TcompFlag {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    NoTrigger = 0x0,
    #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    Bit0MeansTrigger0Completed = 0x01,
    #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    Bit1MeansTrigger1Completed = 0x02,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted3 = 0x03,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted4 = 0x04,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted5 = 0x05,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted6 = 0x06,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted7 = 0x07,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted8 = 0x08,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    AllBitsSetIndicateAllTriggersCompleted = 0x0f,
}
impl TcompFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompFlag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompFlag {
    #[inline(always)]
    fn from(val: u8) -> TcompFlag {
        TcompFlag::from_bits(val)
    }
}
impl From<TcompFlag> for u8 {
    #[inline(always)]
    fn from(val: TcompFlag) -> u8 {
        TcompFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompIe {
    #[doc = "Trigger completion interrupts are disabled."]
    Disabled = 0x0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    Trigger0CompleteEnabled = 0x01,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    Trigger1CompleteEnabled = 0x02,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled3 = 0x03,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled4 = 0x04,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled5 = 0x05,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled6 = 0x06,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled7 = 0x07,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled8 = 0x08,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    AllTriggerCompletesEnabled = 0x0f,
}
impl TcompIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompIe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompIe {
    #[inline(always)]
    fn from(val: u8) -> TcompIe {
        TcompIe::from_bits(val)
    }
}
impl From<TcompIe> for u8 {
    #[inline(always)]
    fn from(val: TcompIe) -> u8 {
        TcompIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompInt {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    FlagClear = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    CompletionDetected = 0x01,
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
pub enum TexcInt {
    #[doc = "No trigger exceptions have occurred."]
    NoException = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    ExceptionDetected = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcNum {
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    NoExceptions = 0x0,
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    Bit0MeansTrigger0Interrupted = 0x01,
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    Bit1MeansTrigger1Interrupted = 0x02,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted3 = 0x03,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted4 = 0x04,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted5 = 0x05,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted6 = 0x06,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted7 = 0x07,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted8 = 0x08,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SetBitsIndicateTriggerXInterrupted9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    AllBitsSetIndicateAllTriggersInterrupted = 0x0f,
}
impl TexcNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcNum {
    #[inline(always)]
    fn from(val: u8) -> TexcNum {
        TexcNum::from_bits(val)
    }
}
impl From<TexcNum> for u8 {
    #[inline(always)]
    fn from(val: TexcNum) -> u8 {
        TexcNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpri {
    #[doc = "Set to highest priority, Level 1."]
    HighestPriority = 0x0,
    #[doc = "Set to corresponding priority level."]
    CorrespondingLowerPriority1 = 0x01,
    #[doc = "Set to corresponding priority level."]
    CorrespondingLowerPriority2 = 0x02,
    #[doc = "Set to lowest priority, Level 4."]
    LowestPriority = 0x03,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x03) }
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
    AbortCurrentOnPriority = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    FinishCurrentOnPriority = 0x01,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    FinishSequenceOnPriority = 0x02,
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
pub enum Trgact {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    Trig0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    Trig1 = 0x01,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    Trig2 = 0x02,
    #[doc = "Command (sequence) associated with Trigger 3 currently being executed."]
    Trig3 = 0x03,
}
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x03) }
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
    Trigger0 = 0x0,
    #[doc = "Trigger source 1 initiated this conversion."]
    Trigger1 = 0x01,
    #[doc = "Trigger source 2 initiated this conversion."]
    Trigger2 = 0x02,
    #[doc = "Trigger source 3 initiated this conversion."]
    Trigger3 = 0x03,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Vr1rngi {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    Ref1FixedVoltageRange = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    Ref1SelectableVoltageRange = 0x01,
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
