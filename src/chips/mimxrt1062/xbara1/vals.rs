#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Den0 {
    #[doc = "DMA disabled"]
    DEN0_0 = 0x0,
    #[doc = "DMA enabled"]
    DEN0_1 = 0x01,
}
impl Den0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Den0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Den0 {
    #[inline(always)]
    fn from(val: u8) -> Den0 {
        Den0::from_bits(val)
    }
}
impl From<Den0> for u8 {
    #[inline(always)]
    fn from(val: Den0) -> u8 {
        Den0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Den1 {
    #[doc = "DMA disabled"]
    DEN1_0 = 0x0,
    #[doc = "DMA enabled"]
    DEN1_1 = 0x01,
}
impl Den1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Den1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Den1 {
    #[inline(always)]
    fn from(val: u8) -> Den1 {
        Den1::from_bits(val)
    }
}
impl From<Den1> for u8 {
    #[inline(always)]
    fn from(val: Den1) -> u8 {
        Den1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Den2 {
    #[doc = "DMA disabled"]
    DEN2_0 = 0x0,
    #[doc = "DMA enabled"]
    DEN2_1 = 0x01,
}
impl Den2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Den2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Den2 {
    #[inline(always)]
    fn from(val: u8) -> Den2 {
        Den2::from_bits(val)
    }
}
impl From<Den2> for u8 {
    #[inline(always)]
    fn from(val: Den2) -> u8 {
        Den2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Den3 {
    #[doc = "DMA disabled"]
    DEN3_0 = 0x0,
    #[doc = "DMA enabled"]
    DEN3_1 = 0x01,
}
impl Den3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Den3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Den3 {
    #[inline(always)]
    fn from(val: u8) -> Den3 {
        Den3::from_bits(val)
    }
}
impl From<Den3> for u8 {
    #[inline(always)]
    fn from(val: Den3) -> u8 {
        Den3::to_bits(val)
    }
}
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ien0 {
    #[doc = "Interrupt disabled"]
    IEN0_0 = 0x0,
    #[doc = "Interrupt enabled"]
    IEN0_1 = 0x01,
}
impl Ien0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ien0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ien0 {
    #[inline(always)]
    fn from(val: u8) -> Ien0 {
        Ien0::from_bits(val)
    }
}
impl From<Ien0> for u8 {
    #[inline(always)]
    fn from(val: Ien0) -> u8 {
        Ien0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ien1 {
    #[doc = "Interrupt disabled"]
    IEN1_0 = 0x0,
    #[doc = "Interrupt enabled"]
    IEN1_1 = 0x01,
}
impl Ien1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ien1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ien1 {
    #[inline(always)]
    fn from(val: u8) -> Ien1 {
        Ien1::from_bits(val)
    }
}
impl From<Ien1> for u8 {
    #[inline(always)]
    fn from(val: Ien1) -> u8 {
        Ien1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ien2 {
    #[doc = "Interrupt disabled"]
    IEN2_0 = 0x0,
    #[doc = "Interrupt enabled"]
    IEN2_1 = 0x01,
}
impl Ien2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ien2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ien2 {
    #[inline(always)]
    fn from(val: u8) -> Ien2 {
        Ien2::from_bits(val)
    }
}
impl From<Ien2> for u8 {
    #[inline(always)]
    fn from(val: Ien2) -> u8 {
        Ien2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ien3 {
    #[doc = "Interrupt disabled"]
    IEN3_0 = 0x0,
    #[doc = "Interrupt enabled"]
    IEN3_1 = 0x01,
}
impl Ien3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ien3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ien3 {
    #[inline(always)]
    fn from(val: u8) -> Ien3 {
        Ien3::from_bits(val)
    }
}
impl From<Ien3> for u8 {
    #[inline(always)]
    fn from(val: Ien3) -> u8 {
        Ien3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sts0 {
    #[doc = "Active edge not yet detected on XBAR_OUT0"]
    STS0_0 = 0x0,
    #[doc = "Active edge detected on XBAR_OUT0"]
    STS0_1 = 0x01,
}
impl Sts0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts0 {
    #[inline(always)]
    fn from(val: u8) -> Sts0 {
        Sts0::from_bits(val)
    }
}
impl From<Sts0> for u8 {
    #[inline(always)]
    fn from(val: Sts0) -> u8 {
        Sts0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sts1 {
    #[doc = "Active edge not yet detected on XBAR_OUT1"]
    STS1_0 = 0x0,
    #[doc = "Active edge detected on XBAR_OUT1"]
    STS1_1 = 0x01,
}
impl Sts1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts1 {
    #[inline(always)]
    fn from(val: u8) -> Sts1 {
        Sts1::from_bits(val)
    }
}
impl From<Sts1> for u8 {
    #[inline(always)]
    fn from(val: Sts1) -> u8 {
        Sts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sts2 {
    #[doc = "Active edge not yet detected on XBAR_OUT2"]
    STS2_0 = 0x0,
    #[doc = "Active edge detected on XBAR_OUT2"]
    STS2_1 = 0x01,
}
impl Sts2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts2 {
    #[inline(always)]
    fn from(val: u8) -> Sts2 {
        Sts2::from_bits(val)
    }
}
impl From<Sts2> for u8 {
    #[inline(always)]
    fn from(val: Sts2) -> u8 {
        Sts2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sts3 {
    #[doc = "Active edge not yet detected on XBAR_OUT3"]
    STS3_0 = 0x0,
    #[doc = "Active edge detected on XBAR_OUT3"]
    STS3_1 = 0x01,
}
impl Sts3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts3 {
    #[inline(always)]
    fn from(val: u8) -> Sts3 {
        Sts3::from_bits(val)
    }
}
impl From<Sts3> for u8 {
    #[inline(always)]
    fn from(val: Sts3) -> u8 {
        Sts3::to_bits(val)
    }
}
