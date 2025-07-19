#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlClrOscSel {
    #[doc = "XTAL OSC"]
    OSC_SEL_0 = 0x0,
    #[doc = "RC OSC"]
    OSC_SEL_1 = 0x01,
}
impl LowpwrCtrlClrOscSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlClrOscSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlClrOscSel {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlClrOscSel {
        LowpwrCtrlClrOscSel::from_bits(val)
    }
}
impl From<LowpwrCtrlClrOscSel> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlClrOscSel) -> u8 {
        LowpwrCtrlClrOscSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlClrXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlClrXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlClrXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlClrXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlClrXtaloscPwrupDelay {
        LowpwrCtrlClrXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlClrXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlClrXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlClrXtaloscPwrupDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlOscSel {
    #[doc = "XTAL OSC"]
    OSC_SEL_0 = 0x0,
    #[doc = "RC OSC"]
    OSC_SEL_1 = 0x01,
}
impl LowpwrCtrlOscSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlOscSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlOscSel {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlOscSel {
        LowpwrCtrlOscSel::from_bits(val)
    }
}
impl From<LowpwrCtrlOscSel> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlOscSel) -> u8 {
        LowpwrCtrlOscSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlSetOscSel {
    #[doc = "XTAL OSC"]
    OSC_SEL_0 = 0x0,
    #[doc = "RC OSC"]
    OSC_SEL_1 = 0x01,
}
impl LowpwrCtrlSetOscSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlSetOscSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlSetOscSel {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlSetOscSel {
        LowpwrCtrlSetOscSel::from_bits(val)
    }
}
impl From<LowpwrCtrlSetOscSel> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlSetOscSel) -> u8 {
        LowpwrCtrlSetOscSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlSetXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlSetXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlSetXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlSetXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlSetXtaloscPwrupDelay {
        LowpwrCtrlSetXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlSetXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlSetXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlSetXtaloscPwrupDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlTogOscSel {
    #[doc = "XTAL OSC"]
    OSC_SEL_0 = 0x0,
    #[doc = "RC OSC"]
    OSC_SEL_1 = 0x01,
}
impl LowpwrCtrlTogOscSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlTogOscSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlTogOscSel {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlTogOscSel {
        LowpwrCtrlTogOscSel::from_bits(val)
    }
}
impl From<LowpwrCtrlTogOscSel> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlTogOscSel) -> u8 {
        LowpwrCtrlTogOscSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlTogXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlTogXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlTogXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlTogXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlTogXtaloscPwrupDelay {
        LowpwrCtrlTogXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlTogXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlTogXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlTogXtaloscPwrupDelay::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowpwrCtrlXtaloscPwrupDelay {
    #[doc = "0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0x0,
    #[doc = "0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 0x01,
    #[doc = "1ms"]
    XTALOSC_PWRUP_DELAY_2 = 0x02,
    #[doc = "2ms"]
    XTALOSC_PWRUP_DELAY_3 = 0x03,
}
impl LowpwrCtrlXtaloscPwrupDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowpwrCtrlXtaloscPwrupDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowpwrCtrlXtaloscPwrupDelay {
    #[inline(always)]
    fn from(val: u8) -> LowpwrCtrlXtaloscPwrupDelay {
        LowpwrCtrlXtaloscPwrupDelay::from_bits(val)
    }
}
impl From<LowpwrCtrlXtaloscPwrupDelay> for u8 {
    #[inline(always)]
    fn from(val: LowpwrCtrlXtaloscPwrupDelay) -> u8 {
        LowpwrCtrlXtaloscPwrupDelay::to_bits(val)
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
pub enum Misc0ClrVidPllPrediv {
    #[doc = "Divide by 1"]
    VID_PLL_PREDIV_0 = 0x0,
    #[doc = "Divide by 2"]
    VID_PLL_PREDIV_1 = 0x01,
}
impl Misc0ClrVidPllPrediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0ClrVidPllPrediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0ClrVidPllPrediv {
    #[inline(always)]
    fn from(val: u8) -> Misc0ClrVidPllPrediv {
        Misc0ClrVidPllPrediv::from_bits(val)
    }
}
impl From<Misc0ClrVidPllPrediv> for u8 {
    #[inline(always)]
    fn from(val: Misc0ClrVidPllPrediv) -> u8 {
        Misc0ClrVidPllPrediv::to_bits(val)
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
pub enum Misc0SetVidPllPrediv {
    #[doc = "Divide by 1"]
    VID_PLL_PREDIV_0 = 0x0,
    #[doc = "Divide by 2"]
    VID_PLL_PREDIV_1 = 0x01,
}
impl Misc0SetVidPllPrediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0SetVidPllPrediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0SetVidPllPrediv {
    #[inline(always)]
    fn from(val: u8) -> Misc0SetVidPllPrediv {
        Misc0SetVidPllPrediv::from_bits(val)
    }
}
impl From<Misc0SetVidPllPrediv> for u8 {
    #[inline(always)]
    fn from(val: Misc0SetVidPllPrediv) -> u8 {
        Misc0SetVidPllPrediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0StopModeConfig {
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
    #[doc = "All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_0 = 0x0,
    #[doc = "Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;"]
    STOP_MODE_CONFIG_1 = 0x01,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=on, New BG=off."]
    STOP_MODE_CONFIG_2 = 0x02,
    #[doc = "XtalOsc=off, RCOsc=on, Old BG=off, New BG=on."]
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
pub enum Misc0TogVidPllPrediv {
    #[doc = "Divide by 1"]
    VID_PLL_PREDIV_0 = 0x0,
    #[doc = "Divide by 2"]
    VID_PLL_PREDIV_1 = 0x01,
}
impl Misc0TogVidPllPrediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0TogVidPllPrediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0TogVidPllPrediv {
    #[inline(always)]
    fn from(val: u8) -> Misc0TogVidPllPrediv {
        Misc0TogVidPllPrediv::from_bits(val)
    }
}
impl From<Misc0TogVidPllPrediv> for u8 {
    #[inline(always)]
    fn from(val: Misc0TogVidPllPrediv) -> u8 {
        Misc0TogVidPllPrediv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc0VidPllPrediv {
    #[doc = "Divide by 1"]
    VID_PLL_PREDIV_0 = 0x0,
    #[doc = "Divide by 2"]
    VID_PLL_PREDIV_1 = 0x01,
}
impl Misc0VidPllPrediv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc0VidPllPrediv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc0VidPllPrediv {
    #[inline(always)]
    fn from(val: u8) -> Misc0VidPllPrediv {
        Misc0VidPllPrediv::from_bits(val)
    }
}
impl From<Misc0VidPllPrediv> for u8 {
    #[inline(always)]
    fn from(val: Misc0VidPllPrediv) -> u8 {
        Misc0VidPllPrediv::to_bits(val)
    }
}
