#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Ac {
        Bfcrt010Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Ac) -> u8 {
        Bfcrt010Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Bc {
        Bfcrt010Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Bc) -> u8 {
        Bfcrt010Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Cc {
        Bfcrt010Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Cc) -> u8 {
        Bfcrt010Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt0Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt0Dc {
        Bfcrt010Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt010Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt0Dc) -> u8 {
        Bfcrt010Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Ac {
        Bfcrt010Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Ac) -> u8 {
        Bfcrt010Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Bc {
        Bfcrt010Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Bc) -> u8 {
        Bfcrt010Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Cc {
        Bfcrt010Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Cc) -> u8 {
        Bfcrt010Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt010Pt1Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt010Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt010Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt010Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt010Pt1Dc {
        Bfcrt010Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt010Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt010Pt1Dc) -> u8 {
        Bfcrt010Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Ac {
        Bfcrt011Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Ac) -> u8 {
        Bfcrt011Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Bc {
        Bfcrt011Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Bc) -> u8 {
        Bfcrt011Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Cc {
        Bfcrt011Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Cc) -> u8 {
        Bfcrt011Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt0Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt0Dc {
        Bfcrt011Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt011Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt0Dc) -> u8 {
        Bfcrt011Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Ac {
        Bfcrt011Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Ac) -> u8 {
        Bfcrt011Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Bc {
        Bfcrt011Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Bc) -> u8 {
        Bfcrt011Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Cc {
        Bfcrt011Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Cc) -> u8 {
        Bfcrt011Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt011Pt1Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt011Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt011Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt011Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt011Pt1Dc {
        Bfcrt011Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt011Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt011Pt1Dc) -> u8 {
        Bfcrt011Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Ac {
        Bfcrt012Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Ac) -> u8 {
        Bfcrt012Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Bc {
        Bfcrt012Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Bc) -> u8 {
        Bfcrt012Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Cc {
        Bfcrt012Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Cc) -> u8 {
        Bfcrt012Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt0Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt0Dc {
        Bfcrt012Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt012Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt0Dc) -> u8 {
        Bfcrt012Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Ac {
        Bfcrt012Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Ac) -> u8 {
        Bfcrt012Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Bc {
        Bfcrt012Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Bc) -> u8 {
        Bfcrt012Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Cc {
        Bfcrt012Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Cc) -> u8 {
        Bfcrt012Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt012Pt1Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt012Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt012Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt012Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt012Pt1Dc {
        Bfcrt012Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt012Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt012Pt1Dc) -> u8 {
        Bfcrt012Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Ac {
        Bfcrt013Pt0Ac::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Ac) -> u8 {
        Bfcrt013Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Bc {
        Bfcrt013Pt0Bc::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Bc) -> u8 {
        Bfcrt013Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Cc {
        Bfcrt013Pt0Cc::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Cc) -> u8 {
        Bfcrt013Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt0Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt0Dc {
        Bfcrt013Pt0Dc::from_bits(val)
    }
}
impl From<Bfcrt013Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt0Dc) -> u8 {
        Bfcrt013Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Ac {
        Bfcrt013Pt1Ac::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Ac) -> u8 {
        Bfcrt013Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Bc {
        Bfcrt013Pt1Bc::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Bc) -> u8 {
        Bfcrt013Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Cc {
        Bfcrt013Pt1Cc::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Cc) -> u8 {
        Bfcrt013Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt013Pt1Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt013Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt013Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt013Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt013Pt1Dc {
        Bfcrt013Pt1Dc::from_bits(val)
    }
}
impl From<Bfcrt013Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt013Pt1Dc) -> u8 {
        Bfcrt013Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Ac {
        Bfcrt230Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Ac) -> u8 {
        Bfcrt230Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Bc {
        Bfcrt230Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Bc) -> u8 {
        Bfcrt230Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Cc {
        Bfcrt230Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Cc) -> u8 {
        Bfcrt230Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt2Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt2Dc {
        Bfcrt230Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt230Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt2Dc) -> u8 {
        Bfcrt230Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Ac {
        Bfcrt230Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Ac) -> u8 {
        Bfcrt230Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Bc {
        Bfcrt230Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Bc) -> u8 {
        Bfcrt230Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Cc {
        Bfcrt230Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Cc) -> u8 {
        Bfcrt230Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt230Pt3Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt230Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt230Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt230Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt230Pt3Dc {
        Bfcrt230Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt230Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt230Pt3Dc) -> u8 {
        Bfcrt230Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Ac {
        Bfcrt231Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Ac) -> u8 {
        Bfcrt231Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Bc {
        Bfcrt231Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Bc) -> u8 {
        Bfcrt231Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Cc {
        Bfcrt231Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Cc) -> u8 {
        Bfcrt231Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt2Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt2Dc {
        Bfcrt231Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt231Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt2Dc) -> u8 {
        Bfcrt231Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Ac {
        Bfcrt231Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Ac) -> u8 {
        Bfcrt231Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Bc {
        Bfcrt231Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Bc) -> u8 {
        Bfcrt231Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Cc {
        Bfcrt231Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Cc) -> u8 {
        Bfcrt231Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt231Pt3Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt231Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt231Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt231Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt231Pt3Dc {
        Bfcrt231Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt231Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt231Pt3Dc) -> u8 {
        Bfcrt231Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Ac {
        Bfcrt232Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Ac) -> u8 {
        Bfcrt232Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Bc {
        Bfcrt232Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Bc) -> u8 {
        Bfcrt232Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Cc {
        Bfcrt232Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Cc) -> u8 {
        Bfcrt232Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt2Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt2Dc {
        Bfcrt232Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt232Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt2Dc) -> u8 {
        Bfcrt232Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Ac {
        Bfcrt232Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Ac) -> u8 {
        Bfcrt232Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Bc {
        Bfcrt232Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Bc) -> u8 {
        Bfcrt232Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Cc {
        Bfcrt232Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Cc) -> u8 {
        Bfcrt232Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt232Pt3Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt232Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt232Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt232Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt232Pt3Dc {
        Bfcrt232Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt232Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt232Pt3Dc) -> u8 {
        Bfcrt232Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input A to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Ac {
        Bfcrt233Pt2Ac::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Ac) -> u8 {
        Bfcrt233Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Bc {
        Bfcrt233Pt2Bc::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Bc) -> u8 {
        Bfcrt233Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Cc {
        Bfcrt233Pt2Cc::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Cc) -> u8 {
        Bfcrt233Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt2Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt2Dc {
        Bfcrt233Pt2Dc::from_bits(val)
    }
}
impl From<Bfcrt233Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt2Dc) -> u8 {
        Bfcrt233Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Ac {
    #[doc = "Force input A to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input A"]
    PASS = 0x01,
    #[doc = "Complement input A"]
    COMPLEMENT = 0x02,
    #[doc = "Force input to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Ac {
        Bfcrt233Pt3Ac::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Ac) -> u8 {
        Bfcrt233Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Bc {
    #[doc = "Force input B to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input B"]
    PASS = 0x01,
    #[doc = "Complement input B"]
    COMPLEMENT = 0x02,
    #[doc = "Force input B to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Bc {
        Bfcrt233Pt3Bc::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Bc) -> u8 {
        Bfcrt233Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Cc {
    #[doc = "Force input C to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input C"]
    PASS = 0x01,
    #[doc = "Complement input C"]
    COMPLEMENT = 0x02,
    #[doc = "Force input C to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Cc {
        Bfcrt233Pt3Cc::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Cc) -> u8 {
        Bfcrt233Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfcrt233Pt3Dc {
    #[doc = "Force input D to become 0"]
    FORCE_0 = 0x0,
    #[doc = "Pass input D"]
    PASS = 0x01,
    #[doc = "Complement input D"]
    COMPLEMENT = 0x02,
    #[doc = "Force input D to become 1"]
    FORCE_1 = 0x03,
}
impl Bfcrt233Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfcrt233Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfcrt233Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> Bfcrt233Pt3Dc {
        Bfcrt233Pt3Dc::from_bits(val)
    }
}
impl From<Bfcrt233Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: Bfcrt233Pt3Dc) -> u8 {
        Bfcrt233Pt3Dc::to_bits(val)
    }
}
