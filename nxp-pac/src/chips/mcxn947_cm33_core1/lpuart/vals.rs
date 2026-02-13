#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brk13 {
    #[doc = "9 to 13 bit times"]
    SHORT = 0x0,
    #[doc = "12 to 15 bit times"]
    LONG = 0x01,
}
impl Brk13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brk13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brk13 {
    #[inline(always)]
    fn from(val: u8) -> Brk13 {
        Brk13::from_bits(val)
    }
}
impl From<Brk13> for u8 {
    #[inline(always)]
    fn from(val: Brk13) -> u8 {
        Brk13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg {
    #[doc = "Becomes 1 after timeout characters are received"]
    CNT_CHAR = 0x0,
    #[doc = "Becomes 1 when idle for timeout bit clocks"]
    CNT_IDLE = 0x01,
    #[doc = "Becomes 1 when idle for timeout bit clocks following the next character"]
    CNT_BUSY_IDLE = 0x02,
    #[doc = "Becomes 1 when idle for at least timeout bit clocks, but a new character is detected before the extended idle timeout is reached"]
    CNT_CHAR_IDLE = 0x03,
}
impl Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg {
    #[inline(always)]
    fn from(val: u8) -> Cfg {
        Cfg::from_bits(val)
    }
}
impl From<Cfg> for u8 {
    #[inline(always)]
    fn from(val: Cfg) -> u8 {
        Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "Enable"]
    ENABLED = 0x0,
    #[doc = "Disable"]
    DISABLED = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtr {
    #[doc = "Logic one"]
    LOGIC_ONE = 0x0,
    #[doc = "Logic zero"]
    LOGIC_ZERO = 0x01,
}
impl Dtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtr {
    #[inline(always)]
    fn from(val: u8) -> Dtr {
        Dtr::from_bits(val)
    }
}
impl From<Dtr> for u8 {
    #[inline(always)]
    fn from(val: Dtr) -> u8 {
        Dtr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set"]
    pub const STANDARD: Self = Self(0x01);
    #[doc = "Standard feature set with MODEM and IrDA support"]
    pub const MODEM: Self = Self(0x03);
    #[doc = "Enhanced feature set with full MODEM, IrDA, and enhanced idle detection"]
    pub const MODEM_IDLE: Self = Self(0x07);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("STANDARD"),
            0x03 => f.write_str("MODEM"),
            0x07 => f.write_str("MODEM_IDLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "STANDARD"),
            0x03 => defmt::write!(f, "MODEM"),
            0x07 => defmt::write!(f, "MODEM_IDLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idlecfg {
    #[doc = "1"]
    IDLE_1 = 0x0,
    #[doc = "2"]
    IDLE_2 = 0x01,
    #[doc = "4"]
    IDLE_4 = 0x02,
    #[doc = "8"]
    IDLE_8 = 0x03,
    #[doc = "16"]
    IDLE_16 = 0x04,
    #[doc = "32"]
    IDLE_32 = 0x05,
    #[doc = "64"]
    IDLE_64 = 0x06,
    #[doc = "128"]
    IDLE_128 = 0x07,
}
impl Idlecfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idlecfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idlecfg {
    #[inline(always)]
    fn from(val: u8) -> Idlecfg {
        Idlecfg::from_bits(val)
    }
}
impl From<Idlecfg> for u8 {
    #[inline(always)]
    fn from(val: Idlecfg) -> u8 {
        Idlecfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ilt {
    #[doc = "After the start bit"]
    FROM_START = 0x0,
    #[doc = "After the stop bit"]
    FROM_STOP = 0x01,
}
impl Ilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ilt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ilt {
    #[inline(always)]
    fn from(val: u8) -> Ilt {
        Ilt::from_bits(val)
    }
}
impl From<Ilt> for u8 {
    #[inline(always)]
    fn from(val: Ilt) -> u8 {
        Ilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loops {
    #[doc = "Normal operation: RXD and TXD use separate pins"]
    NOFFECT = 0x0,
    #[doc = "Loop mode or Single-Wire mode"]
    LOOPBACK = 0x01,
}
impl Loops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loops {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loops {
    #[inline(always)]
    fn from(val: u8) -> Loops {
        Loops::from_bits(val)
    }
}
impl From<Loops> for u8 {
    #[inline(always)]
    fn from(val: Loops) -> u8 {
        Loops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M {
    #[doc = "8-bit"]
    DATA8 = 0x0,
    #[doc = "9-bit"]
    DATA9 = 0x01,
}
impl M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M {
    #[inline(always)]
    fn from(val: u8) -> M {
        M::from_bits(val)
    }
}
impl From<M> for u8 {
    #[inline(always)]
    fn from(val: M) -> u8 {
        M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7 {
    #[doc = "8-bit to 10-bit"]
    NO_EFFECT = 0x0,
    #[doc = "7-bit"]
    DATA7 = 0x01,
}
impl M7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7 {
    #[inline(always)]
    fn from(val: u8) -> M7 {
        M7::from_bits(val)
    }
}
impl From<M7> for u8 {
    #[inline(always)]
    fn from(val: M7) -> u8 {
        M7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Address match wake-up"]
    ADDR_MATCH = 0x0,
    #[doc = "Idle match wake-up"]
    IDLE_MATCH = 0x01,
    #[doc = "Match on and match off"]
    ONOFF_MATCH = 0x02,
    #[doc = "Enables RWU on data match and match on or off for the transmitter CTS input"]
    RWU_MATCH = 0x03,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msbf {
    #[doc = "LSB"]
    LSB_FIRST = 0x0,
    #[doc = "MSB"]
    MSB_FIRST = 0x01,
}
impl Msbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msbf {
    #[inline(always)]
    fn from(val: u8) -> Msbf {
        Msbf::from_bits(val)
    }
}
impl From<Msbf> for u8 {
    #[inline(always)]
    fn from(val: Msbf) -> u8 {
        Msbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrCts {
    #[doc = "Logic one"]
    LOGIC_ONE = 0x0,
    #[doc = "Logic zero"]
    LOGIC_ZERO = 0x01,
}
impl MsrCts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrCts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrCts {
    #[inline(always)]
    fn from(val: u8) -> MsrCts {
        MsrCts::from_bits(val)
    }
}
impl From<MsrCts> for u8 {
    #[inline(always)]
    fn from(val: MsrCts) -> u8 {
        MsrCts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrDcd {
    #[doc = "Logic one"]
    LOGIC_ONE = 0x0,
    #[doc = "Logic zero"]
    LOGIC_ZERO = 0x01,
}
impl MsrDcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrDcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrDcd {
    #[inline(always)]
    fn from(val: u8) -> MsrDcd {
        MsrDcd::from_bits(val)
    }
}
impl From<MsrDcd> for u8 {
    #[inline(always)]
    fn from(val: MsrDcd) -> u8 {
        MsrDcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrDsr {
    #[doc = "Logic one"]
    LOGIC_ONE = 0x0,
    #[doc = "Logic zero"]
    LOGIC_ZERO = 0x01,
}
impl MsrDsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrDsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrDsr {
    #[inline(always)]
    fn from(val: u8) -> MsrDsr {
        MsrDsr::from_bits(val)
    }
}
impl From<MsrDsr> for u8 {
    #[inline(always)]
    fn from(val: MsrDsr) -> u8 {
        MsrDsr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrRin {
    #[doc = "Logic one"]
    LOGIC_ONE = 0x0,
    #[doc = "Logic zero"]
    LOGIC_ZERO = 0x01,
}
impl MsrRin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrRin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrRin {
    #[inline(always)]
    fn from(val: u8) -> MsrRin {
        MsrRin::from_bits(val)
    }
}
impl From<MsrRin> for u8 {
    #[inline(always)]
    fn from(val: MsrRin) -> u8 {
        MsrRin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osr {
    #[doc = "Results in an OSR of 16"]
    DEFAULT = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Results in an OSR of 4 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
    OSR_4 = 0x03,
    #[doc = "Results in an OSR of 5 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
    OSR_5 = 0x04,
    #[doc = "Results in an OSR of 6 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
    OSR_6 = 0x05,
    #[doc = "Results in an OSR of 7 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
    OSR_7 = 0x06,
    #[doc = "Results in an OSR of 8"]
    OSR_8 = 0x07,
    #[doc = "Results in an OSR of 9"]
    OSR_9 = 0x08,
    #[doc = "Results in an OSR of 10"]
    OSR_10 = 0x09,
    #[doc = "Results in an OSR of 11"]
    OSR_11 = 0x0a,
    #[doc = "Results in an OSR of 12"]
    OSR_12 = 0x0b,
    #[doc = "Results in an OSR of 13"]
    OSR_13 = 0x0c,
    #[doc = "Results in an OSR of 14"]
    OSR_14 = 0x0d,
    #[doc = "Results in an OSR of 15"]
    OSR_15 = 0x0e,
    #[doc = "Results in an OSR of 16"]
    OSR_16 = 0x0f,
    #[doc = "Results in an OSR of 17"]
    OSR_17 = 0x10,
    #[doc = "Results in an OSR of 18"]
    OSR_18 = 0x11,
    #[doc = "Results in an OSR of 19"]
    OSR_19 = 0x12,
    #[doc = "Results in an OSR of 20"]
    OSR_20 = 0x13,
    #[doc = "Results in an OSR of 21"]
    OSR_21 = 0x14,
    #[doc = "Results in an OSR of 22"]
    OSR_22 = 0x15,
    #[doc = "Results in an OSR of 23"]
    OSR_23 = 0x16,
    #[doc = "Results in an OSR of 24"]
    OSR_24 = 0x17,
    #[doc = "Results in an OSR of 25"]
    OSR_25 = 0x18,
    #[doc = "Results in an OSR of 26"]
    OSR_26 = 0x19,
    #[doc = "Results in an OSR of 27"]
    OSR_27 = 0x1a,
    #[doc = "Results in an OSR of 28"]
    OSR_28 = 0x1b,
    #[doc = "Results in an OSR of 29"]
    OSR_29 = 0x1c,
    #[doc = "Results in an OSR of 30"]
    OSR_30 = 0x1d,
    #[doc = "Results in an OSR of 31"]
    OSR_31 = 0x1e,
    #[doc = "Results in an OSR of 32"]
    OSR_32 = 0x1f,
}
impl Osr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osr {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osr {
    #[inline(always)]
    fn from(val: u8) -> Osr {
        Osr::from_bits(val)
    }
}
impl From<Osr> for u8 {
    #[inline(always)]
    fn from(val: Osr) -> u8 {
        Osr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt {
    #[doc = "Even parity"]
    EVEN = 0x0,
    #[doc = "Odd parity"]
    ODD = 0x01,
}
impl Pt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt {
    #[inline(always)]
    fn from(val: u8) -> Pt {
        Pt::from_bits(val)
    }
}
impl From<Pt> for u8 {
    #[inline(always)]
    fn from(val: Pt) -> u8 {
        Pt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raf {
    #[doc = "Idle, waiting for a start bit"]
    IDLE = 0x0,
    #[doc = "Receiver active (RXD pin input not idle)"]
    ACTIVE = 0x01,
}
impl Raf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raf {
    #[inline(always)]
    fn from(val: u8) -> Raf {
        Raf::from_bits(val)
    }
}
impl From<Raf> for u8 {
    #[inline(always)]
    fn from(val: Raf) -> u8 {
        Raf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resyncdis {
    #[doc = "Enable"]
    RESYNC = 0x0,
    #[doc = "Disable"]
    NO_RESYNC = 0x01,
}
impl Resyncdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resyncdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resyncdis {
    #[inline(always)]
    fn from(val: u8) -> Resyncdis {
        Resyncdis::from_bits(val)
    }
}
impl From<Resyncdis> for u8 {
    #[inline(always)]
    fn from(val: Resyncdis) -> u8 {
        Resyncdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsrc {
    #[doc = "Internal Loopback mode"]
    NO_EFFECT = 0x0,
    #[doc = "Single-wire mode"]
    ONEWIRE = 0x01,
}
impl Rsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsrc {
    #[inline(always)]
    fn from(val: u8) -> Rsrc {
        Rsrc::from_bits(val)
    }
}
impl From<Rsrc> for u8 {
    #[inline(always)]
    fn from(val: Rsrc) -> u8 {
        Rsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "Not reset"]
    NO_EFFECT = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
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
pub enum Rts {
    #[doc = "Logic one"]
    LOGIC_ONE = 0x0,
    #[doc = "Logic zero"]
    LOGIC_ZERO = 0x01,
}
impl Rts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rts {
    #[inline(always)]
    fn from(val: u8) -> Rts {
        Rts::from_bits(val)
    }
}
impl From<Rts> for u8 {
    #[inline(always)]
    fn from(val: Rts) -> u8 {
        Rts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwu {
    #[doc = "Normal receiver operation"]
    NO_EFFECT = 0x0,
    #[doc = "LPUART receiver in standby, waiting for a wake-up condition"]
    RX_WAKEUP = 0x01,
}
impl Rwu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwu {
    #[inline(always)]
    fn from(val: u8) -> Rwu {
        Rwu::from_bits(val)
    }
}
impl From<Rwu> for u8 {
    #[inline(always)]
    fn from(val: Rwu) -> u8 {
        Rwu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwuid {
    #[doc = "STAT\\[IDLE\\] does not become 1"]
    IDLE_NOTSET = 0x0,
    #[doc = "STAT\\[IDLE\\] becomes 1"]
    IDLE_SET = 0x01,
}
impl Rwuid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwuid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwuid {
    #[inline(always)]
    fn from(val: u8) -> Rwuid {
        Rwuid::from_bits(val)
    }
}
impl From<Rwuid> for u8 {
    #[inline(always)]
    fn from(val: Rwuid) -> u8 {
        Rwuid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfifosize {
    #[doc = "1"]
    FIFO_1 = 0x0,
    #[doc = "4"]
    FIFO_4 = 0x01,
    #[doc = "8"]
    FIFO_8 = 0x02,
    #[doc = "16"]
    FIFO_16 = 0x03,
    #[doc = "32"]
    FIFO_32 = 0x04,
    #[doc = "64"]
    FIFO_64 = 0x05,
    #[doc = "128"]
    FIFO_128 = 0x06,
    #[doc = "256"]
    FIFO_256 = 0x07,
}
impl Rxfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfifosize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfifosize {
    #[inline(always)]
    fn from(val: u8) -> Rxfifosize {
        Rxfifosize::from_bits(val)
    }
}
impl From<Rxfifosize> for u8 {
    #[inline(always)]
    fn from(val: Rxfifosize) -> u8 {
        Rxfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxflush {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "All data flushed out"]
    RXFIFO_RST = 0x01,
}
impl Rxflush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxflush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxflush {
    #[inline(always)]
    fn from(val: u8) -> Rxflush {
        Rxflush::from_bits(val)
    }
}
impl From<Rxflush> for u8 {
    #[inline(always)]
    fn from(val: Rxflush) -> u8 {
        Rxflush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxiden {
    #[doc = "Disable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle"]
    DISABLED = 0x0,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for one character"]
    IDLE_1 = 0x01,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for two characters"]
    IDLE_2 = 0x02,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for four characters"]
    IDLE_4 = 0x03,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for eight characters"]
    IDLE_8 = 0x04,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 16 characters"]
    IDLE_16 = 0x05,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 32 characters"]
    IDLE_32 = 0x06,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 64 characters"]
    IDLE_64 = 0x07,
}
impl Rxiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxiden {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxiden {
    #[inline(always)]
    fn from(val: u8) -> Rxiden {
        Rxiden::from_bits(val)
    }
}
impl From<Rxiden> for u8 {
    #[inline(always)]
    fn from(val: Rxiden) -> u8 {
        Rxiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxmsk {
    #[doc = "Do not mask"]
    NO_EFFECT = 0x0,
    #[doc = "Mask"]
    TX_RTS = 0x01,
}
impl Rxmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxmsk {
    #[inline(always)]
    fn from(val: u8) -> Rxmsk {
        Rxmsk::from_bits(val)
    }
}
impl From<Rxmsk> for u8 {
    #[inline(always)]
    fn from(val: Rxmsk) -> u8 {
        Rxmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxsel {
    #[doc = "RXD"]
    PIN_RXD = 0x0,
    #[doc = "TXD"]
    PIN_TXD = 0x01,
}
impl Rxsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxsel {
    #[inline(always)]
    fn from(val: u8) -> Rxsel {
        Rxsel::from_bits(val)
    }
}
impl From<Rxsel> for u8 {
    #[inline(always)]
    fn from(val: Rxsel) -> u8 {
        Rxsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxwrmsk {
    #[doc = "Do not mask"]
    NO_EFFECT = 0x0,
    #[doc = "Mask"]
    TX_RTS = 0x01,
}
impl Rxwrmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxwrmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxwrmsk {
    #[inline(always)]
    fn from(val: u8) -> Rxwrmsk {
        Rxwrmsk::from_bits(val)
    }
}
impl From<Rxwrmsk> for u8 {
    #[inline(always)]
    fn from(val: Rxwrmsk) -> u8 {
        Rxwrmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbk {
    #[doc = "Normal transmitter operation"]
    NO_EFFECT = 0x0,
    #[doc = "Queue break character(s) to be sent"]
    TX_BREAK = 0x01,
}
impl Sbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbk {
    #[inline(always)]
    fn from(val: u8) -> Sbk {
        Sbk::from_bits(val)
    }
}
impl From<Sbk> for u8 {
    #[inline(always)]
    fn from(val: Sbk) -> u8 {
        Sbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbns {
    #[doc = "One stop bit"]
    ONE = 0x0,
    #[doc = "Two stop bits"]
    TWO = 0x01,
}
impl Sbns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbns {
    #[inline(always)]
    fn from(val: u8) -> Sbns {
        Sbns::from_bits(val)
    }
}
impl From<Sbns> for u8 {
    #[inline(always)]
    fn from(val: Sbns) -> u8 {
        Sbns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tc {
    #[doc = "Transmitter active"]
    ACTIVE = 0x0,
    #[doc = "Transmitter idle"]
    COMPLETE = 0x01,
}
impl Tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tc {
    #[inline(always)]
    fn from(val: u8) -> Tc {
        Tc::from_bits(val)
    }
}
impl From<Tc> for u8 {
    #[inline(always)]
    fn from(val: Tc) -> u8 {
        Tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdre {
    #[doc = "Greater than watermark"]
    TXDATA = 0x0,
    #[doc = "Equal to or less than watermark"]
    NO_TXDATA = 0x01,
}
impl Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tdre {
        Tdre::from_bits(val)
    }
}
impl From<Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tdre) -> u8 {
        Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnp {
    #[doc = "1 / OSR"]
    ONE_SAMPLE = 0x0,
    #[doc = "2 / OSR"]
    TWO_SAMPLE = 0x01,
    #[doc = "3 / OSR"]
    THREE_SAMPLE = 0x02,
    #[doc = "4 / OSR"]
    FOUR_SAMPLE = 0x03,
}
impl Tnp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnp {
    #[inline(always)]
    fn from(val: u8) -> Tnp {
        Tnp::from_bits(val)
    }
}
impl From<Tnp> for u8 {
    #[inline(always)]
    fn from(val: Tnp) -> u8 {
        Tnp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tof {
    #[doc = "Not occurred"]
    NOT_OCCURRED = 0x0,
    #[doc = "Occurred"]
    OCCURRED = 0x01,
    _RESERVED_2 = 0x02,
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
}
impl Tof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tof {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tof {
    #[inline(always)]
    fn from(val: u8) -> Tof {
        Tof::from_bits(val)
    }
}
impl From<Tof> for u8 {
    #[inline(always)]
    fn from(val: Tof) -> u8 {
        Tof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsel {
    #[doc = "Input trigger disabled"]
    DISABLED = 0x0,
    #[doc = "Input trigger used instead of the RXD pin input"]
    TRG_RXD = 0x01,
    #[doc = "Input trigger used instead of the CTS_B pin input"]
    TRG_CTS = 0x02,
    #[doc = "Input trigger used to modulate the TXD pin output, which (after TXINV configuration) is internally ANDed with the input trigger"]
    TRG_TXD = 0x03,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txctsc {
    #[doc = "Sampled at the start of each character"]
    START = 0x0,
    #[doc = "Sampled when the transmitter is idle"]
    IDLE = 0x01,
}
impl Txctsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctsc {
    #[inline(always)]
    fn from(val: u8) -> Txctsc {
        Txctsc::from_bits(val)
    }
}
impl From<Txctsc> for u8 {
    #[inline(always)]
    fn from(val: Txctsc) -> u8 {
        Txctsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txctssrc {
    #[doc = "The CTS_B pin"]
    CTS = 0x0,
    #[doc = "An internal connection to the receiver address match result"]
    MATCH = 0x01,
}
impl Txctssrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctssrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctssrc {
    #[inline(always)]
    fn from(val: u8) -> Txctssrc {
        Txctssrc::from_bits(val)
    }
}
impl From<Txctssrc> for u8 {
    #[inline(always)]
    fn from(val: Txctssrc) -> u8 {
        Txctssrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdir {
    #[doc = "Input"]
    TX_INPUT = 0x0,
    #[doc = "Output"]
    TX_OUTPUT = 0x01,
}
impl Txdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdir {
    #[inline(always)]
    fn from(val: u8) -> Txdir {
        Txdir::from_bits(val)
    }
}
impl From<Txdir> for u8 {
    #[inline(always)]
    fn from(val: Txdir) -> u8 {
        Txdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfifosize {
    #[doc = "1"]
    FIFO_1 = 0x0,
    #[doc = "4"]
    FIFO_4 = 0x01,
    #[doc = "8"]
    FIFO_8 = 0x02,
    #[doc = "16"]
    FIFO_16 = 0x03,
    #[doc = "32"]
    FIFO_32 = 0x04,
    #[doc = "64"]
    FIFO_64 = 0x05,
    #[doc = "128"]
    FIFO_128 = 0x06,
    #[doc = "256"]
    FIFO_256 = 0x07,
}
impl Txfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfifosize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfifosize {
    #[inline(always)]
    fn from(val: u8) -> Txfifosize {
        Txfifosize::from_bits(val)
    }
}
impl From<Txfifosize> for u8 {
    #[inline(always)]
    fn from(val: Txfifosize) -> u8 {
        Txfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txflush {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "All data flushed out"]
    TXFIFO_RST = 0x01,
}
impl Txflush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txflush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txflush {
    #[inline(always)]
    fn from(val: u8) -> Txflush {
        Txflush::from_bits(val)
    }
}
impl From<Txflush> for u8 {
    #[inline(always)]
    fn from(val: Txflush) -> u8 {
        Txflush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txrtspol {
    #[doc = "Active low"]
    LOW = 0x0,
    #[doc = "Active high"]
    HIGH = 0x01,
}
impl Txrtspol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txrtspol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txrtspol {
    #[inline(always)]
    fn from(val: u8) -> Txrtspol {
        Txrtspol::from_bits(val)
    }
}
impl From<Txrtspol> for u8 {
    #[inline(always)]
    fn from(val: Txrtspol) -> u8 {
        Txrtspol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txstall {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Does not become busy"]
    RX_ACTIVE = 0x01,
}
impl Txstall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txstall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txstall {
    #[inline(always)]
    fn from(val: u8) -> Txstall {
        Txstall::from_bits(val)
    }
}
impl From<Txstall> for u8 {
    #[inline(always)]
    fn from(val: Txstall) -> u8 {
        Txstall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wake {
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Mark"]
    MARK = 0x01,
}
impl Wake {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wake {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wake {
    #[inline(always)]
    fn from(val: u8) -> Wake {
        Wake::from_bits(val)
    }
}
impl From<Wake> for u8 {
    #[inline(always)]
    fn from(val: Wake) -> u8 {
        Wake::to_bits(val)
    }
}
