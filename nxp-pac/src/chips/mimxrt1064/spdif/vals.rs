#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSrcSel {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)."]
    ClkSrcSel0 = 0x0,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)."]
    ClkSrcSel1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK."]
    ClkSrcSel3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "REF_CLK_32K (XTALOSC)."]
    ClkSrcSel5 = 0x05,
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)."]
    ClkSrcSel6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SPDIF_EXT_CLK."]
    ClkSrcSel8 = 0x08,
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
    GainSel0 = 0x0,
    #[doc = "16*(2**10)."]
    GainSel1 = 0x01,
    #[doc = "12*(2**10)."]
    GainSel2 = 0x02,
    #[doc = "8*(2**10)."]
    GainSel3 = 0x03,
    #[doc = "6*(2**10)."]
    GainSel4 = 0x04,
    #[doc = "4*(2**10)."]
    GainSel5 = 0x05,
    #[doc = "3*(2**10)."]
    GainSel6 = 0x06,
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
pub enum RxAutoSync {
    #[doc = "Rx FIFO auto sync off."]
    RxAutoSync0 = 0x0,
    #[doc = "RxFIFO auto sync on."]
    RxAutoSync1 = 0x01,
}
impl RxAutoSync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxAutoSync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxAutoSync {
    #[inline(always)]
    fn from(val: u8) -> RxAutoSync {
        RxAutoSync::from_bits(val)
    }
}
impl From<RxAutoSync> for u8 {
    #[inline(always)]
    fn from(val: RxAutoSync) -> u8 {
        RxAutoSync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifoCtrl {
    #[doc = "Normal operation."]
    RxFifoCtrl0 = 0x0,
    #[doc = "Always read zero from Rx data register."]
    RxFifoCtrl1 = 0x01,
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
    RxFifoFullSel0 = 0x0,
    #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs."]
    RxFifoFullSel1 = 0x01,
    #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs."]
    RxFifoFullSel2 = 0x02,
    #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO."]
    RxFifoFullSel3 = 0x03,
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
    RxFifoOffOn0 = 0x0,
    #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface."]
    RxFifoOffOn1 = 0x01,
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
    RxFifoRst0 = 0x0,
    #[doc = "Reset register to 1 sample remaining."]
    RxFifoRst1 = 0x01,
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
    pub const SysclkDf0: Self = Self(0x0);
    #[doc = "divider factor is 2."]
    pub const SysclkDf1: Self = Self(0x01);
    #[doc = "divider factor is 512."]
    pub const SysclkDf511: Self = Self(0x01ff);
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
            0x0 => f.write_str("SysclkDf0"),
            0x01 => f.write_str("SysclkDf1"),
            0x01ff => f.write_str("SysclkDf511"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysclkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SysclkDf0"),
            0x01 => defmt::write!(f, "SysclkDf1"),
            0x01ff => defmt::write!(f, "SysclkDf511"),
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxAllClkEn {
    #[doc = "disable transfer clock."]
    TxAllClkEn0 = 0x0,
    #[doc = "enable transfer clock."]
    TxAllClkEn1 = 0x01,
}
impl TxAllClkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxAllClkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxAllClkEn {
    #[inline(always)]
    fn from(val: u8) -> TxAllClkEn {
        TxAllClkEn::from_bits(val)
    }
}
impl From<TxAllClkEn> for u8 {
    #[inline(always)]
    fn from(val: TxAllClkEn) -> u8 {
        TxAllClkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxAutoSync {
    #[doc = "Tx FIFO auto sync off."]
    TxAutoSync0 = 0x0,
    #[doc = "Tx FIFO auto sync on."]
    TxAutoSync1 = 0x01,
}
impl TxAutoSync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxAutoSync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxAutoSync {
    #[inline(always)]
    fn from(val: u8) -> TxAutoSync {
        TxAutoSync::from_bits(val)
    }
}
impl From<TxAutoSync> for u8 {
    #[inline(always)]
    fn from(val: TxAutoSync) -> u8 {
        TxAutoSync::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TxClkDf(u8);
impl TxClkDf {
    #[doc = "divider factor is 1."]
    pub const TxClkDf0: Self = Self(0x0);
    #[doc = "divider factor is 2."]
    pub const TxClkDf1: Self = Self(0x01);
    #[doc = "divider factor is 128."]
    pub const TxClkDf127: Self = Self(0x7f);
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
            0x0 => f.write_str("TxClkDf0"),
            0x01 => f.write_str("TxClkDf1"),
            0x7f => f.write_str("TxClkDf127"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClkDf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TxClkDf0"),
            0x01 => defmt::write!(f, "TxClkDf1"),
            0x7f => defmt::write!(f, "TxClkDf127"),
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
    #[doc = "XTALOSC input (XTALOSC clock)."]
    TxClkSource0 = 0x0,
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)."]
    TxClkSource1 = 0x01,
    #[doc = "tx_clk1 (from SAI1)."]
    TxClkSource2 = 0x02,
    #[doc = "tx_clk2 SPDIF_EXT_CLK, from pads."]
    TxClkSource3 = 0x03,
    #[doc = "tx_clk3 (from SAI2)."]
    TxClkSource4 = 0x04,
    #[doc = "ipg_clk input (frequency divided)."]
    TxClkSource5 = 0x05,
    #[doc = "tx_clk4 (from SAI3)."]
    TxClkSource6 = 0x06,
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
    TxFifoCtrl0 = 0x0,
    #[doc = "Tx Normal operation."]
    TxFifoCtrl1 = 0x01,
    #[doc = "Reset to 1 sample remaining."]
    TxFifoCtrl2 = 0x02,
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
    TxFifoEmptySel0 = 0x0,
    #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs."]
    TxFifoEmptySel1 = 0x01,
    #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs."]
    TxFifoEmptySel2 = 0x02,
    #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs."]
    TxFifoEmptySel3 = 0x03,
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
    TxSel0 = 0x0,
    #[doc = "Feed-through SPDIFIN."]
    TxSel1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Tx Normal operation."]
    TxSel5 = 0x05,
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
    USrcSel0 = 0x0,
    #[doc = "U channel from SPDIF receive block (CD mode)."]
    USrcSel1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "U channel from on chip transmitter."]
    USrcSel3 = 0x03,
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
    USyncMode0 = 0x0,
    #[doc = "CD user channel subcode."]
    USyncMode1 = 0x01,
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
    ValCtrl0 = 0x0,
    #[doc = "Outgoing Validity always clear."]
    ValCtrl1 = 0x01,
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
