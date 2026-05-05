#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU0 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU0 {
    #[inline(always)]
    fn from(val: u8) -> SU0 {
        SU0::from_bits(val)
    }
}
impl From<SU0> for u8 {
    #[inline(always)]
    fn from(val: SU0) -> u8 {
        SU0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU1 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU1 {
    #[inline(always)]
    fn from(val: u8) -> SU1 {
        SU1::from_bits(val)
    }
}
impl From<SU1> for u8 {
    #[inline(always)]
    fn from(val: SU1) -> u8 {
        SU1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU10 {
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The floating-point state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU10 {
    #[inline(always)]
    fn from(val: u8) -> SU10 {
        SU10::from_bits(val)
    }
}
impl From<SU10> for u8 {
    #[inline(always)]
    fn from(val: SU10) -> u8 {
        SU10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU2 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU2 {
    #[inline(always)]
    fn from(val: u8) -> SU2 {
        SU2::from_bits(val)
    }
}
impl From<SU2> for u8 {
    #[inline(always)]
    fn from(val: SU2) -> u8 {
        SU2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU3 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU3 {
    #[inline(always)]
    fn from(val: u8) -> SU3 {
        SU3::from_bits(val)
    }
}
impl From<SU3> for u8 {
    #[inline(always)]
    fn from(val: SU3) -> u8 {
        SU3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU4 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU4 {
    #[inline(always)]
    fn from(val: u8) -> SU4 {
        SU4::from_bits(val)
    }
}
impl From<SU4> for u8 {
    #[inline(always)]
    fn from(val: SU4) -> u8 {
        SU4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU5 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU5 {
    #[inline(always)]
    fn from(val: u8) -> SU5 {
        SU5::from_bits(val)
    }
}
impl From<SU5> for u8 {
    #[inline(always)]
    fn from(val: SU5) -> u8 {
        SU5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU6 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU6 {
    #[inline(always)]
    fn from(val: u8) -> SU6 {
        SU6::from_bits(val)
    }
}
impl From<SU6> for u8 {
    #[inline(always)]
    fn from(val: SU6) -> u8 {
        SU6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SU7 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl SU7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SU7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SU7 {
    #[inline(always)]
    fn from(val: u8) -> SU7 {
        SU7::from_bits(val)
    }
}
impl From<SU7> for u8 {
    #[inline(always)]
    fn from(val: SU7) -> u8 {
        SU7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS0 {
    #[doc = "The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS0 {
    #[inline(always)]
    fn from(val: u8) -> SUS0 {
        SUS0::from_bits(val)
    }
}
impl From<SUS0> for u8 {
    #[inline(always)]
    fn from(val: SUS0) -> u8 {
        SUS0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS1 {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS1 {
    #[inline(always)]
    fn from(val: u8) -> SUS1 {
        SUS1::from_bits(val)
    }
}
impl From<SUS1> for u8 {
    #[inline(always)]
    fn from(val: SUS1) -> u8 {
        SUS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS10 {
    #[doc = "The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS10 {
    #[inline(always)]
    fn from(val: u8) -> SUS10 {
        SUS10::from_bits(val)
    }
}
impl From<SUS10> for u8 {
    #[inline(always)]
    fn from(val: SUS10) -> u8 {
        SUS10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS2 {
    #[doc = "The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS2 {
    #[inline(always)]
    fn from(val: u8) -> SUS2 {
        SUS2::from_bits(val)
    }
}
impl From<SUS2> for u8 {
    #[inline(always)]
    fn from(val: SUS2) -> u8 {
        SUS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS3 {
    #[doc = "The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS3 {
    #[inline(always)]
    fn from(val: u8) -> SUS3 {
        SUS3::from_bits(val)
    }
}
impl From<SUS3> for u8 {
    #[inline(always)]
    fn from(val: SUS3) -> u8 {
        SUS3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS4 {
    #[doc = "The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS4 {
    #[inline(always)]
    fn from(val: u8) -> SUS4 {
        SUS4::from_bits(val)
    }
}
impl From<SUS4> for u8 {
    #[inline(always)]
    fn from(val: SUS4) -> u8 {
        SUS4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS5 {
    #[doc = "The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS5 {
    #[inline(always)]
    fn from(val: u8) -> SUS5 {
        SUS5::from_bits(val)
    }
}
impl From<SUS5> for u8 {
    #[inline(always)]
    fn from(val: SUS5) -> u8 {
        SUS5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS6 {
    #[doc = "The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS6 {
    #[inline(always)]
    fn from(val: u8) -> SUS6 {
        SUS6::from_bits(val)
    }
}
impl From<SUS6> for u8 {
    #[inline(always)]
    fn from(val: SUS6) -> u8 {
        SUS6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUS7 {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl SUS7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUS7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUS7 {
    #[inline(always)]
    fn from(val: u8) -> SUS7 {
        SUS7::from_bits(val)
    }
}
impl From<SUS7> for u8 {
    #[inline(always)]
    fn from(val: SUS7) -> u8 {
        SUS7::to_bits(val)
    }
}
