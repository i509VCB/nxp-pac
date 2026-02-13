#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chmod {
    #[doc = "TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    TDM_MODE = 0x0,
    #[doc = "Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    OUTPUT_MODE = 0x01,
}
impl Chmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chmod {
    #[inline(always)]
    fn from(val: u8) -> Chmod {
        Chmod::from_bits(val)
    }
}
impl From<Chmod> for u8 {
    #[inline(always)]
    fn from(val: Chmod) -> u8 {
        Chmod::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set."]
    pub const STD: Self = Self(0x0);
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
            0x0 => f.write_str("STD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STD"),
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
pub enum Rcr2Bcd {
    #[doc = "Bit clock is generated externally in Slave mode."]
    EXT_SLAVE_MODE = 0x0,
    #[doc = "Bit clock is generated internally in Master mode."]
    INT_MASTER_MODE = 0x01,
}
impl Rcr2Bcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bcd {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bcd {
        Rcr2Bcd::from_bits(val)
    }
}
impl From<Rcr2Bcd> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bcd) -> u8 {
        Rcr2Bcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bci {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    CLOCKED_AS_IF_EXT_GENERATED = 0x01,
}
impl Rcr2Bci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bci {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bci {
        Rcr2Bci::from_bits(val)
    }
}
impl From<Rcr2Bci> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bci) -> u8 {
        Rcr2Bci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bcp {
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    ACTIVE_HIGH = 0x0,
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    ACTIVE_LOW = 0x01,
}
impl Rcr2Bcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bcp {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bcp {
        Rcr2Bcp::from_bits(val)
    }
}
impl From<Rcr2Bcp> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bcp) -> u8 {
        Rcr2Bcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bcs {
    #[doc = "Use the normal bit clock source."]
    NORMAL = 0x0,
    #[doc = "Swap the bit clock source."]
    SWAP_BIT_CLK_SOURCE = 0x01,
}
impl Rcr2Bcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Bcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Bcs {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Bcs {
        Rcr2Bcs::from_bits(val)
    }
}
impl From<Rcr2Bcs> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Bcs) -> u8 {
        Rcr2Bcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Msel {
    #[doc = "Bus Clock selected."]
    BUS_CLOCK = 0x0,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    MCLK1 = 0x01,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    MCLK2 = 0x02,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    MCLK3 = 0x03,
}
impl Rcr2Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Msel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Msel {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Msel {
        Rcr2Msel::from_bits(val)
    }
}
impl From<Rcr2Msel> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Msel) -> u8 {
        Rcr2Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Sync {
    #[doc = "Asynchronous mode."]
    ASYNC = 0x0,
    #[doc = "Synchronous with transmitter."]
    SYNC_W_TX = 0x01,
}
impl Rcr2Sync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Sync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Sync {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Sync {
        Rcr2Sync::from_bits(val)
    }
}
impl From<Rcr2Sync> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Sync) -> u8 {
        Rcr2Sync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fpack {
    #[doc = "FIFO packing is disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit FIFO packing is enabled"]
    EIGHT_BIT_PACKING = 0x02,
    #[doc = "16-bit FIFO packing is enabled"]
    SIXTEEN_BIT_PACKING = 0x03,
}
impl Rcr4Fpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fpack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fpack {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fpack {
        Rcr4Fpack::from_bits(val)
    }
}
impl From<Rcr4Fpack> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fpack) -> u8 {
        Rcr4Fpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fsd {
    #[doc = "Frame Sync is generated externally in Slave mode."]
    EXT_SLAVE_MODE = 0x0,
    #[doc = "Frame Sync is generated internally in Master mode."]
    INT_MASTER_MODE = 0x01,
}
impl Rcr4Fsd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fsd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fsd {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fsd {
        Rcr4Fsd::from_bits(val)
    }
}
impl From<Rcr4Fsd> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fsd) -> u8 {
        Rcr4Fsd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fsp {
    #[doc = "Frame sync is active high."]
    ACTIVE_HIGH = 0x0,
    #[doc = "Frame sync is active low."]
    ACTIVE_LOW = 0x01,
}
impl Rcr4Fsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fsp {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fsp {
        Rcr4Fsp::from_bits(val)
    }
}
impl From<Rcr4Fsp> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fsp) -> u8 {
        Rcr4Fsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFr {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "FIFO reset."]
    FIFO_RESET = 0x01,
}
impl RcsrFr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFr {
    #[inline(always)]
    fn from(val: u8) -> RcsrFr {
        RcsrFr::from_bits(val)
    }
}
impl From<RcsrFr> for u8 {
    #[inline(always)]
    fn from(val: RcsrFr) -> u8 {
        RcsrFr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFrf {
    #[doc = "Receive FIFO watermark not reached."]
    BELOW_WATERMARK = 0x0,
    #[doc = "Receive FIFO watermark has been reached."]
    WATERMARK_REACHED = 0x01,
}
impl RcsrFrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFrf {
    #[inline(always)]
    fn from(val: u8) -> RcsrFrf {
        RcsrFrf::from_bits(val)
    }
}
impl From<RcsrFrf> for u8 {
    #[inline(always)]
    fn from(val: RcsrFrf) -> u8 {
        RcsrFrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrSr {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Software reset."]
    SW_RESET = 0x01,
}
impl RcsrSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrSr {
    #[inline(always)]
    fn from(val: u8) -> RcsrSr {
        RcsrSr::from_bits(val)
    }
}
impl From<RcsrSr> for u8 {
    #[inline(always)]
    fn from(val: RcsrSr) -> u8 {
        RcsrSr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rwm(u32);
impl Rwm {
    #[doc = "Word N is enabled."]
    pub const WORD_N_ENABLED: Self = Self(0x0);
    #[doc = "Word N is masked."]
    pub const WORD_N_MASKED: Self = Self(0x01);
}
impl Rwm {
    pub const fn from_bits(val: u32) -> Rwm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rwm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WORD_N_ENABLED"),
            0x01 => f.write_str("WORD_N_MASKED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rwm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WORD_N_ENABLED"),
            0x01 => defmt::write!(f, "WORD_N_MASKED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rwm {
    #[inline(always)]
    fn from(val: u32) -> Rwm {
        Rwm::from_bits(val)
    }
}
impl From<Rwm> for u32 {
    #[inline(always)]
    fn from(val: Rwm) -> u32 {
        Rwm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Bcd {
    #[doc = "Bit clock is generated externally in Slave mode."]
    EXT_IN_SLAVE = 0x0,
    #[doc = "Bit clock is generated internally in Master mode."]
    INT_IN_MASTER = 0x01,
}
impl Tcr2Bcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Bcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Bcd {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Bcd {
        Tcr2Bcd::from_bits(val)
    }
}
impl From<Tcr2Bcd> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Bcd) -> u8 {
        Tcr2Bcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Bcp {
    #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    ACTIVE_HIGH = 0x0,
    #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    ACTIVE_LOW = 0x01,
}
impl Tcr2Bcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Bcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Bcp {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Bcp {
        Tcr2Bcp::from_bits(val)
    }
}
impl From<Tcr2Bcp> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Bcp) -> u8 {
        Tcr2Bcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Msel {
    #[doc = "Bus Clock selected."]
    BUS_CLOCK = 0x0,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    MCLK1 = 0x01,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    MCLK2 = 0x02,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    MCLK3 = 0x03,
}
impl Tcr2Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Msel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Msel {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Msel {
        Tcr2Msel::from_bits(val)
    }
}
impl From<Tcr2Msel> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Msel) -> u8 {
        Tcr2Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Sync {
    #[doc = "Asynchronous mode."]
    ASYNC = 0x0,
    #[doc = "Synchronous with receiver."]
    SYNC_W_RX = 0x01,
}
impl Tcr2Sync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Sync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Sync {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Sync {
        Tcr2Sync::from_bits(val)
    }
}
impl From<Tcr2Sync> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Sync) -> u8 {
        Tcr2Sync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fpack {
    #[doc = "FIFO packing is disabled."]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "8-bit FIFO packing is enabled."]
    EIGHT_BIT_FIFO_PACKING = 0x02,
    #[doc = "16-bit FIFO packing is enabled."]
    SIXTEEN_BIT_FIFO_PACKING = 0x03,
}
impl Tcr4Fpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fpack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fpack {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fpack {
        Tcr4Fpack::from_bits(val)
    }
}
impl From<Tcr4Fpack> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fpack) -> u8 {
        Tcr4Fpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fsd {
    #[doc = "Frame sync is generated externally in Slave mode."]
    EXT_IN_SLAVE_MODE = 0x0,
    #[doc = "Frame sync is generated internally in Master mode."]
    INT_IN_MASTER_MODE = 0x01,
}
impl Tcr4Fsd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fsd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fsd {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fsd {
        Tcr4Fsd::from_bits(val)
    }
}
impl From<Tcr4Fsd> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fsd) -> u8 {
        Tcr4Fsd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fsp {
    #[doc = "Frame sync is active high."]
    ACTIVE_HIGH = 0x0,
    #[doc = "Frame sync is active low."]
    ACTIVE_LOW = 0x01,
}
impl Tcr4Fsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fsp {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fsp {
        Tcr4Fsp::from_bits(val)
    }
}
impl From<Tcr4Fsp> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fsp) -> u8 {
        Tcr4Fsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Ondem {
    #[doc = "Internal frame sync is generated continuously."]
    CONTINUOUS_FRAME_SYNC = 0x0,
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    ON_DEMAND_FRAME_SYNC = 0x01,
}
impl Tcr4Ondem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Ondem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Ondem {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Ondem {
        Tcr4Ondem::from_bits(val)
    }
}
impl From<Tcr4Ondem> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Ondem) -> u8 {
        Tcr4Ondem::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFr {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "FIFO reset."]
    RESET = 0x01,
}
impl TcsrFr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFr {
    #[inline(always)]
    fn from(val: u8) -> TcsrFr {
        TcsrFr::from_bits(val)
    }
}
impl From<TcsrFr> for u8 {
    #[inline(always)]
    fn from(val: TcsrFr) -> u8 {
        TcsrFr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Twm(u32);
impl Twm {
    #[doc = "Word N is enabled."]
    pub const WORD_N_ENABLED: Self = Self(0x0);
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    pub const WORD_N_MASKED: Self = Self(0x01);
}
impl Twm {
    pub const fn from_bits(val: u32) -> Twm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Twm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WORD_N_ENABLED"),
            0x01 => f.write_str("WORD_N_MASKED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Twm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WORD_N_ENABLED"),
            0x01 => defmt::write!(f, "WORD_N_MASKED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Twm {
    #[inline(always)]
    fn from(val: u32) -> Twm {
        Twm::from_bits(val)
    }
}
impl From<Twm> for u32 {
    #[inline(always)]
    fn from(val: Twm) -> u32 {
        Twm::to_bits(val)
    }
}
