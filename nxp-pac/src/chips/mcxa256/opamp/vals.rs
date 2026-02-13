#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpaBcSel {
    #[doc = "Default value. Keep power consumption constant"]
    TBD1 = 0x0,
    #[doc = "Reduce power consumption to 1/4"]
    TBD2 = 0x01,
    #[doc = "Reduce power consumption to 1/2"]
    TBD3 = 0x02,
    #[doc = "Double the power consumption"]
    TBD4 = 0x03,
}
impl OpaBcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpaBcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpaBcSel {
    #[inline(always)]
    fn from(val: u8) -> OpaBcSel {
        OpaBcSel::from_bits(val)
    }
}
impl From<OpaBcSel> for u8 {
    #[inline(always)]
    fn from(val: OpaBcSel) -> u8 {
        OpaBcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpaCcSel {
    #[doc = "Fit 2X gains"]
    TBD1 = 0x0,
    #[doc = "Fit 4X gains"]
    TBD2 = 0x01,
    #[doc = "Fit 8X gains"]
    TBD3 = 0x02,
    #[doc = "Fit 16X gains"]
    TBD4 = 0x03,
}
impl OpaCcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpaCcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpaCcSel {
    #[inline(always)]
    fn from(val: u8) -> OpaCcSel {
        OpaCcSel::from_bits(val)
    }
}
impl From<OpaCcSel> for u8 {
    #[inline(always)]
    fn from(val: OpaCcSel) -> u8 {
        OpaCcSel::to_bits(val)
    }
}
