#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Ac {
        EvtgAoi0Bft01Pt0Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Ac) -> u8 {
        EvtgAoi0Bft01Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Bc {
        EvtgAoi0Bft01Pt0Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Bc) -> u8 {
        EvtgAoi0Bft01Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    CC0_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    CC0_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    CC0_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    CC0_3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Cc {
        EvtgAoi0Bft01Pt0Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Cc) -> u8 {
        EvtgAoi0Bft01Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    DC_3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Dc {
        EvtgAoi0Bft01Pt0Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Dc) -> u8 {
        EvtgAoi0Bft01Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    AC_3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Ac {
        EvtgAoi0Bft01Pt1Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Ac) -> u8 {
        EvtgAoi0Bft01Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    BC_3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Bc {
        EvtgAoi0Bft01Pt1Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Bc) -> u8 {
        EvtgAoi0Bft01Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    CC_3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Cc {
        EvtgAoi0Bft01Pt1Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Cc) -> u8 {
        EvtgAoi0Bft01Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Dc {
        EvtgAoi0Bft01Pt1Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Dc) -> u8 {
        EvtgAoi0Bft01Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Ac {
        EvtgAoi0Bft23Pt2Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Ac) -> u8 {
        EvtgAoi0Bft23Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Bc {
        EvtgAoi0Bft23Pt2Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Bc) -> u8 {
        EvtgAoi0Bft23Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Cc {
        EvtgAoi0Bft23Pt2Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Cc) -> u8 {
        EvtgAoi0Bft23Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Dc {
        EvtgAoi0Bft23Pt2Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Dc) -> u8 {
        EvtgAoi0Bft23Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Ac {
        EvtgAoi0Bft23Pt3Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Ac) -> u8 {
        EvtgAoi0Bft23Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Bc {
        EvtgAoi0Bft23Pt3Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Bc) -> u8 {
        EvtgAoi0Bft23Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Cc {
        EvtgAoi0Bft23Pt3Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Cc) -> u8 {
        EvtgAoi0Bft23Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Dc {
        EvtgAoi0Bft23Pt3Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Dc) -> u8 {
        EvtgAoi0Bft23Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Ac {
        EvtgAoi1Bft01Pt0Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Ac) -> u8 {
        EvtgAoi1Bft01Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Bc {
        EvtgAoi1Bft01Pt0Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Bc) -> u8 {
        EvtgAoi1Bft01Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Cc {
        EvtgAoi1Bft01Pt0Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Cc) -> u8 {
        EvtgAoi1Bft01Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Dc {
        EvtgAoi1Bft01Pt0Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Dc) -> u8 {
        EvtgAoi1Bft01Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Ac {
        EvtgAoi1Bft01Pt1Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Ac) -> u8 {
        EvtgAoi1Bft01Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Bc {
        EvtgAoi1Bft01Pt1Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Bc) -> u8 {
        EvtgAoi1Bft01Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Cc {
        EvtgAoi1Bft01Pt1Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Cc) -> u8 {
        EvtgAoi1Bft01Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Dc {
        EvtgAoi1Bft01Pt1Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Dc) -> u8 {
        EvtgAoi1Bft01Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Ac {
        EvtgAoi1Bft23Pt2Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Ac) -> u8 {
        EvtgAoi1Bft23Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Bc {
        EvtgAoi1Bft23Pt2Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Bc) -> u8 {
        EvtgAoi1Bft23Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Cc {
        EvtgAoi1Bft23Pt2Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Cc) -> u8 {
        EvtgAoi1Bft23Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Dc {
        EvtgAoi1Bft23Pt2Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Dc) -> u8 {
        EvtgAoi1Bft23Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Ac {
        EvtgAoi1Bft23Pt3Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Ac) -> u8 {
        EvtgAoi1Bft23Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Bc {
        EvtgAoi1Bft23Pt3Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Bc) -> u8 {
        EvtgAoi1Bft23Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Cc {
        EvtgAoi1Bft23Pt3Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Cc) -> u8 {
        EvtgAoi1Bft23Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Dc {
        EvtgAoi1Bft23Pt3Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Dc) -> u8 {
        EvtgAoi1Bft23Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FbOvrd {
    #[doc = "Replace An"]
    AN = 0x0,
    #[doc = "Replace Bn"]
    BN = 0x01,
    #[doc = "Replace Cn"]
    CN = 0x02,
    #[doc = "Replace Dn"]
    DN = 0x03,
}
impl FbOvrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FbOvrd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FbOvrd {
    #[inline(always)]
    fn from(val: u8) -> FbOvrd {
        FbOvrd::from_bits(val)
    }
}
impl From<FbOvrd> for u8 {
    #[inline(always)]
    fn from(val: FbOvrd) -> u8 {
        FbOvrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FfInit {
    #[doc = "0"]
    FF0 = 0x0,
    #[doc = "1"]
    FF1 = 0x01,
}
impl FfInit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FfInit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FfInit {
    #[inline(always)]
    fn from(val: u8) -> FfInit {
        FfInit::from_bits(val)
    }
}
impl From<FfInit> for u8 {
    #[inline(always)]
    fn from(val: FfInit) -> u8 {
        FfInit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceBypass {
    #[doc = "Will not force the bypass"]
    NFB0 = 0x0,
    #[doc = "Whatever MODE_SEL is, will force bypass flip-flop and route the AOI_0(Filter_0) value directly to EVTG_OUTA"]
    F_FF_A = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ForceBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceBypass {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceBypass {
    #[inline(always)]
    fn from(val: u8) -> ForceBypass {
        ForceBypass::from_bits(val)
    }
}
impl From<ForceBypass> for u8 {
    #[inline(always)]
    fn from(val: ForceBypass) -> u8 {
        ForceBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitEn {
    #[doc = "Write 0 does not generate enable pulse"]
    PULSE = 0x0,
    #[doc = "Write 1 generates enable pulse"]
    NO_PULSE = 0x01,
}
impl InitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitEn {
    #[inline(always)]
    fn from(val: u8) -> InitEn {
        InitEn::from_bits(val)
    }
}
impl From<InitEn> for u8 {
    #[inline(always)]
    fn from(val: InitEn) -> u8 {
        InitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModeSel {
    #[doc = "Bypass mode"]
    BYPASS = 0x0,
    #[doc = "RS Trigger mode"]
    RS = 0x01,
    #[doc = "T-FF mode"]
    TFF = 0x02,
    #[doc = "D-FF mode"]
    DFF = 0x03,
    #[doc = "JK-FF mode"]
    JKFF = 0x04,
    #[doc = "Latch mode"]
    LATCH = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ModeSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModeSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModeSel {
    #[inline(always)]
    fn from(val: u8) -> ModeSel {
        ModeSel::from_bits(val)
    }
}
impl From<ModeSel> for u8 {
    #[inline(always)]
    fn from(val: ModeSel) -> u8 {
        ModeSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncCtrl {
    #[doc = "EVTG input \"An\" will not be synced"]
    A_NOTSYNC = 0x0,
    #[doc = "EVTG input \"An\" will be synced by two bus clk cycles"]
    A_SYNC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SyncCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncCtrl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncCtrl {
    #[inline(always)]
    fn from(val: u8) -> SyncCtrl {
        SyncCtrl::from_bits(val)
    }
}
impl From<SyncCtrl> for u8 {
    #[inline(always)]
    fn from(val: SyncCtrl) -> u8 {
        SyncCtrl::to_bits(val)
    }
}
