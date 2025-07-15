#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints0 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints0 {
    #[inline(always)]
    fn from(val: u8) -> Ints0 {
        Ints0::from_bits(val)
    }
}
impl From<Ints0> for u8 {
    #[inline(always)]
    fn from(val: Ints0) -> u8 {
        Ints0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints1 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints1 {
    #[inline(always)]
    fn from(val: u8) -> Ints1 {
        Ints1::from_bits(val)
    }
}
impl From<Ints1> for u8 {
    #[inline(always)]
    fn from(val: Ints1) -> u8 {
        Ints1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints10 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints10 {
    #[inline(always)]
    fn from(val: u8) -> Ints10 {
        Ints10::from_bits(val)
    }
}
impl From<Ints10> for u8 {
    #[inline(always)]
    fn from(val: Ints10) -> u8 {
        Ints10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints11 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints11 {
    #[inline(always)]
    fn from(val: u8) -> Ints11 {
        Ints11::from_bits(val)
    }
}
impl From<Ints11> for u8 {
    #[inline(always)]
    fn from(val: Ints11) -> u8 {
        Ints11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints12 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints12 {
    #[inline(always)]
    fn from(val: u8) -> Ints12 {
        Ints12::from_bits(val)
    }
}
impl From<Ints12> for u8 {
    #[inline(always)]
    fn from(val: Ints12) -> u8 {
        Ints12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints13 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints13 {
    #[inline(always)]
    fn from(val: u8) -> Ints13 {
        Ints13::from_bits(val)
    }
}
impl From<Ints13> for u8 {
    #[inline(always)]
    fn from(val: Ints13) -> u8 {
        Ints13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints14 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints14 {
    #[inline(always)]
    fn from(val: u8) -> Ints14 {
        Ints14::from_bits(val)
    }
}
impl From<Ints14> for u8 {
    #[inline(always)]
    fn from(val: Ints14) -> u8 {
        Ints14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints15 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints15 {
    #[inline(always)]
    fn from(val: u8) -> Ints15 {
        Ints15::from_bits(val)
    }
}
impl From<Ints15> for u8 {
    #[inline(always)]
    fn from(val: Ints15) -> u8 {
        Ints15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints16 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints16 {
    #[inline(always)]
    fn from(val: u8) -> Ints16 {
        Ints16::from_bits(val)
    }
}
impl From<Ints16> for u8 {
    #[inline(always)]
    fn from(val: Ints16) -> u8 {
        Ints16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints17 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints17 {
    #[inline(always)]
    fn from(val: u8) -> Ints17 {
        Ints17::from_bits(val)
    }
}
impl From<Ints17> for u8 {
    #[inline(always)]
    fn from(val: Ints17) -> u8 {
        Ints17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints18 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints18 {
    #[inline(always)]
    fn from(val: u8) -> Ints18 {
        Ints18::from_bits(val)
    }
}
impl From<Ints18> for u8 {
    #[inline(always)]
    fn from(val: Ints18) -> u8 {
        Ints18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints19 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints19 {
    #[inline(always)]
    fn from(val: u8) -> Ints19 {
        Ints19::from_bits(val)
    }
}
impl From<Ints19> for u8 {
    #[inline(always)]
    fn from(val: Ints19) -> u8 {
        Ints19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints2 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints2 {
    #[inline(always)]
    fn from(val: u8) -> Ints2 {
        Ints2::from_bits(val)
    }
}
impl From<Ints2> for u8 {
    #[inline(always)]
    fn from(val: Ints2) -> u8 {
        Ints2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints20 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints20 {
    #[inline(always)]
    fn from(val: u8) -> Ints20 {
        Ints20::from_bits(val)
    }
}
impl From<Ints20> for u8 {
    #[inline(always)]
    fn from(val: Ints20) -> u8 {
        Ints20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints21 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints21 {
    #[inline(always)]
    fn from(val: u8) -> Ints21 {
        Ints21::from_bits(val)
    }
}
impl From<Ints21> for u8 {
    #[inline(always)]
    fn from(val: Ints21) -> u8 {
        Ints21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints22 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints22 {
    #[inline(always)]
    fn from(val: u8) -> Ints22 {
        Ints22::from_bits(val)
    }
}
impl From<Ints22> for u8 {
    #[inline(always)]
    fn from(val: Ints22) -> u8 {
        Ints22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints23 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints23 {
    #[inline(always)]
    fn from(val: u8) -> Ints23 {
        Ints23::from_bits(val)
    }
}
impl From<Ints23> for u8 {
    #[inline(always)]
    fn from(val: Ints23) -> u8 {
        Ints23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints24 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints24 {
    #[inline(always)]
    fn from(val: u8) -> Ints24 {
        Ints24::from_bits(val)
    }
}
impl From<Ints24> for u8 {
    #[inline(always)]
    fn from(val: Ints24) -> u8 {
        Ints24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints25 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints25 {
    #[inline(always)]
    fn from(val: u8) -> Ints25 {
        Ints25::from_bits(val)
    }
}
impl From<Ints25> for u8 {
    #[inline(always)]
    fn from(val: Ints25) -> u8 {
        Ints25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints26 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints26 {
    #[inline(always)]
    fn from(val: u8) -> Ints26 {
        Ints26::from_bits(val)
    }
}
impl From<Ints26> for u8 {
    #[inline(always)]
    fn from(val: Ints26) -> u8 {
        Ints26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints27 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints27 {
    #[inline(always)]
    fn from(val: u8) -> Ints27 {
        Ints27::from_bits(val)
    }
}
impl From<Ints27> for u8 {
    #[inline(always)]
    fn from(val: Ints27) -> u8 {
        Ints27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints28 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints28 {
    #[inline(always)]
    fn from(val: u8) -> Ints28 {
        Ints28::from_bits(val)
    }
}
impl From<Ints28> for u8 {
    #[inline(always)]
    fn from(val: Ints28) -> u8 {
        Ints28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints29 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints29 {
    #[inline(always)]
    fn from(val: u8) -> Ints29 {
        Ints29::from_bits(val)
    }
}
impl From<Ints29> for u8 {
    #[inline(always)]
    fn from(val: Ints29) -> u8 {
        Ints29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints3 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints3 {
    #[inline(always)]
    fn from(val: u8) -> Ints3 {
        Ints3::from_bits(val)
    }
}
impl From<Ints3> for u8 {
    #[inline(always)]
    fn from(val: Ints3) -> u8 {
        Ints3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints30 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints30 {
    #[inline(always)]
    fn from(val: u8) -> Ints30 {
        Ints30::from_bits(val)
    }
}
impl From<Ints30> for u8 {
    #[inline(always)]
    fn from(val: Ints30) -> u8 {
        Ints30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints31 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints31 {
    #[inline(always)]
    fn from(val: u8) -> Ints31 {
        Ints31::from_bits(val)
    }
}
impl From<Ints31> for u8 {
    #[inline(always)]
    fn from(val: Ints31) -> u8 {
        Ints31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints4 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints4 {
    #[inline(always)]
    fn from(val: u8) -> Ints4 {
        Ints4::from_bits(val)
    }
}
impl From<Ints4> for u8 {
    #[inline(always)]
    fn from(val: Ints4) -> u8 {
        Ints4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints5 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints5 {
    #[inline(always)]
    fn from(val: u8) -> Ints5 {
        Ints5::from_bits(val)
    }
}
impl From<Ints5> for u8 {
    #[inline(always)]
    fn from(val: Ints5) -> u8 {
        Ints5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints6 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints6 {
    #[inline(always)]
    fn from(val: u8) -> Ints6 {
        Ints6::from_bits(val)
    }
}
impl From<Ints6> for u8 {
    #[inline(always)]
    fn from(val: Ints6) -> u8 {
        Ints6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints7 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints7 {
    #[inline(always)]
    fn from(val: u8) -> Ints7 {
        Ints7::from_bits(val)
    }
}
impl From<Ints7> for u8 {
    #[inline(always)]
    fn from(val: Ints7) -> u8 {
        Ints7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints8 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints8 {
    #[inline(always)]
    fn from(val: u8) -> Ints8 {
        Ints8::from_bits(val)
    }
}
impl From<Ints8> for u8 {
    #[inline(always)]
    fn from(val: Ints8) -> u8 {
        Ints8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ints9 {
    #[doc = "The interrupt targets Secure state."]
    SECURE_STATE = 0x0,
    #[doc = "The interrupt targets Non-secure state."]
    NON_SECURE_STATE = 0x01,
}
impl Ints9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ints9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ints9 {
    #[inline(always)]
    fn from(val: u8) -> Ints9 {
        Ints9::from_bits(val)
    }
}
impl From<Ints9> for u8 {
    #[inline(always)]
    fn from(val: Ints9) -> u8 {
        Ints9::to_bits(val)
    }
}
