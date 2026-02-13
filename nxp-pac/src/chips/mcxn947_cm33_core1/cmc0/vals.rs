#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BlrLock {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "BootROM Status and Lock Registers can be written"]
    LOCK010 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "BootROM Status and Lock Registers cannot be written"]
    LOCK101 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl BlrLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BlrLock {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BlrLock {
    #[inline(always)]
    fn from(val: u8) -> BlrLock {
        BlrLock::from_bits(val)
    }
}
impl From<BlrLock> for u8 {
    #[inline(always)]
    fn from(val: BlrLock) -> u8 {
        BlrLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CkctrlCkmode {
    #[doc = "No clock gating"]
    CKMODE0000 = 0x0,
    #[doc = "Core clock is gated"]
    CKMODE0001 = 0x01,
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
    #[doc = "Core, platform, and peripheral clocks are gated, and core enters Low-Power mode."]
    CKMODE1111 = 0x0f,
}
impl CkctrlCkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkctrlCkmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkctrlCkmode {
    #[inline(always)]
    fn from(val: u8) -> CkctrlCkmode {
        CkctrlCkmode::from_bits(val)
    }
}
impl From<CkctrlCkmode> for u8 {
    #[inline(always)]
    fn from(val: CkctrlCkmode) -> u8 {
        CkctrlCkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CkstatCkmode {
    #[doc = "Core clock not gated"]
    CKMODE0000 = 0x0,
    #[doc = "Core clock was gated"]
    CKMODE0001 = 0x01,
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
    #[doc = "Core, platform, and peripheral clocks were gated, and power domain entered Low-Power mode"]
    CKMODE1111 = 0x0f,
}
impl CkstatCkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkstatCkmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkstatCkmode {
    #[inline(always)]
    fn from(val: u8) -> CkstatCkmode {
        CkstatCkmode::from_bits(val)
    }
}
impl From<CkstatCkmode> for u8 {
    #[inline(always)]
    fn from(val: CkstatCkmode) -> u8 {
        CkstatCkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis0 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis0 {
    #[inline(always)]
    fn from(val: u8) -> Dis0 {
        Dis0::from_bits(val)
    }
}
impl From<Dis0> for u8 {
    #[inline(always)]
    fn from(val: Dis0) -> u8 {
        Dis0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis1 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis1 {
    #[inline(always)]
    fn from(val: u8) -> Dis1 {
        Dis1::from_bits(val)
    }
}
impl From<Dis1> for u8 {
    #[inline(always)]
    fn from(val: Dis1) -> u8 {
        Dis1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis10 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis10 {
    #[inline(always)]
    fn from(val: u8) -> Dis10 {
        Dis10::from_bits(val)
    }
}
impl From<Dis10> for u8 {
    #[inline(always)]
    fn from(val: Dis10) -> u8 {
        Dis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis11 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis11 {
    #[inline(always)]
    fn from(val: u8) -> Dis11 {
        Dis11::from_bits(val)
    }
}
impl From<Dis11> for u8 {
    #[inline(always)]
    fn from(val: Dis11) -> u8 {
        Dis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis12 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis12 {
    #[inline(always)]
    fn from(val: u8) -> Dis12 {
        Dis12::from_bits(val)
    }
}
impl From<Dis12> for u8 {
    #[inline(always)]
    fn from(val: Dis12) -> u8 {
        Dis12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis13 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis13 {
    #[inline(always)]
    fn from(val: u8) -> Dis13 {
        Dis13::from_bits(val)
    }
}
impl From<Dis13> for u8 {
    #[inline(always)]
    fn from(val: Dis13) -> u8 {
        Dis13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis14 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis14 {
    #[inline(always)]
    fn from(val: u8) -> Dis14 {
        Dis14::from_bits(val)
    }
}
impl From<Dis14> for u8 {
    #[inline(always)]
    fn from(val: Dis14) -> u8 {
        Dis14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis15 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis15 {
    #[inline(always)]
    fn from(val: u8) -> Dis15 {
        Dis15::from_bits(val)
    }
}
impl From<Dis15> for u8 {
    #[inline(always)]
    fn from(val: Dis15) -> u8 {
        Dis15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis16 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis16 {
    #[inline(always)]
    fn from(val: u8) -> Dis16 {
        Dis16::from_bits(val)
    }
}
impl From<Dis16> for u8 {
    #[inline(always)]
    fn from(val: Dis16) -> u8 {
        Dis16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis17 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis17 {
    #[inline(always)]
    fn from(val: u8) -> Dis17 {
        Dis17::from_bits(val)
    }
}
impl From<Dis17> for u8 {
    #[inline(always)]
    fn from(val: Dis17) -> u8 {
        Dis17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis18 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis18 {
    #[inline(always)]
    fn from(val: u8) -> Dis18 {
        Dis18::from_bits(val)
    }
}
impl From<Dis18> for u8 {
    #[inline(always)]
    fn from(val: Dis18) -> u8 {
        Dis18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis19 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis19 {
    #[inline(always)]
    fn from(val: u8) -> Dis19 {
        Dis19::from_bits(val)
    }
}
impl From<Dis19> for u8 {
    #[inline(always)]
    fn from(val: Dis19) -> u8 {
        Dis19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis2 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis2 {
    #[inline(always)]
    fn from(val: u8) -> Dis2 {
        Dis2::from_bits(val)
    }
}
impl From<Dis2> for u8 {
    #[inline(always)]
    fn from(val: Dis2) -> u8 {
        Dis2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis20 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis20 {
    #[inline(always)]
    fn from(val: u8) -> Dis20 {
        Dis20::from_bits(val)
    }
}
impl From<Dis20> for u8 {
    #[inline(always)]
    fn from(val: Dis20) -> u8 {
        Dis20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis21 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis21 {
    #[inline(always)]
    fn from(val: u8) -> Dis21 {
        Dis21::from_bits(val)
    }
}
impl From<Dis21> for u8 {
    #[inline(always)]
    fn from(val: Dis21) -> u8 {
        Dis21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis22 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis22 {
    #[inline(always)]
    fn from(val: u8) -> Dis22 {
        Dis22::from_bits(val)
    }
}
impl From<Dis22> for u8 {
    #[inline(always)]
    fn from(val: Dis22) -> u8 {
        Dis22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis23 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis23 {
    #[inline(always)]
    fn from(val: u8) -> Dis23 {
        Dis23::from_bits(val)
    }
}
impl From<Dis23> for u8 {
    #[inline(always)]
    fn from(val: Dis23) -> u8 {
        Dis23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis24 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis24 {
    #[inline(always)]
    fn from(val: u8) -> Dis24 {
        Dis24::from_bits(val)
    }
}
impl From<Dis24> for u8 {
    #[inline(always)]
    fn from(val: Dis24) -> u8 {
        Dis24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis25 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis25 {
    #[inline(always)]
    fn from(val: u8) -> Dis25 {
        Dis25::from_bits(val)
    }
}
impl From<Dis25> for u8 {
    #[inline(always)]
    fn from(val: Dis25) -> u8 {
        Dis25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis26 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis26 {
    #[inline(always)]
    fn from(val: u8) -> Dis26 {
        Dis26::from_bits(val)
    }
}
impl From<Dis26> for u8 {
    #[inline(always)]
    fn from(val: Dis26) -> u8 {
        Dis26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis27 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis27 {
    #[inline(always)]
    fn from(val: u8) -> Dis27 {
        Dis27::from_bits(val)
    }
}
impl From<Dis27> for u8 {
    #[inline(always)]
    fn from(val: Dis27) -> u8 {
        Dis27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis28 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis28 {
    #[inline(always)]
    fn from(val: u8) -> Dis28 {
        Dis28::from_bits(val)
    }
}
impl From<Dis28> for u8 {
    #[inline(always)]
    fn from(val: Dis28) -> u8 {
        Dis28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis29 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis29 {
    #[inline(always)]
    fn from(val: u8) -> Dis29 {
        Dis29::from_bits(val)
    }
}
impl From<Dis29> for u8 {
    #[inline(always)]
    fn from(val: Dis29) -> u8 {
        Dis29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis3 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis3 {
    #[inline(always)]
    fn from(val: u8) -> Dis3 {
        Dis3::from_bits(val)
    }
}
impl From<Dis3> for u8 {
    #[inline(always)]
    fn from(val: Dis3) -> u8 {
        Dis3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis30 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis30 {
    #[inline(always)]
    fn from(val: u8) -> Dis30 {
        Dis30::from_bits(val)
    }
}
impl From<Dis30> for u8 {
    #[inline(always)]
    fn from(val: Dis30) -> u8 {
        Dis30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis31 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis31 {
    #[inline(always)]
    fn from(val: u8) -> Dis31 {
        Dis31::from_bits(val)
    }
}
impl From<Dis31> for u8 {
    #[inline(always)]
    fn from(val: Dis31) -> u8 {
        Dis31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis4 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis4 {
    #[inline(always)]
    fn from(val: u8) -> Dis4 {
        Dis4::from_bits(val)
    }
}
impl From<Dis4> for u8 {
    #[inline(always)]
    fn from(val: Dis4) -> u8 {
        Dis4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis5 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis5 {
    #[inline(always)]
    fn from(val: u8) -> Dis5 {
        Dis5::from_bits(val)
    }
}
impl From<Dis5> for u8 {
    #[inline(always)]
    fn from(val: Dis5) -> u8 {
        Dis5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis6 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis6 {
    #[inline(always)]
    fn from(val: u8) -> Dis6 {
        Dis6::from_bits(val)
    }
}
impl From<Dis6> for u8 {
    #[inline(always)]
    fn from(val: Dis6) -> u8 {
        Dis6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis7 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis7 {
    #[inline(always)]
    fn from(val: u8) -> Dis7 {
        Dis7::from_bits(val)
    }
}
impl From<Dis7> for u8 {
    #[inline(always)]
    fn from(val: Dis7) -> u8 {
        Dis7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis8 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis8 {
    #[inline(always)]
    fn from(val: u8) -> Dis8 {
        Dis8::from_bits(val)
    }
}
impl From<Dis8> for u8 {
    #[inline(always)]
    fn from(val: Dis8) -> u8 {
        Dis8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dis9 {
    #[doc = "Enables"]
    ENABLED = 0x0,
    #[doc = "Disables"]
    DISABLED = 0x01,
}
impl Dis9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dis9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dis9 {
    #[inline(always)]
    fn from(val: u8) -> Dis9 {
        Dis9::from_bits(val)
    }
}
impl From<Dis9> for u8 {
    #[inline(always)]
    fn from(val: Dis9) -> u8 {
        Dis9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmctrlmainLpmode {
    #[doc = "Active/Sleep"]
    LPMODE0000 = 0x0,
    #[doc = "Deep Sleep"]
    LPMODE0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Power Down"]
    LPMODE0011 = 0x03,
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
    #[doc = "Deep-Power Down"]
    LPMODE1111 = 0x0f,
}
impl PmctrlmainLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmctrlmainLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmctrlmainLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmctrlmainLpmode {
        PmctrlmainLpmode::from_bits(val)
    }
}
impl From<PmctrlmainLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmctrlmainLpmode) -> u8 {
        PmctrlmainLpmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmctrlwakeLpmode {
    #[doc = "Active/Sleep"]
    LPMODE0000 = 0x0,
    #[doc = "Deep Sleep"]
    LPMODE0001 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Power Down"]
    LPMODE0011 = 0x03,
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
    #[doc = "Deep-Power Down"]
    LPMODE1111 = 0x0f,
}
impl PmctrlwakeLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmctrlwakeLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmctrlwakeLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmctrlwakeLpmode {
        PmctrlwakeLpmode::from_bits(val)
    }
}
impl From<PmctrlwakeLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmctrlwakeLpmode) -> u8 {
        PmctrlwakeLpmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmprotLpmode {
    #[doc = "Not allowed"]
    DISABLED = 0x0,
    #[doc = "Allowed"]
    EN = 0x01,
    #[doc = "Allowed"]
    EN1 = 0x02,
    #[doc = "Allowed"]
    EN2 = 0x03,
    #[doc = "Allowed"]
    EN3 = 0x04,
    #[doc = "Allowed"]
    EN4 = 0x05,
    #[doc = "Allowed"]
    EN5 = 0x06,
    #[doc = "Allowed"]
    EN6 = 0x07,
    #[doc = "Allowed"]
    EN7 = 0x08,
    #[doc = "Allowed"]
    EN8 = 0x09,
    #[doc = "Allowed"]
    EN9 = 0x0a,
    #[doc = "Allowed"]
    EN10 = 0x0b,
    #[doc = "Allowed"]
    EN11 = 0x0c,
    #[doc = "Allowed"]
    EN12 = 0x0d,
    #[doc = "Allowed"]
    EN13 = 0x0e,
    #[doc = "Allowed"]
    EN14 = 0x0f,
}
impl PmprotLpmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmprotLpmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmprotLpmode {
    #[inline(always)]
    fn from(val: u8) -> PmprotLpmode {
        PmprotLpmode::from_bits(val)
    }
}
impl From<PmprotLpmode> for u8 {
    #[inline(always)]
    fn from(val: PmprotLpmode) -> u8 {
        PmprotLpmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret0 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret0 {
    #[inline(always)]
    fn from(val: u8) -> Ret0 {
        Ret0::from_bits(val)
    }
}
impl From<Ret0> for u8 {
    #[inline(always)]
    fn from(val: Ret0) -> u8 {
        Ret0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret1 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret1 {
    #[inline(always)]
    fn from(val: u8) -> Ret1 {
        Ret1::from_bits(val)
    }
}
impl From<Ret1> for u8 {
    #[inline(always)]
    fn from(val: Ret1) -> u8 {
        Ret1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret10 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret10 {
    #[inline(always)]
    fn from(val: u8) -> Ret10 {
        Ret10::from_bits(val)
    }
}
impl From<Ret10> for u8 {
    #[inline(always)]
    fn from(val: Ret10) -> u8 {
        Ret10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret11 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret11 {
    #[inline(always)]
    fn from(val: u8) -> Ret11 {
        Ret11::from_bits(val)
    }
}
impl From<Ret11> for u8 {
    #[inline(always)]
    fn from(val: Ret11) -> u8 {
        Ret11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret12 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret12 {
    #[inline(always)]
    fn from(val: u8) -> Ret12 {
        Ret12::from_bits(val)
    }
}
impl From<Ret12> for u8 {
    #[inline(always)]
    fn from(val: Ret12) -> u8 {
        Ret12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret13 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret13 {
    #[inline(always)]
    fn from(val: u8) -> Ret13 {
        Ret13::from_bits(val)
    }
}
impl From<Ret13> for u8 {
    #[inline(always)]
    fn from(val: Ret13) -> u8 {
        Ret13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret14 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret14 {
    #[inline(always)]
    fn from(val: u8) -> Ret14 {
        Ret14::from_bits(val)
    }
}
impl From<Ret14> for u8 {
    #[inline(always)]
    fn from(val: Ret14) -> u8 {
        Ret14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret15 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret15 {
    #[inline(always)]
    fn from(val: u8) -> Ret15 {
        Ret15::from_bits(val)
    }
}
impl From<Ret15> for u8 {
    #[inline(always)]
    fn from(val: Ret15) -> u8 {
        Ret15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret16 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret16 {
    #[inline(always)]
    fn from(val: u8) -> Ret16 {
        Ret16::from_bits(val)
    }
}
impl From<Ret16> for u8 {
    #[inline(always)]
    fn from(val: Ret16) -> u8 {
        Ret16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret17 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret17 {
    #[inline(always)]
    fn from(val: u8) -> Ret17 {
        Ret17::from_bits(val)
    }
}
impl From<Ret17> for u8 {
    #[inline(always)]
    fn from(val: Ret17) -> u8 {
        Ret17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret18 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret18 {
    #[inline(always)]
    fn from(val: u8) -> Ret18 {
        Ret18::from_bits(val)
    }
}
impl From<Ret18> for u8 {
    #[inline(always)]
    fn from(val: Ret18) -> u8 {
        Ret18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret19 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret19 {
    #[inline(always)]
    fn from(val: u8) -> Ret19 {
        Ret19::from_bits(val)
    }
}
impl From<Ret19> for u8 {
    #[inline(always)]
    fn from(val: Ret19) -> u8 {
        Ret19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret2 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret2 {
    #[inline(always)]
    fn from(val: u8) -> Ret2 {
        Ret2::from_bits(val)
    }
}
impl From<Ret2> for u8 {
    #[inline(always)]
    fn from(val: Ret2) -> u8 {
        Ret2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret20 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret20 {
    #[inline(always)]
    fn from(val: u8) -> Ret20 {
        Ret20::from_bits(val)
    }
}
impl From<Ret20> for u8 {
    #[inline(always)]
    fn from(val: Ret20) -> u8 {
        Ret20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret21 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret21 {
    #[inline(always)]
    fn from(val: u8) -> Ret21 {
        Ret21::from_bits(val)
    }
}
impl From<Ret21> for u8 {
    #[inline(always)]
    fn from(val: Ret21) -> u8 {
        Ret21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret22 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret22 {
    #[inline(always)]
    fn from(val: u8) -> Ret22 {
        Ret22::from_bits(val)
    }
}
impl From<Ret22> for u8 {
    #[inline(always)]
    fn from(val: Ret22) -> u8 {
        Ret22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret23 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret23 {
    #[inline(always)]
    fn from(val: u8) -> Ret23 {
        Ret23::from_bits(val)
    }
}
impl From<Ret23> for u8 {
    #[inline(always)]
    fn from(val: Ret23) -> u8 {
        Ret23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret24 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret24 {
    #[inline(always)]
    fn from(val: u8) -> Ret24 {
        Ret24::from_bits(val)
    }
}
impl From<Ret24> for u8 {
    #[inline(always)]
    fn from(val: Ret24) -> u8 {
        Ret24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret25 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret25 {
    #[inline(always)]
    fn from(val: u8) -> Ret25 {
        Ret25::from_bits(val)
    }
}
impl From<Ret25> for u8 {
    #[inline(always)]
    fn from(val: Ret25) -> u8 {
        Ret25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret26 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret26 {
    #[inline(always)]
    fn from(val: u8) -> Ret26 {
        Ret26::from_bits(val)
    }
}
impl From<Ret26> for u8 {
    #[inline(always)]
    fn from(val: Ret26) -> u8 {
        Ret26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret27 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret27 {
    #[inline(always)]
    fn from(val: u8) -> Ret27 {
        Ret27::from_bits(val)
    }
}
impl From<Ret27> for u8 {
    #[inline(always)]
    fn from(val: Ret27) -> u8 {
        Ret27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret28 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret28 {
    #[inline(always)]
    fn from(val: u8) -> Ret28 {
        Ret28::from_bits(val)
    }
}
impl From<Ret28> for u8 {
    #[inline(always)]
    fn from(val: Ret28) -> u8 {
        Ret28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret29 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret29 {
    #[inline(always)]
    fn from(val: u8) -> Ret29 {
        Ret29::from_bits(val)
    }
}
impl From<Ret29> for u8 {
    #[inline(always)]
    fn from(val: Ret29) -> u8 {
        Ret29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret3 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret3 {
    #[inline(always)]
    fn from(val: u8) -> Ret3 {
        Ret3::from_bits(val)
    }
}
impl From<Ret3> for u8 {
    #[inline(always)]
    fn from(val: Ret3) -> u8 {
        Ret3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret30 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret30 {
    #[inline(always)]
    fn from(val: u8) -> Ret30 {
        Ret30::from_bits(val)
    }
}
impl From<Ret30> for u8 {
    #[inline(always)]
    fn from(val: Ret30) -> u8 {
        Ret30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret31 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret31 {
    #[inline(always)]
    fn from(val: u8) -> Ret31 {
        Ret31::from_bits(val)
    }
}
impl From<Ret31> for u8 {
    #[inline(always)]
    fn from(val: Ret31) -> u8 {
        Ret31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret4 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret4 {
    #[inline(always)]
    fn from(val: u8) -> Ret4 {
        Ret4::from_bits(val)
    }
}
impl From<Ret4> for u8 {
    #[inline(always)]
    fn from(val: Ret4) -> u8 {
        Ret4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret5 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret5 {
    #[inline(always)]
    fn from(val: u8) -> Ret5 {
        Ret5::from_bits(val)
    }
}
impl From<Ret5> for u8 {
    #[inline(always)]
    fn from(val: Ret5) -> u8 {
        Ret5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret6 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret6 {
    #[inline(always)]
    fn from(val: u8) -> Ret6 {
        Ret6::from_bits(val)
    }
}
impl From<Ret6> for u8 {
    #[inline(always)]
    fn from(val: Ret6) -> u8 {
        Ret6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret7 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret7 {
    #[inline(always)]
    fn from(val: u8) -> Ret7 {
        Ret7::from_bits(val)
    }
}
impl From<Ret7> for u8 {
    #[inline(always)]
    fn from(val: Ret7) -> u8 {
        Ret7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret8 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret8 {
    #[inline(always)]
    fn from(val: u8) -> Ret8 {
        Ret8::from_bits(val)
    }
}
impl From<Ret8> for u8 {
    #[inline(always)]
    fn from(val: Ret8) -> u8 {
        Ret8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ret9 {
    #[doc = "Retains"]
    POWEREDOFF = 0x0,
    #[doc = "Powers off"]
    RETAINED = 0x01,
}
impl Ret9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ret9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ret9 {
    #[inline(always)]
    fn from(val: u8) -> Ret9 {
        Ret9::from_bits(val)
    }
}
impl From<Ret9> for u8 {
    #[inline(always)]
    fn from(val: Ret9) -> u8 {
        Ret9::to_bits(val)
    }
}
