#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edge0 {
    #[doc = "STS0 never asserts"]
    EDGE0_0 = 0x0,
    #[doc = "STS0 asserts on rising edges of XBAR_OUT0"]
    EDGE0_1 = 0x01,
    #[doc = "STS0 asserts on falling edges of XBAR_OUT0"]
    EDGE0_2 = 0x02,
    #[doc = "STS0 asserts on rising and falling edges of XBAR_OUT0"]
    EDGE0_3 = 0x03,
}
impl Edge0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edge0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edge0 {
    #[inline(always)]
    fn from(val: u8) -> Edge0 {
        Edge0::from_bits(val)
    }
}
impl From<Edge0> for u8 {
    #[inline(always)]
    fn from(val: Edge0) -> u8 {
        Edge0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edge1 {
    #[doc = "STS1 never asserts"]
    EDGE1_0 = 0x0,
    #[doc = "STS1 asserts on rising edges of XBAR_OUT1"]
    EDGE1_1 = 0x01,
    #[doc = "STS1 asserts on falling edges of XBAR_OUT1"]
    EDGE1_2 = 0x02,
    #[doc = "STS1 asserts on rising and falling edges of XBAR_OUT1"]
    EDGE1_3 = 0x03,
}
impl Edge1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edge1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edge1 {
    #[inline(always)]
    fn from(val: u8) -> Edge1 {
        Edge1::from_bits(val)
    }
}
impl From<Edge1> for u8 {
    #[inline(always)]
    fn from(val: Edge1) -> u8 {
        Edge1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edge2 {
    #[doc = "STS2 never asserts"]
    EDGE2_0 = 0x0,
    #[doc = "STS2 asserts on rising edges of XBAR_OUT2"]
    EDGE2_1 = 0x01,
    #[doc = "STS2 asserts on falling edges of XBAR_OUT2"]
    EDGE2_2 = 0x02,
    #[doc = "STS2 asserts on rising and falling edges of XBAR_OUT2"]
    EDGE2_3 = 0x03,
}
impl Edge2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edge2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edge2 {
    #[inline(always)]
    fn from(val: u8) -> Edge2 {
        Edge2::from_bits(val)
    }
}
impl From<Edge2> for u8 {
    #[inline(always)]
    fn from(val: Edge2) -> u8 {
        Edge2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edge3 {
    #[doc = "STS3 never asserts"]
    EDGE3_0 = 0x0,
    #[doc = "STS3 asserts on rising edges of XBAR_OUT3"]
    EDGE3_1 = 0x01,
    #[doc = "STS3 asserts on falling edges of XBAR_OUT3"]
    EDGE3_2 = 0x02,
    #[doc = "STS3 asserts on rising and falling edges of XBAR_OUT3"]
    EDGE3_3 = 0x03,
}
impl Edge3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edge3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edge3 {
    #[inline(always)]
    fn from(val: u8) -> Edge3 {
        Edge3::from_bits(val)
    }
}
impl From<Edge3> for u8 {
    #[inline(always)]
    fn from(val: Edge3) -> u8 {
        Edge3::to_bits(val)
    }
}
