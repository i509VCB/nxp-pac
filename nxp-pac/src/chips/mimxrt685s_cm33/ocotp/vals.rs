#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Div {
    #[doc = "Divide by 1"]
    DIV_1 = 0x0,
    #[doc = "Divide by 2"]
    DIV_2 = 0x01,
    #[doc = "Divide by 3"]
    DIV_3 = 0x02,
    #[doc = "Divide by 4"]
    DIV_4 = 0x03,
    #[doc = "Divide by 5"]
    DIV_5 = 0x04,
    #[doc = "Divide by 6"]
    DIV_6 = 0x05,
    #[doc = "Divide by 7"]
    DIV_7 = 0x06,
    #[doc = "Divide by 8"]
    DIV_8 = 0x07,
    #[doc = "Divide by 9"]
    DIV_9 = 0x08,
    #[doc = "Divide by 10"]
    DIV_10 = 0x09,
    #[doc = "Divide by 11"]
    DIV_11 = 0x0a,
    #[doc = "Divide by 12"]
    DIV_12 = 0x0b,
    #[doc = "Divide by 13"]
    DIV_13 = 0x0c,
    #[doc = "Divide by 14"]
    DIV_14 = 0x0d,
    #[doc = "Divide by 15"]
    DIV_15 = 0x0e,
    #[doc = "Divide by 16"]
    DIV_16 = 0x0f,
}
impl Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Div {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Div {
    #[inline(always)]
    fn from(val: u8) -> Div {
        Div::from_bits(val)
    }
}
impl From<Div> for u8 {
    #[inline(always)]
    fn from(val: Div) -> u8 {
        Div::to_bits(val)
    }
}
