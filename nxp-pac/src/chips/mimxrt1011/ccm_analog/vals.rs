#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "Spread spectrum modulation disabled"]
    ENABLE_0 = 0x0,
    #[doc = "Soread spectrum modulation enabled"]
    ENABLE_1 = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
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
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
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
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
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
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
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
    #[doc = "All analog except RTC powered down on stop mode assertion."]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Beside RTC, analog bandgap, 1p1 and 2p5 regulators are also on."]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "Beside RTC, 1p1 and 2p5 regulators are also on, low-power bandgap is selected so that the normal analog bandgap together with the rest analog is powered down."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "Beside RTC, low-power bandgap is selected and the rest analog is powered down."]
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
pub enum Misc2ClrPll3Disable {
    #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
    PLL3_DISABLE_0 = 0x0,
    #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
    PLL3_DISABLE_1 = 0x01,
}
impl Misc2ClrPll3Disable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2ClrPll3Disable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2ClrPll3Disable {
    #[inline(always)]
    fn from(val: u8) -> Misc2ClrPll3Disable {
        Misc2ClrPll3Disable::from_bits(val)
    }
}
impl From<Misc2ClrPll3Disable> for u8 {
    #[inline(always)]
    fn from(val: Misc2ClrPll3Disable) -> u8 {
        Misc2ClrPll3Disable::to_bits(val)
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
pub enum Misc2Pll3Disable {
    #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
    PLL3_DISABLE_0 = 0x0,
    #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
    PLL3_DISABLE_1 = 0x01,
}
impl Misc2Pll3Disable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Pll3Disable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Pll3Disable {
    #[inline(always)]
    fn from(val: u8) -> Misc2Pll3Disable {
        Misc2Pll3Disable::from_bits(val)
    }
}
impl From<Misc2Pll3Disable> for u8 {
    #[inline(always)]
    fn from(val: Misc2Pll3Disable) -> u8 {
        Misc2Pll3Disable::to_bits(val)
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
pub enum Misc2SetPll3Disable {
    #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
    PLL3_DISABLE_0 = 0x0,
    #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
    PLL3_DISABLE_1 = 0x01,
}
impl Misc2SetPll3Disable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2SetPll3Disable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2SetPll3Disable {
    #[inline(always)]
    fn from(val: u8) -> Misc2SetPll3Disable {
        Misc2SetPll3Disable::from_bits(val)
    }
}
impl From<Misc2SetPll3Disable> for u8 {
    #[inline(always)]
    fn from(val: Misc2SetPll3Disable) -> u8 {
        Misc2SetPll3Disable::to_bits(val)
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
pub enum Misc2TogPll3Disable {
    #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
    PLL3_DISABLE_0 = 0x0,
    #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
    PLL3_DISABLE_1 = 0x01,
}
impl Misc2TogPll3Disable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2TogPll3Disable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2TogPll3Disable {
    #[inline(always)]
    fn from(val: u8) -> Misc2TogPll3Disable {
        Misc2TogPll3Disable::from_bits(val)
    }
}
impl From<Misc2TogPll3Disable> for u8 {
    #[inline(always)]
    fn from(val: Misc2TogPll3Disable) -> u8 {
        Misc2TogPll3Disable::to_bits(val)
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
pub enum PllAudioBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioBypassClkSrc {
        PllAudioBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioBypassClkSrc) -> u8 {
        PllAudioBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioClrBypassClkSrc {
        PllAudioClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioClrBypassClkSrc) -> u8 {
        PllAudioClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioClrPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioClrPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioClrPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioClrPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioClrPostDivSelect {
        PllAudioClrPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioClrPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioClrPostDivSelect) -> u8 {
        PllAudioClrPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioPostDivSelect {
        PllAudioPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioPostDivSelect) -> u8 {
        PllAudioPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioSetBypassClkSrc {
        PllAudioSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioSetBypassClkSrc) -> u8 {
        PllAudioSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioSetPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioSetPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioSetPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioSetPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioSetPostDivSelect {
        PllAudioSetPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioSetPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioSetPostDivSelect) -> u8 {
        PllAudioSetPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllAudioTogBypassClkSrc {
        PllAudioTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllAudioTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllAudioTogBypassClkSrc) -> u8 {
        PllAudioTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllAudioTogPostDivSelect {
    #[doc = "Divide by 4."]
    POST_DIV_SELECT_0 = 0x0,
    #[doc = "Divide by 2."]
    POST_DIV_SELECT_1 = 0x01,
    #[doc = "Divide by 1."]
    POST_DIV_SELECT_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllAudioTogPostDivSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllAudioTogPostDivSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllAudioTogPostDivSelect {
    #[inline(always)]
    fn from(val: u8) -> PllAudioTogPostDivSelect {
        PllAudioTogPostDivSelect::from_bits(val)
    }
}
impl From<PllAudioTogPostDivSelect> for u8 {
    #[inline(always)]
    fn from(val: PllAudioTogPostDivSelect) -> u8 {
        PllAudioTogPostDivSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetBypassClkSrc {
        PllEnetBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetBypassClkSrc) -> u8 {
        PllEnetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetClrBypassClkSrc {
        PllEnetClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetClrBypassClkSrc) -> u8 {
        PllEnetClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetSetBypassClkSrc {
        PllEnetSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetSetBypassClkSrc) -> u8 {
        PllEnetSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnetTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllEnetTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnetTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnetTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllEnetTogBypassClkSrc {
        PllEnetTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllEnetTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllEnetTogBypassClkSrc) -> u8 {
        PllEnetTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysBypassClkSrc {
        PllSysBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysBypassClkSrc) -> u8 {
        PllSysBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysClrBypassClkSrc {
        PllSysClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysClrBypassClkSrc) -> u8 {
        PllSysClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysSetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysSetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysSetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysSetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysSetBypassClkSrc {
        PllSysSetBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysSetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysSetBypassClkSrc) -> u8 {
        PllSysSetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSysTogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllSysTogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSysTogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSysTogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllSysTogBypassClkSrc {
        PllSysTogBypassClkSrc::from_bits(val)
    }
}
impl From<PllSysTogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllSysTogBypassClkSrc) -> u8 {
        PllSysTogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1BypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1BypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1BypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1BypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1BypassClkSrc {
        PllUsb1BypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1BypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1BypassClkSrc) -> u8 {
        PllUsb1BypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1ClrBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1ClrBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1ClrBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1ClrBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1ClrBypassClkSrc {
        PllUsb1ClrBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1ClrBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1ClrBypassClkSrc) -> u8 {
        PllUsb1ClrBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1ClrEnUsbClks {
    #[doc = "PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0 = 0x0,
    #[doc = "PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1 = 0x01,
}
impl PllUsb1ClrEnUsbClks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1ClrEnUsbClks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1ClrEnUsbClks {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1ClrEnUsbClks {
        PllUsb1ClrEnUsbClks::from_bits(val)
    }
}
impl From<PllUsb1ClrEnUsbClks> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1ClrEnUsbClks) -> u8 {
        PllUsb1ClrEnUsbClks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1EnUsbClks {
    #[doc = "PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0 = 0x0,
    #[doc = "PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1 = 0x01,
}
impl PllUsb1EnUsbClks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1EnUsbClks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1EnUsbClks {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1EnUsbClks {
        PllUsb1EnUsbClks::from_bits(val)
    }
}
impl From<PllUsb1EnUsbClks> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1EnUsbClks) -> u8 {
        PllUsb1EnUsbClks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1SetBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1SetBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1SetBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1SetBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1SetBypassClkSrc {
        PllUsb1SetBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1SetBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1SetBypassClkSrc) -> u8 {
        PllUsb1SetBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1SetEnUsbClks {
    #[doc = "PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0 = 0x0,
    #[doc = "PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1 = 0x01,
}
impl PllUsb1SetEnUsbClks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1SetEnUsbClks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1SetEnUsbClks {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1SetEnUsbClks {
        PllUsb1SetEnUsbClks::from_bits(val)
    }
}
impl From<PllUsb1SetEnUsbClks> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1SetEnUsbClks) -> u8 {
        PllUsb1SetEnUsbClks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1TogBypassClkSrc {
    #[doc = "Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PllUsb1TogBypassClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1TogBypassClkSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1TogBypassClkSrc {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1TogBypassClkSrc {
        PllUsb1TogBypassClkSrc::from_bits(val)
    }
}
impl From<PllUsb1TogBypassClkSrc> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1TogBypassClkSrc) -> u8 {
        PllUsb1TogBypassClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllUsb1TogEnUsbClks {
    #[doc = "PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0 = 0x0,
    #[doc = "PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1 = 0x01,
}
impl PllUsb1TogEnUsbClks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllUsb1TogEnUsbClks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllUsb1TogEnUsbClks {
    #[inline(always)]
    fn from(val: u8) -> PllUsb1TogEnUsbClks {
        PllUsb1TogEnUsbClks::from_bits(val)
    }
}
impl From<PllUsb1TogEnUsbClks> for u8 {
    #[inline(always)]
    fn from(val: PllUsb1TogEnUsbClks) -> u8 {
        PllUsb1TogEnUsbClks::to_bits(val)
    }
}
