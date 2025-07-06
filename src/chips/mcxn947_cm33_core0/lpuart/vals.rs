#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ame {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ame {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ame {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ame {
    #[inline(always)]
    fn from(val: u8) -> Ame {
        Ame::from_bits(val)
    }
}
impl From<Ame> for u8 {
    #[inline(always)]
    fn from(val: Ame) -> u8 {
        Ame::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bothedge {
    #[doc = "Rising edge"]
    DISABLED = 0x0,
    #[doc = "Both rising and falling edges"]
    ENABLED = 0x01,
}
impl Bothedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bothedge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bothedge {
    #[inline(always)]
    fn from(val: u8) -> Bothedge {
        Bothedge::from_bits(val)
    }
}
impl From<Bothedge> for u8 {
    #[inline(always)]
    fn from(val: Bothedge) -> u8 {
        Bothedge::to_bits(val)
    }
}
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
pub enum DataRxempt {
    #[doc = "Valid data"]
    NOT_EMPTY = 0x0,
    #[doc = "Invalid data and empty"]
    EMPTY = 0x01,
}
impl DataRxempt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DataRxempt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DataRxempt {
    #[inline(always)]
    fn from(val: u8) -> DataRxempt {
        DataRxempt::from_bits(val)
    }
}
impl From<DataRxempt> for u8 {
    #[inline(always)]
    fn from(val: DataRxempt) -> u8 {
        DataRxempt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcts {
    #[doc = "Did not change state"]
    NO_CHANGE = 0x0,
    #[doc = "Changed state"]
    CHANGE = 0x01,
}
impl Dcts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcts {
    #[inline(always)]
    fn from(val: u8) -> Dcts {
        Dcts::from_bits(val)
    }
}
impl From<Dcts> for u8 {
    #[inline(always)]
    fn from(val: Dcts) -> u8 {
        Dcts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ddcd {
    #[doc = "Did not change state"]
    NO_CHANGE = 0x0,
    #[doc = "Changed state"]
    CHANGE = 0x01,
}
impl Ddcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddcd {
    #[inline(always)]
    fn from(val: u8) -> Ddcd {
        Ddcd::from_bits(val)
    }
}
impl From<Ddcd> for u8 {
    #[inline(always)]
    fn from(val: Ddcd) -> u8 {
        Ddcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ddsr {
    #[doc = "Did not change state"]
    NO_CHANGE = 0x0,
    #[doc = "Changed state"]
    CHANGE = 0x01,
}
impl Ddsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddsr {
    #[inline(always)]
    fn from(val: u8) -> Ddsr {
        Ddsr::from_bits(val)
    }
}
impl From<Ddsr> for u8 {
    #[inline(always)]
    fn from(val: Ddsr) -> u8 {
        Ddsr::to_bits(val)
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
pub enum Dri {
    #[doc = "Did not change state"]
    NO_CHANGE = 0x0,
    #[doc = "Changed state"]
    CHANGE = 0x01,
}
impl Dri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dri {
    #[inline(always)]
    fn from(val: u8) -> Dri {
        Dri::from_bits(val)
    }
}
impl From<Dri> for u8 {
    #[inline(always)]
    fn from(val: Dri) -> u8 {
        Dri::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fe {
    #[doc = "No framing error detected (this does not guarantee that the framing is correct)"]
    NOERROR = 0x0,
    #[doc = "Framing error detected"]
    ERROR = 0x01,
}
impl Fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fe {
    #[inline(always)]
    fn from(val: u8) -> Fe {
        Fe::from_bits(val)
    }
}
impl From<Fe> for u8 {
    #[inline(always)]
    fn from(val: Fe) -> u8 {
        Fe::to_bits(val)
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
pub enum Feie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Feie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Feie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Feie {
    #[inline(always)]
    fn from(val: u8) -> Feie {
        Feie::from_bits(val)
    }
}
impl From<Feie> for u8 {
    #[inline(always)]
    fn from(val: Feie) -> u8 {
        Feie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoRxempt {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl FifoRxempt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoRxempt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoRxempt {
    #[inline(always)]
    fn from(val: u8) -> FifoRxempt {
        FifoRxempt::from_bits(val)
    }
}
impl From<FifoRxempt> for u8 {
    #[inline(always)]
    fn from(val: FifoRxempt) -> u8 {
        FifoRxempt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fretsc {
    #[doc = "Received without a frame error on reads or transmits a normal character on writes"]
    NO_ERROR = 0x0,
    #[doc = "Received with a frame error on reads or transmits an idle or break character on writes"]
    ERROR = 0x01,
}
impl Fretsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fretsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fretsc {
    #[inline(always)]
    fn from(val: u8) -> Fretsc {
        Fretsc::from_bits(val)
    }
}
impl From<Fretsc> for u8 {
    #[inline(always)]
    fn from(val: Fretsc) -> u8 {
        Fretsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idle {
    #[doc = "Idle line detected"]
    NOIDLE = 0x0,
    #[doc = "Idle line not detected"]
    IDLE = 0x01,
}
impl Idle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idle {
    #[inline(always)]
    fn from(val: u8) -> Idle {
        Idle::from_bits(val)
    }
}
impl From<Idle> for u8 {
    #[inline(always)]
    fn from(val: Idle) -> u8 {
        Idle::to_bits(val)
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
pub enum Idline {
    #[doc = "Not idle"]
    NO_IDLE = 0x0,
    #[doc = "Idle"]
    IDLE = 0x01,
}
impl Idline {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idline {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idline {
    #[inline(always)]
    fn from(val: u8) -> Idline {
        Idline::from_bits(val)
    }
}
impl From<Idline> for u8 {
    #[inline(always)]
    fn from(val: Idline) -> u8 {
        Idline::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ilie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ilie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ilie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ilie {
    #[inline(always)]
    fn from(val: u8) -> Ilie {
        Ilie::from_bits(val)
    }
}
impl From<Ilie> for u8 {
    #[inline(always)]
    fn from(val: Ilie) -> u8 {
        Ilie::to_bits(val)
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
pub enum Iren {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Iren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iren {
    #[inline(always)]
    fn from(val: u8) -> Iren {
        Iren::from_bits(val)
    }
}
impl From<Iren> for u8 {
    #[inline(always)]
    fn from(val: Iren) -> u8 {
        Iren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbkde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Lbkde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbkde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbkde {
    #[inline(always)]
    fn from(val: u8) -> Lbkde {
        Lbkde::from_bits(val)
    }
}
impl From<Lbkde> for u8 {
    #[inline(always)]
    fn from(val: Lbkde) -> u8 {
        Lbkde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbkdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Lbkdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbkdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbkdie {
    #[inline(always)]
    fn from(val: u8) -> Lbkdie {
        Lbkdie::from_bits(val)
    }
}
impl From<Lbkdie> for u8 {
    #[inline(always)]
    fn from(val: Lbkdie) -> u8 {
        Lbkdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbkdif {
    #[doc = "Not detected"]
    NOT_DETECTED = 0x0,
    #[doc = "Detected"]
    DETECTED = 0x01,
}
impl Lbkdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbkdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbkdif {
    #[inline(always)]
    fn from(val: u8) -> Lbkdif {
        Lbkdif::from_bits(val)
    }
}
impl From<Lbkdif> for u8 {
    #[inline(always)]
    fn from(val: Lbkdif) -> u8 {
        Lbkdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbkfe {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Lbkfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbkfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbkfe {
    #[inline(always)]
    fn from(val: u8) -> Lbkfe {
        Lbkfe::from_bits(val)
    }
}
impl From<Lbkfe> for u8 {
    #[inline(always)]
    fn from(val: Lbkfe) -> u8 {
        Lbkfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linbrk {
    #[doc = "Not detected"]
    NO_BREAK = 0x0,
    #[doc = "Detected"]
    BREAK = 0x01,
}
impl Linbrk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linbrk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linbrk {
    #[inline(always)]
    fn from(val: u8) -> Linbrk {
        Linbrk::from_bits(val)
    }
}
impl From<Linbrk> for u8 {
    #[inline(always)]
    fn from(val: Linbrk) -> u8 {
        Linbrk::to_bits(val)
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
pub enum M10 {
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters"]
    DISABLED = 0x0,
    #[doc = "Receiver and transmitter use 10-bit data characters"]
    ENABLED = 0x01,
}
impl M10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M10 {
    #[inline(always)]
    fn from(val: u8) -> M10 {
        M10::from_bits(val)
    }
}
impl From<M10> for u8 {
    #[inline(always)]
    fn from(val: M10) -> u8 {
        M10::to_bits(val)
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
pub enum Ma1f {
    #[doc = "Not equal to MA1"]
    NOMATCH = 0x0,
    #[doc = "Equal to MA1"]
    MATCH = 0x01,
}
impl Ma1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ma1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ma1f {
    #[inline(always)]
    fn from(val: u8) -> Ma1f {
        Ma1f::from_bits(val)
    }
}
impl From<Ma1f> for u8 {
    #[inline(always)]
    fn from(val: Ma1f) -> u8 {
        Ma1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ma1ie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ma1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ma1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ma1ie {
    #[inline(always)]
    fn from(val: u8) -> Ma1ie {
        Ma1ie::from_bits(val)
    }
}
impl From<Ma1ie> for u8 {
    #[inline(always)]
    fn from(val: Ma1ie) -> u8 {
        Ma1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ma2f {
    #[doc = "Not equal to MA2"]
    NOMATCH = 0x0,
    #[doc = "Equal to MA2"]
    MATCH = 0x01,
}
impl Ma2f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ma2f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ma2f {
    #[inline(always)]
    fn from(val: u8) -> Ma2f {
        Ma2f::from_bits(val)
    }
}
impl From<Ma2f> for u8 {
    #[inline(always)]
    fn from(val: Ma2f) -> u8 {
        Ma2f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ma2ie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ma2ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ma2ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ma2ie {
    #[inline(always)]
    fn from(val: u8) -> Ma2ie {
        Ma2ie::from_bits(val)
    }
}
impl From<Ma2ie> for u8 {
    #[inline(always)]
    fn from(val: Ma2ie) -> u8 {
        Ma2ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Maen1 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Maen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maen1 {
    #[inline(always)]
    fn from(val: u8) -> Maen1 {
        Maen1::from_bits(val)
    }
}
impl From<Maen1> for u8 {
    #[inline(always)]
    fn from(val: Maen1) -> u8 {
        Maen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Maen2 {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Maen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maen2 {
    #[inline(always)]
    fn from(val: u8) -> Maen2 {
        Maen2::from_bits(val)
    }
}
impl From<Maen2> for u8 {
    #[inline(always)]
    fn from(val: Maen2) -> u8 {
        Maen2::to_bits(val)
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
pub enum McrCts {
    #[doc = "Disable interrupt"]
    DISABLED = 0x0,
    #[doc = "Enable interrupt"]
    ENABLED = 0x01,
}
impl McrCts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrCts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrCts {
    #[inline(always)]
    fn from(val: u8) -> McrCts {
        McrCts::from_bits(val)
    }
}
impl From<McrCts> for u8 {
    #[inline(always)]
    fn from(val: McrCts) -> u8 {
        McrCts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrDcd {
    #[doc = "Disable interrupt"]
    DISABLED = 0x0,
    #[doc = "Enable interrupt"]
    ENABLED = 0x01,
}
impl McrDcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrDcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrDcd {
    #[inline(always)]
    fn from(val: u8) -> McrDcd {
        McrDcd::from_bits(val)
    }
}
impl From<McrDcd> for u8 {
    #[inline(always)]
    fn from(val: McrDcd) -> u8 {
        McrDcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrDsr {
    #[doc = "Disable interrupt"]
    DISABLED = 0x0,
    #[doc = "Enable interrupt"]
    ENABLED = 0x01,
}
impl McrDsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrDsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrDsr {
    #[inline(always)]
    fn from(val: u8) -> McrDsr {
        McrDsr::from_bits(val)
    }
}
impl From<McrDsr> for u8 {
    #[inline(always)]
    fn from(val: McrDsr) -> u8 {
        McrDsr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrRin {
    #[doc = "Disable interrupt"]
    DISABLED = 0x0,
    #[doc = "Enable interrupt"]
    ENABLED = 0x01,
}
impl McrRin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrRin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrRin {
    #[inline(always)]
    fn from(val: u8) -> McrRin {
        McrRin::from_bits(val)
    }
}
impl From<McrRin> for u8 {
    #[inline(always)]
    fn from(val: McrRin) -> u8 {
        McrRin::to_bits(val)
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
pub enum Msf {
    #[doc = "Field is 0"]
    NOFLAG = 0x0,
    #[doc = "Field is 1"]
    FLAG = 0x01,
}
impl Msf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msf {
    #[inline(always)]
    fn from(val: u8) -> Msf {
        Msf::from_bits(val)
    }
}
impl From<Msf> for u8 {
    #[inline(always)]
    fn from(val: Msf) -> u8 {
        Msf::to_bits(val)
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
pub enum Neie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Neie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Neie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Neie {
    #[inline(always)]
    fn from(val: u8) -> Neie {
        Neie::from_bits(val)
    }
}
impl From<Neie> for u8 {
    #[inline(always)]
    fn from(val: Neie) -> u8 {
        Neie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nf {
    #[doc = "No noise detected"]
    NONOISE = 0x0,
    #[doc = "Noise detected"]
    NOISE = 0x01,
}
impl Nf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nf {
    #[inline(always)]
    fn from(val: u8) -> Nf {
        Nf::from_bits(val)
    }
}
impl From<Nf> for u8 {
    #[inline(always)]
    fn from(val: Nf) -> u8 {
        Nf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noisy {
    #[doc = "Received without noise"]
    NO_NOISE = 0x0,
    #[doc = "Received with noise"]
    NOISE = 0x01,
}
impl Noisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noisy {
    #[inline(always)]
    fn from(val: u8) -> Noisy {
        Noisy::from_bits(val)
    }
}
impl From<Noisy> for u8 {
    #[inline(always)]
    fn from(val: Noisy) -> u8 {
        Noisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Or {
    #[doc = "No overrun"]
    NO_OVERRUN = 0x0,
    #[doc = "Receive overrun (new LPUART data is lost)"]
    OVERRUN = 0x01,
}
impl Or {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Or {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Or {
    #[inline(always)]
    fn from(val: u8) -> Or {
        Or::from_bits(val)
    }
}
impl From<Or> for u8 {
    #[inline(always)]
    fn from(val: Or) -> u8 {
        Or::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Orie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Orie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Orie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Orie {
    #[inline(always)]
    fn from(val: u8) -> Orie {
        Orie::from_bits(val)
    }
}
impl From<Orie> for u8 {
    #[inline(always)]
    fn from(val: Orie) -> u8 {
        Orie::to_bits(val)
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
pub enum Paritye {
    #[doc = "Received without a parity error"]
    NO_PARITY = 0x0,
    #[doc = "Received with a parity error"]
    PARITY = 0x01,
}
impl Paritye {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paritye {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paritye {
    #[inline(always)]
    fn from(val: u8) -> Paritye {
        Paritye::from_bits(val)
    }
}
impl From<Paritye> for u8 {
    #[inline(always)]
    fn from(val: Paritye) -> u8 {
        Paritye::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe {
    #[inline(always)]
    fn from(val: u8) -> Pe {
        Pe::from_bits(val)
    }
}
impl From<Pe> for u8 {
    #[inline(always)]
    fn from(val: Pe) -> u8 {
        Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Peie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Peie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Peie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Peie {
    #[inline(always)]
    fn from(val: u8) -> Peie {
        Peie::from_bits(val)
    }
}
impl From<Peie> for u8 {
    #[inline(always)]
    fn from(val: Peie) -> u8 {
        Peie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pf {
    #[doc = "No parity error detected"]
    NOPARITY = 0x0,
    #[doc = "Parity error detected"]
    PARITY = 0x01,
}
impl Pf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pf {
    #[inline(always)]
    fn from(val: u8) -> Pf {
        Pf::from_bits(val)
    }
}
impl From<Pf> for u8 {
    #[inline(always)]
    fn from(val: Pf) -> u8 {
        Pf::to_bits(val)
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
pub enum Rdmae {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rdmae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdmae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdmae {
    #[inline(always)]
    fn from(val: u8) -> Rdmae {
        Rdmae::from_bits(val)
    }
}
impl From<Rdmae> for u8 {
    #[inline(always)]
    fn from(val: Rdmae) -> u8 {
        Rdmae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdrf {
    #[doc = "Equal to or less than watermark"]
    NO_RXDATA = 0x0,
    #[doc = "Greater than watermark"]
    RXDATA = 0x01,
}
impl Rdrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdrf {
    #[inline(always)]
    fn from(val: u8) -> Rdrf {
        Rdrf::from_bits(val)
    }
}
impl From<Rdrf> for u8 {
    #[inline(always)]
    fn from(val: Rdrf) -> u8 {
        Rdrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Re {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Re {
    #[inline(always)]
    fn from(val: u8) -> Re {
        Re::from_bits(val)
    }
}
impl From<Re> for u8 {
    #[inline(always)]
    fn from(val: Re) -> u8 {
        Re::to_bits(val)
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
pub enum Ridmae {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ridmae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ridmae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ridmae {
    #[inline(always)]
    fn from(val: u8) -> Ridmae {
        Ridmae::from_bits(val)
    }
}
impl From<Ridmae> for u8 {
    #[inline(always)]
    fn from(val: Ridmae) -> u8 {
        Ridmae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rie {
    #[inline(always)]
    fn from(val: u8) -> Rie {
        Rie::from_bits(val)
    }
}
impl From<Rie> for u8 {
    #[inline(always)]
    fn from(val: Rie) -> u8 {
        Rie::to_bits(val)
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
pub enum Rxedgie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Rxedgie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxedgie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxedgie {
    #[inline(always)]
    fn from(val: u8) -> Rxedgie {
        Rxedgie::from_bits(val)
    }
}
impl From<Rxedgie> for u8 {
    #[inline(always)]
    fn from(val: Rxedgie) -> u8 {
        Rxedgie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxedgif {
    #[doc = "Not occurred"]
    NO_EDGE = 0x0,
    #[doc = "Occurred"]
    EDGE = 0x01,
}
impl Rxedgif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxedgif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxedgif {
    #[inline(always)]
    fn from(val: u8) -> Rxedgif {
        Rxedgif::from_bits(val)
    }
}
impl From<Rxedgif> for u8 {
    #[inline(always)]
    fn from(val: Rxedgif) -> u8 {
        Rxedgif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfe {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rxfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfe {
    #[inline(always)]
    fn from(val: u8) -> Rxfe {
        Rxfe::from_bits(val)
    }
}
impl From<Rxfe> for u8 {
    #[inline(always)]
    fn from(val: Rxfe) -> u8 {
        Rxfe::to_bits(val)
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
pub enum Rxinv {
    #[doc = "Inverted"]
    NOT_INVERTED = 0x0,
    #[doc = "Not inverted"]
    INVERTED = 0x01,
}
impl Rxinv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxinv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxinv {
    #[inline(always)]
    fn from(val: u8) -> Rxinv {
        Rxinv::from_bits(val)
    }
}
impl From<Rxinv> for u8 {
    #[inline(always)]
    fn from(val: Rxinv) -> u8 {
        Rxinv::to_bits(val)
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
pub enum Rxrtse {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rxrtse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxrtse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxrtse {
    #[inline(always)]
    fn from(val: u8) -> Rxrtse {
        Rxrtse::from_bits(val)
    }
}
impl From<Rxrtse> for u8 {
    #[inline(always)]
    fn from(val: Rxrtse) -> u8 {
        Rxrtse::to_bits(val)
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
pub enum Rxuf {
    #[doc = "No underflow"]
    NO_UNDERFLOW = 0x0,
    #[doc = "Underflow"]
    UNDERFLOW = 0x01,
}
impl Rxuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxuf {
    #[inline(always)]
    fn from(val: u8) -> Rxuf {
        Rxuf::from_bits(val)
    }
}
impl From<Rxuf> for u8 {
    #[inline(always)]
    fn from(val: Rxuf) -> u8 {
        Rxuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxufe {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rxufe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxufe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxufe {
    #[inline(always)]
    fn from(val: u8) -> Rxufe {
        Rxufe::from_bits(val)
    }
}
impl From<Rxufe> for u8 {
    #[inline(always)]
    fn from(val: Rxufe) -> u8 {
        Rxufe::to_bits(val)
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
pub enum Tcie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Tcie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcie {
    #[inline(always)]
    fn from(val: u8) -> Tcie {
        Tcie::from_bits(val)
    }
}
impl From<Tcie> for u8 {
    #[inline(always)]
    fn from(val: Tcie) -> u8 {
        Tcie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdmae {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Tdmae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdmae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdmae {
    #[inline(always)]
    fn from(val: u8) -> Tdmae {
        Tdmae::from_bits(val)
    }
}
impl From<Tdmae> for u8 {
    #[inline(always)]
    fn from(val: Tdmae) -> u8 {
        Tdmae::to_bits(val)
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
pub enum Te {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Te {
    #[inline(always)]
    fn from(val: u8) -> Te {
        Te::from_bits(val)
    }
}
impl From<Te> for u8 {
    #[inline(always)]
    fn from(val: Te) -> u8 {
        Te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tie {
    #[inline(always)]
    fn from(val: u8) -> Tie {
        Tie::from_bits(val)
    }
}
impl From<Tie> for u8 {
    #[inline(always)]
    fn from(val: Tie) -> u8 {
        Tie::to_bits(val)
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
pub enum Tsf {
    #[doc = "Field is 0"]
    NOFLAG = 0x0,
    #[doc = "Field is 1"]
    FLAG = 0x01,
}
impl Tsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsf {
    #[inline(always)]
    fn from(val: u8) -> Tsf {
        Tsf::from_bits(val)
    }
}
impl From<Tsf> for u8 {
    #[inline(always)]
    fn from(val: Tsf) -> u8 {
        Tsf::to_bits(val)
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
pub enum Txctse {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Txctse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctse {
    #[inline(always)]
    fn from(val: u8) -> Txctse {
        Txctse::from_bits(val)
    }
}
impl From<Txctse> for u8 {
    #[inline(always)]
    fn from(val: Txctse) -> u8 {
        Txctse::to_bits(val)
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
pub enum Txempt {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl Txempt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txempt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txempt {
    #[inline(always)]
    fn from(val: u8) -> Txempt {
        Txempt::from_bits(val)
    }
}
impl From<Txempt> for u8 {
    #[inline(always)]
    fn from(val: Txempt) -> u8 {
        Txempt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfe {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Txfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfe {
    #[inline(always)]
    fn from(val: u8) -> Txfe {
        Txfe::from_bits(val)
    }
}
impl From<Txfe> for u8 {
    #[inline(always)]
    fn from(val: Txfe) -> u8 {
        Txfe::to_bits(val)
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
pub enum Txinv {
    #[doc = "Not inverted"]
    NOT_INVERTED = 0x0,
    #[doc = "Inverted"]
    INVERTED = 0x01,
}
impl Txinv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txinv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txinv {
    #[inline(always)]
    fn from(val: u8) -> Txinv {
        Txinv::from_bits(val)
    }
}
impl From<Txinv> for u8 {
    #[inline(always)]
    fn from(val: Txinv) -> u8 {
        Txinv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txof {
    #[doc = "No overflow"]
    NO_OVERFLOW = 0x0,
    #[doc = "Overflow"]
    OVERFLOW = 0x01,
}
impl Txof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txof {
    #[inline(always)]
    fn from(val: u8) -> Txof {
        Txof::from_bits(val)
    }
}
impl From<Txof> for u8 {
    #[inline(always)]
    fn from(val: Txof) -> u8 {
        Txof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txofe {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Txofe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txofe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txofe {
    #[inline(always)]
    fn from(val: u8) -> Txofe {
        Txofe::from_bits(val)
    }
}
impl From<Txofe> for u8 {
    #[inline(always)]
    fn from(val: Txofe) -> u8 {
        Txofe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txrtse {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Txrtse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txrtse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txrtse {
    #[inline(always)]
    fn from(val: u8) -> Txrtse {
        Txrtse::from_bits(val)
    }
}
impl From<Txrtse> for u8 {
    #[inline(always)]
    fn from(val: Txrtse) -> u8 {
        Txrtse::to_bits(val)
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
