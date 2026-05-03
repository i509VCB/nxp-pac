#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WDEN {
    #[doc = "Stop. The watchdog timer is stopped."]
    STOP = 0x0,
    #[doc = "Run. The watchdog timer is running."]
    RUN = 0x01,
}
impl WDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WDEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WDEN {
    #[inline(always)]
    fn from(val: u8) -> WDEN {
        WDEN::from_bits(val)
    }
}
impl From<WDEN> for u8 {
    #[inline(always)]
    fn from(val: WDEN) -> u8 {
        WDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WDPROTECT {
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    FLEXIBLE = 0x0,
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    THRESHOLD = 0x01,
}
impl WDPROTECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WDPROTECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WDPROTECT {
    #[inline(always)]
    fn from(val: u8) -> WDPROTECT {
        WDPROTECT::from_bits(val)
    }
}
impl From<WDPROTECT> for u8 {
    #[inline(always)]
    fn from(val: WDPROTECT) -> u8 {
        WDPROTECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WDRESET {
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    INTERRUPT = 0x0,
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    RESET = 0x01,
}
impl WDRESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WDRESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WDRESET {
    #[inline(always)]
    fn from(val: u8) -> WDRESET {
        WDRESET::from_bits(val)
    }
}
impl From<WDRESET> for u8 {
    #[inline(always)]
    fn from(val: WDRESET) -> u8 {
        WDRESET::to_bits(val)
    }
}
