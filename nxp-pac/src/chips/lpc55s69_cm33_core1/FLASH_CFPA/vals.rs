#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_CPU1_DBGEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_CPU1_DBGEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_CPU1_DBGEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_CPU1_DBGEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_CPU1_DBGEN {
        DCFG_CC_SOCU_PIN_CPU1_DBGEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_CPU1_DBGEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_CPU1_DBGEN) -> u8 {
        DCFG_CC_SOCU_PIN_CPU1_DBGEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_CPU1_NIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_CPU1_NIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_CPU1_NIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_CPU1_NIDEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_CPU1_NIDEN {
        DCFG_CC_SOCU_PIN_CPU1_NIDEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_CPU1_NIDEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_CPU1_NIDEN) -> u8 {
        DCFG_CC_SOCU_PIN_CPU1_NIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_DBGEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_DBGEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_DBGEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_DBGEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_DBGEN {
        DCFG_CC_SOCU_PIN_DBGEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_DBGEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_DBGEN) -> u8 {
        DCFG_CC_SOCU_PIN_DBGEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_FA_CMD_EN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_FA_CMD_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_FA_CMD_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_FA_CMD_EN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_FA_CMD_EN {
        DCFG_CC_SOCU_PIN_FA_CMD_EN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_FA_CMD_EN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_FA_CMD_EN) -> u8 {
        DCFG_CC_SOCU_PIN_FA_CMD_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_ISP_CMD_EN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_ISP_CMD_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_ISP_CMD_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_ISP_CMD_EN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_ISP_CMD_EN {
        DCFG_CC_SOCU_PIN_ISP_CMD_EN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_ISP_CMD_EN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_ISP_CMD_EN) -> u8 {
        DCFG_CC_SOCU_PIN_ISP_CMD_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_ME_CMD_EN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_ME_CMD_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_ME_CMD_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_ME_CMD_EN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_ME_CMD_EN {
        DCFG_CC_SOCU_PIN_ME_CMD_EN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_ME_CMD_EN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_ME_CMD_EN) -> u8 {
        DCFG_CC_SOCU_PIN_ME_CMD_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_NIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_NIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_NIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_NIDEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_NIDEN {
        DCFG_CC_SOCU_PIN_NIDEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_NIDEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_NIDEN) -> u8 {
        DCFG_CC_SOCU_PIN_NIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_SPIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_SPIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_SPIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_SPIDEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_SPIDEN {
        DCFG_CC_SOCU_PIN_SPIDEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_SPIDEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_SPIDEN) -> u8 {
        DCFG_CC_SOCU_PIN_SPIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_SPNIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_SPNIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_SPNIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_SPNIDEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_SPNIDEN {
        DCFG_CC_SOCU_PIN_SPNIDEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_SPNIDEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_SPNIDEN) -> u8 {
        DCFG_CC_SOCU_PIN_SPNIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCFG_CC_SOCU_PIN_TAPEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl DCFG_CC_SOCU_PIN_TAPEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCFG_CC_SOCU_PIN_TAPEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCFG_CC_SOCU_PIN_TAPEN {
    #[inline(always)]
    fn from(val: u8) -> DCFG_CC_SOCU_PIN_TAPEN {
        DCFG_CC_SOCU_PIN_TAPEN::from_bits(val)
    }
}
impl From<DCFG_CC_SOCU_PIN_TAPEN> for u8 {
    #[inline(always)]
    fn from(val: DCFG_CC_SOCU_PIN_TAPEN) -> u8 {
        DCFG_CC_SOCU_PIN_TAPEN::to_bits(val)
    }
}
