#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt0Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt0Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt0Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt0Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt0Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt0Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt0Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt0Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Cc00 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Cc01 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Cc02 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Cc03 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt1Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt1Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt1Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt1Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt2Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt2Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt2Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt2Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt2Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt2Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt2Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt2Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt2Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt2Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt2Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt2Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt2Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt2Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt2Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt2Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt3Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt3Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt3Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt3Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt3Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt3Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt3Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt3Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt3Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt3Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt3Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt3Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt3Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt3Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt3Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt3Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt0Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt0Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt0Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt0Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt0Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt0Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt0Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt0Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt0Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt0Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt0Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt0Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt0Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt0Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt0Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt0Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt1Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt1Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt1Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt1Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt1Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt1Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt1Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt1Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt1Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt1Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt1Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt1Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt1Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt1Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt1Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt1Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt2Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt2Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt2Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt2Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt2Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt2Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt2Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt2Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt2Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt2Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt2Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt2Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt2Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt2Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt2Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt2Dc3 = 0x03,
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
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt3Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt3Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt3Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt3Ac3 = 0x03,
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
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt3Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt3Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt3Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt3Bc3 = 0x03,
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
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt3Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt3Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt3Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt3Cc3 = 0x03,
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
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt3Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt3Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt3Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt3Dc3 = 0x03,
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
    #[doc = "Replace An."]
    An = 0x0,
    #[doc = "Replace Bn."]
    Bn = 0x01,
    #[doc = "Replace Cn."]
    Cn = 0x02,
    #[doc = "Replace Dn."]
    Dn = 0x03,
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
    #[doc = "0."]
    Ff0 = 0x0,
    #[doc = "1."]
    Ff1 = 0x01,
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
    #[doc = "Will not force the bypass."]
    Nfb0 = 0x0,
    #[doc = "Whatever MODE_SEL is, will force bypass flip-flop and route the AOI_0(Filter_0) value directly to EVTG_OUTA."]
    FFfA = 0x01,
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
    #[doc = "Write 0 does not generate enable pulse."]
    Pulse = 0x0,
    #[doc = "Write 1 generates enable pulse."]
    NoPulse = 0x01,
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
    #[doc = "Bypass mode."]
    Bypass = 0x0,
    #[doc = "RS Trigger mode."]
    Rs = 0x01,
    #[doc = "T-FF mode."]
    Tff = 0x02,
    #[doc = "D-FF mode."]
    Dff = 0x03,
    #[doc = "JK-FF mode."]
    Jkff = 0x04,
    #[doc = "Latch mode."]
    Latch = 0x05,
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
    #[doc = "EVTG input \"An\" will not be synced."]
    ANotsync = 0x0,
    #[doc = "EVTG input \"An\" will be synced by two bus clk cycles."]
    ASync = 0x01,
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
