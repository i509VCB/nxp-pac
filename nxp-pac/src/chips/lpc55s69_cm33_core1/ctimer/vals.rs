#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinsel {
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    CHANNEL_0 = 0x0,
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    CHANNEL_1 = 0x01,
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    CHANNEL_2 = 0x02,
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    CHANNEL_3 = 0x03,
}
impl Cinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinsel {
    #[inline(always)]
    fn from(val: u8) -> Cinsel {
        Cinsel::from_bits(val)
    }
}
impl From<Cinsel> for u8 {
    #[inline(always)]
    fn from(val: Cinsel) -> u8 {
        Cinsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctmode {
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    TIMER = 0x0,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING_EDGE = 0x01,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING_EDGE = 0x02,
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_DUAL_EDGE = 0x03,
}
impl Ctmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctmode {
    #[inline(always)]
    fn from(val: u8) -> Ctmode {
        Ctmode::from_bits(val)
    }
}
impl From<Ctmode> for u8 {
    #[inline(always)]
    fn from(val: Ctmode) -> u8 {
        Ctmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc0 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT0 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT0 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl Emc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc0 {
    #[inline(always)]
    fn from(val: u8) -> Emc0 {
        Emc0::from_bits(val)
    }
}
impl From<Emc0> for u8 {
    #[inline(always)]
    fn from(val: Emc0) -> u8 {
        Emc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc1 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT1 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT1 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl Emc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc1 {
    #[inline(always)]
    fn from(val: u8) -> Emc1 {
        Emc1::from_bits(val)
    }
}
impl From<Emc1> for u8 {
    #[inline(always)]
    fn from(val: Emc1) -> u8 {
        Emc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc2 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT2 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT2 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl Emc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc2 {
    #[inline(always)]
    fn from(val: u8) -> Emc2 {
        Emc2::from_bits(val)
    }
}
impl From<Emc2> for u8 {
    #[inline(always)]
    fn from(val: Emc2) -> u8 {
        Emc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc3 {
    #[doc = "Do Nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT3 pin is LOW if pinned out)."]
    CLEAR = 0x01,
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT3 pin is HIGH if pinned out)."]
    SET = 0x02,
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 0x03,
}
impl Emc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc3 {
    #[inline(always)]
    fn from(val: u8) -> Emc3 {
        Emc3::from_bits(val)
    }
}
impl From<Emc3> for u8 {
    #[inline(always)]
    fn from(val: Emc3) -> u8 {
        Emc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen0 {
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    PWM = 0x01,
}
impl Pwmen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen0 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen0 {
        Pwmen0::from_bits(val)
    }
}
impl From<Pwmen0> for u8 {
    #[inline(always)]
    fn from(val: Pwmen0) -> u8 {
        Pwmen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen1 {
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    PWM = 0x01,
}
impl Pwmen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen1 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen1 {
        Pwmen1::from_bits(val)
    }
}
impl From<Pwmen1> for u8 {
    #[inline(always)]
    fn from(val: Pwmen1) -> u8 {
        Pwmen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen2 {
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    PWM = 0x01,
}
impl Pwmen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen2 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen2 {
        Pwmen2::from_bits(val)
    }
}
impl From<Pwmen2> for u8 {
    #[inline(always)]
    fn from(val: Pwmen2) -> u8 {
        Pwmen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen3 {
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    MATCH = 0x0,
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    PWM = 0x01,
}
impl Pwmen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen3 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen3 {
        Pwmen3::from_bits(val)
    }
}
impl From<Pwmen3> for u8 {
    #[inline(always)]
    fn from(val: Pwmen3) -> u8 {
        Pwmen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Selcc {
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
impl Selcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selcc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selcc {
    #[inline(always)]
    fn from(val: u8) -> Selcc {
        Selcc::from_bits(val)
    }
}
impl From<Selcc> for u8 {
    #[inline(always)]
    fn from(val: Selcc) -> u8 {
        Selcc::to_bits(val)
    }
}
