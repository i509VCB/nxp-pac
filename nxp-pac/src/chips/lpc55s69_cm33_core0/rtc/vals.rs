#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcOscBypass {
    #[doc = "The RTC Oscillator operates normally as a crystal oscillator with the crystal connected between the RTC_XTALIN and RTC_XTALOUT pins."]
    USED = 0x0,
    #[doc = "The RTC Oscillator is in bypass mode. In this mode a clock can be directly input into the RTC_XTALIN pin."]
    BYPASS = 0x01,
}
impl RtcOscBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcOscBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcOscBypass {
    #[inline(always)]
    fn from(val: u8) -> RtcOscBypass {
        RtcOscBypass::from_bits(val)
    }
}
impl From<RtcOscBypass> for u8 {
    #[inline(always)]
    fn from(val: RtcOscBypass) -> u8 {
        RtcOscBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcOscPd {
    #[doc = "See RTC_OSC_BYPASS"]
    POWER_UP = 0x0,
    #[doc = "RTC oscillator is powered-down."]
    POWERED_DOWN = 0x01,
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
pub enum RtcSubsecEna {
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'."]
    POWER_UP = 0x0,
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting commences on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) is set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep power-down mode."]
    POWERED_DOWN = 0x01,
}
impl RtcSubsecEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcSubsecEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcSubsecEna {
    #[inline(always)]
    fn from(val: u8) -> RtcSubsecEna {
        RtcSubsecEna::from_bits(val)
    }
}
impl From<RtcSubsecEna> for u8 {
    #[inline(always)]
    fn from(val: RtcSubsecEna) -> u8 {
        RtcSubsecEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wake1khz {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN = 0x0,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIMEOUT = 0x01,
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
