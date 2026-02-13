#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg0Policy {
    #[doc = "Noncacheable"]
    REG0_00 = 0x0,
    #[doc = "Write-through"]
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
    #[doc = "Noncacheable"]
    REG1_00 = 0x0,
    #[doc = "Write-through"]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg2Policy {
    #[doc = "Noncacheable"]
    REG2_00 = 0x0,
    #[doc = "Write-through"]
    REG2_01 = 0x01,
    #[doc = "Write-back"]
    REG2_10 = 0x02,
    #[doc = "Invalid"]
    REG2_11 = 0x03,
}
impl Reg2Policy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg2Policy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg2Policy {
    #[inline(always)]
    fn from(val: u8) -> Reg2Policy {
        Reg2Policy::from_bits(val)
    }
}
impl From<Reg2Policy> for u8 {
    #[inline(always)]
    fn from(val: Reg2Policy) -> u8 {
        Reg2Policy::to_bits(val)
    }
}
