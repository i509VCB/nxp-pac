#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSrcSel {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    CLKSRC_0B0000 = 0x0,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_0B0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    CLKSRC_0B0011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "REF_CLK_32K (XTALOSC)"]
    CLKSRC_0B0101 = 0x05,
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    CLKSRC_0B0110 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SPDIF_EXT_CLK"]
    CLKSRC_0B1000 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl ClkSrcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSrcSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSrcSel {
    #[inline(always)]
    fn from(val: u8) -> ClkSrcSel {
        ClkSrcSel::from_bits(val)
    }
}
impl From<ClkSrcSel> for u8 {
    #[inline(always)]
    fn from(val: ClkSrcSel) -> u8 {
        ClkSrcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GainSel {
    #[doc = "24*(2**10)"]
    GAINSEL_0B000 = 0x0,
    #[doc = "16*(2**10)"]
    GAINSEL_0B001 = 0x01,
    #[doc = "12*(2**10)"]
    GAINSEL_0B010 = 0x02,
    #[doc = "8*(2**10)"]
    GAINSEL_0B011 = 0x03,
    #[doc = "6*(2**10)"]
    GAINSEL_0B100 = 0x04,
    #[doc = "4*(2**10)"]
    GAINSEL_0B101 = 0x05,
    #[doc = "3*(2**10)"]
    GAINSEL_0B110 = 0x06,
    _RESERVED_7 = 0x07,
}
impl GainSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GainSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GainSel {
    #[inline(always)]
    fn from(val: u8) -> GainSel {
        GainSel::from_bits(val)
    }
}
impl From<GainSel> for u8 {
    #[inline(always)]
    fn from(val: GainSel) -> u8 {
        GainSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputSrcSel {
    #[doc = "SPDIF_IN"]
    SPDIF_IN = 0x0,
    #[doc = "None"]
    NONE_SEL_1 = 0x01,
    #[doc = "None"]
    NONE_SEL_2 = 0x02,
    #[doc = "None"]
    NONE_SEL_3 = 0x03,
}
impl InputSrcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InputSrcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InputSrcSel {
    #[inline(always)]
    fn from(val: u8) -> InputSrcSel {
        InputSrcSel::from_bits(val)
    }
}
impl From<InputSrcSel> for u8 {
    #[inline(always)]
    fn from(val: InputSrcSel) -> u8 {
        InputSrcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifoCtrl {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "Always read zero from Rx data register"]
    ALWAYS_ZERO = 0x01,
}
impl RxFifoCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFifoCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFifoCtrl {
    #[inline(always)]
    fn from(val: u8) -> RxFifoCtrl {
        RxFifoCtrl::from_bits(val)
    }
}
impl From<RxFifoCtrl> for u8 {
    #[inline(always)]
    fn from(val: RxFifoCtrl) -> u8 {
        RxFifoCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifoOffOn {
    #[doc = "SPDIF Rx FIFO is on"]
    ON_0 = 0x0,
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
    OFF_1 = 0x01,
}
impl RxFifoOffOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFifoOffOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFifoOffOn {
    #[inline(always)]
    fn from(val: u8) -> RxFifoOffOn {
        RxFifoOffOn::from_bits(val)
    }
}
impl From<RxFifoOffOn> for u8 {
    #[inline(always)]
    fn from(val: RxFifoOffOn) -> u8 {
        RxFifoOffOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifoRst {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "Reset register to 1 sample remaining"]
    RESET_ONE = 0x01,
}
impl RxFifoRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFifoRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFifoRst {
    #[inline(always)]
    fn from(val: u8) -> RxFifoRst {
        RxFifoRst::from_bits(val)
    }
}
impl From<RxFifoRst> for u8 {
    #[inline(always)]
    fn from(val: RxFifoRst) -> u8 {
        RxFifoRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifofullSel {
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    FULL_INT_1 = 0x0,
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    FULL_INT_4 = 0x01,
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    FULL_INT_8 = 0x02,
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    FULL_INT_16 = 0x03,
}
impl RxFifofullSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFifofullSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFifofullSel {
    #[inline(always)]
    fn from(val: u8) -> RxFifofullSel {
        RxFifofullSel::from_bits(val)
    }
}
impl From<RxFifofullSel> for u8 {
    #[inline(always)]
    fn from(val: RxFifofullSel) -> u8 {
        RxFifofullSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SysclkDf(u16);
impl SysclkDf {
    #[doc = "no clock signal"]
    pub const NO_CLK: Self = Self(0x0);
    #[doc = "divider factor is 2"]
    pub const DIV2: Self = Self(0x01);
    #[doc = "divider factor is 512"]
    pub const DIV512: Self = Self(0x01ff);
}
impl SysclkDf {
    pub const fn from_bits(val: u16) -> SysclkDf {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for SysclkDf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_CLK"),
            0x01 => f.write_str("DIV2"),
            0x01ff => f.write_str("DIV512"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysclkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_CLK"),
            0x01 => defmt::write!(f, "DIV2"),
            0x01ff => defmt::write!(f, "DIV512"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for SysclkDf {
    #[inline(always)]
    fn from(val: u16) -> SysclkDf {
        SysclkDf::from_bits(val)
    }
}
impl From<SysclkDf> for u16 {
    #[inline(always)]
    fn from(val: SysclkDf) -> u16 {
        SysclkDf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TxClkDf(u8);
impl TxClkDf {
    #[doc = "divider factor is 1"]
    pub const DIV1: Self = Self(0x0);
    #[doc = "divider factor is 2"]
    pub const DIV2: Self = Self(0x01);
    #[doc = "divider factor is 128"]
    pub const DIV128: Self = Self(0x7f);
}
impl TxClkDf {
    pub const fn from_bits(val: u8) -> TxClkDf {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for TxClkDf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DIV1"),
            0x01 => f.write_str("DIV2"),
            0x7f => f.write_str("DIV128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DIV1"),
            0x01 => defmt::write!(f, "DIV2"),
            0x7f => defmt::write!(f, "DIV128"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for TxClkDf {
    #[inline(always)]
    fn from(val: u8) -> TxClkDf {
        TxClkDf::from_bits(val)
    }
}
impl From<TxClkDf> for u8 {
    #[inline(always)]
    fn from(val: TxClkDf) -> u8 {
        TxClkDf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxClkSource {
    #[doc = "REF_CLK_32K input (XTALOSC 32 kHz clock)"]
    TXCLK_SRC_0B000 = 0x0,
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See clock control block for more information.)"]
    TXCLK_SRC_0B001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SPDIF_EXT_CLK, from pads"]
    TXCLK_SRC_0B011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "ipg_clk input (frequency divided)"]
    TXCLK_SRC_0B101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TxClkSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxClkSource {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxClkSource {
    #[inline(always)]
    fn from(val: u8) -> TxClkSource {
        TxClkSource::from_bits(val)
    }
}
impl From<TxClkSource> for u8 {
    #[inline(always)]
    fn from(val: TxClkSource) -> u8 {
        TxClkSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxFifoCtrl {
    #[doc = "Send out digital zero on SPDIF Tx"]
    SEND_ZERO = 0x0,
    #[doc = "Tx Normal operation"]
    NORMAL = 0x01,
    #[doc = "Reset to 1 sample remaining"]
    RESET_ONE = 0x02,
    _RESERVED_3 = 0x03,
}
impl TxFifoCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxFifoCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxFifoCtrl {
    #[inline(always)]
    fn from(val: u8) -> TxFifoCtrl {
        TxFifoCtrl::from_bits(val)
    }
}
impl From<TxFifoCtrl> for u8 {
    #[inline(always)]
    fn from(val: TxFifoCtrl) -> u8 {
        TxFifoCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxFifoemptySel {
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
    EMPTY_INT_0 = 0x0,
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    EMPTY_INT_4 = 0x01,
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    EMPTY_INT_8 = 0x02,
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    EMPTY_INT_12 = 0x03,
}
impl TxFifoemptySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxFifoemptySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxFifoemptySel {
    #[inline(always)]
    fn from(val: u8) -> TxFifoemptySel {
        TxFifoemptySel::from_bits(val)
    }
}
impl From<TxFifoemptySel> for u8 {
    #[inline(always)]
    fn from(val: TxFifoemptySel) -> u8 {
        TxFifoemptySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxSel {
    #[doc = "Off and output 0"]
    OFF_OUT0 = 0x0,
    #[doc = "Feed-through SPDIFIN"]
    FEEDTHRU = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Tx Normal operation"]
    NORMAL_OP = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TxSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxSel {
    #[inline(always)]
    fn from(val: u8) -> TxSel {
        TxSel::from_bits(val)
    }
}
impl From<TxSel> for u8 {
    #[inline(always)]
    fn from(val: TxSel) -> u8 {
        TxSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsrcSel {
    #[doc = "No embedded U channel"]
    NONE = 0x0,
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    SPDIF_RXBLOCK = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "U channel from on chip transmitter"]
    CHIP_TRANSMIT = 0x03,
}
impl UsrcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsrcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsrcSel {
    #[inline(always)]
    fn from(val: u8) -> UsrcSel {
        UsrcSel::from_bits(val)
    }
}
impl From<UsrcSel> for u8 {
    #[inline(always)]
    fn from(val: UsrcSel) -> u8 {
        UsrcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsyncMode {
    #[doc = "Non-CD data"]
    NON_CDDATA = 0x0,
    #[doc = "CD user channel subcode"]
    CDUSER_CHSUBCODE = 0x01,
}
impl UsyncMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsyncMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsyncMode {
    #[inline(always)]
    fn from(val: u8) -> UsyncMode {
        UsyncMode::from_bits(val)
    }
}
impl From<UsyncMode> for u8 {
    #[inline(always)]
    fn from(val: UsyncMode) -> u8 {
        UsyncMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ValCtrl {
    #[doc = "Outgoing Validity always set"]
    ALWAYS_SET = 0x0,
    #[doc = "Outgoing Validity always clear"]
    ALWAYS_CLEAR = 0x01,
}
impl ValCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ValCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ValCtrl {
    #[inline(always)]
    fn from(val: u8) -> ValCtrl {
        ValCtrl::from_bits(val)
    }
}
impl From<ValCtrl> for u8 {
    #[inline(always)]
    fn from(val: ValCtrl) -> u8 {
        ValCtrl::to_bits(val)
    }
}
