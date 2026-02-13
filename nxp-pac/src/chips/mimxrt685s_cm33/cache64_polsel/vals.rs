#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg02Policy {
    #[doc = "Non-cache"]
    REG2_00 = 0x0,
    #[doc = "Write-thru"]
    REG2_01 = 0x01,
    #[doc = "Write-back"]
    REG2_10 = 0x02,
    #[doc = "Invalid"]
    REG2_11 = 0x03,
}
impl Reg02Policy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg02Policy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg02Policy {
    #[inline(always)]
    fn from(val: u8) -> Reg02Policy {
        Reg02Policy::from_bits(val)
    }
}
impl From<Reg02Policy> for u8 {
    #[inline(always)]
    fn from(val: Reg02Policy) -> u8 {
        Reg02Policy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg0Policy {
    #[doc = "Non-cache"]
    REG0_00 = 0x0,
    #[doc = "Write-thru"]
    REG0_01 = 0x01,
    #[doc = "Write-back"]
    REG0_10 = 0x02,
    #[doc = "Invalid"]
    REG0_11 = 0x03,
}
impl Reg0Policy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg0Policy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg0Policy {
    #[inline(always)]
    fn from(val: u8) -> Reg0Policy {
        Reg0Policy::from_bits(val)
    }
}
impl From<Reg0Policy> for u8 {
    #[inline(always)]
    fn from(val: Reg0Policy) -> u8 {
        Reg0Policy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1Policy {
    #[doc = "Non-cache"]
    REG1_00 = 0x0,
    #[doc = "Write-thru"]
    REG1_01 = 0x01,
    #[doc = "Write-back"]
    REG1_10 = 0x02,
    #[doc = "Invalid"]
    REG1_11 = 0x03,
}
impl Reg1Policy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1Policy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1Policy {
    #[inline(always)]
    fn from(val: u8) -> Reg1Policy {
        Reg1Policy::from_bits(val)
    }
}
impl From<Reg1Policy> for u8 {
    #[inline(always)]
    fn from(val: Reg1Policy) -> u8 {
        Reg1Policy::to_bits(val)
    }
}
