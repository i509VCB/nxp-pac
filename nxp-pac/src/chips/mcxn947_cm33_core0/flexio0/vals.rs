#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbge {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    EMABLE = 0x01,
}
impl Dbge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbge {
    #[inline(always)]
    fn from(val: u8) -> Dbge {
        Dbge::from_bits(val)
    }
}
impl From<Dbge> for u8 {
    #[inline(always)]
    fn from(val: Dbge) -> u8 {
        Dbge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
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
pub struct Etsf(u8);
impl Etsf {
    #[doc = "Clear"]
    pub const CLR: Self = Self(0x0);
    #[doc = "Set"]
    pub const SET: Self = Self(0x01);
}
impl Etsf {
    pub const fn from_bits(val: u8) -> Etsf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Etsf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLR"),
            0x01 => f.write_str("SET"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etsf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLR"),
            0x01 => defmt::write!(f, "SET"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Etsf {
    #[inline(always)]
    fn from(val: u8) -> Etsf {
        Etsf::from_bits(val)
    }
}
impl From<Etsf> for u8 {
    #[inline(always)]
    fn from(val: Etsf) -> u8 {
        Etsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fastacc {
    #[doc = "Normal"]
    NORMAL = 0x0,
    #[doc = "Fast"]
    FAST = 0x01,
}
impl Fastacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fastacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fastacc {
    #[inline(always)]
    fn from(val: u8) -> Fastacc {
        Fastacc::from_bits(val)
    }
}
impl From<Fastacc> for u8 {
    #[inline(always)]
    fn from(val: Fastacc) -> u8 {
        Fastacc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features implemented"]
    pub const STANDARD: Self = Self(0x0);
    #[doc = "State, logic, and parallel modes supported"]
    pub const STATE_LOGIC_PARALLEL: Self = Self(0x01);
    #[doc = "Pin control registers supported"]
    pub const PINCTRL: Self = Self(0x02);
    #[doc = "State, logic, and parallel modes, plus pin control registers supported"]
    pub const STATE_LOGIC_PARALLEL_PINCTRL: Self = Self(0x03);
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
            0x0 => f.write_str("STANDARD"),
            0x01 => f.write_str("STATE_LOGIC_PARALLEL"),
            0x02 => f.write_str("PINCTRL"),
            0x03 => f.write_str("STATE_LOGIC_PARALLEL_PINCTRL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STANDARD"),
            0x01 => defmt::write!(f, "STATE_LOGIC_PARALLEL"),
            0x02 => defmt::write!(f, "PINCTRL"),
            0x03 => defmt::write!(f, "STATE_LOGIC_PARALLEL_PINCTRL"),
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
pub enum Insrc {
    #[doc = "Pin"]
    PIN = 0x0,
    #[doc = "Shifter n+1 output"]
    SHIFTER_NPLUS1 = 0x01,
}
impl Insrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Insrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Insrc {
    #[inline(always)]
    fn from(val: u8) -> Insrc {
        Insrc::from_bits(val)
    }
}
impl From<Insrc> for u8 {
    #[inline(always)]
    fn from(val: Insrc) -> u8 {
        Insrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latst {
    #[doc = "Store the pre-shift register state"]
    PRESHIFT = 0x0,
    #[doc = "Store the post-shift register state"]
    POSTSHIFT = 0x01,
}
impl Latst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latst {
    #[inline(always)]
    fn from(val: u8) -> Latst {
        Latst::from_bits(val)
    }
}
impl From<Latst> for u8 {
    #[inline(always)]
    fn from(val: Latst) -> u8 {
        Latst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pinins {
    #[doc = "PINSEL selects timer pin input and output"]
    PINSEL = 0x0,
    #[doc = "PINSEL + 1 selects the timer pin input; timer pin output remains selected by PINSEL"]
    PINSELPLUS1 = 0x01,
}
impl Pinins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pinins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pinins {
    #[inline(always)]
    fn from(val: u8) -> Pinins {
        Pinins::from_bits(val)
    }
}
impl From<Pinins> for u8 {
    #[inline(always)]
    fn from(val: Pinins) -> u8 {
        Pinins::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Psf(u32);
impl Psf {
    #[doc = "Clear"]
    pub const CLR: Self = Self(0x0);
    #[doc = "Set"]
    pub const SET: Self = Self(0x01);
}
impl Psf {
    pub const fn from_bits(val: u32) -> Psf {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Psf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLR"),
            0x01 => f.write_str("SET"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLR"),
            0x01 => defmt::write!(f, "SET"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Psf {
    #[inline(always)]
    fn from(val: u32) -> Psf {
        Psf::from_bits(val)
    }
}
impl From<Psf> for u32 {
    #[inline(always)]
    fn from(val: Psf) -> u32 {
        Psf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sef(u8);
impl Sef {
    #[doc = "Clear"]
    pub const CLR: Self = Self(0x0);
    #[doc = "Set"]
    pub const SET: Self = Self(0x01);
}
impl Sef {
    pub const fn from_bits(val: u8) -> Sef {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Sef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLR"),
            0x01 => f.write_str("SET"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sef {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLR"),
            0x01 => defmt::write!(f, "SET"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Sef {
    #[inline(always)]
    fn from(val: u8) -> Sef {
        Sef::from_bits(val)
    }
}
impl From<Sef> for u8 {
    #[inline(always)]
    fn from(val: Sef) -> u8 {
        Sef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShiftctlPincfg {
    #[doc = "Shifter pin output disabled"]
    DISABLE = 0x0,
    #[doc = "Shifter pin open-drain or bidirectional output enable"]
    OPEND_BIDIROUTEN = 0x01,
    #[doc = "Shifter pin bidirectional output data"]
    BIDIR_OUTDATA = 0x02,
    #[doc = "Shifter pin output"]
    OUTPUT = 0x03,
}
impl ShiftctlPincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShiftctlPincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShiftctlPincfg {
    #[inline(always)]
    fn from(val: u8) -> ShiftctlPincfg {
        ShiftctlPincfg::from_bits(val)
    }
}
impl From<ShiftctlPincfg> for u8 {
    #[inline(always)]
    fn from(val: ShiftctlPincfg) -> u8 {
        ShiftctlPincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShiftctlPinpol {
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
    ACTIVE_LOW = 0x01,
}
impl ShiftctlPinpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShiftctlPinpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShiftctlPinpol {
    #[inline(always)]
    fn from(val: u8) -> ShiftctlPinpol {
        ShiftctlPinpol::from_bits(val)
    }
}
impl From<ShiftctlPinpol> for u8 {
    #[inline(always)]
    fn from(val: ShiftctlPinpol) -> u8 {
        ShiftctlPinpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Receive mode; capture the current shifter content into SHIFTBUF on expiration of the timer"]
    RECEIVE = 0x01,
    #[doc = "Transmit mode; load SHIFTBUF contents into the shifter on expiration of the timer"]
    TRANSMIT = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Match Store mode; shifter data is compared to SHIFTBUF content on expiration of the timer"]
    MATCHSTORE = 0x04,
    #[doc = "Match Continuous mode; shifter data is continuously compared to SHIFTBUF contents"]
    MATCHCONT = 0x05,
    #[doc = "State mode; SHIFTBUF contents store programmable state attributes"]
    STATE = 0x06,
    #[doc = "Logic mode; SHIFTBUF contents implement programmable logic lookup table"]
    LOGIC = 0x07,
}
impl Smod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ssf(u8);
impl Ssf {
    #[doc = "Clear"]
    pub const CLR: Self = Self(0x0);
    #[doc = "Set"]
    pub const SET: Self = Self(0x01);
}
impl Ssf {
    pub const fn from_bits(val: u8) -> Ssf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ssf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLR"),
            0x01 => f.write_str("SET"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLR"),
            0x01 => defmt::write!(f, "SET"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ssf {
    #[inline(always)]
    fn from(val: u8) -> Ssf {
        Ssf::from_bits(val)
    }
}
impl From<Ssf> for u8 {
    #[inline(always)]
    fn from(val: Ssf) -> u8 {
        Ssf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssize {
    #[doc = "32-bit"]
    WIDTH32 = 0x0,
    #[doc = "24-bit"]
    WIDTH24 = 0x01,
}
impl Ssize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssize {
    #[inline(always)]
    fn from(val: u8) -> Ssize {
        Ssize::from_bits(val)
    }
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(val: Ssize) -> u8 {
        Ssize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstart {
    #[doc = "Start bit disabled for Transmitter, Receiver, and Match Store modes; Transmitter mode loads data on enable"]
    VALUE00 = 0x0,
    #[doc = "Start bit disabled for Transmitter, Receiver, and Match Store modes; Transmitter mode loads data on first shift"]
    VALUE01 = 0x01,
    #[doc = "Transmitter mode outputs start bit value 0 before loading data on first shift; if start bit is not 0, Receiver and Match Store modes set error flag"]
    VALUE10 = 0x02,
    #[doc = "Transmitter mode outputs start bit value 1 before loading data on first shift; if start bit is not 1, Receiver and Match Store modes set error flag"]
    VALUE11 = 0x03,
}
impl Sstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstart {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstart {
    #[inline(always)]
    fn from(val: u8) -> Sstart {
        Sstart::from_bits(val)
    }
}
impl From<Sstart> for u8 {
    #[inline(always)]
    fn from(val: Sstart) -> u8 {
        Sstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstop {
    #[doc = "Stop bit disabled for Transmitter, Receiver, and Match Store modes"]
    VALUE00 = 0x0,
    #[doc = "Stop bit disabled for Transmitter, Receiver, and Match Store modes; when timer is in stop condition, Receiver and Match Store modes store receive data on the configured shift edge"]
    VALUE01 = 0x01,
    #[doc = "Transmitter mode outputs stop bit value 0 in Match Store mode; if stop bit is not 0, Receiver and Match Store modes set error flag (when timer is in stop condition, these modes also store receive data on the configured shift edge)"]
    VALUE10 = 0x02,
    #[doc = "Transmitter mode outputs stop bit value 1 in Match Store mode; if stop bit is not 1, Receiver and Match Store modes set error flag (when timer is in stop condition, these modes also store receive data on the configured shift edge)"]
    VALUE11 = 0x03,
}
impl Sstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstop {
    #[inline(always)]
    fn from(val: u8) -> Sstop {
        Sstop::from_bits(val)
    }
}
impl From<Sstop> for u8 {
    #[inline(always)]
    fn from(val: Sstop) -> u8 {
        Sstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimctlPincfg {
    #[doc = "Timer pin output disabled"]
    OUTDISABLE = 0x0,
    #[doc = "Timer pin open-drain or bidirectional output enable"]
    OPEND_BIDIROUTEN = 0x01,
    #[doc = "Timer pin bidirectional output data"]
    BIDIR_OUTDATA = 0x02,
    #[doc = "Timer pin output"]
    OUTPUT = 0x03,
}
impl TimctlPincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimctlPincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimctlPincfg {
    #[inline(always)]
    fn from(val: u8) -> TimctlPincfg {
        TimctlPincfg::from_bits(val)
    }
}
impl From<TimctlPincfg> for u8 {
    #[inline(always)]
    fn from(val: TimctlPincfg) -> u8 {
        TimctlPincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimctlPinpol {
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
    ACTIVE_LOW = 0x01,
}
impl TimctlPinpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimctlPinpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimctlPinpol {
    #[inline(always)]
    fn from(val: u8) -> TimctlPinpol {
        TimctlPinpol::from_bits(val)
    }
}
impl From<TimctlPinpol> for u8 {
    #[inline(always)]
    fn from(val: TimctlPinpol) -> u8 {
        TimctlPinpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timdec {
    #[doc = "Decrement counter on FLEXIO clock; shift clock equals timer output"]
    FLEXIO_CLK_SHIFTCLK_TMR_OUT = 0x0,
    #[doc = "Decrement counter on trigger input (both edges); shift clock equals timer output"]
    TRIG_EDGE_SHIFTCLK_TMR_OUT = 0x01,
    #[doc = "Decrement counter on pin input (both edges); shift clock equals pin input"]
    PIN_EDGE_SHIFTCLK_TMR_OUT = 0x02,
    #[doc = "Decrement counter on trigger input (both edges); shift clock equals trigger input"]
    TRIG_EDGE_SHIFTCLK_TRIG_IN = 0x03,
    #[doc = "Decrement counter on FLEXIO clock divided by 16; shift clock equals timer output"]
    FLEXIO_CLK_DIV16_SHIFTCLK_TMR_OUT = 0x04,
    #[doc = "Decrement counter on FLEXIO clock divided by 256; shift clock equals timer output"]
    FLEXIO_CLK_DIV256_SHIFTCLK_TMR_OUT = 0x05,
    #[doc = "Decrement counter on pin input (rising edge); shift clock equals pin input"]
    PIN_RISE_SHIFTCLK_PIN_IN = 0x06,
    #[doc = "Decrement counter on trigger input (rising edge); shift clock equals trigger input"]
    TRIG_RISE_SHIFTCLK_TRIG_IN = 0x07,
}
impl Timdec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timdec {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timdec {
    #[inline(always)]
    fn from(val: u8) -> Timdec {
        Timdec::from_bits(val)
    }
}
impl From<Timdec> for u8 {
    #[inline(always)]
    fn from(val: Timdec) -> u8 {
        Timdec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timdis {
    #[doc = "Timer never disabled"]
    NEVER = 0x0,
    #[doc = "Timer disabled on timer n-1 disable"]
    TMR_NMINUS1 = 0x01,
    #[doc = "Timer disabled on timer compare (upper 8 bits match and decrement)"]
    TMR_CMP = 0x02,
    #[doc = "Timer disabled on timer compare (upper 8 bits match and decrement) and trigger low"]
    TMR_CMP_TRIGLOW = 0x03,
    #[doc = "Timer disabled on pin rising or falling edge"]
    PIN_EDGE = 0x04,
    #[doc = "Timer disabled on pin rising or falling edge provided trigger is high"]
    PIN_EDGE_TRIGHI = 0x05,
    #[doc = "Timer disabled on trigger falling edge"]
    TRIG_FALLEDGE = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timdis {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timdis {
    #[inline(always)]
    fn from(val: u8) -> Timdis {
        Timdis::from_bits(val)
    }
}
impl From<Timdis> for u8 {
    #[inline(always)]
    fn from(val: Timdis) -> u8 {
        Timdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timena {
    #[doc = "Timer always enabled"]
    ENABLE = 0x0,
    #[doc = "Timer enabled on timer n-1 enable"]
    TMR_NMINUS1_EN = 0x01,
    #[doc = "Timer enabled on trigger high"]
    TMR_TRIGHI_EN = 0x02,
    #[doc = "Timer enabled on trigger high and pin high"]
    TMR_TRIG_PIN_HI_EN = 0x03,
    #[doc = "Timer enabled on pin rising edge"]
    TMR_PINRISE_EN = 0x04,
    #[doc = "Timer enabled on pin rising edge and trigger high"]
    TMR_PINRISE_TRIGHI_EN = 0x05,
    #[doc = "Timer enabled on trigger rising edge"]
    TMR_TRIGRISE_EN = 0x06,
    #[doc = "Timer enabled on trigger rising or falling edge"]
    TMR_TRIGEDGE_EN = 0x07,
}
impl Timena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timena {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timena {
    #[inline(always)]
    fn from(val: u8) -> Timena {
        Timena::from_bits(val)
    }
}
impl From<Timena> for u8 {
    #[inline(always)]
    fn from(val: Timena) -> u8 {
        Timena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timod {
    #[doc = "Timer disabled"]
    DISABLE = 0x0,
    #[doc = "Dual 8-bit counters baud mode"]
    DUAL8BIT_BAUD = 0x01,
    #[doc = "Dual 8-bit counters PWM high mode"]
    DUAL8BIT_PWM_H = 0x02,
    #[doc = "Single 16-bit counter mode"]
    SINGLE16BIT = 0x03,
    #[doc = "Single 16-bit counter disable mode"]
    SINGLE16BIT_DISABLE = 0x04,
    #[doc = "Dual 8-bit counters word mode"]
    DUAL8BIT_WORD = 0x05,
    #[doc = "Dual 8-bit counters PWM low mode"]
    DUAL8BIT_PWM_L = 0x06,
    #[doc = "Single 16-bit input capture mode"]
    SINGLE16BIT_IN_CAPTURE = 0x07,
}
impl Timod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timod {
    #[inline(always)]
    fn from(val: u8) -> Timod {
        Timod::from_bits(val)
    }
}
impl From<Timod> for u8 {
    #[inline(always)]
    fn from(val: Timod) -> u8 {
        Timod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timout {
    #[doc = "Logic one when enabled; not affected by timer reset"]
    ONE = 0x0,
    #[doc = "Logic zero when enabled; not affected by timer reset"]
    ZERO = 0x01,
    #[doc = "Logic one when enabled and on timer reset"]
    ONE_TMRRESET = 0x02,
    #[doc = "Logic zero when enabled and on timer reset"]
    ZERO_TMRRESET = 0x03,
}
impl Timout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timout {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timout {
    #[inline(always)]
    fn from(val: u8) -> Timout {
        Timout::from_bits(val)
    }
}
impl From<Timout> for u8 {
    #[inline(always)]
    fn from(val: Timout) -> u8 {
        Timout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timpol {
    #[doc = "Positive edge"]
    POSEDGE = 0x0,
    #[doc = "Negative edge"]
    NEGEDGE = 0x01,
}
impl Timpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timpol {
    #[inline(always)]
    fn from(val: u8) -> Timpol {
        Timpol::from_bits(val)
    }
}
impl From<Timpol> for u8 {
    #[inline(always)]
    fn from(val: Timpol) -> u8 {
        Timpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timrst {
    #[doc = "Never reset timer"]
    NEVER = 0x0,
    #[doc = "Timer reset on timer output high."]
    TMR_OUT_HI = 0x01,
    #[doc = "Timer reset on timer pin equal to timer output"]
    PIN_EQ_TMR_OUT = 0x02,
    #[doc = "Timer reset on timer trigger equal to timer output"]
    TRIG_EQ_TMR_OUT = 0x03,
    #[doc = "Timer reset on timer pin rising edge"]
    PIN_RISE_EDGE = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Timer reset on trigger rising edge"]
    TRIG_RISE_EDGE = 0x06,
    #[doc = "Timer reset on trigger rising or falling edge"]
    TRIG_EDGE = 0x07,
}
impl Timrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timrst {
    #[inline(always)]
    fn from(val: u8) -> Timrst {
        Timrst::from_bits(val)
    }
}
impl From<Timrst> for u8 {
    #[inline(always)]
    fn from(val: Timrst) -> u8 {
        Timrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgpol {
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
    ACTIVE_LOW = 0x01,
}
impl Trgpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgpol {
    #[inline(always)]
    fn from(val: u8) -> Trgpol {
        Trgpol::from_bits(val)
    }
}
impl From<Trgpol> for u8 {
    #[inline(always)]
    fn from(val: Trgpol) -> u8 {
        Trgpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsrc {
    #[doc = "External"]
    EXT_TRIG = 0x0,
    #[doc = "Internal"]
    INTERNAL_TRIG = 0x01,
}
impl Trgsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsrc {
    #[inline(always)]
    fn from(val: u8) -> Trgsrc {
        Trgsrc::from_bits(val)
    }
}
impl From<Trgsrc> for u8 {
    #[inline(always)]
    fn from(val: Trgsrc) -> u8 {
        Trgsrc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tsf(u8);
impl Tsf {
    #[doc = "Clear"]
    pub const CLR: Self = Self(0x0);
    #[doc = "Set"]
    pub const SET: Self = Self(0x01);
}
impl Tsf {
    pub const fn from_bits(val: u8) -> Tsf {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Tsf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLR"),
            0x01 => f.write_str("SET"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLR"),
            0x01 => defmt::write!(f, "SET"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
pub enum Tstop {
    #[doc = "Disabled"]
    STOP_DISABLE = 0x0,
    #[doc = "Enabled on timer compare"]
    ENABLE_TMRCMP = 0x01,
    #[doc = "Enabled on timer disable"]
    ENABLE_TMRDISABLE = 0x02,
    #[doc = "Enabled on timer compare and timer disable"]
    ENABLE_TMR_CMP_DIS = 0x03,
}
impl Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstop {
    #[inline(always)]
    fn from(val: u8) -> Tstop {
        Tstop::from_bits(val)
    }
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(val: Tstop) -> u8 {
        Tstop::to_bits(val)
    }
}
