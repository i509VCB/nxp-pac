#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyTestMode {
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
impl PhyTestMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyTestMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyTestMode {
    #[inline(always)]
    fn from(val: u8) -> PhyTestMode {
        PhyTestMode::from_bits(val)
    }
}
impl From<PhyTestMode> for u8 {
    #[inline(always)]
    fn from(val: PhyTestMode) -> u8 {
        PhyTestMode::to_bits(val)
    }
}
