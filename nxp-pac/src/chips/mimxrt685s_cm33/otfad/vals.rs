#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ade {
    #[doc = "Bypass the fetched data."]
    Ade0 = 0x0,
    #[doc = "Perform the CTR-AES128 mode decryption on the fetched data."]
    Ade1 = 0x01,
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
pub enum Fldm {
    #[doc = "No effect on the operating mode."]
    Fldm0 = 0x0,
    #[doc = "Force entry into LDM after a write with this data bit set. SR\\[MODE\\] signals the operating mode."]
    Fldm1 = 0x01,
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
pub enum Ge {
    #[doc = "OTFAD has decryption disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    Ge0 = 0x0,
    #[doc = "OTFAD has decryption enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    Ge1 = 0x01,
}
impl Ge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ge {
    #[inline(always)]
    fn from(val: u8) -> Ge {
        Ge::from_bits(val)
    }
}
impl From<Ge> for u8 {
    #[inline(always)]
    fn from(val: Ge) -> u8 {
        Ge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gem {
    #[doc = "OTFAD is disabled. All data fetched by the FLEXSPI bypasses OTFAD processing."]
    Gem0 = 0x0,
    #[doc = "OTFAD is enabled, and processes data fetched by the FLEXSPI as defined by the hardware configuration."]
    Gem1 = 0x01,
}
impl Gem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gem {
    #[inline(always)]
    fn from(val: u8) -> Gem {
        Gem::from_bits(val)
    }
}
impl From<Gem> for u8 {
    #[inline(always)]
    fn from(val: Gem) -> u8 {
        Gem::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Operating in Normal mode (NRM)."]
    Mode0 = 0x0,
    #[doc = "Unused (reserved)."]
    Mode1 = 0x01,
    #[doc = "Unused (reserved)."]
    Mode2 = 0x02,
    #[doc = "Operating in Logically Disabled Mode (LDM)."]
    Mode3 = 0x03,
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
    Ro0 = 0x0,
    #[doc = "The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
    Ro1 = 0x01,
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
    Rrae0 = 0x0,
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    Rrae1 = 0x01,
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
    Rram0 = 0x0,
    #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
    Rram1 = 0x01,
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
pub enum Vld {
    #[doc = "Context is invalid."]
    Vld0 = 0x0,
    #[doc = "Context is valid."]
    Vld1 = 0x01,
}
impl Vld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vld {
    #[inline(always)]
    fn from(val: u8) -> Vld {
        Vld::from_bits(val)
    }
}
impl From<Vld> for u8 {
    #[inline(always)]
    fn from(val: Vld) -> u8 {
        Vld::to_bits(val)
    }
}
