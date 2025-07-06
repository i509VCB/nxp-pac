#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su0 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su0 {
    #[inline(always)]
    fn from(val: u8) -> Su0 {
        Su0::from_bits(val)
    }
}
impl From<Su0> for u8 {
    #[inline(always)]
    fn from(val: Su0) -> u8 {
        Su0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su1 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su1 {
    #[inline(always)]
    fn from(val: u8) -> Su1 {
        Su1::from_bits(val)
    }
}
impl From<Su1> for u8 {
    #[inline(always)]
    fn from(val: Su1) -> u8 {
        Su1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su10 {
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The floating-point state is permitted to become UNKNOWN"]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su10 {
    #[inline(always)]
    fn from(val: u8) -> Su10 {
        Su10::from_bits(val)
    }
}
impl From<Su10> for u8 {
    #[inline(always)]
    fn from(val: Su10) -> u8 {
        Su10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su2 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su2 {
    #[inline(always)]
    fn from(val: u8) -> Su2 {
        Su2::from_bits(val)
    }
}
impl From<Su2> for u8 {
    #[inline(always)]
    fn from(val: Su2) -> u8 {
        Su2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su3 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su3 {
    #[inline(always)]
    fn from(val: u8) -> Su3 {
        Su3::from_bits(val)
    }
}
impl From<Su3> for u8 {
    #[inline(always)]
    fn from(val: Su3) -> u8 {
        Su3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su4 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su4 {
    #[inline(always)]
    fn from(val: u8) -> Su4 {
        Su4::from_bits(val)
    }
}
impl From<Su4> for u8 {
    #[inline(always)]
    fn from(val: Su4) -> u8 {
        Su4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su5 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su5 {
    #[inline(always)]
    fn from(val: u8) -> Su5 {
        Su5::from_bits(val)
    }
}
impl From<Su5> for u8 {
    #[inline(always)]
    fn from(val: Su5) -> u8 {
        Su5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su6 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su6 {
    #[inline(always)]
    fn from(val: u8) -> Su6 {
        Su6::from_bits(val)
    }
}
impl From<Su6> for u8 {
    #[inline(always)]
    fn from(val: Su6) -> u8 {
        Su6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su7 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su7 {
    #[inline(always)]
    fn from(val: u8) -> Su7 {
        Su7::from_bits(val)
    }
}
impl From<Su7> for u8 {
    #[inline(always)]
    fn from(val: Su7) -> u8 {
        Su7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus0 {
    #[doc = "The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus0 {
    #[inline(always)]
    fn from(val: u8) -> Sus0 {
        Sus0::from_bits(val)
    }
}
impl From<Sus0> for u8 {
    #[inline(always)]
    fn from(val: Sus0) -> u8 {
        Sus0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus1 {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus1 {
    #[inline(always)]
    fn from(val: u8) -> Sus1 {
        Sus1::from_bits(val)
    }
}
impl From<Sus1> for u8 {
    #[inline(always)]
    fn from(val: Sus1) -> u8 {
        Sus1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus10 {
    #[doc = "The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus10 {
    #[inline(always)]
    fn from(val: u8) -> Sus10 {
        Sus10::from_bits(val)
    }
}
impl From<Sus10> for u8 {
    #[inline(always)]
    fn from(val: Sus10) -> u8 {
        Sus10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus2 {
    #[doc = "The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus2 {
    #[inline(always)]
    fn from(val: u8) -> Sus2 {
        Sus2::from_bits(val)
    }
}
impl From<Sus2> for u8 {
    #[inline(always)]
    fn from(val: Sus2) -> u8 {
        Sus2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus3 {
    #[doc = "The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus3 {
    #[inline(always)]
    fn from(val: u8) -> Sus3 {
        Sus3::from_bits(val)
    }
}
impl From<Sus3> for u8 {
    #[inline(always)]
    fn from(val: Sus3) -> u8 {
        Sus3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus4 {
    #[doc = "The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus4 {
    #[inline(always)]
    fn from(val: u8) -> Sus4 {
        Sus4::from_bits(val)
    }
}
impl From<Sus4> for u8 {
    #[inline(always)]
    fn from(val: Sus4) -> u8 {
        Sus4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus5 {
    #[doc = "The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus5 {
    #[inline(always)]
    fn from(val: u8) -> Sus5 {
        Sus5::from_bits(val)
    }
}
impl From<Sus5> for u8 {
    #[inline(always)]
    fn from(val: Sus5) -> u8 {
        Sus5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus6 {
    #[doc = "The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus6 {
    #[inline(always)]
    fn from(val: u8) -> Sus6 {
        Sus6::from_bits(val)
    }
}
impl From<Sus6> for u8 {
    #[inline(always)]
    fn from(val: Sus6) -> u8 {
        Sus6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus7 {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus7 {
    #[inline(always)]
    fn from(val: u8) -> Sus7 {
        Sus7::from_bits(val)
    }
}
impl From<Sus7> for u8 {
    #[inline(always)]
    fn from(val: Sus7) -> u8 {
        Sus7::to_bits(val)
    }
}
