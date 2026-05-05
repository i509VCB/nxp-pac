#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CINSEL {
    #[doc = "Channel 0. CAPn.0 for CTIMERn."]
    CHANNEL_0 = 0x0,
    #[doc = "Channel 1. CAPn.1 for CTIMERn."]
    CHANNEL_1 = 0x01,
    #[doc = "Channel 2. CAPn.2 for CTIMERn."]
    CHANNEL_2 = 0x02,
    #[doc = "Channel 3. CAPn.3 for CTIMERn."]
    CHANNEL_3 = 0x03,
}
impl CINSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CINSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CINSEL {
    #[inline(always)]
    fn from(val: u8) -> CINSEL {
        CINSEL::from_bits(val)
    }
}
impl From<CINSEL> for u8 {
    #[inline(always)]
    fn from(val: CINSEL) -> u8 {
        CINSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTMODE {
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    TIMER = 0x0,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING_EDGE = 0x01,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING_EDGE = 0x02,
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_DUAL_EDGE = 0x03,
}
impl CTMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTMODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTMODE {
    #[inline(always)]
    fn from(val: u8) -> CTMODE {
        CTMODE::from_bits(val)
    }
}
impl From<CTMODE> for u8 {
    #[inline(always)]
    fn from(val: CTMODE) -> u8 {
        CTMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EMC0 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT0 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT0 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl EMC0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EMC0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EMC0 {
    #[inline(always)]
    fn from(val: u8) -> EMC0 {
        EMC0::from_bits(val)
    }
}
impl From<EMC0> for u8 {
    #[inline(always)]
    fn from(val: EMC0) -> u8 {
        EMC0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EMC1 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT1 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT1 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl EMC1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EMC1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EMC1 {
    #[inline(always)]
    fn from(val: u8) -> EMC1 {
        EMC1::from_bits(val)
    }
}
impl From<EMC1> for u8 {
    #[inline(always)]
    fn from(val: EMC1) -> u8 {
        EMC1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EMC2 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT2 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT2 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl EMC2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EMC2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EMC2 {
    #[inline(always)]
    fn from(val: u8) -> EMC2 {
        EMC2::from_bits(val)
    }
}
impl From<EMC2> for u8 {
    #[inline(always)]
    fn from(val: EMC2) -> u8 {
        EMC2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EMC3 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT3 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT3 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl EMC3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EMC3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EMC3 {
    #[inline(always)]
    fn from(val: u8) -> EMC3 {
        EMC3::from_bits(val)
    }
}
impl From<EMC3> for u8 {
    #[inline(always)]
    fn from(val: EMC3) -> u8 {
        EMC3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWMEN0 {
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    PWM = 0x01,
}
impl PWMEN0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWMEN0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWMEN0 {
    #[inline(always)]
    fn from(val: u8) -> PWMEN0 {
        PWMEN0::from_bits(val)
    }
}
impl From<PWMEN0> for u8 {
    #[inline(always)]
    fn from(val: PWMEN0) -> u8 {
        PWMEN0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWMEN1 {
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    PWM = 0x01,
}
impl PWMEN1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWMEN1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWMEN1 {
    #[inline(always)]
    fn from(val: u8) -> PWMEN1 {
        PWMEN1::from_bits(val)
    }
}
impl From<PWMEN1> for u8 {
    #[inline(always)]
    fn from(val: PWMEN1) -> u8 {
        PWMEN1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWMEN2 {
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    PWM = 0x01,
}
impl PWMEN2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWMEN2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWMEN2 {
    #[inline(always)]
    fn from(val: u8) -> PWMEN2 {
        PWMEN2::from_bits(val)
    }
}
impl From<PWMEN2> for u8 {
    #[inline(always)]
    fn from(val: PWMEN2) -> u8 {
        PWMEN2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWMEN3 {
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    PWM = 0x01,
}
impl PWMEN3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWMEN3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWMEN3 {
    #[inline(always)]
    fn from(val: u8) -> PWMEN3 {
        PWMEN3::from_bits(val)
    }
}
impl From<PWMEN3> for u8 {
    #[inline(always)]
    fn from(val: PWMEN3) -> u8 {
        PWMEN3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SELCC {
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_RISING = 0x0,
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_FALLING = 0x01,
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_RISING = 0x02,
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_FALLING = 0x03,
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_RISING = 0x04,
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_FALLING = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SELCC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SELCC {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SELCC {
    #[inline(always)]
    fn from(val: u8) -> SELCC {
        SELCC::from_bits(val)
    }
}
impl From<SELCC> for u8 {
    #[inline(always)]
    fn from(val: SELCC) -> u8 {
        SELCC::to_bits(val)
    }
}
