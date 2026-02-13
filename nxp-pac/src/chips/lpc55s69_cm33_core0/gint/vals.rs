#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Comb {
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR = 0x0,
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND = 0x01,
}
impl Comb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Comb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Comb {
    #[inline(always)]
    fn from(val: u8) -> Comb {
        Comb::from_bits(val)
    }
}
impl From<Comb> for u8 {
    #[inline(always)]
    fn from(val: Comb) -> u8 {
        Comb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int {
    #[doc = "No request. No interrupt request is pending."]
    NO_REQUEST = 0x0,
    #[doc = "Request active. Interrupt request is active."]
    REQUEST_ACTIVE = 0x01,
}
impl Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int {
    #[inline(always)]
    fn from(val: u8) -> Int {
        Int::from_bits(val)
    }
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(val: Int) -> u8 {
        Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trig {
    #[doc = "Edge-triggered."]
    EDGE_TRIGGERED = 0x0,
    #[doc = "Level-triggered."]
    LEVEL_TRIGGERED = 0x01,
}
impl Trig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trig {
    #[inline(always)]
    fn from(val: u8) -> Trig {
        Trig::from_bits(val)
    }
}
impl From<Trig> for u8 {
    #[inline(always)]
    fn from(val: Trig) -> u8 {
        Trig::to_bits(val)
    }
}
