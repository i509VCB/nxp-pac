#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PHY_TEST_MODE {
    #[doc = "Test mode disabled."]
    DISABLE = 0x0,
    #[doc = "Test_J."]
    TEST_J = 0x01,
    #[doc = "Test_K."]
    TEST_K = 0x02,
    #[doc = "Test_SE0_NAK."]
    TEST_SE0_NAK = 0x03,
    #[doc = "Test_Packet."]
    TEST_PACKET = 0x04,
    #[doc = "Test_Force_Enable."]
    TEST_FORCE_ENABLE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PHY_TEST_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PHY_TEST_MODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PHY_TEST_MODE {
    #[inline(always)]
    fn from(val: u8) -> PHY_TEST_MODE {
        PHY_TEST_MODE::from_bits(val)
    }
}
impl From<PHY_TEST_MODE> for u8 {
    #[inline(always)]
    fn from(val: PHY_TEST_MODE) -> u8 {
        PHY_TEST_MODE::to_bits(val)
    }
}
