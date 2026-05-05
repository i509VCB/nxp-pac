#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADC_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl ADC_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADC_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADC_RULE {
    #[inline(always)]
    fn from(val: u8) -> ADC_RULE {
        ADC_RULE::from_bits(val)
    }
}
impl From<ADC_RULE> for u8 {
    #[inline(always)]
    fn from(val: ADC_RULE) -> u8 {
        ADC_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_SEC_CTRL_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AHB_SEC_CTRL_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_SEC_CTRL_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_SEC_CTRL_RULE {
    #[inline(always)]
    fn from(val: u8) -> AHB_SEC_CTRL_RULE {
        AHB_SEC_CTRL_RULE::from_bits(val)
    }
}
impl From<AHB_SEC_CTRL_RULE> for u8 {
    #[inline(always)]
    fn from(val: AHB_SEC_CTRL_RULE) -> u8 {
        AHB_SEC_CTRL_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_SEC_CTRL_SECT_0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AHB_SEC_CTRL_SECT_0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_SEC_CTRL_SECT_0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_SEC_CTRL_SECT_0_RULE {
    #[inline(always)]
    fn from(val: u8) -> AHB_SEC_CTRL_SECT_0_RULE {
        AHB_SEC_CTRL_SECT_0_RULE::from_bits(val)
    }
}
impl From<AHB_SEC_CTRL_SECT_0_RULE> for u8 {
    #[inline(always)]
    fn from(val: AHB_SEC_CTRL_SECT_0_RULE) -> u8 {
        AHB_SEC_CTRL_SECT_0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_SEC_CTRL_SECT_1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AHB_SEC_CTRL_SECT_1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_SEC_CTRL_SECT_1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_SEC_CTRL_SECT_1_RULE {
    #[inline(always)]
    fn from(val: u8) -> AHB_SEC_CTRL_SECT_1_RULE {
        AHB_SEC_CTRL_SECT_1_RULE::from_bits(val)
    }
}
impl From<AHB_SEC_CTRL_SECT_1_RULE> for u8 {
    #[inline(always)]
    fn from(val: AHB_SEC_CTRL_SECT_1_RULE) -> u8 {
        AHB_SEC_CTRL_SECT_1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_SEC_CTRL_SECT_2_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AHB_SEC_CTRL_SECT_2_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_SEC_CTRL_SECT_2_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_SEC_CTRL_SECT_2_RULE {
    #[inline(always)]
    fn from(val: u8) -> AHB_SEC_CTRL_SECT_2_RULE {
        AHB_SEC_CTRL_SECT_2_RULE::from_bits(val)
    }
}
impl From<AHB_SEC_CTRL_SECT_2_RULE> for u8 {
    #[inline(always)]
    fn from(val: AHB_SEC_CTRL_SECT_2_RULE) -> u8 {
        AHB_SEC_CTRL_SECT_2_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_SEC_CTRL_SECT_3_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl AHB_SEC_CTRL_SECT_3_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_SEC_CTRL_SECT_3_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_SEC_CTRL_SECT_3_RULE {
    #[inline(always)]
    fn from(val: u8) -> AHB_SEC_CTRL_SECT_3_RULE {
        AHB_SEC_CTRL_SECT_3_RULE::from_bits(val)
    }
}
impl From<AHB_SEC_CTRL_SECT_3_RULE> for u8 {
    #[inline(always)]
    fn from(val: AHB_SEC_CTRL_SECT_3_RULE) -> u8 {
        AHB_SEC_CTRL_SECT_3_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ANACTRL_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl ANACTRL_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ANACTRL_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ANACTRL_RULE {
    #[inline(always)]
    fn from(val: u8) -> ANACTRL_RULE {
        ANACTRL_RULE::from_bits(val)
    }
}
impl From<ANACTRL_RULE> for u8 {
    #[inline(always)]
    fn from(val: ANACTRL_RULE) -> u8 {
        ANACTRL_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum APBBRIDGE0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl APBBRIDGE0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> APBBRIDGE0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for APBBRIDGE0_RULE {
    #[inline(always)]
    fn from(val: u8) -> APBBRIDGE0_RULE {
        APBBRIDGE0_RULE::from_bits(val)
    }
}
impl From<APBBRIDGE0_RULE> for u8 {
    #[inline(always)]
    fn from(val: APBBRIDGE0_RULE) -> u8 {
        APBBRIDGE0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum APBBRIDGE1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl APBBRIDGE1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> APBBRIDGE1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for APBBRIDGE1_RULE {
    #[inline(always)]
    fn from(val: u8) -> APBBRIDGE1_RULE {
        APBBRIDGE1_RULE::from_bits(val)
    }
}
impl From<APBBRIDGE1_RULE> for u8 {
    #[inline(always)]
    fn from(val: APBBRIDGE1_RULE) -> u8 {
        APBBRIDGE1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CASPER_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CASPER_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CASPER_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CASPER_RULE {
    #[inline(always)]
    fn from(val: u8) -> CASPER_RULE {
        CASPER_RULE::from_bits(val)
    }
}
impl From<CASPER_RULE> for u8 {
    #[inline(always)]
    fn from(val: CASPER_RULE) -> u8 {
        CASPER_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU0_LOCK_REG_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl CPU0_LOCK_REG_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU0_LOCK_REG_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU0_LOCK_REG_LOCK {
    #[inline(always)]
    fn from(val: u8) -> CPU0_LOCK_REG_LOCK {
        CPU0_LOCK_REG_LOCK::from_bits(val)
    }
}
impl From<CPU0_LOCK_REG_LOCK> for u8 {
    #[inline(always)]
    fn from(val: CPU0_LOCK_REG_LOCK) -> u8 {
        CPU0_LOCK_REG_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU0_LOCK_REG_LOCK_NS_MPU {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl CPU0_LOCK_REG_LOCK_NS_MPU {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU0_LOCK_REG_LOCK_NS_MPU {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU0_LOCK_REG_LOCK_NS_MPU {
    #[inline(always)]
    fn from(val: u8) -> CPU0_LOCK_REG_LOCK_NS_MPU {
        CPU0_LOCK_REG_LOCK_NS_MPU::from_bits(val)
    }
}
impl From<CPU0_LOCK_REG_LOCK_NS_MPU> for u8 {
    #[inline(always)]
    fn from(val: CPU0_LOCK_REG_LOCK_NS_MPU) -> u8 {
        CPU0_LOCK_REG_LOCK_NS_MPU::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU0_LOCK_REG_LOCK_NS_VTOR {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl CPU0_LOCK_REG_LOCK_NS_VTOR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU0_LOCK_REG_LOCK_NS_VTOR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU0_LOCK_REG_LOCK_NS_VTOR {
    #[inline(always)]
    fn from(val: u8) -> CPU0_LOCK_REG_LOCK_NS_VTOR {
        CPU0_LOCK_REG_LOCK_NS_VTOR::from_bits(val)
    }
}
impl From<CPU0_LOCK_REG_LOCK_NS_VTOR> for u8 {
    #[inline(always)]
    fn from(val: CPU0_LOCK_REG_LOCK_NS_VTOR) -> u8 {
        CPU0_LOCK_REG_LOCK_NS_VTOR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU1_LOCK_REG_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl CPU1_LOCK_REG_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU1_LOCK_REG_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU1_LOCK_REG_LOCK {
    #[inline(always)]
    fn from(val: u8) -> CPU1_LOCK_REG_LOCK {
        CPU1_LOCK_REG_LOCK::from_bits(val)
    }
}
impl From<CPU1_LOCK_REG_LOCK> for u8 {
    #[inline(always)]
    fn from(val: CPU1_LOCK_REG_LOCK) -> u8 {
        CPU1_LOCK_REG_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU1_LOCK_REG_LOCK_NS_MPU {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl CPU1_LOCK_REG_LOCK_NS_MPU {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU1_LOCK_REG_LOCK_NS_MPU {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU1_LOCK_REG_LOCK_NS_MPU {
    #[inline(always)]
    fn from(val: u8) -> CPU1_LOCK_REG_LOCK_NS_MPU {
        CPU1_LOCK_REG_LOCK_NS_MPU::from_bits(val)
    }
}
impl From<CPU1_LOCK_REG_LOCK_NS_MPU> for u8 {
    #[inline(always)]
    fn from(val: CPU1_LOCK_REG_LOCK_NS_MPU) -> u8 {
        CPU1_LOCK_REG_LOCK_NS_MPU::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU1_LOCK_REG_LOCK_NS_VTOR {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl CPU1_LOCK_REG_LOCK_NS_VTOR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU1_LOCK_REG_LOCK_NS_VTOR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU1_LOCK_REG_LOCK_NS_VTOR {
    #[inline(always)]
    fn from(val: u8) -> CPU1_LOCK_REG_LOCK_NS_VTOR {
        CPU1_LOCK_REG_LOCK_NS_VTOR::from_bits(val)
    }
}
impl From<CPU1_LOCK_REG_LOCK_NS_VTOR> for u8 {
    #[inline(always)]
    fn from(val: CPU1_LOCK_REG_LOCK_NS_VTOR) -> u8 {
        CPU1_LOCK_REG_LOCK_NS_VTOR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CRC_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CRC_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CRC_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CRC_RULE {
    #[inline(always)]
    fn from(val: u8) -> CRC_RULE {
        CRC_RULE::from_bits(val)
    }
}
impl From<CRC_RULE> for u8 {
    #[inline(always)]
    fn from(val: CRC_RULE) -> u8 {
        CRC_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMER0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CTIMER0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMER0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMER0_RULE {
    #[inline(always)]
    fn from(val: u8) -> CTIMER0_RULE {
        CTIMER0_RULE::from_bits(val)
    }
}
impl From<CTIMER0_RULE> for u8 {
    #[inline(always)]
    fn from(val: CTIMER0_RULE) -> u8 {
        CTIMER0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMER1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CTIMER1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMER1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMER1_RULE {
    #[inline(always)]
    fn from(val: u8) -> CTIMER1_RULE {
        CTIMER1_RULE::from_bits(val)
    }
}
impl From<CTIMER1_RULE> for u8 {
    #[inline(always)]
    fn from(val: CTIMER1_RULE) -> u8 {
        CTIMER1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMER2_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CTIMER2_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMER2_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMER2_RULE {
    #[inline(always)]
    fn from(val: u8) -> CTIMER2_RULE {
        CTIMER2_RULE::from_bits(val)
    }
}
impl From<CTIMER2_RULE> for u8 {
    #[inline(always)]
    fn from(val: CTIMER2_RULE) -> u8 {
        CTIMER2_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMER3_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CTIMER3_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMER3_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMER3_RULE {
    #[inline(always)]
    fn from(val: u8) -> CTIMER3_RULE {
        CTIMER3_RULE::from_bits(val)
    }
}
impl From<CTIMER3_RULE> for u8 {
    #[inline(always)]
    fn from(val: CTIMER3_RULE) -> u8 {
        CTIMER3_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMER4_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl CTIMER4_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMER4_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMER4_RULE {
    #[inline(always)]
    fn from(val: u8) -> CTIMER4_RULE {
        CTIMER4_RULE::from_bits(val)
    }
}
impl From<CTIMER4_RULE> for u8 {
    #[inline(always)]
    fn from(val: CTIMER4_RULE) -> u8 {
        CTIMER4_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DBG_MAILBOX_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl DBG_MAILBOX_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DBG_MAILBOX_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DBG_MAILBOX_RULE {
    #[inline(always)]
    fn from(val: u8) -> DBG_MAILBOX_RULE {
        DBG_MAILBOX_RULE::from_bits(val)
    }
}
impl From<DBG_MAILBOX_RULE> for u8 {
    #[inline(always)]
    fn from(val: DBG_MAILBOX_RULE) -> u8 {
        DBG_MAILBOX_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl DMA0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA0_RULE {
    #[inline(always)]
    fn from(val: u8) -> DMA0_RULE {
        DMA0_RULE::from_bits(val)
    }
}
impl From<DMA0_RULE> for u8 {
    #[inline(always)]
    fn from(val: DMA0_RULE) -> u8 {
        DMA0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl DMA1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA1_RULE {
    #[inline(always)]
    fn from(val: u8) -> DMA1_RULE {
        DMA1_RULE::from_bits(val)
    }
}
impl From<DMA1_RULE> for u8 {
    #[inline(always)]
    fn from(val: DMA1_RULE) -> u8 {
        DMA1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASH_CTRL_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLASH_CTRL_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASH_CTRL_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASH_CTRL_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLASH_CTRL_RULE {
        FLASH_CTRL_RULE::from_bits(val)
    }
}
impl From<FLASH_CTRL_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLASH_CTRL_RULE) -> u8 {
        FLASH_CTRL_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASH_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLASH_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASH_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASH_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLASH_RULE {
        FLASH_RULE::from_bits(val)
    }
}
impl From<FLASH_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLASH_RULE) -> u8 {
        FLASH_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM0_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM0_RULE {
        FLEXCOMM0_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM0_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM0_RULE) -> u8 {
        FLEXCOMM0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM1_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM1_RULE {
        FLEXCOMM1_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM1_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM1_RULE) -> u8 {
        FLEXCOMM1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM2_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM2_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM2_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM2_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM2_RULE {
        FLEXCOMM2_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM2_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM2_RULE) -> u8 {
        FLEXCOMM2_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM3_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM3_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM3_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM3_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM3_RULE {
        FLEXCOMM3_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM3_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM3_RULE) -> u8 {
        FLEXCOMM3_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM4_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM4_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM4_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM4_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM4_RULE {
        FLEXCOMM4_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM4_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM4_RULE) -> u8 {
        FLEXCOMM4_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM5_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM5_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM5_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM5_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM5_RULE {
        FLEXCOMM5_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM5_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM5_RULE) -> u8 {
        FLEXCOMM5_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM6_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM6_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM6_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM6_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM6_RULE {
        FLEXCOMM6_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM6_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM6_RULE) -> u8 {
        FLEXCOMM6_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLEXCOMM7_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FLEXCOMM7_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLEXCOMM7_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLEXCOMM7_RULE {
    #[inline(always)]
    fn from(val: u8) -> FLEXCOMM7_RULE {
        FLEXCOMM7_RULE::from_bits(val)
    }
}
impl From<FLEXCOMM7_RULE> for u8 {
    #[inline(always)]
    fn from(val: FLEXCOMM7_RULE) -> u8 {
        FLEXCOMM7_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FS_USB_DEV_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl FS_USB_DEV_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FS_USB_DEV_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FS_USB_DEV_RULE {
    #[inline(always)]
    fn from(val: u8) -> FS_USB_DEV_RULE {
        FS_USB_DEV_RULE::from_bits(val)
    }
}
impl From<FS_USB_DEV_RULE> for u8 {
    #[inline(always)]
    fn from(val: FS_USB_DEV_RULE) -> u8 {
        FS_USB_DEV_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GINT0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl GINT0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GINT0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GINT0_RULE {
    #[inline(always)]
    fn from(val: u8) -> GINT0_RULE {
        GINT0_RULE::from_bits(val)
    }
}
impl From<GINT0_RULE> for u8 {
    #[inline(always)]
    fn from(val: GINT0_RULE) -> u8 {
        GINT0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GINT1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl GINT1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GINT1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GINT1_RULE {
    #[inline(always)]
    fn from(val: u8) -> GINT1_RULE {
        GINT1_RULE::from_bits(val)
    }
}
impl From<GINT1_RULE> for u8 {
    #[inline(always)]
    fn from(val: GINT1_RULE) -> u8 {
        GINT1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl GPIO0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO0_RULE {
    #[inline(always)]
    fn from(val: u8) -> GPIO0_RULE {
        GPIO0_RULE::from_bits(val)
    }
}
impl From<GPIO0_RULE> for u8 {
    #[inline(always)]
    fn from(val: GPIO0_RULE) -> u8 {
        GPIO0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl GPIO1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO1_RULE {
    #[inline(always)]
    fn from(val: u8) -> GPIO1_RULE {
        GPIO1_RULE::from_bits(val)
    }
}
impl From<GPIO1_RULE> for u8 {
    #[inline(always)]
    fn from(val: GPIO1_RULE) -> u8 {
        GPIO1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HASH_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl HASH_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HASH_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HASH_RULE {
    #[inline(always)]
    fn from(val: u8) -> HASH_RULE {
        HASH_RULE::from_bits(val)
    }
}
impl From<HASH_RULE> for u8 {
    #[inline(always)]
    fn from(val: HASH_RULE) -> u8 {
        HASH_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HS_LSPI_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl HS_LSPI_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HS_LSPI_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HS_LSPI_RULE {
    #[inline(always)]
    fn from(val: u8) -> HS_LSPI_RULE {
        HS_LSPI_RULE::from_bits(val)
    }
}
impl From<HS_LSPI_RULE> for u8 {
    #[inline(always)]
    fn from(val: HS_LSPI_RULE) -> u8 {
        HS_LSPI_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INPUTMUX_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl INPUTMUX_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INPUTMUX_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INPUTMUX_RULE {
    #[inline(always)]
    fn from(val: u8) -> INPUTMUX_RULE {
        INPUTMUX_RULE::from_bits(val)
    }
}
impl From<INPUTMUX_RULE> for u8 {
    #[inline(always)]
    fn from(val: INPUTMUX_RULE) -> u8 {
        INPUTMUX_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCON_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl IOCON_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCON_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCON_RULE {
    #[inline(always)]
    fn from(val: u8) -> IOCON_RULE {
        IOCON_RULE::from_bits(val)
    }
}
impl From<IOCON_RULE> for u8 {
    #[inline(always)]
    fn from(val: IOCON_RULE) -> u8 {
        IOCON_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOCK_SAU {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LOCK_SAU {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOCK_SAU {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOCK_SAU {
    #[inline(always)]
    fn from(val: u8) -> LOCK_SAU {
        LOCK_SAU::from_bits(val)
    }
}
impl From<LOCK_SAU> for u8 {
    #[inline(always)]
    fn from(val: LOCK_SAU) -> u8 {
        LOCK_SAU::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOCK_S_MPU {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LOCK_S_MPU {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOCK_S_MPU {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOCK_S_MPU {
    #[inline(always)]
    fn from(val: u8) -> LOCK_S_MPU {
        LOCK_S_MPU::from_bits(val)
    }
}
impl From<LOCK_S_MPU> for u8 {
    #[inline(always)]
    fn from(val: LOCK_S_MPU) -> u8 {
        LOCK_S_MPU::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOCK_S_VTAIRCR {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl LOCK_S_VTAIRCR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOCK_S_VTAIRCR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOCK_S_VTAIRCR {
    #[inline(always)]
    fn from(val: u8) -> LOCK_S_VTAIRCR {
        LOCK_S_VTAIRCR::from_bits(val)
    }
}
impl From<LOCK_S_VTAIRCR> for u8 {
    #[inline(always)]
    fn from(val: LOCK_S_VTAIRCR) -> u8 {
        LOCK_S_VTAIRCR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAILBOX_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MAILBOX_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAILBOX_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAILBOX_RULE {
    #[inline(always)]
    fn from(val: u8) -> MAILBOX_RULE {
        MAILBOX_RULE::from_bits(val)
    }
}
impl From<MAILBOX_RULE> for u8 {
    #[inline(always)]
    fn from(val: MAILBOX_RULE) -> u8 {
        MAILBOX_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_CPU1C {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_CPU1C {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_CPU1C {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_CPU1C {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_CPU1C {
        MASTER_SEC_ANTI_POL_REG_CPU1C::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_CPU1C> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_CPU1C) -> u8 {
        MASTER_SEC_ANTI_POL_REG_CPU1C::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_CPU1S {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_CPU1S {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_CPU1S {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_CPU1S {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_CPU1S {
        MASTER_SEC_ANTI_POL_REG_CPU1S::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_CPU1S> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_CPU1S) -> u8 {
        MASTER_SEC_ANTI_POL_REG_CPU1S::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_HASH {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_HASH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_HASH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_HASH {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_HASH {
        MASTER_SEC_ANTI_POL_REG_HASH::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_HASH> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_HASH) -> u8 {
        MASTER_SEC_ANTI_POL_REG_HASH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_PQ {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_PQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_PQ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_PQ {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_PQ {
        MASTER_SEC_ANTI_POL_REG_PQ::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_PQ> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_PQ) -> u8 {
        MASTER_SEC_ANTI_POL_REG_PQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_SDIO {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_SDIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_SDIO {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_SDIO {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_SDIO {
        MASTER_SEC_ANTI_POL_REG_SDIO::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_SDIO> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_SDIO) -> u8 {
        MASTER_SEC_ANTI_POL_REG_SDIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_SDMA0 {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_SDMA0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_SDMA0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_SDMA0 {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_SDMA0 {
        MASTER_SEC_ANTI_POL_REG_SDMA0::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_SDMA0> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_SDMA0) -> u8 {
        MASTER_SEC_ANTI_POL_REG_SDMA0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_SDMA1 {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_SDMA1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_SDMA1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_SDMA1 {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_SDMA1 {
        MASTER_SEC_ANTI_POL_REG_SDMA1::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_SDMA1> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_SDMA1) -> u8 {
        MASTER_SEC_ANTI_POL_REG_SDMA1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_USBFSD {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_USBFSD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_USBFSD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_USBFSD {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_USBFSD {
        MASTER_SEC_ANTI_POL_REG_USBFSD::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_USBFSD> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_USBFSD) -> u8 {
        MASTER_SEC_ANTI_POL_REG_USBFSD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_ANTI_POL_REG_USBFSH {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x0,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x01,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x02,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x03,
}
impl MASTER_SEC_ANTI_POL_REG_USBFSH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_ANTI_POL_REG_USBFSH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_ANTI_POL_REG_USBFSH {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_ANTI_POL_REG_USBFSH {
        MASTER_SEC_ANTI_POL_REG_USBFSH::from_bits(val)
    }
}
impl From<MASTER_SEC_ANTI_POL_REG_USBFSH> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_ANTI_POL_REG_USBFSH) -> u8 {
        MASTER_SEC_ANTI_POL_REG_USBFSH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_ANTIPOL_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MASTER_SEC_LEVEL_ANTIPOL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_ANTIPOL_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_ANTIPOL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_ANTIPOL_LOCK {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_ANTIPOL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_ANTIPOL_LOCK) -> u8 {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_CPU1C {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_CPU1C {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_CPU1C {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_CPU1C {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_CPU1C {
        MASTER_SEC_LEVEL_CPU1C::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_CPU1C> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_CPU1C) -> u8 {
        MASTER_SEC_LEVEL_CPU1C::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_CPU1S {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_CPU1S {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_CPU1S {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_CPU1S {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_CPU1S {
        MASTER_SEC_LEVEL_CPU1S::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_CPU1S> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_CPU1S) -> u8 {
        MASTER_SEC_LEVEL_CPU1S::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_HASH {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_HASH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_HASH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_HASH {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_HASH {
        MASTER_SEC_LEVEL_HASH::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_HASH> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_HASH) -> u8 {
        MASTER_SEC_LEVEL_HASH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MASTER_SEC_LEVEL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_LOCK {
        MASTER_SEC_LEVEL_LOCK::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_LOCK) -> u8 {
        MASTER_SEC_LEVEL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_PQ {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_PQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_PQ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_PQ {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_PQ {
        MASTER_SEC_LEVEL_PQ::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_PQ> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_PQ) -> u8 {
        MASTER_SEC_LEVEL_PQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_SDIO {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_SDIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_SDIO {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_SDIO {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_SDIO {
        MASTER_SEC_LEVEL_SDIO::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_SDIO> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_SDIO) -> u8 {
        MASTER_SEC_LEVEL_SDIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_SDMA0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_SDMA0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_SDMA0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_SDMA0 {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_SDMA0 {
        MASTER_SEC_LEVEL_SDMA0::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_SDMA0> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_SDMA0) -> u8 {
        MASTER_SEC_LEVEL_SDMA0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_SDMA1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_SDMA1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_SDMA1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_SDMA1 {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_SDMA1 {
        MASTER_SEC_LEVEL_SDMA1::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_SDMA1> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_SDMA1) -> u8 {
        MASTER_SEC_LEVEL_SDMA1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_USBFSD {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_USBFSD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_USBFSD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_USBFSD {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_USBFSD {
        MASTER_SEC_LEVEL_USBFSD::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_USBFSD> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_USBFSD) -> u8 {
        MASTER_SEC_LEVEL_USBFSD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER_SEC_LEVEL_USBFSH {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MASTER_SEC_LEVEL_USBFSH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER_SEC_LEVEL_USBFSH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER_SEC_LEVEL_USBFSH {
    #[inline(always)]
    fn from(val: u8) -> MASTER_SEC_LEVEL_USBFSH {
        MASTER_SEC_LEVEL_USBFSH::from_bits(val)
    }
}
impl From<MASTER_SEC_LEVEL_USBFSH> for u8 {
    #[inline(always)]
    fn from(val: MASTER_SEC_LEVEL_USBFSH) -> u8 {
        MASTER_SEC_LEVEL_USBFSH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
    _RESERVED_0 = 0x0,
    #[doc = "Simple master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
        MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE) -> u8 {
        MISC_CTRL_DP_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE {
    _RESERVED_0 = 0x0,
    #[doc = "Smart master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE {
        MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE) -> u8 {
        MISC_CTRL_DP_REG_DISABLE_SMART_MASTER_STRICT_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT {
    _RESERVED_0 = 0x0,
    #[doc = "Disable abort fort secure checker."]
    DISABLE = 0x01,
    #[doc = "Enable abort fort secure checker."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT {
        MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT) -> u8 {
        MISC_CTRL_DP_REG_DISABLE_VIOLATION_ABORT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK {
        MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK) -> u8 {
        MISC_CTRL_DP_REG_ENABLE_NS_PRIV_CHECK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING {
        MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING) -> u8 {
        MISC_CTRL_DP_REG_ENABLE_SECURE_CHECKING::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK {
        MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK) -> u8 {
        MISC_CTRL_DP_REG_ENABLE_S_PRIV_CHECK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_IDAU_ALL_NS {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disable."]
    DISABLE = 0x01,
    #[doc = "IDAU is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_IDAU_ALL_NS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_IDAU_ALL_NS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_IDAU_ALL_NS {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_IDAU_ALL_NS {
        MISC_CTRL_DP_REG_IDAU_ALL_NS::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_IDAU_ALL_NS> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_IDAU_ALL_NS) -> u8 {
        MISC_CTRL_DP_REG_IDAU_ALL_NS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_DP_REG_WRITE_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    RESTRICTED = 0x01,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_DP_REG_WRITE_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_DP_REG_WRITE_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_DP_REG_WRITE_LOCK {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_DP_REG_WRITE_LOCK {
        MISC_CTRL_DP_REG_WRITE_LOCK::from_bits(val)
    }
}
impl From<MISC_CTRL_DP_REG_WRITE_LOCK> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_DP_REG_WRITE_LOCK) -> u8 {
        MISC_CTRL_DP_REG_WRITE_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
    _RESERVED_0 = 0x0,
    #[doc = "Simple master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE {
        MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE) -> u8 {
        MISC_CTRL_REG_DISABLE_SIMPLE_MASTER_STRICT_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE {
    _RESERVED_0 = 0x0,
    #[doc = "Smart master in tier mode."]
    TIER_MODE = 0x01,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE {
        MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE) -> u8 {
        MISC_CTRL_REG_DISABLE_SMART_MASTER_STRICT_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_DISABLE_VIOLATION_ABORT {
    _RESERVED_0 = 0x0,
    #[doc = "Disable abort fort secure checker."]
    DISABLE = 0x01,
    #[doc = "Enable abort fort secure checker."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_DISABLE_VIOLATION_ABORT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_DISABLE_VIOLATION_ABORT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_DISABLE_VIOLATION_ABORT {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_DISABLE_VIOLATION_ABORT {
        MISC_CTRL_REG_DISABLE_VIOLATION_ABORT::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_DISABLE_VIOLATION_ABORT> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_DISABLE_VIOLATION_ABORT) -> u8 {
        MISC_CTRL_REG_DISABLE_VIOLATION_ABORT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK {
    _RESERVED_0 = 0x0,
    #[doc = "Enabled (restricted mode)."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK {
        MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK) -> u8 {
        MISC_CTRL_REG_ENABLE_NS_PRIV_CHECK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_ENABLE_SECURE_CHECKING {
    _RESERVED_0 = 0x0,
    #[doc = "Enabled (restricted mode)."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_ENABLE_SECURE_CHECKING {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_ENABLE_SECURE_CHECKING {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_ENABLE_SECURE_CHECKING {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_ENABLE_SECURE_CHECKING {
        MISC_CTRL_REG_ENABLE_SECURE_CHECKING::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_ENABLE_SECURE_CHECKING> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_ENABLE_SECURE_CHECKING) -> u8 {
        MISC_CTRL_REG_ENABLE_SECURE_CHECKING::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_ENABLE_S_PRIV_CHECK {
    _RESERVED_0 = 0x0,
    #[doc = "Enabled (restricted mode)."]
    ENABLE = 0x01,
    #[doc = "Disable check."]
    DISABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_ENABLE_S_PRIV_CHECK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_ENABLE_S_PRIV_CHECK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_ENABLE_S_PRIV_CHECK {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_ENABLE_S_PRIV_CHECK {
        MISC_CTRL_REG_ENABLE_S_PRIV_CHECK::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_ENABLE_S_PRIV_CHECK> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_ENABLE_S_PRIV_CHECK) -> u8 {
        MISC_CTRL_REG_ENABLE_S_PRIV_CHECK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_IDAU_ALL_NS {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disable."]
    DISABLE = 0x01,
    #[doc = "IDAU is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_IDAU_ALL_NS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_IDAU_ALL_NS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_IDAU_ALL_NS {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_IDAU_ALL_NS {
        MISC_CTRL_REG_IDAU_ALL_NS::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_IDAU_ALL_NS> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_IDAU_ALL_NS) -> u8 {
        MISC_CTRL_REG_IDAU_ALL_NS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC_CTRL_REG_WRITE_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    RESTRICTED = 0x01,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MISC_CTRL_REG_WRITE_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC_CTRL_REG_WRITE_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC_CTRL_REG_WRITE_LOCK {
    #[inline(always)]
    fn from(val: u8) -> MISC_CTRL_REG_WRITE_LOCK {
        MISC_CTRL_REG_WRITE_LOCK::from_bits(val)
    }
}
impl From<MISC_CTRL_REG_WRITE_LOCK> for u8 {
    #[inline(always)]
    fn from(val: MISC_CTRL_REG_WRITE_LOCK) -> u8 {
        MISC_CTRL_REG_WRITE_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MRT_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl MRT_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MRT_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MRT_RULE {
    #[inline(always)]
    fn from(val: u8) -> MRT_RULE {
        MRT_RULE::from_bits(val)
    }
}
impl From<MRT_RULE> for u8 {
    #[inline(always)]
    fn from(val: MRT_RULE) -> u8 {
        MRT_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OSEVENT_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl OSEVENT_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OSEVENT_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OSEVENT_RULE {
    #[inline(always)]
    fn from(val: u8) -> OSEVENT_RULE {
        OSEVENT_RULE::from_bits(val)
    }
}
impl From<OSEVENT_RULE> for u8 {
    #[inline(always)]
    fn from(val: OSEVENT_RULE) -> u8 {
        OSEVENT_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PINT_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PINT_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PINT_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PINT_RULE {
    #[inline(always)]
    fn from(val: u8) -> PINT_RULE {
        PINT_RULE::from_bits(val)
    }
}
impl From<PINT_RULE> for u8 {
    #[inline(always)]
    fn from(val: PINT_RULE) -> u8 {
        PINT_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN0_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN0_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN0_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN0_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN0_SEC_MASK {
        PIO0_PIN0_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN0_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN0_SEC_MASK) -> u8 {
        PIO0_PIN0_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN10_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN10_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN10_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN10_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN10_SEC_MASK {
        PIO0_PIN10_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN10_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN10_SEC_MASK) -> u8 {
        PIO0_PIN10_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN11_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN11_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN11_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN11_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN11_SEC_MASK {
        PIO0_PIN11_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN11_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN11_SEC_MASK) -> u8 {
        PIO0_PIN11_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN12_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN12_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN12_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN12_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN12_SEC_MASK {
        PIO0_PIN12_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN12_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN12_SEC_MASK) -> u8 {
        PIO0_PIN12_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN13_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN13_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN13_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN13_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN13_SEC_MASK {
        PIO0_PIN13_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN13_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN13_SEC_MASK) -> u8 {
        PIO0_PIN13_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN14_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN14_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN14_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN14_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN14_SEC_MASK {
        PIO0_PIN14_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN14_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN14_SEC_MASK) -> u8 {
        PIO0_PIN14_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN15_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN15_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN15_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN15_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN15_SEC_MASK {
        PIO0_PIN15_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN15_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN15_SEC_MASK) -> u8 {
        PIO0_PIN15_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN16_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN16_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN16_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN16_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN16_SEC_MASK {
        PIO0_PIN16_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN16_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN16_SEC_MASK) -> u8 {
        PIO0_PIN16_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN17_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN17_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN17_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN17_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN17_SEC_MASK {
        PIO0_PIN17_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN17_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN17_SEC_MASK) -> u8 {
        PIO0_PIN17_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN18_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN18_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN18_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN18_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN18_SEC_MASK {
        PIO0_PIN18_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN18_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN18_SEC_MASK) -> u8 {
        PIO0_PIN18_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN19_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN19_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN19_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN19_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN19_SEC_MASK {
        PIO0_PIN19_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN19_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN19_SEC_MASK) -> u8 {
        PIO0_PIN19_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN1_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN1_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN1_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN1_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN1_SEC_MASK {
        PIO0_PIN1_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN1_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN1_SEC_MASK) -> u8 {
        PIO0_PIN1_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN20_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN20_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN20_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN20_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN20_SEC_MASK {
        PIO0_PIN20_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN20_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN20_SEC_MASK) -> u8 {
        PIO0_PIN20_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN21_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN21_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN21_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN21_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN21_SEC_MASK {
        PIO0_PIN21_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN21_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN21_SEC_MASK) -> u8 {
        PIO0_PIN21_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN22_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN22_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN22_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN22_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN22_SEC_MASK {
        PIO0_PIN22_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN22_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN22_SEC_MASK) -> u8 {
        PIO0_PIN22_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN23_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN23_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN23_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN23_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN23_SEC_MASK {
        PIO0_PIN23_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN23_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN23_SEC_MASK) -> u8 {
        PIO0_PIN23_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN24_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN24_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN24_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN24_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN24_SEC_MASK {
        PIO0_PIN24_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN24_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN24_SEC_MASK) -> u8 {
        PIO0_PIN24_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN25_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN25_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN25_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN25_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN25_SEC_MASK {
        PIO0_PIN25_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN25_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN25_SEC_MASK) -> u8 {
        PIO0_PIN25_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN26_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN26_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN26_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN26_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN26_SEC_MASK {
        PIO0_PIN26_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN26_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN26_SEC_MASK) -> u8 {
        PIO0_PIN26_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN27_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN27_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN27_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN27_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN27_SEC_MASK {
        PIO0_PIN27_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN27_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN27_SEC_MASK) -> u8 {
        PIO0_PIN27_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN28_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN28_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN28_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN28_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN28_SEC_MASK {
        PIO0_PIN28_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN28_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN28_SEC_MASK) -> u8 {
        PIO0_PIN28_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN29_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN29_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN29_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN29_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN29_SEC_MASK {
        PIO0_PIN29_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN29_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN29_SEC_MASK) -> u8 {
        PIO0_PIN29_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN2_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN2_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN2_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN2_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN2_SEC_MASK {
        PIO0_PIN2_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN2_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN2_SEC_MASK) -> u8 {
        PIO0_PIN2_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN30_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN30_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN30_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN30_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN30_SEC_MASK {
        PIO0_PIN30_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN30_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN30_SEC_MASK) -> u8 {
        PIO0_PIN30_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN31_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN31_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN31_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN31_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN31_SEC_MASK {
        PIO0_PIN31_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN31_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN31_SEC_MASK) -> u8 {
        PIO0_PIN31_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN3_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN3_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN3_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN3_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN3_SEC_MASK {
        PIO0_PIN3_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN3_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN3_SEC_MASK) -> u8 {
        PIO0_PIN3_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN4_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN4_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN4_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN4_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN4_SEC_MASK {
        PIO0_PIN4_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN4_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN4_SEC_MASK) -> u8 {
        PIO0_PIN4_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN5_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN5_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN5_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN5_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN5_SEC_MASK {
        PIO0_PIN5_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN5_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN5_SEC_MASK) -> u8 {
        PIO0_PIN5_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN6_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN6_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN6_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN6_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN6_SEC_MASK {
        PIO0_PIN6_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN6_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN6_SEC_MASK) -> u8 {
        PIO0_PIN6_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN7_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN7_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN7_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN7_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN7_SEC_MASK {
        PIO0_PIN7_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN7_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN7_SEC_MASK) -> u8 {
        PIO0_PIN7_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN8_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN8_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN8_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN8_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN8_SEC_MASK {
        PIO0_PIN8_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN8_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN8_SEC_MASK) -> u8 {
        PIO0_PIN8_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_PIN9_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO0_PIN9_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_PIN9_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_PIN9_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO0_PIN9_SEC_MASK {
        PIO0_PIN9_SEC_MASK::from_bits(val)
    }
}
impl From<PIO0_PIN9_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO0_PIN9_SEC_MASK) -> u8 {
        PIO0_PIN9_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN0_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN0_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN0_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN0_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN0_SEC_MASK {
        PIO1_PIN0_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN0_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN0_SEC_MASK) -> u8 {
        PIO1_PIN0_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN10_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN10_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN10_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN10_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN10_SEC_MASK {
        PIO1_PIN10_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN10_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN10_SEC_MASK) -> u8 {
        PIO1_PIN10_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN11_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN11_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN11_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN11_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN11_SEC_MASK {
        PIO1_PIN11_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN11_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN11_SEC_MASK) -> u8 {
        PIO1_PIN11_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN12_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN12_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN12_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN12_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN12_SEC_MASK {
        PIO1_PIN12_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN12_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN12_SEC_MASK) -> u8 {
        PIO1_PIN12_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN13_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN13_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN13_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN13_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN13_SEC_MASK {
        PIO1_PIN13_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN13_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN13_SEC_MASK) -> u8 {
        PIO1_PIN13_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN14_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN14_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN14_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN14_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN14_SEC_MASK {
        PIO1_PIN14_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN14_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN14_SEC_MASK) -> u8 {
        PIO1_PIN14_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN15_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN15_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN15_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN15_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN15_SEC_MASK {
        PIO1_PIN15_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN15_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN15_SEC_MASK) -> u8 {
        PIO1_PIN15_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN16_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN16_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN16_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN16_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN16_SEC_MASK {
        PIO1_PIN16_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN16_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN16_SEC_MASK) -> u8 {
        PIO1_PIN16_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN17_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN17_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN17_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN17_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN17_SEC_MASK {
        PIO1_PIN17_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN17_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN17_SEC_MASK) -> u8 {
        PIO1_PIN17_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN18_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN18_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN18_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN18_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN18_SEC_MASK {
        PIO1_PIN18_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN18_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN18_SEC_MASK) -> u8 {
        PIO1_PIN18_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN19_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN19_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN19_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN19_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN19_SEC_MASK {
        PIO1_PIN19_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN19_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN19_SEC_MASK) -> u8 {
        PIO1_PIN19_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN1_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN1_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN1_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN1_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN1_SEC_MASK {
        PIO1_PIN1_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN1_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN1_SEC_MASK) -> u8 {
        PIO1_PIN1_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN20_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN20_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN20_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN20_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN20_SEC_MASK {
        PIO1_PIN20_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN20_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN20_SEC_MASK) -> u8 {
        PIO1_PIN20_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN21_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN21_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN21_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN21_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN21_SEC_MASK {
        PIO1_PIN21_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN21_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN21_SEC_MASK) -> u8 {
        PIO1_PIN21_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN22_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN22_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN22_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN22_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN22_SEC_MASK {
        PIO1_PIN22_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN22_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN22_SEC_MASK) -> u8 {
        PIO1_PIN22_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN23_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN23_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN23_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN23_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN23_SEC_MASK {
        PIO1_PIN23_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN23_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN23_SEC_MASK) -> u8 {
        PIO1_PIN23_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN24_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN24_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN24_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN24_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN24_SEC_MASK {
        PIO1_PIN24_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN24_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN24_SEC_MASK) -> u8 {
        PIO1_PIN24_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN25_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN25_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN25_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN25_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN25_SEC_MASK {
        PIO1_PIN25_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN25_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN25_SEC_MASK) -> u8 {
        PIO1_PIN25_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN26_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN26_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN26_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN26_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN26_SEC_MASK {
        PIO1_PIN26_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN26_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN26_SEC_MASK) -> u8 {
        PIO1_PIN26_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN27_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN27_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN27_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN27_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN27_SEC_MASK {
        PIO1_PIN27_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN27_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN27_SEC_MASK) -> u8 {
        PIO1_PIN27_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN28_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN28_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN28_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN28_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN28_SEC_MASK {
        PIO1_PIN28_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN28_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN28_SEC_MASK) -> u8 {
        PIO1_PIN28_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN29_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN29_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN29_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN29_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN29_SEC_MASK {
        PIO1_PIN29_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN29_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN29_SEC_MASK) -> u8 {
        PIO1_PIN29_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN2_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN2_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN2_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN2_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN2_SEC_MASK {
        PIO1_PIN2_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN2_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN2_SEC_MASK) -> u8 {
        PIO1_PIN2_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN30_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN30_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN30_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN30_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN30_SEC_MASK {
        PIO1_PIN30_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN30_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN30_SEC_MASK) -> u8 {
        PIO1_PIN30_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN31_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN31_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN31_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN31_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN31_SEC_MASK {
        PIO1_PIN31_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN31_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN31_SEC_MASK) -> u8 {
        PIO1_PIN31_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN3_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN3_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN3_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN3_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN3_SEC_MASK {
        PIO1_PIN3_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN3_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN3_SEC_MASK) -> u8 {
        PIO1_PIN3_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN4_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN4_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN4_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN4_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN4_SEC_MASK {
        PIO1_PIN4_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN4_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN4_SEC_MASK) -> u8 {
        PIO1_PIN4_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN5_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN5_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN5_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN5_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN5_SEC_MASK {
        PIO1_PIN5_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN5_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN5_SEC_MASK) -> u8 {
        PIO1_PIN5_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN6_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN6_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN6_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN6_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN6_SEC_MASK {
        PIO1_PIN6_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN6_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN6_SEC_MASK) -> u8 {
        PIO1_PIN6_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN7_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN7_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN7_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN7_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN7_SEC_MASK {
        PIO1_PIN7_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN7_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN7_SEC_MASK) -> u8 {
        PIO1_PIN7_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN8_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN8_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN8_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN8_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN8_SEC_MASK {
        PIO1_PIN8_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN8_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN8_SEC_MASK) -> u8 {
        PIO1_PIN8_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_PIN9_SEC_MASK {
    #[doc = "Pin state is blocked to non-secure world."]
    BLOCKED = 0x0,
    #[doc = "Pin state is readable by non-secure world."]
    READABLE = 0x01,
}
impl PIO1_PIN9_SEC_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_PIN9_SEC_MASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_PIN9_SEC_MASK {
    #[inline(always)]
    fn from(val: u8) -> PIO1_PIN9_SEC_MASK {
        PIO1_PIN9_SEC_MASK::from_bits(val)
    }
}
impl From<PIO1_PIN9_SEC_MASK> for u8 {
    #[inline(always)]
    fn from(val: PIO1_PIN9_SEC_MASK) -> u8 {
        PIO1_PIN9_SEC_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLU_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PLU_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLU_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLU_RULE {
    #[inline(always)]
    fn from(val: u8) -> PLU_RULE {
        PLU_RULE::from_bits(val)
    }
}
impl From<PLU_RULE> for u8 {
    #[inline(always)]
    fn from(val: PLU_RULE) -> u8 {
        PLU_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PMC_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PMC_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PMC_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PMC_RULE {
    #[inline(always)]
    fn from(val: u8) -> PMC_RULE {
        PMC_RULE::from_bits(val)
    }
}
impl From<PMC_RULE> for u8 {
    #[inline(always)]
    fn from(val: PMC_RULE) -> u8 {
        PMC_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PQ_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PQ_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PQ_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PQ_RULE {
    #[inline(always)]
    fn from(val: u8) -> PQ_RULE {
        PQ_RULE::from_bits(val)
    }
}
impl From<PQ_RULE> for u8 {
    #[inline(always)]
    fn from(val: PQ_RULE) -> u8 {
        PQ_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PRINCE_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PRINCE_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PRINCE_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PRINCE_RULE {
    #[inline(always)]
    fn from(val: u8) -> PRINCE_RULE {
        PRINCE_RULE::from_bits(val)
    }
}
impl From<PRINCE_RULE> for u8 {
    #[inline(always)]
    fn from(val: PRINCE_RULE) -> u8 {
        PRINCE_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PUF_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl PUF_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PUF_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PUF_RULE {
    #[inline(always)]
    fn from(val: u8) -> PUF_RULE {
        PUF_RULE::from_bits(val)
    }
}
impl From<PUF_RULE> for u8 {
    #[inline(always)]
    fn from(val: PUF_RULE) -> u8 {
        PUF_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAM0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM0_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAM0_RULE {
        RAM0_RULE::from_bits(val)
    }
}
impl From<RAM0_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAM0_RULE) -> u8 {
        RAM0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAM1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM1_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAM1_RULE {
        RAM1_RULE::from_bits(val)
    }
}
impl From<RAM1_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAM1_RULE) -> u8 {
        RAM1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM2_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAM2_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM2_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM2_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAM2_RULE {
        RAM2_RULE::from_bits(val)
    }
}
impl From<RAM2_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAM2_RULE) -> u8 {
        RAM2_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM3_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAM3_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM3_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM3_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAM3_RULE {
        RAM3_RULE::from_bits(val)
    }
}
impl From<RAM3_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAM3_RULE) -> u8 {
        RAM3_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM4_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAM4_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM4_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM4_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAM4_RULE {
        RAM4_RULE::from_bits(val)
    }
}
impl From<RAM4_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAM4_RULE) -> u8 {
        RAM4_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAMX_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAMX_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAMX_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAMX_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAMX_RULE {
        RAMX_RULE::from_bits(val)
    }
}
impl From<RAMX_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAMX_RULE) -> u8 {
        RAMX_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_USB_HS_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RAM_USB_HS_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_USB_HS_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_USB_HS_RULE {
    #[inline(always)]
    fn from(val: u8) -> RAM_USB_HS_RULE {
        RAM_USB_HS_RULE::from_bits(val)
    }
}
impl From<RAM_USB_HS_RULE> for u8 {
    #[inline(always)]
    fn from(val: RAM_USB_HS_RULE) -> u8 {
        RAM_USB_HS_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RNG_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RNG_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RNG_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RNG_RULE {
    #[inline(always)]
    fn from(val: u8) -> RNG_RULE {
        RNG_RULE::from_bits(val)
    }
}
impl From<RNG_RULE> for u8 {
    #[inline(always)]
    fn from(val: RNG_RULE) -> u8 {
        RNG_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ROM_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl ROM_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ROM_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ROM_RULE {
    #[inline(always)]
    fn from(val: u8) -> ROM_RULE {
        ROM_RULE::from_bits(val)
    }
}
impl From<ROM_RULE> for u8 {
    #[inline(always)]
    fn from(val: ROM_RULE) -> u8 {
        ROM_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl RTC_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_RULE {
    #[inline(always)]
    fn from(val: u8) -> RTC_RULE {
        RTC_RULE::from_bits(val)
    }
}
impl From<RTC_RULE> for u8 {
    #[inline(always)]
    fn from(val: RTC_RULE) -> u8 {
        RTC_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCT_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SCT_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCT_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCT_RULE {
    #[inline(always)]
    fn from(val: u8) -> SCT_RULE {
        SCT_RULE::from_bits(val)
    }
}
impl From<SCT_RULE> for u8 {
    #[inline(always)]
    fn from(val: SCT_RULE) -> u8 {
        SCT_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SDIO_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SDIO_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SDIO_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SDIO_RULE {
    #[inline(always)]
    fn from(val: u8) -> SDIO_RULE {
        SDIO_RULE::from_bits(val)
    }
}
impl From<SDIO_RULE> for u8 {
    #[inline(always)]
    fn from(val: SDIO_RULE) -> u8 {
        SDIO_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CPU1_INT_MASK0_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SEC_CPU1_INT_MASK0_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CPU1_INT_MASK0_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CPU1_INT_MASK0_LOCK {
    #[inline(always)]
    fn from(val: u8) -> SEC_CPU1_INT_MASK0_LOCK {
        SEC_CPU1_INT_MASK0_LOCK::from_bits(val)
    }
}
impl From<SEC_CPU1_INT_MASK0_LOCK> for u8 {
    #[inline(always)]
    fn from(val: SEC_CPU1_INT_MASK0_LOCK) -> u8 {
        SEC_CPU1_INT_MASK0_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CPU1_INT_MASK1_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SEC_CPU1_INT_MASK1_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CPU1_INT_MASK1_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CPU1_INT_MASK1_LOCK {
    #[inline(always)]
    fn from(val: u8) -> SEC_CPU1_INT_MASK1_LOCK {
        SEC_CPU1_INT_MASK1_LOCK::from_bits(val)
    }
}
impl From<SEC_CPU1_INT_MASK1_LOCK> for u8 {
    #[inline(always)]
    fn from(val: SEC_CPU1_INT_MASK1_LOCK) -> u8 {
        SEC_CPU1_INT_MASK1_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE0 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE1 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE2 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE3 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE4 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE5 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE6 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE0_RULE7 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE0 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE0) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE1 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE1) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE2 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE2) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE3 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE3) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE4 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE4) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE5 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE5) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE6 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE6) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE1_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE1_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE1_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE1_RULE7 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE1_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE1_RULE7) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE1_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE0 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE0) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE1 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE1) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE2 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE2) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE3 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE3) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE4 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE4) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE5 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE5) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE6 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE6) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_FLASH_MEM_RULE2_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_FLASH_MEM_RULE2_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_FLASH_MEM_RULE2_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_FLASH_MEM_RULE2_RULE7 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_FLASH_MEM_RULE2_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_FLASH_MEM_RULE2_RULE7) -> u8 {
        SEC_CTRL_FLASH_MEM_RULE2_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE0 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE1 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE2 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE3 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE4 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE5 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE6 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE0_RULE7 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE0 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE0) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE1 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE1) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE2 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE2) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE3 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE3) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE4 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE4) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE5 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE5) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE6 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE6) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM0_MEM_RULE1_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM0_MEM_RULE1_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM0_MEM_RULE1_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM0_MEM_RULE1_RULE7 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM0_MEM_RULE1_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM0_MEM_RULE1_RULE7) -> u8 {
        SEC_CTRL_RAM0_MEM_RULE1_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE0 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE1 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE2 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE3 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE4 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE5 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE6 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE0_RULE7 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE0 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE0) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE1 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE1) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE2 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE2) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE3 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE3) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE4 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE4) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE5 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE5) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE6 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE6) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM1_MEM_RULE1_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM1_MEM_RULE1_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM1_MEM_RULE1_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM1_MEM_RULE1_RULE7 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM1_MEM_RULE1_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM1_MEM_RULE1_RULE7) -> u8 {
        SEC_CTRL_RAM1_MEM_RULE1_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE0 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE1 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE2 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE3 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE4 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE5 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE6 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE0_RULE7 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE0 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE0) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE1 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE1) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE2 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE2) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE3 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE3) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE4 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE4) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE5 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE5) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE6 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE6) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM2_MEM_RULE1_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM2_MEM_RULE1_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM2_MEM_RULE1_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM2_MEM_RULE1_RULE7 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM2_MEM_RULE1_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM2_MEM_RULE1_RULE7) -> u8 {
        SEC_CTRL_RAM2_MEM_RULE1_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE0 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE1 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE2 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE3 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE4 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE5 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE6 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE0_RULE7 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE0 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE0) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE1 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE1) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE2 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE2) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE3 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE3) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE4 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE4) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE5 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE5) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE6 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE6) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM3_MEM_RULE1_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM3_MEM_RULE1_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM3_MEM_RULE1_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM3_MEM_RULE1_RULE7 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM3_MEM_RULE1_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM3_MEM_RULE1_RULE7) -> u8 {
        SEC_CTRL_RAM3_MEM_RULE1_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM4_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM4_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM4_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE0 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM4_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM4_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM4_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM4_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM4_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE1 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM4_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM4_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM4_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM4_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM4_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE2 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM4_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM4_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAM4_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAM4_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAM4_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAM4_MEM_RULE0_RULE3 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAM4_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAM4_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_RAM4_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE0 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE1 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE2 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE3 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE4 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE5 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE6 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_RAMX_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_RAMX_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_RAMX_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_RAMX_MEM_RULE0_RULE7 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_RAMX_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_RAMX_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_RAMX_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE0 {
        SEC_CTRL_ROM_MEM_RULE0_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE0) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE1 {
        SEC_CTRL_ROM_MEM_RULE0_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE1) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE2 {
        SEC_CTRL_ROM_MEM_RULE0_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE2) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE3 {
        SEC_CTRL_ROM_MEM_RULE0_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE3) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE4 {
        SEC_CTRL_ROM_MEM_RULE0_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE4) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE5 {
        SEC_CTRL_ROM_MEM_RULE0_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE5) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE6 {
        SEC_CTRL_ROM_MEM_RULE0_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE6) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE0_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE0_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE0_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE0_RULE7 {
        SEC_CTRL_ROM_MEM_RULE0_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE0_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE0_RULE7) -> u8 {
        SEC_CTRL_ROM_MEM_RULE0_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE0 {
        SEC_CTRL_ROM_MEM_RULE1_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE0) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE1 {
        SEC_CTRL_ROM_MEM_RULE1_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE1) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE2 {
        SEC_CTRL_ROM_MEM_RULE1_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE2) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE3 {
        SEC_CTRL_ROM_MEM_RULE1_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE3) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE4 {
        SEC_CTRL_ROM_MEM_RULE1_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE4) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE5 {
        SEC_CTRL_ROM_MEM_RULE1_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE5) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE6 {
        SEC_CTRL_ROM_MEM_RULE1_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE6) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE1_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE1_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE1_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE1_RULE7 {
        SEC_CTRL_ROM_MEM_RULE1_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE1_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE1_RULE7) -> u8 {
        SEC_CTRL_ROM_MEM_RULE1_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE0 {
        SEC_CTRL_ROM_MEM_RULE2_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE0) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE1 {
        SEC_CTRL_ROM_MEM_RULE2_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE1) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE2 {
        SEC_CTRL_ROM_MEM_RULE2_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE2) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE3 {
        SEC_CTRL_ROM_MEM_RULE2_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE3) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE4 {
        SEC_CTRL_ROM_MEM_RULE2_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE4) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE5 {
        SEC_CTRL_ROM_MEM_RULE2_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE5) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE6 {
        SEC_CTRL_ROM_MEM_RULE2_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE6) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE2_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE2_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE2_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE2_RULE7 {
        SEC_CTRL_ROM_MEM_RULE2_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE2_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE2_RULE7) -> u8 {
        SEC_CTRL_ROM_MEM_RULE2_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE0 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE0 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE0 {
        SEC_CTRL_ROM_MEM_RULE3_RULE0::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE0> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE0) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE1 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE1 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE1 {
        SEC_CTRL_ROM_MEM_RULE3_RULE1::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE1> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE1) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE2 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE2 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE2 {
        SEC_CTRL_ROM_MEM_RULE3_RULE2::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE2> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE2) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE3 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE3 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE3 {
        SEC_CTRL_ROM_MEM_RULE3_RULE3::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE3> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE3) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE4 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE4 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE4 {
        SEC_CTRL_ROM_MEM_RULE3_RULE4::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE4> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE4) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE5 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE5 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE5 {
        SEC_CTRL_ROM_MEM_RULE3_RULE5::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE5> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE5) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE6 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE6 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE6 {
        SEC_CTRL_ROM_MEM_RULE3_RULE6::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE6> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE6) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_CTRL_ROM_MEM_RULE3_RULE7 {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_CTRL_ROM_MEM_RULE3_RULE7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_CTRL_ROM_MEM_RULE3_RULE7 {
    #[inline(always)]
    fn from(val: u8) -> SEC_CTRL_ROM_MEM_RULE3_RULE7 {
        SEC_CTRL_ROM_MEM_RULE3_RULE7::from_bits(val)
    }
}
impl From<SEC_CTRL_ROM_MEM_RULE3_RULE7> for u8 {
    #[inline(always)]
    fn from(val: SEC_CTRL_ROM_MEM_RULE3_RULE7) -> u8 {
        SEC_CTRL_ROM_MEM_RULE3_RULE7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_GPIO_MASK0_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SEC_GPIO_MASK0_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_GPIO_MASK0_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_GPIO_MASK0_LOCK {
    #[inline(always)]
    fn from(val: u8) -> SEC_GPIO_MASK0_LOCK {
        SEC_GPIO_MASK0_LOCK::from_bits(val)
    }
}
impl From<SEC_GPIO_MASK0_LOCK> for u8 {
    #[inline(always)]
    fn from(val: SEC_GPIO_MASK0_LOCK) -> u8 {
        SEC_GPIO_MASK0_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_GPIO_MASK1_LOCK {
    _RESERVED_0 = 0x0,
    #[doc = "Restricted mode."]
    BLOCKED = 0x01,
    #[doc = "Writable."]
    WRITABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SEC_GPIO_MASK1_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_GPIO_MASK1_LOCK {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_GPIO_MASK1_LOCK {
    #[inline(always)]
    fn from(val: u8) -> SEC_GPIO_MASK1_LOCK {
        SEC_GPIO_MASK1_LOCK::from_bits(val)
    }
}
impl From<SEC_GPIO_MASK1_LOCK> for u8 {
    #[inline(always)]
    fn from(val: SEC_GPIO_MASK1_LOCK) -> u8 {
        SEC_GPIO_MASK1_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_PINT_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SEC_PINT_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_PINT_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_PINT_RULE {
    #[inline(always)]
    fn from(val: u8) -> SEC_PINT_RULE {
        SEC_PINT_RULE::from_bits(val)
    }
}
impl From<SEC_PINT_RULE> for u8 {
    #[inline(always)]
    fn from(val: SEC_PINT_RULE) -> u8 {
        SEC_PINT_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_VIO_INFO_DATA_ACCESS {
    #[doc = "Code access."]
    CODE = 0x0,
    #[doc = "Data access."]
    DATA = 0x01,
}
impl SEC_VIO_INFO_DATA_ACCESS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_VIO_INFO_DATA_ACCESS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_VIO_INFO_DATA_ACCESS {
    #[inline(always)]
    fn from(val: u8) -> SEC_VIO_INFO_DATA_ACCESS {
        SEC_VIO_INFO_DATA_ACCESS::from_bits(val)
    }
}
impl From<SEC_VIO_INFO_DATA_ACCESS> for u8 {
    #[inline(always)]
    fn from(val: SEC_VIO_INFO_DATA_ACCESS) -> u8 {
        SEC_VIO_INFO_DATA_ACCESS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_VIO_INFO_MASTER {
    #[doc = "CPU0 Code."]
    VALUE_0 = 0x0,
    #[doc = "CPU0 System."]
    VALUE_1 = 0x01,
    #[doc = "CPU1 Data."]
    VALUE_2 = 0x02,
    #[doc = "CPU1 System."]
    VALUE_3 = 0x03,
    #[doc = "USB-HS Device."]
    VALUE_4 = 0x04,
    #[doc = "SDMA0."]
    VALUE_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "SDIO."]
    VALUE_8 = 0x08,
    #[doc = "PowerQuad."]
    VALUE_9 = 0x09,
    #[doc = "HASH."]
    VALUE_10 = 0x0a,
    #[doc = "USB-FS Host."]
    VALUE_11 = 0x0b,
    #[doc = "SDMA1."]
    VALUE_12 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SEC_VIO_INFO_MASTER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_VIO_INFO_MASTER {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_VIO_INFO_MASTER {
    #[inline(always)]
    fn from(val: u8) -> SEC_VIO_INFO_MASTER {
        SEC_VIO_INFO_MASTER::from_bits(val)
    }
}
impl From<SEC_VIO_INFO_MASTER> for u8 {
    #[inline(always)]
    fn from(val: SEC_VIO_INFO_MASTER) -> u8 {
        SEC_VIO_INFO_MASTER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_VIO_INFO_WRITE {
    #[doc = "Read access."]
    READ = 0x0,
    #[doc = "Write access."]
    WRITE = 0x01,
}
impl SEC_VIO_INFO_WRITE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_VIO_INFO_WRITE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_VIO_INFO_WRITE {
    #[inline(always)]
    fn from(val: u8) -> SEC_VIO_INFO_WRITE {
        SEC_VIO_INFO_WRITE::from_bits(val)
    }
}
impl From<SEC_VIO_INFO_WRITE> for u8 {
    #[inline(always)]
    fn from(val: SEC_VIO_INFO_WRITE) -> u8 {
        SEC_VIO_INFO_WRITE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_SECT_0_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SRAM_SECT_0_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_SECT_0_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_SECT_0_RULE {
    #[inline(always)]
    fn from(val: u8) -> SRAM_SECT_0_RULE {
        SRAM_SECT_0_RULE::from_bits(val)
    }
}
impl From<SRAM_SECT_0_RULE> for u8 {
    #[inline(always)]
    fn from(val: SRAM_SECT_0_RULE) -> u8 {
        SRAM_SECT_0_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_SECT_1_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SRAM_SECT_1_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_SECT_1_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_SECT_1_RULE {
    #[inline(always)]
    fn from(val: u8) -> SRAM_SECT_1_RULE {
        SRAM_SECT_1_RULE::from_bits(val)
    }
}
impl From<SRAM_SECT_1_RULE> for u8 {
    #[inline(always)]
    fn from(val: SRAM_SECT_1_RULE) -> u8 {
        SRAM_SECT_1_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_SECT_2_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SRAM_SECT_2_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_SECT_2_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_SECT_2_RULE {
    #[inline(always)]
    fn from(val: u8) -> SRAM_SECT_2_RULE {
        SRAM_SECT_2_RULE::from_bits(val)
    }
}
impl From<SRAM_SECT_2_RULE> for u8 {
    #[inline(always)]
    fn from(val: SRAM_SECT_2_RULE) -> u8 {
        SRAM_SECT_2_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_SECT_3_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SRAM_SECT_3_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_SECT_3_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_SECT_3_RULE {
    #[inline(always)]
    fn from(val: u8) -> SRAM_SECT_3_RULE {
        SRAM_SECT_3_RULE::from_bits(val)
    }
}
impl From<SRAM_SECT_3_RULE> for u8 {
    #[inline(always)]
    fn from(val: SRAM_SECT_3_RULE) -> u8 {
        SRAM_SECT_3_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSCON_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SYSCON_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSCON_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSCON_RULE {
    #[inline(always)]
    fn from(val: u8) -> SYSCON_RULE {
        SYSCON_RULE::from_bits(val)
    }
}
impl From<SYSCON_RULE> for u8 {
    #[inline(always)]
    fn from(val: SYSCON_RULE) -> u8 {
        SYSCON_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSCTRL_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl SYSCTRL_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSCTRL_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSCTRL_RULE {
    #[inline(always)]
    fn from(val: u8) -> SYSCTRL_RULE {
        SYSCTRL_RULE::from_bits(val)
    }
}
impl From<SYSCTRL_RULE> for u8 {
    #[inline(always)]
    fn from(val: SYSCTRL_RULE) -> u8 {
        SYSCTRL_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USBHPHY_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl USBHPHY_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USBHPHY_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USBHPHY_RULE {
    #[inline(always)]
    fn from(val: u8) -> USBHPHY_RULE {
        USBHPHY_RULE::from_bits(val)
    }
}
impl From<USBHPHY_RULE> for u8 {
    #[inline(always)]
    fn from(val: USBHPHY_RULE) -> u8 {
        USBHPHY_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB_FS_HOST_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl USB_FS_HOST_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB_FS_HOST_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB_FS_HOST_RULE {
    #[inline(always)]
    fn from(val: u8) -> USB_FS_HOST_RULE {
        USB_FS_HOST_RULE::from_bits(val)
    }
}
impl From<USB_FS_HOST_RULE> for u8 {
    #[inline(always)]
    fn from(val: USB_FS_HOST_RULE) -> u8 {
        USB_FS_HOST_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB_HS_DEV_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl USB_HS_DEV_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB_HS_DEV_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB_HS_DEV_RULE {
    #[inline(always)]
    fn from(val: u8) -> USB_HS_DEV_RULE {
        USB_HS_DEV_RULE::from_bits(val)
    }
}
impl From<USB_HS_DEV_RULE> for u8 {
    #[inline(always)]
    fn from(val: USB_HS_DEV_RULE) -> u8 {
        USB_HS_DEV_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB_HS_HOST_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl USB_HS_HOST_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB_HS_HOST_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB_HS_HOST_RULE {
    #[inline(always)]
    fn from(val: u8) -> USB_HS_HOST_RULE {
        USB_HS_HOST_RULE::from_bits(val)
    }
}
impl From<USB_HS_HOST_RULE> for u8 {
    #[inline(always)]
    fn from(val: USB_HS_HOST_RULE) -> u8 {
        USB_HS_HOST_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UTICK_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl UTICK_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UTICK_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UTICK_RULE {
    #[inline(always)]
    fn from(val: u8) -> UTICK_RULE {
        UTICK_RULE::from_bits(val)
    }
}
impl From<UTICK_RULE> for u8 {
    #[inline(always)]
    fn from(val: UTICK_RULE) -> u8 {
        UTICK_RULE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WWDT_RULE {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0x0,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P = 0x01,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 0x02,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P = 0x03,
}
impl WWDT_RULE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WWDT_RULE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WWDT_RULE {
    #[inline(always)]
    fn from(val: u8) -> WWDT_RULE {
        WWDT_RULE::from_bits(val)
    }
}
impl From<WWDT_RULE> for u8 {
    #[inline(always)]
    fn from(val: WWDT_RULE) -> u8 {
        WWDT_RULE::to_bits(val)
    }
}
