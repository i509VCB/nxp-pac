#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMB {
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR = 0x0,
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND = 0x01,
}
impl COMB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMB {
    #[inline(always)]
    fn from(val: u8) -> COMB {
        COMB::from_bits(val)
    }
}
impl From<COMB> for u8 {
    #[inline(always)]
    fn from(val: COMB) -> u8 {
        COMB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT {
    #[doc = "No request. No interrupt request is pending."]
    NO_REQUEST = 0x0,
    #[doc = "Request active. Interrupt request is active."]
    REQUEST_ACTIVE = 0x01,
}
impl INT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT {
    #[inline(always)]
    fn from(val: u8) -> INT {
        INT::from_bits(val)
    }
}
impl From<INT> for u8 {
    #[inline(always)]
    fn from(val: INT) -> u8 {
        INT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIG {
    #[doc = "Edge-triggered."]
    EDGE_TRIGGERED = 0x0,
    #[doc = "Level-triggered."]
    LEVEL_TRIGGERED = 0x01,
}
impl TRIG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIG {
    #[inline(always)]
    fn from(val: u8) -> TRIG {
        TRIG::from_bits(val)
    }
}
impl From<TRIG> for u8 {
    #[inline(always)]
    fn from(val: TRIG) -> u8 {
        TRIG::to_bits(val)
    }
}
