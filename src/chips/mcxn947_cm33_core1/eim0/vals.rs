#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich0en {
    #[doc = "Error injection is disabled on Error Injection Channel 0"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 0"]
    ENABLE = 0x01,
}
impl Eich0en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich0en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich0en {
    #[inline(always)]
    fn from(val: u8) -> Eich0en {
        Eich0en::from_bits(val)
    }
}
impl From<Eich0en> for u8 {
    #[inline(always)]
    fn from(val: Eich0en) -> u8 {
        Eich0en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich1en {
    #[doc = "Error injection is disabled on Error Injection Channel 1"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 1"]
    ENABLE = 0x01,
}
impl Eich1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich1en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich1en {
    #[inline(always)]
    fn from(val: u8) -> Eich1en {
        Eich1en::from_bits(val)
    }
}
impl From<Eich1en> for u8 {
    #[inline(always)]
    fn from(val: Eich1en) -> u8 {
        Eich1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich2en {
    #[doc = "Error injection is disabled on Error Injection Channel 2"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 2"]
    ENABLE = 0x01,
}
impl Eich2en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich2en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich2en {
    #[inline(always)]
    fn from(val: u8) -> Eich2en {
        Eich2en::from_bits(val)
    }
}
impl From<Eich2en> for u8 {
    #[inline(always)]
    fn from(val: Eich2en) -> u8 {
        Eich2en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich3en {
    #[doc = "Error injection is disabled on Error Injection Channel 3"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 3"]
    ENABLE = 0x01,
}
impl Eich3en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich3en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich3en {
    #[inline(always)]
    fn from(val: u8) -> Eich3en {
        Eich3en::from_bits(val)
    }
}
impl From<Eich3en> for u8 {
    #[inline(always)]
    fn from(val: Eich3en) -> u8 {
        Eich3en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich4en {
    #[doc = "Error injection is disabled on Error Injection Channel 4"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 4"]
    ENABLE = 0x01,
}
impl Eich4en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich4en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich4en {
    #[inline(always)]
    fn from(val: u8) -> Eich4en {
        Eich4en::from_bits(val)
    }
}
impl From<Eich4en> for u8 {
    #[inline(always)]
    fn from(val: Eich4en) -> u8 {
        Eich4en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich5en {
    #[doc = "Error injection is disabled on Error Injection Channel 5"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 5"]
    ENABLE = 0x01,
}
impl Eich5en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich5en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich5en {
    #[inline(always)]
    fn from(val: u8) -> Eich5en {
        Eich5en::from_bits(val)
    }
}
impl From<Eich5en> for u8 {
    #[inline(always)]
    fn from(val: Eich5en) -> u8 {
        Eich5en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich6en {
    #[doc = "Error injection is disabled on Error Injection Channel 6"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 6"]
    ENABLE = 0x01,
}
impl Eich6en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich6en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich6en {
    #[inline(always)]
    fn from(val: u8) -> Eich6en {
        Eich6en::from_bits(val)
    }
}
impl From<Eich6en> for u8 {
    #[inline(always)]
    fn from(val: Eich6en) -> u8 {
        Eich6en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich7en {
    #[doc = "Error injection is disabled on Error Injection Channel 7"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 7"]
    ENABLE = 0x01,
}
impl Eich7en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich7en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich7en {
    #[inline(always)]
    fn from(val: u8) -> Eich7en {
        Eich7en::from_bits(val)
    }
}
impl From<Eich7en> for u8 {
    #[inline(always)]
    fn from(val: Eich7en) -> u8 {
        Eich7en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eich8en {
    #[doc = "Error injection is disabled on Error Injection Channel 8"]
    DISABLE = 0x0,
    #[doc = "Error injection is enabled on Error Injection Channel 8"]
    ENABLE = 0x01,
}
impl Eich8en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eich8en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eich8en {
    #[inline(always)]
    fn from(val: u8) -> Eich8en {
        Eich8en::from_bits(val)
    }
}
impl From<Eich8en> for u8 {
    #[inline(always)]
    fn from(val: Eich8en) -> u8 {
        Eich8en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Geien {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Geien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Geien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Geien {
    #[inline(always)]
    fn from(val: u8) -> Geien {
        Geien::from_bits(val)
    }
}
impl From<Geien> for u8 {
    #[inline(always)]
    fn from(val: Geien) -> u8 {
        Geien::to_bits(val)
    }
}
