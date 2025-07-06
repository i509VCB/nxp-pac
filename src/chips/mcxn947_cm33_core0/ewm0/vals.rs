#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Assin {
    #[doc = "Logic 0"]
    DISABLE = 0x0,
    #[doc = "Logic 1"]
    ENABLE = 0x01,
}
impl Assin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Assin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Assin {
    #[inline(always)]
    fn from(val: u8) -> Assin {
        Assin::from_bits(val)
    }
}
impl From<Assin> for u8 {
    #[inline(always)]
    fn from(val: Assin) -> u8 {
        Assin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewmen {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Ewmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewmen {
    #[inline(always)]
    fn from(val: u8) -> Ewmen {
        Ewmen::from_bits(val)
    }
}
impl From<Ewmen> for u8 {
    #[inline(always)]
    fn from(val: Ewmen) -> u8 {
        Ewmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inen {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Inen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inen {
    #[inline(always)]
    fn from(val: u8) -> Inen {
        Inen::from_bits(val)
    }
}
impl From<Inen> for u8 {
    #[inline(always)]
    fn from(val: Inen) -> u8 {
        Inen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inten {
    #[doc = "Deasserts interrupt requests"]
    ZERO = 0x0,
    #[doc = "Generates interrupt requests"]
    INT_REQ = 0x01,
}
impl Inten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inten {
    #[inline(always)]
    fn from(val: u8) -> Inten {
        Inten::from_bits(val)
    }
}
impl From<Inten> for u8 {
    #[inline(always)]
    fn from(val: Inten) -> u8 {
        Inten::to_bits(val)
    }
}
