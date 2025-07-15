#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "The sequence is not running."]
    SEQ_NOT_RUNNING = 0x0,
    #[doc = "The sequence is running."]
    SEQ_RUNNING = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bc12 {
    #[doc = "Compatible with BC1.1"]
    BC11 = 0x0,
    #[doc = "Compatible with BC1.2 (default)"]
    BC12 = 0x01,
}
impl Bc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bc12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bc12 {
    #[inline(always)]
    fn from(val: u8) -> Bc12 {
        Bc12::from_bits(val)
    }
}
impl From<Bc12> for u8 {
    #[inline(always)]
    fn from(val: Bc12) -> u8 {
        Bc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CheckDm {
    _RESERVED_0 = 0x0,
    #[doc = "1 ms - 15 ms"]
    MS_1 = 0x01,
    #[doc = "1 ms - 15 ms"]
    MS_2 = 0x02,
    #[doc = "1 ms - 15 ms"]
    MS_3 = 0x03,
    #[doc = "1 ms - 15 ms"]
    MS_4 = 0x04,
    #[doc = "1 ms - 15 ms"]
    MS_5 = 0x05,
    #[doc = "1 ms - 15 ms"]
    MS_6 = 0x06,
    #[doc = "1 ms - 15 ms"]
    MS_7 = 0x07,
    #[doc = "1 ms - 15 ms"]
    MS_8 = 0x08,
    #[doc = "1 ms - 15 ms"]
    MS_9 = 0x09,
    #[doc = "1 ms - 15 ms"]
    MS_10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CheckDm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CheckDm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CheckDm {
    #[inline(always)]
    fn from(val: u8) -> CheckDm {
        CheckDm::from_bits(val)
    }
}
impl From<CheckDm> for u8 {
    #[inline(always)]
    fn from(val: CheckDm) -> u8 {
        CheckDm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockUnit {
    #[doc = "kHz Speed (between 4 kHz and 1023 kHz)"]
    KHZ_CLK = 0x0,
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    MHZ_CLK = 0x01,
}
impl ClockUnit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockUnit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockUnit {
    #[inline(always)]
    fn from(val: u8) -> ClockUnit {
        ClockUnit::from_bits(val)
    }
}
impl From<ClockUnit> for u8 {
    #[inline(always)]
    fn from(val: ClockUnit) -> u8 {
        ClockUnit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iack {
    #[doc = "Do not clear the interrupt."]
    INT_NOCLEAR = 0x0,
    #[doc = "Clear the IF field (interrupt flag)."]
    INT_CLEAR = 0x01,
}
impl Iack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iack {
    #[inline(always)]
    fn from(val: u8) -> Iack {
        Iack::from_bits(val)
    }
}
impl From<Iack> for u8 {
    #[inline(always)]
    fn from(val: Iack) -> u8 {
        Iack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie {
    #[doc = "Disable interrupts to the system."]
    DIS_INT = 0x0,
    #[doc = "Enable interrupts to the system."]
    EN_INT = 0x01,
}
impl Ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie {
    #[inline(always)]
    fn from(val: u8) -> Ie {
        Ie::from_bits(val)
    }
}
impl From<Ie> for u8 {
    #[inline(always)]
    fn from(val: Ie) -> u8 {
        Ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum If {
    #[doc = "No interrupt is pending."]
    INT_PEND = 0x0,
    #[doc = "An interrupt is pending."]
    INT_NOPEND = 0x01,
}
impl If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for If {
    #[inline(always)]
    fn from(val: u8) -> If {
        If::from_bits(val)
    }
}
impl From<If> for u8 {
    #[inline(always)]
    fn from(val: If) -> u8 {
        If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "No overrides. Field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    NO_OVERRIDE = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    PRI_DET_OVERRIDE = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enables VDM_SRC voltage source only."]
    CDP_ADVERT = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqRes {
    #[doc = "No results to report."]
    NO_RESULT = 0x0,
    #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    CONN_SDP = 0x01,
    #[doc = "Attached to a charging port. The exact meaning depends on the STATUS\\[SEQ_STAT\\] field (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)"]
    CONN_CP = 0x02,
    #[doc = "Attached to a DCP."]
    CONN_DCP = 0x03,
}
impl SeqRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqRes {
    #[inline(always)]
    fn from(val: u8) -> SeqRes {
        SeqRes::from_bits(val)
    }
}
impl From<SeqRes> for u8 {
    #[inline(always)]
    fn from(val: SeqRes) -> u8 {
        SeqRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqStat {
    #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    NO_DATA_PIN_CONN = 0x0,
    #[doc = "Data pin contact detection is complete."]
    DATA_PIN_CONN = 0x01,
    #[doc = "Charging port detection is complete."]
    CP_DET_DONE = 0x02,
    #[doc = "Charger type detection is complete."]
    CT_DET_DONE = 0x03,
}
impl SeqStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqStat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqStat {
    #[inline(always)]
    fn from(val: u8) -> SeqStat {
        SeqStat::from_bits(val)
    }
}
impl From<SeqStat> for u8 {
    #[inline(always)]
    fn from(val: SeqStat) -> u8 {
        SeqStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sr {
    #[doc = "Do not perform a software reset."]
    NO_RESET = 0x0,
    #[doc = "Perform a software reset."]
    SW_RESET = 0x01,
}
impl Sr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sr {
    #[inline(always)]
    fn from(val: u8) -> Sr {
        Sr::from_bits(val)
    }
}
impl From<Sr> for u8 {
    #[inline(always)]
    fn from(val: Sr) -> u8 {
        Sr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TdcdDbnc(u16);
impl TdcdDbnc {
    #[doc = "1 ms - 1023 ms"]
    pub const MS_1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_10: Self = Self(0x0a);
}
impl TdcdDbnc {
    pub const fn from_bits(val: u16) -> TdcdDbnc {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TdcdDbnc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("MS_1"),
            0x02 => f.write_str("MS_2"),
            0x03 => f.write_str("MS_3"),
            0x04 => f.write_str("MS_4"),
            0x05 => f.write_str("MS_5"),
            0x06 => f.write_str("MS_6"),
            0x07 => f.write_str("MS_7"),
            0x08 => f.write_str("MS_8"),
            0x09 => f.write_str("MS_9"),
            0x0a => f.write_str("MS_10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TdcdDbnc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MS_1"),
            0x02 => defmt::write!(f, "MS_2"),
            0x03 => defmt::write!(f, "MS_3"),
            0x04 => defmt::write!(f, "MS_4"),
            0x05 => defmt::write!(f, "MS_5"),
            0x06 => defmt::write!(f, "MS_6"),
            0x07 => defmt::write!(f, "MS_7"),
            0x08 => defmt::write!(f, "MS_8"),
            0x09 => defmt::write!(f, "MS_9"),
            0x0a => defmt::write!(f, "MS_10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TdcdDbnc {
    #[inline(always)]
    fn from(val: u16) -> TdcdDbnc {
        TdcdDbnc::from_bits(val)
    }
}
impl From<TdcdDbnc> for u16 {
    #[inline(always)]
    fn from(val: TdcdDbnc) -> u16 {
        TdcdDbnc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TseqInit(u16);
impl TseqInit {
    #[doc = "0 ms - 1023 ms"]
    pub const MS_0: Self = Self(0x0);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_1: Self = Self(0x01);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_2: Self = Self(0x02);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_3: Self = Self(0x03);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_4: Self = Self(0x04);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_5: Self = Self(0x05);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_6: Self = Self(0x06);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_7: Self = Self(0x07);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_8: Self = Self(0x08);
    #[doc = "0 ms - 1023 ms"]
    pub const MS_9: Self = Self(0x09);
}
impl TseqInit {
    pub const fn from_bits(val: u16) -> TseqInit {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TseqInit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MS_0"),
            0x01 => f.write_str("MS_1"),
            0x02 => f.write_str("MS_2"),
            0x03 => f.write_str("MS_3"),
            0x04 => f.write_str("MS_4"),
            0x05 => f.write_str("MS_5"),
            0x06 => f.write_str("MS_6"),
            0x07 => f.write_str("MS_7"),
            0x08 => f.write_str("MS_8"),
            0x09 => f.write_str("MS_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TseqInit {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MS_0"),
            0x01 => defmt::write!(f, "MS_1"),
            0x02 => defmt::write!(f, "MS_2"),
            0x03 => defmt::write!(f, "MS_3"),
            0x04 => defmt::write!(f, "MS_4"),
            0x05 => defmt::write!(f, "MS_5"),
            0x06 => defmt::write!(f, "MS_6"),
            0x07 => defmt::write!(f, "MS_7"),
            0x08 => defmt::write!(f, "MS_8"),
            0x09 => defmt::write!(f, "MS_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TseqInit {
    #[inline(always)]
    fn from(val: u16) -> TseqInit {
        TseqInit::from_bits(val)
    }
}
impl From<TseqInit> for u16 {
    #[inline(always)]
    fn from(val: TseqInit) -> u16 {
        TseqInit::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdmsrcOn(u16);
impl TvdmsrcOn {
    #[doc = "0 ms - 40 ms"]
    pub const MS_0: Self = Self(0x0);
    #[doc = "0 ms - 40 ms"]
    pub const MS_1: Self = Self(0x01);
    #[doc = "0 ms - 40 ms"]
    pub const MS_2: Self = Self(0x02);
    #[doc = "0 ms - 40 ms"]
    pub const MS_3: Self = Self(0x03);
    #[doc = "0 ms - 40 ms"]
    pub const MS_4: Self = Self(0x04);
    #[doc = "0 ms - 40 ms"]
    pub const MS_5: Self = Self(0x05);
    #[doc = "0 ms - 40 ms"]
    pub const MS_6: Self = Self(0x06);
    #[doc = "0 ms - 40 ms"]
    pub const MS_7: Self = Self(0x07);
    #[doc = "0 ms - 40 ms"]
    pub const MS_8: Self = Self(0x08);
    #[doc = "0 ms - 40 ms"]
    pub const MS_9: Self = Self(0x09);
}
impl TvdmsrcOn {
    pub const fn from_bits(val: u16) -> TvdmsrcOn {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TvdmsrcOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MS_0"),
            0x01 => f.write_str("MS_1"),
            0x02 => f.write_str("MS_2"),
            0x03 => f.write_str("MS_3"),
            0x04 => f.write_str("MS_4"),
            0x05 => f.write_str("MS_5"),
            0x06 => f.write_str("MS_6"),
            0x07 => f.write_str("MS_7"),
            0x08 => f.write_str("MS_8"),
            0x09 => f.write_str("MS_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdmsrcOn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MS_0"),
            0x01 => defmt::write!(f, "MS_1"),
            0x02 => defmt::write!(f, "MS_2"),
            0x03 => defmt::write!(f, "MS_3"),
            0x04 => defmt::write!(f, "MS_4"),
            0x05 => defmt::write!(f, "MS_5"),
            0x06 => defmt::write!(f, "MS_6"),
            0x07 => defmt::write!(f, "MS_7"),
            0x08 => defmt::write!(f, "MS_8"),
            0x09 => defmt::write!(f, "MS_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdmsrcOn {
    #[inline(always)]
    fn from(val: u16) -> TvdmsrcOn {
        TvdmsrcOn::from_bits(val)
    }
}
impl From<TvdmsrcOn> for u16 {
    #[inline(always)]
    fn from(val: TvdmsrcOn) -> u16 {
        TvdmsrcOn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdpsrcCon(u16);
impl TvdpsrcCon {
    #[doc = "1 ms - 1023 ms"]
    pub const MS_1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_10: Self = Self(0x0a);
}
impl TvdpsrcCon {
    pub const fn from_bits(val: u16) -> TvdpsrcCon {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TvdpsrcCon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("MS_1"),
            0x02 => f.write_str("MS_2"),
            0x03 => f.write_str("MS_3"),
            0x04 => f.write_str("MS_4"),
            0x05 => f.write_str("MS_5"),
            0x06 => f.write_str("MS_6"),
            0x07 => f.write_str("MS_7"),
            0x08 => f.write_str("MS_8"),
            0x09 => f.write_str("MS_9"),
            0x0a => f.write_str("MS_10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdpsrcCon {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MS_1"),
            0x02 => defmt::write!(f, "MS_2"),
            0x03 => defmt::write!(f, "MS_3"),
            0x04 => defmt::write!(f, "MS_4"),
            0x05 => defmt::write!(f, "MS_5"),
            0x06 => defmt::write!(f, "MS_6"),
            0x07 => defmt::write!(f, "MS_7"),
            0x08 => defmt::write!(f, "MS_8"),
            0x09 => defmt::write!(f, "MS_9"),
            0x0a => defmt::write!(f, "MS_10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdpsrcCon {
    #[inline(always)]
    fn from(val: u16) -> TvdpsrcCon {
        TvdpsrcCon::from_bits(val)
    }
}
impl From<TvdpsrcCon> for u16 {
    #[inline(always)]
    fn from(val: TvdpsrcCon) -> u16 {
        TvdpsrcCon::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TvdpsrcOn(u16);
impl TvdpsrcOn {
    #[doc = "1 ms - 1023 ms"]
    pub const MS_1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_10: Self = Self(0x0a);
}
impl TvdpsrcOn {
    pub const fn from_bits(val: u16) -> TvdpsrcOn {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TvdpsrcOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("MS_1"),
            0x02 => f.write_str("MS_2"),
            0x03 => f.write_str("MS_3"),
            0x04 => f.write_str("MS_4"),
            0x05 => f.write_str("MS_5"),
            0x06 => f.write_str("MS_6"),
            0x07 => f.write_str("MS_7"),
            0x08 => f.write_str("MS_8"),
            0x09 => f.write_str("MS_9"),
            0x0a => f.write_str("MS_10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TvdpsrcOn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MS_1"),
            0x02 => defmt::write!(f, "MS_2"),
            0x03 => defmt::write!(f, "MS_3"),
            0x04 => defmt::write!(f, "MS_4"),
            0x05 => defmt::write!(f, "MS_5"),
            0x06 => defmt::write!(f, "MS_6"),
            0x07 => defmt::write!(f, "MS_7"),
            0x08 => defmt::write!(f, "MS_8"),
            0x09 => defmt::write!(f, "MS_9"),
            0x0a => defmt::write!(f, "MS_10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TvdpsrcOn {
    #[inline(always)]
    fn from(val: u16) -> TvdpsrcOn {
        TvdpsrcOn::from_bits(val)
    }
}
impl From<TvdpsrcOn> for u16 {
    #[inline(always)]
    fn from(val: TvdpsrcOn) -> u16 {
        TvdpsrcOn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TwaitAfterPrd(u16);
impl TwaitAfterPrd {
    #[doc = "1 ms - 1023 ms"]
    pub const MS_1: Self = Self(0x01);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_2: Self = Self(0x02);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_3: Self = Self(0x03);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_4: Self = Self(0x04);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_5: Self = Self(0x05);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_6: Self = Self(0x06);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_7: Self = Self(0x07);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_8: Self = Self(0x08);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_9: Self = Self(0x09);
    #[doc = "1 ms - 1023 ms"]
    pub const MS_10: Self = Self(0x0a);
}
impl TwaitAfterPrd {
    pub const fn from_bits(val: u16) -> TwaitAfterPrd {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for TwaitAfterPrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("MS_1"),
            0x02 => f.write_str("MS_2"),
            0x03 => f.write_str("MS_3"),
            0x04 => f.write_str("MS_4"),
            0x05 => f.write_str("MS_5"),
            0x06 => f.write_str("MS_6"),
            0x07 => f.write_str("MS_7"),
            0x08 => f.write_str("MS_8"),
            0x09 => f.write_str("MS_9"),
            0x0a => f.write_str("MS_10"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TwaitAfterPrd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MS_1"),
            0x02 => defmt::write!(f, "MS_2"),
            0x03 => defmt::write!(f, "MS_3"),
            0x04 => defmt::write!(f, "MS_4"),
            0x05 => defmt::write!(f, "MS_5"),
            0x06 => defmt::write!(f, "MS_6"),
            0x07 => defmt::write!(f, "MS_7"),
            0x08 => defmt::write!(f, "MS_8"),
            0x09 => defmt::write!(f, "MS_9"),
            0x0a => defmt::write!(f, "MS_10"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for TwaitAfterPrd {
    #[inline(always)]
    fn from(val: u16) -> TwaitAfterPrd {
        TwaitAfterPrd::from_bits(val)
    }
}
impl From<TwaitAfterPrd> for u16 {
    #[inline(always)]
    fn from(val: TwaitAfterPrd) -> u16 {
        TwaitAfterPrd::to_bits(val)
    }
}
