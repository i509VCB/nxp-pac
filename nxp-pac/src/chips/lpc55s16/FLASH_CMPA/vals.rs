#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BLOCK_ENROLL {
    #[doc = "Allow PUF enroll operation."]
    ALLOW = 0x0,
    #[doc = "Disable PUF enroll operation."]
    DISABLE = 0x01,
    #[doc = "Disable PUF enroll operation."]
    VALUE_2 = 0x02,
    #[doc = "Disable PUF enroll operation."]
    VALUE_3 = 0x03,
}
impl BLOCK_ENROLL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BLOCK_ENROLL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BLOCK_ENROLL {
    #[inline(always)]
    fn from(val: u8) -> BLOCK_ENROLL {
        BLOCK_ENROLL::from_bits(val)
    }
}
impl From<BLOCK_ENROLL> for u8 {
    #[inline(always)]
    fn from(val: BLOCK_ENROLL) -> u8 {
        BLOCK_ENROLL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BLOCK_SET_KEY {
    #[doc = "Allow PUF Key Code generation."]
    ALLOW = 0x0,
    #[doc = "Disable PUF Key Code generation."]
    DISABLE = 0x01,
    #[doc = "Disable PUF Key Code generation."]
    VALUE_2 = 0x02,
    #[doc = "Disable PUF Key Code generation."]
    VALUE_3 = 0x03,
}
impl BLOCK_SET_KEY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BLOCK_SET_KEY {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BLOCK_SET_KEY {
    #[inline(always)]
    fn from(val: u8) -> BLOCK_SET_KEY {
        BLOCK_SET_KEY::from_bits(val)
    }
}
impl From<BLOCK_SET_KEY> for u8 {
    #[inline(always)]
    fn from(val: BLOCK_SET_KEY) -> u8 {
        BLOCK_SET_KEY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOT_SEED_CUST_CFG {
    #[doc = "not included."]
    NOT_INCLUD = 0x0,
    #[doc = "included."]
    INCLUD = 0x01,
    #[doc = "included."]
    VALUE_2 = 0x02,
    #[doc = "included."]
    VALUE_3 = 0x03,
}
impl BOOT_SEED_CUST_CFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOT_SEED_CUST_CFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOT_SEED_CUST_CFG {
    #[inline(always)]
    fn from(val: u8) -> BOOT_SEED_CUST_CFG {
        BOOT_SEED_CUST_CFG::from_bits(val)
    }
}
impl From<BOOT_SEED_CUST_CFG> for u8 {
    #[inline(always)]
    fn from(val: BOOT_SEED_CUST_CFG) -> u8 {
        BOOT_SEED_CUST_CFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOT_SEED_INC_EPOCH {
    #[doc = "not included."]
    NOT_INCLUD = 0x0,
    #[doc = "included."]
    INCLUD = 0x01,
    #[doc = "included."]
    VALUE_2 = 0x02,
    #[doc = "included."]
    VALUE_3 = 0x03,
}
impl BOOT_SEED_INC_EPOCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOT_SEED_INC_EPOCH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOT_SEED_INC_EPOCH {
    #[inline(always)]
    fn from(val: u8) -> BOOT_SEED_INC_EPOCH {
        BOOT_SEED_INC_EPOCH::from_bits(val)
    }
}
impl From<BOOT_SEED_INC_EPOCH> for u8 {
    #[inline(always)]
    fn from(val: BOOT_SEED_INC_EPOCH) -> u8 {
        BOOT_SEED_INC_EPOCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOT_SEED_INC_NXP_CFG {
    #[doc = "not included."]
    NOT_INCLUD = 0x0,
    #[doc = "included."]
    INCLUD = 0x01,
    #[doc = "included."]
    VALUE_2 = 0x02,
    #[doc = "included."]
    VALUE_3 = 0x03,
}
impl BOOT_SEED_INC_NXP_CFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOT_SEED_INC_NXP_CFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOT_SEED_INC_NXP_CFG {
    #[inline(always)]
    fn from(val: u8) -> BOOT_SEED_INC_NXP_CFG {
        BOOT_SEED_INC_NXP_CFG::from_bits(val)
    }
}
impl From<BOOT_SEED_INC_NXP_CFG> for u8 {
    #[inline(always)]
    fn from(val: BOOT_SEED_INC_NXP_CFG) -> u8 {
        BOOT_SEED_INC_NXP_CFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOT_SPEED {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE."]
    VALUE_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "48MHz FRO."]
    VALUE_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl BOOT_SPEED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOT_SPEED {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOT_SPEED {
    #[inline(always)]
    fn from(val: u8) -> BOOT_SPEED {
        BOOT_SPEED::from_bits(val)
    }
}
impl From<BOOT_SPEED> for u8 {
    #[inline(always)]
    fn from(val: BOOT_SPEED) -> u8 {
        BOOT_SPEED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_DBGEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_DBGEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_DBGEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_DBGEN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_DBGEN {
        CC_SOCU_PIN_DBGEN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_DBGEN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_DBGEN) -> u8 {
        CC_SOCU_PIN_DBGEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_FA_ME_CMD_EN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_FA_ME_CMD_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_FA_ME_CMD_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_FA_ME_CMD_EN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_FA_ME_CMD_EN {
        CC_SOCU_PIN_FA_ME_CMD_EN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_FA_ME_CMD_EN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_FA_ME_CMD_EN) -> u8 {
        CC_SOCU_PIN_FA_ME_CMD_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_ISP_CMD_EN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_ISP_CMD_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_ISP_CMD_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_ISP_CMD_EN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_ISP_CMD_EN {
        CC_SOCU_PIN_ISP_CMD_EN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_ISP_CMD_EN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_ISP_CMD_EN) -> u8 {
        CC_SOCU_PIN_ISP_CMD_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_NIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_NIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_NIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_NIDEN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_NIDEN {
        CC_SOCU_PIN_NIDEN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_NIDEN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_NIDEN) -> u8 {
        CC_SOCU_PIN_NIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_SPIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_SPIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_SPIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_SPIDEN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_SPIDEN {
        CC_SOCU_PIN_SPIDEN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_SPIDEN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_SPIDEN) -> u8 {
        CC_SOCU_PIN_SPIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_SPNIDEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_SPNIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_SPNIDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_SPNIDEN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_SPNIDEN {
        CC_SOCU_PIN_SPNIDEN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_SPNIDEN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_SPNIDEN) -> u8 {
        CC_SOCU_PIN_SPNIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CC_SOCU_PIN_TAPEN {
    #[doc = "Use DAP to enable."]
    ENABLE = 0x0,
    #[doc = "Fixed state."]
    DISABLE = 0x01,
}
impl CC_SOCU_PIN_TAPEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CC_SOCU_PIN_TAPEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CC_SOCU_PIN_TAPEN {
    #[inline(always)]
    fn from(val: u8) -> CC_SOCU_PIN_TAPEN {
        CC_SOCU_PIN_TAPEN::from_bits(val)
    }
}
impl From<CC_SOCU_PIN_TAPEN> for u8 {
    #[inline(always)]
    fn from(val: CC_SOCU_PIN_TAPEN) -> u8 {
        CC_SOCU_PIN_TAPEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEFAULT_ISP_MODE {
    #[doc = "Auto ISP."]
    AUTO_ISP = 0x0,
    #[doc = "USB_HID_ISP."]
    USB_HID_ISP = 0x01,
    #[doc = "UART ISP."]
    UART_ISP = 0x02,
    #[doc = "SPI Slave ISP."]
    SPI_ISP = 0x03,
    #[doc = "I2C Slave ISP."]
    I2C_ISP = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Disable ISP fall through."]
    DISABLE = 0x07,
}
impl DEFAULT_ISP_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEFAULT_ISP_MODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEFAULT_ISP_MODE {
    #[inline(always)]
    fn from(val: u8) -> DEFAULT_ISP_MODE {
        DEFAULT_ISP_MODE::from_bits(val)
    }
}
impl From<DEFAULT_ISP_MODE> for u8 {
    #[inline(always)]
    fn from(val: DEFAULT_ISP_MODE) -> u8 {
        DEFAULT_ISP_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DICE_CUST_CFG {
    #[doc = "not included."]
    NOT_INCLUD = 0x0,
    #[doc = "included."]
    UNCLUD = 0x01,
    #[doc = "included."]
    VALUE_2 = 0x02,
    #[doc = "included."]
    VALUE_3 = 0x03,
}
impl DICE_CUST_CFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DICE_CUST_CFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DICE_CUST_CFG {
    #[inline(always)]
    fn from(val: u8) -> DICE_CUST_CFG {
        DICE_CUST_CFG::from_bits(val)
    }
}
impl From<DICE_CUST_CFG> for u8 {
    #[inline(always)]
    fn from(val: DICE_CUST_CFG) -> u8 {
        DICE_CUST_CFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DICE_INC_NXP_CFG {
    #[doc = "not included."]
    NOT_INCLUD = 0x0,
    #[doc = "included."]
    INCLUD = 0x01,
    #[doc = "included."]
    VALUE_2 = 0x02,
    #[doc = "included."]
    VALUE_3 = 0x03,
}
impl DICE_INC_NXP_CFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DICE_INC_NXP_CFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DICE_INC_NXP_CFG {
    #[inline(always)]
    fn from(val: u8) -> DICE_INC_NXP_CFG {
        DICE_INC_NXP_CFG::from_bits(val)
    }
}
impl From<DICE_INC_NXP_CFG> for u8 {
    #[inline(always)]
    fn from(val: DICE_INC_NXP_CFG) -> u8 {
        DICE_INC_NXP_CFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DICE_INC_SEC_EPOCH {
    #[doc = "not included."]
    NOT_INCLUD = 0x0,
    #[doc = "included."]
    INCLUD = 0x01,
    #[doc = "included."]
    VALUE_2 = 0x02,
    #[doc = "included."]
    VALUE_3 = 0x03,
}
impl DICE_INC_SEC_EPOCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DICE_INC_SEC_EPOCH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DICE_INC_SEC_EPOCH {
    #[inline(always)]
    fn from(val: u8) -> DICE_INC_SEC_EPOCH {
        DICE_INC_SEC_EPOCH::from_bits(val)
    }
}
impl From<DICE_INC_SEC_EPOCH> for u8 {
    #[inline(always)]
    fn from(val: DICE_INC_SEC_EPOCH) -> u8 {
        DICE_INC_SEC_EPOCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOCK_REG0 {
    #[doc = "Region is not locked."]
    UNLOCK = 0x0,
    #[doc = "Region is locked."]
    LOCK = 0x01,
    #[doc = "Region is locked."]
    VALUE_2 = 0x02,
    #[doc = "Region is locked."]
    VALUE_3 = 0x03,
}
impl LOCK_REG0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOCK_REG0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOCK_REG0 {
    #[inline(always)]
    fn from(val: u8) -> LOCK_REG0 {
        LOCK_REG0::from_bits(val)
    }
}
impl From<LOCK_REG0> for u8 {
    #[inline(always)]
    fn from(val: LOCK_REG0) -> u8 {
        LOCK_REG0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOCK_REG1 {
    #[doc = "Region is not locked."]
    UNLOCK = 0x0,
    #[doc = "Region is locked."]
    LOCK = 0x01,
    #[doc = "Region is locked."]
    VALUE_2 = 0x02,
    #[doc = "Region is locked."]
    VALUE_3 = 0x03,
}
impl LOCK_REG1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOCK_REG1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOCK_REG1 {
    #[inline(always)]
    fn from(val: u8) -> LOCK_REG1 {
        LOCK_REG1::from_bits(val)
    }
}
impl From<LOCK_REG1> for u8 {
    #[inline(always)]
    fn from(val: LOCK_REG1) -> u8 {
        LOCK_REG1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REG0_ERASE_CHECK_EN {
    #[doc = "Region is disabled."]
    DISABLE = 0x0,
    #[doc = "Region is enabled."]
    ENABLE = 0x01,
    #[doc = "Region is enabled."]
    VALUE_2 = 0x02,
    #[doc = "Region is enabled."]
    VALUE_3 = 0x03,
}
impl REG0_ERASE_CHECK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REG0_ERASE_CHECK_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REG0_ERASE_CHECK_EN {
    #[inline(always)]
    fn from(val: u8) -> REG0_ERASE_CHECK_EN {
        REG0_ERASE_CHECK_EN::from_bits(val)
    }
}
impl From<REG0_ERASE_CHECK_EN> for u8 {
    #[inline(always)]
    fn from(val: REG0_ERASE_CHECK_EN) -> u8 {
        REG0_ERASE_CHECK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REG1_ERASE_CHECK_EN {
    #[doc = "Region is disabled."]
    DISABLE = 0x0,
    #[doc = "Region is enabled."]
    ENABLE = 0x01,
    #[doc = "Region is enabled."]
    VALUE_2 = 0x02,
    #[doc = "Region is enabled."]
    VALUE_3 = 0x03,
}
impl REG1_ERASE_CHECK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REG1_ERASE_CHECK_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REG1_ERASE_CHECK_EN {
    #[inline(always)]
    fn from(val: u8) -> REG1_ERASE_CHECK_EN {
        REG1_ERASE_CHECK_EN::from_bits(val)
    }
}
impl From<REG1_ERASE_CHECK_EN> for u8 {
    #[inline(always)]
    fn from(val: REG1_ERASE_CHECK_EN) -> u8 {
        REG1_ERASE_CHECK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REG2_ERASE_CHECK_EN {
    #[doc = "Region is disabled."]
    DISABLE = 0x0,
    #[doc = "Region is enabled."]
    ENABLE = 0x01,
    #[doc = "Region is enabled."]
    VALUE_2 = 0x02,
    #[doc = "Region is enabled."]
    VALUE_3 = 0x03,
}
impl REG2_ERASE_CHECK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REG2_ERASE_CHECK_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REG2_ERASE_CHECK_EN {
    #[inline(always)]
    fn from(val: u8) -> REG2_ERASE_CHECK_EN {
        REG2_ERASE_CHECK_EN::from_bits(val)
    }
}
impl From<REG2_ERASE_CHECK_EN> for u8 {
    #[inline(always)]
    fn from(val: REG2_ERASE_CHECK_EN) -> u8 {
        REG2_ERASE_CHECK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RSA4K {
    #[doc = "Allow RSA2048 and higher."]
    VALUE_0 = 0x0,
    #[doc = "RSA4096 only."]
    VALUE_1 = 0x01,
    #[doc = "RSA4096 only."]
    VALUE_2 = 0x02,
    #[doc = "RSA4096 only."]
    VALUE_3 = 0x03,
}
impl RSA4K {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RSA4K {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RSA4K {
    #[inline(always)]
    fn from(val: u8) -> RSA4K {
        RSA4K::from_bits(val)
    }
}
impl From<RSA4K> for u8 {
    #[inline(always)]
    fn from(val: RSA4K) -> u8 {
        RSA4K::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEC_BOOT_EN {
    #[doc = "Plain image (internal flash with or without CRC)."]
    DISABLE = 0x0,
    #[doc = "Boot signed images. (internal flash, RSA signed)."]
    ENABLE = 0x01,
    #[doc = "Boot signed images. (internal flash, RSA signed)."]
    VALUE_2 = 0x02,
    #[doc = "Boot signed images. (internal flash, RSA signed)."]
    VALUE_3 = 0x03,
}
impl SEC_BOOT_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEC_BOOT_EN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEC_BOOT_EN {
    #[inline(always)]
    fn from(val: u8) -> SEC_BOOT_EN {
        SEC_BOOT_EN::from_bits(val)
    }
}
impl From<SEC_BOOT_EN> for u8 {
    #[inline(always)]
    fn from(val: SEC_BOOT_EN) -> u8 {
        SEC_BOOT_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SKIP_BOOT_SEED {
    #[doc = "Enable BOOT_SEED."]
    ENABLE = 0x0,
    #[doc = "Disable BOOT_SEED."]
    DISABLE = 0x01,
    #[doc = "Disable BOOT_SEED."]
    VALUE_2 = 0x02,
    #[doc = "Disable BOOT_SEED."]
    VALUE_3 = 0x03,
}
impl SKIP_BOOT_SEED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SKIP_BOOT_SEED {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SKIP_BOOT_SEED {
    #[inline(always)]
    fn from(val: u8) -> SKIP_BOOT_SEED {
        SKIP_BOOT_SEED::from_bits(val)
    }
}
impl From<SKIP_BOOT_SEED> for u8 {
    #[inline(always)]
    fn from(val: SKIP_BOOT_SEED) -> u8 {
        SKIP_BOOT_SEED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SKIP_DICE {
    #[doc = "Enable DICE."]
    ENABLE = 0x0,
    #[doc = "Disable DICE."]
    DISABLE = 0x01,
    #[doc = "Disable DICE."]
    VALUE_2 = 0x02,
    #[doc = "Disable DICE."]
    VALUE_3 = 0x03,
}
impl SKIP_DICE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SKIP_DICE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SKIP_DICE {
    #[inline(always)]
    fn from(val: u8) -> SKIP_DICE {
        SKIP_DICE::from_bits(val)
    }
}
impl From<SKIP_DICE> for u8 {
    #[inline(always)]
    fn from(val: SKIP_DICE) -> u8 {
        SKIP_DICE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TZM_IMAGE_TYPE {
    #[doc = "TZ-M image mode is taken from application image header."]
    VALUE_0 = 0x0,
    #[doc = "TZ-M disabled image, boots to non-secure mode."]
    VALUE_1 = 0x01,
    #[doc = "TZ-M enabled image, boots to secure mode."]
    VALUE_2 = 0x02,
    #[doc = "TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header."]
    VALUE_3 = 0x03,
}
impl TZM_IMAGE_TYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TZM_IMAGE_TYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TZM_IMAGE_TYPE {
    #[inline(always)]
    fn from(val: u8) -> TZM_IMAGE_TYPE {
        TZM_IMAGE_TYPE::from_bits(val)
    }
}
impl From<TZM_IMAGE_TYPE> for u8 {
    #[inline(always)]
    fn from(val: TZM_IMAGE_TYPE) -> u8 {
        TZM_IMAGE_TYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID {
    #[doc = "Capa Bank trimmings not valid. Default trimmings value are used."]
    NOT_TRIM = 0x0,
    #[doc = "Capa Bank trimmings valid."]
    VALID = 0x01,
}
impl XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID {
    #[inline(always)]
    fn from(val: u8) -> XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID {
        XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID::from_bits(val)
    }
}
impl From<XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID> for u8 {
    #[inline(always)]
    fn from(val: XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID) -> u8 {
        XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID {
    #[doc = "Capa Bank trimmings not valid. Default trimmings value are used."]
    NOT_TRIM = 0x0,
    #[doc = "Capa Bank trimmings valid."]
    VALID = 0x01,
}
impl XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID {
    #[inline(always)]
    fn from(val: u8) -> XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID {
        XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID::from_bits(val)
    }
}
impl From<XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID> for u8 {
    #[inline(always)]
    fn from(val: XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID) -> u8 {
        XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID::to_bits(val)
    }
}
