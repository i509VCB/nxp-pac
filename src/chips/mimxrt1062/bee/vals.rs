#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BeeEnable {
    #[doc = "Disable BEE"]
    BEE_ENABLE_0 = 0x0,
    #[doc = "Enable BEE"]
    BEE_ENABLE_1 = 0x01,
}
impl BeeEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BeeEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BeeEnable {
    #[inline(always)]
    fn from(val: u8) -> BeeEnable {
        BeeEnable::from_bits(val)
    }
}
impl From<BeeEnable> for u8 {
    #[inline(always)]
    fn from(val: BeeEnable) -> u8 {
        BeeEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlAesModeR0 {
    #[doc = "ECB"]
    CTRL_AES_MODE_R0_0 = 0x0,
    #[doc = "CTR"]
    CTRL_AES_MODE_R0_1 = 0x01,
}
impl CtrlAesModeR0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlAesModeR0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlAesModeR0 {
    #[inline(always)]
    fn from(val: u8) -> CtrlAesModeR0 {
        CtrlAesModeR0::from_bits(val)
    }
}
impl From<CtrlAesModeR0> for u8 {
    #[inline(always)]
    fn from(val: CtrlAesModeR0) -> u8 {
        CtrlAesModeR0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlAesModeR1 {
    #[doc = "ECB"]
    CTRL_AES_MODE_R1_0 = 0x0,
    #[doc = "CTR"]
    CTRL_AES_MODE_R1_1 = 0x01,
}
impl CtrlAesModeR1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlAesModeR1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlAesModeR1 {
    #[inline(always)]
    fn from(val: u8) -> CtrlAesModeR1 {
        CtrlAesModeR1::from_bits(val)
    }
}
impl From<CtrlAesModeR1> for u8 {
    #[inline(always)]
    fn from(val: CtrlAesModeR1) -> u8 {
        CtrlAesModeR1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyRegionSel {
    #[doc = "Load AES key for region0"]
    KEY_REGION_SEL_0 = 0x0,
    #[doc = "Load AES key for region1"]
    KEY_REGION_SEL_1 = 0x01,
}
impl KeyRegionSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyRegionSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyRegionSel {
    #[inline(always)]
    fn from(val: u8) -> KeyRegionSel {
        KeyRegionSel::from_bits(val)
    }
}
impl From<KeyRegionSel> for u8 {
    #[inline(always)]
    fn from(val: KeyRegionSel) -> u8 {
        KeyRegionSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LittleEndian {
    #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    LITTLE_ENDIAN_0 = 0x0,
    #[doc = "The input and output data of AES core is not swapped."]
    LITTLE_ENDIAN_1 = 0x01,
}
impl LittleEndian {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LittleEndian {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LittleEndian {
    #[inline(always)]
    fn from(val: u8) -> LittleEndian {
        LittleEndian::from_bits(val)
    }
}
impl From<LittleEndian> for u8 {
    #[inline(always)]
    fn from(val: LittleEndian) -> u8 {
        LittleEndian::to_bits(val)
    }
}
