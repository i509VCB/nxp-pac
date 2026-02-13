#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CkDis {
    #[doc = "PUF RAM clock is disabled."]
    CLOCK_OFF = 0x0,
    #[doc = "PUF RAM clock is enabled."]
    CLOCK_ON = 0x01,
}
impl CkDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkDis {
    #[inline(always)]
    fn from(val: u8) -> CkDis {
        CkDis::from_bits(val)
    }
}
impl From<CkDis> for u8 {
    #[inline(always)]
    fn from(val: CkDis) -> u8 {
        CkDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyenableKey0 {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    DISABLED_0 = 0x0,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    DISABLED_1 = 0x01,
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY0 register."]
    ENABLED = 0x02,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY0 register."]
    DISABLED_3 = 0x03,
}
impl KeyenableKey0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyenableKey0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyenableKey0 {
    #[inline(always)]
    fn from(val: u8) -> KeyenableKey0 {
        KeyenableKey0::from_bits(val)
    }
}
impl From<KeyenableKey0> for u8 {
    #[inline(always)]
    fn from(val: KeyenableKey0) -> u8 {
        KeyenableKey0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyenableKey1 {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    DISABLED_0 = 0x0,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    DISABLED_1 = 0x01,
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY1 register."]
    ENABLED = 0x02,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY1 register."]
    DISABLED_3 = 0x03,
}
impl KeyenableKey1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyenableKey1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyenableKey1 {
    #[inline(always)]
    fn from(val: u8) -> KeyenableKey1 {
        KeyenableKey1::from_bits(val)
    }
}
impl From<KeyenableKey1> for u8 {
    #[inline(always)]
    fn from(val: KeyenableKey1) -> u8 {
        KeyenableKey1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyenableKey2 {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    DISABLED_0 = 0x0,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    DISABLED_1 = 0x01,
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY2 register."]
    ENABLED = 0x02,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY2 register."]
    DISABLED_3 = 0x03,
}
impl KeyenableKey2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyenableKey2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyenableKey2 {
    #[inline(always)]
    fn from(val: u8) -> KeyenableKey2 {
        KeyenableKey2::from_bits(val)
    }
}
impl From<KeyenableKey2> for u8 {
    #[inline(always)]
    fn from(val: KeyenableKey2) -> u8 {
        KeyenableKey2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyenableKey3 {
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    DISABLED_0 = 0x0,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    DISABLED_1 = 0x01,
    #[doc = "Data coming from the PUF Index 0 interface are shifted in the KEY3 register."]
    ENABLED = 0x02,
    #[doc = "Data coming from the PUF Index 0 interface are NOT shifted in the KEY3 register."]
    DISABLED_3 = 0x03,
}
impl KeyenableKey3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyenableKey3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyenableKey3 {
    #[inline(always)]
    fn from(val: u8) -> KeyenableKey3 {
        KeyenableKey3::from_bits(val)
    }
}
impl From<KeyenableKey3> for u8 {
    #[inline(always)]
    fn from(val: KeyenableKey3) -> u8 {
        KeyenableKey3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeylockKey0 {
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\] and KEYRESET\\[KEY0\\] is NOT allowed."]
    KEY0LOCK_0 = 0x0,
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\] and KEYRESET\\[KEY0\\] is NOT allowed."]
    KEY0LOCK_1 = 0x01,
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\] and KEYRESET\\[KEY0\\] is allowed."]
    KEY0UNLOCK = 0x02,
    #[doc = "Write access to KEY0MASK, KEYENABLE\\[KEY0\\] and KEYRESET\\[KEY0\\] is NOT allowed."]
    KEY0LOCK_3 = 0x03,
}
impl KeylockKey0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeylockKey0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeylockKey0 {
    #[inline(always)]
    fn from(val: u8) -> KeylockKey0 {
        KeylockKey0::from_bits(val)
    }
}
impl From<KeylockKey0> for u8 {
    #[inline(always)]
    fn from(val: KeylockKey0) -> u8 {
        KeylockKey0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeylockKey1 {
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\] and KEYRESET\\[KEY1\\] is NOT allowed."]
    KEY1LOCK_0 = 0x0,
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\] and KEYRESET\\[KEY1\\] is NOT allowed."]
    KEY1LOCK_1 = 0x01,
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\] and KEYRESET\\[KEY1\\] is allowed."]
    KEY1UNLOCK = 0x02,
    #[doc = "Write access to KEY1MASK, KEYENABLE\\[KEY1\\] and KEYRESET\\[KEY1\\] is NOT allowed."]
    KEY1LOCK_3 = 0x03,
}
impl KeylockKey1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeylockKey1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeylockKey1 {
    #[inline(always)]
    fn from(val: u8) -> KeylockKey1 {
        KeylockKey1::from_bits(val)
    }
}
impl From<KeylockKey1> for u8 {
    #[inline(always)]
    fn from(val: KeylockKey1) -> u8 {
        KeylockKey1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeylockKey2 {
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\] and KEYRESET\\[KEY2\\] is NOT allowed."]
    KEY2LOCK_0 = 0x0,
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\] and KEYRESET\\[KEY2\\] is NOT allowed."]
    KEY2LOCK_1 = 0x01,
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\] and KEYRESET\\[KEY2\\] is allowed."]
    KEY2UNLOCK = 0x02,
    #[doc = "Write access to KEY2MASK, KEYENABLE\\[KEY2\\] and KEYRESET\\[KEY2\\] is NOT allowed."]
    KEY2LOCK_3 = 0x03,
}
impl KeylockKey2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeylockKey2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeylockKey2 {
    #[inline(always)]
    fn from(val: u8) -> KeylockKey2 {
        KeylockKey2::from_bits(val)
    }
}
impl From<KeylockKey2> for u8 {
    #[inline(always)]
    fn from(val: KeylockKey2) -> u8 {
        KeylockKey2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeylockKey3 {
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\] and KEYRESET\\[KEY3\\] is NOT allowed."]
    KEY3LOCK_0 = 0x0,
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\] and KEYRESET\\[KEY3\\] is NOT allowed."]
    KEY3LOCK_1 = 0x01,
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\] and KEYRESET\\[KEY3\\] is allowed."]
    KEY3UNLOCK = 0x02,
    #[doc = "Write access to KEY3MASK, KEYENABLE\\[KEY3\\] and KEYRESET\\[KEY3\\] is NOT allowed."]
    KEY3LOCK_3 = 0x03,
}
impl KeylockKey3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeylockKey3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeylockKey3 {
    #[inline(always)]
    fn from(val: u8) -> KeylockKey3 {
        KeylockKey3::from_bits(val)
    }
}
impl From<KeylockKey3> for u8 {
    #[inline(always)]
    fn from(val: KeylockKey3) -> u8 {
        KeylockKey3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyresetKey0 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Reset KEY0 Hold register and SHIFT_STATUS\\[KEY0\\]."]
    RESET = 0x02,
    _RESERVED_3 = 0x03,
}
impl KeyresetKey0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyresetKey0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyresetKey0 {
    #[inline(always)]
    fn from(val: u8) -> KeyresetKey0 {
        KeyresetKey0::from_bits(val)
    }
}
impl From<KeyresetKey0> for u8 {
    #[inline(always)]
    fn from(val: KeyresetKey0) -> u8 {
        KeyresetKey0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyresetKey1 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Reset KEY1 Hold register and SHIFT_STATUS\\[KEY1\\]."]
    RESET = 0x02,
    _RESERVED_3 = 0x03,
}
impl KeyresetKey1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyresetKey1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyresetKey1 {
    #[inline(always)]
    fn from(val: u8) -> KeyresetKey1 {
        KeyresetKey1::from_bits(val)
    }
}
impl From<KeyresetKey1> for u8 {
    #[inline(always)]
    fn from(val: KeyresetKey1) -> u8 {
        KeyresetKey1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyresetKey2 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Reset KEY2 Hold register and SHIFT_STATUS\\[KEY2\\]."]
    RESET = 0x02,
    _RESERVED_3 = 0x03,
}
impl KeyresetKey2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyresetKey2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyresetKey2 {
    #[inline(always)]
    fn from(val: u8) -> KeyresetKey2 {
        KeyresetKey2::from_bits(val)
    }
}
impl From<KeyresetKey2> for u8 {
    #[inline(always)]
    fn from(val: KeyresetKey2) -> u8 {
        KeyresetKey2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyresetKey3 {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Reset KEY3 Hold register and SHIFT_STATUS\\[KEY3\\]."]
    RESET = 0x02,
    _RESERVED_3 = 0x03,
}
impl KeyresetKey3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyresetKey3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyresetKey3 {
    #[inline(always)]
    fn from(val: u8) -> KeyresetKey3 {
        KeyresetKey3::from_bits(val)
    }
}
impl From<KeyresetKey3> for u8 {
    #[inline(always)]
    fn from(val: KeyresetKey3) -> u8 {
        KeyresetKey3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamOn {
    #[doc = "Power Off"]
    POWER_OFF = 0x0,
    #[doc = "Power On"]
    POWER_ON = 0x01,
}
impl RamOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamOn {
    #[inline(always)]
    fn from(val: u8) -> RamOn {
        RamOn::from_bits(val)
    }
}
impl From<RamOn> for u8 {
    #[inline(always)]
    fn from(val: RamOn) -> u8 {
        RamOn::to_bits(val)
    }
}
