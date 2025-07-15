#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chmod {
    #[doc = "TDM mode"]
    TDM_MODE = 0x0,
    #[doc = "Output mode"]
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
    #[doc = "Standard feature set"]
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
pub enum Frsz {
    #[doc = "1"]
    ONE_WORD = 0x0,
    #[doc = "2"]
    TWO_WORDS = 0x01,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_2 = 0x02,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_3 = 0x03,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_4 = 0x04,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_5 = 0x05,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_6 = 0x06,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_7 = 0x07,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_8 = 0x08,
    #[doc = "(FRSZ value + 1)"]
    N_WORDS_9 = 0x09,
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
    #[doc = "32"]
    MAX_WORDS = 0x1f,
}
impl Frsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frsz {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frsz {
    #[inline(always)]
    fn from(val: u8) -> Frsz {
        Frsz::from_bits(val)
    }
}
impl From<Frsz> for u8 {
    #[inline(always)]
    fn from(val: Frsz) -> u8 {
        Frsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrMsel {
    #[doc = "Controller clock (MCLK) option 1"]
    MCLK1 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Controller clock (MCLK) option 2"]
    MCLK2 = 0x02,
    #[doc = "Controller clock (MCLK) option 3"]
    MCLK3 = 0x03,
}
impl McrMsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrMsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrMsel {
    #[inline(always)]
    fn from(val: u8) -> McrMsel {
        McrMsel::from_bits(val)
    }
}
impl From<McrMsel> for u8 {
    #[inline(always)]
    fn from(val: McrMsel) -> u8 {
        McrMsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Moe {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Moe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Moe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Moe {
    #[inline(always)]
    fn from(val: u8) -> Moe {
        Moe::from_bits(val)
    }
}
impl From<Moe> for u8 {
    #[inline(always)]
    fn from(val: Moe) -> u8 {
        Moe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr2Bcd {
    #[doc = "Generated externally in Target mode"]
    EXT_TARGET_MODE = 0x0,
    #[doc = "Generated internally in Controller mode"]
    INT_CONTROLLER_MODE = 0x01,
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
    #[doc = "Disable"]
    NO_EFFECT = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
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
    #[doc = "Use the normal bit clock source"]
    NORMAL = 0x0,
    #[doc = "Swap the bit clock source"]
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
    #[doc = "Bus clock"]
    BUS_CLOCK = 0x0,
    #[doc = "Controller clock (MCLK) option 1"]
    MCLK1 = 0x01,
    #[doc = "Controller clock (MCLK) option 2"]
    MCLK2 = 0x02,
    #[doc = "Controller clock (MCLK) option 3"]
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
    #[doc = "Asynchronous mode"]
    ASYNC = 0x0,
    #[doc = "Synchronous with transmitter"]
    SYNC_W_TX = 0x01,
    #[doc = "Synchronous with another SAI receiver"]
    SYNC_W_ANOTHER_SAI_RX = 0x02,
    #[doc = "Synchronous with another SAI transmitter"]
    SYNC_W_ANOTHER_SAI_TX = 0x03,
}
impl Rcr2Sync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Sync {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Rcr4Fcomb {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable on FIFO writes (from receive shift registers)"]
    ENA_ON_FIFO_WRITES = 0x01,
    #[doc = "Enable on FIFO reads (by software)"]
    ENA_ON_FIFO_READS = 0x02,
    #[doc = "Enable on FIFO writes (from receive shift registers) and reads (by software)"]
    ENA_ON_FIFO_WRITES_READS = 0x03,
}
impl Rcr4Fcomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fcomb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fcomb {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fcomb {
        Rcr4Fcomb::from_bits(val)
    }
}
impl From<Rcr4Fcomb> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fcomb) -> u8 {
        Rcr4Fcomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Fpack {
    #[doc = "Disable"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable 8-bit FIFO packing"]
    EIGHT_BIT_PACKING = 0x02,
    #[doc = "Enable 16-bit FIFO packing"]
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
    #[doc = "Generated externally in Target mode"]
    EXT_TARGET_MODE = 0x0,
    #[doc = "Generated internally in Controller mode"]
    INT_CONTROLLER_MODE = 0x01,
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
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
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
pub enum Rcr5Fbt {
    #[doc = "0"]
    INDEX0 = 0x0,
    #[doc = "FBT value"]
    INDEX_1 = 0x01,
    #[doc = "FBT value"]
    INDEX_2 = 0x02,
    #[doc = "FBT value"]
    INDEX_3 = 0x03,
    #[doc = "FBT value"]
    INDEX_4 = 0x04,
    #[doc = "FBT value"]
    INDEX_5 = 0x05,
    #[doc = "FBT value"]
    INDEX_6 = 0x06,
    #[doc = "FBT value"]
    INDEX_7 = 0x07,
    #[doc = "FBT value"]
    INDEX_8 = 0x08,
    #[doc = "FBT value"]
    INDEX_9 = 0x09,
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
    #[doc = "31"]
    INDEX31 = 0x1f,
}
impl Rcr5Fbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr5Fbt {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr5Fbt {
    #[inline(always)]
    fn from(val: u8) -> Rcr5Fbt {
        Rcr5Fbt::from_bits(val)
    }
}
impl From<Rcr5Fbt> for u8 {
    #[inline(always)]
    fn from(val: Rcr5Fbt) -> u8 {
        Rcr5Fbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr5W0w {
    #[doc = "1"]
    MIN = 0x0,
    #[doc = "2"]
    TWO = 0x01,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_2 = 0x02,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_3 = 0x03,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_4 = 0x04,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_5 = 0x05,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_6 = 0x06,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_7 = 0x07,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_8 = 0x08,
    #[doc = "(W0W value + 1)"]
    THREE_THIRTYONE_9 = 0x09,
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
    #[doc = "32"]
    MAX = 0x1f,
}
impl Rcr5W0w {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr5W0w {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr5W0w {
    #[inline(always)]
    fn from(val: u8) -> Rcr5W0w {
        Rcr5W0w::from_bits(val)
    }
}
impl From<Rcr5W0w> for u8 {
    #[inline(always)]
    fn from(val: Rcr5W0w) -> u8 {
        Rcr5W0w::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr5Wnw {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8"]
    EIGHT = 0x07,
    #[doc = "9"]
    NINE = 0x08,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_9 = 0x09,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_10 = 0x0a,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_11 = 0x0b,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_12 = 0x0c,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_13 = 0x0d,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_14 = 0x0e,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_15 = 0x0f,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_16 = 0x10,
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
    #[doc = "32"]
    MAX = 0x1f,
}
impl Rcr5Wnw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr5Wnw {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr5Wnw {
    #[inline(always)]
    fn from(val: u8) -> Rcr5Wnw {
        Rcr5Wnw::from_bits(val)
    }
}
impl From<Rcr5Wnw> for u8 {
    #[inline(always)]
    fn from(val: Rcr5Wnw) -> u8 {
        Rcr5Wnw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFr {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Reset"]
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
    #[doc = "Watermark not reached"]
    BELOW_WATERMARK = 0x0,
    #[doc = "Watermark reached"]
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
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Software reset"]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfw {
    #[doc = "1"]
    MIN = 0x0,
    #[doc = "2"]
    TWO = 0x01,
    #[doc = "(RFW value + 1)"]
    WATERMARK_2 = 0x02,
    #[doc = "(RFW value + 1)"]
    WATERMARK_3 = 0x03,
    #[doc = "(RFW value + 1)"]
    WATERMARK_4 = 0x04,
    #[doc = "(RFW value + 1)"]
    WATERMARK_5 = 0x05,
    #[doc = "(RFW value + 1)"]
    WATERMARK_6 = 0x06,
    #[doc = "8"]
    MAX = 0x07,
}
impl Rfw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfw {
    #[inline(always)]
    fn from(val: u8) -> Rfw {
        Rfw::from_bits(val)
    }
}
impl From<Rfw> for u8 {
    #[inline(always)]
    fn from(val: Rfw) -> u8 {
        Rfw::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rwm(u32);
impl Rwm {
    #[doc = "Enable"]
    pub const WORD_N_ENABLED: Self = Self(0x0);
    #[doc = "Mask"]
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
pub enum Sywd {
    #[doc = "1"]
    MIN = 0x0,
    #[doc = "2"]
    TWO_CLOCKS = 0x01,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_2 = 0x02,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_3 = 0x03,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_4 = 0x04,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_5 = 0x05,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_6 = 0x06,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_7 = 0x07,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_8 = 0x08,
    #[doc = "(SYWD value + 1)"]
    N_CLOCKS_9 = 0x09,
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
    #[doc = "32"]
    THIRTYTWO_CLOCKS = 0x1f,
}
impl Sywd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sywd {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sywd {
    #[inline(always)]
    fn from(val: u8) -> Sywd {
        Sywd::from_bits(val)
    }
}
impl From<Sywd> for u8 {
    #[inline(always)]
    fn from(val: Sywd) -> u8 {
        Sywd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Bcd {
    #[doc = "Generate externally in Target mode"]
    EXT_IN_TARGET = 0x0,
    #[doc = "Generate internally in Controller mode"]
    INT_IN_CONTROLLER = 0x01,
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
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
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
    #[doc = "Bus clock"]
    BUS_CLOCK = 0x0,
    #[doc = "Controller clock (MCLK) option 1"]
    MCLK1 = 0x01,
    #[doc = "Controller clock (MCLK) option 2"]
    MCLK2 = 0x02,
    #[doc = "Controller clock (MCLK) option 3"]
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
    #[doc = "Asynchronous mode"]
    ASYNC = 0x0,
    #[doc = "Synchronous with receiver"]
    SYNC_W_RX = 0x01,
    #[doc = "Synchronous with another SAI transmitter"]
    SYNC_W_TX = 0x02,
    #[doc = "Synchronous with another SAI receiver"]
    SYNC_W_ANOTHER_SAI_RX = 0x03,
}
impl Tcr2Sync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Sync {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Tcr4Fcomb {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable on FIFO reads (from transmit shift registers)"]
    ENABLED_ON_FIFO_READS = 0x01,
    #[doc = "Enable on FIFO writes (by software)"]
    ENABLED_ON_FIFO_WRITES = 0x02,
    #[doc = "Enable on FIFO reads (from transmit shift registers) and writes (by software)"]
    ENABLED_ON_FIFO_READS_WRITES = 0x03,
}
impl Tcr4Fcomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fcomb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fcomb {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fcomb {
        Tcr4Fcomb::from_bits(val)
    }
}
impl From<Tcr4Fcomb> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fcomb) -> u8 {
        Tcr4Fcomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr4Fpack {
    #[doc = "Disable FIFO packing"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enable 8-bit FIFO packing"]
    EIGHT_BIT_FIFO_PACKING = 0x02,
    #[doc = "Enable 16-bit FIFO packing"]
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
    #[doc = "Generated externally in Target mode"]
    EXT_IN_TARGET_MODE = 0x0,
    #[doc = "Generated internally in Controller mode"]
    INT_IN_CONTROLLER_MODE = 0x01,
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
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
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
    #[doc = "Generated continuously"]
    CONTINUOUS_FRAME_SYNC = 0x0,
    #[doc = "Generated after the FIFO warning flag is cleared"]
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
pub enum Tcr5Fbt {
    #[doc = "0"]
    INDEX0 = 0x0,
    #[doc = "FBT"]
    INDEX_1 = 0x01,
    #[doc = "FBT"]
    INDEX_2 = 0x02,
    #[doc = "FBT"]
    INDEX_3 = 0x03,
    #[doc = "FBT"]
    INDEX_4 = 0x04,
    #[doc = "FBT"]
    INDEX_5 = 0x05,
    #[doc = "FBT"]
    INDEX_6 = 0x06,
    #[doc = "FBT"]
    INDEX_7 = 0x07,
    #[doc = "FBT"]
    INDEX_8 = 0x08,
    #[doc = "FBT"]
    INDEX_9 = 0x09,
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
    #[doc = "31"]
    INDEX31 = 0x1f,
}
impl Tcr5Fbt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr5Fbt {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr5Fbt {
    #[inline(always)]
    fn from(val: u8) -> Tcr5Fbt {
        Tcr5Fbt::from_bits(val)
    }
}
impl From<Tcr5Fbt> for u8 {
    #[inline(always)]
    fn from(val: Tcr5Fbt) -> u8 {
        Tcr5Fbt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr5W0w {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8"]
    EIGHT = 0x07,
    #[doc = "9"]
    NINE = 0x08,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_9 = 0x09,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_10 = 0x0a,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_11 = 0x0b,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_12 = 0x0c,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_13 = 0x0d,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_14 = 0x0e,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_15 = 0x0f,
    #[doc = "(W0W value + 1)"]
    TEN_THIRTYONE_16 = 0x10,
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
    #[doc = "32"]
    MAX = 0x1f,
}
impl Tcr5W0w {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr5W0w {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr5W0w {
    #[inline(always)]
    fn from(val: u8) -> Tcr5W0w {
        Tcr5W0w::from_bits(val)
    }
}
impl From<Tcr5W0w> for u8 {
    #[inline(always)]
    fn from(val: Tcr5W0w) -> u8 {
        Tcr5W0w::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr5Wnw {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8"]
    EIGHT = 0x07,
    #[doc = "9"]
    NINE = 0x08,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_9 = 0x09,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_10 = 0x0a,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_11 = 0x0b,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_12 = 0x0c,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_13 = 0x0d,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_14 = 0x0e,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_15 = 0x0f,
    #[doc = "(WNW value + 1)"]
    TEN_THIRTYONE_16 = 0x10,
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
    #[doc = "32"]
    MAX = 0x1f,
}
impl Tcr5Wnw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr5Wnw {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr5Wnw {
    #[inline(always)]
    fn from(val: u8) -> Tcr5Wnw {
        Tcr5Wnw::from_bits(val)
    }
}
impl From<Tcr5Wnw> for u8 {
    #[inline(always)]
    fn from(val: Tcr5Wnw) -> u8 {
        Tcr5Wnw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFr {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "FIFO reset"]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfw {
    #[doc = "1"]
    MIN = 0x0,
    #[doc = "2"]
    TWO = 0x01,
    #[doc = "(TFW +1)"]
    WATERMARK_VALUE_2 = 0x02,
    #[doc = "(TFW +1)"]
    WATERMARK_VALUE_3 = 0x03,
    #[doc = "(TFW +1)"]
    WATERMARK_VALUE_4 = 0x04,
    #[doc = "(TFW +1)"]
    WATERMARK_VALUE_5 = 0x05,
    #[doc = "(TFW +1)"]
    WATERMARK_VALUE_6 = 0x06,
    #[doc = "8"]
    MAX = 0x07,
}
impl Tfw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfw {
    #[inline(always)]
    fn from(val: u8) -> Tfw {
        Tfw::from_bits(val)
    }
}
impl From<Tfw> for u8 {
    #[inline(always)]
    fn from(val: Tfw) -> u8 {
        Tfw::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Twm(u32);
impl Twm {
    #[doc = "Enable"]
    pub const WORD_N_ENABLED: Self = Self(0x0);
    #[doc = "Mask"]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdfl {
    #[doc = "Word 1"]
    WORD_1 = 0x0,
    #[doc = "Word 2"]
    WORD_2 = 0x01,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_2 = 0x02,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_3 = 0x03,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_4 = 0x04,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_5 = 0x05,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_6 = 0x06,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_7 = 0x07,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_8 = 0x08,
    #[doc = "Word (WDFL value + 1)"]
    WORD_N_9 = 0x09,
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
    #[doc = "Word 32"]
    WORD_MAX = 0x1f,
}
impl Wdfl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdfl {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdfl {
    #[inline(always)]
    fn from(val: u8) -> Wdfl {
        Wdfl::from_bits(val)
    }
}
impl From<Wdfl> for u8 {
    #[inline(always)]
    fn from(val: Wdfl) -> u8 {
        Wdfl::to_bits(val)
    }
}
