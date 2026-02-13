#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fxor {
    #[doc = "Disables XOR on reading data."]
    NOXOR = 0x0,
    #[doc = "Inverts or complements the read value of the CRC Data."]
    INVERT = 0x01,
}
impl Fxor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fxor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fxor {
    #[inline(always)]
    fn from(val: u8) -> Fxor {
        Fxor::from_bits(val)
    }
}
impl From<Fxor> for u8 {
    #[inline(always)]
    fn from(val: Fxor) -> u8 {
        Fxor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcrc {
    #[doc = "16 bits"]
    B16 = 0x0,
    #[doc = "32 bits"]
    B32 = 0x01,
}
impl Tcrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcrc {
    #[inline(always)]
    fn from(val: u8) -> Tcrc {
        Tcrc::from_bits(val)
    }
}
impl From<Tcrc> for u8 {
    #[inline(always)]
    fn from(val: Tcrc) -> u8 {
        Tcrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tot {
    #[doc = "No transposition"]
    NOTRNPS = 0x0,
    #[doc = "Bits in bytes are transposed, but bytes are not transposed."]
    BTS_TRNPS = 0x01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    BYTS_BTS_TRNPS = 0x02,
    #[doc = "Only bytes are transposed, no bits in a byte are transposed."]
    BYTS_TRNPS = 0x03,
}
impl Tot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tot {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tot {
    #[inline(always)]
    fn from(val: u8) -> Tot {
        Tot::from_bits(val)
    }
}
impl From<Tot> for u8 {
    #[inline(always)]
    fn from(val: Tot) -> u8 {
        Tot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Totr {
    #[doc = "No transposition"]
    NOTRNPS = 0x0,
    #[doc = "Bits in bytes are transposed, but bytes are not transposed."]
    BTS_TRNPS = 0x01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    BYTS_BTS_TRNPS = 0x02,
    #[doc = "Only bytes are transposed, no bits in a byte are transposed."]
    BYTS_TRNPS = 0x03,
}
impl Totr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Totr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Totr {
    #[inline(always)]
    fn from(val: u8) -> Totr {
        Totr::from_bits(val)
    }
}
impl From<Totr> for u8 {
    #[inline(always)]
    fn from(val: Totr) -> u8 {
        Totr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Was {
    #[doc = "Data values"]
    DATA = 0x0,
    #[doc = "Seed values"]
    SEED = 0x01,
}
impl Was {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Was {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Was {
    #[inline(always)]
    fn from(val: u8) -> Was {
        Was::from_bits(val)
    }
}
impl From<Was> for u8 {
    #[inline(always)]
    fn from(val: Was) -> u8 {
        Was::to_bits(val)
    }
}
