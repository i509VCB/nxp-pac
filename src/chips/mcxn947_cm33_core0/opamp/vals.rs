#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Biasc {
    #[doc = "Default"]
    DEF = 0x0,
    #[doc = "Increase current"]
    INC = 0x01,
    #[doc = "Decrease current"]
    DEC = 0x02,
    #[doc = "Further decrease current"]
    FUR_DEC = 0x03,
}
impl Biasc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Biasc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Biasc {
    #[inline(always)]
    fn from(val: u8) -> Biasc {
        Biasc::from_bits(val)
    }
}
impl From<Biasc> for u8 {
    #[inline(always)]
    fn from(val: Biasc) -> u8 {
        Biasc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inpf {
    #[doc = "Positive input 0 (INP0)"]
    INP0 = 0x0,
    #[doc = "Positive input 1 (INP1)"]
    INP1 = 0x01,
}
impl Inpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inpf {
    #[inline(always)]
    fn from(val: u8) -> Inpf {
        Inpf::from_bits(val)
    }
}
impl From<Inpf> for u8 {
    #[inline(always)]
    fn from(val: Inpf) -> u8 {
        Inpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inpsel {
    #[doc = "When OPAMP is not in trigger mode, select positive input 0 (INP0)"]
    INP0 = 0x0,
    #[doc = "When OPAMP is not in trigger mode, select positive input 1 (INP1)"]
    INP1 = 0x01,
}
impl Inpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inpsel {
    #[inline(always)]
    fn from(val: u8) -> Inpsel {
        Inpsel::from_bits(val)
    }
}
impl From<Inpsel> for u8 {
    #[inline(always)]
    fn from(val: Inpsel) -> u8 {
        Inpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intref {
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA"]
    VDDA2 = 0x0,
    #[doc = "Select OPAMP input rail to rail voltage from 0 to VDD_ANA-0.8V"]
    VDDA3V = 0x01,
    #[doc = "Select OPAMP input rail to rail voltage from 0.8V to VDD_ANA"]
    VSSA3V = 0x02,
    #[doc = "Not allowed"]
    NOT = 0x03,
}
impl Intref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intref {
    #[inline(always)]
    fn from(val: u8) -> Intref {
        Intref::from_bits(val)
    }
}
impl From<Intref> for u8 {
    #[inline(always)]
    fn from(val: Intref) -> u8 {
        Intref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "High performance mode"]
    LOW = 0x0,
    #[doc = "Low power mode"]
    HIGH = 0x01,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub enum Ngain {
    #[doc = "Buffer"]
    BUFFER = 0x0,
    #[doc = "Ngain=1"]
    G1 = 0x01,
    #[doc = "Ngain=2"]
    G2 = 0x02,
    #[doc = "Ngain=4"]
    G4 = 0x03,
    #[doc = "Ngain=8"]
    G8 = 0x04,
    #[doc = "Ngain=16"]
    G16 = 0x05,
    #[doc = "Ngain=33"]
    G33 = 0x06,
    #[doc = "Ngain=64"]
    G64 = 0x07,
}
impl Ngain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ngain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ngain {
    #[inline(always)]
    fn from(val: u8) -> Ngain {
        Ngain::from_bits(val)
    }
}
impl From<Ngain> for u8 {
    #[inline(always)]
    fn from(val: Ngain) -> u8 {
        Ngain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PgaFunction {
    #[doc = "Core amplifier enabled"]
    CORE_AMP = 0x0,
    #[doc = "PGA function enabled"]
    PGA = 0x01,
}
impl PgaFunction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PgaFunction {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PgaFunction {
    #[inline(always)]
    fn from(val: u8) -> PgaFunction {
        PgaFunction::from_bits(val)
    }
}
impl From<PgaFunction> for u8 {
    #[inline(always)]
    fn from(val: PgaFunction) -> u8 {
        PgaFunction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pgain {
    #[doc = "Positive input 1 (INP1)"]
    INP1 = 0x0,
    #[doc = "Pgain=1"]
    G2 = 0x01,
    #[doc = "Pgain=2"]
    G3 = 0x02,
    #[doc = "Pgain=4"]
    G5 = 0x03,
    #[doc = "Pgain=8"]
    G9 = 0x04,
    #[doc = "Pgain=16"]
    G17 = 0x05,
    #[doc = "Pgain=33"]
    G34 = 0x06,
    #[doc = "Pgain=64"]
    G65 = 0x07,
}
impl Pgain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pgain {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pgain {
    #[inline(always)]
    fn from(val: u8) -> Pgain {
        Pgain::from_bits(val)
    }
}
impl From<Pgain> for u8 {
    #[inline(always)]
    fn from(val: Pgain) -> u8 {
        Pgain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pref {
    #[doc = "Input 0"]
    VAL0 = 0x0,
    #[doc = "Input 1"]
    VAL1 = 0x01,
    #[doc = "Input 2"]
    VAL2 = 0x02,
    #[doc = "Input 3"]
    VAL3 = 0x03,
}
impl Pref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pref {
    #[inline(always)]
    fn from(val: u8) -> Pref {
        Pref::from_bits(val)
    }
}
impl From<Pref> for u8 {
    #[inline(always)]
    fn from(val: Pref) -> u8 {
        Pref::to_bits(val)
    }
}
