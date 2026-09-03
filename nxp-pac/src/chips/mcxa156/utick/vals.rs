#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "Inactive (stopped)."]
    Timerisnotactive = 0x0,
    #[doc = "Active."]
    Timerisactive = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr0 {
    #[doc = "Does nothing."]
    Capclr0nothing = 0x0,
    #[doc = "Clears the CAP0 register value."]
    Capclr0cleared = 0x01,
}
impl Capclr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr0 {
    #[inline(always)]
    fn from(val: u8) -> Capclr0 {
        Capclr0::from_bits(val)
    }
}
impl From<Capclr0> for u8 {
    #[inline(always)]
    fn from(val: Capclr0) -> u8 {
        Capclr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr1 {
    #[doc = "Does nothing."]
    Capclr1nothing = 0x0,
    #[doc = "Clears the CAP1 register value."]
    Capclr1cleared = 0x01,
}
impl Capclr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr1 {
    #[inline(always)]
    fn from(val: u8) -> Capclr1 {
        Capclr1::from_bits(val)
    }
}
impl From<Capclr1> for u8 {
    #[inline(always)]
    fn from(val: Capclr1) -> u8 {
        Capclr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr2 {
    #[doc = "Does nothing."]
    Capclr2nothing = 0x0,
    #[doc = "Clears the CAP2 register value."]
    Capclr2cleared = 0x01,
}
impl Capclr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr2 {
    #[inline(always)]
    fn from(val: u8) -> Capclr2 {
        Capclr2::from_bits(val)
    }
}
impl From<Capclr2> for u8 {
    #[inline(always)]
    fn from(val: Capclr2) -> u8 {
        Capclr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr3 {
    #[doc = "Does nothing."]
    Capclr3nothing = 0x0,
    #[doc = "Clears the CAP3 register value."]
    Capclr3cleared = 0x01,
}
impl Capclr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr3 {
    #[inline(always)]
    fn from(val: u8) -> Capclr3 {
        Capclr3::from_bits(val)
    }
}
impl From<Capclr3> for u8 {
    #[inline(always)]
    fn from(val: Capclr3) -> u8 {
        Capclr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen0 {
    #[doc = "Disable."]
    Capen0isdisabled = 0x0,
    #[doc = "Enable."]
    Capen0isenabled = 0x01,
}
impl Capen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen0 {
    #[inline(always)]
    fn from(val: u8) -> Capen0 {
        Capen0::from_bits(val)
    }
}
impl From<Capen0> for u8 {
    #[inline(always)]
    fn from(val: Capen0) -> u8 {
        Capen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen1 {
    #[doc = "Disable."]
    Capen1isdisabled = 0x0,
    #[doc = "Enable."]
    Capen1isenabled = 0x01,
}
impl Capen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen1 {
    #[inline(always)]
    fn from(val: u8) -> Capen1 {
        Capen1::from_bits(val)
    }
}
impl From<Capen1> for u8 {
    #[inline(always)]
    fn from(val: Capen1) -> u8 {
        Capen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen2 {
    #[doc = "Disable."]
    Capen2isdisabled = 0x0,
    #[doc = "Enable."]
    Capen2isenabled = 0x01,
}
impl Capen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen2 {
    #[inline(always)]
    fn from(val: u8) -> Capen2 {
        Capen2::from_bits(val)
    }
}
impl From<Capen2> for u8 {
    #[inline(always)]
    fn from(val: Capen2) -> u8 {
        Capen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen3 {
    #[doc = "Disable."]
    Capen3isdisabled = 0x0,
    #[doc = "Enable."]
    Capen3isenabled = 0x01,
}
impl Capen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen3 {
    #[inline(always)]
    fn from(val: u8) -> Capen3 {
        Capen3::from_bits(val)
    }
}
impl From<Capen3> for u8 {
    #[inline(always)]
    fn from(val: Capen3) -> u8 {
        Capen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol0 {
    #[doc = "Positive."]
    Cappol0posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol0negedgecapture = 0x01,
}
impl Cappol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol0 {
    #[inline(always)]
    fn from(val: u8) -> Cappol0 {
        Cappol0::from_bits(val)
    }
}
impl From<Cappol0> for u8 {
    #[inline(always)]
    fn from(val: Cappol0) -> u8 {
        Cappol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol1 {
    #[doc = "Positive."]
    Cappol1posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol1negedgecapture = 0x01,
}
impl Cappol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol1 {
    #[inline(always)]
    fn from(val: u8) -> Cappol1 {
        Cappol1::from_bits(val)
    }
}
impl From<Cappol1> for u8 {
    #[inline(always)]
    fn from(val: Cappol1) -> u8 {
        Cappol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol2 {
    #[doc = "Positive."]
    Cappol2posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol2negedgecapture = 0x01,
}
impl Cappol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol2 {
    #[inline(always)]
    fn from(val: u8) -> Cappol2 {
        Cappol2::from_bits(val)
    }
}
impl From<Cappol2> for u8 {
    #[inline(always)]
    fn from(val: Cappol2) -> u8 {
        Cappol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol3 {
    #[doc = "Positive."]
    Cappol3posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol3negedgecapture = 0x01,
}
impl Cappol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol3 {
    #[inline(always)]
    fn from(val: u8) -> Cappol3 {
        Cappol3::from_bits(val)
    }
}
impl From<Cappol3> for u8 {
    #[inline(always)]
    fn from(val: Cappol3) -> u8 {
        Cappol3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repeat {
    #[doc = "One-time delay."]
    Delayonce = 0x0,
    #[doc = "Delay repeats continuously."]
    Delayrepeats = 0x01,
}
impl Repeat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repeat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repeat {
    #[inline(always)]
    fn from(val: u8) -> Repeat {
        Repeat::from_bits(val)
    }
}
impl From<Repeat> for u8 {
    #[inline(always)]
    fn from(val: Repeat) -> u8 {
        Repeat::to_bits(val)
    }
}
