#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS0 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS0 {
    #[inline(always)]
    fn from(val: u8) -> INTS0 {
        INTS0::from_bits(val)
    }
}
impl From<INTS0> for u8 {
    #[inline(always)]
    fn from(val: INTS0) -> u8 {
        INTS0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS1 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS1 {
    #[inline(always)]
    fn from(val: u8) -> INTS1 {
        INTS1::from_bits(val)
    }
}
impl From<INTS1> for u8 {
    #[inline(always)]
    fn from(val: INTS1) -> u8 {
        INTS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS10 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS10 {
    #[inline(always)]
    fn from(val: u8) -> INTS10 {
        INTS10::from_bits(val)
    }
}
impl From<INTS10> for u8 {
    #[inline(always)]
    fn from(val: INTS10) -> u8 {
        INTS10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS11 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS11 {
    #[inline(always)]
    fn from(val: u8) -> INTS11 {
        INTS11::from_bits(val)
    }
}
impl From<INTS11> for u8 {
    #[inline(always)]
    fn from(val: INTS11) -> u8 {
        INTS11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS12 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS12 {
    #[inline(always)]
    fn from(val: u8) -> INTS12 {
        INTS12::from_bits(val)
    }
}
impl From<INTS12> for u8 {
    #[inline(always)]
    fn from(val: INTS12) -> u8 {
        INTS12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS13 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS13 {
    #[inline(always)]
    fn from(val: u8) -> INTS13 {
        INTS13::from_bits(val)
    }
}
impl From<INTS13> for u8 {
    #[inline(always)]
    fn from(val: INTS13) -> u8 {
        INTS13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS14 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS14 {
    #[inline(always)]
    fn from(val: u8) -> INTS14 {
        INTS14::from_bits(val)
    }
}
impl From<INTS14> for u8 {
    #[inline(always)]
    fn from(val: INTS14) -> u8 {
        INTS14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS15 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS15 {
    #[inline(always)]
    fn from(val: u8) -> INTS15 {
        INTS15::from_bits(val)
    }
}
impl From<INTS15> for u8 {
    #[inline(always)]
    fn from(val: INTS15) -> u8 {
        INTS15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS16 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS16 {
    #[inline(always)]
    fn from(val: u8) -> INTS16 {
        INTS16::from_bits(val)
    }
}
impl From<INTS16> for u8 {
    #[inline(always)]
    fn from(val: INTS16) -> u8 {
        INTS16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS17 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS17 {
    #[inline(always)]
    fn from(val: u8) -> INTS17 {
        INTS17::from_bits(val)
    }
}
impl From<INTS17> for u8 {
    #[inline(always)]
    fn from(val: INTS17) -> u8 {
        INTS17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS18 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS18 {
    #[inline(always)]
    fn from(val: u8) -> INTS18 {
        INTS18::from_bits(val)
    }
}
impl From<INTS18> for u8 {
    #[inline(always)]
    fn from(val: INTS18) -> u8 {
        INTS18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS19 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS19 {
    #[inline(always)]
    fn from(val: u8) -> INTS19 {
        INTS19::from_bits(val)
    }
}
impl From<INTS19> for u8 {
    #[inline(always)]
    fn from(val: INTS19) -> u8 {
        INTS19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS2 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS2 {
    #[inline(always)]
    fn from(val: u8) -> INTS2 {
        INTS2::from_bits(val)
    }
}
impl From<INTS2> for u8 {
    #[inline(always)]
    fn from(val: INTS2) -> u8 {
        INTS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS20 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS20 {
    #[inline(always)]
    fn from(val: u8) -> INTS20 {
        INTS20::from_bits(val)
    }
}
impl From<INTS20> for u8 {
    #[inline(always)]
    fn from(val: INTS20) -> u8 {
        INTS20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS21 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS21 {
    #[inline(always)]
    fn from(val: u8) -> INTS21 {
        INTS21::from_bits(val)
    }
}
impl From<INTS21> for u8 {
    #[inline(always)]
    fn from(val: INTS21) -> u8 {
        INTS21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS22 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS22 {
    #[inline(always)]
    fn from(val: u8) -> INTS22 {
        INTS22::from_bits(val)
    }
}
impl From<INTS22> for u8 {
    #[inline(always)]
    fn from(val: INTS22) -> u8 {
        INTS22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS23 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS23 {
    #[inline(always)]
    fn from(val: u8) -> INTS23 {
        INTS23::from_bits(val)
    }
}
impl From<INTS23> for u8 {
    #[inline(always)]
    fn from(val: INTS23) -> u8 {
        INTS23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS24 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS24 {
    #[inline(always)]
    fn from(val: u8) -> INTS24 {
        INTS24::from_bits(val)
    }
}
impl From<INTS24> for u8 {
    #[inline(always)]
    fn from(val: INTS24) -> u8 {
        INTS24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS25 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS25 {
    #[inline(always)]
    fn from(val: u8) -> INTS25 {
        INTS25::from_bits(val)
    }
}
impl From<INTS25> for u8 {
    #[inline(always)]
    fn from(val: INTS25) -> u8 {
        INTS25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS26 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS26 {
    #[inline(always)]
    fn from(val: u8) -> INTS26 {
        INTS26::from_bits(val)
    }
}
impl From<INTS26> for u8 {
    #[inline(always)]
    fn from(val: INTS26) -> u8 {
        INTS26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS27 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS27 {
    #[inline(always)]
    fn from(val: u8) -> INTS27 {
        INTS27::from_bits(val)
    }
}
impl From<INTS27> for u8 {
    #[inline(always)]
    fn from(val: INTS27) -> u8 {
        INTS27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS28 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS28 {
    #[inline(always)]
    fn from(val: u8) -> INTS28 {
        INTS28::from_bits(val)
    }
}
impl From<INTS28> for u8 {
    #[inline(always)]
    fn from(val: INTS28) -> u8 {
        INTS28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS29 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS29 {
    #[inline(always)]
    fn from(val: u8) -> INTS29 {
        INTS29::from_bits(val)
    }
}
impl From<INTS29> for u8 {
    #[inline(always)]
    fn from(val: INTS29) -> u8 {
        INTS29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS3 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS3 {
    #[inline(always)]
    fn from(val: u8) -> INTS3 {
        INTS3::from_bits(val)
    }
}
impl From<INTS3> for u8 {
    #[inline(always)]
    fn from(val: INTS3) -> u8 {
        INTS3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS30 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS30 {
    #[inline(always)]
    fn from(val: u8) -> INTS30 {
        INTS30::from_bits(val)
    }
}
impl From<INTS30> for u8 {
    #[inline(always)]
    fn from(val: INTS30) -> u8 {
        INTS30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS31 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS31 {
    #[inline(always)]
    fn from(val: u8) -> INTS31 {
        INTS31::from_bits(val)
    }
}
impl From<INTS31> for u8 {
    #[inline(always)]
    fn from(val: INTS31) -> u8 {
        INTS31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS4 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS4 {
    #[inline(always)]
    fn from(val: u8) -> INTS4 {
        INTS4::from_bits(val)
    }
}
impl From<INTS4> for u8 {
    #[inline(always)]
    fn from(val: INTS4) -> u8 {
        INTS4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS5 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS5 {
    #[inline(always)]
    fn from(val: u8) -> INTS5 {
        INTS5::from_bits(val)
    }
}
impl From<INTS5> for u8 {
    #[inline(always)]
    fn from(val: INTS5) -> u8 {
        INTS5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS6 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS6 {
    #[inline(always)]
    fn from(val: u8) -> INTS6 {
        INTS6::from_bits(val)
    }
}
impl From<INTS6> for u8 {
    #[inline(always)]
    fn from(val: INTS6) -> u8 {
        INTS6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS7 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS7 {
    #[inline(always)]
    fn from(val: u8) -> INTS7 {
        INTS7::from_bits(val)
    }
}
impl From<INTS7> for u8 {
    #[inline(always)]
    fn from(val: INTS7) -> u8 {
        INTS7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS8 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS8 {
    #[inline(always)]
    fn from(val: u8) -> INTS8 {
        INTS8::from_bits(val)
    }
}
impl From<INTS8> for u8 {
    #[inline(always)]
    fn from(val: INTS8) -> u8 {
        INTS8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTS9 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl INTS9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTS9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTS9 {
    #[inline(always)]
    fn from(val: u8) -> INTS9 {
        INTS9::from_bits(val)
    }
}
impl From<INTS9> for u8 {
    #[inline(always)]
    fn from(val: INTS9) -> u8 {
        INTS9::to_bits(val)
    }
}
