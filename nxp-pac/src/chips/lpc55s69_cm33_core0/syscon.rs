#[doc = "ADC clock source select"]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcclkselSel {
    #[doc = "Main clk."]
    mainclk = 0x0,
    #[doc = "PLL0 clk."]
    pll0 = 0x01,
    #[doc = "FRO 96 MHZ clk."]
    fro96 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "No clk."]
    none = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl AdcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> AdcclkselSel {
        AdcclkselSel::from_bits(val)
    }
}
impl From<AdcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: AdcclkselSel) -> u8 {
        AdcclkselSel::to_bits(val)
    }
}
