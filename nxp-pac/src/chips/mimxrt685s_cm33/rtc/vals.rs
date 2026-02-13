#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcOscPd {
    #[doc = "The RTC oscillator is enabled. This bit must be cleared in order for the RTC module to function"]
    ENABLE = 0x0,
    #[doc = "The RTC oscillator is shut-off to reserve power consumption. RTC operation is disabled."]
    SHUT_OFF = 0x01,
}
impl RtcOscPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcOscPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcOscPd {
    #[inline(always)]
    fn from(val: u8) -> RtcOscPd {
        RtcOscPd::from_bits(val)
    }
}
impl From<RtcOscPd> for u8 {
    #[inline(always)]
    fn from(val: RtcOscPd) -> u8 {
        RtcOscPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wake1khz {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN = 0x0,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIME_OUT = 0x01,
}
impl Wake1khz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wake1khz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wake1khz {
    #[inline(always)]
    fn from(val: u8) -> Wake1khz {
        Wake1khz::from_bits(val)
    }
}
impl From<Wake1khz> for u8 {
    #[inline(always)]
    fn from(val: Wake1khz) -> u8 {
        Wake1khz::to_bits(val)
    }
}
