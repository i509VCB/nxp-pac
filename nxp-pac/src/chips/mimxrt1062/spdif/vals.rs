#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSrcSel {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)."]
    Clksrc0b0000 = 0x0,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)."]
    Clksrc0b0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK."]
    Clksrc0b0011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "REF_CLK_32K (XTALOSC)."]
    Clksrc0b0101 = 0x05,
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)."]
    Clksrc0b0110 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SPDIF_EXT_CLK."]
    Clksrc0b1000 = 0x08,
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
    #[doc = "24*(2**10)."]
    Gainsel0b000 = 0x0,
    #[doc = "16*(2**10)."]
    Gainsel0b001 = 0x01,
    #[doc = "12*(2**10)."]
    Gainsel0b010 = 0x02,
    #[doc = "8*(2**10)."]
    Gainsel0b011 = 0x03,
    #[doc = "6*(2**10)."]
    Gainsel0b100 = 0x04,
    #[doc = "4*(2**10)."]
    Gainsel0b101 = 0x05,
    #[doc = "3*(2**10)."]
    Gainsel0b110 = 0x06,
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
    #[doc = "SPDIF_IN."]
    SpdifIn = 0x0,
    #[doc = "None."]
    NoneSel1 = 0x01,
    #[doc = "None."]
    NoneSel2 = 0x02,
    #[doc = "None."]
    NoneSel3 = 0x03,
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
    #[doc = "Normal operation."]
    Normal = 0x0,
    #[doc = "Always read zero from Rx data register."]
    AlwaysZero = 0x01,
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
pub enum RxFifoFullSel {
    #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs."]
    FullInt1 = 0x0,
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs."]
    FullInt4 = 0x01,
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs."]
    FullInt8 = 0x02,
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO."]
    FullInt16 = 0x03,
}
impl RxFifoFullSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFifoFullSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFifoFullSel {
    #[inline(always)]
    fn from(val: u8) -> RxFifoFullSel {
        RxFifoFullSel::from_bits(val)
    }
}
impl From<RxFifoFullSel> for u8 {
    #[inline(always)]
    fn from(val: RxFifoFullSel) -> u8 {
        RxFifoFullSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifoOffOn {
    #[doc = "SPDIF Rx FIFO is on."]
    On0 = 0x0,
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface."]
    Off1 = 0x01,
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
    #[doc = "Normal operation."]
    Normal = 0x0,
    #[doc = "Reset register to 1 sample remaining."]
    ResetOne = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SysclkDf(u16);
impl SysclkDf {
    #[doc = "no clock signal."]
    pub const NoClk: Self = Self(0x0);
    #[doc = "divider factor is 2."]
    pub const Div2: Self = Self(0x01);
    #[doc = "divider factor is 512."]
    pub const Div512: Self = Self(0x01ff);
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
            0x0 => f.write_str("NoClk"),
            0x01 => f.write_str("Div2"),
            0x01ff => f.write_str("Div512"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysclkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NoClk"),
            0x01 => defmt::write!(f, "Div2"),
            0x01ff => defmt::write!(f, "Div512"),
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
    #[doc = "divider factor is 1."]
    pub const Div1: Self = Self(0x0);
    #[doc = "divider factor is 2."]
    pub const Div2: Self = Self(0x01);
    #[doc = "divider factor is 128."]
    pub const Div128: Self = Self(0x7f);
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
            0x0 => f.write_str("Div1"),
            0x01 => f.write_str("Div2"),
            0x7f => f.write_str("Div128"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Div1"),
            0x01 => defmt::write!(f, "Div2"),
            0x7f => defmt::write!(f, "Div128"),
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
    #[doc = "REF_CLK_32K input (XTALOSC 32 kHz clock)."]
    TxclkSrc0b000 = 0x0,
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See clock control block for more information.)."]
    TxclkSrc0b001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SPDIF_EXT_CLK, from pads."]
    TxclkSrc0b011 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "ipg_clk input (frequency divided)."]
    TxclkSrc0b101 = 0x05,
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
    #[doc = "Send out digital zero on SPDIF Tx."]
    SendZero = 0x0,
    #[doc = "Tx Normal operation."]
    Normal = 0x01,
    #[doc = "Reset to 1 sample remaining."]
    ResetOne = 0x02,
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
pub enum TxFifoEmptySel {
    #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs."]
    EmptyInt0 = 0x0,
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs."]
    EmptyInt4 = 0x01,
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs."]
    EmptyInt8 = 0x02,
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs."]
    EmptyInt12 = 0x03,
}
impl TxFifoEmptySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxFifoEmptySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxFifoEmptySel {
    #[inline(always)]
    fn from(val: u8) -> TxFifoEmptySel {
        TxFifoEmptySel::from_bits(val)
    }
}
impl From<TxFifoEmptySel> for u8 {
    #[inline(always)]
    fn from(val: TxFifoEmptySel) -> u8 {
        TxFifoEmptySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxSel {
    #[doc = "Off and output 0."]
    OffOut0 = 0x0,
    #[doc = "Feed-through SPDIFIN."]
    Feedthru = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Tx Normal operation."]
    NormalOp = 0x05,
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
pub enum USrcSel {
    #[doc = "No embedded U channel."]
    None = 0x0,
    #[doc = "U channel from SPDIF receive block (CD mode)."]
    SpdifRxblock = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "U channel from on chip transmitter."]
    ChipTransmit = 0x03,
}
impl USrcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USrcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USrcSel {
    #[inline(always)]
    fn from(val: u8) -> USrcSel {
        USrcSel::from_bits(val)
    }
}
impl From<USrcSel> for u8 {
    #[inline(always)]
    fn from(val: USrcSel) -> u8 {
        USrcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USyncMode {
    #[doc = "Non-CD data."]
    NonCddata = 0x0,
    #[doc = "CD user channel subcode."]
    CduserChsubcode = 0x01,
}
impl USyncMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USyncMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USyncMode {
    #[inline(always)]
    fn from(val: u8) -> USyncMode {
        USyncMode::from_bits(val)
    }
}
impl From<USyncMode> for u8 {
    #[inline(always)]
    fn from(val: USyncMode) -> u8 {
        USyncMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ValCtrl {
    #[doc = "Outgoing Validity always set."]
    AlwaysSet = 0x0,
    #[doc = "Outgoing Validity always clear."]
    AlwaysClear = 0x01,
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
