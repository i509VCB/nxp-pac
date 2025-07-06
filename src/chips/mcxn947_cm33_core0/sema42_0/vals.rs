#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate0Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate0Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate0Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate0Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate0Gtfsm {
        Gate0Gtfsm::from_bits(val)
    }
}
impl From<Gate0Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate0Gtfsm) -> u8 {
        Gate0Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate10Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate10Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate10Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate10Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate10Gtfsm {
        Gate10Gtfsm::from_bits(val)
    }
}
impl From<Gate10Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate10Gtfsm) -> u8 {
        Gate10Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate11Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate11Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate11Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate11Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate11Gtfsm {
        Gate11Gtfsm::from_bits(val)
    }
}
impl From<Gate11Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate11Gtfsm) -> u8 {
        Gate11Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate12Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate12Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate12Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate12Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate12Gtfsm {
        Gate12Gtfsm::from_bits(val)
    }
}
impl From<Gate12Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate12Gtfsm) -> u8 {
        Gate12Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate13Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate13Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate13Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate13Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate13Gtfsm {
        Gate13Gtfsm::from_bits(val)
    }
}
impl From<Gate13Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate13Gtfsm) -> u8 {
        Gate13Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate14Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate14Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate14Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate14Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate14Gtfsm {
        Gate14Gtfsm::from_bits(val)
    }
}
impl From<Gate14Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate14Gtfsm) -> u8 {
        Gate14Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate15Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate15Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate15Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate15Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate15Gtfsm {
        Gate15Gtfsm::from_bits(val)
    }
}
impl From<Gate15Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate15Gtfsm) -> u8 {
        Gate15Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate1Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate1Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate1Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate1Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate1Gtfsm {
        Gate1Gtfsm::from_bits(val)
    }
}
impl From<Gate1Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate1Gtfsm) -> u8 {
        Gate1Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate2Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate2Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate2Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate2Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate2Gtfsm {
        Gate2Gtfsm::from_bits(val)
    }
}
impl From<Gate2Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate2Gtfsm) -> u8 {
        Gate2Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate3Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate3Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate3Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate3Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate3Gtfsm {
        Gate3Gtfsm::from_bits(val)
    }
}
impl From<Gate3Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate3Gtfsm) -> u8 {
        Gate3Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate4Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate4Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate4Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate4Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate4Gtfsm {
        Gate4Gtfsm::from_bits(val)
    }
}
impl From<Gate4Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate4Gtfsm) -> u8 {
        Gate4Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate5Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate5Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate5Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate5Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate5Gtfsm {
        Gate5Gtfsm::from_bits(val)
    }
}
impl From<Gate5Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate5Gtfsm) -> u8 {
        Gate5Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate6Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate6Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate6Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate6Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate6Gtfsm {
        Gate6Gtfsm::from_bits(val)
    }
}
impl From<Gate6Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate6Gtfsm) -> u8 {
        Gate6Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate7Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate7Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate7Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate7Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate7Gtfsm {
        Gate7Gtfsm::from_bits(val)
    }
}
impl From<Gate7Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate7Gtfsm) -> u8 {
        Gate7Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate8Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate8Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate8Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate8Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate8Gtfsm {
        Gate8Gtfsm::from_bits(val)
    }
}
impl From<Gate8Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate8Gtfsm) -> u8 {
        Gate8Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate9Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate9Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate9Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate9Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate9Gtfsm {
        Gate9Gtfsm::from_bits(val)
    }
}
impl From<Gate9Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate9Gtfsm) -> u8 {
        Gate9Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstgsm {
    #[doc = "Idle, waiting for the first data pattern write."]
    IDLE = 0x0,
    #[doc = "Waiting for the second data pattern write"]
    WAITING = 0x01,
    #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
    TWO_WRITE_DONE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rstgsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstgsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstgsm {
    #[inline(always)]
    fn from(val: u8) -> Rstgsm {
        Rstgsm::from_bits(val)
    }
}
impl From<Rstgsm> for u8 {
    #[inline(always)]
    fn from(val: Rstgsm) -> u8 {
        Rstgsm::to_bits(val)
    }
}
