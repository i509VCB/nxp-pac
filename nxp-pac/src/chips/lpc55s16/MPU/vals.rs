#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENABLE {
    #[doc = "The MPU is disabled."]
    MPU_DISABLE = 0x0,
    #[doc = "The MPU is enabled."]
    MPU_ENABLED = 0x01,
}
impl ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENABLE {
    #[inline(always)]
    fn from(val: u8) -> ENABLE {
        ENABLE::from_bits(val)
    }
}
impl From<ENABLE> for u8 {
    #[inline(always)]
    fn from(val: ENABLE) -> u8 {
        ENABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PRIVDEFENA {
    #[doc = "Disables the default memory map. Any instruction or data access that does not access a defined region faults."]
    ENABLE = 0x0,
    #[doc = "Enables the default memory map as a background region for privileged accesses."]
    DISABLE = 0x01,
}
impl PRIVDEFENA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PRIVDEFENA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PRIVDEFENA {
    #[inline(always)]
    fn from(val: u8) -> PRIVDEFENA {
        PRIVDEFENA::from_bits(val)
    }
}
impl From<PRIVDEFENA> for u8 {
    #[inline(always)]
    fn from(val: PRIVDEFENA) -> u8 {
        PRIVDEFENA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A1_AP {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RBAR_A1_AP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A1_AP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A1_AP {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A1_AP {
        RBAR_A1_AP::from_bits(val)
    }
}
impl From<RBAR_A1_AP> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A1_AP) -> u8 {
        RBAR_A1_AP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A1_SH {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable."]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RBAR_A1_SH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A1_SH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A1_SH {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A1_SH {
        RBAR_A1_SH::from_bits(val)
    }
}
impl From<RBAR_A1_SH> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A1_SH) -> u8 {
        RBAR_A1_SH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A1_XN {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RBAR_A1_XN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A1_XN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A1_XN {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A1_XN {
        RBAR_A1_XN::from_bits(val)
    }
}
impl From<RBAR_A1_XN> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A1_XN) -> u8 {
        RBAR_A1_XN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A2_AP {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RBAR_A2_AP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A2_AP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A2_AP {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A2_AP {
        RBAR_A2_AP::from_bits(val)
    }
}
impl From<RBAR_A2_AP> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A2_AP) -> u8 {
        RBAR_A2_AP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A2_SH {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable."]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RBAR_A2_SH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A2_SH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A2_SH {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A2_SH {
        RBAR_A2_SH::from_bits(val)
    }
}
impl From<RBAR_A2_SH> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A2_SH) -> u8 {
        RBAR_A2_SH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A2_XN {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RBAR_A2_XN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A2_XN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A2_XN {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A2_XN {
        RBAR_A2_XN::from_bits(val)
    }
}
impl From<RBAR_A2_XN> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A2_XN) -> u8 {
        RBAR_A2_XN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A3_AP {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RBAR_A3_AP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A3_AP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A3_AP {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A3_AP {
        RBAR_A3_AP::from_bits(val)
    }
}
impl From<RBAR_A3_AP> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A3_AP) -> u8 {
        RBAR_A3_AP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A3_SH {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable."]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RBAR_A3_SH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A3_SH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A3_SH {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A3_SH {
        RBAR_A3_SH::from_bits(val)
    }
}
impl From<RBAR_A3_SH> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A3_SH) -> u8 {
        RBAR_A3_SH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_A3_XN {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RBAR_A3_XN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_A3_XN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_A3_XN {
    #[inline(always)]
    fn from(val: u8) -> RBAR_A3_XN {
        RBAR_A3_XN::from_bits(val)
    }
}
impl From<RBAR_A3_XN> for u8 {
    #[inline(always)]
    fn from(val: RBAR_A3_XN) -> u8 {
        RBAR_A3_XN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_AP {
    #[doc = "Read/write by privileged code only."]
    PRIVILEGED_CODE = 0x0,
    #[doc = "Read/write by any privilege level."]
    PRIVILEGED_ANY = 0x01,
    #[doc = "Read-only by privileged code only."]
    PRIVILEGED_CODE_READ_ONLY = 0x02,
    #[doc = "Read-only by any privilege level."]
    PRIVILEGED_ANY_READ_ONLY = 0x03,
}
impl RBAR_AP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_AP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_AP {
    #[inline(always)]
    fn from(val: u8) -> RBAR_AP {
        RBAR_AP::from_bits(val)
    }
}
impl From<RBAR_AP> for u8 {
    #[inline(always)]
    fn from(val: RBAR_AP) -> u8 {
        RBAR_AP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_SH {
    #[doc = "Non-shareable memory."]
    NON_SHAREABLE = 0x0,
    #[doc = "Unpredictable."]
    UNPREDICTABLE = 0x01,
    #[doc = "Outer shareable."]
    OUTER_SHAREABLE = 0x02,
    #[doc = "Inner Shareable."]
    INNER_SHAREABLE = 0x03,
}
impl RBAR_SH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_SH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_SH {
    #[inline(always)]
    fn from(val: u8) -> RBAR_SH {
        RBAR_SH::from_bits(val)
    }
}
impl From<RBAR_SH> for u8 {
    #[inline(always)]
    fn from(val: RBAR_SH) -> u8 {
        RBAR_SH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RBAR_XN {
    #[doc = "Execution is only permitted if read permitted."]
    EXECUTE = 0x0,
    #[doc = "Execution is not permitted."]
    EXECUTE_NEVER = 0x01,
}
impl RBAR_XN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RBAR_XN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RBAR_XN {
    #[inline(always)]
    fn from(val: u8) -> RBAR_XN {
        RBAR_XN::from_bits(val)
    }
}
impl From<RBAR_XN> for u8 {
    #[inline(always)]
    fn from(val: RBAR_XN) -> u8 {
        RBAR_XN::to_bits(val)
    }
}
