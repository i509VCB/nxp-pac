#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ade {
    #[doc = "Bypass the fetched data."]
    Bypass = 0x0,
    #[doc = "Perform the CTR-AES128 mode decryption on the fetched data."]
    Decrypt = 0x01,
}
impl Ade {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ade {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ade {
    #[inline(always)]
    fn from(val: u8) -> Ade {
        Ade::from_bits(val)
    }
}
impl From<Ade> for u8 {
    #[inline(always)]
    fn from(val: Ade) -> u8 {
        Ade::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ferr {
    #[doc = "No effect on the SR\\[KBERE\\] indicator."]
    NoEffect = 0x0,
    #[doc = "SR\\[KBERR\\] is immediately set after a write with this data bit set."]
    ForceError = 0x01,
}
impl Ferr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ferr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ferr {
    #[inline(always)]
    fn from(val: u8) -> Ferr {
        Ferr::from_bits(val)
    }
}
impl From<Ferr> for u8 {
    #[inline(always)]
    fn from(val: Ferr) -> u8 {
        Ferr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fldm {
    #[doc = "No effect on the operating mode."]
    NoEffect = 0x0,
    #[doc = "Force entry into LDM after a write with this data bit set. SR\\[MODE\\] signals the operating mode."]
    ForceLdm = 0x01,
}
impl Fldm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fldm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fldm {
    #[inline(always)]
    fn from(val: u8) -> Fldm {
        Fldm::from_bits(val)
    }
}
impl From<Fldm> for u8 {
    #[inline(always)]
    fn from(val: Fldm) -> u8 {
        Fldm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsvm {
    #[doc = "No effect on the operating mode."]
    NoEffect = 0x0,
    #[doc = "Force entry into SVM after a write with this data bit set and the data bit associated with FLDM cleared. SR\\[MODE\\] signals the operating mode."]
    ForceSvm = 0x01,
}
impl Fsvm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsvm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsvm {
    #[inline(always)]
    fn from(val: u8) -> Fsvm {
        Fsvm::from_bits(val)
    }
}
impl From<Fsvm> for u8 {
    #[inline(always)]
    fn from(val: Fsvm) -> u8 {
        Fsvm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Operating in Normal mode (NRM)."]
    Normal = 0x0,
    #[doc = "Unused (reserved)."]
    Res01 = 0x01,
    #[doc = "Operating in Security Violation Mode (SVM)."]
    Res10Svm = 0x02,
    #[doc = "Operating in Logically Disabled Mode (LDM)."]
    Ldm = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ro {
    #[doc = "The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
    Normal = 0x0,
    #[doc = "The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    Restrict = 0x01,
}
impl Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ro {
    #[inline(always)]
    fn from(val: u8) -> Ro {
        Ro::from_bits(val)
    }
}
impl From<Ro> for u8 {
    #[inline(always)]
    fn from(val: Ro) -> u8 {
        Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrae {
    #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    Normal = 0x0,
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    Restrict = 0x01,
}
impl Rrae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrae {
    #[inline(always)]
    fn from(val: u8) -> Rrae {
        Rrae::from_bits(val)
    }
}
impl From<Rrae> for u8 {
    #[inline(always)]
    fn from(val: Rrae) -> u8 {
        Rrae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rram {
    #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
    Normal = 0x0,
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    Restricted = 0x01,
}
impl Rram {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rram {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rram {
    #[inline(always)]
    fn from(val: u8) -> Rram {
        Rram::from_bits(val)
    }
}
impl From<Rram> for u8 {
    #[inline(always)]
    fn from(val: Rram) -> u8 {
        Rram::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skbp {
    #[doc = "Key blob processing is not initiated."]
    NoEffect = 0x0,
    #[doc = "Properly-enabled key blob processing is initiated."]
    InitKb = 0x01,
}
impl Skbp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skbp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skbp {
    #[inline(always)]
    fn from(val: u8) -> Skbp {
        Skbp::from_bits(val)
    }
}
impl From<Skbp> for u8 {
    #[inline(always)]
    fn from(val: Skbp) -> u8 {
        Skbp::to_bits(val)
    }
}
