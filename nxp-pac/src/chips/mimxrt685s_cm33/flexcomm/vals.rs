#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Persel {
    #[doc = "No peripheral selected."]
    NoPeriphSelected = 0x0,
    #[doc = "USART function selected."]
    Usart = 0x01,
    #[doc = "SPI function selected."]
    Spi = 0x02,
    #[doc = "I2C function selected."]
    I2c = 0x03,
    #[doc = "I2S transmit function selected."]
    I2sTransmit = 0x04,
    #[doc = "I2S receive function selected."]
    I2sReceive = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Persel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Persel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Persel {
    #[inline(always)]
    fn from(val: u8) -> Persel {
        Persel::from_bits(val)
    }
}
impl From<Persel> for u8 {
    #[inline(always)]
    fn from(val: Persel) -> u8 {
        Persel::to_bits(val)
    }
}
