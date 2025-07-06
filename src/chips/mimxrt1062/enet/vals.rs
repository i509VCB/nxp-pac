#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addins {
    #[doc = "The source MAC address is not modified by the MAC."]
    ZERO = 0x0,
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    ONE = 0x01,
}
impl Addins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addins {
    #[inline(always)]
    fn from(val: u8) -> Addins {
        Addins::from_bits(val)
    }
}
impl From<Addins> for u8 {
    #[inline(always)]
    fn from(val: Addins) -> u8 {
        Addins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addsel {
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    VAL_MAC = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Addsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addsel {
    #[inline(always)]
    fn from(val: u8) -> Addsel {
        Addsel::from_bits(val)
    }
}
impl From<Addsel> for u8 {
    #[inline(always)]
    fn from(val: Addsel) -> u8 {
        Addsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Babr {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Babr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Babr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Babr {
    #[inline(always)]
    fn from(val: u8) -> Babr {
        Babr::from_bits(val)
    }
}
impl From<Babr> for u8 {
    #[inline(always)]
    fn from(val: Babr) -> u8 {
        Babr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Babt {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Babt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Babt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Babt {
    #[inline(always)]
    fn from(val: u8) -> Babt {
        Babt::from_bits(val)
    }
}
impl From<Babt> for u8 {
    #[inline(always)]
    fn from(val: Babt) -> u8 {
        Babt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BcRej {
    #[doc = "Will not reject frames as described above"]
    ZERO = 0x0,
    #[doc = "Will reject frames as described above"]
    ONE = 0x01,
}
impl BcRej {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BcRej {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BcRej {
    #[inline(always)]
    fn from(val: u8) -> BcRej {
        BcRej::from_bits(val)
    }
}
impl From<BcRej> for u8 {
    #[inline(always)]
    fn from(val: BcRej) -> u8 {
        BcRej::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capture {
    #[doc = "No effect."]
    ZERO = 0x0,
    #[doc = "The current time is captured and can be read from the ATVR register."]
    ONE = 0x01,
}
impl Capture {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capture {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capture {
    #[inline(always)]
    fn from(val: u8) -> Capture {
        Capture::from_bits(val)
    }
}
impl From<Capture> for u8 {
    #[inline(always)]
    fn from(val: Capture) -> u8 {
        Capture::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfen {
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    ZERO = 0x0,
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    ONE = 0x01,
}
impl Cfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfen {
    #[inline(always)]
    fn from(val: u8) -> Cfen {
        Cfen::from_bits(val)
    }
}
impl From<Cfen> for u8 {
    #[inline(always)]
    fn from(val: Cfen) -> u8 {
        Cfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgen {
    #[doc = "MAC continues operation in debug mode."]
    ZERO = 0x0,
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    ONE = 0x01,
}
impl Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgen {
    #[inline(always)]
    fn from(val: u8) -> Dbgen {
        Dbgen::from_bits(val)
    }
}
impl From<Dbgen> for u8 {
    #[inline(always)]
    fn from(val: Dbgen) -> u8 {
        Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbswp {
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    ZERO = 0x0,
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    ONE = 0x01,
}
impl Dbswp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbswp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbswp {
    #[inline(always)]
    fn from(val: u8) -> Dbswp {
        Dbswp::from_bits(val)
    }
}
impl From<Dbswp> for u8 {
    #[inline(always)]
    fn from(val: Dbswp) -> u8 {
        Dbswp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPre {
    #[doc = "Preamble enabled."]
    ZERO = 0x0,
    #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
    ONE = 0x01,
}
impl DisPre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPre {
    #[inline(always)]
    fn from(val: u8) -> DisPre {
        DisPre::from_bits(val)
    }
}
impl From<DisPre> for u8 {
    #[inline(always)]
    fn from(val: DisPre) -> u8 {
        DisPre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drt {
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    ZERO = 0x0,
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    ONE = 0x01,
}
impl Drt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drt {
    #[inline(always)]
    fn from(val: u8) -> Drt {
        Drt::from_bits(val)
    }
}
impl From<Drt> for u8 {
    #[inline(always)]
    fn from(val: Drt) -> u8 {
        Drt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eberr {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Eberr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eberr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eberr {
    #[inline(always)]
    fn from(val: u8) -> Eberr {
        Eberr::from_bits(val)
    }
}
impl From<Eberr> for u8 {
    #[inline(always)]
    fn from(val: Eberr) -> u8 {
        Eberr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "The timer stops at the current value."]
    ZERO = 0x0,
    #[doc = "The timer starts incrementing."]
    ONE = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En1588 {
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    ZERO = 0x0,
    #[doc = "Enhanced frame time-stamping functions enabled. Has no effect within the MAC besides controlling the DMA control bit ena_1588."]
    ONE = 0x01,
}
impl En1588 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En1588 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En1588 {
    #[inline(always)]
    fn from(val: u8) -> En1588 {
        En1588::from_bits(val)
    }
}
impl From<En1588> for u8 {
    #[inline(always)]
    fn from(val: En1588) -> u8 {
        En1588::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etheren {
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    ZERO = 0x0,
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    ONE = 0x01,
}
impl Etheren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etheren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etheren {
    #[inline(always)]
    fn from(val: u8) -> Etheren {
        Etheren::from_bits(val)
    }
}
impl From<Etheren> for u8 {
    #[inline(always)]
    fn from(val: Etheren) -> u8 {
        Etheren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fce {
    #[doc = "Disable flow control"]
    ZERO = 0x0,
    #[doc = "Enable flow control"]
    ONE = 0x01,
}
impl Fce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fce {
    #[inline(always)]
    fn from(val: u8) -> Fce {
        Fce::from_bits(val)
    }
}
impl From<Fce> for u8 {
    #[inline(always)]
    fn from(val: Fce) -> u8 {
        Fce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fden {
    #[doc = "Disable full-duplex"]
    ZERO = 0x0,
    #[doc = "Enable full-duplex"]
    ONE = 0x01,
}
impl Fden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fden {
    #[inline(always)]
    fn from(val: u8) -> Fden {
        Fden::from_bits(val)
    }
}
impl From<Fden> for u8 {
    #[inline(always)]
    fn from(val: Fden) -> u8 {
        Fden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gra {
    #[doc = "The corresponding interrupt source is masked."]
    MASKED = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    UMASKED = 0x01,
}
impl Gra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gra {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gra {
    #[inline(always)]
    fn from(val: u8) -> Gra {
        Gra::from_bits(val)
    }
}
impl From<Gra> for u8 {
    #[inline(always)]
    fn from(val: Gra) -> u8 {
        Gra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grs {
    #[doc = "Receive not stopped"]
    ZERO = 0x0,
    #[doc = "Receive stopped"]
    ONE = 0x01,
}
impl Grs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grs {
    #[inline(always)]
    fn from(val: u8) -> Grs {
        Grs::from_bits(val)
    }
}
impl From<Grs> for u8 {
    #[inline(always)]
    fn from(val: Grs) -> u8 {
        Grs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gts {
    #[doc = "Disable graceful transmit stop"]
    ZERO = 0x0,
    #[doc = "Enable graceful transmit stop"]
    ONE = 0x01,
}
impl Gts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gts {
    #[inline(always)]
    fn from(val: u8) -> Gts {
        Gts::from_bits(val)
    }
}
impl From<Gts> for u8 {
    #[inline(always)]
    fn from(val: Gts) -> u8 {
        Gts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Holdtime {
    #[doc = "1 internal module clock cycle"]
    VAL_1 = 0x0,
    #[doc = "2 internal module clock cycles"]
    VAL2 = 0x01,
    #[doc = "3 internal module clock cycles"]
    VAL3 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8 internal module clock cycles"]
    VAL8 = 0x07,
}
impl Holdtime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Holdtime {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Holdtime {
    #[inline(always)]
    fn from(val: u8) -> Holdtime {
        Holdtime::from_bits(val)
    }
}
impl From<Holdtime> for u8 {
    #[inline(always)]
    fn from(val: Holdtime) -> u8 {
        Holdtime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipchk {
    #[doc = "Checksum is not inserted."]
    ZERO = 0x0,
    #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    ONE = 0x01,
}
impl Ipchk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipchk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipchk {
    #[inline(always)]
    fn from(val: u8) -> Ipchk {
        Ipchk::from_bits(val)
    }
}
impl From<Ipchk> for u8 {
    #[inline(always)]
    fn from(val: Ipchk) -> u8 {
        Ipchk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipdis {
    #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
    ZERO = 0x0,
    #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    ONE = 0x01,
}
impl Ipdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipdis {
    #[inline(always)]
    fn from(val: u8) -> Ipdis {
        Ipdis::from_bits(val)
    }
}
impl From<Ipdis> for u8 {
    #[inline(always)]
    fn from(val: Ipdis) -> u8 {
        Ipdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lc {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Lc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lc {
    #[inline(always)]
    fn from(val: u8) -> Lc {
        Lc::from_bits(val)
    }
}
impl From<Lc> for u8 {
    #[inline(always)]
    fn from(val: Lc) -> u8 {
        Lc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linedis {
    #[doc = "Frames with errors are not discarded."]
    ZERO = 0x0,
    #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    ONE = 0x01,
}
impl Linedis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linedis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linedis {
    #[inline(always)]
    fn from(val: u8) -> Linedis {
        Linedis::from_bits(val)
    }
}
impl From<Linedis> for u8 {
    #[inline(always)]
    fn from(val: Linedis) -> u8 {
        Linedis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loop {
    #[doc = "Loopback disabled."]
    ZERO = 0x0,
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    ONE = 0x01,
}
impl Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loop {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub enum Magicen {
    #[doc = "Magic detection logic disabled."]
    ZERO = 0x0,
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
    ONE = 0x01,
}
impl Magicen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Magicen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Magicen {
    #[inline(always)]
    fn from(val: u8) -> Magicen {
        Magicen::from_bits(val)
    }
}
impl From<Magicen> for u8 {
    #[inline(always)]
    fn from(val: Magicen) -> u8 {
        Magicen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MibClear {
    #[doc = "See note above."]
    ZERO = 0x0,
    #[doc = "All statistics counters are reset to 0."]
    ONE = 0x01,
}
impl MibClear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MibClear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MibClear {
    #[inline(always)]
    fn from(val: u8) -> MibClear {
        MibClear::from_bits(val)
    }
}
impl From<MibClear> for u8 {
    #[inline(always)]
    fn from(val: MibClear) -> u8 {
        MibClear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MibDis {
    #[doc = "MIB logic is enabled."]
    ZERO = 0x0,
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    ONE = 0x01,
}
impl MibDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MibDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MibDis {
    #[inline(always)]
    fn from(val: u8) -> MibDis {
        MibDis::from_bits(val)
    }
}
impl From<MibDis> for u8 {
    #[inline(always)]
    fn from(val: MibDis) -> u8 {
        MibDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MibIdle {
    #[doc = "The MIB block is updating MIB counters."]
    ZERO = 0x0,
    #[doc = "The MIB block is not currently updating any MIB counters."]
    ONE = 0x01,
}
impl MibIdle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MibIdle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MibIdle {
    #[inline(always)]
    fn from(val: u8) -> MibIdle {
        MibIdle::from_bits(val)
    }
}
impl From<MibIdle> for u8 {
    #[inline(always)]
    fn from(val: MibIdle) -> u8 {
        MibIdle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mii {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Mii {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mii {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mii {
    #[inline(always)]
    fn from(val: u8) -> Mii {
        Mii::from_bits(val)
    }
}
impl From<Mii> for u8 {
    #[inline(always)]
    fn from(val: Mii) -> u8 {
        Mii::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nlc {
    #[doc = "The payload length check is disabled."]
    ZERO = 0x0,
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\] field."]
    ONE = 0x01,
}
impl Nlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nlc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nlc {
    #[inline(always)]
    fn from(val: u8) -> Nlc {
        Nlc::from_bits(val)
    }
}
impl From<Nlc> for u8 {
    #[inline(always)]
    fn from(val: Nlc) -> u8 {
        Nlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Offen {
    #[doc = "Disable."]
    ZERO = 0x0,
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    ONE = 0x01,
}
impl Offen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Offen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Offen {
    #[inline(always)]
    fn from(val: u8) -> Offen {
        Offen::from_bits(val)
    }
}
impl From<Offen> for u8 {
    #[inline(always)]
    fn from(val: Offen) -> u8 {
        Offen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Offrst {
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    ZERO = 0x0,
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    ONE = 0x01,
}
impl Offrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Offrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Offrst {
    #[inline(always)]
    fn from(val: u8) -> Offrst {
        Offrst::from_bits(val)
    }
}
impl From<Offrst> for u8 {
    #[inline(always)]
    fn from(val: Offrst) -> u8 {
        Offrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paden {
    #[doc = "No padding is removed on receive by the MAC."]
    ZERO = 0x0,
    #[doc = "Padding is removed from received frames."]
    ONE = 0x01,
}
impl Paden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paden {
    #[inline(always)]
    fn from(val: u8) -> Paden {
        Paden::from_bits(val)
    }
}
impl From<Paden> for u8 {
    #[inline(always)]
    fn from(val: Paden) -> u8 {
        Paden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Padrem {
    #[doc = "Padding not removed."]
    ZERO = 0x0,
    #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
    ONE = 0x01,
}
impl Padrem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Padrem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Padrem {
    #[inline(always)]
    fn from(val: u8) -> Padrem {
        Padrem::from_bits(val)
    }
}
impl From<Padrem> for u8 {
    #[inline(always)]
    fn from(val: Padrem) -> u8 {
        Padrem::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paufwd {
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    ZERO = 0x0,
    #[doc = "Pause frames are forwarded to the user application."]
    ONE = 0x01,
}
impl Paufwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paufwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paufwd {
    #[inline(always)]
    fn from(val: u8) -> Paufwd {
        Paufwd::from_bits(val)
    }
}
impl From<Paufwd> for u8 {
    #[inline(always)]
    fn from(val: Paufwd) -> u8 {
        Paufwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Peren {
    #[doc = "Disable."]
    ZERO = 0x0,
    #[doc = "A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    ONE = 0x01,
}
impl Peren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Peren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Peren {
    #[inline(always)]
    fn from(val: u8) -> Peren {
        Peren::from_bits(val)
    }
}
impl From<Peren> for u8 {
    #[inline(always)]
    fn from(val: Peren) -> u8 {
        Peren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pinper {
    #[doc = "Disable."]
    ZERO = 0x0,
    #[doc = "Enable."]
    ONE = 0x01,
}
impl Pinper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pinper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pinper {
    #[inline(always)]
    fn from(val: u8) -> Pinper {
        Pinper::from_bits(val)
    }
}
impl From<Pinper> for u8 {
    #[inline(always)]
    fn from(val: Pinper) -> u8 {
        Pinper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plr {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Plr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plr {
    #[inline(always)]
    fn from(val: u8) -> Plr {
        Plr::from_bits(val)
    }
}
impl From<Plr> for u8 {
    #[inline(always)]
    fn from(val: Plr) -> u8 {
        Plr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prochk {
    #[doc = "Checksum not inserted."]
    ZERO = 0x0,
    #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    ONE = 0x01,
}
impl Prochk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prochk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prochk {
    #[inline(always)]
    fn from(val: u8) -> Prochk {
        Prochk::from_bits(val)
    }
}
impl From<Prochk> for u8 {
    #[inline(always)]
    fn from(val: Prochk) -> u8 {
        Prochk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prodis {
    #[doc = "Frames with wrong checksum are not discarded."]
    ZERO = 0x0,
    #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    ONE = 0x01,
}
impl Prodis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prodis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prodis {
    #[inline(always)]
    fn from(val: u8) -> Prodis {
        Prodis::from_bits(val)
    }
}
impl From<Prodis> for u8 {
    #[inline(always)]
    fn from(val: Prodis) -> u8 {
        Prodis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prom {
    #[doc = "Disabled."]
    ZERO = 0x0,
    #[doc = "Enabled."]
    ONE = 0x01,
}
impl Prom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prom {
    #[inline(always)]
    fn from(val: u8) -> Prom {
        Prom::from_bits(val)
    }
}
impl From<Prom> for u8 {
    #[inline(always)]
    fn from(val: Prom) -> u8 {
        Prom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RaccShift16 {
    #[doc = "Disabled."]
    ZERO = 0x0,
    #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    ONE = 0x01,
}
impl RaccShift16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RaccShift16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RaccShift16 {
    #[inline(always)]
    fn from(val: u8) -> RaccShift16 {
        RaccShift16::from_bits(val)
    }
}
impl From<RaccShift16> for u8 {
    #[inline(always)]
    fn from(val: RaccShift16) -> u8 {
        RaccShift16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcrCrcfwd {
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    ZERO = 0x0,
    #[doc = "The CRC field is stripped from the frame."]
    ONE = 0x01,
}
impl RcrCrcfwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcrCrcfwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcrCrcfwd {
    #[inline(always)]
    fn from(val: u8) -> RcrCrcfwd {
        RcrCrcfwd::from_bits(val)
    }
}
impl From<RcrCrcfwd> for u8 {
    #[inline(always)]
    fn from(val: RcrCrcfwd) -> u8 {
        RcrCrcfwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rl {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rl {
    #[inline(always)]
    fn from(val: u8) -> Rl {
        Rl::from_bits(val)
    }
}
impl From<Rl> for u8 {
    #[inline(always)]
    fn from(val: Rl) -> u8 {
        Rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rmii10t {
    #[doc = "100-Mbit/s operation."]
    ZERO = 0x0,
    #[doc = "10-Mbit/s operation."]
    ONE = 0x01,
}
impl Rmii10t {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rmii10t {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rmii10t {
    #[inline(always)]
    fn from(val: u8) -> Rmii10t {
        Rmii10t::from_bits(val)
    }
}
impl From<Rmii10t> for u8 {
    #[inline(always)]
    fn from(val: Rmii10t) -> u8 {
        Rmii10t::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiMode {
    #[doc = "MAC configured for MII mode."]
    ZERO = 0x0,
    #[doc = "MAC configured for RMII operation."]
    ONE = 0x01,
}
impl RmiiMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiMode {
    #[inline(always)]
    fn from(val: u8) -> RmiiMode {
        RmiiMode::from_bits(val)
    }
}
impl From<RmiiMode> for u8 {
    #[inline(always)]
    fn from(val: RmiiMode) -> u8 {
        RmiiMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxb {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Rxb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxb {
    #[inline(always)]
    fn from(val: u8) -> Rxb {
        Rxb::from_bits(val)
    }
}
impl From<Rxb> for u8 {
    #[inline(always)]
    fn from(val: Rxb) -> u8 {
        Rxb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxf {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Rxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxf {
    #[inline(always)]
    fn from(val: u8) -> Rxf {
        Rxf::from_bits(val)
    }
}
impl From<Rxf> for u8 {
    #[inline(always)]
    fn from(val: Rxf) -> u8 {
        Rxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxic0Iccs {
    #[doc = "Use MII/GMII TX clocks."]
    ZERO = 0x0,
    #[doc = "Use ENET system clock."]
    ONE = 0x01,
}
impl Rxic0Iccs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxic0Iccs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxic0Iccs {
    #[inline(always)]
    fn from(val: u8) -> Rxic0Iccs {
        Rxic0Iccs::from_bits(val)
    }
}
impl From<Rxic0Iccs> for u8 {
    #[inline(always)]
    fn from(val: Rxic0Iccs) -> u8 {
        Rxic0Iccs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxic0Icen {
    #[doc = "Disable Interrupt coalescing."]
    ZERO = 0x0,
    #[doc = "Enable Interrupt coalescing."]
    ONE = 0x01,
}
impl Rxic0Icen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxic0Icen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxic0Icen {
    #[inline(always)]
    fn from(val: u8) -> Rxic0Icen {
        Rxic0Icen::from_bits(val)
    }
}
impl From<Rxic0Icen> for u8 {
    #[inline(always)]
    fn from(val: Rxic0Icen) -> u8 {
        Rxic0Icen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slave {
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    ZERO = 0x0,
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    ONE = 0x01,
}
impl Slave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slave {
    #[inline(always)]
    fn from(val: u8) -> Slave {
        Slave::from_bits(val)
    }
}
impl From<Slave> for u8 {
    #[inline(always)]
    fn from(val: Slave) -> u8 {
        Slave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleep {
    #[doc = "Normal operating mode."]
    ZERO = 0x0,
    #[doc = "Sleep mode."]
    ONE = 0x01,
}
impl Sleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleep {
    #[inline(always)]
    fn from(val: u8) -> Sleep {
        Sleep::from_bits(val)
    }
}
impl From<Sleep> for u8 {
    #[inline(always)]
    fn from(val: Sleep) -> u8 {
        Sleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Strfwd {
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    ZERO = 0x0,
    #[doc = "Enabled."]
    ONE = 0x01,
}
impl Strfwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Strfwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Strfwd {
    #[inline(always)]
    fn from(val: u8) -> Strfwd {
        Strfwd::from_bits(val)
    }
}
impl From<Strfwd> for u8 {
    #[inline(always)]
    fn from(val: Strfwd) -> u8 {
        Strfwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TaccShift16 {
    #[doc = "Disabled."]
    ZERO = 0x0,
    #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    ONE = 0x01,
}
impl TaccShift16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TaccShift16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TaccShift16 {
    #[inline(always)]
    fn from(val: u8) -> TaccShift16 {
        TaccShift16::from_bits(val)
    }
}
impl From<TaccShift16> for u8 {
    #[inline(always)]
    fn from(val: TaccShift16) -> u8 {
        TaccShift16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcrCrcfwd {
    #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
    ZERO = 0x0,
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    ONE = 0x01,
}
impl TcrCrcfwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcrCrcfwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcrCrcfwd {
    #[inline(always)]
    fn from(val: u8) -> TcrCrcfwd {
        TcrCrcfwd::from_bits(val)
    }
}
impl From<TcrCrcfwd> for u8 {
    #[inline(always)]
    fn from(val: TcrCrcfwd) -> u8 {
        TcrCrcfwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tdre {
    #[doc = "DMA request is disabled"]
    ZERO = 0x0,
    #[doc = "DMA request is enabled"]
    ONE = 0x01,
}
impl Tcsr0Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tdre {
        Tcsr0Tdre::from_bits(val)
    }
}
impl From<Tcsr0Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tdre) -> u8 {
        Tcsr0Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tf {
    #[doc = "Input Capture or Output Compare has not occurred."]
    ZERO = 0x0,
    #[doc = "Input Capture or Output Compare has occurred."]
    ONE = 0x01,
}
impl Tcsr0Tf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tf {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tf {
        Tcsr0Tf::from_bits(val)
    }
}
impl From<Tcsr0Tf> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tf) -> u8 {
        Tcsr0Tf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tie {
    #[doc = "Interrupt is disabled"]
    ZERO = 0x0,
    #[doc = "Interrupt is enabled"]
    ONE = 0x01,
}
impl Tcsr0Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tie {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tie {
        Tcsr0Tie::from_bits(val)
    }
}
impl From<Tcsr0Tie> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tie) -> u8 {
        Tcsr0Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr0Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tmode {
        Tcsr0Tmode::from_bits(val)
    }
}
impl From<Tcsr0Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tmode) -> u8 {
        Tcsr0Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr0Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tpwc {
        Tcsr0Tpwc::from_bits(val)
    }
}
impl From<Tcsr0Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tpwc) -> u8 {
        Tcsr0Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tdre {
    #[doc = "DMA request is disabled"]
    ZERO = 0x0,
    #[doc = "DMA request is enabled"]
    ONE = 0x01,
}
impl Tcsr1Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tdre {
        Tcsr1Tdre::from_bits(val)
    }
}
impl From<Tcsr1Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tdre) -> u8 {
        Tcsr1Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tf {
    #[doc = "Input Capture or Output Compare has not occurred."]
    ZERO = 0x0,
    #[doc = "Input Capture or Output Compare has occurred."]
    ONE = 0x01,
}
impl Tcsr1Tf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tf {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tf {
        Tcsr1Tf::from_bits(val)
    }
}
impl From<Tcsr1Tf> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tf) -> u8 {
        Tcsr1Tf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tie {
    #[doc = "Interrupt is disabled"]
    ZERO = 0x0,
    #[doc = "Interrupt is enabled"]
    ONE = 0x01,
}
impl Tcsr1Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tie {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tie {
        Tcsr1Tie::from_bits(val)
    }
}
impl From<Tcsr1Tie> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tie) -> u8 {
        Tcsr1Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr1Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tmode {
        Tcsr1Tmode::from_bits(val)
    }
}
impl From<Tcsr1Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tmode) -> u8 {
        Tcsr1Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr1Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tpwc {
        Tcsr1Tpwc::from_bits(val)
    }
}
impl From<Tcsr1Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tpwc) -> u8 {
        Tcsr1Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tdre {
    #[doc = "DMA request is disabled"]
    ZERO = 0x0,
    #[doc = "DMA request is enabled"]
    ONE = 0x01,
}
impl Tcsr2Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tdre {
        Tcsr2Tdre::from_bits(val)
    }
}
impl From<Tcsr2Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tdre) -> u8 {
        Tcsr2Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tf {
    #[doc = "Input Capture or Output Compare has not occurred."]
    ZERO = 0x0,
    #[doc = "Input Capture or Output Compare has occurred."]
    ONE = 0x01,
}
impl Tcsr2Tf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tf {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tf {
        Tcsr2Tf::from_bits(val)
    }
}
impl From<Tcsr2Tf> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tf) -> u8 {
        Tcsr2Tf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tie {
    #[doc = "Interrupt is disabled"]
    ZERO = 0x0,
    #[doc = "Interrupt is enabled"]
    ONE = 0x01,
}
impl Tcsr2Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tie {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tie {
        Tcsr2Tie::from_bits(val)
    }
}
impl From<Tcsr2Tie> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tie) -> u8 {
        Tcsr2Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr2Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tmode {
        Tcsr2Tmode::from_bits(val)
    }
}
impl From<Tcsr2Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tmode) -> u8 {
        Tcsr2Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr2Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tpwc {
        Tcsr2Tpwc::from_bits(val)
    }
}
impl From<Tcsr2Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tpwc) -> u8 {
        Tcsr2Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tdre {
    #[doc = "DMA request is disabled"]
    ZERO = 0x0,
    #[doc = "DMA request is enabled"]
    ONE = 0x01,
}
impl Tcsr3Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tdre {
        Tcsr3Tdre::from_bits(val)
    }
}
impl From<Tcsr3Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tdre) -> u8 {
        Tcsr3Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tf {
    #[doc = "Input Capture or Output Compare has not occurred."]
    ZERO = 0x0,
    #[doc = "Input Capture or Output Compare has occurred."]
    ONE = 0x01,
}
impl Tcsr3Tf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tf {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tf {
        Tcsr3Tf::from_bits(val)
    }
}
impl From<Tcsr3Tf> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tf) -> u8 {
        Tcsr3Tf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tie {
    #[doc = "Interrupt is disabled"]
    ZERO = 0x0,
    #[doc = "Interrupt is enabled"]
    ONE = 0x01,
}
impl Tcsr3Tie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tie {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tie {
        Tcsr3Tie::from_bits(val)
    }
}
impl From<Tcsr3Tie> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tie) -> u8 {
        Tcsr3Tie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr3Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tmode {
        Tcsr3Tmode::from_bits(val)
    }
}
impl From<Tcsr3Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tmode) -> u8 {
        Tcsr3Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr3Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tpwc {
        Tcsr3Tpwc::from_bits(val)
    }
}
impl From<Tcsr3Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tpwc) -> u8 {
        Tcsr3Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tf0 {
    #[doc = "Timer Flag for Channel 0 is clear"]
    ZERO = 0x0,
    #[doc = "Timer Flag for Channel 0 is set"]
    ONE = 0x01,
}
impl Tf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tf0 {
    #[inline(always)]
    fn from(val: u8) -> Tf0 {
        Tf0::from_bits(val)
    }
}
impl From<Tf0> for u8 {
    #[inline(always)]
    fn from(val: Tf0) -> u8 {
        Tf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tf1 {
    #[doc = "Timer Flag for Channel 1 is clear"]
    ZERO = 0x0,
    #[doc = "Timer Flag for Channel 1 is set"]
    ONE = 0x01,
}
impl Tf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tf1 {
    #[inline(always)]
    fn from(val: u8) -> Tf1 {
        Tf1::from_bits(val)
    }
}
impl From<Tf1> for u8 {
    #[inline(always)]
    fn from(val: Tf1) -> u8 {
        Tf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tf2 {
    #[doc = "Timer Flag for Channel 2 is clear"]
    ZERO = 0x0,
    #[doc = "Timer Flag for Channel 2 is set"]
    ONE = 0x01,
}
impl Tf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tf2 {
    #[inline(always)]
    fn from(val: u8) -> Tf2 {
        Tf2::from_bits(val)
    }
}
impl From<Tf2> for u8 {
    #[inline(always)]
    fn from(val: Tf2) -> u8 {
        Tf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tf3 {
    #[doc = "Timer Flag for Channel 3 is clear"]
    ZERO = 0x0,
    #[doc = "Timer Flag for Channel 3 is set"]
    ONE = 0x01,
}
impl Tf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tf3 {
    #[inline(always)]
    fn from(val: u8) -> Tf3 {
        Tf3::from_bits(val)
    }
}
impl From<Tf3> for u8 {
    #[inline(always)]
    fn from(val: Tf3) -> u8 {
        Tf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TfcPause {
    #[doc = "No PAUSE frame transmitted."]
    ZERO = 0x0,
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    ONE = 0x01,
}
impl TfcPause {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TfcPause {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TfcPause {
    #[inline(always)]
    fn from(val: u8) -> TfcPause {
        TfcPause::from_bits(val)
    }
}
impl From<TfcPause> for u8 {
    #[inline(always)]
    fn from(val: TfcPause) -> u8 {
        TfcPause::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfwr {
    #[doc = "64 bytes written."]
    VAL64_0 = 0x0,
    #[doc = "64 bytes written."]
    VAL64_1 = 0x01,
    #[doc = "128 bytes written."]
    VAL128 = 0x02,
    #[doc = "192 bytes written."]
    VAL192 = 0x03,
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
    #[doc = "1984 bytes written."]
    VAL1984 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Tfwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfwr {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfwr {
    #[inline(always)]
    fn from(val: u8) -> Tfwr {
        Tfwr::from_bits(val)
    }
}
impl From<Tfwr> for u8 {
    #[inline(always)]
    fn from(val: Tfwr) -> u8 {
        Tfwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsAvail {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl TsAvail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsAvail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsAvail {
    #[inline(always)]
    fn from(val: u8) -> TsAvail {
        TsAvail::from_bits(val)
    }
}
impl From<TsAvail> for u8 {
    #[inline(always)]
    fn from(val: TsAvail) -> u8 {
        TsAvail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsTimer {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl TsTimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsTimer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsTimer {
    #[inline(always)]
    fn from(val: u8) -> TsTimer {
        TsTimer::from_bits(val)
    }
}
impl From<TsTimer> for u8 {
    #[inline(always)]
    fn from(val: TsTimer) -> u8 {
        TsTimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txb {
    #[doc = "The corresponding interrupt source is masked."]
    MASKED = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    UNMASKED = 0x01,
}
impl Txb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txb {
    #[inline(always)]
    fn from(val: u8) -> Txb {
        Txb::from_bits(val)
    }
}
impl From<Txb> for u8 {
    #[inline(always)]
    fn from(val: Txb) -> u8 {
        Txb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txf {
    #[doc = "The corresponding interrupt source is masked."]
    MASKED = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    UNMASKED = 0x01,
}
impl Txf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txf {
    #[inline(always)]
    fn from(val: u8) -> Txf {
        Txf::from_bits(val)
    }
}
impl From<Txf> for u8 {
    #[inline(always)]
    fn from(val: Txf) -> u8 {
        Txf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txic0Iccs {
    #[doc = "Use MII/GMII TX clocks."]
    ZERO = 0x0,
    #[doc = "Use ENET system clock."]
    ONE = 0x01,
}
impl Txic0Iccs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txic0Iccs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txic0Iccs {
    #[inline(always)]
    fn from(val: u8) -> Txic0Iccs {
        Txic0Iccs::from_bits(val)
    }
}
impl From<Txic0Iccs> for u8 {
    #[inline(always)]
    fn from(val: Txic0Iccs) -> u8 {
        Txic0Iccs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txic0Icen {
    #[doc = "Disable Interrupt coalescing."]
    ZERO = 0x0,
    #[doc = "Enable Interrupt coalescing."]
    ONE = 0x01,
}
impl Txic0Icen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txic0Icen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txic0Icen {
    #[inline(always)]
    fn from(val: u8) -> Txic0Icen {
        Txic0Icen::from_bits(val)
    }
}
impl From<Txic0Icen> for u8 {
    #[inline(always)]
    fn from(val: Txic0Icen) -> u8 {
        Txic0Icen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Un {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Un {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Un {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Un {
    #[inline(always)]
    fn from(val: u8) -> Un {
        Un::from_bits(val)
    }
}
impl From<Un> for u8 {
    #[inline(always)]
    fn from(val: Un) -> u8 {
        Un::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakeup {
    #[doc = "The corresponding interrupt source is masked."]
    ZERO = 0x0,
    #[doc = "The corresponding interrupt source is not masked."]
    ONE = 0x01,
}
impl Wakeup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeup {
    #[inline(always)]
    fn from(val: u8) -> Wakeup {
        Wakeup::from_bits(val)
    }
}
impl From<Wakeup> for u8 {
    #[inline(always)]
    fn from(val: Wakeup) -> u8 {
        Wakeup::to_bits(val)
    }
}
