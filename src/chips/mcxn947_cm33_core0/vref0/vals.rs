#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf21en {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl Buf21en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf21en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf21en {
    #[inline(always)]
    fn from(val: u8) -> Buf21en {
        Buf21en::from_bits(val)
    }
}
impl From<Buf21en> for u8 {
    #[inline(always)]
    fn from(val: Buf21en) -> u8 {
        Buf21en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chopen {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl Chopen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chopen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chopen {
    #[inline(always)]
    fn from(val: u8) -> Chopen {
        Chopen::from_bits(val)
    }
}
impl From<Chopen> for u8 {
    #[inline(always)]
    fn from(val: Chopen) -> u8 {
        Chopen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hcbgen {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl Hcbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hcbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hcbgen {
    #[inline(always)]
    fn from(val: u8) -> Hcbgen {
        Hcbgen::from_bits(val)
    }
}
impl From<Hcbgen> for u8 {
    #[inline(always)]
    fn from(val: Hcbgen) -> u8 {
        Hcbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HiPwrLv {
    #[doc = "Low-power"]
    LOW = 0x0,
    #[doc = "High-power"]
    HIGH = 0x01,
}
impl HiPwrLv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HiPwrLv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HiPwrLv {
    #[inline(always)]
    fn from(val: u8) -> HiPwrLv {
        HiPwrLv::from_bits(val)
    }
}
impl From<HiPwrLv> for u8 {
    #[inline(always)]
    fn from(val: HiPwrLv) -> u8 {
        HiPwrLv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icompen {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl Icompen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icompen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icompen {
    #[inline(always)]
    fn from(val: u8) -> Icompen {
        Icompen::from_bits(val)
    }
}
impl From<Icompen> for u8 {
    #[inline(always)]
    fn from(val: Icompen) -> u8 {
        Icompen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpbgBufEn {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl LpbgBufEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpbgBufEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpbgBufEn {
    #[inline(always)]
    fn from(val: u8) -> LpbgBufEn {
        LpbgBufEn::from_bits(val)
    }
}
impl From<LpbgBufEn> for u8 {
    #[inline(always)]
    fn from(val: LpbgBufEn) -> u8 {
        LpbgBufEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpbgen {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl Lpbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpbgen {
    #[inline(always)]
    fn from(val: u8) -> Lpbgen {
        Lpbgen::from_bits(val)
    }
}
impl From<Lpbgen> for u8 {
    #[inline(always)]
    fn from(val: Lpbgen) -> u8 {
        Lpbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Regen {
    #[doc = "Disables"]
    DIS = 0x0,
    #[doc = "Enables"]
    ENA = 0x01,
}
impl Regen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Regen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Regen {
    #[inline(always)]
    fn from(val: u8) -> Regen {
        Regen::from_bits(val)
    }
}
impl From<Regen> for u8 {
    #[inline(always)]
    fn from(val: Regen) -> u8 {
        Regen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrefst {
    #[doc = "Disabled and unstable"]
    DIS_NOTSTABLE = 0x0,
    #[doc = "Stable"]
    STABLE = 0x01,
}
impl Vrefst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrefst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrefst {
    #[inline(always)]
    fn from(val: u8) -> Vrefst {
        Vrefst::from_bits(val)
    }
}
impl From<Vrefst> for u8 {
    #[inline(always)]
    fn from(val: Vrefst) -> u8 {
        Vrefst::to_bits(val)
    }
}
