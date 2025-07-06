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
pub enum CkctrlLock {
    #[doc = "Allowed"]
    DISABLED = 0x0,
    #[doc = "Blocked"]
    ENABLED = 0x01,
}
impl CkctrlLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CkctrlLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CkctrlLock {
    #[inline(always)]
    fn from(val: u8) -> CkctrlLock {
        CkctrlLock::from_bits(val)
    }
}
impl From<CkctrlLock> for u8 {
    #[inline(always)]
    fn from(val: CkctrlLock) -> u8 {
        CkctrlLock::to_bits(val)
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
pub enum Filten {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl Filten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filten {
    #[inline(always)]
    fn from(val: u8) -> Filten {
        Filten::from_bits(val)
    }
}
impl From<Filten> for u8 {
    #[inline(always)]
    fn from(val: Filten) -> u8 {
        Filten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashdis {
    #[doc = "No effect"]
    DISABLED = 0x0,
    #[doc = "Flash memory is disabled"]
    ENABLED = 0x01,
}
impl Flashdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashdis {
    #[inline(always)]
    fn from(val: u8) -> Flashdis {
        Flashdis::from_bits(val)
    }
}
impl From<Flashdis> for u8 {
    #[inline(always)]
    fn from(val: Flashdis) -> u8 {
        Flashdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashdoze {
    #[doc = "No effect"]
    DISABLED = 0x0,
    #[doc = "Flash memory is disabled when core is sleeping (CKMODE > 0)"]
    ENABLED = 0x01,
}
impl Flashdoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashdoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashdoze {
    #[inline(always)]
    fn from(val: u8) -> Flashdoze {
        Flashdoze::from_bits(val)
    }
}
impl From<Flashdoze> for u8 {
    #[inline(always)]
    fn from(val: Flashdoze) -> u8 {
        Flashdoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Forcecfg {
    #[doc = "No effect"]
    DISABLED = 0x0,
    #[doc = "Asserts"]
    ENABLED = 0x01,
}
impl Forcecfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Forcecfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Forcecfg {
    #[inline(always)]
    fn from(val: u8) -> Forcecfg {
        Forcecfg::from_bits(val)
    }
}
impl From<Forcecfg> for u8 {
    #[inline(always)]
    fn from(val: Forcecfg) -> u8 {
        Forcecfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpfen {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl Lpfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpfen {
    #[inline(always)]
    fn from(val: u8) -> Lpfen {
        Lpfen::from_bits(val)
    }
}
impl From<Lpfen> for u8 {
    #[inline(always)]
    fn from(val: Lpfen) -> u8 {
        Lpfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npie {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl Npie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npie {
    #[inline(always)]
    fn from(val: u8) -> Npie {
        Npie::from_bits(val)
    }
}
impl From<Npie> for u8 {
    #[inline(always)]
    fn from(val: Npie) -> u8 {
        Npie::to_bits(val)
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
pub enum PmprotLock {
    #[doc = "Allowed"]
    DISABLED = 0x0,
    #[doc = "Blocked"]
    ENABLED = 0x01,
}
impl PmprotLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmprotLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmprotLock {
    #[inline(always)]
    fn from(val: u8) -> PmprotLock {
        PmprotLock::from_bits(val)
    }
}
impl From<PmprotLock> for u8 {
    #[inline(always)]
    fn from(val: PmprotLock) -> u8 {
        PmprotLock::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sod {
    #[doc = "Remains enabled"]
    DISABLED = 0x0,
    #[doc = "Disabled"]
    ENABLED = 0x01,
}
impl Sod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sod {
    #[inline(always)]
    fn from(val: u8) -> Sod {
        Sod::from_bits(val)
    }
}
impl From<Sod> for u8 {
    #[inline(always)]
    fn from(val: Sod) -> u8 {
        Sod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieCdog0 {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieCdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieCdog0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieCdog0 {
    #[inline(always)]
    fn from(val: u8) -> SrieCdog0 {
        SrieCdog0::from_bits(val)
    }
}
impl From<SrieCdog0> for u8 {
    #[inline(always)]
    fn from(val: SrieCdog0) -> u8 {
        SrieCdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieCdog1 {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieCdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieCdog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieCdog1 {
    #[inline(always)]
    fn from(val: u8) -> SrieCdog1 {
        SrieCdog1::from_bits(val)
    }
}
impl From<SrieCdog1> for u8 {
    #[inline(always)]
    fn from(val: SrieCdog1) -> u8 {
        SrieCdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieCpu1 {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieCpu1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieCpu1 {
    #[inline(always)]
    fn from(val: u8) -> SrieCpu1 {
        SrieCpu1::from_bits(val)
    }
}
impl From<SrieCpu1> for u8 {
    #[inline(always)]
    fn from(val: SrieCpu1) -> u8 {
        SrieCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieDap {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieDap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieDap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieDap {
    #[inline(always)]
    fn from(val: u8) -> SrieDap {
        SrieDap::from_bits(val)
    }
}
impl From<SrieDap> for u8 {
    #[inline(always)]
    fn from(val: SrieDap) -> u8 {
        SrieDap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieLockup {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieLockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieLockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieLockup {
    #[inline(always)]
    fn from(val: u8) -> SrieLockup {
        SrieLockup::from_bits(val)
    }
}
impl From<SrieLockup> for u8 {
    #[inline(always)]
    fn from(val: SrieLockup) -> u8 {
        SrieLockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieLpack {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieLpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieLpack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieLpack {
    #[inline(always)]
    fn from(val: u8) -> SrieLpack {
        SrieLpack::from_bits(val)
    }
}
impl From<SrieLpack> for u8 {
    #[inline(always)]
    fn from(val: SrieLpack) -> u8 {
        SrieLpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SriePin {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SriePin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SriePin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SriePin {
    #[inline(always)]
    fn from(val: u8) -> SriePin {
        SriePin::from_bits(val)
    }
}
impl From<SriePin> for u8 {
    #[inline(always)]
    fn from(val: SriePin) -> u8 {
        SriePin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieScg {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieScg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieScg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieScg {
    #[inline(always)]
    fn from(val: u8) -> SrieScg {
        SrieScg::from_bits(val)
    }
}
impl From<SrieScg> for u8 {
    #[inline(always)]
    fn from(val: SrieScg) -> u8 {
        SrieScg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieSw {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieSw {
    #[inline(always)]
    fn from(val: u8) -> SrieSw {
        SrieSw::from_bits(val)
    }
}
impl From<SrieSw> for u8 {
    #[inline(always)]
    fn from(val: SrieSw) -> u8 {
        SrieSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieVbat {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieVbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieVbat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieVbat {
    #[inline(always)]
    fn from(val: u8) -> SrieVbat {
        SrieVbat::from_bits(val)
    }
}
impl From<SrieVbat> for u8 {
    #[inline(always)]
    fn from(val: SrieVbat) -> u8 {
        SrieVbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieWwdt0 {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieWwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieWwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieWwdt0 {
    #[inline(always)]
    fn from(val: u8) -> SrieWwdt0 {
        SrieWwdt0::from_bits(val)
    }
}
impl From<SrieWwdt0> for u8 {
    #[inline(always)]
    fn from(val: SrieWwdt0) -> u8 {
        SrieWwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrieWwdt1 {
    #[doc = "Interrupt disabled"]
    DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
    ENABLED = 0x01,
}
impl SrieWwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrieWwdt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrieWwdt1 {
    #[inline(always)]
    fn from(val: u8) -> SrieWwdt1 {
        SrieWwdt1::from_bits(val)
    }
}
impl From<SrieWwdt1> for u8 {
    #[inline(always)]
    fn from(val: SrieWwdt1) -> u8 {
        SrieWwdt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifCdog0 {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifCdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifCdog0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifCdog0 {
    #[inline(always)]
    fn from(val: u8) -> SrifCdog0 {
        SrifCdog0::from_bits(val)
    }
}
impl From<SrifCdog0> for u8 {
    #[inline(always)]
    fn from(val: SrifCdog0) -> u8 {
        SrifCdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifCdog1 {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifCdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifCdog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifCdog1 {
    #[inline(always)]
    fn from(val: u8) -> SrifCdog1 {
        SrifCdog1::from_bits(val)
    }
}
impl From<SrifCdog1> for u8 {
    #[inline(always)]
    fn from(val: SrifCdog1) -> u8 {
        SrifCdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifCpu1 {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifCpu1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifCpu1 {
    #[inline(always)]
    fn from(val: u8) -> SrifCpu1 {
        SrifCpu1::from_bits(val)
    }
}
impl From<SrifCpu1> for u8 {
    #[inline(always)]
    fn from(val: SrifCpu1) -> u8 {
        SrifCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifDap {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifDap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifDap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifDap {
    #[inline(always)]
    fn from(val: u8) -> SrifDap {
        SrifDap::from_bits(val)
    }
}
impl From<SrifDap> for u8 {
    #[inline(always)]
    fn from(val: SrifDap) -> u8 {
        SrifDap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifLockup {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifLockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifLockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifLockup {
    #[inline(always)]
    fn from(val: u8) -> SrifLockup {
        SrifLockup::from_bits(val)
    }
}
impl From<SrifLockup> for u8 {
    #[inline(always)]
    fn from(val: SrifLockup) -> u8 {
        SrifLockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifLpack {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifLpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifLpack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifLpack {
    #[inline(always)]
    fn from(val: u8) -> SrifLpack {
        SrifLpack::from_bits(val)
    }
}
impl From<SrifLpack> for u8 {
    #[inline(always)]
    fn from(val: SrifLpack) -> u8 {
        SrifLpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifPin {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifPin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifPin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifPin {
    #[inline(always)]
    fn from(val: u8) -> SrifPin {
        SrifPin::from_bits(val)
    }
}
impl From<SrifPin> for u8 {
    #[inline(always)]
    fn from(val: SrifPin) -> u8 {
        SrifPin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifSw {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifSw {
    #[inline(always)]
    fn from(val: u8) -> SrifSw {
        SrifSw::from_bits(val)
    }
}
impl From<SrifSw> for u8 {
    #[inline(always)]
    fn from(val: SrifSw) -> u8 {
        SrifSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifVbat {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifVbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifVbat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifVbat {
    #[inline(always)]
    fn from(val: u8) -> SrifVbat {
        SrifVbat::from_bits(val)
    }
}
impl From<SrifVbat> for u8 {
    #[inline(always)]
    fn from(val: SrifVbat) -> u8 {
        SrifVbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifWwdt0 {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifWwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifWwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifWwdt0 {
    #[inline(always)]
    fn from(val: u8) -> SrifWwdt0 {
        SrifWwdt0::from_bits(val)
    }
}
impl From<SrifWwdt0> for u8 {
    #[inline(always)]
    fn from(val: SrifWwdt0) -> u8 {
        SrifWwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrifWwdt1 {
    #[doc = "Reset source not pending"]
    DISABLED = 0x0,
    #[doc = "Reset source pending"]
    ENABLED = 0x01,
}
impl SrifWwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrifWwdt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrifWwdt1 {
    #[inline(always)]
    fn from(val: u8) -> SrifWwdt1 {
        SrifWwdt1::from_bits(val)
    }
}
impl From<SrifWwdt1> for u8 {
    #[inline(always)]
    fn from(val: SrifWwdt1) -> u8 {
        SrifWwdt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsCdog0 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SrsCdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsCdog0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsCdog0 {
    #[inline(always)]
    fn from(val: u8) -> SrsCdog0 {
        SrsCdog0::from_bits(val)
    }
}
impl From<SrsCdog0> for u8 {
    #[inline(always)]
    fn from(val: SrsCdog0) -> u8 {
        SrsCdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsCdog1 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SrsCdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsCdog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsCdog1 {
    #[inline(always)]
    fn from(val: u8) -> SrsCdog1 {
        SrsCdog1::from_bits(val)
    }
}
impl From<SrsCdog1> for u8 {
    #[inline(always)]
    fn from(val: SrsCdog1) -> u8 {
        SrsCdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsCpu1 {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsCpu1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsCpu1 {
    #[inline(always)]
    fn from(val: u8) -> SrsCpu1 {
        SrsCpu1::from_bits(val)
    }
}
impl From<SrsCpu1> for u8 {
    #[inline(always)]
    fn from(val: SrsCpu1) -> u8 {
        SrsCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsDap {
    #[doc = "Reset was not generated"]
    DISABLED = 0x0,
    #[doc = "Reset was generated"]
    ENABLED = 0x01,
}
impl SrsDap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsDap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsDap {
    #[inline(always)]
    fn from(val: u8) -> SrsDap {
        SrsDap::from_bits(val)
    }
}
impl From<SrsDap> for u8 {
    #[inline(always)]
    fn from(val: SrsDap) -> u8 {
        SrsDap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsFatal {
    #[doc = "Reset was not generated"]
    DISABLED = 0x0,
    #[doc = "Reset was generated"]
    ENABLED = 0x01,
}
impl SrsFatal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsFatal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsFatal {
    #[inline(always)]
    fn from(val: u8) -> SrsFatal {
        SrsFatal::from_bits(val)
    }
}
impl From<SrsFatal> for u8 {
    #[inline(always)]
    fn from(val: SrsFatal) -> u8 {
        SrsFatal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsJtag {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsJtag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsJtag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsJtag {
    #[inline(always)]
    fn from(val: u8) -> SrsJtag {
        SrsJtag::from_bits(val)
    }
}
impl From<SrsJtag> for u8 {
    #[inline(always)]
    fn from(val: SrsJtag) -> u8 {
        SrsJtag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsLockup {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsLockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsLockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsLockup {
    #[inline(always)]
    fn from(val: u8) -> SrsLockup {
        SrsLockup::from_bits(val)
    }
}
impl From<SrsLockup> for u8 {
    #[inline(always)]
    fn from(val: SrsLockup) -> u8 {
        SrsLockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsLpack {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsLpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsLpack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsLpack {
    #[inline(always)]
    fn from(val: u8) -> SrsLpack {
        SrsLpack::from_bits(val)
    }
}
impl From<SrsLpack> for u8 {
    #[inline(always)]
    fn from(val: SrsLpack) -> u8 {
        SrsLpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsPin {
    #[doc = "Reset was not generated"]
    DISABLED = 0x0,
    #[doc = "Reset was generated"]
    ENABLED = 0x01,
}
impl SrsPin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsPin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsPin {
    #[inline(always)]
    fn from(val: u8) -> SrsPin {
        SrsPin::from_bits(val)
    }
}
impl From<SrsPin> for u8 {
    #[inline(always)]
    fn from(val: SrsPin) -> u8 {
        SrsPin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsPor {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsPor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsPor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsPor {
    #[inline(always)]
    fn from(val: u8) -> SrsPor {
        SrsPor::from_bits(val)
    }
}
impl From<SrsPor> for u8 {
    #[inline(always)]
    fn from(val: SrsPor) -> u8 {
        SrsPor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsRstack {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsRstack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsRstack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsRstack {
    #[inline(always)]
    fn from(val: u8) -> SrsRstack {
        SrsRstack::from_bits(val)
    }
}
impl From<SrsRstack> for u8 {
    #[inline(always)]
    fn from(val: SrsRstack) -> u8 {
        SrsRstack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsScg {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SrsScg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsScg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsScg {
    #[inline(always)]
    fn from(val: u8) -> SrsScg {
        SrsScg::from_bits(val)
    }
}
impl From<SrsScg> for u8 {
    #[inline(always)]
    fn from(val: SrsScg) -> u8 {
        SrsScg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsSecvio {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsSecvio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsSecvio {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsSecvio {
    #[inline(always)]
    fn from(val: u8) -> SrsSecvio {
        SrsSecvio::from_bits(val)
    }
}
impl From<SrsSecvio> for u8 {
    #[inline(always)]
    fn from(val: SrsSecvio) -> u8 {
        SrsSecvio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsSw {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsSw {
    #[inline(always)]
    fn from(val: u8) -> SrsSw {
        SrsSw::from_bits(val)
    }
}
impl From<SrsSw> for u8 {
    #[inline(always)]
    fn from(val: SrsSw) -> u8 {
        SrsSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsTamper {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsTamper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsTamper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsTamper {
    #[inline(always)]
    fn from(val: u8) -> SrsTamper {
        SrsTamper::from_bits(val)
    }
}
impl From<SrsTamper> for u8 {
    #[inline(always)]
    fn from(val: SrsTamper) -> u8 {
        SrsTamper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsVbat {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsVbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsVbat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsVbat {
    #[inline(always)]
    fn from(val: u8) -> SrsVbat {
        SrsVbat::from_bits(val)
    }
}
impl From<SrsVbat> for u8 {
    #[inline(always)]
    fn from(val: SrsVbat) -> u8 {
        SrsVbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsVd {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsVd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsVd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsVd {
    #[inline(always)]
    fn from(val: u8) -> SrsVd {
        SrsVd::from_bits(val)
    }
}
impl From<SrsVd> for u8 {
    #[inline(always)]
    fn from(val: SrsVd) -> u8 {
        SrsVd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsWakeup {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsWakeup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsWakeup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsWakeup {
    #[inline(always)]
    fn from(val: u8) -> SrsWakeup {
        SrsWakeup::from_bits(val)
    }
}
impl From<SrsWakeup> for u8 {
    #[inline(always)]
    fn from(val: SrsWakeup) -> u8 {
        SrsWakeup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsWarm {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SrsWarm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsWarm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsWarm {
    #[inline(always)]
    fn from(val: u8) -> SrsWarm {
        SrsWarm::from_bits(val)
    }
}
impl From<SrsWarm> for u8 {
    #[inline(always)]
    fn from(val: SrsWarm) -> u8 {
        SrsWarm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsWwdt0 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SrsWwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsWwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsWwdt0 {
    #[inline(always)]
    fn from(val: u8) -> SrsWwdt0 {
        SrsWwdt0::from_bits(val)
    }
}
impl From<SrsWwdt0> for u8 {
    #[inline(always)]
    fn from(val: SrsWwdt0) -> u8 {
        SrsWwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrsWwdt1 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SrsWwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrsWwdt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrsWwdt1 {
    #[inline(always)]
    fn from(val: u8) -> SrsWwdt1 {
        SrsWwdt1::from_bits(val)
    }
}
impl From<SrsWwdt1> for u8 {
    #[inline(always)]
    fn from(val: SrsWwdt1) -> u8 {
        SrsWwdt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsCdog0 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SsrsCdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsCdog0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsCdog0 {
    #[inline(always)]
    fn from(val: u8) -> SsrsCdog0 {
        SsrsCdog0::from_bits(val)
    }
}
impl From<SsrsCdog0> for u8 {
    #[inline(always)]
    fn from(val: SsrsCdog0) -> u8 {
        SsrsCdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsCdog1 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SsrsCdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsCdog1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsCdog1 {
    #[inline(always)]
    fn from(val: u8) -> SsrsCdog1 {
        SsrsCdog1::from_bits(val)
    }
}
impl From<SsrsCdog1> for u8 {
    #[inline(always)]
    fn from(val: SsrsCdog1) -> u8 {
        SsrsCdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsCpu1 {
    #[doc = "Reset not generated from CPU1 reset source."]
    DISABLED = 0x0,
    #[doc = "Reset generated from CPU1 reset source."]
    ENABLED = 0x01,
}
impl SsrsCpu1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsCpu1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsCpu1 {
    #[inline(always)]
    fn from(val: u8) -> SsrsCpu1 {
        SsrsCpu1::from_bits(val)
    }
}
impl From<SsrsCpu1> for u8 {
    #[inline(always)]
    fn from(val: SsrsCpu1) -> u8 {
        SsrsCpu1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsDap {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsDap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsDap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsDap {
    #[inline(always)]
    fn from(val: u8) -> SsrsDap {
        SsrsDap::from_bits(val)
    }
}
impl From<SsrsDap> for u8 {
    #[inline(always)]
    fn from(val: SsrsDap) -> u8 {
        SsrsDap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsFatal {
    #[doc = "Reset was not generated"]
    DISABLED = 0x0,
    #[doc = "Reset was generated"]
    ENABLED = 0x01,
}
impl SsrsFatal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsFatal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsFatal {
    #[inline(always)]
    fn from(val: u8) -> SsrsFatal {
        SsrsFatal::from_bits(val)
    }
}
impl From<SsrsFatal> for u8 {
    #[inline(always)]
    fn from(val: SsrsFatal) -> u8 {
        SsrsFatal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsJtag {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsJtag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsJtag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsJtag {
    #[inline(always)]
    fn from(val: u8) -> SsrsJtag {
        SsrsJtag::from_bits(val)
    }
}
impl From<SsrsJtag> for u8 {
    #[inline(always)]
    fn from(val: SsrsJtag) -> u8 {
        SsrsJtag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsLockup {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsLockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsLockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsLockup {
    #[inline(always)]
    fn from(val: u8) -> SsrsLockup {
        SsrsLockup::from_bits(val)
    }
}
impl From<SsrsLockup> for u8 {
    #[inline(always)]
    fn from(val: SsrsLockup) -> u8 {
        SsrsLockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsLpack {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsLpack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsLpack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsLpack {
    #[inline(always)]
    fn from(val: u8) -> SsrsLpack {
        SsrsLpack::from_bits(val)
    }
}
impl From<SsrsLpack> for u8 {
    #[inline(always)]
    fn from(val: SsrsLpack) -> u8 {
        SsrsLpack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsPin {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsPin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsPin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsPin {
    #[inline(always)]
    fn from(val: u8) -> SsrsPin {
        SsrsPin::from_bits(val)
    }
}
impl From<SsrsPin> for u8 {
    #[inline(always)]
    fn from(val: SsrsPin) -> u8 {
        SsrsPin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsPor {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsPor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsPor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsPor {
    #[inline(always)]
    fn from(val: u8) -> SsrsPor {
        SsrsPor::from_bits(val)
    }
}
impl From<SsrsPor> for u8 {
    #[inline(always)]
    fn from(val: SsrsPor) -> u8 {
        SsrsPor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsRstack {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsRstack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsRstack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsRstack {
    #[inline(always)]
    fn from(val: u8) -> SsrsRstack {
        SsrsRstack::from_bits(val)
    }
}
impl From<SsrsRstack> for u8 {
    #[inline(always)]
    fn from(val: SsrsRstack) -> u8 {
        SsrsRstack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsScg {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SsrsScg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsScg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsScg {
    #[inline(always)]
    fn from(val: u8) -> SsrsScg {
        SsrsScg::from_bits(val)
    }
}
impl From<SsrsScg> for u8 {
    #[inline(always)]
    fn from(val: SsrsScg) -> u8 {
        SsrsScg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsSecvio {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsSecvio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsSecvio {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsSecvio {
    #[inline(always)]
    fn from(val: u8) -> SsrsSecvio {
        SsrsSecvio::from_bits(val)
    }
}
impl From<SsrsSecvio> for u8 {
    #[inline(always)]
    fn from(val: SsrsSecvio) -> u8 {
        SsrsSecvio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsSw {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsSw {
    #[inline(always)]
    fn from(val: u8) -> SsrsSw {
        SsrsSw::from_bits(val)
    }
}
impl From<SsrsSw> for u8 {
    #[inline(always)]
    fn from(val: SsrsSw) -> u8 {
        SsrsSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsTamper {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsTamper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsTamper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsTamper {
    #[inline(always)]
    fn from(val: u8) -> SsrsTamper {
        SsrsTamper::from_bits(val)
    }
}
impl From<SsrsTamper> for u8 {
    #[inline(always)]
    fn from(val: SsrsTamper) -> u8 {
        SsrsTamper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsVbat {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsVbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsVbat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsVbat {
    #[inline(always)]
    fn from(val: u8) -> SsrsVbat {
        SsrsVbat::from_bits(val)
    }
}
impl From<SsrsVbat> for u8 {
    #[inline(always)]
    fn from(val: SsrsVbat) -> u8 {
        SsrsVbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsVd {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsVd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsVd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsVd {
    #[inline(always)]
    fn from(val: u8) -> SsrsVd {
        SsrsVd::from_bits(val)
    }
}
impl From<SsrsVd> for u8 {
    #[inline(always)]
    fn from(val: SsrsVd) -> u8 {
        SsrsVd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsWakeup {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsWakeup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsWakeup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsWakeup {
    #[inline(always)]
    fn from(val: u8) -> SsrsWakeup {
        SsrsWakeup::from_bits(val)
    }
}
impl From<SsrsWakeup> for u8 {
    #[inline(always)]
    fn from(val: SsrsWakeup) -> u8 {
        SsrsWakeup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsWarm {
    #[doc = "Reset not generated"]
    DISABLED = 0x0,
    #[doc = "Reset generated"]
    ENABLED = 0x01,
}
impl SsrsWarm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsWarm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsWarm {
    #[inline(always)]
    fn from(val: u8) -> SsrsWarm {
        SsrsWarm::from_bits(val)
    }
}
impl From<SsrsWarm> for u8 {
    #[inline(always)]
    fn from(val: SsrsWarm) -> u8 {
        SsrsWarm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsWwdt0 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SsrsWwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsWwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsWwdt0 {
    #[inline(always)]
    fn from(val: u8) -> SsrsWwdt0 {
        SsrsWwdt0::from_bits(val)
    }
}
impl From<SsrsWwdt0> for u8 {
    #[inline(always)]
    fn from(val: SsrsWwdt0) -> u8 {
        SsrsWwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrsWwdt1 {
    #[doc = "Reset is not generated"]
    DISABLED = 0x0,
    #[doc = "Reset is generated"]
    ENABLED = 0x01,
}
impl SsrsWwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrsWwdt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrsWwdt1 {
    #[inline(always)]
    fn from(val: u8) -> SsrsWwdt1 {
        SsrsWwdt1::from_bits(val)
    }
}
impl From<SsrsWwdt1> for u8 {
    #[inline(always)]
    fn from(val: SsrsWwdt1) -> u8 {
        SsrsWwdt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Valid {
    #[doc = "Core clock not gated"]
    DISABLED = 0x0,
    #[doc = "Core clock was gated due to Low-Power mode entry"]
    ENABLED = 0x01,
}
impl Valid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Valid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Valid {
    #[inline(always)]
    fn from(val: u8) -> Valid {
        Valid::from_bits(val)
    }
}
impl From<Valid> for u8 {
    #[inline(always)]
    fn from(val: Valid) -> u8 {
        Valid::to_bits(val)
    }
}
