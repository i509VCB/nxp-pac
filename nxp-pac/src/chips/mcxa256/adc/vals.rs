#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcActive {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    NOT_ACTIVE = 0x0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    BUSY = 0x01,
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
pub enum Avgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avgs {
    #[inline(always)]
    fn from(val: u8) -> Avgs {
        Avgs::from_bits(val)
    }
}
impl From<Avgs> for u8 {
    #[inline(always)]
    fn from(val: Avgs) -> u8 {
        Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalAvgs {
    #[doc = "Single conversion."]
    NO_AVERAGE = 0x0,
    #[doc = "2 conversions averaged."]
    AVERAGE_2 = 0x01,
    #[doc = "4 conversions averaged."]
    AVERAGE_4 = 0x02,
    #[doc = "8 conversions averaged."]
    AVERAGE_8 = 0x03,
    #[doc = "16 conversions averaged."]
    AVERAGE_16 = 0x04,
    #[doc = "32 conversions averaged."]
    AVERAGE_32 = 0x05,
    #[doc = "64 conversions averaged."]
    AVERAGE_64 = 0x06,
    #[doc = "128 conversions averaged."]
    AVERAGE_128 = 0x07,
    #[doc = "256 conversions averaged."]
    AVERAGE_256 = 0x08,
    #[doc = "512 conversions averaged."]
    AVERAGE_512 = 0x09,
    #[doc = "1024 conversions averaged."]
    AVERAGE_1024 = 0x0a,
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
    NOT_SET = 0x0,
    #[doc = "The ADC is calibrated."]
    HARDWARE_CAL_STEP_COMPLETED = 0x01,
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
    #[doc = "No request for hardware calibration has been made"]
    NO_CALIBRATION_REQUEST = 0x0,
    #[doc = "A request for hardware calibration has been made"]
    CALIBRATION_REQUEST_PENDING = 0x01,
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
    #[doc = "No request for high speed mode trim has been made"]
    NO_ACTIVE_HS_TRIM_REQUEST = 0x0,
    #[doc = "Request for high speed mode trim has been made"]
    HS_TRIM_REQUEST_PENDING = 0x01,
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
    #[doc = "No request for offset calibration has been made"]
    NO_ACTIVE_OFFSET_CALIBRATION_REQUEST = 0x0,
    #[doc = "Request for offset calibration function"]
    OFFSET_CALIBRATION_REQUEST_PENDING = 0x01,
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
    CAL_FUNCTION_NOT_AVAILABLE = 0x0,
    #[doc = "Calibration Implemented."]
    CAL_FUNCTION_AVAILABLE = 0x01,
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
    NO_COMMAND_ACTIVE = 0x0,
    #[doc = "Command 1 currently being executed."]
    COMMAND_1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    COMMAND_2 = 0x02,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_3 = 0x03,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_4 = 0x04,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_5 = 0x05,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_6 = 0x06,
    #[doc = "Associated command number is currently being executed."]
    COMMAND_X_7 = 0x07,
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
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    NOT_VALID = 0x0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    CMD1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CORRESPONDING_CMD_6 = 0x06,
    #[doc = "CMD7 buffer used as control settings for this conversion."]
    CMD7 = 0x07,
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
pub enum Cmpen {
    #[doc = "Compare disabled."]
    DISABLED_ALWAYS_STORE_RESULT = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    COMPARE_RESULT_STORE_IF_TRUE = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE = 0x03,
}
impl Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmpen {
        Cmpen::from_bits(val)
    }
}
impl From<Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmpen) -> u8 {
        Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csw {
    #[doc = "Channel scaling not supported."]
    CSCALE_NOT_SUPPORTED = 0x0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    BIT_WIDTH_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    BIT_WIDTH_6 = 0x06,
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
pub enum Ctype {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A_SIDE_CHANNEL = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ctype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctype {
    #[inline(always)]
    fn from(val: u8) -> Ctype {
        Ctype::from_bits(val)
    }
}
impl From<Ctype> for u8 {
    #[inline(always)]
    fn from(val: Ctype) -> u8 {
        Ctype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diffen {
    #[doc = "Differential operation not supported."]
    DIFFERENTIAL_NOT_SUPPORTED = 0x0,
    #[doc = "Differential operation supported."]
    DIFFERENTIAL_SUPPORTED = 0x01,
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
    ENABLED = 0x0,
    #[doc = "ADC is disabled in low power mode."]
    DISABLED = 0x01,
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
    pub const ENTRIES_2: Self = Self(0x01);
    #[doc = "Result FIFO depth = 4 datawords."]
    pub const ENTRIES_4: Self = Self(0x04);
    #[doc = "Result FIFO depth = 8 datawords."]
    pub const ENTRIES_8: Self = Self(0x08);
    #[doc = "Result FIFO depth = 16 datawords."]
    pub const ENTRIES_16: Self = Self(0x10);
    #[doc = "Result FIFO depth = 32 datawords."]
    pub const ENTRIES_32: Self = Self(0x20);
    #[doc = "Result FIFO depth = 64 datawords."]
    pub const ENTRIES_64: Self = Self(0x40);
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
            0x01 => f.write_str("ENTRIES_2"),
            0x04 => f.write_str("ENTRIES_4"),
            0x08 => f.write_str("ENTRIES_8"),
            0x10 => f.write_str("ENTRIES_16"),
            0x20 => f.write_str("ENTRIES_32"),
            0x40 => f.write_str("ENTRIES_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifosize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "ENTRIES_2"),
            0x04 => defmt::write!(f, "ENTRIES_4"),
            0x08 => defmt::write!(f, "ENTRIES_8"),
            0x10 => defmt::write!(f, "ENTRIES_16"),
            0x20 => defmt::write!(f, "ENTRIES_32"),
            0x40 => defmt::write!(f, "ENTRIES_64"),
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
    NO_OVERFLOW = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    OVERFLOW_DETECTED = 0x01,
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
    GAIN_CAL_NOT_VALID = 0x0,
    #[doc = "The GAIN_CAL value is valid. GAIN_CAL should be used by software to derive GCRa\\[GCALR\\]."]
    HARDWARE_CAL_ROUTINE_COMPLETED = 0x01,
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
    ENABLED = 0x0,
    #[doc = "High priority trigger exceptions are disabled."]
    DISABLED = 0x01,
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
    #[doc = "No extra cycle added"]
    HSEXTRA_0 = 0x0,
    #[doc = "Extra cycle added"]
    HSEXTRA_1 = 0x01,
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
    INTERNAL_CLK_NOT_AVAILABLE = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    INTERNAL_CLK_AVAILABLE = 0x01,
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
pub enum Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    CMD_EXEC_1X = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    CMD_EXEC_2X = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    CMD_EXEC_3X = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    CMD_EXECUTES_CORRESPONDING_TIMES_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CMD_EXEC_15X = 0x0f,
}
impl Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loop {
    #[inline(always)]
    fn from(val: u8) -> Loop {
        Loop::from_bits(val)
    }
}
impl From<Loop> for u8 {
    #[inline(always)]
    fn from(val: Loop) -> u8 {
        Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopcnt {
    #[doc = "Result is from initial conversion in command."]
    RESULT_1 = 0x0,
    #[doc = "Result is from second conversion in command."]
    RESULT_2 = 0x01,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_2 = 0x02,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_3 = 0x03,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_4 = 0x04,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_5 = 0x05,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_6 = 0x06,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_7 = 0x07,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_8 = 0x08,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    CORRESPONDING_RESULT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    RESULT_16 = 0x0f,
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
pub enum Mode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion."]
    DATA_12_BITS = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion."]
    DATA_16_BITS = 0x01,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mvi {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MULTIPLE_REF_NOT_SUPPORTED = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MULTIPLE_REF_SUPPORTED = 0x01,
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
pub enum Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NO_NEXT_CMD_TERMINATE_ON_FINISH = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    DO_CMD1_NEXT = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    DO_CORRESPONDING_CMD_NEXT_6 = 0x06,
    #[doc = "Select CMD7 command buffer register as next command."]
    DO_CMD7_NEXT = 0x07,
}
impl Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Next {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Next {
    #[inline(always)]
    fn from(val: u8) -> Next {
        Next::from_bits(val)
    }
}
impl From<Next> for u8 {
    #[inline(always)]
    fn from(val: Next) -> u8 {
        Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumFifo {
    #[doc = "N/A"]
    NO_FIFO_IMPLEMENTED = 0x0,
    #[doc = "This design supports one result FIFO."]
    CNT_1 = 0x01,
    #[doc = "This design supports two result FIFOs."]
    CNT_2 = 0x02,
    #[doc = "This design supports three result FIFOs."]
    CNT_3 = 0x03,
    #[doc = "This design supports four result FIFOs."]
    CNT_4 = 0x04,
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
    SINGLE_CONVERTOR = 0x0,
    #[doc = "This design supports two simultaneous single ended conversions."]
    DUAL_CONVERTOR = 0x01,
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
    #[doc = "Low power"]
    LOWEST = 0x0,
    #[doc = "High power"]
    HIGHEST = 0x01,
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
    BELOW_THRESHOLD = 0x0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    ABOVE_THRESHOLD = 0x01,
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
    OPTION_1 = 0x0,
    #[doc = "Option 2 setting."]
    OPTION_2 = 0x01,
    #[doc = "Option 3 setting."]
    OPTION_3 = 0x02,
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
    MAX_13_BIT = 0x0,
    #[doc = "Up to 16-bit single ended resolution supported (and 16-bit differential resolution if VERID\\[DIFFEN\\] = 1b)."]
    MAX_16_BIT = 0x01,
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
    RELEASED_FROM_RESET = 0x0,
    #[doc = "ADC logic is reset."]
    HELD_IN_RESET = 0x01,
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
    NO_ACTION = 0x0,
    #[doc = "FIFO 0 is reset."]
    TRIGGER_RESET = 0x01,
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
pub enum Sts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    SAMPLE_3P5 = 0x0,
    #[doc = "3.5 + 21 ADCK cycles; 5.5 ADCK cycles total sample time."]
    SAMPLE_5P5 = 0x01,
    #[doc = "3.5 + 22 ADCK cycles; 7.5 ADCK cycles total sample time."]
    SAMPLE_7P5 = 0x02,
    #[doc = "3.5 + 23 ADCK cycles; 11.5 ADCK cycles total sample time."]
    SAMPLE_11P5 = 0x03,
    #[doc = "3.5 + 24 ADCK cycles; 19.5 ADCK cycles total sample time."]
    SAMPLE_19P5 = 0x04,
    #[doc = "3.5 + 25 ADCK cycles; 35.5 ADCK cycles total sample time."]
    SAMPLE_35P5 = 0x05,
    #[doc = "3.5 + 26 ADCK cycles; 67.5 ADCK cycles total sample time."]
    SAMPLE_67P5 = 0x06,
    #[doc = "3.5 + 27 ADCK cycles; 131.5 ADCK cycles total sample time."]
    SAMPLE_131P5 = 0x07,
}
impl Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts {
    #[inline(always)]
    fn from(val: u8) -> Sts {
        Sts::from_bits(val)
    }
}
impl From<Sts> for u8 {
    #[inline(always)]
    fn from(val: Sts) -> u8 {
        Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    NOT_VALID = 0x0,
    #[doc = "CMD1 is executed"]
    EXECUTE_CMD1 = 0x01,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_2 = 0x02,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_3 = 0x03,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_4 = 0x04,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_5 = 0x05,
    #[doc = "Corresponding CMD is executed"]
    EXECUTE_CORRESPONDING_CMD_6 = 0x06,
    #[doc = "CMD7 is executed"]
    EXECUTE_CMD7 = 0x07,
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
    NO_TRIGGER = 0x0,
    #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    BIT0_MEANS_TRIGGER_0_COMPLETED = 0x01,
    #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    BIT1_MEANS_TRIGGER_1_COMPLETED = 0x02,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3 = 0x03,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4 = 0x04,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5 = 0x05,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6 = 0x06,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7 = 0x07,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8 = 0x08,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED = 0x0f,
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
    DISABLED = 0x0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    TRIGGER_0_COMPLETE_ENABLED = 0x01,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    TRIGGER_1_COMPLETE_ENABLED = 0x02,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_3 = 0x03,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_4 = 0x04,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_5 = 0x05,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_6 = 0x06,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_7 = 0x07,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_8 = 0x08,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TRIGGER_X_COMPLETE_ENABLED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    ALL_TRIGGER_COMPLETES_ENABLED = 0x0f,
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
    FLAG_CLEAR = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    COMPLETION_DETECTED = 0x01,
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
    NO_EXCEPTION = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    EXCEPTION_DETECTED = 0x01,
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
    NO_EXCEPTIONS = 0x0,
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    BIT0_MEANS_TRIGGER_0_INTERRUPTED = 0x01,
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    BIT1_MEANS_TRIGGER_1_INTERRUPTED = 0x02,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3 = 0x03,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4 = 0x04,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5 = 0x05,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6 = 0x06,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7 = 0x07,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8 = 0x08,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED = 0x0f,
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
    #[doc = "Set to highest priority, Level 1"]
    HIGHEST_PRIORITY = 0x0,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_1 = 0x01,
    #[doc = "Set to corresponding priority level"]
    CORRESPONDING_LOWER_PRIORITY_2 = 0x02,
    #[doc = "Set to lowest priority, Level 4"]
    LOWEST_PRIORITY = 0x03,
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
    ABORT_CURRENT_ON_PRIORITY = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    FINISH_CURRENT_ON_PRIORITY = 0x01,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    FINISH_SEQUENCE_ON_PRIORITY = 0x02,
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
    TRIG_0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRIG_1 = 0x01,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    TRIG_2 = 0x02,
    #[doc = "Command (sequence) associated with Trigger 3 currently being executed."]
    TRIG_3 = 0x03,
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
    TRIGGER_0 = 0x0,
    #[doc = "Trigger source 1 initiated this conversion."]
    TRIGGER_1 = 0x01,
    #[doc = "Trigger source 2 initiated this conversion."]
    TRIGGER_2 = 0x02,
    #[doc = "Trigger source 3 initiated this conversion."]
    TRIGGER_3 = 0x03,
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
    REF1_FIXED_VOLTAGE_RANGE = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    REF1_SELECTABLE_VOLTAGE_RANGE = 0x01,
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
