#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active0 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active0 {
    #[inline(always)]
    fn from(val: u8) -> Active0 {
        Active0::from_bits(val)
    }
}
impl From<Active0> for u8 {
    #[inline(always)]
    fn from(val: Active0) -> u8 {
        Active0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active1 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active1 {
    #[inline(always)]
    fn from(val: u8) -> Active1 {
        Active1::from_bits(val)
    }
}
impl From<Active1> for u8 {
    #[inline(always)]
    fn from(val: Active1) -> u8 {
        Active1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active10 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active10 {
    #[inline(always)]
    fn from(val: u8) -> Active10 {
        Active10::from_bits(val)
    }
}
impl From<Active10> for u8 {
    #[inline(always)]
    fn from(val: Active10) -> u8 {
        Active10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active11 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active11 {
    #[inline(always)]
    fn from(val: u8) -> Active11 {
        Active11::from_bits(val)
    }
}
impl From<Active11> for u8 {
    #[inline(always)]
    fn from(val: Active11) -> u8 {
        Active11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active12 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active12 {
    #[inline(always)]
    fn from(val: u8) -> Active12 {
        Active12::from_bits(val)
    }
}
impl From<Active12> for u8 {
    #[inline(always)]
    fn from(val: Active12) -> u8 {
        Active12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active13 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active13 {
    #[inline(always)]
    fn from(val: u8) -> Active13 {
        Active13::from_bits(val)
    }
}
impl From<Active13> for u8 {
    #[inline(always)]
    fn from(val: Active13) -> u8 {
        Active13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active14 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active14 {
    #[inline(always)]
    fn from(val: u8) -> Active14 {
        Active14::from_bits(val)
    }
}
impl From<Active14> for u8 {
    #[inline(always)]
    fn from(val: Active14) -> u8 {
        Active14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active15 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active15 {
    #[inline(always)]
    fn from(val: u8) -> Active15 {
        Active15::from_bits(val)
    }
}
impl From<Active15> for u8 {
    #[inline(always)]
    fn from(val: Active15) -> u8 {
        Active15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active16 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active16 {
    #[inline(always)]
    fn from(val: u8) -> Active16 {
        Active16::from_bits(val)
    }
}
impl From<Active16> for u8 {
    #[inline(always)]
    fn from(val: Active16) -> u8 {
        Active16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active17 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active17 {
    #[inline(always)]
    fn from(val: u8) -> Active17 {
        Active17::from_bits(val)
    }
}
impl From<Active17> for u8 {
    #[inline(always)]
    fn from(val: Active17) -> u8 {
        Active17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active18 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active18 {
    #[inline(always)]
    fn from(val: u8) -> Active18 {
        Active18::from_bits(val)
    }
}
impl From<Active18> for u8 {
    #[inline(always)]
    fn from(val: Active18) -> u8 {
        Active18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active19 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active19 {
    #[inline(always)]
    fn from(val: u8) -> Active19 {
        Active19::from_bits(val)
    }
}
impl From<Active19> for u8 {
    #[inline(always)]
    fn from(val: Active19) -> u8 {
        Active19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active2 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active2 {
    #[inline(always)]
    fn from(val: u8) -> Active2 {
        Active2::from_bits(val)
    }
}
impl From<Active2> for u8 {
    #[inline(always)]
    fn from(val: Active2) -> u8 {
        Active2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active20 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active20 {
    #[inline(always)]
    fn from(val: u8) -> Active20 {
        Active20::from_bits(val)
    }
}
impl From<Active20> for u8 {
    #[inline(always)]
    fn from(val: Active20) -> u8 {
        Active20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active21 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active21 {
    #[inline(always)]
    fn from(val: u8) -> Active21 {
        Active21::from_bits(val)
    }
}
impl From<Active21> for u8 {
    #[inline(always)]
    fn from(val: Active21) -> u8 {
        Active21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active22 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active22 {
    #[inline(always)]
    fn from(val: u8) -> Active22 {
        Active22::from_bits(val)
    }
}
impl From<Active22> for u8 {
    #[inline(always)]
    fn from(val: Active22) -> u8 {
        Active22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active23 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active23 {
    #[inline(always)]
    fn from(val: u8) -> Active23 {
        Active23::from_bits(val)
    }
}
impl From<Active23> for u8 {
    #[inline(always)]
    fn from(val: Active23) -> u8 {
        Active23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active24 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active24 {
    #[inline(always)]
    fn from(val: u8) -> Active24 {
        Active24::from_bits(val)
    }
}
impl From<Active24> for u8 {
    #[inline(always)]
    fn from(val: Active24) -> u8 {
        Active24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active25 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active25 {
    #[inline(always)]
    fn from(val: u8) -> Active25 {
        Active25::from_bits(val)
    }
}
impl From<Active25> for u8 {
    #[inline(always)]
    fn from(val: Active25) -> u8 {
        Active25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active26 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active26 {
    #[inline(always)]
    fn from(val: u8) -> Active26 {
        Active26::from_bits(val)
    }
}
impl From<Active26> for u8 {
    #[inline(always)]
    fn from(val: Active26) -> u8 {
        Active26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active27 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active27 {
    #[inline(always)]
    fn from(val: u8) -> Active27 {
        Active27::from_bits(val)
    }
}
impl From<Active27> for u8 {
    #[inline(always)]
    fn from(val: Active27) -> u8 {
        Active27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active28 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active28 {
    #[inline(always)]
    fn from(val: u8) -> Active28 {
        Active28::from_bits(val)
    }
}
impl From<Active28> for u8 {
    #[inline(always)]
    fn from(val: Active28) -> u8 {
        Active28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active29 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active29 {
    #[inline(always)]
    fn from(val: u8) -> Active29 {
        Active29::from_bits(val)
    }
}
impl From<Active29> for u8 {
    #[inline(always)]
    fn from(val: Active29) -> u8 {
        Active29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active3 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active3 {
    #[inline(always)]
    fn from(val: u8) -> Active3 {
        Active3::from_bits(val)
    }
}
impl From<Active3> for u8 {
    #[inline(always)]
    fn from(val: Active3) -> u8 {
        Active3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active30 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active30 {
    #[inline(always)]
    fn from(val: u8) -> Active30 {
        Active30::from_bits(val)
    }
}
impl From<Active30> for u8 {
    #[inline(always)]
    fn from(val: Active30) -> u8 {
        Active30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active31 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active31 {
    #[inline(always)]
    fn from(val: u8) -> Active31 {
        Active31::from_bits(val)
    }
}
impl From<Active31> for u8 {
    #[inline(always)]
    fn from(val: Active31) -> u8 {
        Active31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active4 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active4 {
    #[inline(always)]
    fn from(val: u8) -> Active4 {
        Active4::from_bits(val)
    }
}
impl From<Active4> for u8 {
    #[inline(always)]
    fn from(val: Active4) -> u8 {
        Active4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active5 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active5 {
    #[inline(always)]
    fn from(val: u8) -> Active5 {
        Active5::from_bits(val)
    }
}
impl From<Active5> for u8 {
    #[inline(always)]
    fn from(val: Active5) -> u8 {
        Active5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active6 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active6 {
    #[inline(always)]
    fn from(val: u8) -> Active6 {
        Active6::from_bits(val)
    }
}
impl From<Active6> for u8 {
    #[inline(always)]
    fn from(val: Active6) -> u8 {
        Active6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active7 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active7 {
    #[inline(always)]
    fn from(val: u8) -> Active7 {
        Active7::from_bits(val)
    }
}
impl From<Active7> for u8 {
    #[inline(always)]
    fn from(val: Active7) -> u8 {
        Active7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active8 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active8 {
    #[inline(always)]
    fn from(val: u8) -> Active8 {
        Active8::from_bits(val)
    }
}
impl From<Active8> for u8 {
    #[inline(always)]
    fn from(val: Active8) -> u8 {
        Active8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active9 {
    #[doc = "The interrupt is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The interrupt is active."]
    ACTIVE = 0x01,
}
impl Active9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active9 {
    #[inline(always)]
    fn from(val: u8) -> Active9 {
        Active9::from_bits(val)
    }
}
impl From<Active9> for u8 {
    #[inline(always)]
    fn from(val: Active9) -> u8 {
        Active9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena0 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena0 {
    #[inline(always)]
    fn from(val: u8) -> Clrena0 {
        Clrena0::from_bits(val)
    }
}
impl From<Clrena0> for u8 {
    #[inline(always)]
    fn from(val: Clrena0) -> u8 {
        Clrena0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena1 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena1 {
    #[inline(always)]
    fn from(val: u8) -> Clrena1 {
        Clrena1::from_bits(val)
    }
}
impl From<Clrena1> for u8 {
    #[inline(always)]
    fn from(val: Clrena1) -> u8 {
        Clrena1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena10 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena10 {
    #[inline(always)]
    fn from(val: u8) -> Clrena10 {
        Clrena10::from_bits(val)
    }
}
impl From<Clrena10> for u8 {
    #[inline(always)]
    fn from(val: Clrena10) -> u8 {
        Clrena10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena11 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena11 {
    #[inline(always)]
    fn from(val: u8) -> Clrena11 {
        Clrena11::from_bits(val)
    }
}
impl From<Clrena11> for u8 {
    #[inline(always)]
    fn from(val: Clrena11) -> u8 {
        Clrena11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena12 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena12 {
    #[inline(always)]
    fn from(val: u8) -> Clrena12 {
        Clrena12::from_bits(val)
    }
}
impl From<Clrena12> for u8 {
    #[inline(always)]
    fn from(val: Clrena12) -> u8 {
        Clrena12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena13 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena13 {
    #[inline(always)]
    fn from(val: u8) -> Clrena13 {
        Clrena13::from_bits(val)
    }
}
impl From<Clrena13> for u8 {
    #[inline(always)]
    fn from(val: Clrena13) -> u8 {
        Clrena13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena14 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena14 {
    #[inline(always)]
    fn from(val: u8) -> Clrena14 {
        Clrena14::from_bits(val)
    }
}
impl From<Clrena14> for u8 {
    #[inline(always)]
    fn from(val: Clrena14) -> u8 {
        Clrena14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena15 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena15 {
    #[inline(always)]
    fn from(val: u8) -> Clrena15 {
        Clrena15::from_bits(val)
    }
}
impl From<Clrena15> for u8 {
    #[inline(always)]
    fn from(val: Clrena15) -> u8 {
        Clrena15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena16 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena16 {
    #[inline(always)]
    fn from(val: u8) -> Clrena16 {
        Clrena16::from_bits(val)
    }
}
impl From<Clrena16> for u8 {
    #[inline(always)]
    fn from(val: Clrena16) -> u8 {
        Clrena16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena17 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena17 {
    #[inline(always)]
    fn from(val: u8) -> Clrena17 {
        Clrena17::from_bits(val)
    }
}
impl From<Clrena17> for u8 {
    #[inline(always)]
    fn from(val: Clrena17) -> u8 {
        Clrena17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena18 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena18 {
    #[inline(always)]
    fn from(val: u8) -> Clrena18 {
        Clrena18::from_bits(val)
    }
}
impl From<Clrena18> for u8 {
    #[inline(always)]
    fn from(val: Clrena18) -> u8 {
        Clrena18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena19 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena19 {
    #[inline(always)]
    fn from(val: u8) -> Clrena19 {
        Clrena19::from_bits(val)
    }
}
impl From<Clrena19> for u8 {
    #[inline(always)]
    fn from(val: Clrena19) -> u8 {
        Clrena19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena2 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena2 {
    #[inline(always)]
    fn from(val: u8) -> Clrena2 {
        Clrena2::from_bits(val)
    }
}
impl From<Clrena2> for u8 {
    #[inline(always)]
    fn from(val: Clrena2) -> u8 {
        Clrena2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena20 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena20 {
    #[inline(always)]
    fn from(val: u8) -> Clrena20 {
        Clrena20::from_bits(val)
    }
}
impl From<Clrena20> for u8 {
    #[inline(always)]
    fn from(val: Clrena20) -> u8 {
        Clrena20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena21 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena21 {
    #[inline(always)]
    fn from(val: u8) -> Clrena21 {
        Clrena21::from_bits(val)
    }
}
impl From<Clrena21> for u8 {
    #[inline(always)]
    fn from(val: Clrena21) -> u8 {
        Clrena21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena22 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena22 {
    #[inline(always)]
    fn from(val: u8) -> Clrena22 {
        Clrena22::from_bits(val)
    }
}
impl From<Clrena22> for u8 {
    #[inline(always)]
    fn from(val: Clrena22) -> u8 {
        Clrena22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena23 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena23 {
    #[inline(always)]
    fn from(val: u8) -> Clrena23 {
        Clrena23::from_bits(val)
    }
}
impl From<Clrena23> for u8 {
    #[inline(always)]
    fn from(val: Clrena23) -> u8 {
        Clrena23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena24 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena24 {
    #[inline(always)]
    fn from(val: u8) -> Clrena24 {
        Clrena24::from_bits(val)
    }
}
impl From<Clrena24> for u8 {
    #[inline(always)]
    fn from(val: Clrena24) -> u8 {
        Clrena24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena25 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena25 {
    #[inline(always)]
    fn from(val: u8) -> Clrena25 {
        Clrena25::from_bits(val)
    }
}
impl From<Clrena25> for u8 {
    #[inline(always)]
    fn from(val: Clrena25) -> u8 {
        Clrena25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena26 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena26 {
    #[inline(always)]
    fn from(val: u8) -> Clrena26 {
        Clrena26::from_bits(val)
    }
}
impl From<Clrena26> for u8 {
    #[inline(always)]
    fn from(val: Clrena26) -> u8 {
        Clrena26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena27 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena27 {
    #[inline(always)]
    fn from(val: u8) -> Clrena27 {
        Clrena27::from_bits(val)
    }
}
impl From<Clrena27> for u8 {
    #[inline(always)]
    fn from(val: Clrena27) -> u8 {
        Clrena27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena28 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena28 {
    #[inline(always)]
    fn from(val: u8) -> Clrena28 {
        Clrena28::from_bits(val)
    }
}
impl From<Clrena28> for u8 {
    #[inline(always)]
    fn from(val: Clrena28) -> u8 {
        Clrena28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena29 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena29 {
    #[inline(always)]
    fn from(val: u8) -> Clrena29 {
        Clrena29::from_bits(val)
    }
}
impl From<Clrena29> for u8 {
    #[inline(always)]
    fn from(val: Clrena29) -> u8 {
        Clrena29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena3 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena3 {
    #[inline(always)]
    fn from(val: u8) -> Clrena3 {
        Clrena3::from_bits(val)
    }
}
impl From<Clrena3> for u8 {
    #[inline(always)]
    fn from(val: Clrena3) -> u8 {
        Clrena3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena30 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena30 {
    #[inline(always)]
    fn from(val: u8) -> Clrena30 {
        Clrena30::from_bits(val)
    }
}
impl From<Clrena30> for u8 {
    #[inline(always)]
    fn from(val: Clrena30) -> u8 {
        Clrena30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena31 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena31 {
    #[inline(always)]
    fn from(val: u8) -> Clrena31 {
        Clrena31::from_bits(val)
    }
}
impl From<Clrena31> for u8 {
    #[inline(always)]
    fn from(val: Clrena31) -> u8 {
        Clrena31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena4 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena4 {
    #[inline(always)]
    fn from(val: u8) -> Clrena4 {
        Clrena4::from_bits(val)
    }
}
impl From<Clrena4> for u8 {
    #[inline(always)]
    fn from(val: Clrena4) -> u8 {
        Clrena4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena5 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena5 {
    #[inline(always)]
    fn from(val: u8) -> Clrena5 {
        Clrena5::from_bits(val)
    }
}
impl From<Clrena5> for u8 {
    #[inline(always)]
    fn from(val: Clrena5) -> u8 {
        Clrena5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena6 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena6 {
    #[inline(always)]
    fn from(val: u8) -> Clrena6 {
        Clrena6::from_bits(val)
    }
}
impl From<Clrena6> for u8 {
    #[inline(always)]
    fn from(val: Clrena6) -> u8 {
        Clrena6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena7 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena7 {
    #[inline(always)]
    fn from(val: u8) -> Clrena7 {
        Clrena7::from_bits(val)
    }
}
impl From<Clrena7> for u8 {
    #[inline(always)]
    fn from(val: Clrena7) -> u8 {
        Clrena7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena8 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena8 {
    #[inline(always)]
    fn from(val: u8) -> Clrena8 {
        Clrena8::from_bits(val)
    }
}
impl From<Clrena8> for u8 {
    #[inline(always)]
    fn from(val: Clrena8) -> u8 {
        Clrena8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrena9 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Clrena9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrena9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrena9 {
    #[inline(always)]
    fn from(val: u8) -> Clrena9 {
        Clrena9::from_bits(val)
    }
}
impl From<Clrena9> for u8 {
    #[inline(always)]
    fn from(val: Clrena9) -> u8 {
        Clrena9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend0 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend0 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend0 {
        Clrpend0::from_bits(val)
    }
}
impl From<Clrpend0> for u8 {
    #[inline(always)]
    fn from(val: Clrpend0) -> u8 {
        Clrpend0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend1 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend1 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend1 {
        Clrpend1::from_bits(val)
    }
}
impl From<Clrpend1> for u8 {
    #[inline(always)]
    fn from(val: Clrpend1) -> u8 {
        Clrpend1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend10 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend10 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend10 {
        Clrpend10::from_bits(val)
    }
}
impl From<Clrpend10> for u8 {
    #[inline(always)]
    fn from(val: Clrpend10) -> u8 {
        Clrpend10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend11 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend11 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend11 {
        Clrpend11::from_bits(val)
    }
}
impl From<Clrpend11> for u8 {
    #[inline(always)]
    fn from(val: Clrpend11) -> u8 {
        Clrpend11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend12 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend12 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend12 {
        Clrpend12::from_bits(val)
    }
}
impl From<Clrpend12> for u8 {
    #[inline(always)]
    fn from(val: Clrpend12) -> u8 {
        Clrpend12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend13 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend13 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend13 {
        Clrpend13::from_bits(val)
    }
}
impl From<Clrpend13> for u8 {
    #[inline(always)]
    fn from(val: Clrpend13) -> u8 {
        Clrpend13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend14 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend14 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend14 {
        Clrpend14::from_bits(val)
    }
}
impl From<Clrpend14> for u8 {
    #[inline(always)]
    fn from(val: Clrpend14) -> u8 {
        Clrpend14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend15 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend15 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend15 {
        Clrpend15::from_bits(val)
    }
}
impl From<Clrpend15> for u8 {
    #[inline(always)]
    fn from(val: Clrpend15) -> u8 {
        Clrpend15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend16 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend16 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend16 {
        Clrpend16::from_bits(val)
    }
}
impl From<Clrpend16> for u8 {
    #[inline(always)]
    fn from(val: Clrpend16) -> u8 {
        Clrpend16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend17 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend17 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend17 {
        Clrpend17::from_bits(val)
    }
}
impl From<Clrpend17> for u8 {
    #[inline(always)]
    fn from(val: Clrpend17) -> u8 {
        Clrpend17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend18 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend18 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend18 {
        Clrpend18::from_bits(val)
    }
}
impl From<Clrpend18> for u8 {
    #[inline(always)]
    fn from(val: Clrpend18) -> u8 {
        Clrpend18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend19 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend19 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend19 {
        Clrpend19::from_bits(val)
    }
}
impl From<Clrpend19> for u8 {
    #[inline(always)]
    fn from(val: Clrpend19) -> u8 {
        Clrpend19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend2 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend2 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend2 {
        Clrpend2::from_bits(val)
    }
}
impl From<Clrpend2> for u8 {
    #[inline(always)]
    fn from(val: Clrpend2) -> u8 {
        Clrpend2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend20 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend20 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend20 {
        Clrpend20::from_bits(val)
    }
}
impl From<Clrpend20> for u8 {
    #[inline(always)]
    fn from(val: Clrpend20) -> u8 {
        Clrpend20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend21 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend21 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend21 {
        Clrpend21::from_bits(val)
    }
}
impl From<Clrpend21> for u8 {
    #[inline(always)]
    fn from(val: Clrpend21) -> u8 {
        Clrpend21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend22 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend22 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend22 {
        Clrpend22::from_bits(val)
    }
}
impl From<Clrpend22> for u8 {
    #[inline(always)]
    fn from(val: Clrpend22) -> u8 {
        Clrpend22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend23 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend23 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend23 {
        Clrpend23::from_bits(val)
    }
}
impl From<Clrpend23> for u8 {
    #[inline(always)]
    fn from(val: Clrpend23) -> u8 {
        Clrpend23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend24 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend24 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend24 {
        Clrpend24::from_bits(val)
    }
}
impl From<Clrpend24> for u8 {
    #[inline(always)]
    fn from(val: Clrpend24) -> u8 {
        Clrpend24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend25 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend25 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend25 {
        Clrpend25::from_bits(val)
    }
}
impl From<Clrpend25> for u8 {
    #[inline(always)]
    fn from(val: Clrpend25) -> u8 {
        Clrpend25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend26 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend26 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend26 {
        Clrpend26::from_bits(val)
    }
}
impl From<Clrpend26> for u8 {
    #[inline(always)]
    fn from(val: Clrpend26) -> u8 {
        Clrpend26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend27 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend27 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend27 {
        Clrpend27::from_bits(val)
    }
}
impl From<Clrpend27> for u8 {
    #[inline(always)]
    fn from(val: Clrpend27) -> u8 {
        Clrpend27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend28 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend28 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend28 {
        Clrpend28::from_bits(val)
    }
}
impl From<Clrpend28> for u8 {
    #[inline(always)]
    fn from(val: Clrpend28) -> u8 {
        Clrpend28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend29 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend29 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend29 {
        Clrpend29::from_bits(val)
    }
}
impl From<Clrpend29> for u8 {
    #[inline(always)]
    fn from(val: Clrpend29) -> u8 {
        Clrpend29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend3 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend3 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend3 {
        Clrpend3::from_bits(val)
    }
}
impl From<Clrpend3> for u8 {
    #[inline(always)]
    fn from(val: Clrpend3) -> u8 {
        Clrpend3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend30 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend30 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend30 {
        Clrpend30::from_bits(val)
    }
}
impl From<Clrpend30> for u8 {
    #[inline(always)]
    fn from(val: Clrpend30) -> u8 {
        Clrpend30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend31 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend31 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend31 {
        Clrpend31::from_bits(val)
    }
}
impl From<Clrpend31> for u8 {
    #[inline(always)]
    fn from(val: Clrpend31) -> u8 {
        Clrpend31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend4 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend4 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend4 {
        Clrpend4::from_bits(val)
    }
}
impl From<Clrpend4> for u8 {
    #[inline(always)]
    fn from(val: Clrpend4) -> u8 {
        Clrpend4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend5 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend5 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend5 {
        Clrpend5::from_bits(val)
    }
}
impl From<Clrpend5> for u8 {
    #[inline(always)]
    fn from(val: Clrpend5) -> u8 {
        Clrpend5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend6 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend6 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend6 {
        Clrpend6::from_bits(val)
    }
}
impl From<Clrpend6> for u8 {
    #[inline(always)]
    fn from(val: Clrpend6) -> u8 {
        Clrpend6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend7 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend7 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend7 {
        Clrpend7::from_bits(val)
    }
}
impl From<Clrpend7> for u8 {
    #[inline(always)]
    fn from(val: Clrpend7) -> u8 {
        Clrpend7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend8 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend8 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend8 {
        Clrpend8::from_bits(val)
    }
}
impl From<Clrpend8> for u8 {
    #[inline(always)]
    fn from(val: Clrpend8) -> u8 {
        Clrpend8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrpend9 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Clear pending state of interrupt 32n+m; Read: Interrupt 32n+m is pending"]
    PENDING = 0x01,
}
impl Clrpend9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrpend9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrpend9 {
    #[inline(always)]
    fn from(val: u8) -> Clrpend9 {
        Clrpend9::from_bits(val)
    }
}
impl From<Clrpend9> for u8 {
    #[inline(always)]
    fn from(val: Clrpend9) -> u8 {
        Clrpend9::to_bits(val)
    }
}
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena0 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena0 {
    #[inline(always)]
    fn from(val: u8) -> Setena0 {
        Setena0::from_bits(val)
    }
}
impl From<Setena0> for u8 {
    #[inline(always)]
    fn from(val: Setena0) -> u8 {
        Setena0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena1 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena1 {
    #[inline(always)]
    fn from(val: u8) -> Setena1 {
        Setena1::from_bits(val)
    }
}
impl From<Setena1> for u8 {
    #[inline(always)]
    fn from(val: Setena1) -> u8 {
        Setena1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena10 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena10 {
    #[inline(always)]
    fn from(val: u8) -> Setena10 {
        Setena10::from_bits(val)
    }
}
impl From<Setena10> for u8 {
    #[inline(always)]
    fn from(val: Setena10) -> u8 {
        Setena10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena11 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena11 {
    #[inline(always)]
    fn from(val: u8) -> Setena11 {
        Setena11::from_bits(val)
    }
}
impl From<Setena11> for u8 {
    #[inline(always)]
    fn from(val: Setena11) -> u8 {
        Setena11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena12 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena12 {
    #[inline(always)]
    fn from(val: u8) -> Setena12 {
        Setena12::from_bits(val)
    }
}
impl From<Setena12> for u8 {
    #[inline(always)]
    fn from(val: Setena12) -> u8 {
        Setena12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena13 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena13 {
    #[inline(always)]
    fn from(val: u8) -> Setena13 {
        Setena13::from_bits(val)
    }
}
impl From<Setena13> for u8 {
    #[inline(always)]
    fn from(val: Setena13) -> u8 {
        Setena13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena14 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena14 {
    #[inline(always)]
    fn from(val: u8) -> Setena14 {
        Setena14::from_bits(val)
    }
}
impl From<Setena14> for u8 {
    #[inline(always)]
    fn from(val: Setena14) -> u8 {
        Setena14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena15 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena15 {
    #[inline(always)]
    fn from(val: u8) -> Setena15 {
        Setena15::from_bits(val)
    }
}
impl From<Setena15> for u8 {
    #[inline(always)]
    fn from(val: Setena15) -> u8 {
        Setena15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena16 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena16 {
    #[inline(always)]
    fn from(val: u8) -> Setena16 {
        Setena16::from_bits(val)
    }
}
impl From<Setena16> for u8 {
    #[inline(always)]
    fn from(val: Setena16) -> u8 {
        Setena16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena17 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena17 {
    #[inline(always)]
    fn from(val: u8) -> Setena17 {
        Setena17::from_bits(val)
    }
}
impl From<Setena17> for u8 {
    #[inline(always)]
    fn from(val: Setena17) -> u8 {
        Setena17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena18 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena18 {
    #[inline(always)]
    fn from(val: u8) -> Setena18 {
        Setena18::from_bits(val)
    }
}
impl From<Setena18> for u8 {
    #[inline(always)]
    fn from(val: Setena18) -> u8 {
        Setena18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena19 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena19 {
    #[inline(always)]
    fn from(val: u8) -> Setena19 {
        Setena19::from_bits(val)
    }
}
impl From<Setena19> for u8 {
    #[inline(always)]
    fn from(val: Setena19) -> u8 {
        Setena19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena2 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena2 {
    #[inline(always)]
    fn from(val: u8) -> Setena2 {
        Setena2::from_bits(val)
    }
}
impl From<Setena2> for u8 {
    #[inline(always)]
    fn from(val: Setena2) -> u8 {
        Setena2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena20 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena20 {
    #[inline(always)]
    fn from(val: u8) -> Setena20 {
        Setena20::from_bits(val)
    }
}
impl From<Setena20> for u8 {
    #[inline(always)]
    fn from(val: Setena20) -> u8 {
        Setena20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena21 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena21 {
    #[inline(always)]
    fn from(val: u8) -> Setena21 {
        Setena21::from_bits(val)
    }
}
impl From<Setena21> for u8 {
    #[inline(always)]
    fn from(val: Setena21) -> u8 {
        Setena21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena22 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena22 {
    #[inline(always)]
    fn from(val: u8) -> Setena22 {
        Setena22::from_bits(val)
    }
}
impl From<Setena22> for u8 {
    #[inline(always)]
    fn from(val: Setena22) -> u8 {
        Setena22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena23 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena23 {
    #[inline(always)]
    fn from(val: u8) -> Setena23 {
        Setena23::from_bits(val)
    }
}
impl From<Setena23> for u8 {
    #[inline(always)]
    fn from(val: Setena23) -> u8 {
        Setena23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena24 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena24 {
    #[inline(always)]
    fn from(val: u8) -> Setena24 {
        Setena24::from_bits(val)
    }
}
impl From<Setena24> for u8 {
    #[inline(always)]
    fn from(val: Setena24) -> u8 {
        Setena24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena25 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena25 {
    #[inline(always)]
    fn from(val: u8) -> Setena25 {
        Setena25::from_bits(val)
    }
}
impl From<Setena25> for u8 {
    #[inline(always)]
    fn from(val: Setena25) -> u8 {
        Setena25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena26 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena26 {
    #[inline(always)]
    fn from(val: u8) -> Setena26 {
        Setena26::from_bits(val)
    }
}
impl From<Setena26> for u8 {
    #[inline(always)]
    fn from(val: Setena26) -> u8 {
        Setena26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena27 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena27 {
    #[inline(always)]
    fn from(val: u8) -> Setena27 {
        Setena27::from_bits(val)
    }
}
impl From<Setena27> for u8 {
    #[inline(always)]
    fn from(val: Setena27) -> u8 {
        Setena27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena28 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena28 {
    #[inline(always)]
    fn from(val: u8) -> Setena28 {
        Setena28::from_bits(val)
    }
}
impl From<Setena28> for u8 {
    #[inline(always)]
    fn from(val: Setena28) -> u8 {
        Setena28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena29 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena29 {
    #[inline(always)]
    fn from(val: u8) -> Setena29 {
        Setena29::from_bits(val)
    }
}
impl From<Setena29> for u8 {
    #[inline(always)]
    fn from(val: Setena29) -> u8 {
        Setena29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena3 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena3 {
    #[inline(always)]
    fn from(val: u8) -> Setena3 {
        Setena3::from_bits(val)
    }
}
impl From<Setena3> for u8 {
    #[inline(always)]
    fn from(val: Setena3) -> u8 {
        Setena3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena30 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena30 {
    #[inline(always)]
    fn from(val: u8) -> Setena30 {
        Setena30::from_bits(val)
    }
}
impl From<Setena30> for u8 {
    #[inline(always)]
    fn from(val: Setena30) -> u8 {
        Setena30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena31 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena31 {
    #[inline(always)]
    fn from(val: u8) -> Setena31 {
        Setena31::from_bits(val)
    }
}
impl From<Setena31> for u8 {
    #[inline(always)]
    fn from(val: Setena31) -> u8 {
        Setena31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena4 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena4 {
    #[inline(always)]
    fn from(val: u8) -> Setena4 {
        Setena4::from_bits(val)
    }
}
impl From<Setena4> for u8 {
    #[inline(always)]
    fn from(val: Setena4) -> u8 {
        Setena4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena5 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena5 {
    #[inline(always)]
    fn from(val: u8) -> Setena5 {
        Setena5::from_bits(val)
    }
}
impl From<Setena5> for u8 {
    #[inline(always)]
    fn from(val: Setena5) -> u8 {
        Setena5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena6 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena6 {
    #[inline(always)]
    fn from(val: u8) -> Setena6 {
        Setena6::from_bits(val)
    }
}
impl From<Setena6> for u8 {
    #[inline(always)]
    fn from(val: Setena6) -> u8 {
        Setena6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena7 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena7 {
    #[inline(always)]
    fn from(val: u8) -> Setena7 {
        Setena7::from_bits(val)
    }
}
impl From<Setena7> for u8 {
    #[inline(always)]
    fn from(val: Setena7) -> u8 {
        Setena7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena8 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena8 {
    #[inline(always)]
    fn from(val: u8) -> Setena8 {
        Setena8::from_bits(val)
    }
}
impl From<Setena8> for u8 {
    #[inline(always)]
    fn from(val: Setena8) -> u8 {
        Setena8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setena9 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m disabled"]
    DISABLED = 0x0,
    #[doc = "Write: Enable interrupt 32n+m; Read: Interrupt 32n+m enabled"]
    ENABLED = 0x01,
}
impl Setena9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setena9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setena9 {
    #[inline(always)]
    fn from(val: u8) -> Setena9 {
        Setena9::from_bits(val)
    }
}
impl From<Setena9> for u8 {
    #[inline(always)]
    fn from(val: Setena9) -> u8 {
        Setena9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend0 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend0 {
    #[inline(always)]
    fn from(val: u8) -> Setpend0 {
        Setpend0::from_bits(val)
    }
}
impl From<Setpend0> for u8 {
    #[inline(always)]
    fn from(val: Setpend0) -> u8 {
        Setpend0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend1 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend1 {
    #[inline(always)]
    fn from(val: u8) -> Setpend1 {
        Setpend1::from_bits(val)
    }
}
impl From<Setpend1> for u8 {
    #[inline(always)]
    fn from(val: Setpend1) -> u8 {
        Setpend1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend10 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend10 {
    #[inline(always)]
    fn from(val: u8) -> Setpend10 {
        Setpend10::from_bits(val)
    }
}
impl From<Setpend10> for u8 {
    #[inline(always)]
    fn from(val: Setpend10) -> u8 {
        Setpend10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend11 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend11 {
    #[inline(always)]
    fn from(val: u8) -> Setpend11 {
        Setpend11::from_bits(val)
    }
}
impl From<Setpend11> for u8 {
    #[inline(always)]
    fn from(val: Setpend11) -> u8 {
        Setpend11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend12 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend12 {
    #[inline(always)]
    fn from(val: u8) -> Setpend12 {
        Setpend12::from_bits(val)
    }
}
impl From<Setpend12> for u8 {
    #[inline(always)]
    fn from(val: Setpend12) -> u8 {
        Setpend12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend13 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend13 {
    #[inline(always)]
    fn from(val: u8) -> Setpend13 {
        Setpend13::from_bits(val)
    }
}
impl From<Setpend13> for u8 {
    #[inline(always)]
    fn from(val: Setpend13) -> u8 {
        Setpend13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend14 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend14 {
    #[inline(always)]
    fn from(val: u8) -> Setpend14 {
        Setpend14::from_bits(val)
    }
}
impl From<Setpend14> for u8 {
    #[inline(always)]
    fn from(val: Setpend14) -> u8 {
        Setpend14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend15 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend15 {
    #[inline(always)]
    fn from(val: u8) -> Setpend15 {
        Setpend15::from_bits(val)
    }
}
impl From<Setpend15> for u8 {
    #[inline(always)]
    fn from(val: Setpend15) -> u8 {
        Setpend15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend16 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend16 {
    #[inline(always)]
    fn from(val: u8) -> Setpend16 {
        Setpend16::from_bits(val)
    }
}
impl From<Setpend16> for u8 {
    #[inline(always)]
    fn from(val: Setpend16) -> u8 {
        Setpend16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend17 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend17 {
    #[inline(always)]
    fn from(val: u8) -> Setpend17 {
        Setpend17::from_bits(val)
    }
}
impl From<Setpend17> for u8 {
    #[inline(always)]
    fn from(val: Setpend17) -> u8 {
        Setpend17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend18 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend18 {
    #[inline(always)]
    fn from(val: u8) -> Setpend18 {
        Setpend18::from_bits(val)
    }
}
impl From<Setpend18> for u8 {
    #[inline(always)]
    fn from(val: Setpend18) -> u8 {
        Setpend18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend19 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend19 {
    #[inline(always)]
    fn from(val: u8) -> Setpend19 {
        Setpend19::from_bits(val)
    }
}
impl From<Setpend19> for u8 {
    #[inline(always)]
    fn from(val: Setpend19) -> u8 {
        Setpend19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend2 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend2 {
    #[inline(always)]
    fn from(val: u8) -> Setpend2 {
        Setpend2::from_bits(val)
    }
}
impl From<Setpend2> for u8 {
    #[inline(always)]
    fn from(val: Setpend2) -> u8 {
        Setpend2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend20 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend20 {
    #[inline(always)]
    fn from(val: u8) -> Setpend20 {
        Setpend20::from_bits(val)
    }
}
impl From<Setpend20> for u8 {
    #[inline(always)]
    fn from(val: Setpend20) -> u8 {
        Setpend20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend21 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend21 {
    #[inline(always)]
    fn from(val: u8) -> Setpend21 {
        Setpend21::from_bits(val)
    }
}
impl From<Setpend21> for u8 {
    #[inline(always)]
    fn from(val: Setpend21) -> u8 {
        Setpend21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend22 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend22 {
    #[inline(always)]
    fn from(val: u8) -> Setpend22 {
        Setpend22::from_bits(val)
    }
}
impl From<Setpend22> for u8 {
    #[inline(always)]
    fn from(val: Setpend22) -> u8 {
        Setpend22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend23 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend23 {
    #[inline(always)]
    fn from(val: u8) -> Setpend23 {
        Setpend23::from_bits(val)
    }
}
impl From<Setpend23> for u8 {
    #[inline(always)]
    fn from(val: Setpend23) -> u8 {
        Setpend23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend24 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend24 {
    #[inline(always)]
    fn from(val: u8) -> Setpend24 {
        Setpend24::from_bits(val)
    }
}
impl From<Setpend24> for u8 {
    #[inline(always)]
    fn from(val: Setpend24) -> u8 {
        Setpend24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend25 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend25 {
    #[inline(always)]
    fn from(val: u8) -> Setpend25 {
        Setpend25::from_bits(val)
    }
}
impl From<Setpend25> for u8 {
    #[inline(always)]
    fn from(val: Setpend25) -> u8 {
        Setpend25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend26 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend26 {
    #[inline(always)]
    fn from(val: u8) -> Setpend26 {
        Setpend26::from_bits(val)
    }
}
impl From<Setpend26> for u8 {
    #[inline(always)]
    fn from(val: Setpend26) -> u8 {
        Setpend26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend27 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend27 {
    #[inline(always)]
    fn from(val: u8) -> Setpend27 {
        Setpend27::from_bits(val)
    }
}
impl From<Setpend27> for u8 {
    #[inline(always)]
    fn from(val: Setpend27) -> u8 {
        Setpend27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend28 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend28 {
    #[inline(always)]
    fn from(val: u8) -> Setpend28 {
        Setpend28::from_bits(val)
    }
}
impl From<Setpend28> for u8 {
    #[inline(always)]
    fn from(val: Setpend28) -> u8 {
        Setpend28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend29 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend29 {
    #[inline(always)]
    fn from(val: u8) -> Setpend29 {
        Setpend29::from_bits(val)
    }
}
impl From<Setpend29> for u8 {
    #[inline(always)]
    fn from(val: Setpend29) -> u8 {
        Setpend29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend3 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend3 {
    #[inline(always)]
    fn from(val: u8) -> Setpend3 {
        Setpend3::from_bits(val)
    }
}
impl From<Setpend3> for u8 {
    #[inline(always)]
    fn from(val: Setpend3) -> u8 {
        Setpend3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend30 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend30 {
    #[inline(always)]
    fn from(val: u8) -> Setpend30 {
        Setpend30::from_bits(val)
    }
}
impl From<Setpend30> for u8 {
    #[inline(always)]
    fn from(val: Setpend30) -> u8 {
        Setpend30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend31 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend31 {
    #[inline(always)]
    fn from(val: u8) -> Setpend31 {
        Setpend31::from_bits(val)
    }
}
impl From<Setpend31> for u8 {
    #[inline(always)]
    fn from(val: Setpend31) -> u8 {
        Setpend31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend4 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend4 {
    #[inline(always)]
    fn from(val: u8) -> Setpend4 {
        Setpend4::from_bits(val)
    }
}
impl From<Setpend4> for u8 {
    #[inline(always)]
    fn from(val: Setpend4) -> u8 {
        Setpend4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend5 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend5 {
    #[inline(always)]
    fn from(val: u8) -> Setpend5 {
        Setpend5::from_bits(val)
    }
}
impl From<Setpend5> for u8 {
    #[inline(always)]
    fn from(val: Setpend5) -> u8 {
        Setpend5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend6 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend6 {
    #[inline(always)]
    fn from(val: u8) -> Setpend6 {
        Setpend6::from_bits(val)
    }
}
impl From<Setpend6> for u8 {
    #[inline(always)]
    fn from(val: Setpend6) -> u8 {
        Setpend6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend7 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend7 {
    #[inline(always)]
    fn from(val: u8) -> Setpend7 {
        Setpend7::from_bits(val)
    }
}
impl From<Setpend7> for u8 {
    #[inline(always)]
    fn from(val: Setpend7) -> u8 {
        Setpend7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend8 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend8 {
    #[inline(always)]
    fn from(val: u8) -> Setpend8 {
        Setpend8::from_bits(val)
    }
}
impl From<Setpend8> for u8 {
    #[inline(always)]
    fn from(val: Setpend8) -> u8 {
        Setpend8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setpend9 {
    #[doc = "Write: No effect; Read: Interrupt 32n+m is not pending"]
    NOT_PENDING = 0x0,
    #[doc = "Write: Pend interrupt 32n+m; Read: Interrupt 32n+m pending"]
    PENDING = 0x01,
}
impl Setpend9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setpend9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setpend9 {
    #[inline(always)]
    fn from(val: u8) -> Setpend9 {
        Setpend9::from_bits(val)
    }
}
impl From<Setpend9> for u8 {
    #[inline(always)]
    fn from(val: Setpend9) -> u8 {
        Setpend9::to_bits(val)
    }
}
