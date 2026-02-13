#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt0Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3 = 0x03,
}
impl Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Pt0Ac {
        Pt0Ac::from_bits(val)
    }
}
impl From<Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Pt0Ac) -> u8 {
        Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt0Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3 = 0x03,
}
impl Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Pt0Bc {
        Pt0Bc::from_bits(val)
    }
}
impl From<Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Pt0Bc) -> u8 {
        Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt0Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3 = 0x03,
}
impl Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Pt0Cc {
        Pt0Cc::from_bits(val)
    }
}
impl From<Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Pt0Cc) -> u8 {
        Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt0Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3 = 0x03,
}
impl Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Pt0Dc {
        Pt0Dc::from_bits(val)
    }
}
impl From<Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Pt0Dc) -> u8 {
        Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt1Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3 = 0x03,
}
impl Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Pt1Ac {
        Pt1Ac::from_bits(val)
    }
}
impl From<Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Pt1Ac) -> u8 {
        Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt1Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3 = 0x03,
}
impl Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Pt1Bc {
        Pt1Bc::from_bits(val)
    }
}
impl From<Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Pt1Bc) -> u8 {
        Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt1Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3 = 0x03,
}
impl Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Pt1Cc {
        Pt1Cc::from_bits(val)
    }
}
impl From<Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Pt1Cc) -> u8 {
        Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt1Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3 = 0x03,
}
impl Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Pt1Dc {
        Pt1Dc::from_bits(val)
    }
}
impl From<Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Pt1Dc) -> u8 {
        Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt2Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3 = 0x03,
}
impl Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Pt2Ac {
        Pt2Ac::from_bits(val)
    }
}
impl From<Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Pt2Ac) -> u8 {
        Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt2Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3 = 0x03,
}
impl Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Pt2Bc {
        Pt2Bc::from_bits(val)
    }
}
impl From<Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Pt2Bc) -> u8 {
        Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt2Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3 = 0x03,
}
impl Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Pt2Cc {
        Pt2Cc::from_bits(val)
    }
}
impl From<Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Pt2Cc) -> u8 {
        Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt2Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3 = 0x03,
}
impl Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Pt2Dc {
        Pt2Dc::from_bits(val)
    }
}
impl From<Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Pt2Dc) -> u8 {
        Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt3Ac {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0x0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1 = 0x01,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3 = 0x03,
}
impl Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Pt3Ac {
        Pt3Ac::from_bits(val)
    }
}
impl From<Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Pt3Ac) -> u8 {
        Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt3Bc {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0x0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1 = 0x01,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3 = 0x03,
}
impl Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Pt3Bc {
        Pt3Bc::from_bits(val)
    }
}
impl From<Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Pt3Bc) -> u8 {
        Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt3Cc {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0x0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1 = 0x01,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3 = 0x03,
}
impl Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Pt3Cc {
        Pt3Cc::from_bits(val)
    }
}
impl From<Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Pt3Cc) -> u8 {
        Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt3Dc {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0x0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1 = 0x01,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3 = 0x03,
}
impl Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Pt3Dc {
        Pt3Dc::from_bits(val)
    }
}
impl From<Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Pt3Dc) -> u8 {
        Pt3Dc::to_bits(val)
    }
}
