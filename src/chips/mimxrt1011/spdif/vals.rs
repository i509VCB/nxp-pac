#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSrcSel {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    CLK_SRC_SEL_0 = 0x0,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    CLK_SRC_SEL_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    CLK_SRC_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "REF_CLK_32K (XTALOSC)"]
    CLK_SRC_SEL_5 = 0x05,
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    CLK_SRC_SEL_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SPDIF_EXT_CLK"]
    CLK_SRC_SEL_8 = 0x08,
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
    GAIN_SEL_0 = 0x0,
    #[doc = "16*(2**10)"]
    GAIN_SEL_1 = 0x01,
    #[doc = "12*(2**10)"]
    GAIN_SEL_2 = 0x02,
    #[doc = "8*(2**10)"]
    GAIN_SEL_3 = 0x03,
    #[doc = "6*(2**10)"]
    GAIN_SEL_4 = 0x04,
    #[doc = "4*(2**10)"]
    GAIN_SEL_5 = 0x05,
    #[doc = "3*(2**10)"]
    GAIN_SEL_6 = 0x06,
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
pub enum RxFifofullSel {
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
    RX_FIFOFULL_SEL_0 = 0x0,
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
    RX_FIFOFULL_SEL_1 = 0x01,
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
    RX_FIFOFULL_SEL_2 = 0x02,
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
    RX_FIFOFULL_SEL_3 = 0x03,
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
    pub const SYSCLK_DF_0: Self = Self(0x0);
    #[doc = "divider factor is 2"]
    pub const SYSCLK_DF_1: Self = Self(0x01);
    #[doc = "divider factor is 512"]
    pub const SYSCLK_DF_511: Self = Self(0x01ff);
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
            0x0 => f.write_str("SYSCLK_DF_0"),
            0x01 => f.write_str("SYSCLK_DF_1"),
            0x01ff => f.write_str("SYSCLK_DF_511"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysclkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SYSCLK_DF_0"),
            0x01 => defmt::write!(f, "SYSCLK_DF_1"),
            0x01ff => defmt::write!(f, "SYSCLK_DF_511"),
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
    pub const TX_CLK_DF_0: Self = Self(0x0);
    #[doc = "divider factor is 2"]
    pub const TX_CLK_DF_1: Self = Self(0x01);
    #[doc = "divider factor is 128"]
    pub const TX_CLK_DF_127: Self = Self(0x7f);
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
            0x0 => f.write_str("TX_CLK_DF_0"),
            0x01 => f.write_str("TX_CLK_DF_1"),
            0x7f => f.write_str("TX_CLK_DF_127"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TX_CLK_DF_0"),
            0x01 => defmt::write!(f, "TX_CLK_DF_1"),
            0x7f => defmt::write!(f, "TX_CLK_DF_127"),
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
    #[doc = "XTALOSC input (XTALOSC clock)"]
    TX_CLK_SOURCE_0 = 0x0,
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    TX_CLK_SOURCE_1 = 0x01,
    #[doc = "tx_clk1 (from SAI1)"]
    TX_CLK_SOURCE_2 = 0x02,
    #[doc = "tx_clk2 SPDIF_EXT_CLK, from pads"]
    TX_CLK_SOURCE_3 = 0x03,
    #[doc = "tx_clk3 (from SAI2)"]
    TX_CLK_SOURCE_4 = 0x04,
    #[doc = "ipg_clk input (frequency divided)"]
    TX_CLK_SOURCE_5 = 0x05,
    #[doc = "tx_clk4 (from SAI3)"]
    TX_CLK_SOURCE_6 = 0x06,
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
    TX_FIFO_CTRL_0 = 0x0,
    #[doc = "Tx Normal operation"]
    TX_FIFO_CTRL_1 = 0x01,
    #[doc = "Reset to 1 sample remaining"]
    TX_FIFO_CTRL_2 = 0x02,
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
    TX_FIFOEMPTY_SEL_0 = 0x0,
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_1 = 0x01,
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_2 = 0x02,
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
    TX_FIFOEMPTY_SEL_3 = 0x03,
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
    TX_SEL_0 = 0x0,
    #[doc = "Feed-through SPDIFIN"]
    TX_SEL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Tx Normal operation"]
    TX_SEL_5 = 0x05,
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
    USRC_SEL_0 = 0x0,
    #[doc = "U channel from SPDIF receive block (CD mode)"]
    USRC_SEL_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "U channel from on chip transmitter"]
    USRC_SEL_3 = 0x03,
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
