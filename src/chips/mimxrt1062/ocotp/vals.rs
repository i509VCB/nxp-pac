#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FieldReturn {
    #[doc = "The device is a functional part."]
    FUNCTIONAL = 0x0,
    #[doc = "The device is a field returned part."]
    FIELD_RETURNED = 0x01,
}
impl FieldReturn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FieldReturn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FieldReturn {
    #[inline(always)]
    fn from(val: u8) -> FieldReturn {
        FieldReturn::from_bits(val)
    }
}
impl From<FieldReturn> for u8 {
    #[inline(always)]
    fn from(val: FieldReturn) -> u8 {
        FieldReturn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HabJde {
    #[doc = "JTAG debugging is not enabled by the HAB (it may still be enabled by other mechanisms)."]
    JTAG_DBG_DIS = 0x0,
    #[doc = "JTAG debugging is enabled by the HAB (though this signal may be gated off)."]
    JTAG_DBG_EN = 0x01,
}
impl HabJde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HabJde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HabJde {
    #[inline(always)]
    fn from(val: u8) -> HabJde {
        HabJde::from_bits(val)
    }
}
impl From<HabJde> for u8 {
    #[inline(always)]
    fn from(val: HabJde) -> u8 {
        HabJde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReloadShadows {
    #[doc = "Do not force shadow register re-load."]
    SHADOW_NOFORCE_RELOAD = 0x0,
    #[doc = "Force shadow register re-load. This bit is cleared automatically after shadow registers are re-loaded."]
    SHADOW_FORCE_RELOAD = 0x01,
}
impl ReloadShadows {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReloadShadows {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReloadShadows {
    #[inline(always)]
    fn from(val: u8) -> ReloadShadows {
        ReloadShadows::from_bits(val)
    }
}
impl From<ReloadShadows> for u8 {
    #[inline(always)]
    fn from(val: ReloadShadows) -> u8 {
        ReloadShadows::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct WrUnlock(u16);
impl WrUnlock {
    #[doc = "OTP write access is locked."]
    pub const NO_KEY: Self = Self(0x0);
    #[doc = "OTP write access is unlocked."]
    pub const KEY: Self = Self(0x3e77);
}
impl WrUnlock {
    pub const fn from_bits(val: u16) -> WrUnlock {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for WrUnlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_KEY"),
            0x3e77 => f.write_str("KEY"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WrUnlock {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_KEY"),
            0x3e77 => defmt::write!(f, "KEY"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for WrUnlock {
    #[inline(always)]
    fn from(val: u16) -> WrUnlock {
        WrUnlock::from_bits(val)
    }
}
impl From<WrUnlock> for u16 {
    #[inline(always)]
    fn from(val: WrUnlock) -> u16 {
        WrUnlock::to_bits(val)
    }
}
