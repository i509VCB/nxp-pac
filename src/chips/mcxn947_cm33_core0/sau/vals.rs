#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Allns {
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    SECURED_MEMORY = 0x0,
    #[doc = "Memory is marked as Non-secure."]
    NON_SECURED_MEMORY = 0x01,
}
impl Allns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Allns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Allns {
    #[inline(always)]
    fn from(val: u8) -> Allns {
        Allns::from_bits(val)
    }
}
impl From<Allns> for u8 {
    #[inline(always)]
    fn from(val: Allns) -> u8 {
        Allns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Auviol {
    #[doc = "Error has not occurred."]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Auviol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Auviol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Auviol {
    #[inline(always)]
    fn from(val: u8) -> Auviol {
        Auviol::from_bits(val)
    }
}
impl From<Auviol> for u8 {
    #[inline(always)]
    fn from(val: Auviol) -> u8 {
        Auviol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlEnable {
    #[doc = "The SAU is disabled."]
    DISABLED = 0x0,
    #[doc = "The SAU is enabled."]
    ENABLED = 0x01,
}
impl CtrlEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlEnable {
    #[inline(always)]
    fn from(val: u8) -> CtrlEnable {
        CtrlEnable::from_bits(val)
    }
}
impl From<CtrlEnable> for u8 {
    #[inline(always)]
    fn from(val: CtrlEnable) -> u8 {
        CtrlEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invep {
    #[doc = "Error has not occurred."]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Invep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invep {
    #[inline(always)]
    fn from(val: u8) -> Invep {
        Invep::from_bits(val)
    }
}
impl From<Invep> for u8 {
    #[inline(always)]
    fn from(val: Invep) -> u8 {
        Invep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inver {
    #[doc = "Error has not occurred."]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Inver {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inver {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inver {
    #[inline(always)]
    fn from(val: u8) -> Inver {
        Inver::from_bits(val)
    }
}
impl From<Inver> for u8 {
    #[inline(always)]
    fn from(val: Inver) -> u8 {
        Inver::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invis {
    #[doc = "Error has not occurred."]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Invis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invis {
    #[inline(always)]
    fn from(val: u8) -> Invis {
        Invis::from_bits(val)
    }
}
impl From<Invis> for u8 {
    #[inline(always)]
    fn from(val: Invis) -> u8 {
        Invis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invtran {
    #[doc = "Error has not occurred."]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Invtran {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invtran {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invtran {
    #[inline(always)]
    fn from(val: u8) -> Invtran {
        Invtran::from_bits(val)
    }
}
impl From<Invtran> for u8 {
    #[inline(always)]
    fn from(val: Invtran) -> u8 {
        Invtran::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lserr {
    #[doc = "Error has not occurred"]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Lserr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lserr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lserr {
    #[inline(always)]
    fn from(val: u8) -> Lserr {
        Lserr::from_bits(val)
    }
}
impl From<Lserr> for u8 {
    #[inline(always)]
    fn from(val: Lserr) -> u8 {
        Lserr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsperr {
    #[doc = "Error has not occurred."]
    NO_ERROR = 0x0,
    #[doc = "Error has occurred."]
    ERROR = 0x01,
}
impl Lsperr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsperr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsperr {
    #[inline(always)]
    fn from(val: u8) -> Lsperr {
        Lsperr::from_bits(val)
    }
}
impl From<Lsperr> for u8 {
    #[inline(always)]
    fn from(val: Lsperr) -> u8 {
        Lsperr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nsc {
    #[doc = "Region is not Non-secure callable."]
    NOT_NON_SECURE_CALLABLE = 0x0,
    #[doc = "Region is Non-secure callable."]
    NON_SECURE_CALLABLE = 0x01,
}
impl Nsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nsc {
    #[inline(always)]
    fn from(val: u8) -> Nsc {
        Nsc::from_bits(val)
    }
}
impl From<Nsc> for u8 {
    #[inline(always)]
    fn from(val: Nsc) -> u8 {
        Nsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarEnable {
    #[doc = "SAU region is enabled."]
    ENABLED = 0x0,
    #[doc = "SAU region is disabled."]
    DISABLED = 0x01,
}
impl RlarEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarEnable {
    #[inline(always)]
    fn from(val: u8) -> RlarEnable {
        RlarEnable::from_bits(val)
    }
}
impl From<RlarEnable> for u8 {
    #[inline(always)]
    fn from(val: RlarEnable) -> u8 {
        RlarEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sfarvalid {
    #[doc = "SFAR content not valid."]
    NOT_VALID = 0x0,
    #[doc = "SFAR content valid."]
    VALID = 0x01,
}
impl Sfarvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sfarvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sfarvalid {
    #[inline(always)]
    fn from(val: u8) -> Sfarvalid {
        Sfarvalid::from_bits(val)
    }
}
impl From<Sfarvalid> for u8 {
    #[inline(always)]
    fn from(val: Sfarvalid) -> u8 {
        Sfarvalid::to_bits(val)
    }
}
