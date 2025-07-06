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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val0 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val0 {
    #[inline(always)]
    fn from(val: u8) -> Val0 {
        Val0::from_bits(val)
    }
}
impl From<Val0> for u8 {
    #[inline(always)]
    fn from(val: Val0) -> u8 {
        Val0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val1 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val1 {
    #[inline(always)]
    fn from(val: u8) -> Val1 {
        Val1::from_bits(val)
    }
}
impl From<Val1> for u8 {
    #[inline(always)]
    fn from(val: Val1) -> u8 {
        Val1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val10 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val10 {
    #[inline(always)]
    fn from(val: u8) -> Val10 {
        Val10::from_bits(val)
    }
}
impl From<Val10> for u8 {
    #[inline(always)]
    fn from(val: Val10) -> u8 {
        Val10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val11 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val11 {
    #[inline(always)]
    fn from(val: u8) -> Val11 {
        Val11::from_bits(val)
    }
}
impl From<Val11> for u8 {
    #[inline(always)]
    fn from(val: Val11) -> u8 {
        Val11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val12 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val12 {
    #[inline(always)]
    fn from(val: u8) -> Val12 {
        Val12::from_bits(val)
    }
}
impl From<Val12> for u8 {
    #[inline(always)]
    fn from(val: Val12) -> u8 {
        Val12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val13 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val13 {
    #[inline(always)]
    fn from(val: u8) -> Val13 {
        Val13::from_bits(val)
    }
}
impl From<Val13> for u8 {
    #[inline(always)]
    fn from(val: Val13) -> u8 {
        Val13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val14 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val14 {
    #[inline(always)]
    fn from(val: u8) -> Val14 {
        Val14::from_bits(val)
    }
}
impl From<Val14> for u8 {
    #[inline(always)]
    fn from(val: Val14) -> u8 {
        Val14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val15 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val15 {
    #[inline(always)]
    fn from(val: u8) -> Val15 {
        Val15::from_bits(val)
    }
}
impl From<Val15> for u8 {
    #[inline(always)]
    fn from(val: Val15) -> u8 {
        Val15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val16 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val16 {
    #[inline(always)]
    fn from(val: u8) -> Val16 {
        Val16::from_bits(val)
    }
}
impl From<Val16> for u8 {
    #[inline(always)]
    fn from(val: Val16) -> u8 {
        Val16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val17 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val17 {
    #[inline(always)]
    fn from(val: u8) -> Val17 {
        Val17::from_bits(val)
    }
}
impl From<Val17> for u8 {
    #[inline(always)]
    fn from(val: Val17) -> u8 {
        Val17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val18 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val18 {
    #[inline(always)]
    fn from(val: u8) -> Val18 {
        Val18::from_bits(val)
    }
}
impl From<Val18> for u8 {
    #[inline(always)]
    fn from(val: Val18) -> u8 {
        Val18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val19 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val19 {
    #[inline(always)]
    fn from(val: u8) -> Val19 {
        Val19::from_bits(val)
    }
}
impl From<Val19> for u8 {
    #[inline(always)]
    fn from(val: Val19) -> u8 {
        Val19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val2 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val2 {
    #[inline(always)]
    fn from(val: u8) -> Val2 {
        Val2::from_bits(val)
    }
}
impl From<Val2> for u8 {
    #[inline(always)]
    fn from(val: Val2) -> u8 {
        Val2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val20 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val20 {
    #[inline(always)]
    fn from(val: u8) -> Val20 {
        Val20::from_bits(val)
    }
}
impl From<Val20> for u8 {
    #[inline(always)]
    fn from(val: Val20) -> u8 {
        Val20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val21 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val21 {
    #[inline(always)]
    fn from(val: u8) -> Val21 {
        Val21::from_bits(val)
    }
}
impl From<Val21> for u8 {
    #[inline(always)]
    fn from(val: Val21) -> u8 {
        Val21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val22 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val22 {
    #[inline(always)]
    fn from(val: u8) -> Val22 {
        Val22::from_bits(val)
    }
}
impl From<Val22> for u8 {
    #[inline(always)]
    fn from(val: Val22) -> u8 {
        Val22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val23 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val23 {
    #[inline(always)]
    fn from(val: u8) -> Val23 {
        Val23::from_bits(val)
    }
}
impl From<Val23> for u8 {
    #[inline(always)]
    fn from(val: Val23) -> u8 {
        Val23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val24 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val24 {
    #[inline(always)]
    fn from(val: u8) -> Val24 {
        Val24::from_bits(val)
    }
}
impl From<Val24> for u8 {
    #[inline(always)]
    fn from(val: Val24) -> u8 {
        Val24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val25 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val25 {
    #[inline(always)]
    fn from(val: u8) -> Val25 {
        Val25::from_bits(val)
    }
}
impl From<Val25> for u8 {
    #[inline(always)]
    fn from(val: Val25) -> u8 {
        Val25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val26 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val26 {
    #[inline(always)]
    fn from(val: u8) -> Val26 {
        Val26::from_bits(val)
    }
}
impl From<Val26> for u8 {
    #[inline(always)]
    fn from(val: Val26) -> u8 {
        Val26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val27 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val27 {
    #[inline(always)]
    fn from(val: u8) -> Val27 {
        Val27::from_bits(val)
    }
}
impl From<Val27> for u8 {
    #[inline(always)]
    fn from(val: Val27) -> u8 {
        Val27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val28 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val28 {
    #[inline(always)]
    fn from(val: u8) -> Val28 {
        Val28::from_bits(val)
    }
}
impl From<Val28> for u8 {
    #[inline(always)]
    fn from(val: Val28) -> u8 {
        Val28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val29 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val29 {
    #[inline(always)]
    fn from(val: u8) -> Val29 {
        Val29::from_bits(val)
    }
}
impl From<Val29> for u8 {
    #[inline(always)]
    fn from(val: Val29) -> u8 {
        Val29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val3 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val3 {
    #[inline(always)]
    fn from(val: u8) -> Val3 {
        Val3::from_bits(val)
    }
}
impl From<Val3> for u8 {
    #[inline(always)]
    fn from(val: Val3) -> u8 {
        Val3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val30 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val30 {
    #[inline(always)]
    fn from(val: u8) -> Val30 {
        Val30::from_bits(val)
    }
}
impl From<Val30> for u8 {
    #[inline(always)]
    fn from(val: Val30) -> u8 {
        Val30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val31 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val31 {
    #[inline(always)]
    fn from(val: u8) -> Val31 {
        Val31::from_bits(val)
    }
}
impl From<Val31> for u8 {
    #[inline(always)]
    fn from(val: Val31) -> u8 {
        Val31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val4 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val4 {
    #[inline(always)]
    fn from(val: u8) -> Val4 {
        Val4::from_bits(val)
    }
}
impl From<Val4> for u8 {
    #[inline(always)]
    fn from(val: Val4) -> u8 {
        Val4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val5 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val5 {
    #[inline(always)]
    fn from(val: u8) -> Val5 {
        Val5::from_bits(val)
    }
}
impl From<Val5> for u8 {
    #[inline(always)]
    fn from(val: Val5) -> u8 {
        Val5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val6 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val6 {
    #[inline(always)]
    fn from(val: u8) -> Val6 {
        Val6::from_bits(val)
    }
}
impl From<Val6> for u8 {
    #[inline(always)]
    fn from(val: Val6) -> u8 {
        Val6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val7 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val7 {
    #[inline(always)]
    fn from(val: u8) -> Val7 {
        Val7::from_bits(val)
    }
}
impl From<Val7> for u8 {
    #[inline(always)]
    fn from(val: Val7) -> u8 {
        Val7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val8 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val8 {
    #[inline(always)]
    fn from(val: u8) -> Val8 {
        Val8::from_bits(val)
    }
}
impl From<Val8> for u8 {
    #[inline(always)]
    fn from(val: Val8) -> u8 {
        Val8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Val9 {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Val9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Val9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Val9 {
    #[inline(always)]
    fn from(val: u8) -> Val9 {
        Val9::from_bits(val)
    }
}
impl From<Val9> for u8 {
    #[inline(always)]
    fn from(val: Val9) -> u8 {
        Val9::to_bits(val)
    }
}
