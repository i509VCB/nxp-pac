#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerIntena {
    #[doc = "Interrupts blocked"]
    INTERRUPTS_BLOCKED = 0x0,
    #[doc = "Interrupts enabled"]
    INTERRUPTS_ENABLED = 0x01,
}
impl OstimerIntena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerIntena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerIntena {
    #[inline(always)]
    fn from(val: u8) -> OstimerIntena {
        OstimerIntena::from_bits(val)
    }
}
impl From<OstimerIntena> for u8 {
    #[inline(always)]
    fn from(val: OstimerIntena) -> u8 {
        OstimerIntena::to_bits(val)
    }
}
