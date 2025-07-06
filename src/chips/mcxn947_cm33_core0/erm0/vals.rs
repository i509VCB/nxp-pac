#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie0 {
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie0 {
    #[inline(always)]
    fn from(val: u8) -> Encie0 {
        Encie0::from_bits(val)
    }
}
impl From<Encie0> for u8 {
    #[inline(always)]
    fn from(val: Encie0) -> u8 {
        Encie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie1 {
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie1 {
    #[inline(always)]
    fn from(val: u8) -> Encie1 {
        Encie1::from_bits(val)
    }
}
impl From<Encie1> for u8 {
    #[inline(always)]
    fn from(val: Encie1) -> u8 {
        Encie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie2 {
    #[doc = "Interrupt notification of Memory 2 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 2 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie2 {
    #[inline(always)]
    fn from(val: u8) -> Encie2 {
        Encie2::from_bits(val)
    }
}
impl From<Encie2> for u8 {
    #[inline(always)]
    fn from(val: Encie2) -> u8 {
        Encie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie3 {
    #[doc = "Interrupt notification of Memory 3 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 3 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie3 {
    #[inline(always)]
    fn from(val: u8) -> Encie3 {
        Encie3::from_bits(val)
    }
}
impl From<Encie3> for u8 {
    #[inline(always)]
    fn from(val: Encie3) -> u8 {
        Encie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie4 {
    #[doc = "Interrupt notification of Memory 4 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 4 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie4 {
    #[inline(always)]
    fn from(val: u8) -> Encie4 {
        Encie4::from_bits(val)
    }
}
impl From<Encie4> for u8 {
    #[inline(always)]
    fn from(val: Encie4) -> u8 {
        Encie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie5 {
    #[doc = "Interrupt notification of Memory 5 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 5 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie5 {
    #[inline(always)]
    fn from(val: u8) -> Encie5 {
        Encie5::from_bits(val)
    }
}
impl From<Encie5> for u8 {
    #[inline(always)]
    fn from(val: Encie5) -> u8 {
        Encie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie6 {
    #[doc = "Interrupt notification of Memory 6 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 6 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie6 {
    #[inline(always)]
    fn from(val: u8) -> Encie6 {
        Encie6::from_bits(val)
    }
}
impl From<Encie6> for u8 {
    #[inline(always)]
    fn from(val: Encie6) -> u8 {
        Encie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie7 {
    #[doc = "Interrupt notification of Memory 7 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 7 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie7 {
    #[inline(always)]
    fn from(val: u8) -> Encie7 {
        Encie7::from_bits(val)
    }
}
impl From<Encie7> for u8 {
    #[inline(always)]
    fn from(val: Encie7) -> u8 {
        Encie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie8 {
    #[doc = "Interrupt notification of Memory 8 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 8 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie8 {
    #[inline(always)]
    fn from(val: u8) -> Encie8 {
        Encie8::from_bits(val)
    }
}
impl From<Encie8> for u8 {
    #[inline(always)]
    fn from(val: Encie8) -> u8 {
        Encie8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Encie9 {
    #[doc = "Interrupt notification of Memory 9 non-correctable error events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 9 non-correctable error events is enabled."]
    ENABLE = 0x01,
}
impl Encie9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Encie9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Encie9 {
    #[inline(always)]
    fn from(val: u8) -> Encie9 {
        Encie9::from_bits(val)
    }
}
impl From<Encie9> for u8 {
    #[inline(always)]
    fn from(val: Encie9) -> u8 {
        Encie9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie0 {
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie0 {
    #[inline(always)]
    fn from(val: u8) -> Escie0 {
        Escie0::from_bits(val)
    }
}
impl From<Escie0> for u8 {
    #[inline(always)]
    fn from(val: Escie0) -> u8 {
        Escie0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie1 {
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie1 {
    #[inline(always)]
    fn from(val: u8) -> Escie1 {
        Escie1::from_bits(val)
    }
}
impl From<Escie1> for u8 {
    #[inline(always)]
    fn from(val: Escie1) -> u8 {
        Escie1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie2 {
    #[doc = "Interrupt notification of Memory 2 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 2 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie2 {
    #[inline(always)]
    fn from(val: u8) -> Escie2 {
        Escie2::from_bits(val)
    }
}
impl From<Escie2> for u8 {
    #[inline(always)]
    fn from(val: Escie2) -> u8 {
        Escie2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie3 {
    #[doc = "Interrupt notification of Memory 3 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 3 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie3 {
    #[inline(always)]
    fn from(val: u8) -> Escie3 {
        Escie3::from_bits(val)
    }
}
impl From<Escie3> for u8 {
    #[inline(always)]
    fn from(val: Escie3) -> u8 {
        Escie3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie4 {
    #[doc = "Interrupt notification of Memory 4 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 4 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie4 {
    #[inline(always)]
    fn from(val: u8) -> Escie4 {
        Escie4::from_bits(val)
    }
}
impl From<Escie4> for u8 {
    #[inline(always)]
    fn from(val: Escie4) -> u8 {
        Escie4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie5 {
    #[doc = "Interrupt notification of Memory 5 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 5 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie5 {
    #[inline(always)]
    fn from(val: u8) -> Escie5 {
        Escie5::from_bits(val)
    }
}
impl From<Escie5> for u8 {
    #[inline(always)]
    fn from(val: Escie5) -> u8 {
        Escie5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie6 {
    #[doc = "Interrupt notification of Memory 6 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 6 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie6 {
    #[inline(always)]
    fn from(val: u8) -> Escie6 {
        Escie6::from_bits(val)
    }
}
impl From<Escie6> for u8 {
    #[inline(always)]
    fn from(val: Escie6) -> u8 {
        Escie6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie7 {
    #[doc = "Interrupt notification of Memory 7 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 7 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie7 {
    #[inline(always)]
    fn from(val: u8) -> Escie7 {
        Escie7::from_bits(val)
    }
}
impl From<Escie7> for u8 {
    #[inline(always)]
    fn from(val: Escie7) -> u8 {
        Escie7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie8 {
    #[doc = "Interrupt notification of Memory 8 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 8 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie8 {
    #[inline(always)]
    fn from(val: u8) -> Escie8 {
        Escie8::from_bits(val)
    }
}
impl From<Escie8> for u8 {
    #[inline(always)]
    fn from(val: Escie8) -> u8 {
        Escie8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Escie9 {
    #[doc = "Interrupt notification of Memory 9 single-bit correction events is disabled."]
    DISABLE = 0x0,
    #[doc = "Interrupt notification of Memory 9 single-bit correction events is enabled."]
    ENABLE = 0x01,
}
impl Escie9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Escie9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Escie9 {
    #[inline(always)]
    fn from(val: u8) -> Escie9 {
        Escie9::from_bits(val)
    }
}
impl From<Escie9> for u8 {
    #[inline(always)]
    fn from(val: Escie9) -> u8 {
        Escie9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce0 {
    #[doc = "No non-correctable error event on Memory 0 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 0 detected."]
    ERROR = 0x01,
}
impl Nce0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce0 {
    #[inline(always)]
    fn from(val: u8) -> Nce0 {
        Nce0::from_bits(val)
    }
}
impl From<Nce0> for u8 {
    #[inline(always)]
    fn from(val: Nce0) -> u8 {
        Nce0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce1 {
    #[doc = "No non-correctable error event on Memory 1 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 1 detected."]
    ERROR = 0x01,
}
impl Nce1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce1 {
    #[inline(always)]
    fn from(val: u8) -> Nce1 {
        Nce1::from_bits(val)
    }
}
impl From<Nce1> for u8 {
    #[inline(always)]
    fn from(val: Nce1) -> u8 {
        Nce1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce2 {
    #[doc = "No non-correctable error event on Memory 2 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 2 detected."]
    ERROR = 0x01,
}
impl Nce2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce2 {
    #[inline(always)]
    fn from(val: u8) -> Nce2 {
        Nce2::from_bits(val)
    }
}
impl From<Nce2> for u8 {
    #[inline(always)]
    fn from(val: Nce2) -> u8 {
        Nce2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce3 {
    #[doc = "No non-correctable error event on Memory 3 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 3 detected."]
    ERROR = 0x01,
}
impl Nce3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce3 {
    #[inline(always)]
    fn from(val: u8) -> Nce3 {
        Nce3::from_bits(val)
    }
}
impl From<Nce3> for u8 {
    #[inline(always)]
    fn from(val: Nce3) -> u8 {
        Nce3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce4 {
    #[doc = "No non-correctable error event on Memory 4 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 4 detected."]
    ERROR = 0x01,
}
impl Nce4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce4 {
    #[inline(always)]
    fn from(val: u8) -> Nce4 {
        Nce4::from_bits(val)
    }
}
impl From<Nce4> for u8 {
    #[inline(always)]
    fn from(val: Nce4) -> u8 {
        Nce4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce5 {
    #[doc = "No non-correctable error event on Memory 5 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 5 detected."]
    ERROR = 0x01,
}
impl Nce5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce5 {
    #[inline(always)]
    fn from(val: u8) -> Nce5 {
        Nce5::from_bits(val)
    }
}
impl From<Nce5> for u8 {
    #[inline(always)]
    fn from(val: Nce5) -> u8 {
        Nce5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce6 {
    #[doc = "No non-correctable error event on Memory 6 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 6 detected."]
    ERROR = 0x01,
}
impl Nce6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce6 {
    #[inline(always)]
    fn from(val: u8) -> Nce6 {
        Nce6::from_bits(val)
    }
}
impl From<Nce6> for u8 {
    #[inline(always)]
    fn from(val: Nce6) -> u8 {
        Nce6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce7 {
    #[doc = "No non-correctable error event on Memory 7 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 7 detected."]
    ERROR = 0x01,
}
impl Nce7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce7 {
    #[inline(always)]
    fn from(val: u8) -> Nce7 {
        Nce7::from_bits(val)
    }
}
impl From<Nce7> for u8 {
    #[inline(always)]
    fn from(val: Nce7) -> u8 {
        Nce7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce8 {
    #[doc = "No non-correctable error event on Memory 8 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 8 detected."]
    ERROR = 0x01,
}
impl Nce8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce8 {
    #[inline(always)]
    fn from(val: u8) -> Nce8 {
        Nce8::from_bits(val)
    }
}
impl From<Nce8> for u8 {
    #[inline(always)]
    fn from(val: Nce8) -> u8 {
        Nce8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce9 {
    #[doc = "No non-correctable error event on Memory 9 detected."]
    NO_ERROR = 0x0,
    #[doc = "Non-correctable error event on Memory 9 detected."]
    ERROR = 0x01,
}
impl Nce9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce9 {
    #[inline(always)]
    fn from(val: u8) -> Nce9 {
        Nce9::from_bits(val)
    }
}
impl From<Nce9> for u8 {
    #[inline(always)]
    fn from(val: Nce9) -> u8 {
        Nce9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc0 {
    #[doc = "No single-bit correction event on Memory 0 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 0 detected."]
    EVENT = 0x01,
}
impl Sbc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc0 {
    #[inline(always)]
    fn from(val: u8) -> Sbc0 {
        Sbc0::from_bits(val)
    }
}
impl From<Sbc0> for u8 {
    #[inline(always)]
    fn from(val: Sbc0) -> u8 {
        Sbc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc1 {
    #[doc = "No single-bit correction event on Memory 1 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 1 detected."]
    EVENT = 0x01,
}
impl Sbc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc1 {
    #[inline(always)]
    fn from(val: u8) -> Sbc1 {
        Sbc1::from_bits(val)
    }
}
impl From<Sbc1> for u8 {
    #[inline(always)]
    fn from(val: Sbc1) -> u8 {
        Sbc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc2 {
    #[doc = "No single-bit correction event on Memory 2 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 2 detected."]
    EVENT = 0x01,
}
impl Sbc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc2 {
    #[inline(always)]
    fn from(val: u8) -> Sbc2 {
        Sbc2::from_bits(val)
    }
}
impl From<Sbc2> for u8 {
    #[inline(always)]
    fn from(val: Sbc2) -> u8 {
        Sbc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc3 {
    #[doc = "No single-bit correction event on Memory 3 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 3 detected."]
    EVENT = 0x01,
}
impl Sbc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc3 {
    #[inline(always)]
    fn from(val: u8) -> Sbc3 {
        Sbc3::from_bits(val)
    }
}
impl From<Sbc3> for u8 {
    #[inline(always)]
    fn from(val: Sbc3) -> u8 {
        Sbc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc4 {
    #[doc = "No single-bit correction event on Memory 4 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 4 detected."]
    EVENT = 0x01,
}
impl Sbc4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc4 {
    #[inline(always)]
    fn from(val: u8) -> Sbc4 {
        Sbc4::from_bits(val)
    }
}
impl From<Sbc4> for u8 {
    #[inline(always)]
    fn from(val: Sbc4) -> u8 {
        Sbc4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc5 {
    #[doc = "No single-bit correction event on Memory 5 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 5 detected."]
    CORR_EVENT = 0x01,
}
impl Sbc5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc5 {
    #[inline(always)]
    fn from(val: u8) -> Sbc5 {
        Sbc5::from_bits(val)
    }
}
impl From<Sbc5> for u8 {
    #[inline(always)]
    fn from(val: Sbc5) -> u8 {
        Sbc5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc6 {
    #[doc = "No single-bit correction event on Memory 6 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 6 detected."]
    EVENT = 0x01,
}
impl Sbc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc6 {
    #[inline(always)]
    fn from(val: u8) -> Sbc6 {
        Sbc6::from_bits(val)
    }
}
impl From<Sbc6> for u8 {
    #[inline(always)]
    fn from(val: Sbc6) -> u8 {
        Sbc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc7 {
    #[doc = "No single-bit correction event on Memory 7 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 7 detected."]
    EVENT = 0x01,
}
impl Sbc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc7 {
    #[inline(always)]
    fn from(val: u8) -> Sbc7 {
        Sbc7::from_bits(val)
    }
}
impl From<Sbc7> for u8 {
    #[inline(always)]
    fn from(val: Sbc7) -> u8 {
        Sbc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc8 {
    #[doc = "No single-bit correction event on Memory 8 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 8 detected."]
    EVENT = 0x01,
}
impl Sbc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc8 {
    #[inline(always)]
    fn from(val: u8) -> Sbc8 {
        Sbc8::from_bits(val)
    }
}
impl From<Sbc8> for u8 {
    #[inline(always)]
    fn from(val: Sbc8) -> u8 {
        Sbc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbc9 {
    #[doc = "No single-bit correction event on Memory 9 detected."]
    NO_EVENT = 0x0,
    #[doc = "Single-bit correction event on Memory 9 detected."]
    EVENT = 0x01,
}
impl Sbc9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbc9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbc9 {
    #[inline(always)]
    fn from(val: u8) -> Sbc9 {
        Sbc9::from_bits(val)
    }
}
impl From<Sbc9> for u8 {
    #[inline(always)]
    fn from(val: Sbc9) -> u8 {
        Sbc9::to_bits(val)
    }
}
