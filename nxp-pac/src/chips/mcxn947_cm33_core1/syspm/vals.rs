#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmode {
    #[doc = "Counted in both User and Privileged modes"]
    USER_AND_PRIV = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Counted only in User mode"]
    USER_ONLY = 0x02,
    #[doc = "Counted only in Privileged mode"]
    PRIV_ONLY = 0x03,
}
impl Cmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmode {
    #[inline(always)]
    fn from(val: u8) -> Cmode {
        Cmode::from_bits(val)
    }
}
impl From<Cmode> for u8 {
    #[inline(always)]
    fn from(val: Cmode) -> u8 {
        Cmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rectr1 {
    #[doc = "Run normally"]
    RUN = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl Rectr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rectr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rectr1 {
    #[inline(always)]
    fn from(val: u8) -> Rectr1 {
        Rectr1::from_bits(val)
    }
}
impl From<Rectr1> for u8 {
    #[inline(always)]
    fn from(val: Rectr1) -> u8 {
        Rectr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rectr2 {
    #[doc = "Run normally"]
    RUN = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl Rectr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rectr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rectr2 {
    #[inline(always)]
    fn from(val: u8) -> Rectr2 {
        Rectr2::from_bits(val)
    }
}
impl From<Rectr2> for u8 {
    #[inline(always)]
    fn from(val: Rectr2) -> u8 {
        Rectr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rectr3 {
    #[doc = "Run normally"]
    RUN = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl Rectr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rectr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rectr3 {
    #[inline(always)]
    fn from(val: u8) -> Rectr3 {
        Rectr3::from_bits(val)
    }
}
impl From<Rectr3> for u8 {
    #[inline(always)]
    fn from(val: Rectr3) -> u8 {
        Rectr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssc {
    #[doc = "Idle or no-op"]
    IDLE = 0x0,
    #[doc = "Local stop"]
    LSTOP = 0x01,
    #[doc = "Local start"]
    LSTART_2 = 0x02,
    #[doc = "Local start"]
    LSTART_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ssc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssc {
    #[inline(always)]
    fn from(val: u8) -> Ssc {
        Ssc::from_bits(val)
    }
}
impl From<Ssc> for u8 {
    #[inline(always)]
    fn from(val: Ssc) -> u8 {
        Ssc::to_bits(val)
    }
}
