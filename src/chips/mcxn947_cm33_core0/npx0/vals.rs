#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctx0lk {
    #[doc = "Lock disabled: VMAPCTX0 remains read-write"]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTX0 (becomes read-only)"]
    LOCK_ENABLED = 0x01,
}
impl Ctx0lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctx0lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctx0lk {
    #[inline(always)]
    fn from(val: u8) -> Ctx0lk {
        Ctx0lk::from_bits(val)
    }
}
impl From<Ctx0lk> for u8 {
    #[inline(always)]
    fn from(val: Ctx0lk) -> u8 {
        Ctx0lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctx1lk {
    #[doc = "Lock disabled: VMAPCTX1 remains read-write"]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTX1 (becomes read-only)"]
    LOCK_ENABLED = 0x01,
}
impl Ctx1lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctx1lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctx1lk {
    #[inline(always)]
    fn from(val: u8) -> Ctx1lk {
        Ctx1lk::from_bits(val)
    }
}
impl From<Ctx1lk> for u8 {
    #[inline(always)]
    fn from(val: Ctx1lk) -> u8 {
        Ctx1lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctx2lk {
    #[doc = "Lock disabled: VMAPCTX2 remains read-write"]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTX2 (becomes read-only)"]
    LOCK_ENABLED = 0x01,
}
impl Ctx2lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctx2lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctx2lk {
    #[inline(always)]
    fn from(val: u8) -> Ctx2lk {
        Ctx2lk::from_bits(val)
    }
}
impl From<Ctx2lk> for u8 {
    #[inline(always)]
    fn from(val: Ctx2lk) -> u8 {
        Ctx2lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctx3lk {
    #[doc = "Lock disabled: VMAPCTX3 remains read-write"]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTX3 (becomes read-only)"]
    LOCK_ENABLED = 0x01,
}
impl Ctx3lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctx3lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctx3lk {
    #[inline(always)]
    fn from(val: u8) -> Ctx3lk {
        Ctx3lk::from_bits(val)
    }
}
impl From<Ctx3lk> for u8 {
    #[inline(always)]
    fn from(val: Ctx3lk) -> u8 {
        Ctx3lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gde {
    #[doc = "Global decryption disabled. NPX on-the-fly decryption is globally disabled. Subsequent reads return 0."]
    DECRYPTION_DISABLED = 0x0,
    #[doc = "Global decryption enabled. NPX on-the-fly decryption is globally enabled. Subsequent reads return 1."]
    DECRYPTION_ENABLED = 0x01,
}
impl Gde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gde {
    #[inline(always)]
    fn from(val: u8) -> Gde {
        Gde::from_bits(val)
    }
}
impl From<Gde> for u8 {
    #[inline(always)]
    fn from(val: Gde) -> u8 {
        Gde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gee {
    #[doc = "Global encryption disabled. NPX on-the-fly encryption is disabled. Subsequent reads return 0."]
    ENCRYPTION_DISABLED = 0x0,
    #[doc = "Global encryption enabled. NPX on-the-fly encryption is enabled if the flash access hits in a valid memory context. Subsequent reads return 1."]
    ENCRYPTION_ENABLED = 0x01,
}
impl Gee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gee {
    #[inline(always)]
    fn from(val: u8) -> Gee {
        Gee::from_bits(val)
    }
}
impl From<Gee> for u8 {
    #[inline(always)]
    fn from(val: Gee) -> u8 {
        Gee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Glk {
    #[doc = "Lock disabled. Subsequent reads return 0."]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTXn, NPXCR, or CACMSK. Subsequent reads return 1."]
    LOCK_ENABLED = 0x01,
}
impl Glk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Glk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Glk {
    #[inline(always)]
    fn from(val: u8) -> Glk {
        Glk::from_bits(val)
    }
}
impl From<Glk> for u8 {
    #[inline(always)]
    fn from(val: Glk) -> u8 {
        Glk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mlk {
    #[doc = "Lock disabled. Subsequent reads return 0."]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to mask. Subsequent reads return 1."]
    LOCK_ENABLED = 0x01,
}
impl Mlk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mlk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mlk {
    #[inline(always)]
    fn from(val: u8) -> Mlk {
        Mlk::from_bits(val)
    }
}
impl From<Mlk> for u8 {
    #[inline(always)]
    fn from(val: Mlk) -> u8 {
        Mlk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Numctx {
    #[doc = "No (zero) implemented memory contexts"]
    ZERO_CTX = 0x0,
    #[doc = "1 implemented memory contexts"]
    ONE_CTX = 0x01,
    #[doc = "2 implemented memory contexts"]
    TWO_CTX = 0x02,
    #[doc = "3 implemented memory contexts"]
    THREE_CTX = 0x03,
    #[doc = "4 implemented memory contexts"]
    FOUR_CTX = 0x04,
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
}
impl Numctx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Numctx {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Numctx {
    #[inline(always)]
    fn from(val: u8) -> Numctx {
        Numctx::from_bits(val)
    }
}
impl From<Numctx> for u8 {
    #[inline(always)]
    fn from(val: Numctx) -> u8 {
        Numctx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remaplk {
    #[doc = "Lock disabled: can write to REMAP"]
    LOCK_DISABLED = 0x0,
    #[doc = "Lock enabled: cannot write to REMAP"]
    LOCK_ENABLED = 0x01,
}
impl Remaplk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remaplk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remaplk {
    #[inline(always)]
    fn from(val: u8) -> Remaplk {
        Remaplk::from_bits(val)
    }
}
impl From<Remaplk> for u8 {
    #[inline(always)]
    fn from(val: Remaplk) -> u8 {
        Remaplk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V0 {
    #[doc = "Not valid"]
    KEY_NOTVALID = 0x0,
    #[doc = "Valid"]
    KEY_VALID = 0x01,
}
impl V0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V0 {
    #[inline(always)]
    fn from(val: u8) -> V0 {
        V0::from_bits(val)
    }
}
impl From<V0> for u8 {
    #[inline(always)]
    fn from(val: V0) -> u8 {
        V0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V1 {
    #[doc = "Not valid"]
    KEY_NOTVALID = 0x0,
    #[doc = "Valid"]
    KEY_VALID = 0x01,
}
impl V1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V1 {
    #[inline(always)]
    fn from(val: u8) -> V1 {
        V1::from_bits(val)
    }
}
impl From<V1> for u8 {
    #[inline(always)]
    fn from(val: V1) -> u8 {
        V1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V2 {
    #[doc = "Not valid"]
    KEY_NOTVALID = 0x0,
    #[doc = "Valid"]
    KEY_VALID = 0x01,
}
impl V2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V2 {
    #[inline(always)]
    fn from(val: u8) -> V2 {
        V2::from_bits(val)
    }
}
impl From<V2> for u8 {
    #[inline(always)]
    fn from(val: V2) -> u8 {
        V2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V3 {
    #[doc = "Not valid"]
    KEY_NOTVALID = 0x0,
    #[doc = "Valid"]
    KEY_VALID = 0x01,
}
impl V3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V3 {
    #[inline(always)]
    fn from(val: u8) -> V3 {
        V3::from_bits(val)
    }
}
impl From<V3> for u8 {
    #[inline(always)]
    fn from(val: V3) -> u8 {
        V3::to_bits(val)
    }
}
