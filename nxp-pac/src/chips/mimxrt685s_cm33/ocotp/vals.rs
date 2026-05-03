#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Div {
    #[doc = "Divide by 1."]
    Div1 = 0x0,
    #[doc = "Divide by 2."]
    Div2 = 0x01,
    #[doc = "Divide by 3."]
    Div3 = 0x02,
    #[doc = "Divide by 4."]
    Div4 = 0x03,
    #[doc = "Divide by 5."]
    Div5 = 0x04,
    #[doc = "Divide by 6."]
    Div6 = 0x05,
    #[doc = "Divide by 7."]
    Div7 = 0x06,
    #[doc = "Divide by 8."]
    Div8 = 0x07,
    #[doc = "Divide by 9."]
    Div9 = 0x08,
    #[doc = "Divide by 10."]
    Div10 = 0x09,
    #[doc = "Divide by 11."]
    Div11 = 0x0a,
    #[doc = "Divide by 12."]
    Div12 = 0x0b,
    #[doc = "Divide by 13."]
    Div13 = 0x0c,
    #[doc = "Divide by 14."]
    Div14 = 0x0d,
    #[doc = "Divide by 15."]
    Div15 = 0x0e,
    #[doc = "Divide by 16."]
    Div16 = 0x0f,
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
