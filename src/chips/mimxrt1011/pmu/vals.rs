#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClkgateCtrl {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE = 0x0,
    #[doc = "Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE = 0x01,
}
impl Misc0ClkgateCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClkgateCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClkgateCtrl {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClkgateCtrl {
        Misc0ClkgateCtrl::from_bits(val)
    }
}
impl From<Misc0ClkgateCtrl> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClkgateCtrl) -> u8 {
        Misc0ClkgateCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0ClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClkgateDelay {
        Misc0ClkgateDelay::from_bits(val)
    }
}
impl From<Misc0ClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClkgateDelay) -> u8 {
        Misc0ClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrClkgateCtrl {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE = 0x0,
    #[doc = "Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE = 0x01,
}
impl Misc0ClrClkgateCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrClkgateCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrClkgateCtrl {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrClkgateCtrl {
        Misc0ClrClkgateCtrl::from_bits(val)
    }
}
impl From<Misc0ClrClkgateCtrl> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrClkgateCtrl) -> u8 {
        Misc0ClrClkgateCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0ClrClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrClkgateDelay {
        Misc0ClrClkgateDelay::from_bits(val)
    }
}
impl From<Misc0ClrClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrClkgateDelay) -> u8 {
        Misc0ClrClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrDisconHighSnvs {
    #[doc = "Turn on the switch"]
    DISCON_HIGH_SNVS_0 = 0x0,
    #[doc = "Turn off the switch"]
    DISCON_HIGH_SNVS_1 = 0x01,
}
impl Misc0ClrDisconHighSnvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrDisconHighSnvs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrDisconHighSnvs {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrDisconHighSnvs {
        Misc0ClrDisconHighSnvs::from_bits(val)
    }
}
impl From<Misc0ClrDisconHighSnvs> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrDisconHighSnvs) -> u8 {
        Misc0ClrDisconHighSnvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrOscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0ClrOscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrOscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrOscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrOscI {
        Misc0ClrOscI::from_bits(val)
    }
}
impl From<Misc0ClrOscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrOscI) -> u8 {
        Misc0ClrOscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrReftopSelfbiasoff {
    #[doc = "Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0 = 0x0,
    #[doc = "Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1 = 0x01,
}
impl Misc0ClrReftopSelfbiasoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrReftopSelfbiasoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrReftopSelfbiasoff {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrReftopSelfbiasoff {
        Misc0ClrReftopSelfbiasoff::from_bits(val)
    }
}
impl From<Misc0ClrReftopSelfbiasoff> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrReftopSelfbiasoff) -> u8 {
        Misc0ClrReftopSelfbiasoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0ClrReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrReftopVbgadj {
        Misc0ClrReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0ClrReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrReftopVbgadj) -> u8 {
        Misc0ClrReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrRtcXtalSource {
    #[doc = "Internal ring oscillator"]
    RTC_XTAL_SOURCE_0 = 0x0,
    #[doc = "RTC_XTAL"]
    RTC_XTAL_SOURCE_1 = 0x01,
}
impl Misc0ClrRtcXtalSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrRtcXtalSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrRtcXtalSource {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrRtcXtalSource {
        Misc0ClrRtcXtalSource::from_bits(val)
    }
}
impl From<Misc0ClrRtcXtalSource> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrRtcXtalSource) -> u8 {
        Misc0ClrRtcXtalSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ClrStopModeConfig {
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0ClrStopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrStopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrStopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrStopModeConfig {
        Misc0ClrStopModeConfig::from_bits(val)
    }
}
impl From<Misc0ClrStopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrStopModeConfig) -> u8 {
        Misc0ClrStopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0DisconHighSnvs {
    #[doc = "Turn on the switch"]
    DISCON_HIGH_SNVS_0 = 0x0,
    #[doc = "Turn off the switch"]
    DISCON_HIGH_SNVS_1 = 0x01,
}
impl Misc0DisconHighSnvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0DisconHighSnvs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0DisconHighSnvs {
    #[inline(always)]
    fn from(val: u8) -> Misc0DisconHighSnvs {
        Misc0DisconHighSnvs::from_bits(val)
    }
}
impl From<Misc0DisconHighSnvs> for u8 {
    #[inline(always)]
    fn from(val: Misc0DisconHighSnvs) -> u8 {
        Misc0DisconHighSnvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0OscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0OscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0OscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0OscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0OscI {
        Misc0OscI::from_bits(val)
    }
}
impl From<Misc0OscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0OscI) -> u8 {
        Misc0OscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ReftopSelfbiasoff {
    #[doc = "Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0 = 0x0,
    #[doc = "Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1 = 0x01,
}
impl Misc0ReftopSelfbiasoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ReftopSelfbiasoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ReftopSelfbiasoff {
    #[inline(always)]
    fn from(val: u8) -> Misc0ReftopSelfbiasoff {
        Misc0ReftopSelfbiasoff::from_bits(val)
    }
}
impl From<Misc0ReftopSelfbiasoff> for u8 {
    #[inline(always)]
    fn from(val: Misc0ReftopSelfbiasoff) -> u8 {
        Misc0ReftopSelfbiasoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0ReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0ReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0ReftopVbgadj {
        Misc0ReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0ReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0ReftopVbgadj) -> u8 {
        Misc0ReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0RtcXtalSource {
    #[doc = "Internal ring oscillator"]
    RTC_XTAL_SOURCE_0 = 0x0,
    #[doc = "RTC_XTAL"]
    RTC_XTAL_SOURCE_1 = 0x01,
}
impl Misc0RtcXtalSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0RtcXtalSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0RtcXtalSource {
    #[inline(always)]
    fn from(val: u8) -> Misc0RtcXtalSource {
        Misc0RtcXtalSource::from_bits(val)
    }
}
impl From<Misc0RtcXtalSource> for u8 {
    #[inline(always)]
    fn from(val: Misc0RtcXtalSource) -> u8 {
        Misc0RtcXtalSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetClkgateCtrl {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE = 0x0,
    #[doc = "Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE = 0x01,
}
impl Misc0SetClkgateCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetClkgateCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetClkgateCtrl {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetClkgateCtrl {
        Misc0SetClkgateCtrl::from_bits(val)
    }
}
impl From<Misc0SetClkgateCtrl> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetClkgateCtrl) -> u8 {
        Misc0SetClkgateCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0SetClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetClkgateDelay {
        Misc0SetClkgateDelay::from_bits(val)
    }
}
impl From<Misc0SetClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetClkgateDelay) -> u8 {
        Misc0SetClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetDisconHighSnvs {
    #[doc = "Turn on the switch"]
    DISCON_HIGH_SNVS_0 = 0x0,
    #[doc = "Turn off the switch"]
    DISCON_HIGH_SNVS_1 = 0x01,
}
impl Misc0SetDisconHighSnvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetDisconHighSnvs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetDisconHighSnvs {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetDisconHighSnvs {
        Misc0SetDisconHighSnvs::from_bits(val)
    }
}
impl From<Misc0SetDisconHighSnvs> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetDisconHighSnvs) -> u8 {
        Misc0SetDisconHighSnvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetOscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0SetOscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetOscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetOscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetOscI {
        Misc0SetOscI::from_bits(val)
    }
}
impl From<Misc0SetOscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetOscI) -> u8 {
        Misc0SetOscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetReftopSelfbiasoff {
    #[doc = "Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0 = 0x0,
    #[doc = "Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1 = 0x01,
}
impl Misc0SetReftopSelfbiasoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetReftopSelfbiasoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetReftopSelfbiasoff {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetReftopSelfbiasoff {
        Misc0SetReftopSelfbiasoff::from_bits(val)
    }
}
impl From<Misc0SetReftopSelfbiasoff> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetReftopSelfbiasoff) -> u8 {
        Misc0SetReftopSelfbiasoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0SetReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetReftopVbgadj {
        Misc0SetReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0SetReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetReftopVbgadj) -> u8 {
        Misc0SetReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetRtcXtalSource {
    #[doc = "Internal ring oscillator"]
    RTC_XTAL_SOURCE_0 = 0x0,
    #[doc = "RTC_XTAL"]
    RTC_XTAL_SOURCE_1 = 0x01,
}
impl Misc0SetRtcXtalSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetRtcXtalSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetRtcXtalSource {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetRtcXtalSource {
        Misc0SetRtcXtalSource::from_bits(val)
    }
}
impl From<Misc0SetRtcXtalSource> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetRtcXtalSource) -> u8 {
        Misc0SetRtcXtalSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0SetStopModeConfig {
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0SetStopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetStopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetStopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetStopModeConfig {
        Misc0SetStopModeConfig::from_bits(val)
    }
}
impl From<Misc0SetStopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetStopModeConfig) -> u8 {
        Misc0SetStopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0StopModeConfig {
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0StopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0StopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0StopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0StopModeConfig {
        Misc0StopModeConfig::from_bits(val)
    }
}
impl From<Misc0StopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0StopModeConfig) -> u8 {
        Misc0StopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogClkgateCtrl {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE = 0x0,
    #[doc = "Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE = 0x01,
}
impl Misc0TogClkgateCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogClkgateCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogClkgateCtrl {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogClkgateCtrl {
        Misc0TogClkgateCtrl::from_bits(val)
    }
}
impl From<Misc0TogClkgateCtrl> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogClkgateCtrl) -> u8 {
        Misc0TogClkgateCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogClkgateDelay {
    #[doc = "0.5ms"]
    CLKGATE_DELAY_0 = 0x0,
    #[doc = "1.0ms"]
    CLKGATE_DELAY_1 = 0x01,
    #[doc = "2.0ms"]
    CLKGATE_DELAY_2 = 0x02,
    #[doc = "3.0ms"]
    CLKGATE_DELAY_3 = 0x03,
    #[doc = "4.0ms"]
    CLKGATE_DELAY_4 = 0x04,
    #[doc = "5.0ms"]
    CLKGATE_DELAY_5 = 0x05,
    #[doc = "6.0ms"]
    CLKGATE_DELAY_6 = 0x06,
    #[doc = "7.0ms"]
    CLKGATE_DELAY_7 = 0x07,
}
impl Misc0TogClkgateDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogClkgateDelay {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogClkgateDelay {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogClkgateDelay {
        Misc0TogClkgateDelay::from_bits(val)
    }
}
impl From<Misc0TogClkgateDelay> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogClkgateDelay) -> u8 {
        Misc0TogClkgateDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogDisconHighSnvs {
    #[doc = "Turn on the switch"]
    DISCON_HIGH_SNVS_0 = 0x0,
    #[doc = "Turn off the switch"]
    DISCON_HIGH_SNVS_1 = 0x01,
}
impl Misc0TogDisconHighSnvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogDisconHighSnvs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogDisconHighSnvs {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogDisconHighSnvs {
        Misc0TogDisconHighSnvs::from_bits(val)
    }
}
impl From<Misc0TogDisconHighSnvs> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogDisconHighSnvs) -> u8 {
        Misc0TogDisconHighSnvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogOscI {
    #[doc = "Nominal"]
    NOMINAL = 0x0,
    #[doc = "Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 0x01,
    #[doc = "Decrease current by 25.0%"]
    MINUS_25_PERCENT = 0x02,
    #[doc = "Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 0x03,
}
impl Misc0TogOscI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogOscI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogOscI {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogOscI {
        Misc0TogOscI::from_bits(val)
    }
}
impl From<Misc0TogOscI> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogOscI) -> u8 {
        Misc0TogOscI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogReftopSelfbiasoff {
    #[doc = "Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0 = 0x0,
    #[doc = "Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1 = 0x01,
}
impl Misc0TogReftopSelfbiasoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogReftopSelfbiasoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogReftopSelfbiasoff {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogReftopSelfbiasoff {
        Misc0TogReftopSelfbiasoff::from_bits(val)
    }
}
impl From<Misc0TogReftopSelfbiasoff> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogReftopSelfbiasoff) -> u8 {
        Misc0TogReftopSelfbiasoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogReftopVbgadj {
    #[doc = "Nominal VBG"]
    REFTOP_VBGADJ_0 = 0x0,
    #[doc = "VBG+0.78%"]
    REFTOP_VBGADJ_1 = 0x01,
    #[doc = "VBG+1.56%"]
    REFTOP_VBGADJ_2 = 0x02,
    #[doc = "VBG+2.34%"]
    REFTOP_VBGADJ_3 = 0x03,
    #[doc = "VBG-0.78%"]
    REFTOP_VBGADJ_4 = 0x04,
    #[doc = "VBG-1.56%"]
    REFTOP_VBGADJ_5 = 0x05,
    #[doc = "VBG-2.34%"]
    REFTOP_VBGADJ_6 = 0x06,
    #[doc = "VBG-3.12%"]
    REFTOP_VBGADJ_7 = 0x07,
}
impl Misc0TogReftopVbgadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogReftopVbgadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogReftopVbgadj {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogReftopVbgadj {
        Misc0TogReftopVbgadj::from_bits(val)
    }
}
impl From<Misc0TogReftopVbgadj> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogReftopVbgadj) -> u8 {
        Misc0TogReftopVbgadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogRtcXtalSource {
    #[doc = "Internal ring oscillator"]
    RTC_XTAL_SOURCE_0 = 0x0,
    #[doc = "RTC_XTAL"]
    RTC_XTAL_SOURCE_1 = 0x01,
}
impl Misc0TogRtcXtalSource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogRtcXtalSource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogRtcXtalSource {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogRtcXtalSource {
        Misc0TogRtcXtalSource::from_bits(val)
    }
}
impl From<Misc0TogRtcXtalSource> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogRtcXtalSource) -> u8 {
        Misc0TogRtcXtalSource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0TogStopModeConfig {
    #[doc = "SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Analog regulators are ON."]
    STANDBY = 0x01,
    #[doc = "STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "STOP (very lower power)"]
    STOP_MODE_CONFIG_3 = 0x03,
}
impl Misc0TogStopModeConfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogStopModeConfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogStopModeConfig {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogStopModeConfig {
        Misc0TogStopModeConfig::from_bits(val)
    }
}
impl From<Misc0TogStopModeConfig> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogStopModeConfig) -> u8 {
        Misc0TogStopModeConfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2AudioDivLsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_LSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_LSB_1 = 0x01,
}
impl Misc2AudioDivLsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2AudioDivLsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2AudioDivLsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2AudioDivLsb {
        Misc2AudioDivLsb::from_bits(val)
    }
}
impl From<Misc2AudioDivLsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2AudioDivLsb) -> u8 {
        Misc2AudioDivLsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2AudioDivMsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_MSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_MSB_1 = 0x01,
}
impl Misc2AudioDivMsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2AudioDivMsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2AudioDivMsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2AudioDivMsb {
        Misc2AudioDivMsb::from_bits(val)
    }
}
impl From<Misc2AudioDivMsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2AudioDivMsb) -> u8 {
        Misc2AudioDivMsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrAudioDivLsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_LSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_LSB_1 = 0x01,
}
impl Misc2ClrAudioDivLsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrAudioDivLsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrAudioDivLsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrAudioDivLsb {
        Misc2ClrAudioDivLsb::from_bits(val)
    }
}
impl From<Misc2ClrAudioDivLsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrAudioDivLsb) -> u8 {
        Misc2ClrAudioDivLsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrAudioDivMsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_MSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_MSB_1 = 0x01,
}
impl Misc2ClrAudioDivMsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrAudioDivMsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrAudioDivMsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrAudioDivMsb {
        Misc2ClrAudioDivMsb::from_bits(val)
    }
}
impl From<Misc2ClrAudioDivMsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrAudioDivMsb) -> u8 {
        Misc2ClrAudioDivMsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrReg0BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2ClrReg0BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrReg0BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrReg0BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrReg0BoOffset {
        Misc2ClrReg0BoOffset::from_bits(val)
    }
}
impl From<Misc2ClrReg0BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrReg0BoOffset) -> u8 {
        Misc2ClrReg0BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrReg0StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2ClrReg0StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrReg0StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrReg0StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrReg0StepTime {
        Misc2ClrReg0StepTime::from_bits(val)
    }
}
impl From<Misc2ClrReg0StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrReg0StepTime) -> u8 {
        Misc2ClrReg0StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrReg1BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2ClrReg1BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrReg1BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrReg1BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrReg1BoOffset {
        Misc2ClrReg1BoOffset::from_bits(val)
    }
}
impl From<Misc2ClrReg1BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrReg1BoOffset) -> u8 {
        Misc2ClrReg1BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrReg1StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2ClrReg1StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrReg1StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrReg1StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrReg1StepTime {
        Misc2ClrReg1StepTime::from_bits(val)
    }
}
impl From<Misc2ClrReg1StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrReg1StepTime) -> u8 {
        Misc2ClrReg1StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrReg2BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2ClrReg2BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrReg2BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrReg2BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrReg2BoOffset {
        Misc2ClrReg2BoOffset::from_bits(val)
    }
}
impl From<Misc2ClrReg2BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrReg2BoOffset) -> u8 {
        Misc2ClrReg2BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2ClrReg2StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2ClrReg2StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrReg2StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrReg2StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrReg2StepTime {
        Misc2ClrReg2StepTime::from_bits(val)
    }
}
impl From<Misc2ClrReg2StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrReg2StepTime) -> u8 {
        Misc2ClrReg2StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Reg0BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2Reg0BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Reg0BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Reg0BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2Reg0BoOffset {
        Misc2Reg0BoOffset::from_bits(val)
    }
}
impl From<Misc2Reg0BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2Reg0BoOffset) -> u8 {
        Misc2Reg0BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Reg0StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2Reg0StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Reg0StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Reg0StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2Reg0StepTime {
        Misc2Reg0StepTime::from_bits(val)
    }
}
impl From<Misc2Reg0StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2Reg0StepTime) -> u8 {
        Misc2Reg0StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Reg1BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2Reg1BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Reg1BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Reg1BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2Reg1BoOffset {
        Misc2Reg1BoOffset::from_bits(val)
    }
}
impl From<Misc2Reg1BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2Reg1BoOffset) -> u8 {
        Misc2Reg1BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Reg1StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2Reg1StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Reg1StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Reg1StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2Reg1StepTime {
        Misc2Reg1StepTime::from_bits(val)
    }
}
impl From<Misc2Reg1StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2Reg1StepTime) -> u8 {
        Misc2Reg1StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Reg2BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2Reg2BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Reg2BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Reg2BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2Reg2BoOffset {
        Misc2Reg2BoOffset::from_bits(val)
    }
}
impl From<Misc2Reg2BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2Reg2BoOffset) -> u8 {
        Misc2Reg2BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Reg2StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2Reg2StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Reg2StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Reg2StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2Reg2StepTime {
        Misc2Reg2StepTime::from_bits(val)
    }
}
impl From<Misc2Reg2StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2Reg2StepTime) -> u8 {
        Misc2Reg2StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetAudioDivLsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_LSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_LSB_1 = 0x01,
}
impl Misc2SetAudioDivLsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetAudioDivLsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetAudioDivLsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetAudioDivLsb {
        Misc2SetAudioDivLsb::from_bits(val)
    }
}
impl From<Misc2SetAudioDivLsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetAudioDivLsb) -> u8 {
        Misc2SetAudioDivLsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetAudioDivMsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_MSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_MSB_1 = 0x01,
}
impl Misc2SetAudioDivMsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetAudioDivMsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetAudioDivMsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetAudioDivMsb {
        Misc2SetAudioDivMsb::from_bits(val)
    }
}
impl From<Misc2SetAudioDivMsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetAudioDivMsb) -> u8 {
        Misc2SetAudioDivMsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetReg0BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2SetReg0BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetReg0BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetReg0BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetReg0BoOffset {
        Misc2SetReg0BoOffset::from_bits(val)
    }
}
impl From<Misc2SetReg0BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetReg0BoOffset) -> u8 {
        Misc2SetReg0BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetReg0StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2SetReg0StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetReg0StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetReg0StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetReg0StepTime {
        Misc2SetReg0StepTime::from_bits(val)
    }
}
impl From<Misc2SetReg0StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetReg0StepTime) -> u8 {
        Misc2SetReg0StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetReg1BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2SetReg1BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetReg1BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetReg1BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetReg1BoOffset {
        Misc2SetReg1BoOffset::from_bits(val)
    }
}
impl From<Misc2SetReg1BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetReg1BoOffset) -> u8 {
        Misc2SetReg1BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetReg1StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2SetReg1StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetReg1StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetReg1StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetReg1StepTime {
        Misc2SetReg1StepTime::from_bits(val)
    }
}
impl From<Misc2SetReg1StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetReg1StepTime) -> u8 {
        Misc2SetReg1StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetReg2BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2SetReg2BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetReg2BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetReg2BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetReg2BoOffset {
        Misc2SetReg2BoOffset::from_bits(val)
    }
}
impl From<Misc2SetReg2BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetReg2BoOffset) -> u8 {
        Misc2SetReg2BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2SetReg2StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2SetReg2StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetReg2StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetReg2StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetReg2StepTime {
        Misc2SetReg2StepTime::from_bits(val)
    }
}
impl From<Misc2SetReg2StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetReg2StepTime) -> u8 {
        Misc2SetReg2StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogAudioDivLsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_LSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_LSB_1 = 0x01,
}
impl Misc2TogAudioDivLsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogAudioDivLsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogAudioDivLsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogAudioDivLsb {
        Misc2TogAudioDivLsb::from_bits(val)
    }
}
impl From<Misc2TogAudioDivLsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogAudioDivLsb) -> u8 {
        Misc2TogAudioDivLsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogAudioDivMsb {
    #[doc = "divide by 1 (Default)"]
    AUDIO_DIV_MSB_0 = 0x0,
    #[doc = "divide by 2"]
    AUDIO_DIV_MSB_1 = 0x01,
}
impl Misc2TogAudioDivMsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogAudioDivMsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogAudioDivMsb {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogAudioDivMsb {
        Misc2TogAudioDivMsb::from_bits(val)
    }
}
impl From<Misc2TogAudioDivMsb> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogAudioDivMsb) -> u8 {
        Misc2TogAudioDivMsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogReg0BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 0x07,
}
impl Misc2TogReg0BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogReg0BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogReg0BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogReg0BoOffset {
        Misc2TogReg0BoOffset::from_bits(val)
    }
}
impl From<Misc2TogReg0BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogReg0BoOffset) -> u8 {
        Misc2TogReg0BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogReg0StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2TogReg0StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogReg0StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogReg0StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogReg0StepTime {
        Misc2TogReg0StepTime::from_bits(val)
    }
}
impl From<Misc2TogReg0StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogReg0StepTime) -> u8 {
        Misc2TogReg0StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogReg1BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 0x07,
}
impl Misc2TogReg1BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogReg1BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogReg1BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogReg1BoOffset {
        Misc2TogReg1BoOffset::from_bits(val)
    }
}
impl From<Misc2TogReg1BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogReg1BoOffset) -> u8 {
        Misc2TogReg1BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogReg1StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2TogReg1StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogReg1StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogReg1StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogReg1StepTime {
        Misc2TogReg1StepTime::from_bits(val)
    }
}
impl From<Misc2TogReg1StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogReg1StepTime) -> u8 {
        Misc2TogReg1StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogReg2BoOffset {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 0x07,
}
impl Misc2TogReg2BoOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogReg2BoOffset {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogReg2BoOffset {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogReg2BoOffset {
        Misc2TogReg2BoOffset::from_bits(val)
    }
}
impl From<Misc2TogReg2BoOffset> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogReg2BoOffset) -> u8 {
        Misc2TogReg2BoOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2TogReg2StepTime {
    #[doc = "64"]
    _64_CLOCKS = 0x0,
    #[doc = "128"]
    _128_CLOCKS = 0x01,
    #[doc = "256"]
    _256_CLOCKS = 0x02,
    #[doc = "512"]
    _512_CLOCKS = 0x03,
}
impl Misc2TogReg2StepTime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogReg2StepTime {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogReg2StepTime {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogReg2StepTime {
        Misc2TogReg2StepTime::from_bits(val)
    }
}
impl From<Misc2TogReg2StepTime> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogReg2StepTime) -> u8 {
        Misc2TogReg2StepTime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1ClrOutputTrg {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "0.8V"]
    OUTPUT_TRG_4 = 0x04,
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
    _RESERVED_f = 0x0f,
    #[doc = "1.1V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Reg1p1ClrOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1ClrOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1ClrOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1ClrOutputTrg {
        Reg1p1ClrOutputTrg::from_bits(val)
    }
}
impl From<Reg1p1ClrOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1ClrOutputTrg) -> u8 {
        Reg1p1ClrOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1ClrSelrefWeakLinreg {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0 = 0x0,
    #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
    SELREF_WEAK_LINREG_1 = 0x01,
}
impl Reg1p1ClrSelrefWeakLinreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1ClrSelrefWeakLinreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1ClrSelrefWeakLinreg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1ClrSelrefWeakLinreg {
        Reg1p1ClrSelrefWeakLinreg::from_bits(val)
    }
}
impl From<Reg1p1ClrSelrefWeakLinreg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1ClrSelrefWeakLinreg) -> u8 {
        Reg1p1ClrSelrefWeakLinreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1OutputTrg {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "0.8V"]
    OUTPUT_TRG_4 = 0x04,
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
    _RESERVED_f = 0x0f,
    #[doc = "1.1V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Reg1p1OutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1OutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1OutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1OutputTrg {
        Reg1p1OutputTrg::from_bits(val)
    }
}
impl From<Reg1p1OutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1OutputTrg) -> u8 {
        Reg1p1OutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1SelrefWeakLinreg {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0 = 0x0,
    #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
    SELREF_WEAK_LINREG_1 = 0x01,
}
impl Reg1p1SelrefWeakLinreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1SelrefWeakLinreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1SelrefWeakLinreg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1SelrefWeakLinreg {
        Reg1p1SelrefWeakLinreg::from_bits(val)
    }
}
impl From<Reg1p1SelrefWeakLinreg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1SelrefWeakLinreg) -> u8 {
        Reg1p1SelrefWeakLinreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1SetOutputTrg {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "0.8V"]
    OUTPUT_TRG_4 = 0x04,
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
    _RESERVED_f = 0x0f,
    #[doc = "1.1V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Reg1p1SetOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1SetOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1SetOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1SetOutputTrg {
        Reg1p1SetOutputTrg::from_bits(val)
    }
}
impl From<Reg1p1SetOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1SetOutputTrg) -> u8 {
        Reg1p1SetOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1SetSelrefWeakLinreg {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0 = 0x0,
    #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
    SELREF_WEAK_LINREG_1 = 0x01,
}
impl Reg1p1SetSelrefWeakLinreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1SetSelrefWeakLinreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1SetSelrefWeakLinreg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1SetSelrefWeakLinreg {
        Reg1p1SetSelrefWeakLinreg::from_bits(val)
    }
}
impl From<Reg1p1SetSelrefWeakLinreg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1SetSelrefWeakLinreg) -> u8 {
        Reg1p1SetSelrefWeakLinreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1TogOutputTrg {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "0.8V"]
    OUTPUT_TRG_4 = 0x04,
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
    _RESERVED_f = 0x0f,
    #[doc = "1.1V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Reg1p1TogOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1TogOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1TogOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1TogOutputTrg {
        Reg1p1TogOutputTrg::from_bits(val)
    }
}
impl From<Reg1p1TogOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1TogOutputTrg) -> u8 {
        Reg1p1TogOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg1p1TogSelrefWeakLinreg {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0 = 0x0,
    #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
    SELREF_WEAK_LINREG_1 = 0x01,
}
impl Reg1p1TogSelrefWeakLinreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg1p1TogSelrefWeakLinreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg1p1TogSelrefWeakLinreg {
    #[inline(always)]
    fn from(val: u8) -> Reg1p1TogSelrefWeakLinreg {
        Reg1p1TogSelrefWeakLinreg::from_bits(val)
    }
}
impl From<Reg1p1TogSelrefWeakLinreg> for u8 {
    #[inline(always)]
    fn from(val: Reg1p1TogSelrefWeakLinreg) -> u8 {
        Reg1p1TogSelrefWeakLinreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg2p5ClrOutputTrg {
    #[doc = "2.10V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_f = 0x0f,
    #[doc = "2.50V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "2.875V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg2p5ClrOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg2p5ClrOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg2p5ClrOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5ClrOutputTrg {
        Reg2p5ClrOutputTrg::from_bits(val)
    }
}
impl From<Reg2p5ClrOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5ClrOutputTrg) -> u8 {
        Reg2p5ClrOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg2p5OutputTrg {
    #[doc = "2.10V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_f = 0x0f,
    #[doc = "2.50V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "2.875V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg2p5OutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg2p5OutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg2p5OutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5OutputTrg {
        Reg2p5OutputTrg::from_bits(val)
    }
}
impl From<Reg2p5OutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5OutputTrg) -> u8 {
        Reg2p5OutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg2p5SetOutputTrg {
    #[doc = "2.10V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_f = 0x0f,
    #[doc = "2.50V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "2.875V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg2p5SetOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg2p5SetOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg2p5SetOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5SetOutputTrg {
        Reg2p5SetOutputTrg::from_bits(val)
    }
}
impl From<Reg2p5SetOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5SetOutputTrg) -> u8 {
        Reg2p5SetOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg2p5TogOutputTrg {
    #[doc = "2.10V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_f = 0x0f,
    #[doc = "2.50V"]
    OUTPUT_TRG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "2.875V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg2p5TogOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg2p5TogOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg2p5TogOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg2p5TogOutputTrg {
        Reg2p5TogOutputTrg::from_bits(val)
    }
}
impl From<Reg2p5TogOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg2p5TogOutputTrg) -> u8 {
        Reg2p5TogOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0ClrOutputTrg {
    #[doc = "2.625V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    #[doc = "3.000V"]
    OUTPUT_TRG_15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "3.400V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg3p0ClrOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0ClrOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0ClrOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0ClrOutputTrg {
        Reg3p0ClrOutputTrg::from_bits(val)
    }
}
impl From<Reg3p0ClrOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0ClrOutputTrg) -> u8 {
        Reg3p0ClrOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0ClrVbusSel {
    #[doc = "Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS = 0x0,
    #[doc = "Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS = 0x01,
}
impl Reg3p0ClrVbusSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0ClrVbusSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0ClrVbusSel {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0ClrVbusSel {
        Reg3p0ClrVbusSel::from_bits(val)
    }
}
impl From<Reg3p0ClrVbusSel> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0ClrVbusSel) -> u8 {
        Reg3p0ClrVbusSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0OutputTrg {
    #[doc = "2.625V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    #[doc = "3.000V"]
    OUTPUT_TRG_15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "3.400V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg3p0OutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0OutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0OutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0OutputTrg {
        Reg3p0OutputTrg::from_bits(val)
    }
}
impl From<Reg3p0OutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0OutputTrg) -> u8 {
        Reg3p0OutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0SetOutputTrg {
    #[doc = "2.625V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    #[doc = "3.000V"]
    OUTPUT_TRG_15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "3.400V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg3p0SetOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0SetOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0SetOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0SetOutputTrg {
        Reg3p0SetOutputTrg::from_bits(val)
    }
}
impl From<Reg3p0SetOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0SetOutputTrg) -> u8 {
        Reg3p0SetOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0SetVbusSel {
    #[doc = "Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS = 0x0,
    #[doc = "Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS = 0x01,
}
impl Reg3p0SetVbusSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0SetVbusSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0SetVbusSel {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0SetVbusSel {
        Reg3p0SetVbusSel::from_bits(val)
    }
}
impl From<Reg3p0SetVbusSel> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0SetVbusSel) -> u8 {
        Reg3p0SetVbusSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0TogOutputTrg {
    #[doc = "2.625V"]
    OUTPUT_TRG_0 = 0x0,
    _RESERVED_1 = 0x01,
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
    #[doc = "3.000V"]
    OUTPUT_TRG_15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "3.400V"]
    OUTPUT_TRG_31 = 0x1f,
}
impl Reg3p0TogOutputTrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0TogOutputTrg {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0TogOutputTrg {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0TogOutputTrg {
        Reg3p0TogOutputTrg::from_bits(val)
    }
}
impl From<Reg3p0TogOutputTrg> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0TogOutputTrg) -> u8 {
        Reg3p0TogOutputTrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0TogVbusSel {
    #[doc = "Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS = 0x0,
    #[doc = "Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS = 0x01,
}
impl Reg3p0TogVbusSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0TogVbusSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0TogVbusSel {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0TogVbusSel {
        Reg3p0TogVbusSel::from_bits(val)
    }
}
impl From<Reg3p0TogVbusSel> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0TogVbusSel) -> u8 {
        Reg3p0TogVbusSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reg3p0VbusSel {
    #[doc = "Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS = 0x0,
    #[doc = "Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS = 0x01,
}
impl Reg3p0VbusSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reg3p0VbusSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reg3p0VbusSel {
    #[inline(always)]
    fn from(val: u8) -> Reg3p0VbusSel {
        Reg3p0VbusSel::from_bits(val)
    }
}
impl From<Reg3p0VbusSel> for u8 {
    #[inline(always)]
    fn from(val: Reg3p0VbusSel) -> u8 {
        Reg3p0VbusSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0x0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreClrRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrRampRate {
        RegCoreClrRampRate::from_bits(val)
    }
}
impl From<RegCoreClrRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrRampRate) -> u8 {
        RegCoreClrRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrReg0Adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreClrReg0Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg0Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg0Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg0Adj {
        RegCoreClrReg0Adj::from_bits(val)
    }
}
impl From<RegCoreClrReg0Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg0Adj) -> u8 {
        RegCoreClrReg0Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrReg0Targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreClrReg0Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg0Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg0Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg0Targ {
        RegCoreClrReg0Targ::from_bits(val)
    }
}
impl From<RegCoreClrReg0Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg0Targ) -> u8 {
        RegCoreClrReg0Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrReg1Adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreClrReg1Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg1Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg1Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg1Adj {
        RegCoreClrReg1Adj::from_bits(val)
    }
}
impl From<RegCoreClrReg1Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg1Adj) -> u8 {
        RegCoreClrReg1Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrReg1Targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreClrReg1Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg1Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg1Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg1Targ {
        RegCoreClrReg1Targ::from_bits(val)
    }
}
impl From<RegCoreClrReg1Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg1Targ) -> u8 {
        RegCoreClrReg1Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrReg2Adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreClrReg2Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg2Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg2Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg2Adj {
        RegCoreClrReg2Adj::from_bits(val)
    }
}
impl From<RegCoreClrReg2Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg2Adj) -> u8 {
        RegCoreClrReg2Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreClrReg2Targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreClrReg2Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreClrReg2Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreClrReg2Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreClrReg2Targ {
        RegCoreClrReg2Targ::from_bits(val)
    }
}
impl From<RegCoreClrReg2Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreClrReg2Targ) -> u8 {
        RegCoreClrReg2Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0x0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreRampRate {
        RegCoreRampRate::from_bits(val)
    }
}
impl From<RegCoreRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreRampRate) -> u8 {
        RegCoreRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreReg0Adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreReg0Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg0Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg0Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg0Adj {
        RegCoreReg0Adj::from_bits(val)
    }
}
impl From<RegCoreReg0Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg0Adj) -> u8 {
        RegCoreReg0Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreReg0Targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreReg0Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg0Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg0Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg0Targ {
        RegCoreReg0Targ::from_bits(val)
    }
}
impl From<RegCoreReg0Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg0Targ) -> u8 {
        RegCoreReg0Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreReg1Adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreReg1Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg1Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg1Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg1Adj {
        RegCoreReg1Adj::from_bits(val)
    }
}
impl From<RegCoreReg1Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg1Adj) -> u8 {
        RegCoreReg1Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreReg1Targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreReg1Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg1Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg1Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg1Targ {
        RegCoreReg1Targ::from_bits(val)
    }
}
impl From<RegCoreReg1Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg1Targ) -> u8 {
        RegCoreReg1Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreReg2Adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreReg2Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg2Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg2Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg2Adj {
        RegCoreReg2Adj::from_bits(val)
    }
}
impl From<RegCoreReg2Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg2Adj) -> u8 {
        RegCoreReg2Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreReg2Targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreReg2Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreReg2Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreReg2Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreReg2Targ {
        RegCoreReg2Targ::from_bits(val)
    }
}
impl From<RegCoreReg2Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreReg2Targ) -> u8 {
        RegCoreReg2Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0x0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreSetRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetRampRate {
        RegCoreSetRampRate::from_bits(val)
    }
}
impl From<RegCoreSetRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetRampRate) -> u8 {
        RegCoreSetRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetReg0Adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreSetReg0Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg0Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg0Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg0Adj {
        RegCoreSetReg0Adj::from_bits(val)
    }
}
impl From<RegCoreSetReg0Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg0Adj) -> u8 {
        RegCoreSetReg0Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetReg0Targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreSetReg0Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg0Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg0Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg0Targ {
        RegCoreSetReg0Targ::from_bits(val)
    }
}
impl From<RegCoreSetReg0Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg0Targ) -> u8 {
        RegCoreSetReg0Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetReg1Adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreSetReg1Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg1Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg1Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg1Adj {
        RegCoreSetReg1Adj::from_bits(val)
    }
}
impl From<RegCoreSetReg1Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg1Adj) -> u8 {
        RegCoreSetReg1Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetReg1Targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreSetReg1Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg1Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg1Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg1Targ {
        RegCoreSetReg1Targ::from_bits(val)
    }
}
impl From<RegCoreSetReg1Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg1Targ) -> u8 {
        RegCoreSetReg1Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetReg2Adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreSetReg2Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg2Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg2Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg2Adj {
        RegCoreSetReg2Adj::from_bits(val)
    }
}
impl From<RegCoreSetReg2Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg2Adj) -> u8 {
        RegCoreSetReg2Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreSetReg2Targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreSetReg2Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreSetReg2Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreSetReg2Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreSetReg2Targ {
        RegCoreSetReg2Targ::from_bits(val)
    }
}
impl From<RegCoreSetReg2Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreSetReg2Targ) -> u8 {
        RegCoreSetReg2Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogRampRate {
    #[doc = "Fast"]
    RAMP_RATE_0 = 0x0,
    #[doc = "Medium Fast"]
    RAMP_RATE_1 = 0x01,
    #[doc = "Medium Slow"]
    RAMP_RATE_2 = 0x02,
    #[doc = "Slow"]
    RAMP_RATE_3 = 0x03,
}
impl RegCoreTogRampRate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogRampRate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogRampRate {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogRampRate {
        RegCoreTogRampRate::from_bits(val)
    }
}
impl From<RegCoreTogRampRate> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogRampRate) -> u8 {
        RegCoreTogRampRate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogReg0Adj {
    #[doc = "No adjustment"]
    REG0_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG0_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG0_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG0_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG0_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG0_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG0_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG0_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG0_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG0_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG0_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG0_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG0_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG0_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG0_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG0_ADJ_15 = 0x0f,
}
impl RegCoreTogReg0Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg0Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg0Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg0Adj {
        RegCoreTogReg0Adj::from_bits(val)
    }
}
impl From<RegCoreTogReg0Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg0Adj) -> u8 {
        RegCoreTogReg0Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogReg0Targ {
    #[doc = "Power gated off"]
    REG0_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG0_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG0_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG0_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG0_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG0_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG0_TARG_31 = 0x1f,
}
impl RegCoreTogReg0Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg0Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg0Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg0Targ {
        RegCoreTogReg0Targ::from_bits(val)
    }
}
impl From<RegCoreTogReg0Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg0Targ) -> u8 {
        RegCoreTogReg0Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogReg1Adj {
    #[doc = "No adjustment"]
    REG1_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG1_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG1_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG1_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG1_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG1_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG1_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG1_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG1_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG1_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG1_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG1_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG1_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG1_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG1_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG1_ADJ_15 = 0x0f,
}
impl RegCoreTogReg1Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg1Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg1Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg1Adj {
        RegCoreTogReg1Adj::from_bits(val)
    }
}
impl From<RegCoreTogReg1Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg1Adj) -> u8 {
        RegCoreTogReg1Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogReg1Targ {
    #[doc = "Power gated off"]
    REG1_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG1_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG1_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG1_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG1_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG1_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG1_TARG_31 = 0x1f,
}
impl RegCoreTogReg1Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg1Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg1Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg1Targ {
        RegCoreTogReg1Targ::from_bits(val)
    }
}
impl From<RegCoreTogReg1Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg1Targ) -> u8 {
        RegCoreTogReg1Targ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogReg2Adj {
    #[doc = "No adjustment"]
    REG2_ADJ_0 = 0x0,
    #[doc = "+ 0.25%"]
    REG2_ADJ_1 = 0x01,
    #[doc = "+ 0.50%"]
    REG2_ADJ_2 = 0x02,
    #[doc = "+ 0.75%"]
    REG2_ADJ_3 = 0x03,
    #[doc = "+ 1.00%"]
    REG2_ADJ_4 = 0x04,
    #[doc = "+ 1.25%"]
    REG2_ADJ_5 = 0x05,
    #[doc = "+ 1.50%"]
    REG2_ADJ_6 = 0x06,
    #[doc = "+ 1.75%"]
    REG2_ADJ_7 = 0x07,
    #[doc = "- 0.25%"]
    REG2_ADJ_8 = 0x08,
    #[doc = "- 0.50%"]
    REG2_ADJ_9 = 0x09,
    #[doc = "- 0.75%"]
    REG2_ADJ_10 = 0x0a,
    #[doc = "- 1.00%"]
    REG2_ADJ_11 = 0x0b,
    #[doc = "- 1.25%"]
    REG2_ADJ_12 = 0x0c,
    #[doc = "- 1.50%"]
    REG2_ADJ_13 = 0x0d,
    #[doc = "- 1.75%"]
    REG2_ADJ_14 = 0x0e,
    #[doc = "- 2.00%"]
    REG2_ADJ_15 = 0x0f,
}
impl RegCoreTogReg2Adj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg2Adj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg2Adj {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg2Adj {
        RegCoreTogReg2Adj::from_bits(val)
    }
}
impl From<RegCoreTogReg2Adj> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg2Adj) -> u8 {
        RegCoreTogReg2Adj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegCoreTogReg2Targ {
    #[doc = "Power gated off"]
    REG2_TARG_0 = 0x0,
    #[doc = "Target core voltage = 0.725V"]
    REG2_TARG_1 = 0x01,
    #[doc = "Target core voltage = 0.750V"]
    REG2_TARG_2 = 0x02,
    #[doc = "Target core voltage = 0.775V"]
    REG2_TARG_3 = 0x03,
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
    _RESERVED_f = 0x0f,
    #[doc = "Target core voltage = 1.100V"]
    REG2_TARG_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Target core voltage = 1.450V"]
    REG2_TARG_30 = 0x1e,
    #[doc = "Power FET switched full on. No regulation."]
    REG2_TARG_31 = 0x1f,
}
impl RegCoreTogReg2Targ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegCoreTogReg2Targ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegCoreTogReg2Targ {
    #[inline(always)]
    fn from(val: u8) -> RegCoreTogReg2Targ {
        RegCoreTogReg2Targ::from_bits(val)
    }
}
impl From<RegCoreTogReg2Targ> for u8 {
    #[inline(always)]
    fn from(val: RegCoreTogReg2Targ) -> u8 {
        RegCoreTogReg2Targ::to_bits(val)
    }
}
