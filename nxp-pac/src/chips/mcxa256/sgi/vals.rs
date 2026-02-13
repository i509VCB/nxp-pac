#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cmd(u8);
impl Cmd {
    #[doc = "8'h00 - ECB mode"]
    pub const ECB: Self = Self(0x0);
    #[doc = "8'h01 - CTR mode"]
    pub const CTR: Self = Self(0x01);
    #[doc = "8'h02 - CBC mode"]
    pub const CBC: Self = Self(0x02);
    #[doc = "8'h03 - CBCMAC mode"]
    pub const CBCMAC: Self = Self(0x03);
    #[doc = "8'h10 - Key Wrap/Unwrap(128 bit key data)"]
    pub const KW128: Self = Self(0x10);
    #[doc = "8'h11 - Key Wrap/Unwrap(256 bit key data)"]
    pub const KW256: Self = Self(0x11);
}
impl Cmd {
    pub const fn from_bits(val: u8) -> Cmd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ECB"),
            0x01 => f.write_str("CTR"),
            0x02 => f.write_str("CBC"),
            0x03 => f.write_str("CBCMAC"),
            0x10 => f.write_str("KW128"),
            0x11 => f.write_str("KW256"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECB"),
            0x01 => defmt::write!(f, "CTR"),
            0x02 => defmt::write!(f, "CBC"),
            0x03 => defmt::write!(f, "CBCMAC"),
            0x10 => defmt::write!(f, "KW128"),
            0x11 => defmt::write!(f, "KW256"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[doc = "ERROR(all values other than 0x05 indicate ERROR)"]
    ERROR = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "NO_ERROR"]
    NO_ERROR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Error {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Error {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Error {
    #[inline(always)]
    fn from(val: u8) -> Error {
        Error::from_bits(val)
    }
}
impl From<Error> for u8 {
    #[inline(always)]
    fn from(val: Error) -> u8 {
        Error::to_bits(val)
    }
}
