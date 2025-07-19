#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel2 {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_2_0 = 0x0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_2_1 = 0x01,
}
impl IomuxcXbarDirSel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel2 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel2 {
        IomuxcXbarDirSel2::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel2> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel2) -> u8 {
        IomuxcXbarDirSel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel3 {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_3_0 = 0x0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_3_1 = 0x01,
}
impl IomuxcXbarDirSel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel3 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel3 {
        IomuxcXbarDirSel3::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel3> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel3) -> u8 {
        IomuxcXbarDirSel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockOcramTzAddr {
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_ADDR_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_ADDR_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl LockOcramTzAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockOcramTzAddr {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockOcramTzAddr {
    #[inline(always)]
    fn from(val: u8) -> LockOcramTzAddr {
        LockOcramTzAddr::from_bits(val)
    }
}
impl From<LockOcramTzAddr> for u8 {
    #[inline(always)]
    fn from(val: LockOcramTzAddr) -> u8 {
        LockOcramTzAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR0Ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R0_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R0_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR0Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR0Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR0Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR0Ctrl {
        M7ApcAcR0Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR0Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR0Ctrl) -> u8 {
        M7ApcAcR0Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR1Ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R1_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R1_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR1Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR1Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR1Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR1Ctrl {
        M7ApcAcR1Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR1Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR1Ctrl) -> u8 {
        M7ApcAcR1Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR2Ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R2_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R2_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR2Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR2Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR2Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR2Ctrl {
        M7ApcAcR2Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR2Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR2Ctrl) -> u8 {
        M7ApcAcR2Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR3Ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R3_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R3_CTRL_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR3Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR3Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR3Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR3Ctrl {
        M7ApcAcR3Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR3Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR3Ctrl) -> u8 {
        M7ApcAcR3Ctrl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MqsClkDiv(u8);
impl MqsClkDiv {
    #[doc = "mclk frequency = hmclk frequency"]
    pub const MQS_CLK_DIV_0: Self = Self(0x0);
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    pub const MQS_CLK_DIV_1: Self = Self(0x01);
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    pub const MQS_CLK_DIV_2: Self = Self(0x02);
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    pub const MQS_CLK_DIV_255: Self = Self(0xff);
}
impl MqsClkDiv {
    pub const fn from_bits(val: u8) -> MqsClkDiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MqsClkDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MQS_CLK_DIV_0"),
            0x01 => f.write_str("MQS_CLK_DIV_1"),
            0x02 => f.write_str("MQS_CLK_DIV_2"),
            0xff => f.write_str("MQS_CLK_DIV_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MqsClkDiv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MQS_CLK_DIV_0"),
            0x01 => defmt::write!(f, "MQS_CLK_DIV_1"),
            0x02 => defmt::write!(f, "MQS_CLK_DIV_2"),
            0xff => defmt::write!(f, "MQS_CLK_DIV_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MqsClkDiv {
    #[inline(always)]
    fn from(val: u8) -> MqsClkDiv {
        MqsClkDiv::from_bits(val)
    }
}
impl From<MqsClkDiv> for u8 {
    #[inline(always)]
    fn from(val: MqsClkDiv) -> u8 {
        MqsClkDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk1Sel {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai1Mclk1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk1Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk1Sel {
        Sai1Mclk1Sel::from_bits(val)
    }
}
impl From<Sai1Mclk1Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk1Sel) -> u8 {
        Sai1Mclk1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk2Sel {
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai1Mclk2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk2Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk2Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk2Sel {
        Sai1Mclk2Sel::from_bits(val)
    }
}
impl From<Sai1Mclk2Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk2Sel) -> u8 {
        Sai1Mclk2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk3Sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0 = 0x0,
    #[doc = "SPDIF_EXT_CLK"]
    SAI1_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3 = 0x03,
}
impl Sai1Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk3Sel {
        Sai1Mclk3Sel::from_bits(val)
    }
}
impl From<Sai1Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk3Sel) -> u8 {
        Sai1Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1MclkDir {
    #[doc = "sai1.MCLK is input signal"]
    SAI1_MCLK_DIR_0 = 0x0,
    #[doc = "sai1.MCLK is output signal"]
    SAI1_MCLK_DIR_1 = 0x01,
}
impl Sai1MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai1MclkDir {
        Sai1MclkDir::from_bits(val)
    }
}
impl From<Sai1MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai1MclkDir) -> u8 {
        Sai1MclkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3Mclk3Sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0 = 0x0,
    #[doc = "SPDIF_EXT_CLK"]
    SAI3_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3 = 0x03,
}
impl Sai3Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai3Mclk3Sel {
        Sai3Mclk3Sel::from_bits(val)
    }
}
impl From<Sai3Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai3Mclk3Sel) -> u8 {
        Sai3Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3MclkDir {
    #[doc = "sai3.MCLK is input signal"]
    SAI3_MCLK_DIR_0 = 0x0,
    #[doc = "sai3.MCLK is output signal"]
    SAI3_MCLK_DIR_1 = 0x01,
}
impl Sai3MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai3MclkDir {
        Sai3MclkDir::from_bits(val)
    }
}
impl From<Sai3MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai3MclkDir) -> u8 {
        Sai3MclkDir::to_bits(val)
    }
}
