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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diven {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Diven {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diven {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diven {
    #[inline(always)]
    fn from(val: u8) -> Diven {
        Diven::from_bits(val)
    }
}
impl From<Diven> for u8 {
    #[inline(always)]
    fn from(val: Diven) -> u8 {
        Diven::to_bits(val)
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
pub enum Rcp {
    #[doc = "No effect"]
    DISABLE = 0x0,
    #[doc = "Next FIFO to be read"]
    ENABLE = 0x01,
}
impl Rcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcp {
    #[inline(always)]
    fn from(val: u8) -> Rcp {
        Rcp::from_bits(val)
    }
}
impl From<Rcp> for u8 {
    #[inline(always)]
    fn from(val: Rcp) -> u8 {
        Rcp::to_bits(val)
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
pub enum Rcr2Byp {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Rcr2Byp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr2Byp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr2Byp {
    #[inline(always)]
    fn from(val: u8) -> Rcr2Byp {
        Rcr2Byp::from_bits(val)
    }
}
impl From<Rcr2Byp> for u8 {
    #[inline(always)]
    fn from(val: Rcr2Byp) -> u8 {
        Rcr2Byp::to_bits(val)
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
pub enum Rcr4Fcont {
    #[doc = "From the start of the next frame after the FIFO error flag is cleared"]
    DISABLE = 0x0,
    #[doc = "From the same word that caused the FIFO error to become 1 after the FIFO warning flag is cleared"]
    ENABLE = 0x01,
}
impl Rcr4Fcont {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fcont {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fcont {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fcont {
        Rcr4Fcont::from_bits(val)
    }
}
impl From<Rcr4Fcont> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fcont) -> u8 {
        Rcr4Fcont::to_bits(val)
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
pub enum Rcr4Fse {
    #[doc = "First bit of the frame"]
    DISABLE = 0x0,
    #[doc = "One bit before the first bit of the frame"]
    ENABLE = 0x01,
}
impl Rcr4Fse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Fse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Fse {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Fse {
        Rcr4Fse::from_bits(val)
    }
}
impl From<Rcr4Fse> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Fse) -> u8 {
        Rcr4Fse::to_bits(val)
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
pub enum Rcr4Mf {
    #[doc = "LSB"]
    DISABLE = 0x0,
    #[doc = "MSB"]
    ENABLE = 0x01,
}
impl Rcr4Mf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Mf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Mf {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Mf {
        Rcr4Mf::from_bits(val)
    }
}
impl From<Rcr4Mf> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Mf) -> u8 {
        Rcr4Mf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcr4Ondem {
    #[doc = "Generated continuously"]
    DISABLE = 0x0,
    #[doc = "Generated when the FIFO warning flag is 0"]
    ENABLE = 0x01,
}
impl Rcr4Ondem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcr4Ondem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcr4Ondem {
    #[inline(always)]
    fn from(val: u8) -> Rcr4Ondem {
        Rcr4Ondem::from_bits(val)
    }
}
impl From<Rcr4Ondem> for u8 {
    #[inline(always)]
    fn from(val: Rcr4Ondem) -> u8 {
        Rcr4Ondem::to_bits(val)
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
pub enum RcsrBce {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrBce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrBce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrBce {
    #[inline(always)]
    fn from(val: u8) -> RcsrBce {
        RcsrBce::from_bits(val)
    }
}
impl From<RcsrBce> for u8 {
    #[inline(always)]
    fn from(val: RcsrBce) -> u8 {
        RcsrBce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrDbge {
    #[doc = "Disable after completing the current frame"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrDbge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrDbge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrDbge {
    #[inline(always)]
    fn from(val: u8) -> RcsrDbge {
        RcsrDbge::from_bits(val)
    }
}
impl From<RcsrDbge> for u8 {
    #[inline(always)]
    fn from(val: RcsrDbge) -> u8 {
        RcsrDbge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFef {
    #[doc = "No error"]
    NO_FLAG = 0x0,
    #[doc = "Receive overflow detected"]
    FLAG = 0x01,
}
impl RcsrFef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFef {
    #[inline(always)]
    fn from(val: u8) -> RcsrFef {
        RcsrFef::from_bits(val)
    }
}
impl From<RcsrFef> for u8 {
    #[inline(always)]
    fn from(val: RcsrFef) -> u8 {
        RcsrFef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFeie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrFeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFeie {
    #[inline(always)]
    fn from(val: u8) -> RcsrFeie {
        RcsrFeie::from_bits(val)
    }
}
impl From<RcsrFeie> for u8 {
    #[inline(always)]
    fn from(val: RcsrFeie) -> u8 {
        RcsrFeie::to_bits(val)
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
pub enum RcsrFrde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrFrde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFrde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFrde {
    #[inline(always)]
    fn from(val: u8) -> RcsrFrde {
        RcsrFrde::from_bits(val)
    }
}
impl From<RcsrFrde> for u8 {
    #[inline(always)]
    fn from(val: RcsrFrde) -> u8 {
        RcsrFrde::to_bits(val)
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
pub enum RcsrFrie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrFrie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFrie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFrie {
    #[inline(always)]
    fn from(val: u8) -> RcsrFrie {
        RcsrFrie::from_bits(val)
    }
}
impl From<RcsrFrie> for u8 {
    #[inline(always)]
    fn from(val: RcsrFrie) -> u8 {
        RcsrFrie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFwde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrFwde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFwde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFwde {
    #[inline(always)]
    fn from(val: u8) -> RcsrFwde {
        RcsrFwde::from_bits(val)
    }
}
impl From<RcsrFwde> for u8 {
    #[inline(always)]
    fn from(val: RcsrFwde) -> u8 {
        RcsrFwde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFwf {
    #[doc = "Not full"]
    NOT_FULL = 0x0,
    #[doc = "Full"]
    FULL = 0x01,
}
impl RcsrFwf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFwf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFwf {
    #[inline(always)]
    fn from(val: u8) -> RcsrFwf {
        RcsrFwf::from_bits(val)
    }
}
impl From<RcsrFwf> for u8 {
    #[inline(always)]
    fn from(val: RcsrFwf) -> u8 {
        RcsrFwf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrFwie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrFwie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrFwie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrFwie {
    #[inline(always)]
    fn from(val: u8) -> RcsrFwie {
        RcsrFwie::from_bits(val)
    }
}
impl From<RcsrFwie> for u8 {
    #[inline(always)]
    fn from(val: RcsrFwie) -> u8 {
        RcsrFwie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrSef {
    #[doc = "Not detected"]
    NO_FLAG = 0x0,
    #[doc = "Detected"]
    FLAG = 0x01,
}
impl RcsrSef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrSef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrSef {
    #[inline(always)]
    fn from(val: u8) -> RcsrSef {
        RcsrSef::from_bits(val)
    }
}
impl From<RcsrSef> for u8 {
    #[inline(always)]
    fn from(val: RcsrSef) -> u8 {
        RcsrSef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrSeie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrSeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrSeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrSeie {
    #[inline(always)]
    fn from(val: u8) -> RcsrSeie {
        RcsrSeie::from_bits(val)
    }
}
impl From<RcsrSeie> for u8 {
    #[inline(always)]
    fn from(val: RcsrSeie) -> u8 {
        RcsrSeie::to_bits(val)
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
pub enum RcsrStope {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrStope {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrStope {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrStope {
    #[inline(always)]
    fn from(val: u8) -> RcsrStope {
        RcsrStope::from_bits(val)
    }
}
impl From<RcsrStope> for u8 {
    #[inline(always)]
    fn from(val: RcsrStope) -> u8 {
        RcsrStope::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrWsf {
    #[doc = "Not detected"]
    NO_FLAG = 0x0,
    #[doc = "Detected"]
    FLAG = 0x01,
}
impl RcsrWsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrWsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrWsf {
    #[inline(always)]
    fn from(val: u8) -> RcsrWsf {
        RcsrWsf::from_bits(val)
    }
}
impl From<RcsrWsf> for u8 {
    #[inline(always)]
    fn from(val: RcsrWsf) -> u8 {
        RcsrWsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RcsrWsie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl RcsrWsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RcsrWsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RcsrWsie {
    #[inline(always)]
    fn from(val: u8) -> RcsrWsie {
        RcsrWsie::from_bits(val)
    }
}
impl From<RcsrWsie> for u8 {
    #[inline(always)]
    fn from(val: RcsrWsie) -> u8 {
        RcsrWsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Re {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (or receiver disabled and not yet reached end of frame)"]
    ENABLE = 0x01,
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
pub enum Tcr2Bci {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Tcr2Bci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Bci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Bci {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Bci {
        Tcr2Bci::from_bits(val)
    }
}
impl From<Tcr2Bci> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Bci) -> u8 {
        Tcr2Bci::to_bits(val)
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
pub enum Tcr2Bcs {
    #[doc = "Use the normal bit clock source"]
    DISABLE = 0x0,
    #[doc = "Swap the bit clock source"]
    ENABLE = 0x01,
}
impl Tcr2Bcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Bcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Bcs {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Bcs {
        Tcr2Bcs::from_bits(val)
    }
}
impl From<Tcr2Bcs> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Bcs) -> u8 {
        Tcr2Bcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcr2Byp {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Tcr2Byp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr2Byp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr2Byp {
    #[inline(always)]
    fn from(val: u8) -> Tcr2Byp {
        Tcr2Byp::from_bits(val)
    }
}
impl From<Tcr2Byp> for u8 {
    #[inline(always)]
    fn from(val: Tcr2Byp) -> u8 {
        Tcr2Byp::to_bits(val)
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
pub enum Tcr4Fcont {
    #[doc = "Continue from the start of the next frame"]
    DISABLE = 0x0,
    #[doc = "Continue from the same word that caused the FIFO error"]
    ENABLE = 0x01,
}
impl Tcr4Fcont {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fcont {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fcont {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fcont {
        Tcr4Fcont::from_bits(val)
    }
}
impl From<Tcr4Fcont> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fcont) -> u8 {
        Tcr4Fcont::to_bits(val)
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
pub enum Tcr4Fse {
    #[doc = "First bit of the frame"]
    DISABLE = 0x0,
    #[doc = "One bit before the first bit of the frame"]
    ENABLE = 0x01,
}
impl Tcr4Fse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Fse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Fse {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Fse {
        Tcr4Fse::from_bits(val)
    }
}
impl From<Tcr4Fse> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Fse) -> u8 {
        Tcr4Fse::to_bits(val)
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
pub enum Tcr4Mf {
    #[doc = "LSB"]
    DISABLE = 0x0,
    #[doc = "MSB"]
    ENABLE = 0x01,
}
impl Tcr4Mf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcr4Mf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcr4Mf {
    #[inline(always)]
    fn from(val: u8) -> Tcr4Mf {
        Tcr4Mf::from_bits(val)
    }
}
impl From<Tcr4Mf> for u8 {
    #[inline(always)]
    fn from(val: Tcr4Mf) -> u8 {
        Tcr4Mf::to_bits(val)
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
pub enum TcsrBce {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrBce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrBce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrBce {
    #[inline(always)]
    fn from(val: u8) -> TcsrBce {
        TcsrBce::from_bits(val)
    }
}
impl From<TcsrBce> for u8 {
    #[inline(always)]
    fn from(val: TcsrBce) -> u8 {
        TcsrBce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrDbge {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrDbge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrDbge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrDbge {
    #[inline(always)]
    fn from(val: u8) -> TcsrDbge {
        TcsrDbge::from_bits(val)
    }
}
impl From<TcsrDbge> for u8 {
    #[inline(always)]
    fn from(val: TcsrDbge) -> u8 {
        TcsrDbge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFef {
    #[doc = "Not detected"]
    NO_FLAG = 0x0,
    #[doc = "Detected"]
    FLAG = 0x01,
}
impl TcsrFef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFef {
    #[inline(always)]
    fn from(val: u8) -> TcsrFef {
        TcsrFef::from_bits(val)
    }
}
impl From<TcsrFef> for u8 {
    #[inline(always)]
    fn from(val: TcsrFef) -> u8 {
        TcsrFef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFeie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrFeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFeie {
    #[inline(always)]
    fn from(val: u8) -> TcsrFeie {
        TcsrFeie::from_bits(val)
    }
}
impl From<TcsrFeie> for u8 {
    #[inline(always)]
    fn from(val: TcsrFeie) -> u8 {
        TcsrFeie::to_bits(val)
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
pub enum TcsrFrde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrFrde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFrde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFrde {
    #[inline(always)]
    fn from(val: u8) -> TcsrFrde {
        TcsrFrde::from_bits(val)
    }
}
impl From<TcsrFrde> for u8 {
    #[inline(always)]
    fn from(val: TcsrFrde) -> u8 {
        TcsrFrde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFrf {
    #[doc = "Watermark not reached"]
    NO_FLAG = 0x0,
    #[doc = "Watermark reached"]
    FLAG = 0x01,
}
impl TcsrFrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFrf {
    #[inline(always)]
    fn from(val: u8) -> TcsrFrf {
        TcsrFrf::from_bits(val)
    }
}
impl From<TcsrFrf> for u8 {
    #[inline(always)]
    fn from(val: TcsrFrf) -> u8 {
        TcsrFrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFrie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrFrie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFrie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFrie {
    #[inline(always)]
    fn from(val: u8) -> TcsrFrie {
        TcsrFrie::from_bits(val)
    }
}
impl From<TcsrFrie> for u8 {
    #[inline(always)]
    fn from(val: TcsrFrie) -> u8 {
        TcsrFrie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFwde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrFwde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFwde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFwde {
    #[inline(always)]
    fn from(val: u8) -> TcsrFwde {
        TcsrFwde::from_bits(val)
    }
}
impl From<TcsrFwde> for u8 {
    #[inline(always)]
    fn from(val: TcsrFwde) -> u8 {
        TcsrFwde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFwf {
    #[doc = "Not empty"]
    DISABLE = 0x0,
    #[doc = "Empty"]
    ENABLE = 0x01,
}
impl TcsrFwf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFwf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFwf {
    #[inline(always)]
    fn from(val: u8) -> TcsrFwf {
        TcsrFwf::from_bits(val)
    }
}
impl From<TcsrFwf> for u8 {
    #[inline(always)]
    fn from(val: TcsrFwf) -> u8 {
        TcsrFwf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrFwie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrFwie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrFwie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrFwie {
    #[inline(always)]
    fn from(val: u8) -> TcsrFwie {
        TcsrFwie::from_bits(val)
    }
}
impl From<TcsrFwie> for u8 {
    #[inline(always)]
    fn from(val: TcsrFwie) -> u8 {
        TcsrFwie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrSef {
    #[doc = "Not detected"]
    NO_FLAG = 0x0,
    #[doc = "Detected"]
    FLAG = 0x01,
}
impl TcsrSef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrSef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrSef {
    #[inline(always)]
    fn from(val: u8) -> TcsrSef {
        TcsrSef::from_bits(val)
    }
}
impl From<TcsrSef> for u8 {
    #[inline(always)]
    fn from(val: TcsrSef) -> u8 {
        TcsrSef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrSeie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrSeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrSeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrSeie {
    #[inline(always)]
    fn from(val: u8) -> TcsrSeie {
        TcsrSeie::from_bits(val)
    }
}
impl From<TcsrSeie> for u8 {
    #[inline(always)]
    fn from(val: TcsrSeie) -> u8 {
        TcsrSeie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrSr {
    #[doc = "No effect"]
    DISABLE = 0x0,
    #[doc = "Software reset"]
    ENABLE = 0x01,
}
impl TcsrSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrSr {
    #[inline(always)]
    fn from(val: u8) -> TcsrSr {
        TcsrSr::from_bits(val)
    }
}
impl From<TcsrSr> for u8 {
    #[inline(always)]
    fn from(val: TcsrSr) -> u8 {
        TcsrSr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrStope {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrStope {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrStope {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrStope {
    #[inline(always)]
    fn from(val: u8) -> TcsrStope {
        TcsrStope::from_bits(val)
    }
}
impl From<TcsrStope> for u8 {
    #[inline(always)]
    fn from(val: TcsrStope) -> u8 {
        TcsrStope::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrWsf {
    #[doc = "Not detected"]
    NO_FLAG = 0x0,
    #[doc = "Detected"]
    FLAG = 0x01,
}
impl TcsrWsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrWsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrWsf {
    #[inline(always)]
    fn from(val: u8) -> TcsrWsf {
        TcsrWsf::from_bits(val)
    }
}
impl From<TcsrWsf> for u8 {
    #[inline(always)]
    fn from(val: TcsrWsf) -> u8 {
        TcsrWsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcsrWsie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl TcsrWsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcsrWsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcsrWsie {
    #[inline(always)]
    fn from(val: u8) -> TcsrWsie {
        TcsrWsie::from_bits(val)
    }
}
impl From<TcsrWsie> for u8 {
    #[inline(always)]
    fn from(val: TcsrWsie) -> u8 {
        TcsrWsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Te {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable (or transmitter has been disabled and has not yet reached the end of the frame)"]
    ENABLE = 0x01,
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
pub enum Wcp {
    #[doc = "No effect"]
    DISABLE = 0x0,
    #[doc = "Next FIFO to be written"]
    ENABLE = 0x01,
}
impl Wcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wcp {
    #[inline(always)]
    fn from(val: u8) -> Wcp {
        Wcp::from_bits(val)
    }
}
impl From<Wcp> for u8 {
    #[inline(always)]
    fn from(val: Wcp) -> u8 {
        Wcp::to_bits(val)
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
