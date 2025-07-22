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
