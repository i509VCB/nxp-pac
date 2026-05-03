#[doc = "Key destination for PUF key."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(u8);
impl Key {
    #[doc = "Do not send key to any hardware engine."]
    pub const none: Self = Self(0x55);
    #[doc = "Send key to AES engine."]
    pub const aes: Self = Self(0x56);
    #[doc = "Send key to PRINCE engine for memory layout 0."]
    pub const prince0: Self = Self(0x59);
    #[doc = "Send key to PRINCE engine for memory layout 1."]
    pub const prince1: Self = Self(0x65);
    #[doc = "Send key to PRINCE engine for memory layout 2."]
    pub const prince2: Self = Self(0x95);
}
impl Key {
    pub const fn from_bits(val: u8) -> Key {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x55 => f.write_str("none"),
            0x56 => f.write_str("aes"),
            0x59 => f.write_str("prince0"),
            0x65 => f.write_str("prince1"),
            0x95 => f.write_str("prince2"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Key {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x55 => defmt::write!(f, "none"),
            0x56 => defmt::write!(f, "aes"),
            0x59 => defmt::write!(f, "prince0"),
            0x65 => defmt::write!(f, "prince1"),
            0x95 => defmt::write!(f, "prince2"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Key {
    #[inline(always)]
    fn from(val: u8) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(val: Key) -> u8 {
        Key::to_bits(val)
    }
}
