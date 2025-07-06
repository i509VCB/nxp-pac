#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrWakeTimer {
    #[doc = "No effect"]
    DISABLE = 0x0,
    #[doc = "Clear the wake timer counter"]
    ENABLE = 0x01,
}
impl ClrWakeTimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrWakeTimer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrWakeTimer {
    #[inline(always)]
    fn from(val: u8) -> ClrWakeTimer {
        ClrWakeTimer::from_bits(val)
    }
}
impl From<ClrWakeTimer> for u8 {
    #[inline(always)]
    fn from(val: ClrWakeTimer) -> u8 {
        ClrWakeTimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl IntrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrEn {
    #[inline(always)]
    fn from(val: u8) -> IntrEn {
        IntrEn::from_bits(val)
    }
}
impl From<IntrEn> for u8 {
    #[inline(always)]
    fn from(val: IntrEn) -> u8 {
        IntrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscDivEna {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl OscDivEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscDivEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscDivEna {
    #[inline(always)]
    fn from(val: u8) -> OscDivEna {
        OscDivEna::from_bits(val)
    }
}
impl From<OscDivEna> for u8 {
    #[inline(always)]
    fn from(val: OscDivEna) -> u8 {
        OscDivEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SubSecondCntEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SubSecondCntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubSecondCntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubSecondCntEn {
    #[inline(always)]
    fn from(val: u8) -> SubSecondCntEn {
        SubSecondCntEn::from_bits(val)
    }
}
impl From<SubSecondCntEn> for u8 {
    #[inline(always)]
    fn from(val: SubSecondCntEn) -> u8 {
        SubSecondCntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeFlag {
    #[doc = "Not timed out"]
    DISABLE = 0x0,
    #[doc = "Timed out"]
    ENABLE = 0x01,
}
impl WakeFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeFlag {
    #[inline(always)]
    fn from(val: u8) -> WakeFlag {
        WakeFlag::from_bits(val)
    }
}
impl From<WakeFlag> for u8 {
    #[inline(always)]
    fn from(val: WakeFlag) -> u8 {
        WakeFlag::to_bits(val)
    }
}
