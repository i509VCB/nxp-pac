#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_OSC_BYPASS {
    #[doc = "The RTC Oscillator operates normally as a crystal oscillator with the crystal connected between the RTC_XTALIN and RTC_XTALOUT pins."]
    USED = 0x0,
    #[doc = "The RTC Oscillator is in bypass mode. In this mode a clock can be directly input into the RTC_XTALIN pin."]
    BYPASS = 0x01,
}
impl RTC_OSC_BYPASS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_OSC_BYPASS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_OSC_BYPASS {
    #[inline(always)]
    fn from(val: u8) -> RTC_OSC_BYPASS {
        RTC_OSC_BYPASS::from_bits(val)
    }
}
impl From<RTC_OSC_BYPASS> for u8 {
    #[inline(always)]
    fn from(val: RTC_OSC_BYPASS) -> u8 {
        RTC_OSC_BYPASS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_OSC_PD {
    #[doc = "See RTC_OSC_BYPASS."]
    POWER_UP = 0x0,
    #[doc = "RTC oscillator is powered-down."]
    POWERED_DOWN = 0x01,
}
impl RTC_OSC_PD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_OSC_PD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_OSC_PD {
    #[inline(always)]
    fn from(val: u8) -> RTC_OSC_PD {
        RTC_OSC_PD::from_bits(val)
    }
}
impl From<RTC_OSC_PD> for u8 {
    #[inline(always)]
    fn from(val: RTC_OSC_PD) -> u8 {
        RTC_OSC_PD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_SUBSEC_ENA {
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'."]
    POWER_UP = 0x0,
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting commences on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) is set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep power-down mode."]
    POWERED_DOWN = 0x01,
}
impl RTC_SUBSEC_ENA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_SUBSEC_ENA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_SUBSEC_ENA {
    #[inline(always)]
    fn from(val: u8) -> RTC_SUBSEC_ENA {
        RTC_SUBSEC_ENA::from_bits(val)
    }
}
impl From<RTC_SUBSEC_ENA> for u8 {
    #[inline(always)]
    fn from(val: RTC_SUBSEC_ENA) -> u8 {
        RTC_SUBSEC_ENA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WAKE1KHZ {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN = 0x0,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIMEOUT = 0x01,
}
impl WAKE1KHZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WAKE1KHZ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WAKE1KHZ {
    #[inline(always)]
    fn from(val: u8) -> WAKE1KHZ {
        WAKE1KHZ::from_bits(val)
    }
}
impl From<WAKE1KHZ> for u8 {
    #[inline(always)]
    fn from(val: WAKE1KHZ) -> u8 {
        WAKE1KHZ::to_bits(val)
    }
}
