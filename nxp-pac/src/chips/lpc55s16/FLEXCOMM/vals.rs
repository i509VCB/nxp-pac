#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PERSEL {
    #[doc = "No peripheral selected."]
    NO_PERIPH_SELECTED = 0x0,
    #[doc = "USART function selected."]
    USART = 0x01,
    #[doc = "SPI function selected."]
    SPI = 0x02,
    #[doc = "I2C function selected."]
    I2C = 0x03,
    #[doc = "I2S transmit function selected."]
    I2S_TRANSMIT = 0x04,
    #[doc = "I2S receive function selected."]
    I2S_RECEIVE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PERSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PERSEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PERSEL {
    #[inline(always)]
    fn from(val: u8) -> PERSEL {
        PERSEL::from_bits(val)
    }
}
impl From<PERSEL> for u8 {
    #[inline(always)]
    fn from(val: PERSEL) -> u8 {
        PERSEL::to_bits(val)
    }
}
